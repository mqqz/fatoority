from pathlib import Path
from fatoora import SignedInvoice

with SignedInvoice.from_file("signed.xml") as invoice:
    payload = invoice.qr_code
Path("qr.txt").write_text(payload, encoding="utf-8")
print(payload)
