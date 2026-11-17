"""Every 0.5 name deprecated in 0.6 still works, warns at the caller, and names its replacement."""
from __future__ import annotations

from datetime import date, datetime, timezone
from decimal import Decimal
import json
from pathlib import Path
import warnings

import pytest

import fatoora
from fatoora import (Adjustment, Config, CsrProperties, Environment, FinalizedInvoice,
                     InvoiceBuilder, InvoiceSigner, InvoiceSubType, InvoiceTypeKind,
                     LineItemBuilder, OriginalInvoiceRef, PaymentMeans, SignedInvoice,
                     TaxExemption, VatCategory, ZatcaClient, ComplianceCsid, ProductionCsid)
from test_numeric_contract import builder

FIXTURES = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures"
SIGNED_XML = FIXTURES / "invoices/sample-simplified-invoice.xml"
CSR_CONFIG = FIXTURES / "csr-configs/csr-config-example-EN.properties"
CONTRACT = json.loads(Path(__file__).with_name("public_api.json").read_text())


def built():
    return builder().add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD).build()


def signed():
    return SignedInvoice.from_file(SIGNED_XML)


def fake_client(monkeypatch):
    client = ZatcaClient(Config(Environment.NON_PRODUCTION))
    monkeypatch.setattr(client, "_invoke", lambda *args: object())
    return client


# name in public_api.json "__deprecated__" -> (call, replacement named in the message)
CASES = {
    "Signer": (lambda mp: fatoora.Signer, "InvoiceSigner"),
    "CsidCompliance": (lambda mp: fatoora.CsidCompliance, "ComplianceCsid"),
    "CsidProduction": (lambda mp: fatoora.CsidProduction, "ProductionCsid"),
    "parse_finalized_invoice_xml": (lambda mp: fatoora.parse_finalized_invoice_xml(built().xml), "FinalizedInvoice.from_xml"),
    "parse_finalized_invoice_xml_file": (lambda mp: fatoora.parse_finalized_invoice_xml_file(SIGNED_XML), "FinalizedInvoice.from_file"),
    "parse_signed_invoice_xml": (lambda mp: fatoora.parse_signed_invoice_xml(SIGNED_XML.read_text()), "SignedInvoice.from_xml"),
    "parse_signed_invoice_xml_file": (lambda mp: fatoora.parse_signed_invoice_xml_file(SIGNED_XML), "SignedInvoice.from_file"),
    "validate_xml_invoice_from_str": (lambda mp: fatoora.validate_xml_invoice_from_str(Config(), SIGNED_XML.read_text()), "validate_xml"),
    "validate_zatca_invoice_from_str": (lambda mp: fatoora.validate_zatca_invoice_from_str(Config(), "<wrong/>"), "validate_zatca"),
    "invoice_hash_base64_from_xml_str": (lambda mp: fatoora.invoice_hash_base64_from_xml_str(SIGNED_XML.read_text()), "hash_xml"),
    "CsrProperties.from_properties_str": (lambda mp: CsrProperties.from_properties_str(CSR_CONFIG.read_text()), "CsrProperties.from_str"),
    "CsrProperties.parse_csr_config": (lambda mp: CsrProperties.parse_csr_config(CSR_CONFIG.read_text()), "CsrProperties.from_str"),
    "CsrProperties.parse_csr_config_file": (lambda mp: CsrProperties.parse_csr_config_file(CSR_CONFIG), "CsrProperties.from_file"),
    "FinalizedInvoice.hash_base64": (lambda mp: built().hash_base64(), "hash()"),
    "FinalizedInvoice.to_xml": (lambda mp: built().to_xml(), "xml property"),
    "SignedInvoice.hash_base64": (lambda mp: signed().hash_base64(), "hash()"),
    "SignedInvoice.to_xml": (lambda mp: signed().to_xml(), "xml property"),
    "SignedInvoice.into_xml": (lambda mp: signed().into_xml(), "xml property"),
    "InvoiceBuilder.new": (lambda mp: InvoiceBuilder.new(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED), "InvoiceBuilder(invoice_type"),
    "InvoiceBuilder.new_with_reference": (lambda mp: InvoiceBuilder.new_with_reference(
        InvoiceTypeKind.CREDIT_NOTE, InvoiceSubType.SIMPLIFIED, OriginalInvoiceRef.new("1"), "r"), "original_invoice="),
    "InvoiceBuilder.set_note": (lambda mp: builder().set_note("en", "hello"), "add_note(text, language=...)"),
    "InvoiceBuilder.set_note_without_language": (lambda mp: builder().set_note_without_language("hello"), "add_note(text)"),
    "InvoiceBuilder.set_payment_means_code": (lambda mp: builder().set_payment_means_code("10"), "add_payment_means"),
    "InvoiceBuilder.flags": (lambda mp: builder().flags(1), "set_flags"),
    "InvoiceBuilder.set_tax_exemption": (lambda mp: builder().set_tax_exemption(VatCategory.EXEMPT, 0, TaxExemption("VATEX-SA-29")), "add_tax_exemption"),
    "InvoiceBuilder.add_document_adjustment": (lambda mp: builder().add_document_adjustment(True, Adjustment.amount(1), VatCategory.STANDARD, 15), "add_charge"),
    "InvoiceBuilder.set_allowance": (lambda mp: builder().set_allowance("r", 1), "add_allowance"),
    "InvoiceBuilder.invoice_level_charge": (lambda mp: builder().invoice_level_charge(1), "add_charge"),
    "InvoiceBuilder.invoice_level_discount": (lambda mp: builder().invoice_level_discount(1), "add_allowance"),
    "InvoiceBuilder.allowance_reason": (lambda mp: builder().allowance_reason("r"), "with_reason"),
    "LineItemBuilder.new": (lambda mp: LineItemBuilder.new(), "LineItemBuilder()"),
    "ZatcaClient.post_csr_for_ccsid": (lambda mp: fake_client(mp).post_csr_for_ccsid(None, "1"), "request_compliance_csid"),
    "ZatcaClient.post_ccsid_for_pcsid": (lambda mp: fake_client(mp).post_ccsid_for_pcsid(None), "request_production_csid"),
}


def test_every_deprecated_name_has_a_case():
    assert sorted(CASES) == CONTRACT["__deprecated__"]


@pytest.mark.parametrize("name", sorted(CASES))
def test_deprecated_name_warns_at_caller_and_still_works(name, monkeypatch):
    call, replacement = CASES[name]
    with pytest.warns(DeprecationWarning) as record:
        result = call(monkeypatch)
    messages = [str(w.message) for w in record if w.category is DeprecationWarning]
    assert len(messages) == 1, messages
    assert replacement in messages[0] and "removed in 0.7" in messages[0]
    # The warning points at the caller (this file), not at fatoora internals.
    assert record[0].filename == __file__
    assert result is not None


def test_deprecated_class_aliases_on_import_and_submodules():
    with pytest.warns(DeprecationWarning, match="InvoiceSigner") as record:
        from fatoora import Signer
    assert Signer is InvoiceSigner and record[0].filename == __file__
    with pytest.warns(DeprecationWarning, match="ComplianceCsid"):
        from fatoora.api import CsidCompliance
    assert CsidCompliance is ComplianceCsid
    with pytest.warns(DeprecationWarning, match="ProductionCsid"):
        assert fatoora.api.CsidProduction is ProductionCsid
    with pytest.warns(DeprecationWarning, match="InvoiceSigner"):
        from fatoora.sign import Signer as sign_signer
    assert sign_signer is InvoiceSigner
    with pytest.raises(AttributeError):
        fatoora.NoSuchName


def test_deprecated_validation_keeps_the_dict_report():
    with pytest.warns(DeprecationWarning):
        report = fatoora.validate_zatca_invoice_from_str(Config(), "<wrong/>")
    assert isinstance(report, dict) and report == fatoora.validate_zatca("<wrong/>",
        evaluated_at=report["evaluated_at"]).to_dict()


# --- getter call form -----------------------------------------------------------------

def test_calling_a_property_warns_and_returns_the_05_value():
    invoice = SignedInvoice.from_file(SIGNED_XML)
    with pytest.warns(DeprecationWarning, match=r"SignedInvoice.id\(\) is deprecated; read the 'id' property") as record:
        value = invoice.id()
    assert value == invoice.id and record[0].filename == __file__
    # Properties whose type changed keep the 0.5 string through the call form.
    with pytest.warns(DeprecationWarning):
        assert invoice.issue_datetime() == "2022-08-17T14:41:08Z"
    assert invoice.issue_datetime == datetime(2022, 8, 17, 14, 41, 8, tzinfo=timezone.utc)
    with pytest.warns(DeprecationWarning):
        assert invoice.issue_date() == "2022-08-17"
    assert invoice.issue_date == date(2022, 8, 17)
    with pytest.warns(DeprecationWarning):
        assert invoice.signing_time() == "2025-07-22T15:51:28"
    with pytest.warns(DeprecationWarning):
        assert invoice.totals().payable_amount == invoice.totals.payable_amount


def test_calling_a_property_that_is_none_still_works():
    invoice = built()
    with pytest.warns(DeprecationWarning):
        assert invoice.buyer() is None
    with pytest.warns(DeprecationWarning):
        assert invoice.seller().address().district() is None
    assert invoice.buyer is None


def test_property_call_form_inside_pytest_assert_rewriting():
    # pytest rewrites ``assert x.f() == y`` to ``@py_assert1 = x.f; @py_assert1()``.
    invoice = built()
    with pytest.warns(DeprecationWarning):
        assert invoice.id() == "numeric"
    with pytest.warns(DeprecationWarning):
        assert invoice.buyer() is None
    assert invoice.id == "numeric"
    assert invoice.buyer is None


def test_property_access_never_warns():
    invoice = built()
    with warnings.catch_warnings():
        warnings.simplefilter("error")
        values = [invoice.id, getattr(invoice, "uuid"), invoice.totals, invoice.flags]
        assert [f"{invoice.id}"] == ["numeric"]
        assert all(v is not None for v in values)


def test_stored_bound_getter_is_the_plain_value():
    # The one 0.5 spelling the transition cannot serve: keeping the getter to call later.
    getter = built().id
    assert getter == "numeric"
    with pytest.raises(TypeError):
        getter()


@pytest.mark.filterwarnings("ignore::DeprecationWarning")
def test_aggregate_setters_still_build_and_chain():
    b = builder().add_line_item("item", 2, "PCE", 50, 15, VatCategory.STANDARD)
    assert b.set_allowance("Loyalty", "10").invoice_level_charge(5) is b
    invoice = b.build()
    assert (invoice.totals.allowance_total, invoice.totals.charge_total) == (Decimal("10"), Decimal("5"))
    b = builder().add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.add_document_adjustment(False, Adjustment.amount(3), VatCategory.STANDARD, 15)
    assert b.build().totals.allowance_total == Decimal("3")
    b = builder().set_payment_means_code("30")
    assert [p.code for p in b.add_line_item("i", 1, "PCE", 1, 15, VatCategory.STANDARD).build().payment_means] == ["10", "30"]
    assert isinstance(PaymentMeans.new("10"), PaymentMeans)
