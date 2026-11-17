from __future__ import annotations

from datetime import datetime, timezone
from uuid import UUID

from fatoora import (
    Address,
    InvoiceBuilder,
    InvoiceSubType,
    InvoiceTypeKind,
    PaymentMeans,
    VatCategory,
)


def main() -> None:
    builder = InvoiceBuilder(
        InvoiceTypeKind.TAX,
        InvoiceSubType.SIMPLIFIED,
        id="INV-1",
        uuid=UUID("123e4567-e89b-12d3-a456-426614174000"),
        issue_datetime=datetime(2023, 11, 14, 22, 13, 20, tzinfo=timezone.utc),
        currency="SAR",
        previous_invoice_hash="hash",
        invoice_counter=1,
        vat_category=VatCategory.STANDARD,
    )
    address = Address("SAU", "Riyadh", "King Fahd", "1234", "12222")
    invoice = (
        builder.set_seller("Acme Inc", address=address, vat_id="399999999900003")
        .add_payment_means(PaymentMeans("10"))
        .add_line_item(
            description="Item",
            quantity="1.0",
            unit_code="PCE",
            unit_price="100.0",
            vat_rate="15.0",
            vat_category=VatCategory.STANDARD,
        )
        .build()
    )
    print(invoice.xml)
    print(invoice.issue_datetime.isoformat(), invoice.hash())


if __name__ == "__main__":
    main()
