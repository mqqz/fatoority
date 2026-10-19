#!/usr/bin/env python3
"""Explicitly authorized developer-portal probes; never targets production.

Issues one disposable compliance credential and submits the reviewed 15-case set.
Keys, authentication tokens and secrets stay temporary and are never logged.
"""
import argparse
import base64
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import urllib.error
import urllib.request
import uuid
import xml.etree.ElementTree as ET

import qr_parity as qr

REPO = Path(__file__).resolve().parents[1]
BASE = "https://gw-fatoora.zatca.gov.sa/e-invoicing/developer-portal/"


def post(path, payload, credentials=None, otp=None):
    headers = {"Accept": "application/json", "Content-Type": "application/json",
               "Accept-Version": "V2", "Accept-Language": "en"}
    if credentials:
        headers["Authorization"] = "Basic " + base64.b64encode(
            f"{credentials['binarySecurityToken']}:{credentials['secret']}".encode()).decode()
    if otp:
        headers["OTP"] = otp
    request = urllib.request.Request(BASE + path, json.dumps(payload).encode(), headers, method="POST")
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            status, raw = response.status, response.read(4 * 1024 * 1024)
    except urllib.error.HTTPError as error:
        status, raw = error.code, error.read(4 * 1024 * 1024)
    try:
        body = json.loads(raw)
    except ValueError:
        body = {"non_json_body": raw.decode(errors="replace")[:4096]}
    return status, body


def fresh(xml):
    # A new test invoice, derived from public fixtures, not a repair of stored XML.
    xml = re.sub(r"<ext:UBLExtensions\b.*?</ext:UBLExtensions>", "", xml, flags=re.S)
    xml = re.sub(r"<cac:Signature\b.*?</cac:Signature>", "", xml, flags=re.S)
    xml = re.sub(r"<cac:AdditionalDocumentReference\b[^>]*>(?:(?!</cac:AdditionalDocumentReference>).)*<cbc:ID>QR</cbc:ID>.*?</cac:AdditionalDocumentReference>", "", xml, flags=re.S)
    now = datetime.now(timezone.utc)
    for field, value in [("IssueDate", f"{now:%Y-%m-%d}"), ("IssueTime", f"{now:%H:%M:%S}"), ("UUID", str(uuid.uuid4()))]:
        xml = re.sub(fr"<cbc:{field}>[^<]*</cbc:{field}>", f"<cbc:{field}>{value}</cbc:{field}>", xml, count=1)
    return xml


def change_tag(xml, tag, value):
    encoded = qr.embedded(xml)
    raw = base64.b64decode(encoded)
    at, fields = 0, {}
    while at < len(raw):
        key, size = raw[at:at + 2]
        fields[key] = raw[at + 2:at + 2 + size]
        at += 2 + size
    if value is None:
        fields.pop(tag, None)
    else:
        fields[tag] = value
    changed = b"".join(bytes([key, len(value)]) + value for key, value in sorted(fields.items()))
    return xml.replace(encoded, base64.b64encode(changed).decode(), 1)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    os.umask(0o077)
    report = {"environment": "developer-portal", "base_url": BASE,
              "run_utc": datetime.now(timezone.utc).isoformat(), "cases": {}}
    cli = REPO / "target/debug/fatoora"
    try:
        with tempfile.TemporaryDirectory(prefix="qr-development-credentials-") as directory:
            private = Path(directory)
            csr, key, cert = private / "request.csr", private / "key.pem", private / "certificate.pem"
            subprocess.run([cli, "csr", "--csr-config", REPO / "fatoora-core/tests/fixtures/csr-configs/csr-config-example-EN.properties",
                            "--private-key", key, "--generated-csr", csr, "--pem"], check=True, capture_output=True)
            status, credentials = post("compliance", {"csr": base64.b64encode(csr.read_bytes()).decode()}, otp="123345")
            report["onboarding"] = {"http_status": status, "issued": bool(credentials.get("binarySecurityToken") and credentials.get("secret"))}
            if status not in (200, 201) or not report["onboarding"]["issued"]:
                report["onboarding"]["response"] = {k: v for k, v in credentials.items() if k not in ("secret", "binarySecurityToken")}
                return
            der = base64.b64decode(base64.b64decode(credentials["binarySecurityToken"]))
            encoded = base64.b64encode(der).decode()
            cert.write_text("-----BEGIN CERTIFICATE-----\n" + "\n".join(encoded[i:i + 64] for i in range(0, len(encoded), 64)) + "\n-----END CERTIFICATE-----\n")

            def sign(name, xml):
                source, signed = output / (name + ".input.xml"), output / (name + ".signed.xml")
                source.write_text(xml)
                result = subprocess.run([cli, "sign", "--invoice", source, "--cert", cert, "--key", key, "--signed-invoice", signed], capture_output=True, text=True)
                if result.returncode:
                    raise RuntimeError("development signing failed: " + result.stderr[:2048])
                return signed.read_text()

            def submit(name, xml, note):
                path = output / (name + ".xml")
                path.write_text(xml)
                root = ET.fromstring(xml)
                digest = subprocess.run([cli, "generate-hash", "--invoice", path], check=True, capture_output=True, text=True).stdout.strip()
                status, body = post("compliance/invoices", {"invoiceHash": digest,
                    "uuid": root.findtext("cbc:UUID", namespaces=qr.NS), "invoice": base64.b64encode(xml.encode()).decode()}, credentials)
                report["cases"][name] = {"endpoint": "compliance/invoices", "http_status": status, "note": note,
                    "invoice_sha256": hashlib.sha256(xml.encode()).hexdigest(), "qr": qr.fields(qr.embedded(xml)), "response": body}
                (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
                print(name, status, body.get("validationResults", {}).get("status"), body.get("reportingStatus"), body.get("clearanceStatus"), flush=True)

            signed_cases = {}
            for subtype in ["simplified", "standard"]:
                for kind in ["invoice", "credit", "debit"]:
                    name = f"{subtype}-{kind}"
                    xml = (REPO / f"fatoora-core/tests/fixtures/sdk-parity/cases/{name}/input.xml").read_text()
                    signed_cases[name] = sign(name, fresh(xml))
                    submit(name, signed_cases[name], "baseline with subtype-correct tags")
            simplified = signed_cases["simplified-invoice"]
            for name, tag, value, note in [
                ("tag4-equivalent-scale", 4, b"115.0", "XML 115.00, QR 115.0"),
                ("tag4-equivalent-integer", 4, b"115", "XML 115.00, QR 115"),
                ("tag5-equivalent-scale", 5, b"15.0", "XML 15.00, QR 15.0"),
                ("tag4-wrong-value", 4, b"116.00", "negative control: wrong monetary value"),
                ("tag5-wrong-value", 5, b"16.00", "negative control: wrong monetary value"),
                ("simplified-missing-tag9", 9, None, "negative control: missing required Tag 9"),
            ]:
                submit(name, change_tag(simplified, tag, value), note)
            stamp = base64.b64decode(qr.fields(qr.embedded(simplified))["tag9_base64"])
            submit("standard-unexpected-tag9", change_tag(signed_cases["standard-invoice"], 9, stamp), "negative control: prohibited standard Tag 9")
            source = REPO / "fatoora-core/tests/fixtures/invoices/Standard/Invoice/Advance Payment adjustments with foreign currency invoice.xml"
            foreign = sign("foreign-currency", fresh(source.read_text()))
            submit("foreign-currency-bt110", foreign, "Tag 5 is 285 USD (BT-110)")
            submit("foreign-currency-bt111", change_tag(foreign, 5, b"1068.75"), "same XML, Tag 5 changed to SDK's 1068.75 SAR (BT-111)")
    except Exception as error:
        report["execution_error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
