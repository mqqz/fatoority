from pathlib import Path
from fatoora import InvoiceBuilder, InvoiceSubType, InvoiceTypeKind, VatCategory

builder = InvoiceBuilder.new(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
builder.set_id("INV-1001")
builder.set_uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
builder.set_issue_datetime("2026-10-26T09:00:00Z")
builder.set_currency("SAR")
# Demonstration predecessor; use your own invoice history in production.
builder.set_previous_invoice_hash(
    "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ=="
)
builder.set_invoice_counter(1)
builder.set_payment_means_code("10")
builder.set_vat_category(VatCategory.STANDARD)
builder.set_seller(
    name="Example Store", country_code="SA", city="Riyadh",
    street="King Fahd Road", building_number="1234", postal_code="12222",
    district="Olaya", additional_number="5678", vat_id="399999999900003",
    other_id="7003339333", other_id_scheme="CRN",
)
builder.add_line_item(
    description="Notebook", quantity="2", unit_code="PCE", unit_price="50",
    vat_rate="15", vat_category=VatCategory.STANDARD,
)
invoice = builder.build()
Path("invoice.xml").write_text(invoice.to_xml(), encoding="utf-8")
totals = invoice.totals()
print(f"VAT: {totals.tax_amount} SAR")
print(f"Payable: {totals.payable_amount} SAR")
print("Saved invoice.xml")
