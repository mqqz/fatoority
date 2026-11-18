"""The 0.6 value models: frozen dataclasses read once from an invoice and accepted as input."""
from __future__ import annotations

import dataclasses
from datetime import date
from decimal import Decimal

import pytest

from fatoora import (Address, Adjustment, FinalizedInvoice, Identifier, InvoiceBuilder,
                     InvoiceLineItem, InvoiceNote, InvoiceSubType, InvoiceTypeKind,
                     OriginalInvoiceRef, Party, PaymentMeans, TaxExemption, VatCategory)
from fatoora.errors import FatooraError, InvalidInputError

ADDRESS = Address("SA", "Riyadh", "King Fahd", "1234", "12222")
SELLER = Party("Acme", ADDRESS, vat_id="399999999900003",
               identifiers=(Identifier("7003339333", "CRN"), Identifier("extra", "OTH")))


def header(**kwargs) -> InvoiceBuilder:
    values = dict(id="INV-1", uuid="uuid-1", issue_datetime="2024-01-01T12:30:00Z", currency="SAR",
                  previous_invoice_hash="hash", invoice_counter=1, vat_category=VatCategory.STANDARD)
    values.update(kwargs)
    return InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.STANDARD, **values)


def test_models_are_frozen_hashable_values():
    for value in (ADDRESS, SELLER, Identifier("1"), InvoiceNote("hi"), OriginalInvoiceRef("1"),
                  PaymentMeans("10"), Adjustment(1)):
        assert dataclasses.is_dataclass(value)
        with pytest.raises(dataclasses.FrozenInstanceError):
            value.foo = 1  # type: ignore[attr-defined]
        assert value == dataclasses.replace(value)
        hash(value)


def test_constructors_validate_and_normalize():
    assert ADDRESS.country_code == "SAU"
    assert ADDRESS.with_province("Riyadh") == dataclasses.replace(ADDRESS, province="Riyadh")
    with pytest.raises(FatooraError):
        Address("XX", "Riyadh", "King Fahd", "1234", "12222")
    assert SELLER.vat_id == Identifier("399999999900003")
    assert SELLER.other_id == Identifier("7003339333", "CRN")
    assert Party("Buyer").other_id is None
    with pytest.raises(TypeError):
        Party("Acme", identifiers="7003339333")
    reference = OriginalInvoiceRef("INV-0", "uuid", "2023-11-13", "10:20:30", "388")
    assert reference.issue_date == date(2023, 11, 13)
    assert reference == OriginalInvoiceRef("INV-0", "uuid", date(2023, 11, 13), "10:20:30", "388")
    with pytest.raises(FatooraError):
        OriginalInvoiceRef("INV-0", issue_date="13/11/2023")
    with pytest.raises(TypeError):
        PaymentMeans("30", "one note")
    means = PaymentMeans("30").with_instruction_note("a").with_instruction_note("b").with_account("SA01")
    assert means == PaymentMeans("30", ("a", "b"), "SA01") and means.instruction_note == "a"
    assert InvoiceNote("hi").language == "en" and InvoiceNote("مرحبا", "ar").language == "ar"


def test_adjustment_fields_and_percent_of():
    assert Adjustment(10) == Adjustment("10") == Adjustment(Decimal("10.0"))
    percent = Adjustment.percent_of(250, 10, reason="discount")
    assert (percent.amount, percent.base_amount, percent.percentage) == (Decimal("25"), 250, 10)
    assert percent.with_reason(code="95") == dataclasses.replace(percent, reason_code="95")
    with pytest.raises(ValueError):
        Adjustment(1, base_amount=100)
    with pytest.raises(TypeError):
        Adjustment(0.5)
    with pytest.raises(ValueError):
        Adjustment("NaN")
    with pytest.raises(TypeError):
        Adjustment(1, charge=1)


def test_line_item_input_calculates_amounts():
    item = InvoiceLineItem("Item", "PCE", 2, "50", vat_rate=15, vat_category=VatCategory.STANDARD,
                           adjustments=(Adjustment(10, reason="discount"),),
                           item_seller_identifier=Identifier("SKU"))
    assert item.total_amount is None and item.vat_amount is None
    with pytest.raises(TypeError, match="vat_category"):
        InvoiceLineItem("Item", "PCE", 1, 1, vat_rate=15)
    invoice = (header().set_seller(SELLER).add_payment_means(PaymentMeans("10"))
               .add_item(item).build())
    built = invoice.line_item(0)
    assert (built.total_amount, built.vat_amount) == (Decimal("90"), Decimal("13.50"))
    assert built.adjustments == (Adjustment(10, reason="discount", category=VatCategory.STANDARD, rate=15),)
    assert built.item_seller_identifier == Identifier("SKU")
    with pytest.raises(InvalidInputError):
        invoice.line_item(1)
    with pytest.raises(FatooraError):
        header().add_item(dataclasses.replace(item, total_amount=Decimal("1")))


def test_values_read_from_an_invoice_rebuild_it():
    buyer = Party("Buyer", Address("SA", "Jeddah", "Street", "1", "22222", district="Al Hamra"),
                  identifiers=(Identifier("1", "NAT"),))
    original = (header(seller=SELLER, buyer=buyer)
                .add_payment_means(PaymentMeans("30", ("first", "second"), "SA01", "net 30"))
                .add_note("note", language="en")
                .add_billing_reference(OriginalInvoiceRef("INV-0", issue_date=date(2023, 11, 13)))
                .add_line_item("Item", 1, "PCE", 100, 15, VatCategory.STANDARD)
                .add_charge(Adjustment(5, reason="freight"), VatCategory.STANDARD, 15)
                .build())
    parsed = FinalizedInvoice.from_xml(original.xml)
    assert parsed.seller == SELLER and parsed.buyer == buyer
    assert parsed.payment_means == [PaymentMeans("30", ("first", "second"), "SA01", "net 30")]
    assert parsed.notes == [InvoiceNote("note", "en")]
    assert parsed.billing_references == [OriginalInvoiceRef("INV-0", issue_date=date(2023, 11, 13))]
    assert parsed.adjustments == [Adjustment(5, reason="freight", charge=True,
                                             category=VatCategory.STANDARD, rate=15)]

    rebuilt = header(seller=parsed.seller, buyer=parsed.buyer)
    for means in parsed.payment_means:
        rebuilt.add_payment_means(means)
    for note in parsed.notes:
        rebuilt.add_note(note.text, language=note.language_id)
    for reference in parsed.billing_references:
        rebuilt.add_billing_reference(reference)
    for item in parsed.line_items:
        rebuilt.add_item(item)
    for adjustment in parsed.adjustments:
        rebuilt.add_adjustment(adjustment)
    assert rebuilt.build().xml == original.xml


def test_party_setters():
    invoice = header().set_seller(SELLER).set_buyer(Party("Walk-in")).add_payment_means(
        PaymentMeans("10")).add_line_item("Item", 1, "PCE", 100, 15, VatCategory.STANDARD).build()
    assert invoice.buyer == Party("Walk-in")
    with pytest.raises(TypeError, match="not both"):
        header().set_seller(SELLER, vat_id="399999999900003")
    with pytest.raises(ValueError, match="address"):
        header().set_seller(Party("Acme", vat_id="399999999900003"))


def test_document_adjustment_category_from_the_value():
    entry = Adjustment(10, category=VatCategory.EXEMPT, rate=0,
                       exemption=TaxExemption("VATEX-SA-29", "exempt"))
    invoice = (header(vat_category=VatCategory.EXEMPT).set_seller(SELLER)
               .add_payment_means(PaymentMeans("10"))
               .add_line_item("Item", 1, "PCE", 100, 0, VatCategory.EXEMPT,
                              exemption=TaxExemption("VATEX-SA-29", "exempt"))
               .add_tax_exemption(VatCategory.EXEMPT, 0, TaxExemption("VATEX-SA-29", "exempt"))
               .add_allowance(entry).build())
    assert invoice.adjustments == [entry]
    with pytest.raises(TypeError, match="category and rate"):
        header().add_charge(Adjustment(1))
