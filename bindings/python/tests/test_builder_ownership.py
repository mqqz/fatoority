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
    assert invoice.id == "numeric"
    with pytest.raises(FfiError):
        b.set_id("after build")
    b.close()
    invoice.close()


@pytest.mark.parametrize("kind", [2, 3])
def test_full_billing_reference_and_language_less_note(kind):
    from datetime import date
    from fatoora import (FinalizedInvoice, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                         OriginalInvoiceRef, PaymentMeans, VatCategory)

    reference = OriginalInvoiceRef("original", "uuid", date(2023, 11, 13), "10:20:30", "388")
    b = InvoiceBuilder(InvoiceTypeKind(kind), InvoiceSubType.SIMPLIFIED,
                       original_invoice=reference, original_invoice_reason="تصحيح &",
                       id="note", uuid="note-uuid", issue_datetime="2024-01-01T12:30:00Z",
                       currency="SAR", previous_invoice_hash="hash", invoice_counter=1,
                       vat_category=VatCategory.STANDARD)
    (b.add_payment_means(PaymentMeans("10"))
      .set_seller("Acme", country_code="SA", city="Riyadh", street="King Fahd",
                  building_number="1234", postal_code="12222", vat_id="399999999900003")
      .add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
      .add_note("old", language="en")
      .add_note("ملاحظة &"))
    invoice = b.build()
    with pytest.raises(FfiError):
        b.add_note("consumed")
    parsed = FinalizedInvoice.from_xml(invoice.xml)
    restored = parsed.original_invoice_ref
    assert parsed.notes[0].text == "old"
    note = parsed.notes[1]
    assert parsed.original_invoice_reason == "تصحيح &"
    parsed.close()
    assert restored.issue_date == date(2023, 11, 13)
    assert restored.issue_time == "10:20:30"
    assert restored.document_type_code == "388"
    assert note.language_id is None
    assert note.text == "ملاحظة &"


def test_request_id_is_none_when_absent():
    from fatoora import ComplianceCsid, ProductionCsid, Environment

    for cls in (ComplianceCsid, ProductionCsid):
        assert cls(Environment.SIMULATION, token="token", secret="secret").request_id is None
        assert cls(Environment.SIMULATION, token="token", secret="secret", request_id="42").request_id == "42"
