#!/usr/bin/env python3
"""Compare QR fields using the pinned SDK and the built Fatoority CLI.

Uses an isolated SDK copy. Outputs are evidence, not remote clearance results.
"""
import argparse
import base64
import json
import os
from datetime import datetime, timezone
from pathlib import Path
import re
import xml.etree.ElementTree as ET

import sdk_parity as sdk

CBC = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
CAC = "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
NS = {"cbc": CBC, "cac": CAC}


def embedded(xml):
    root = ET.fromstring(xml)
    return next(
        n.findtext("cac:Attachment/cbc:EmbeddedDocumentBinaryObject", namespaces=NS)
        for n in root.findall("cac:AdditionalDocumentReference", NS)
        if n.findtext("cbc:ID", namespaces=NS) == "QR"
    )


def fields(encoded):
    raw = base64.b64decode(encoded, validate=True)
    tags = {}
    at = 0
    previous = 0
    while at < len(raw):
        if len(raw) - at < 2:
            raise sdk.CaptureError("truncated QR header")
        tag, size = raw[at:at + 2]
        at += 2
        if tag <= previous or not size or tag > 9 or at + size > len(raw):
            raise sdk.CaptureError("invalid QR framing")
        tags[tag] = raw[at:at + size]
        at += size
        previous = tag
    return {"encoded": encoded, "tags": list(tags), "tag4": tags[4].decode(), "tag5": tags[5].decode(),
            "tag9_base64": base64.b64encode(tags[9]).decode() if 9 in tags else None}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sdk-root", type=Path, default=Path(os.environ.get("FATOORA_HOME", "/missing-sdk")))
    parser.add_argument("--cli", type=Path, default=sdk.REPO / "target/debug/fatoora")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = sdk.sdk_root(args.sdk_root)
    output = args.output.resolve()
    sdk.new_output(output)
    cli = args.cli.resolve()
    before = {str(p.relative_to(root)): sdk.digest(p) for p in root.rglob("*") if p.is_file()}
    cases = {}
    for subtype in ["standard", "simplified"]:
        for kind in ["invoice", "credit", "debit"]:
            name = f"{subtype}-{kind}"
            cases[name] = (sdk.CORPUS / "cases" / name / "input.xml").read_text()
    base = cases["simplified-invoice"]
    for amount in ["300", "300.0", "300.00", "300.01", "0.00", "45.0", "45.00", "123456789012345678901234.56"]:
        cases[f"decimal-{amount}"] = re.sub(
            r"(?P<open><cbc:(?:PayableAmount|TaxAmount)\b[^>]*>)[^<]*(?P<close></cbc:(?:PayableAmount|TaxAmount)>)",
            lambda m: m["open"] + amount + m["close"], base)
    cases["foreign-currency-vat"] = (sdk.REPO / "fatoora-core/tests/fixtures/invoices/Standard/Invoice/Advance Payment adjustments with foreign currency invoice.xml").read_text()
    results = {}
    with sdk.isolated_sdk(root) as (scratch, env):
        env["JAVA_TOOL_OPTIONS"] = "-Dlog4j2.hostName=localhost -Dfile.encoding=UTF-8"
        for name, xml in cases.items():
            directory = output / name
            directory.mkdir()
            source = directory / "input.xml"
            source.write_text(xml)
            rust = directory / "fatoority.xml"
            official = directory / "sdk.xml"
            record = sdk.run([cli, "sign", "--invoice", source, "--cert", sdk.CORPUS / "credentials/certificate.der",
                              "--key", sdk.CORPUS / "credentials/private-key.der", "--cert-format", "der", "--key-format", "der",
                              "--signed-invoice", rust], sdk.REPO, os.environ.copy(), directory / "evidence/fatoority-sign")
            if record["exit_code"]:
                raise sdk.CaptureError(f"Fatoority signing failed: {name}")
            record, raw = sdk.invoke(scratch, env, ["-sign", "-invoice", source, "-signedInvoice", official], directory / "evidence/sdk-sign")
            if record["exit_code"] or not official.exists():
                raise sdk.CaptureError(f"SDK signing failed: {name}: {raw}")
            record, raw = sdk.invoke(scratch, env, ["-qr", "-invoice", official], directory / "evidence/sdk-qr")
            matches = re.findall(r"QR code\s*=\s*(\S+)", raw)
            if record["exit_code"] or len(matches) != 1:
                raise sdk.CaptureError(f"SDK QR generation failed: {name}")
            result = {"input_sha256": sdk.digest(source),
                      "subtype": ET.fromstring(xml).find("cbc:InvoiceTypeCode", NS).attrib["name"],
                      "sdk": fields(matches[0]), "fatoority": fields(embedded(rust.read_text()))}
            for engine, path in [("sdk", official), ("fatoority", rust)]:
                record, raw = sdk.invoke(scratch, env, ["-validate", "-invoice", path], directory / f"evidence/sdk-validates-{engine}")
                result[f"sdk_validation_{engine}"] = sdk.parse_validation_report(raw, record["exit_code"])
            native_evidence = directory / "evidence/fatoority-validation"
            sdk.run([cli, "validate", "--invoice", rust, "--profile", "zatca", "--format", "json"], sdk.REPO, os.environ.copy(), native_evidence)
            native = json.loads((native_evidence / "stdout.txt").read_text())
            result["fatoority_validation"] = {
                "profile": native["profile"],
                "stages": {stage["stage"]: {"status": stage["status"], "findings": stage["findings"]} for stage in native["stages"]},
            }
            results[name] = result
            print("compared", name, flush=True)
        source = sdk.REPO / "fatoora-core/tests/fixtures/business-rules/integrity/cases/qr-total-scale/input.xml"
        record, raw = sdk.invoke(scratch, env, ["-validate", "-invoice", source], output / "qr-total-scale/evidence")
        native_evidence = output / "qr-total-scale/fatoority-validation"
        sdk.run([cli, "validate", "--invoice", source, "--profile", "zatca", "--format", "json"], sdk.REPO, os.environ.copy(), native_evidence)
        native = json.loads((native_evidence / "stdout.txt").read_text())
        results["qr-total-scale"] = {"input_sha256": sdk.digest(source), "sdk_validation": sdk.parse_validation_report(raw, record["exit_code"]),
            "fatoority_sdk_qr": next(stage for stage in native["stages"] if stage["stage"] == "qr"),
            "fatoority_policy": "SDK-QR-004: equal money; SDK-profile lexical mismatch"}
    after = {str(p.relative_to(root)): sdk.digest(p) for p in root.rglob("*") if p.is_file()}
    if before != after:
        raise sdk.CaptureError("Installed SDK changed")
    sources = ["scripts/qr_parity.py", "fatoora-core/src/invoice/qr.rs", "fatoora-core/src/invoice/sign.rs",
               "fatoora-core/src/invoice/builder.rs", "fatoora-core/src/invoice/xml/parse.rs",
               "fatoora-core/src/invoice/xml/nav.rs", "fatoora-core/src/invoice/validation/integrity.rs", "fatoora-cli/src/main.rs"]
    sdk.write_json(output / "report.json", {"sdk_version": sdk.VERSION, "jar_sha256": sdk.JAR_SHA256,
                   "run_utc": datetime.now(timezone.utc).isoformat(),
                   "sources": {name: sdk.digest(sdk.REPO / name) for name in sources},
                   "results": results, "installed_sdk_unchanged": True})


if __name__ == "__main__":
    main()
