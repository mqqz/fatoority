"""Ergonomics: typing marker, path-likes, repr, flags, and keyword-only arguments."""
from __future__ import annotations

import inspect
from importlib.metadata import distribution
from pathlib import Path
import warnings

import pytest

from fatoora import (Address, ClearanceMode, Config, ComplianceCsid, ProductionCsid, CsrProperties, Environment,
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
    creds = cls(Environment.SIMULATION, token=TOKEN, secret=SECRET, request_id="req-42")
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
        getattr(client, method)(None, None, ClearanceMode.DISABLED)
    with pytest.raises(TypeError, match="'clearance'"):
        getattr(client, method)(None, None)
    with pytest.raises(TypeError, match="not both"):
        getattr(client, method)(None, None, clearance=ClearanceMode.DISABLED, clearance_status=False)
    for value in (False, "1"):
        with pytest.raises(TypeError, match="ClearanceMode"):
            getattr(client, method)(None, None, clearance=value)
    with pytest.raises(TypeError, match="bool"):
        getattr(client, method)(None, None, clearance_status=1)


@pytest.mark.parametrize("method", ["report_simplified_invoice", "clear_standard_invoice"])
def test_clearance_status_bool_warns_at_caller(monkeypatch, method):
    client = _fake_client(monkeypatch)
    with pytest.warns(DeprecationWarning, match="clearance=ClearanceMode.ENABLED") as record:
        getattr(client, method)(None, None, clearance_status=True)
    assert record[0].filename == __file__ and "removed in 0.7" in str(record[0].message)


def test_csr_properties_is_keyword_only():
    with pytest.raises(TypeError, match="positional argument"):
        CsrProperties(*CSR_FIELDS.values())
    with pytest.raises(TypeError, match="positional argument"), pytest.warns(DeprecationWarning):
        CsrProperties.new(*CSR_FIELDS.values())


@pytest.mark.parametrize("cls", [ComplianceCsid, ProductionCsid])
def test_credentials_take_keywords_after_env(cls):
    with pytest.raises(TypeError, match="positional argument"):
        cls(Environment.SIMULATION, TOKEN, SECRET)
    creds = cls(Environment.SIMULATION, token=TOKEN, secret=SECRET, request_id="r")
    assert (creds.binary_security_token, creds.secret, creds.request_id) == (TOKEN, SECRET, "r")
    # The deprecated factory keeps the 0.5 order: (env, token, secret, request_id).
    with pytest.warns(DeprecationWarning, match=f"{cls.__name__}\\(env, token="):
        old = cls.new(Environment.SIMULATION, TOKEN, SECRET, "r")
    assert (old.binary_security_token, old.secret, old.request_id) == (TOKEN, SECRET, "r")


@pytest.mark.parametrize("cls,hint", [
    ("InvoiceSigner", "from_pem"), ("SigningKey", "generate"), ("Csr", "CsrProperties.build"),
    ("FinalizedInvoice", "from_xml"), ("SignedInvoice", "from_xml"),
    ("ValidationResponse", "ZatcaClient"), ("ValidationResults", "validation_results"),
    ("ValidationMessage", "messages"),
])
def test_types_without_a_constructor_name_their_factories(cls, hint):
    import fatoora
    with pytest.raises(TypeError, match=f"{cls} has no public constructor; use .*{hint}"):
        getattr(fatoora, cls)()


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
        CsrProperties(**CSR_FIELDS)
        ComplianceCsid(Environment.SIMULATION, token=TOKEN, secret=SECRET)
        client = _fake_client(monkeypatch)
        for method in ("report_simplified_invoice", "clear_standard_invoice"):
            getattr(client, method)(None, None, clearance=ClearanceMode.DISABLED)
            getattr(client, method)(None, None, clearance=ClearanceMode.ENABLED, accept_language="en")
            getattr(client, method)(invoice=None, pcsid=None, clearance=ClearanceMode.DISABLED)


def test_keyword_only_signatures():
    kinds = {name: p.kind for name, p in inspect.signature(InvoiceBuilder.set_seller).parameters.items()}
    assert list(kinds)[:2] == ["self", "name"]
    assert all(kind is inspect.Parameter.KEYWORD_ONLY for kind in list(kinds.values())[2:])
    for factory in (CsrProperties, CsrProperties.new):
        params = inspect.signature(factory).parameters
        assert list(params) == list(CSR_FIELDS)
        assert all(p.kind is inspect.Parameter.KEYWORD_ONLY for p in params.values())
    params = inspect.signature(ZatcaClient.report_simplified_invoice).parameters
    assert [p.kind for p in params.values()][-3:] == [inspect.Parameter.KEYWORD_ONLY] * 3
    params = inspect.signature(ProductionCsid).parameters
    assert [(name, p.kind is inspect.Parameter.KEYWORD_ONLY) for name, p in params.items()] == [
        ("env", False), ("token", True), ("secret", True), ("request_id", True)]
    assert InvoiceBuilder.add_note.__name__ == "add_note"
