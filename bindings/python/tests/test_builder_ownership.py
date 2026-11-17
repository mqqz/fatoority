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

    reference = OriginalInvoiceRef.new("original", "uuid", date(2023, 11, 13), "10:20:30", "388")
    b = InvoiceBuilder(InvoiceTypeKind(kind), InvoiceSubType.SIMPLIFIED,
                       original_invoice=reference, original_invoice_reason="تصحيح &",
                       id="note", uuid="note-uuid", issue_datetime="2024-01-01T12:30:00Z",
                       currency="SAR", previous_invoice_hash="hash", invoice_counter=1,
                       vat_category=VatCategory.STANDARD)
    reference.close()
    (b.add_payment_means(PaymentMeans.new("10"))
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


@pytest.mark.filterwarnings("ignore::DeprecationWarning")
def test_aggregate_setters_build_document_adjustments():
    # Deprecated in 0.6 (test_deprecations.py asserts the warnings); behaviour kept until 0.7.
    from decimal import Decimal
    from fatoora import Adjustment, VatCategory

    b = builder()
    b.add_line_item("item", 2, "PCE", 50, 15, VatCategory.STANDARD)
    b.set_allowance("Loyalty", "10")
    b.invoice_level_charge(5)
    invoice = b.build()
    totals = invoice.totals
    assert (totals.allowance_total, totals.charge_total) == (Decimal("10"), Decimal("5"))
    assert totals.payable_amount == Decimal("109.25")
    assert invoice.invoice_level_discount == Decimal("10")
    assert invoice.invoice_level_charge == Decimal("5")
    assert invoice.allowance_reason == "Loyalty"
    assert [a.charge for a in invoice.adjustments] == [False, True]

    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.allowance_reason("reason only")
    assert b.build().totals.allowance_total == 0

    with pytest.raises(FfiError):
        builder().invoice_level_discount("NaN")

    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.invoice_level_discount(1)
    b.add_document_adjustment(False, Adjustment.amount(1), VatCategory.STANDARD, 15)
    with pytest.raises(ValueError):
        b.build()

    # A failed explicit add leaves the builder free to use the aggregate setters.
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    with pytest.raises(FfiError):
        b.add_document_adjustment(False, Adjustment.amount(1), VatCategory.STANDARD, "NaN")
    b.invoice_level_discount(1)
    assert b.build().totals.allowance_total == Decimal("1")

    # A rejected aggregate reason keeps the pending discount for a corrected retry.
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.set_allowance("bad\0reason", 10)
    with pytest.raises((FfiError, ValueError)):
        b.build()
    b.allowance_reason("corrected")
    invoice = b.build()
    assert invoice.totals.allowance_total == Decimal("10")
    assert invoice.allowance_reason == "corrected"

    # Prepayment adjustment lines (document type 386) don't count towards the
    # inferred rate, matching core totals.
    from fatoora import DocumentReference
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.add_line_item("prepayment", 0, "PCE", 0, 5, VatCategory.STANDARD,
                    document_references=(DocumentReference("46531", issue_date="2024-01-01", issue_time="12:00:00", document_type_code="386"),))
    b.set_allowance("Loyalty", "10")
    invoice = b.build()
    assert [(a.category, a.rate) for a in invoice.adjustments] == [(VatCategory.STANDARD, Decimal("15"))]

    b = builder()
    b.add_line_item("a", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.add_line_item("b", 1, "PCE", 100, 5, VatCategory.STANDARD)
    b.invoice_level_charge(1)
    with pytest.raises(ValueError):
        b.build()


def test_request_id_is_none_when_absent():
    from fatoora import ComplianceCsid, ProductionCsid, Environment

    for cls in (ComplianceCsid, ProductionCsid):
        assert cls.new(Environment.SIMULATION, "token", "secret").request_id is None
        assert cls.new(Environment.SIMULATION, "token", "secret", request_id="42").request_id == "42"
