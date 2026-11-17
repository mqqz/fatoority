from pathlib import Path
import sys
from fatoora import validate_xml, validate_zatca

xml = Path("invoice.xml").read_text(encoding="utf-8")
if "--zatca" in sys.argv:
    report = validate_zatca(
        xml, previous_invoice_hash=Path("previous-hash.txt").read_text().strip(),
    )
    for finding in report.findings:
        print(finding.severity.value, finding.code, finding.message)
    print(report.to_json())
    sys.exit(0 if report.is_valid else 1)
validate_xml(xml)
print("XSD valid")
