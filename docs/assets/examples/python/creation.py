from datetime import datetime, timezone
from pathlib import Path
from uuid import UUID
from fatoora import (Address, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                     PaymentMeans, VatCategory)

seller_address = Address(
    "SA", "Riyadh", "King Fahd Road", "1234", "12222",
    additional_number="5678", district="Olaya",
)
invoice = (
    InvoiceBuilder(
        InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
        id="INV-1001",
        uuid=UUID("8e6000cf-1a98-4174-b3e7-b5d5954bc10d"),
        issue_datetime=datetime(2026, 10, 26, 9, 0, tzinfo=timezone.utc),
        currency="SAR",
        # Demonstration predecessor; use your own invoice history in production.
        previous_invoice_hash="NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==",
        invoice_counter=1,
        vat_category=VatCategory.STANDARD,
    )
    .set_seller("Example Store", address=seller_address, vat_id="399999999900003",
                other_id="7003339333", other_id_scheme="CRN")
    .add_payment_means(PaymentMeans("10"))
    .add_line_item(description="Notebook", quantity="2", unit_code="PCE", unit_price="50",
                   vat_rate="15", vat_category=VatCategory.STANDARD)
    .build()
)
Path("invoice.xml").write_text(invoice.xml, encoding="utf-8")
totals = invoice.totals
print(f"VAT: {totals.tax_amount} SAR")
print(f"Payable: {totals.payable_amount} SAR")
print("Saved invoice.xml")
