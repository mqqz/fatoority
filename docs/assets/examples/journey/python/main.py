"""One simplified invoice: create, sign, validate, report, inspect the result."""
import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import sys
from uuid import uuid4

from fatoora import InvoiceBuilder, InvoiceSubType, InvoiceTypeKind, VatCategory
from fatoora.api import CsidProduction, ZatcaClient
from fatoora.config import Config, Environment
from fatoora.sign import Signer
from fatoora.errors import FfiError
from fatoora.validation import validate_zatca_invoice_from_str

ENVIRONMENT = Environment.NON_PRODUCTION  # Integration Sandbox only.


def required(name):
    value = os.environ.get(name, "").strip()
    if not value:
        raise ValueError(f"Set {name} first")
    return value


def save_json(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def messages(items):
    result = []
    for item in items:
        with item:
            result.append({"code": item.code(), "message": item.message()})
    return result


def handle_response(response, output):
    with response.validation_results() as results:
        summary = {
            "http_status": response.http_status(),
            "outcome": response.outcome().name,
            "reporting_status": response.reporting_status(),
            "validation_status": results.status(),
            "info": messages(results.info_messages()),
            "warnings": messages(results.warning_messages()),
            "errors": messages(results.error_messages()),
        }
    save_json(output / "response.json", summary)
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    response.ensure_accepted()  # A successful HTTP status alone is insufficient.
    print("Sandbox reporting accepted. Keep any warnings in response.json.")


def run(input_path, submit):
    # Read credentials before creating an invoice; never write them into artifacts.
    certificate = Path(required("ZATCA_CERT_PATH")).read_text(encoding="utf-8")
    key = Path(required("ZATCA_KEY_PATH")).read_text(encoding="utf-8")
    token, secret = (required("ZATCA_TOKEN"), required("ZATCA_SECRET")) if submit else (None, None)
    data = json.loads(Path(input_path).read_text(encoding="utf-8"))
    seller = data["seller"]
    invoice_uuid = str(uuid4())
    output = Path("runs") / invoice_uuid
    output.mkdir(parents=True, exist_ok=False)
    print(f"Artifacts: {output}")
    state = {"uuid": invoice_uuid, "invoice_id": data["invoice_id"], "stage": "create", "submitted": False}
    save_json(output / "state.json", state)
    try:
        with InvoiceBuilder.new(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED) as builder:
            builder.set_id(data["invoice_id"])
            builder.set_uuid(invoice_uuid)
            builder.set_issue_datetime(datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))
            builder.set_currency("SAR")
            builder.set_invoice_counter(int(data["invoice_counter"]))
            builder.set_previous_invoice_hash(data["previous_invoice_hash"])
            builder.set_payment_means_code("10")
            builder.set_vat_category(VatCategory.STANDARD)
            builder.set_seller(
                name=seller["name"], country_code="SA", city=seller["city"],
                street=seller["street"], building_number=seller["building_number"],
                postal_code=seller["postal_code"], district=seller["district"],
                additional_number=seller["additional_number"], vat_id=seller["vat_id"],
                other_id=seller["crn"], other_id_scheme="CRN",
            )
            builder.add_line_item("دفتر", "2", "PCE", "50", "15", VatCategory.STANDARD)
            with builder.build() as invoice, Signer.from_pem(certificate, key) as signer:
                (output / "invoice.xml").write_text(invoice.to_xml(), encoding="utf-8")
                print("Created: SAR 100 net + SAR 15 VAT = SAR 115 payable")
                state["stage"] = "sign"
                with invoice.sign(signer) as signed, Config(ENVIRONMENT) as config:
                    signed_xml = signed.xml()
                    (output / "signed.xml").write_text(signed_xml, encoding="utf-8")
                    state["invoice_hash"] = signed.invoice_hash()
                    state["stage"] = "validate"
                    report = validate_zatca_invoice_from_str(
                        config, signed_xml, previous_invoice_hash=data["previous_invoice_hash"],
                    )
                    save_json(output / "validation.json", report)
                    if not report["is_valid"]:
                        raise ValueError("Local validation failed; inspect validation.json. Nothing was submitted.")
                    print("Local validation passed; inspect validation.json for warnings")
                    if not submit:
                        state["stage"] = "local_only"
                        print("Local run complete. No reporting request sent. Use --submit with sandbox credentials.")
                        return
                    state["stage"] = "report"
                    state["submitted"] = True  # Attempted; outcome may be unknown if the connection fails.
                    save_json(output / "state.json", state)
                    with CsidProduction.new(ENVIRONMENT, token, secret, None) as credentials:
                        with ZatcaClient(config) as client:
                            with client.report_simplified_invoice(signed, credentials, False, "en") as response:
                                handle_response(response, output)
                    state["stage"] = "accepted"
    except Exception as error:
        state["error_type"] = type(error).__name__
        if isinstance(error, FfiError):
            save_json(output / "error.json", {"code": error.code, "details": error.details})
        raise
    finally:
        save_json(output / "state.json", state)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", default="input.json")
    parser.add_argument("--submit", action="store_true", help="Report to the Integration Sandbox")
    args = parser.parse_args()
    try:
        run(args.input, args.submit)
    except Exception as error:
        print(f"Stopped: {error}", file=sys.stderr)
        print("No automatic retry. Inspect saved artifacts before another run.", file=sys.stderr)
        sys.exit(1)
