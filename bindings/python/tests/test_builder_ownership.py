import pytest

from fatoora.errors import FfiError
from test_numeric_contract import builder


def test_failed_setter_can_be_corrected_and_build_consumes():
    from fatoora import VatCategory

    b = builder()
    with pytest.raises(FfiError):
        b.set_currency("invalid")
    b.set_currency("SAR")
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    invoice = b.build()
    assert invoice.id() == "numeric"
    with pytest.raises(FfiError):
        b.set_id("after build")
    b.close()
    invoice.close()


@pytest.mark.parametrize("kind", [2, 3])
def test_full_billing_reference_and_language_less_note(kind):
    from fatoora import (InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                         OriginalInvoiceRef, VatCategory, parse_finalized_invoice_xml)

    reference = OriginalInvoiceRef.new("original", "uuid", "2023-11-13", "10:20:30", "388")
    b = InvoiceBuilder.new_with_reference(InvoiceTypeKind(kind), InvoiceSubType.SIMPLIFIED,
                                         reference, "تصحيح &")
    reference.close()
    b.set_id("note")
    b.set_uuid("note-uuid")
    b.set_issue_datetime("2024-01-01T12:30:00Z")
    b.set_currency("SAR")
    b.set_previous_invoice_hash("hash")
    b.set_invoice_counter(1)
    b.set_payment_means_code("10")
    b.set_vat_category(VatCategory.STANDARD)
    b.set_seller(name="Acme", country_code="SA", city="Riyadh", street="King Fahd",
                 building_number="1234", postal_code="12222", vat_id="399999999900003")
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.set_note("en", "old")
    b.set_note_without_language("ملاحظة &")
    invoice = b.build()
    with pytest.raises(FfiError):
        b.set_note_without_language("consumed")
    parsed = parse_finalized_invoice_xml(invoice.xml())
    restored = parsed.original_invoice_ref()
    assert parsed.notes()[0].text() == "old"
    note = parsed.notes()[1]
    assert parsed.original_invoice_reason() == "تصحيح &"
    parsed.close()
    assert restored.issue_time() == "10:20:30"
    assert restored.document_type_code() == "388"
    assert note.language_id() is None
    assert note.text() == "ملاحظة &"
