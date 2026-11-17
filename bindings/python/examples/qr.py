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
qr = signed.qr_code
assert qr
# --8<-- [end:example]
