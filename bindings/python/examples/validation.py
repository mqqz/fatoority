from pathlib import Path

invoice_xml_path = (
    Path(__file__).resolve().parents[3]
    / "fatoora-core"
    / "tests"
    / "fixtures"
    / "invoices"
    / "sample-simplified-invoice.xml"
)

# --8<-- [start:example]
from fatoora import validate_xml

# invoice_xml_path = "path/to/invoice.xml"
xml = invoice_xml_path.read_text(encoding="utf-8")

result = validate_xml(xml)
assert result is True
# --8<-- [end:example]
