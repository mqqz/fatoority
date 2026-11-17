from pathlib import Path

signed_xml_path = (
    Path(__file__).resolve().parents[3]
    / "fatoora-core"
    / "tests"
    / "fixtures"
    / "invoices"
    / "sample-simplified-invoice.xml"
)

# --8<-- [start:example]
from fatoora import SignedInvoice

# signed_xml_path = "path/to/signed_invoice.xml"
signed_xml = signed_xml_path.read_text(encoding="utf-8")

signed = SignedInvoice.from_xml(signed_xml)

stored_hash = signed.invoice_hash  # stored in the signature
assert signed.hash() == stored_hash  # recomputed from the XML
qr = signed.qr_code
signing_time = signed.signing_time  # timezone-aware datetime (UTC)
assert signing_time.tzinfo is not None
assert qr
# --8<-- [end:example]
