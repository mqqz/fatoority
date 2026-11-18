"""0.6 Pythonic surface: properties, keyword constructor, chaining, datetime/UUID values."""
from __future__ import annotations

from datetime import date, datetime, timedelta, timezone
from decimal import Decimal
from pathlib import Path
import uuid

import pytest

from fatoora import (Address, Adjustment, ComplianceCsid, Config, CsrProperties, Environment,
                     FinalizedInvoice, InvoiceBuilder, InvoiceFlag, InvoiceSigner, InvoiceSubType,
                     InvoiceTypeKind, LineItemBuilder, OriginalInvoiceRef, PaymentMeans,
                     ProductionCsid, SignedInvoice, SigningKey, TaxExemption, VatCategory,
                     ZatcaClient, hash_xml)
from fatoora.errors import FatooraError

FIXTURES = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures"
SIGNED_XML = FIXTURES / "invoices/sample-simplified-invoice.xml"
CSR_CONFIG = FIXTURES / "csr-configs/csr-config-example-EN.properties"
CREDENTIALS = FIXTURES / "sdk-parity/credentials"
UUID = uuid.UUID("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
ISSUED = datetime(2024, 1, 1, 15, 30, tzinfo=timezone(timedelta(hours=3)))
SELLER_ADDRESS = dict(country_code="SA", city="Riyadh", street="King Fahd",
                      building_number="1234", postal_code="12222")


def header(**overrides):
    fields = dict(id="INV-7", uuid=UUID, issue_datetime=ISSUED, currency="SAR",
                  previous_invoice_hash="hash", invoice_counter=7,
                  vat_category=VatCategory.STANDARD)
    fields.update(overrides)
    return InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED, **fields)


def complete(builder: InvoiceBuilder) -> InvoiceBuilder:
    return (builder.set_seller("Acme", vat_id="399999999900003", **SELLER_ADDRESS)
            .add_payment_means(PaymentMeans("10"))
            .add_line_item("item", 2, "PCE", 50, 15, VatCategory.STANDARD))


def test_constructor_keywords_and_chained_setters_build_an_invoice():
    invoice = complete(header()).build()
    assert invoice.id == "INV-7"
    assert invoice.uuid == str(UUID)
    assert invoice.invoice_counter == 7
    assert invoice.currency == "SAR"
    assert invoice.vat_category is VatCategory.STANDARD
    assert invoice.totals.payable_amount == Decimal("115.00")


def test_every_builder_setter_returns_the_builder():
    b = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
    chained = (b.set_id("x").set_uuid(UUID).set_issue_datetime(ISSUED).set_currency("SAR")
               .set_previous_invoice_hash("hash").set_invoice_counter(1)
               .set_vat_category(VatCategory.STANDARD).set_flags(InvoiceFlag.EXPORT)
               .set_delivery(date(2024, 1, 1)).set_payment_instruction("pay")
               .set_purchase_order_id("PO").set_contract_id("C").add_note("n")
               .add_payment_means(PaymentMeans("10"))
               .set_seller("Acme", vat_id="399999999900003", **SELLER_ADDRESS)
               .add_seller_identifier("7003339333", "CRN")
               .set_buyer_without_address("Buyer").add_buyer_identifier("1", "NAT")
               .add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
               .add_allowance(Adjustment(1).with_reason("discount"), VatCategory.STANDARD, 15)
               .add_charge(Adjustment(2).with_reason("fee"), VatCategory.STANDARD, 15)
               .set_prepaid_amount(0).set_payable_rounding_amount(0))
    assert chained is b
    line = LineItemBuilder()
    assert line.set_item_name("x").set_invoiced_quantity(1).set_item_net_price(1) is line
    invoice = b.build()
    assert [(a.charge, a.amount) for a in invoice.adjustments] == [(False, Decimal("1")), (True, Decimal("2"))]
    assert invoice.flags == {InvoiceFlag.EXPORT}


def test_getters_are_properties_returning_plain_values():
    for cls, name in [(FinalizedInvoice, "id"), (SignedInvoice, "qr_code"),
                      (Config, "env"), (ProductionCsid, "secret")]:
        assert isinstance(getattr(cls, name), property)
    address = Address("SA", "Riyadh", "King Fahd", "1234", "12222")
    assert type(address.city) is str and address.district is None
    with pytest.raises(AttributeError):
        address.city = "Jeddah"
    invoice = complete(header()).build()
    assert type(invoice.line_item_count) is int
    assert invoice.buyer is None
    assert invoice.seller.address.city == "Riyadh"


def test_datetime_uuid_and_date_values():
    b = complete(header()).set_delivery(date(2024, 1, 2), "2024-01-03")
    invoice = b.build()
    assert invoice.issue_datetime == datetime(2024, 1, 1, 12, 30, tzinfo=timezone.utc)
    assert invoice.issue_datetime.tzinfo is not None
    assert invoice.issue_date == date(2024, 1, 1)
    assert invoice.actual_delivery_date == date(2024, 1, 2)
    assert invoice.latest_delivery_date == date(2024, 1, 3)
    signed = SignedInvoice.from_file(SIGNED_XML)
    assert signed.signing_time == datetime(2025, 7, 22, 15, 51, 28, tzinfo=timezone.utc)
    assert signed.issue_datetime.utcoffset() == timedelta(0)


def test_strings_are_still_accepted():
    invoice = complete(header(uuid=str(UUID), issue_datetime="2024-01-01T12:30:00Z")).build()
    assert invoice.issue_datetime == datetime(2024, 1, 1, 12, 30, tzinfo=timezone.utc)


def test_naive_and_wrongly_typed_values_are_rejected():
    with pytest.raises(ValueError, match="timezone-aware"):
        header(issue_datetime=datetime(2024, 1, 1, 12, 30))
    with pytest.raises(TypeError, match="uuid"):
        header(uuid=12345)
    with pytest.raises(TypeError, match="date"):
        InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED).set_delivery(ISSUED)


def test_original_invoice_reference_in_constructor():
    reference = OriginalInvoiceRef("INV-1", UUID, date(2023, 11, 13))
    assert reference.uuid == str(UUID) and reference.issue_date == date(2023, 11, 13)
    note = complete(InvoiceBuilder(
        InvoiceTypeKind.CREDIT_NOTE, InvoiceSubType.SIMPLIFIED,
        original_invoice=reference, original_invoice_reason="return",
        id="CN-1", uuid=UUID, issue_datetime=ISSUED, currency="SAR",
        previous_invoice_hash="hash", invoice_counter=8, vat_category=VatCategory.STANDARD,
    )).build()
    assert note.invoice_type_kind is InvoiceTypeKind.CREDIT_NOTE
    assert note.original_invoice_ref.id == "INV-1"
    assert note.original_invoice_reason == "return"
    with pytest.raises(TypeError, match="original_invoice_reason"):
        InvoiceBuilder(InvoiceTypeKind.CREDIT_NOTE, InvoiceSubType.SIMPLIFIED, original_invoice=reference)


def test_party_setters_accept_an_address():
    address = Address("SA", "Riyadh", "King Fahd", "1234", "12222", district="Olaya").with_province("Riyadh")
    b = header().set_seller("Acme", vat_id="399999999900003", address=address)
    b.set_buyer("Buyer", address=address)
    invoice = b.add_payment_means(PaymentMeans("10")).add_line_item(
        "item", 1, "PCE", 100, 15, VatCategory.STANDARD).build()
    assert address.city == "Riyadh"  # borrowed, not consumed
    assert invoice.seller.address.district == "Olaya"
    assert invoice.buyer.address.province == "Riyadh"


@pytest.mark.parametrize("call,match", [
    (lambda b, a: b.set_seller("Acme", vat_id="1", address=a, city="Jeddah"), "not both"),
    (lambda b, a: b.set_buyer("Buyer", address=a, province="Riyadh"), "not both"),
    (lambda b, a: b.set_seller("Acme", vat_id="1", country_code="SA"), "missing required"),
    (lambda b, a: b.set_buyer("Buyer", address="Riyadh"), "must be an Address"),
    (lambda b, a: b.set_seller("Acme", **SELLER_ADDRESS), "vat_id"),
])
def test_party_setter_argument_errors(call, match):
    address = Address("SA", "Riyadh", "King Fahd", "1234", "12222")
    with pytest.raises(TypeError, match=match):
        call(InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED), address)


def test_add_note_language_is_keyword_only():
    invoice = complete(header()).add_note("hello").add_note("مرحبا", language="ar").build()
    assert [(n.language_id, n.text) for n in invoice.notes] == [(None, "hello"), ("ar", "مرحبا")]


def test_document_adjustments_and_tax_exemptions():
    exemption = TaxExemption("VATEX-SA-29", "exempt")
    b = header(vat_category=VatCategory.EXEMPT).set_seller("Acme", vat_id="399999999900003", **SELLER_ADDRESS)
    b.add_payment_means(PaymentMeans("10")).add_line_item(
        "item", 1, "PCE", 100, 0, VatCategory.EXEMPT, exemption=exemption)
    b.add_tax_exemption(VatCategory.EXEMPT, 0, exemption)
    b.add_allowance(Adjustment(10).with_reason("discount"), VatCategory.EXEMPT, 0, exemption)
    invoice = b.build()
    assert invoice.totals.allowance_total == Decimal("10")
    assert invoice.adjustments[0].exemption == exemption


def test_parsing_classmethods_and_hashes():
    xml = SIGNED_XML.read_text()
    signed = SignedInvoice.from_xml(xml)
    assert SignedInvoice.from_file(SIGNED_XML).id == signed.id
    assert signed.hash() == signed.invoice_hash == hash_xml(xml)
    invoice = complete(header()).build()
    parsed = FinalizedInvoice.from_xml(invoice.xml)
    assert isinstance(parsed, FinalizedInvoice) and parsed.hash() == invoice.hash()
    with pytest.raises(FatooraError):
        SignedInvoice.from_xml(invoice.xml)


def test_invoice_signer_signs_finalized_invoices():
    signer = InvoiceSigner.from_der((CREDENTIALS / "certificate.der").read_bytes(),
                                    (CREDENTIALS / "private-key.der").read_bytes())
    assert "BEGIN CERTIFICATE" in signer.certificate_pem
    invoice = complete(header()).build()
    invoice_id = invoice.id  # caches the invoice data
    signed = invoice.sign(signer)
    with pytest.raises(FatooraError, match="consumed"):
        invoice.id  # signing consumed the finalized invoice, cached data included
    assert signed.id == invoice_id
    assert isinstance(signed, SignedInvoice)
    assert signed.hash() == signed.invoice_hash
    assert SignedInvoice.from_xml(signed.xml).qr_code == signed.qr_code


def test_csr_properties_from_str_and_file():
    text = CSR_CONFIG.read_text()
    for props in (CsrProperties.from_str(text), CsrProperties.from_file(CSR_CONFIG)):
        csr = props.build(SigningKey.generate(), Environment.NON_PRODUCTION)
        assert "TST-886431145-399999999900003" in csr.subject_string


def test_client_request_methods_use_the_core_names(monkeypatch):
    client = ZatcaClient(Config(Environment.NON_PRODUCTION))
    calls = []

    def fake(name, *args):
        calls.append(name)
        return object()

    monkeypatch.setattr(client, "_invoke", fake)
    assert isinstance(client.request_compliance_csid(None, "123456"), ComplianceCsid)
    assert isinstance(client.request_production_csid(None), ProductionCsid)
    assert calls == ["_blocking_request_compliance_csid", "_blocking_request_production_csid"]


def test_default_evaluation_clock_parses_on_every_python():
    from fatoora import validate_zatca

    # The default clock snapshot carries nanoseconds; datetime keeps microseconds.
    report = validate_zatca("<wrong/>")
    assert report.evaluated_at.utcoffset() == timedelta(0)
    assert abs(datetime.now(timezone.utc) - report.evaluated_at) < timedelta(minutes=5)


@pytest.mark.parametrize("timeout,expected", [
    (30, 30_000), (2.5, 2_500), (0.0001, 1), (timedelta(seconds=1, microseconds=500), 1_001),
    (timedelta(days=49), 49 * 86_400_000),
])
def test_client_timeout_accepts_seconds_and_timedelta(monkeypatch, timeout, expected):
    from fatoora import api, _native

    seen = []
    real = _native.ZatcaClient.with_timeout
    monkeypatch.setattr(_native.ZatcaClient, "with_timeout",
                        lambda config, ms: seen.append(ms) or real(config, ms))
    with ZatcaClient(Config(Environment.NON_PRODUCTION), timeout=timeout) as client:
        assert isinstance(client, api.ZatcaClient)
    assert seen == [expected]


@pytest.mark.parametrize("timeout,error", [
    (0, ValueError), (-1.0, ValueError), (float("nan"), ValueError), (float("inf"), ValueError),
    (timedelta(0), ValueError), (timedelta(days=50), ValueError),
    (True, TypeError), ("30", TypeError), (Decimal("30"), TypeError),
])
def test_client_timeout_rejects_invalid_values(timeout, error):
    with pytest.raises(error, match="timeout"):
        ZatcaClient(Config(Environment.NON_PRODUCTION), timeout=timeout)


def test_native_zero_timeout_is_invalid_input():
    # The facade rejects zero first; the native constructor classifies it the same way.
    from fatoora import api, _native
    from fatoora.errors import InvalidInputError

    with pytest.raises(InvalidInputError, match="timeout must be greater than zero") as caught:
        api._call(_native.ZatcaClient.with_timeout, Config(Environment.NON_PRODUCTION), 0)
    assert caught.value.details["type"] == "invalid_timeout"


def test_client_timeout_is_keyword_only():
    with pytest.raises(TypeError):
        ZatcaClient(Config(Environment.NON_PRODUCTION), 30)
    with ZatcaClient(Config(Environment.NON_PRODUCTION), timeout=None):
        pass
