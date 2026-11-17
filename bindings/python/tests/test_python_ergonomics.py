"""Ergonomics: typing marker, path-likes, repr, flags, and keyword-only arguments."""
from __future__ import annotations

import inspect
from importlib.metadata import distribution
from pathlib import Path
import warnings

import pytest

from fatoora import (Address, Config, ComplianceCsid, ProductionCsid, CsrProperties, Environment,
                     InvoiceBuilder, InvoiceFlag, SigningKey, InvoiceSubType, InvoiceTypeKind, VatCategory,
                     ZatcaClient, FinalizedInvoice, SignedInvoice)
from test_numeric_contract import builder

FIXTURES = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures"
CSR_CONFIG = FIXTURES / "csr-configs/csr-config-example-EN.properties"
SIGNED_XML = FIXTURES / "invoices/sample-simplified-invoice.xml"
SECRET = "super-secret-value"
TOKEN = "binary-security-token-value"
CSR_FIELDS = dict(
    common_name="TST-886431145-399999999900003",
    serial_number="1-TST|2-TST|3-ed22f1d8-e6a2-1118-9b58-d9a8f11e445f",
    organization_identifier="399999999900003",
    organization_unit_name="Riyadh Branch",
    organization_name="Maximum Speed Tech Supply LTD",
    country_name="SA",
    invoice_type="1100",
    location_address="RRRD2929",
    industry_business_category="Supply activities",
)


def test_distribution_ships_py_typed_marker():
    files = {str(p) for p in distribution("fatoority").files}
    assert "fatoora/py.typed" in files


@pytest.mark.parametrize("path", [CSR_CONFIG, str(CSR_CONFIG)])
def test_from_file_accepts_path_like(path):
    props = CsrProperties.from_file(path)
    csr = props.build(SigningKey.generate(), Environment.NON_PRODUCTION)
    assert "TST-886431145-399999999900003" in csr.subject_string


@pytest.mark.parametrize("as_path", [True, False])
def test_invoice_file_parsers_accept_path_like(tmp_path, as_path):
    signed = SignedInvoice.from_file(SIGNED_XML if as_path else str(SIGNED_XML))
    assert signed.id
    unsigned = tmp_path / "finalized.xml"
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    unsigned.write_text(b.build().xml, encoding="utf-8")
    finalized = FinalizedInvoice.from_file(unsigned if as_path else str(unsigned))
    assert finalized.id == "numeric"


def test_handle_reprs_show_identifying_fields():
    assert repr(Config(Environment.NON_PRODUCTION)) == "Config(env=Environment.NON_PRODUCTION)"
    address = Address("SA", "Riyadh", "King Fahd", "1234", "12222")
    assert repr(address) == ("Address(country_code='SAU', city='Riyadh', street='King Fahd', "
                             "building_number='1234', postal_code='12222', additional_street=None, "
                             "additional_number=None, district=None, province=None)")
    signed = SignedInvoice.from_xml(SIGNED_XML.read_text())
    assert repr(signed) == f"SignedInvoice(id={signed.id!r}, uuid={signed.uuid!r})"
    assert repr(InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)) == "InvoiceBuilder()"


def test_closed_handles_repr_safely():
    config = Config(Environment.NON_PRODUCTION)
    config.close()
    assert repr(config) == "<Config closed>"
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.build()
    # A consumed builder still reprs (no fields to read).
    assert repr(b) == "InvoiceBuilder()"


def test_repr_never_raises_when_a_getter_fails():
    config = Config(Environment.NON_PRODUCTION)
    config._repr_fields = ("env", "does_not_exist")
    assert repr(config).startswith("<Config object at 0x")


@pytest.mark.parametrize("cls", [ComplianceCsid, ProductionCsid])
def test_credential_repr_redacts_secrets(cls):
    creds = cls.new(Environment.SIMULATION, TOKEN, SECRET, "req-42")
    text = repr(creds)
    assert text == (f"{cls.__name__}(request_id='req-42', env=Environment.SIMULATION, "
                    "binary_security_token='<redacted>', secret='<redacted>')")
    assert SECRET not in text and TOKEN not in text
    assert SECRET not in str(creds) and SECRET not in f"{creds!r}"
    creds.close()
    assert repr(creds) == f"<{cls.__name__} closed>"


def test_invoice_flag_is_int_flag_and_flags_returns_members():
    assert issubclass(InvoiceFlag, int)
    assert [int(f) for f in (InvoiceFlag.THIRD_PARTY, InvoiceFlag.NOMINAL, InvoiceFlag.EXPORT,
                             InvoiceFlag.SUMMARY, InvoiceFlag.SELF_BILLED)] == [1, 2, 4, 8, 16]
    b = builder()
    b.set_flags(InvoiceFlag.EXPORT | InvoiceFlag.SUMMARY)
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    invoice = b.build()
    assert invoice.flags_raw == 12
    assert invoice.flags == {InvoiceFlag.EXPORT, InvoiceFlag.SUMMARY}
    assert all(type(f) is InvoiceFlag for f in invoice.flags)
    assert invoice.is_export() and invoice.is_summary() and not invoice.is_nominal()


def test_builder_set_flags_accepts_plain_int():
    b = builder()
    b.set_flags(1 | 16)
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    assert b.build().flags == {InvoiceFlag.THIRD_PARTY, InvoiceFlag.SELF_BILLED}


SELLER = dict(country_code="SA", city="Riyadh", street="King Fahd", building_number="1234",
              postal_code="12222", vat_id="399999999900003")


def _fake_client(monkeypatch):
    client = ZatcaClient(Config(Environment.NON_PRODUCTION))
    monkeypatch.setattr(client, "_invoke", lambda *args: object())
    return client


@pytest.mark.parametrize("call", [
    lambda b: b.set_seller("Acme", *SELLER.values()),
    lambda b: b.set_buyer("Buyer", "SA", "Riyadh", "Street", "", ""),
    lambda b: b.add_note("hello", "en"),
])
def test_builder_keyword_only_arguments_are_enforced(call):
    # 0.5.1 warned on these positional calls; 0.6 makes them keyword-only.
    b = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
    with pytest.raises(TypeError, match="positional argument"):
        call(b)


@pytest.mark.parametrize("method", ["report_simplified_invoice", "clear_standard_invoice"])
def test_client_flags_are_keyword_only(monkeypatch, method):
    client = _fake_client(monkeypatch)
    with pytest.raises(TypeError, match="positional argument"):
        getattr(client, method)(None, None, False)
    with pytest.raises(TypeError, match="clearance_status"):
        getattr(client, method)(None, None)


def test_csr_properties_new_is_keyword_only():
    with pytest.raises(TypeError, match="positional argument"):
        CsrProperties.new(*CSR_FIELDS.values())


def test_keyword_calls_do_not_warn(monkeypatch):
    with warnings.catch_warnings():
        warnings.simplefilter("error", DeprecationWarning)
        b = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
        b.set_seller("Acme", **SELLER)
        b.set_buyer("Buyer", country_code="SA", city="Riyadh", street="Street",
                    building_number="", postal_code="")
        b.add_note("hello")
        b.add_note("hello", language="en")
        b.add_note(text="hello", language="en")
        CsrProperties.new(**CSR_FIELDS)
        client = _fake_client(monkeypatch)
        for method in ("report_simplified_invoice", "clear_standard_invoice"):
            getattr(client, method)(None, None, clearance_status=False)
            getattr(client, method)(None, None, clearance_status=True, accept_language="en")
            getattr(client, method)(invoice=None, pcsid=None, clearance_status=False)


def test_keyword_only_signatures():
    kinds = {name: p.kind for name, p in inspect.signature(InvoiceBuilder.set_seller).parameters.items()}
    assert list(kinds)[:2] == ["self", "name"]
    assert all(kind is inspect.Parameter.KEYWORD_ONLY for kind in list(kinds.values())[2:])
    params = inspect.signature(CsrProperties.new).parameters
    assert list(params) == list(CSR_FIELDS)
    assert all(p.kind is inspect.Parameter.KEYWORD_ONLY for p in params.values())
    params = inspect.signature(ZatcaClient.report_simplified_invoice).parameters
    assert [p.kind for p in params.values()][-2:] == [inspect.Parameter.KEYWORD_ONLY] * 2
    assert InvoiceBuilder.add_note.__name__ == "add_note"
