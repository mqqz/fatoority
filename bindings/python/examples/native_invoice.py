"""Construct an invoice with exact decimals through the generated bindings."""
from decimal import Decimal
from fatoora import InvoiceBuilder, InvoiceTypeKind, InvoiceSubType, PaymentMeans, VatCategory

builder = (
    InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
    .set_id("INV-1")
    .set_uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
    .set_issue_datetime("2024-01-01T12:30:00Z")
    .set_currency("SAR")
    .set_previous_invoice_hash("hash")
    .set_invoice_counter(1)
    .add_payment_means(PaymentMeans("10"))
    .set_vat_category(VatCategory.STANDARD)
    .set_seller("شركة الاختبار", country_code="SA", city="Riyadh", street="King Fahd",
                building_number="1234", postal_code="12222", vat_id="399999999900003")
    .add_line_item("خدمة", 3, "PCE", Decimal("0.3333"), 15, VatCategory.STANDARD)
)
xml = builder.build().xml
assert ">1.15</cbc:TaxInclusiveAmount>" in xml
print(xml)
