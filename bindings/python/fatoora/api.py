from __future__ import annotations

from contextlib import ExitStack
from dataclasses import dataclass, field
from datetime import date, datetime, timezone
from decimal import Decimal
from enum import Enum, IntEnum, IntFlag
import json
import os
import re
from threading import RLock
from typing import Any, Optional, Union
import uuid as _uuid

from . import _native
from ._compat import alias_getattr, deprecated
from .errors import InvalidInputError, error_class_for_code

class Environment(IntEnum):
    NON_PRODUCTION = 0
    SIMULATION = 1
    PRODUCTION = 2

class InvoiceOutcome(IntEnum):
    """Outcome of the invoked operation; compliance acceptance only means the check passed."""
    UNKNOWN = 0
    ACCEPTED = 1
    REJECTED = 2

class InvoiceTypeKind(IntEnum):
    TAX = 0
    PREPAYMENT = 1
    CREDIT_NOTE = 2
    DEBIT_NOTE = 3

class InvoiceSubType(IntEnum):
    STANDARD = 0
    SIMPLIFIED = 1

class VatCategory(IntEnum):
    EXEMPT = 0
    STANDARD = 1
    ZERO = 2
    OUT_OF_SCOPE = 3

class InvoiceFlag(IntFlag):
    THIRD_PARTY = 1
    NOMINAL = 2
    EXPORT = 4
    SUMMARY = 8
    SELF_BILLED = 16

@dataclass(frozen=True)
class TaxExemption:
    reason_code: Optional[str] = None
    reason: Optional[str] = None

    def _native_value(self):
        return _call(_native.TaxExemption.new, self.reason_code, self.reason)

    @classmethod
    def _from_native(cls, value):
        return cls(_call(value.reason_code), _call(value.reason))

@dataclass(frozen=True)
class AllowanceCharge:
    charge: bool
    amount: Decimal
    category: VatCategory
    rate: Decimal
    reason: Optional[str] = None
    reason_code: Optional[str] = None
    exemption: TaxExemption = TaxExemption()
    tax_category_count: int = 1
    base_amount: Optional[Decimal] = None
    percentage: Optional[Decimal] = None
    omitted_rate: bool = False

@dataclass(frozen=True)
class VatBreakdown:
    category: VatCategory
    rate: Decimal
    taxable_amount: Decimal
    tax_amount: Decimal
    exemption: TaxExemption
    omitted_rate: bool = False

    def _native_value(self):
        value = _call(_native.VatBreakdown.new, int(self.category), _decimal(self.rate), _decimal(self.taxable_amount), _decimal(self.tax_amount), self.exemption._native_value())
        return _call(value.without_vat_rate) if self.omitted_rate else value

@dataclass(frozen=True)
class DocumentReference:
    id: str
    uuid: Optional[str] = None
    issue_date: Optional[str] = None
    issue_time: Optional[str] = None
    document_type_code: Optional[str] = None

@dataclass(frozen=True)
class InvoiceLineItem:
    description: str
    unit_code: str
    quantity: Decimal
    unit_price: Decimal
    total_amount: Decimal
    vat_rate: Decimal
    vat_amount: Decimal
    vat_category: VatCategory
    id: Optional[str] = None
    exemption: TaxExemption = TaxExemption()
    document_references: tuple[DocumentReference, ...] = ()
    tax_subtotals: tuple[VatBreakdown, ...] = ()
    item_price_base_quantity: Optional[Decimal] = None
    item_price_base_quantity_unit_code: Optional[str] = None
    item_gross_price: Optional[Decimal] = None
    item_price_discount: Optional[Decimal] = None
    adjustments: tuple[AllowanceCharge, ...] = ()
    item_seller_identifier: Optional[tuple[str, Optional[str]]] = None
    item_buyer_identifier: Optional[tuple[str, Optional[str]]] = None
    item_standard_identifier: Optional[tuple[str, Optional[str]]] = None
    omitted_vat_rate: bool = False
    omitted_tax_amount: bool = False
    omitted_gross_amount: bool = False

@dataclass(frozen=True)
class InvoiceTotals:
    tax_inclusive: Decimal
    tax_amount: Decimal
    line_extension: Decimal
    allowance_total: Decimal
    charge_total: Decimal
    taxable_amount: Decimal
    prepaid_amount: Decimal
    payable_rounding_amount: Decimal
    payable_amount: Decimal


_StrPath = Union[str, "os.PathLike[str]"]
_DecimalIn = Union[Decimal, str, int]
_UuidIn = Union[_uuid.UUID, str]
_DateIn = Union[date, str]
_DateTimeIn = Union[datetime, str]


def _repr_value(value):
    if isinstance(value, Enum):
        return f"{type(value).__name__}.{value.name}"
    return repr(value)


class _Owned:
    """Own a generated object; serialize calls that could consume its state."""
    # Property names shown by ``repr``; ``_repr_redacted`` names are never read.
    _repr_fields: tuple[str, ...] = ()
    _repr_redacted: tuple[str, ...] = ()

    def __init__(self, handle):
        self._handle = handle
        self._lock = RLock()

    @classmethod
    def _wrap(cls, handle):
        """Wrap a native handle without running a public ``__init__``."""
        obj = cls.__new__(cls)
        _Owned.__init__(obj, handle)
        return obj

    def close(self):
        with self._lock:
            self._handle = None

    def __enter__(self):
        with self._lock:
            self._require_open()
        return self

    def __exit__(self, *args):
        self.close()

    def _require_open(self):
        if self._handle is None:
            raise InvalidInputError("object is closed", 1, {"type": "binding_error"})
        return self._handle

    def _invoke(self, name, *args):
        return _call(lambda owner, *values: getattr(owner, name)(*values), self, *args)

    def __repr__(self) -> str:
        name = type(self).__name__
        try:
            if getattr(self, "_handle", None) is None:
                return f"<{name} closed>"
            parts = [f"{field}={_repr_value(getattr(self, field))}" for field in self._repr_fields]
            parts += [f"{field}='<redacted>'" for field in self._repr_redacted]
            return f"{name}({', '.join(parts)})"
        except Exception:
            return f"<{name} object at {id(self):#x}>"


def _call(function, *args):
    # Hold all argument owners through GIL-releasing calls. Stable ordering avoids
    # deadlocks when two callers use the same objects in a different order.
    owners = sorted({id(v): v for v in args if isinstance(v, _Owned)}.values(), key=id)
    with ExitStack() as locks:
        for owner in owners:
            locks.enter_context(owner._lock)
        values = [v._require_open() if isinstance(v, _Owned) else v for v in args]
        try:
            value = function(*values)
        except Exception as exc:
            if len(exc.args) != 1 or not isinstance(exc.args[0], _native.BindingError):
                raise
            error = exc.args[0]
            code = error.code()
            raise error_class_for_code(code)(error.message(), code, json.loads(error.details_json())) from None
        return _convert(value)


def _native_enum_member(value):
    # nanobind wraps each generated enum in a class holding a nested Python
    # enum of the same name; compare against it to recover the member.
    for member in getattr(type(value), type(value).__name__):
        if value == member:
            return member
    raise ValueError(f"unexpected native enum value: {value!r}")


def _native_enum_value(value) -> int:
    return _native_enum_member(value).value


def _native_enum_wire(value) -> str:
    # ``PreviousInvoiceHash`` -> ``previous_invoice_hash``: the JSON wire name.
    return re.sub(r"(?<!^)(?=[A-Z])", "_", _native_enum_member(value).name).lower()


def _convert(value):
    public = _NATIVE_ENUMS.get(type(value))
    if public is not None:
        return public(_native_enum_value(value))
    if isinstance(value, _native.Text):
        return _call(value.value)
    if isinstance(value, _native.Bytes):
        return value._copy()
    if isinstance(value, _native.BytesList):
        return [_call(value.get, i) for i in range(value.len())]
    return value


# Generated enums share discriminants with the public IntEnums; the others
# surface as plain ints for internal mapping.
_NATIVE_ENUMS = {
    _native.Environment: Environment,
    _native.InvoiceOutcome: InvoiceOutcome,
    _native.InvoiceTypeKind: InvoiceTypeKind,
    _native.InvoiceSubType: InvoiceSubType,
    _native.VatCategory: VatCategory,
    _native.AdjustmentKind: int,
}
_CHARGE = 1  # _native.AdjustmentKind.Charge


def _flag(value: bool, name: str) -> int:
    # Booleans map onto two-variant native enums (AdjustmentKind, ClearanceMode)
    # whose discriminant 1 is the "true" case.
    if not isinstance(value, bool):
        raise TypeError(f"{name} must be a bool")
    return int(value)


def _decimal(value: _DecimalIn) -> str:
    if isinstance(value, (float, bool)) or not isinstance(value, (Decimal, str, int)):
        raise TypeError("decimal values must be Decimal, str, or int")
    return format(value, "f") if isinstance(value, Decimal) else str(value)

def _optional_decimal(value):
    return None if value is None else Decimal(value)

def _identifier(value):
    return None if value is None else (_call(value.value), _call(value.scheme))

def _optional(cls, value):
    return None if value is None else cls._wrap(value)


# --- dates, times and UUIDs ---------------------------------------------------------

def _uuid_text(value: Optional[_UuidIn], name: str = "uuid") -> Optional[str]:
    if value is None or isinstance(value, str):
        return value
    if isinstance(value, _uuid.UUID):
        return str(value)
    raise TypeError(f"{name} must be a uuid.UUID or str")


def _date_text(value: Optional[_DateIn], name: str) -> Optional[str]:
    if value is None or isinstance(value, str):
        return value
    if isinstance(value, datetime):
        raise TypeError(f"{name} must be a datetime.date, not a datetime; use .date()")
    if isinstance(value, date):
        return value.isoformat()
    raise TypeError(f"{name} must be a datetime.date or str")


def _timestamp_text(value: Optional[_DateTimeIn], name: str) -> Optional[str]:
    """Aware ``datetime`` -> ZATCA UTC text ``YYYY-MM-DDTHH:MM:SSZ``."""
    if value is None or isinstance(value, str):
        return value
    if not isinstance(value, datetime):
        raise TypeError(f"{name} must be a timezone-aware datetime or str")
    if value.tzinfo is None or value.utcoffset() is None:
        raise ValueError(f"{name} must be timezone-aware (for example tzinfo=timezone.utc)")
    return value.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _parse_date(text: Optional[str]) -> Optional[date]:
    return None if text is None else date.fromisoformat(text)


def _parse_timestamp(text: Optional[str]) -> Optional[datetime]:
    """Parse RFC 3339 text; a missing offset means UTC (ZATCA timestamps are UTC)."""
    if text is None:
        return None
    value = text[:-1] + "+00:00" if text.endswith(("Z", "z")) else text
    # Core may report nanoseconds; Python 3.9/3.10 only parse 3 or 6 fraction
    # digits, and ``datetime`` holds microseconds anyway.
    value = re.sub(r"\.(\d+)", lambda m: "." + m.group(1)[:6].ljust(6, "0"), value, count=1)
    parsed = datetime.fromisoformat(value)
    return parsed if parsed.tzinfo is not None else parsed.replace(tzinfo=timezone.utc)


class Config(_Owned):
    _repr_fields = ('env',)

    def __init__(self, env: Environment = Environment.NON_PRODUCTION):
        super().__init__(_call(_native.Config.new, int(env)))

    @property
    def env(self) -> Environment:
        return Environment(self._invoke("env"))

class InvoiceSigner(_Owned):
    """Signs invoices with a certificate and its private key."""

    def sign_xml(self, xml: str) -> str:
        return self._invoke("sign_xml", xml)

    @classmethod
    def from_pem(cls, cert_pem: str, key_pem: str) -> 'InvoiceSigner':
        return cls._wrap(_call(_native.InvoiceSigner.from_pem, cert_pem, key_pem))

    @classmethod
    def from_der(cls, cert_der: bytes, key_der: bytes) -> 'InvoiceSigner':
        return cls._wrap(_call(_native.InvoiceSigner.from_der, cert_der, key_der))

    @property
    def certificate_der(self) -> bytes:
        return self._invoke("certificate_der")

    @property
    def certificate_pem(self) -> str:
        return self._invoke("certificate_pem")

class SigningKey(_Owned):
    @classmethod
    def from_pem(cls, pem: str) -> 'SigningKey':
        return cls._wrap(_call(_native.SigningKey.from_pem, pem))

    @classmethod
    def from_der(cls, der: bytes) -> 'SigningKey':
        return cls._wrap(_call(_native.SigningKey.from_der, der))

    @classmethod
    def generate(cls) -> 'SigningKey':
        return cls._wrap(_call(_native.SigningKey.generate))

    def to_pem(self) -> str:
        return self._invoke("to_pem")

    def to_der(self) -> bytes:
        return self._invoke("to_der")

class CsrProperties(_Owned):
    @classmethod
    def new(cls, *, common_name: str, serial_number: str, organization_identifier: str, organization_unit_name: str, organization_name: str, country_name: str, invoice_type: str, location_address: str, industry_business_category: str) -> 'CsrProperties':
        return cls._wrap(_call(_native.CsrProperties.new, common_name, serial_number, organization_identifier, organization_unit_name, organization_name, country_name, invoice_type, location_address, industry_business_category))

    @classmethod
    def from_str(cls, properties: str) -> 'CsrProperties':
        """Parse CSR properties text (the ZATCA ``csr.*=`` properties format)."""
        return cls._wrap(_call(_native.CsrProperties.from_str, properties))

    @classmethod
    def from_file(cls, path: _StrPath) -> 'CsrProperties':
        """Read and parse a CSR properties file."""
        return cls._wrap(_call(_native.CsrProperties.from_file, os.fspath(path)))

    @classmethod
    @deprecated("CsrProperties.from_str(properties)")
    def from_properties_str(cls, properties: str) -> 'CsrProperties':
        return cls.from_str(properties)

    @classmethod
    @deprecated("CsrProperties.from_str(properties)")
    def parse_csr_config(cls, properties: str) -> 'CsrProperties':
        return cls.from_str(properties)

    @classmethod
    @deprecated("CsrProperties.from_file(path)")
    def parse_csr_config_file(cls, path: _StrPath) -> 'CsrProperties':
        return cls.from_file(path)

    def build(self, key: SigningKey, env: Environment) -> 'Csr':
        return Csr._wrap(self._invoke("build", key, int(env)))

class Csr(_Owned):
    _repr_fields = ('subject_string',)

    @classmethod
    def from_der(cls, der: bytes) -> 'Csr':
        return cls._wrap(_call(_native.Csr.from_der, der))

    def to_base64(self) -> str:
        return self._invoke("to_base64")

    def to_pem_base64(self) -> str:
        return self._invoke("to_pem_base64")

    def to_der(self) -> bytes:
        return self._invoke("to_der")

    def to_pem(self) -> str:
        return self._invoke("to_pem")

    @property
    def subject_string(self) -> str:
        return self._invoke("subject_string")

    @property
    def extension_values_der(self) -> list[bytes]:
        return self._invoke("extension_values_der")

class _Csid(_Owned):
    _repr_fields = ('request_id', 'env')
    _repr_redacted = ('binary_security_token', 'secret')
    _native_type: Any = None

    @classmethod
    def new(cls, env: Environment, token: str, secret: str, request_id: Optional[str]=None):
        return cls._wrap(_call(cls._native_type.create, int(env), request_id, token, secret))

    @property
    def request_id(self) -> Optional[str]:
        """The ZATCA request ID, or ``None`` when the credentials have none."""
        return self._invoke("request_id")

    @property
    def env(self) -> Environment:
        return Environment(self._invoke("env"))

    @property
    def binary_security_token(self) -> str:
        return self._invoke("binary_security_token")

    @property
    def secret(self) -> str:
        return self._invoke("secret")

class ComplianceCsid(_Csid):
    """Compliance CSID credentials, returned by ``ZatcaClient.request_compliance_csid``."""
    _native_type = _native.ComplianceCsid

    @classmethod
    def new(cls, env: Environment, token: str, secret: str, request_id: Optional[str]=None) -> 'ComplianceCsid':
        return super().new(env, token, secret, request_id)

class ProductionCsid(_Csid):
    """Production CSID credentials, returned by ``ZatcaClient.request_production_csid``."""
    _native_type = _native.ProductionCsid

    @classmethod
    def new(cls, env: Environment, token: str, secret: str, request_id: Optional[str]=None) -> 'ProductionCsid':
        return super().new(env, token, secret, request_id)

class ValidationMessage(_Owned):
    _repr_fields = ('message_type', 'code', 'status')

    @property
    def message_type(self) -> Optional[str]:
        return self._invoke("message_type_raw")

    @property
    def code(self) -> Optional[str]:
        return self._invoke("code")

    @property
    def category(self) -> Optional[str]:
        return self._invoke("category")

    @property
    def message(self) -> Optional[str]:
        return self._invoke("message")

    @property
    def status(self) -> Optional[str]:
        return self._invoke("status_raw")

class ValidationResults(_Owned):
    _repr_fields = ('status',)

    @property
    def status(self) -> Optional[str]:
        return self._invoke("status_raw")

    @property
    def info_messages(self) -> list[ValidationMessage]:
        return [ValidationMessage._wrap(self._invoke("info_message", i)) for i in range(self._invoke("info_len"))]

    @property
    def warning_messages(self) -> list[ValidationMessage]:
        return [ValidationMessage._wrap(self._invoke("warning_message", i)) for i in range(self._invoke("warning_len"))]

    @property
    def error_messages(self) -> list[ValidationMessage]:
        return [ValidationMessage._wrap(self._invoke("error_message", i)) for i in range(self._invoke("error_len"))]

class ValidationResponse(_Owned):
    _repr_fields = ('http_status', 'outcome')

    @property
    def http_status(self) -> Optional[int]:
        return self._invoke("http_status")

    @property
    def outcome(self) -> InvoiceOutcome:
        return self._invoke("outcome")

    def ensure_accepted(self) -> None:
        return self._invoke("ensure_accepted")

    @property
    def cleared_invoice_base64(self) -> Optional[str]:
        return self._invoke("cleared_invoice_base64")

    @property
    def cleared_invoice_xml(self) -> Optional[str]:
        return self._invoke("cleared_invoice_xml")

    @property
    def reporting_status(self) -> Optional[str]:
        return self._invoke("reporting_status_raw")

    @property
    def clearance_status(self) -> Optional[str]:
        return self._invoke("clearance_status_raw")

    @property
    def qr_seller_status(self) -> Optional[str]:
        return self._invoke("qr_seller_status")

    @property
    def qr_buyer_status(self) -> Optional[str]:
        return self._invoke("qr_buyer_status")

    @property
    def validation_results(self) -> ValidationResults:
        return ValidationResults._wrap(self._invoke("validation_results"))

class VatId(_Owned):
    _repr_fields = ('value',)

    @property
    def value(self) -> str:
        return self._invoke("value")

class OtherId(_Owned):
    _repr_fields = ('value', 'scheme')

    @property
    def value(self) -> str:
        return self._invoke("value")

    @property
    def scheme(self) -> Optional[str]:
        return self._invoke("scheme")

class Address(_Owned):
    _repr_fields = ('country_code', 'city', 'street', 'building_number', 'postal_code')

    def with_province(self, province: str) -> "Address":
        return Address._wrap(self._invoke("with_province", province))

    @property
    def province(self) -> Optional[str]:
        return self._invoke("province")

    @classmethod
    def new(cls, country_code: str, city: str, street: str, building_number: str, postal_code: str, additional_street: Optional[str]=None, additional_number: Optional[str]=None, district: Optional[str]=None) -> 'Address':
        return cls._wrap(_call(_native.Address.new, country_code, city, street, building_number, postal_code, additional_street, additional_number, district))

    @property
    def country_code(self) -> str:
        return self._invoke("country_code")

    @property
    def city(self) -> str:
        return self._invoke("city")

    @property
    def street(self) -> str:
        return self._invoke("street")

    @property
    def additional_street(self) -> Optional[str]:
        return self._invoke("additional_street")

    @property
    def building_number(self) -> str:
        return self._invoke("building_number")

    @property
    def additional_number(self) -> Optional[str]:
        return self._invoke("additional_number")

    @property
    def postal_code(self) -> str:
        return self._invoke("postal_code")

    @property
    def district(self) -> Optional[str]:
        return self._invoke("district")

class Party(_Owned):
    _repr_fields = ('name',)

    @property
    def identifiers(self) -> list[OtherId]:
        return [OtherId._wrap(self._invoke("identifier", i)) for i in range(self._invoke("identifiers_len"))]

    @property
    def name(self) -> str:
        return self._invoke("name")

    @property
    def address(self) -> Optional[Address]:
        return _optional(Address, self._invoke("address"))

    @property
    def vat_id(self) -> Optional[VatId]:
        return _optional(VatId, self._invoke("vat_id"))

    @property
    def other_id(self) -> Optional[OtherId]:
        return _optional(OtherId, self._invoke("other_id"))

class InvoiceNote(_Owned):
    _repr_fields = ('language_id', 'text')

    @property
    def language_id(self) -> Optional[str]:
        return self._invoke("language_id")

    @property
    def language(self) -> str:
        return self._invoke("language")

    @property
    def text(self) -> str:
        return self._invoke("text")

class OriginalInvoiceRef(_Owned):
    _repr_fields = ('id', 'uuid')

    @classmethod
    def new(cls, id: str, uuid: Optional[_UuidIn] = None, date: Optional[_DateIn] = None, time: Optional[str] = None, type_code: Optional[str] = None) -> 'OriginalInvoiceRef':
        return cls._wrap(_call(_native.OriginalInvoiceRef.new, id, _uuid_text(uuid), _date_text(date, "date"), time, type_code))

    @property
    def issue_time(self) -> Optional[str]:
        """``HH:MM:SS`` (AST) or ``HH:MM:SSZ`` (UTC), as written in the invoice."""
        return self._invoke("issue_time")

    @property
    def document_type_code(self) -> Optional[str]:
        return self._invoke("document_type_code")

    @property
    def id(self) -> str:
        return self._invoke("id")

    @property
    def uuid(self) -> Optional[str]:
        return self._invoke("uuid")

    @property
    def issue_date(self) -> Optional[date]:
        return _parse_date(self._invoke("issue_date"))

class ZatcaClient(_Owned):
    def __init__(self, config: Config):
        super().__init__(_call(_native.ZatcaClient.create, config))

    def request_compliance_csid(self, csr: Csr, otp: str) -> ComplianceCsid:
        """Exchange a CSR and OTP for compliance CSID credentials."""
        return ComplianceCsid._wrap(self._invoke("_blocking_request_compliance_csid", csr, otp))

    def request_production_csid(self, ccsid: ComplianceCsid) -> ProductionCsid:
        """Exchange compliance credentials for production CSID credentials."""
        return ProductionCsid._wrap(self._invoke("_blocking_request_production_csid", ccsid))

    @deprecated("request_compliance_csid(csr, otp)")
    def post_csr_for_ccsid(self, csr: Csr, otp: str) -> ComplianceCsid:
        return self.request_compliance_csid(csr, otp)

    @deprecated("request_production_csid(ccsid)")
    def post_ccsid_for_pcsid(self, ccsid: ComplianceCsid) -> ProductionCsid:
        return self.request_production_csid(ccsid)

    def renew_csid(self, pcsid: ProductionCsid, csr: Csr, otp: str, accept_language: Optional[str]=None) -> ProductionCsid:
        return ProductionCsid._wrap(self._invoke("_blocking_renew_csid", pcsid, csr, otp, accept_language))

    def check_invoice_compliance(self, invoice: 'SignedInvoice', ccsid: ComplianceCsid) -> 'ValidationResponse':
        return ValidationResponse._wrap(self._invoke("_blocking_check_invoice_compliance", invoice, ccsid))

    def report_simplified_invoice(self, invoice: 'SignedInvoice', pcsid: ProductionCsid, *, clearance_status: bool, accept_language: Optional[str]=None) -> 'ValidationResponse':
        return ValidationResponse._wrap(self._invoke("_blocking_report_simplified_invoice", invoice, pcsid, _flag(clearance_status, "clearance_status"), accept_language))

    def clear_standard_invoice(self, invoice: 'SignedInvoice', pcsid: ProductionCsid, *, clearance_status: bool, accept_language: Optional[str]=None) -> 'ValidationResponse':
        return ValidationResponse._wrap(self._invoke("_blocking_clear_standard_invoice", invoice, pcsid, _flag(clearance_status, "clearance_status"), accept_language))

class _Invoice(_Owned):
    _repr_fields = ('id', 'uuid')

    @classmethod
    def from_xml(cls, xml: str):
        return cls._wrap(_call(cls._native_type.from_xml, xml))

    @classmethod
    def from_file(cls, path: _StrPath):
        return cls._wrap(_call(cls._native_type.from_file, os.fspath(path)))

    def _data(self, name, *args):
        data = self._invoke("data")
        return _call(getattr(data, name), *args)

    @property
    def issue_time(self) -> str:
        """``HH:MM:SS`` (AST) or ``HH:MM:SSZ`` (UTC), as written in the invoice."""
        return self._data("issue_time")

    @property
    def issue_date(self) -> date:
        return _parse_date(self._data("issue_date"))

    @property
    def id(self) -> str:
        return self._data("id")

    @property
    def uuid(self) -> str:
        """The invoice UUID text (core keeps it verbatim; it may not be RFC 4122)."""
        return self._data("uuid")

    @property
    def issue_datetime(self) -> datetime:
        """Issue timestamp as a timezone-aware (UTC) ``datetime``."""
        return _parse_timestamp(self._data("issue_datetime"))

    @property
    def currency(self) -> str:
        return self._data("currency")

    @property
    def previous_invoice_hash(self) -> str:
        return self._data("previous_invoice_hash")

    @property
    def invoice_counter(self) -> int:
        return self._data("invoice_counter")

    @property
    def payment_means_code(self) -> str:
        return self._data("payment_means_code")

    @property
    def vat_category(self) -> VatCategory:
        return VatCategory(self._data("vat_category"))

    @property
    def invoice_level_charge(self) -> Decimal:
        return Decimal(self._data("invoice_level_charge"))

    @property
    def invoice_level_discount(self) -> Decimal:
        return Decimal(self._data("invoice_level_discount"))

    @property
    def allowance_reason(self) -> Optional[str]:
        return self._data("allowance_reason")

    @property
    def seller(self) -> Party:
        return Party._wrap(self._data("seller"))

    @property
    def buyer(self) -> Optional[Party]:
        return _optional(Party, self._data("buyer"))

    @property
    def note(self) -> Optional[InvoiceNote]:
        return _optional(InvoiceNote, self._data("note"))

    @property
    def invoice_type_kind(self) -> InvoiceTypeKind:
        return InvoiceTypeKind(self._data("invoice_type_kind"))

    @property
    def invoice_sub_type(self) -> InvoiceSubType:
        return InvoiceSubType(self._data("invoice_sub_type"))

    @property
    def original_invoice_ref(self) -> Optional[OriginalInvoiceRef]:
        return _optional(OriginalInvoiceRef, self._data("original_invoice_ref"))

    @property
    def original_invoice_reason(self) -> Optional[str]:
        return self._data("original_invoice_reason")

    @property
    def actual_delivery_date(self) -> Optional[date]:
        return _parse_date(self._data("actual_delivery_date"))

    @property
    def latest_delivery_date(self) -> Optional[date]:
        return _parse_date(self._data("latest_delivery_date"))

    @property
    def payment_instruction(self) -> Optional[str]:
        return self._data("payment_instruction")

    @property
    def tax_currency(self) -> Optional[str]:
        return self._data("tax_currency")

    @property
    def tax_currency_amount(self) -> Optional[Decimal]:
        return _optional_decimal(self._data("tax_currency_amount"))

    @property
    def notes(self) -> list[InvoiceNote]:
        data = self._invoke("data")
        return [InvoiceNote._wrap(_call(data.note_at, i)) for i in range(_call(data.notes_len))]

    @property
    def billing_references(self) -> list[OriginalInvoiceRef]:
        data = self._invoke("data")
        return [OriginalInvoiceRef._wrap(_call(data.billing_reference, i)) for i in range(_call(data.billing_references_len))]

    @property
    def payment_means(self) -> list[PaymentMeans]:
        data = self._invoke("data")
        return [PaymentMeans._wrap(_call(data.payment_means, i)) for i in range(_call(data.payment_means_len))]

    @property
    def purchase_order_id(self) -> Optional[str]:
        return self._data("purchase_order_id")

    @property
    def contract_id(self) -> Optional[str]:
        return self._data("contract_id")

    @property
    def adjustments(self) -> list[AllowanceCharge]:
        data = self._invoke("data")
        result = []
        for i in range(_call(data.adjustments_len)):
            a = _call(data.adjustment, i)
            result.append(AllowanceCharge(_call(a.kind) == _CHARGE, Decimal(_call(a.amount)), VatCategory(_call(a.category)), Decimal(_call(a.rate)), _call(a.reason), _call(a.reason_code), TaxExemption._from_native(_call(a.exemption)), _call(a.tax_category_count), _optional_decimal(_call(a.base_amount)), _optional_decimal(_call(a.percentage)), not _call(a.has_vat_rate)))
        return result

    @property
    def vat_breakdown(self) -> list[VatBreakdown]:
        totals = self._invoke("totals")
        result = []
        for i in range(_call(totals.vat_breakdown_len)):
            group = _call(totals.vat_breakdown, i)
            result.append(VatBreakdown(VatCategory(_call(group.category)), Decimal(_call(group.rate)), Decimal(_call(group.taxable_amount)), Decimal(_call(group.tax_amount)), TaxExemption._from_native(_call(group.exemption)), not _call(group.has_vat_rate)))
        return result

    @property
    def line_item_count(self) -> int:
        return self._data("line_items_len")

    def line_item(self, index: int) -> InvoiceLineItem:
        return self._line_item_value(self._data("line_item", index))

    @staticmethod
    def _line_item_value(item) -> InvoiceLineItem:
        references = []
        subtotals = []
        for i in range(_call(item.document_references_len)):
            ref = _call(item.document_reference, i)
            references.append(DocumentReference(_call(ref.id), _call(ref.uuid), _call(ref.issue_date), _call(ref.issue_time), _call(ref.document_type_code)))
        for i in range(_call(item.tax_subtotals_len)):
            tax = _call(item.tax_subtotal, i)
            subtotals.append(VatBreakdown(VatCategory(_call(tax.category)), Decimal(_call(tax.rate)), Decimal(_call(tax.taxable_amount)), Decimal(_call(tax.tax_amount)), TaxExemption._from_native(_call(tax.exemption)), not _call(tax.has_vat_rate)))
        return InvoiceLineItem(description=(_call(item.description)), unit_code=(_call(item.unit_code)), quantity=Decimal(_call(item.quantity)), unit_price=Decimal(_call(item.unit_price)), total_amount=Decimal(_call(item.total_amount)), vat_rate=Decimal(_call(item.vat_rate)), vat_amount=Decimal(_call(item.vat_amount)), vat_category=VatCategory(_call(item.vat_category)), id=_call(item.id), exemption=TaxExemption._from_native(_call(item.exemption)), document_references=tuple(references), tax_subtotals=tuple(subtotals),
            item_price_base_quantity=_optional_decimal(_call(item.item_price_base_quantity)),
            item_price_base_quantity_unit_code=_call(item.item_price_base_quantity_unit_code),
            item_gross_price=_optional_decimal(_call(item.item_gross_price)),
            item_price_discount=_optional_decimal(_call(item.item_price_discount)),
            adjustments=tuple(AllowanceCharge(_call(item.adjustment_kind, i) == _CHARGE, Decimal(_call(a.value)), VatCategory(_call(item.vat_category)), Decimal(_call(item.vat_rate)), _call(a.reason), _call(a.reason_code), base_amount=_optional_decimal(_call(a.base_amount)), percentage=_optional_decimal(_call(a.percent))) for i in range(_call(item.adjustments_len)) for a in [_call(item.adjustment, i)]),
            item_seller_identifier=_identifier(_call(item.item_seller_identifier)),
            item_buyer_identifier=_identifier(_call(item.item_buyer_identifier)),
            item_standard_identifier=_identifier(_call(item.item_standard_identifier)),
            omitted_vat_rate=not _call(item.has_vat_rate),
            omitted_tax_amount=not _call(item.has_tax_total),
            omitted_gross_amount=not _call(item.has_gross_amount))

    @property
    def line_items(self) -> list[InvoiceLineItem]:
        data = self._invoke("data")
        return [self._line_item_value(_call(data.line_item, i))
                for i in range(_call(data.line_items_len))]

    @property
    def totals(self) -> InvoiceTotals:
        totals = self._invoke("totals")
        return InvoiceTotals(**{name: Decimal(_call(getattr(totals, name))) for name in InvoiceTotals.__dataclass_fields__})

    @property
    def flags_raw(self) -> int:
        return self._data("flags")

    @property
    def flags(self) -> set[InvoiceFlag]:
        bits = self._data("flags")
        # Use the declared single-bit members rather than version-specific flag iteration.
        return {flag for flag in InvoiceFlag.__members__.values() if bits & flag.value}

    def _has_flag(self, flag: InvoiceFlag) -> bool:
        return bool(self._data("flags") & flag.value)

    def is_third_party(self) -> bool:
        return self._has_flag(InvoiceFlag.THIRD_PARTY)

    def is_nominal(self) -> bool:
        return self._has_flag(InvoiceFlag.NOMINAL)

    def is_export(self) -> bool:
        return self._has_flag(InvoiceFlag.EXPORT)

    def is_summary(self) -> bool:
        return self._has_flag(InvoiceFlag.SUMMARY)

    def is_self_billed(self) -> bool:
        return self._has_flag(InvoiceFlag.SELF_BILLED)

    def is_simplified(self) -> bool:
        return InvoiceSubType(self._data("invoice_sub_type")) == InvoiceSubType.SIMPLIFIED

    @property
    def xml(self) -> str:
        """The invoice XML."""
        return self._invoke("xml")

    @deprecated("the xml property")
    def to_xml(self) -> str:
        return self._invoke("xml")

    def hash(self) -> str:
        """Compute the base64 invoice hash (the next invoice's ``previous_invoice_hash``)."""
        return self._invoke("hash")

    @deprecated("hash()")
    def hash_base64(self) -> str:
        return self.hash()


class FinalizedInvoice(_Invoice):
    _native_type = _native.FinalizedInvoice

    @classmethod
    def from_xml(cls, xml: str) -> 'FinalizedInvoice':
        """Parse unsigned invoice XML."""
        return super().from_xml(xml)

    @classmethod
    def from_file(cls, path: _StrPath) -> 'FinalizedInvoice':
        """Read and parse an unsigned invoice XML file."""
        return super().from_file(path)

    def sign(self, signer: InvoiceSigner) -> SignedInvoice:
        return SignedInvoice._wrap(signer._invoke("sign", self))


class SignedInvoice(_Invoice):
    _native_type = _native.SignedInvoice

    @classmethod
    def from_xml(cls, xml: str) -> 'SignedInvoice':
        """Parse signed invoice XML, checking its local signature, digest and QR integrity."""
        return super().from_xml(xml)

    @classmethod
    def from_file(cls, path: _StrPath) -> 'SignedInvoice':
        """Read and parse a signed invoice XML file."""
        return super().from_file(path)

    @deprecated("the xml property")
    def into_xml(self) -> str:
        return self._invoke("into_xml")

    def to_xml_base64(self) -> str:
        return self._invoke("to_xml_base64")

    @property
    def qr_code(self) -> str:
        return self._invoke("qr_code")

    @property
    def invoice_hash(self) -> str:
        """The invoice hash stored in the signature (see ``hash()`` to recompute it)."""
        return self._invoke("invoice_hash")

    @property
    def signature(self) -> str:
        return self._invoke("signature")

    @property
    def public_key(self) -> str:
        return self._invoke("public_key")

    @property
    def zatca_key_signature(self) -> Optional[str]:
        return self._invoke("zatca_key_signature")

    @property
    def cert_hash(self) -> str:
        return self._invoke("cert_hash")

    @property
    def signed_props_hash(self) -> str:
        return self._invoke("signed_props_hash")

    @property
    def signing_time(self) -> datetime:
        """Signing time as a timezone-aware (UTC) ``datetime``."""
        return _parse_timestamp(self._invoke("signing_time"))

    @property
    def issuer(self) -> str:
        return self._invoke("issuer")

    @property
    def serial(self) -> str:
        return self._invoke("serial")

class Adjustment(_Owned):
    _repr_fields = ('value',)

    @classmethod
    def amount(cls, value: _DecimalIn) -> "Adjustment":
        return cls._wrap(_call(_native.Adjustment.amount, _decimal(value)))

    @classmethod
    def percentage(cls, base: _DecimalIn, percent: _DecimalIn) -> "Adjustment":
        return cls._wrap(_call(_native.Adjustment.percentage, _decimal(base), _decimal(percent)))

    def with_supplied_amount(self, value: _DecimalIn) -> "Adjustment":
        return Adjustment._wrap(self._invoke("with_supplied_amount", _decimal(value)))

    def with_reason(self, reason: Optional[str] = None, code: Optional[str] = None) -> "Adjustment":
        return Adjustment._wrap(self._invoke("with_reason", reason, code))

    @property
    def value(self) -> Decimal:
        return Decimal(self._invoke("value"))

    @property
    def base_amount(self) -> Optional[Decimal]:
        return _optional_decimal(self._invoke("base_amount"))

    @property
    def percent(self) -> Optional[Decimal]:
        return _optional_decimal(self._invoke("percent"))

    @property
    def reason(self) -> Optional[str]:
        return self._invoke("reason")

    @property
    def reason_code(self) -> Optional[str]:
        return self._invoke("reason_code")

class PaymentMeans(_Owned):
    _repr_fields = ('code',)

    def with_instruction_note(self, value: str) -> "PaymentMeans":
        return PaymentMeans._wrap(self._invoke("with_instruction_note", value))

    @property
    def instruction_notes(self) -> list[str]:
        return [self._invoke("instruction_note_at", i) for i in range(self._invoke("instruction_notes_len"))]

    @classmethod
    def new(cls, code: str, instruction: Optional[str] = None) -> "PaymentMeans":
        return cls._wrap(_call(_native.PaymentMeans.new, code, instruction))

    def with_account(self, id: Optional[str] = None, terms: Optional[str] = None) -> "PaymentMeans":
        return PaymentMeans._wrap(self._invoke("with_account", id, terms))

    @property
    def code(self) -> str:
        return self._invoke("code")

    @property
    def instruction_note(self) -> Optional[str]:
        return self._invoke("instruction_note")

    def has_account(self) -> bool:
        return self._invoke("has_account")

    @property
    def account_identifier(self) -> Optional[str]:
        return self._invoke("account_identifier")

    @property
    def payment_terms(self) -> Optional[str]:
        return self._invoke("payment_terms")

class LineItemBuilder(_Owned):
    """Builds one line item for ``InvoiceBuilder.add_item``-style use; setters chain."""

    def __init__(self) -> None:
        super().__init__(_call(_native.LineItemBuilder.new))

    @classmethod
    @deprecated("LineItemBuilder()")
    def new(cls) -> "LineItemBuilder":
        return cls()

    def build(self) -> InvoiceLineItem:
        return FinalizedInvoice._line_item_value(self._invoke("build"))

    def _set(self, name, *args) -> "LineItemBuilder":
        self._invoke(name, *args)
        return self

    def set_item_name(self, value: str) -> "LineItemBuilder":
        return self._set("set_item_name", value)

    def set_invoiced_quantity_unit_code(self, value: str) -> "LineItemBuilder":
        return self._set("set_invoiced_quantity_unit_code", value)

    def set_item_price_base_quantity_unit_code(self, value: str) -> "LineItemBuilder":
        return self._set("set_item_price_base_quantity_unit_code", value)

    def set_id(self, value: str) -> "LineItemBuilder":
        return self._set("set_id", value)

    def set_invoiced_quantity(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_invoiced_quantity", _decimal(value))

    def set_item_net_price(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_item_net_price", _decimal(value))

    def set_item_gross_price(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_item_gross_price", _decimal(value))

    def set_item_price_discount(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_item_price_discount", _decimal(value))

    def set_item_price_base_quantity(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_item_price_base_quantity", _decimal(value))

    def set_tax_total_present(self, present: bool) -> "LineItemBuilder":
        return self._set("set_tax_total_present", present)

    def set_gross_amount_present(self, present: bool) -> "LineItemBuilder":
        return self._set("set_gross_amount_present", present)

    def set_line_net_amount(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_line_net_amount", _decimal(value))

    def set_line_vat_amount(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_line_vat_amount", _decimal(value))

    def set_line_amount_with_vat(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_line_amount_with_vat", _decimal(value))

    def set_vat_rate(self, value: _DecimalIn) -> "LineItemBuilder":
        return self._set("set_vat_rate", _decimal(value))

    def set_vat_category(self, value: VatCategory) -> "LineItemBuilder":
        return self._set("set_vat_category", int(value))

    def set_exemption(self, value: TaxExemption) -> "LineItemBuilder":
        return self._set("set_exemption", value._native_value())

    def add_allowance(self, value: Adjustment) -> "LineItemBuilder":
        return self._set("add_allowance", value)

    def add_charge(self, value: Adjustment) -> "LineItemBuilder":
        return self._set("add_charge", value)

    def add_document_reference(self, value: OriginalInvoiceRef) -> "LineItemBuilder":
        return self._set("add_document_reference", value)

    def add_tax_subtotal(self, value: VatBreakdown) -> "LineItemBuilder":
        return self._set("add_tax_subtotal", value._native_value())

    def set_item_seller_identifier(self, value: str, scheme: Optional[str] = None) -> "LineItemBuilder":
        return self._set("set_item_seller_identifier", value, scheme)

    def set_item_buyer_identifier(self, value: str, scheme: Optional[str] = None) -> "LineItemBuilder":
        return self._set("set_item_buyer_identifier", value, scheme)

    def set_item_standard_identifier(self, value: str, scheme: Optional[str] = None) -> "LineItemBuilder":
        return self._set("set_item_standard_identifier", value, scheme)


_ADDRESS_FIELDS = ("country_code", "city", "street", "building_number", "postal_code")


def _party_address(method: str, address: Optional[Address], fields: dict[str, Optional[str]],
                   province: Optional[str]) -> Address:
    given = [name for name, value in fields.items() if value is not None]
    if address is not None:
        if not isinstance(address, Address):
            raise TypeError(f"{method}(): address must be an Address")
        if given or province is not None:
            raise TypeError(f"{method}(): pass either address=Address(...) or the address fields, not both "
                            f"(got {', '.join(given + (['province'] if province is not None else []))})")
        return address
    missing = [name for name in _ADDRESS_FIELDS if fields[name] is None]
    if missing:
        raise TypeError(f"{method}() missing required keyword argument(s): {', '.join(missing)} "
                        "(or pass address=Address.new(...))")
    result = Address.new(**fields)
    return result.with_province(province) if province is not None else result


class InvoiceBuilder(_Owned):
    """Builds an invoice. Setters return the builder, so calls chain.

    Pass the header fields as keyword arguments, use the setters, or mix both::

        invoice = (InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
                                  id="INV-1", uuid=uuid4(), issue_datetime=now,
                                  currency="SAR", previous_invoice_hash=seed,
                                  invoice_counter=1, vat_category=VatCategory.STANDARD)
                   .set_seller("Acme", vat_id="399999999900003", address=address)
                   .add_line_item("Item", 1, "PCE", 100, 15, VatCategory.STANDARD)
                   .build())
    """

    def __init__(self, invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, *,
                 id: Optional[str] = None,
                 uuid: Optional[_UuidIn] = None,
                 issue_datetime: Optional[_DateTimeIn] = None,
                 currency: Optional[str] = None,
                 previous_invoice_hash: Optional[str] = None,
                 invoice_counter: Optional[int] = None,
                 vat_category: Optional[VatCategory] = None,
                 flags: Optional[Union[int, InvoiceFlag]] = None,
                 original_invoice: Optional[OriginalInvoiceRef] = None,
                 original_invoice_reason: Optional[str] = None) -> None:
        if original_invoice is not None:
            if original_invoice_reason is None:
                raise TypeError("InvoiceBuilder(): original_invoice requires original_invoice_reason")
            handle = _call(_native.InvoiceBuilder.new_with_reference, int(invoice_type), int(invoice_subtype),
                           original_invoice, original_invoice_reason)
        else:
            handle = _call(_native.InvoiceBuilder.new, int(invoice_type), int(invoice_subtype),
                           None, None, None, original_invoice_reason)
        super().__init__(handle)
        self._init_state()
        for name, value in (("set_id", id), ("set_uuid", uuid), ("set_issue_datetime", issue_datetime),
                            ("set_currency", currency), ("set_previous_invoice_hash", previous_invoice_hash),
                            ("set_invoice_counter", invoice_counter), ("set_vat_category", vat_category),
                            ("set_flags", flags)):
            if value is not None:
                getattr(self, name)(value)

    @classmethod
    def _wrap(cls, handle):
        obj = super()._wrap(handle)
        obj._init_state()
        return obj

    def _init_state(self) -> None:
        # Inputs for the deprecated aggregate setters, which ``build`` applies as adjustments.
        self._legacy: Optional[dict[str, Any]] = None
        self._line_rates: list[tuple[VatCategory, Decimal]] = []
        self._vat_category: Optional[VatCategory] = None
        self._explicit_adjustments = False

    @classmethod
    @deprecated("InvoiceBuilder(invoice_type, invoice_subtype, ...)")
    def new(cls, invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, original_invoice_id: Optional[str]=None, original_invoice_uuid: Optional[str]=None, original_invoice_issue_date: Optional[str]=None, original_invoice_reason: Optional[str]=None) -> 'InvoiceBuilder':
        return cls._wrap(_call(_native.InvoiceBuilder.new, int(invoice_type), int(invoice_subtype), original_invoice_id, original_invoice_uuid, original_invoice_issue_date, original_invoice_reason))

    @classmethod
    @deprecated("InvoiceBuilder(invoice_type, invoice_subtype, original_invoice=reference, original_invoice_reason=reason)")
    def new_with_reference(cls, invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, reference: OriginalInvoiceRef, reason: str) -> 'InvoiceBuilder':
        return cls._wrap(_call(_native.InvoiceBuilder.new_with_reference, int(invoice_type), int(invoice_subtype), reference, reason))

    def _set(self, name, *args) -> "InvoiceBuilder":
        self._invoke(name, *args)
        return self

    # --- header ---------------------------------------------------------------------

    def set_id(self, invoice_id: str) -> "InvoiceBuilder":
        return self._set("set_id", invoice_id)

    def set_uuid(self, uuid: _UuidIn) -> "InvoiceBuilder":
        return self._set("set_uuid", _uuid_text(uuid))

    def set_issue_datetime(self, issue_datetime: _DateTimeIn) -> "InvoiceBuilder":
        """Accepts a timezone-aware ``datetime`` (converted to UTC) or ``YYYY-MM-DDTHH:MM:SSZ`` text."""
        return self._set("set_issue_datetime", _timestamp_text(issue_datetime, "issue_datetime"))

    def set_currency(self, currency_code: str) -> "InvoiceBuilder":
        return self._set("set_currency", currency_code)

    def set_previous_invoice_hash(self, previous_invoice_hash: str) -> "InvoiceBuilder":
        return self._set("set_previous_invoice_hash", previous_invoice_hash)

    def set_invoice_counter(self, invoice_counter: int) -> "InvoiceBuilder":
        return self._set("set_invoice_counter", invoice_counter)

    def set_vat_category(self, vat_category: VatCategory) -> "InvoiceBuilder":
        self._invoke("set_vat_category", int(vat_category))
        self._vat_category = VatCategory(vat_category)
        return self

    def set_flags(self, flags: Union[int, InvoiceFlag]) -> "InvoiceBuilder":
        return self._set("set_flags", int(flags))

    @deprecated("set_flags(flags)")
    def flags(self, flags: Union[int, InvoiceFlag]) -> "InvoiceBuilder":
        return self.set_flags(flags)

    def set_delivery(self, actual: Optional[_DateIn] = None, latest: Optional[_DateIn] = None) -> "InvoiceBuilder":
        return self._set("set_delivery", _date_text(actual, "actual"), _date_text(latest, "latest"))

    def set_tax_currency(self, currency: str, amount: _DecimalIn) -> "InvoiceBuilder":
        return self._set("set_tax_currency", currency, _decimal(amount))

    def set_payment_instruction(self, instruction: str) -> "InvoiceBuilder":
        return self._set("set_payment_instruction", instruction)

    def set_prepaid_amount(self, amount: _DecimalIn) -> "InvoiceBuilder":
        return self._set("set_prepaid_amount", _decimal(amount))

    def set_payable_rounding_amount(self, amount: _DecimalIn) -> "InvoiceBuilder":
        return self._set("set_payable_rounding_amount", _decimal(amount))

    def set_purchase_order_id(self, value: str) -> "InvoiceBuilder":
        return self._set("set_purchase_order_id", value)

    def set_contract_id(self, value: str) -> "InvoiceBuilder":
        return self._set("set_contract_id", value)

    def add_note(self, text: str, *, language: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("add_note", language, text)

    @deprecated("add_note(text, language=...)")
    def set_note(self, language: str, text: str) -> "InvoiceBuilder":
        return self.add_note(text, language=language)

    @deprecated("add_note(text)")
    def set_note_without_language(self, text: str) -> "InvoiceBuilder":
        return self.add_note(text)

    def add_billing_reference(self, value: OriginalInvoiceRef) -> "InvoiceBuilder":
        return self._set("add_billing_reference", value)

    def add_payment_means(self, value: PaymentMeans) -> "InvoiceBuilder":
        return self._set("add_payment_means", value)

    @deprecated("add_payment_means(PaymentMeans.new(code))")
    def set_payment_means_code(self, payment_means_code: str) -> "InvoiceBuilder":
        """Append a payment means with this code."""
        return self.add_payment_means(PaymentMeans.new(payment_means_code))

    # --- parties ----------------------------------------------------------------------

    def set_seller(self, name: str, *, vat_id: str, address: Optional[Address] = None,
                   country_code: Optional[str] = None, city: Optional[str] = None,
                   street: Optional[str] = None, building_number: Optional[str] = None,
                   postal_code: Optional[str] = None, additional_street: Optional[str] = None,
                   additional_number: Optional[str] = None, district: Optional[str] = None,
                   province: Optional[str] = None, other_id: Optional[str] = None,
                   other_id_scheme: Optional[str] = None) -> "InvoiceBuilder":
        """Set the seller. Pass ``address=Address.new(...)`` or the address fields."""
        resolved = _party_address("set_seller", address, dict(
            country_code=country_code, city=city, street=street, building_number=building_number,
            postal_code=postal_code, additional_street=additional_street,
            additional_number=additional_number, district=district), province)
        return self._set("set_seller", name, resolved, vat_id, other_id, other_id_scheme)

    def set_buyer(self, name: str, *, address: Optional[Address] = None,
                  country_code: Optional[str] = None, city: Optional[str] = None,
                  street: Optional[str] = None, building_number: Optional[str] = None,
                  postal_code: Optional[str] = None, vat_id: Optional[str] = None,
                  other_id: Optional[str] = None, other_id_scheme: Optional[str] = None,
                  additional_street: Optional[str] = None, additional_number: Optional[str] = None,
                  district: Optional[str] = None, province: Optional[str] = None) -> "InvoiceBuilder":
        """Set the buyer. Pass ``address=Address.new(...)`` or the address fields."""
        resolved = _party_address("set_buyer", address, dict(
            country_code=country_code, city=city, street=street, building_number=building_number,
            postal_code=postal_code, additional_street=additional_street,
            additional_number=additional_number, district=district), province)
        return self._set("set_buyer", name, resolved, vat_id, other_id, other_id_scheme)

    def set_buyer_without_address(self, name: str, vat_id: Optional[str] = None, other_id: Optional[str] = None, scheme: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("set_buyer_without_address", name, vat_id, other_id, scheme)

    def add_seller_identifier(self, value: str, scheme: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("add_seller_identifier", value, scheme)

    def add_buyer_identifier(self, value: str, scheme: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("add_buyer_identifier", value, scheme)

    # --- lines ------------------------------------------------------------------------

    def add_line_item(self, description: str, quantity: _DecimalIn, unit_code: str, unit_price: _DecimalIn, vat_rate: _DecimalIn, vat_category: VatCategory, *, id: Optional[str] = None, exemption: TaxExemption = TaxExemption(), document_references: tuple[DocumentReference, ...] = (), tax_subtotals: tuple[VatBreakdown, ...] = ()) -> "InvoiceBuilder":
        item = _call(_native.InvoiceLineItem.new, description, _decimal(quantity), unit_code, _decimal(unit_price), _decimal(vat_rate), int(vat_category))
        item = _call(item.with_metadata, id, exemption._native_value())
        for ref in document_references:
            native = _call(_native.OriginalInvoiceRef.new, ref.id, ref.uuid, ref.issue_date, ref.issue_time, ref.document_type_code)
            item = _call(item.with_document_reference, native)
        for tax in tax_subtotals:
            native = tax._native_value()
            item = _call(item.with_tax_subtotal, native)
        self._invoke("add_item", item)
        self._record_line(vat_category, vat_rate, document_references)
        return self

    def add_item(self, item: InvoiceLineItem) -> "InvoiceBuilder":
        b = LineItemBuilder()
        b.set_item_name(item.description)
        b.set_invoiced_quantity(item.quantity)
        if item.unit_code:
            b.set_invoiced_quantity_unit_code(item.unit_code)
        b.set_item_net_price(item.unit_price)
        if not item.omitted_vat_rate:
            b.set_vat_rate(item.vat_rate)
        elif item.vat_rate != 0:
            raise ValueError("omitted VAT rate must be zero")
        b.set_vat_category(item.vat_category)
        b.set_line_net_amount(item.total_amount)
        b.set_line_vat_amount(item.vat_amount)
        b.set_tax_total_present(not item.omitted_tax_amount)
        b.set_gross_amount_present(not item.omitted_gross_amount)
        b.set_exemption(item.exemption)
        if item.id is not None:
            b.set_id(item.id)
        for name in ("item_price_base_quantity", "item_price_base_quantity_unit_code", "item_gross_price", "item_price_discount"):
            value = getattr(item, name)
            if value is not None:
                getattr(b, "set_" + name)(value)
        for name in ("item_seller_identifier", "item_buyer_identifier", "item_standard_identifier"):
            value = getattr(item, name)
            if value is not None:
                getattr(b, "set_" + name)(*value)
        for a in item.adjustments:
            adjustment = Adjustment.amount(a.amount) if a.percentage is None else Adjustment.percentage(a.base_amount, a.percentage).with_supplied_amount(a.amount)
            adjustment = adjustment.with_reason(a.reason, a.reason_code)
            (b.add_charge if a.charge else b.add_allowance)(adjustment)
        for ref in item.document_references:
            b.add_document_reference(OriginalInvoiceRef.new(ref.id, ref.uuid, ref.issue_date, ref.issue_time, ref.document_type_code))
        for tax in item.tax_subtotals:
            b.add_tax_subtotal(tax)
        self._invoke("add_item", b._invoke("build"))
        self._record_line(item.vat_category, item.vat_rate, item.document_references)
        return self

    # --- document totals --------------------------------------------------------------

    def add_vat_breakdown(self, value: VatBreakdown) -> "InvoiceBuilder":
        return self._set("add_vat_breakdown", value._native_value())

    def add_tax_exemption(self, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption) -> "InvoiceBuilder":
        return self._set("add_tax_exemption", int(category), _decimal(rate), exemption._native_value())

    @deprecated("add_tax_exemption(category, rate, exemption)")
    def set_tax_exemption(self, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption) -> "InvoiceBuilder":
        return self.add_tax_exemption(category, rate, exemption)

    def add_allowance(self, value: Adjustment, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption = TaxExemption()) -> "InvoiceBuilder":
        """Append a document-level allowance (discount) in ``category`` at ``rate``."""
        self._invoke("add_allowance", value, int(category), _decimal(rate), exemption._native_value())
        self._explicit_adjustments = True
        return self

    def add_charge(self, value: Adjustment, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption = TaxExemption()) -> "InvoiceBuilder":
        """Append a document-level charge in ``category`` at ``rate``."""
        self._invoke("add_charge", value, int(category), _decimal(rate), exemption._native_value())
        self._explicit_adjustments = True
        return self

    @deprecated("add_allowance(value, category, rate) or add_charge(value, category, rate)")
    def add_document_adjustment(self, charge: bool, value: Adjustment, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption = TaxExemption()) -> "InvoiceBuilder":
        _flag(charge, "charge")
        return (self.add_charge if charge else self.add_allowance)(value, category, rate, exemption)

    def add_adjustment(self, adjustment: AllowanceCharge) -> "InvoiceBuilder":
        if (adjustment.base_amount is None) != (adjustment.percentage is None):
            raise ValueError("adjustment base_amount and percentage must be supplied together")
        native = _call(_native.AllowanceCharge.new, _flag(adjustment.charge, "charge"), _decimal(adjustment.amount), adjustment.reason, adjustment.reason_code, int(adjustment.category), _decimal(adjustment.rate), adjustment.exemption._native_value())
        native = _call(native.with_tax_category_count, adjustment.tax_category_count)
        if adjustment.percentage is not None:
            native = _call(native.with_percentage, _decimal(adjustment.base_amount), _decimal(adjustment.percentage))
        if adjustment.omitted_rate:
            native = _call(native.without_vat_rate)
        self._invoke("add_adjustment", native)
        self._explicit_adjustments = True
        return self

    # The deprecated aggregate setters below are applied by ``build`` as one
    # document-level allowance and one charge in the document VAT category, at
    # the single VAT rate of the lines in that category. They cannot be combined
    # with ``add_allowance``, ``add_charge`` or ``add_adjustment``.

    @deprecated("add_allowance(Adjustment.amount(amount).with_reason(reason), category, rate)")
    def set_allowance(self, reason: str, amount: _DecimalIn) -> "InvoiceBuilder":
        self._aggregate_set("discount", amount)
        self._aggregate_set("reason", reason)
        return self

    @deprecated("add_charge(Adjustment.amount(charge), category, rate)")
    def invoice_level_charge(self, charge: _DecimalIn) -> "InvoiceBuilder":
        self._aggregate_set("charge", charge)
        return self

    @deprecated("add_allowance(Adjustment.amount(discount), category, rate)")
    def invoice_level_discount(self, discount: _DecimalIn) -> "InvoiceBuilder":
        self._aggregate_set("discount", discount)
        return self

    @deprecated("Adjustment.with_reason(reason) on each add_allowance/add_charge entry")
    def allowance_reason(self, reason: str) -> "InvoiceBuilder":
        self._aggregate_set("reason", reason)
        return self

    def _aggregate_set(self, key: str, value) -> None:
        if key == "reason":
            if not isinstance(value, str):
                raise TypeError("reason must be a string")
        else:
            value = Adjustment.amount(value)
        self._require_open()
        if self._legacy is None:
            self._legacy = {"charge": None, "discount": None, "reason": None}
        self._legacy[key] = value

    def build(self) -> FinalizedInvoice:
        with self._lock:
            self._require_open()
            self._apply_aggregate()
            return FinalizedInvoice._wrap(self._invoke("build"))

    def _record_line(self, category, rate, document_references=()) -> None:
        # Mirror core totals: prepayment adjustment lines (a reference to
        # document type 386) are not part of the taxable groups.
        if any((ref.document_type_code or "").strip() == "386" for ref in document_references):
            return
        self._line_rates.append((VatCategory(category), Decimal(_decimal(rate))))

    def _apply_aggregate(self) -> None:
        legacy = self._legacy
        if legacy is None:
            return
        charge, discount, reason = legacy["charge"], legacy["discount"], legacy["reason"]
        charge = None if charge is None or charge.value == 0 else charge
        discount = None if discount is None or discount.value == 0 else discount
        if charge is None and discount is None and reason is None:
            return
        if self._explicit_adjustments:
            raise ValueError("invoice_level_charge/invoice_level_discount/allowance_reason cannot be combined with document adjustments")
        category = self._vat_category
        rates = {rate for cat, rate in self._line_rates if cat == category}
        if category is None or len(rates) != 1:
            raise ValueError("aggregate document adjustments need a document VAT category with exactly one line VAT rate; use add_allowance/add_charge")
        rate = _decimal(rates.pop())
        # Prepare (and validate) both entries before touching the native builder,
        # and keep the pending values until they are added, so a rejected input
        # can be corrected and build() retried.
        allowance = None
        if discount is not None or reason is not None:
            allowance = (discount or Adjustment.amount(0)).with_reason(reason or "discount")
        surcharge = charge.with_reason(reason or "charge") if charge is not None else None
        exemption = TaxExemption()._native_value()
        if allowance is not None:
            self._invoke("add_allowance", allowance, int(category), rate, exemption)
        if surcharge is not None:
            self._invoke("add_charge", surcharge, int(category), rate, exemption)
        self._legacy = None


# --- local validation -----------------------------------------------------------------

class ZatcaStage(str, Enum):
    """Validation stage, in execution order. Values are the JSON wire names."""
    UNKNOWN = "unknown"
    XSD = "xsd"
    CEN = "cen"
    KSA = "ksa"
    SIGNATURE = "signature"
    QR = "qr"
    PREVIOUS_INVOICE_HASH = "previous_invoice_hash"

class ZatcaStageStatus(str, Enum):
    UNKNOWN = "unknown"
    NOT_RUN = "not_run"
    COMPLETED = "completed"
    NOT_APPLICABLE = "not_applicable"
    CONTEXT_REQUIRED = "context_required"
    EVALUATION_FAILED = "evaluation_failed"

class Severity(str, Enum):
    WARNING = "warning"
    ERROR = "error"

class ValidationLayer(str, Enum):
    UNKNOWN = "unknown"
    FIELD_CHECKS = "field_checks"
    XSD = "xsd"
    BUSINESS_RULES = "business_rules"
    SIGNATURE = "signature"
    QR = "qr"
    PREVIOUS_INVOICE_HASH = "previous_invoice_hash"

class ValidationLocationKind(str, Enum):
    UNKNOWN = "unknown"
    FIELD = "field"
    X_PATH = "x_path"
    XML = "xml"


def _wire_enum(cls, value):
    # Unknown future values degrade to UNKNOWN rather than failing the report.
    try:
        return cls(_native_enum_wire(value))
    except ValueError:
        return cls("unknown")


@dataclass(frozen=True)
class ValidationLocation:
    """Where a finding applies: a model field path, an XPath, or XML line/column."""
    kind: ValidationLocationKind
    path: Optional[str] = None
    line: Optional[int] = None
    column: Optional[int] = None

@dataclass(frozen=True)
class ZatcaFinding:
    severity: Severity
    layer: ValidationLayer
    code: str
    message: str
    location: Optional[ValidationLocation] = None
    assertion_site: Optional[str] = None

@dataclass(frozen=True)
class ZatcaRuleSource:
    """The bundled stylesheet a business-rule stage ran, with its SHA-256."""
    stylesheet: str
    sha256: str

@dataclass(frozen=True)
class ZatcaStageReport:
    stage: ZatcaStage
    status: ZatcaStageStatus
    findings: tuple[ZatcaFinding, ...] = ()
    evaluated_assertions: tuple[str, ...] = ()
    provenance: Optional[ZatcaRuleSource] = None

@dataclass(frozen=True)
class ZatcaReport:
    """Local ZATCA validation report (``validate_zatca``).

    ``is_valid`` is the verdict: no error findings *and* complete coverage.
    """
    is_valid: bool
    is_complete: bool
    has_errors: bool
    stages: tuple[ZatcaStageReport, ...]
    evaluated_at: datetime
    profile: str
    schema_version: int
    _json: str = field(default="", repr=False, compare=False)

    @property
    def findings(self) -> tuple[ZatcaFinding, ...]:
        """Every finding, in stage order."""
        return tuple(finding for stage in self.stages for finding in stage.findings)

    @property
    def errors(self) -> tuple[ZatcaFinding, ...]:
        return tuple(f for f in self.findings if f.severity is Severity.ERROR)

    @property
    def warnings(self) -> tuple[ZatcaFinding, ...]:
        return tuple(f for f in self.findings if f.severity is Severity.WARNING)

    def stage(self, stage: ZatcaStage) -> Optional[ZatcaStageReport]:
        """The report for one stage, or ``None`` if it is not in the report."""
        stage = ZatcaStage(stage)
        return next((s for s in self.stages if s.stage is stage), None)

    def to_json(self) -> str:
        """The report as JSON (the cross-language report schema)."""
        return self._json

    def to_dict(self) -> dict[str, Any]:
        """The report as plain JSON-compatible data (the 0.5 return value)."""
        return json.loads(self._json)

    @classmethod
    def _from_native(cls, report) -> "ZatcaReport":
        stages = []
        for i in range(_call(report.stages_len)):
            stage = _call(report.stage, i)
            findings = []
            for j in range(_call(stage.findings_len)):
                f = _call(stage.finding, j)
                kind = _call(f.location_kind)
                location = None if kind is None else ValidationLocation(
                    _wire_enum(ValidationLocationKind, kind), _call(f.location_path),
                    _call(f.location_line), _call(f.location_column))
                findings.append(ZatcaFinding(
                    _wire_enum(Severity, _call(f.severity)), _wire_enum(ValidationLayer, _call(f.layer)),
                    _call(f.code), _call(f.message), location, _call(f.assertion_site)))
            source = _call(stage.provenance)
            stages.append(ZatcaStageReport(
                _wire_enum(ZatcaStage, _call(stage.stage)), _wire_enum(ZatcaStageStatus, _call(stage.status)),
                tuple(findings),
                tuple(_call(stage.evaluated_assertion, j) for j in range(_call(stage.evaluated_assertions_len))),
                None if source is None else ZatcaRuleSource(_call(source.stylesheet), _call(source.sha256))))
        return cls(_call(report.is_valid), _call(report.is_complete), _call(report.has_errors), tuple(stages),
                   _parse_timestamp(_call(report.evaluated_at)), _call(report.profile),
                   _call(report.schema_version), _call(report.to_json))


def _validated_text(name: str, value: Optional[str]) -> Optional[str]:
    if value is not None and not isinstance(value, str):
        raise TypeError(f"{name} must be a string or None")
    if value is not None and "\x00" in value:
        raise ValueError(f"{name} contains interior NUL")
    return value


def _validate_zatca_native(xml: str, previous_invoice_hash: Optional[str], evaluated_at: Optional[_DateTimeIn]):
    if not isinstance(xml, str):
        raise TypeError("xml must be a string")
    if isinstance(evaluated_at, datetime):
        if evaluated_at.tzinfo is None or evaluated_at.utcoffset() is None:
            raise ValueError("evaluated_at must be timezone-aware")
        evaluated_at = evaluated_at.isoformat(timespec="seconds")
    _validated_text("xml", xml)
    _validated_text("previous_invoice_hash", previous_invoice_hash)
    _validated_text("evaluated_at", evaluated_at)
    options = json.dumps({"previous_invoice_hash": previous_invoice_hash,
                          "evaluated_at": evaluated_at})
    return _call(_native.Xml.validate_zatca, xml, options)


def validate_zatca(xml: str, *, previous_invoice_hash: Optional[str] = None,
                   evaluated_at: Optional[_DateTimeIn] = None) -> ZatcaReport:
    """Run the full local ZATCA validation profile and return a :class:`ZatcaReport`.

    The report covers rejection and incomplete coverage too: check ``is_valid``.
    Missing predecessor context (``previous_invoice_hash``) makes ``is_complete``
    false. ``evaluated_at`` takes an aware ``datetime`` or RFC 3339 text; the
    default is one UTC clock snapshot. Execution failures raise ``FatooraError``
    with the partial report (as a ``dict``) in ``error.details['report']``. Local
    integrity does not establish certificate issuer trust or remote acceptance.
    """
    return ZatcaReport._from_native(_validate_zatca_native(xml, previous_invoice_hash, evaluated_at))


def validate_xml(xml: str) -> bool:
    """Validate the invoice against the bundled UBL XSD; ``True`` or raises."""
    return _call(_native.Xml.validate, xml)


def hash_xml(xml: str) -> str:
    """Base64 invoice hash of invoice XML (the next invoice's ``previous_invoice_hash``)."""
    return _call(_native.Xml.hash, xml)


@deprecated("validate_zatca(xml, ...), which returns a ZatcaReport (report.to_dict() gives this dict)")
def validate_zatca_invoice_from_str(
    config: Config,
    xml: str,
    *,
    previous_invoice_hash: Optional[str] = None,
    evaluated_at: Optional[str] = None,
) -> dict[str, Any]:
    # Validation is stateless; ``config`` is accepted for compatibility only.
    _call(lambda _config: None, config)
    report = _validate_zatca_native(xml, previous_invoice_hash, evaluated_at)
    return json.loads(_call(report.to_json))


@deprecated("validate_xml(xml)")
def validate_xml_invoice_from_str(config: Config, xml: str) -> bool:
    # Validation is stateless; ``config`` is accepted for compatibility only.
    _call(lambda _config: None, config)
    return validate_xml(xml)


@deprecated("hash_xml(xml)")
def invoice_hash_base64_from_xml_str(xml: str) -> str:
    return hash_xml(xml)


@deprecated("FinalizedInvoice.from_xml(xml)")
def parse_finalized_invoice_xml(xml: str) -> FinalizedInvoice:
    return FinalizedInvoice.from_xml(xml)


@deprecated("FinalizedInvoice.from_file(path)")
def parse_finalized_invoice_xml_file(path: _StrPath) -> FinalizedInvoice:
    return FinalizedInvoice.from_file(path)


@deprecated("SignedInvoice.from_xml(xml)")
def parse_signed_invoice_xml(xml: str) -> SignedInvoice:
    return SignedInvoice.from_xml(xml)


@deprecated("SignedInvoice.from_file(path)")
def parse_signed_invoice_xml_file(path: _StrPath) -> SignedInvoice:
    return SignedInvoice.from_file(path)


# Deprecated class names, served by module ``__getattr__`` so that using them warns.
DEPRECATED_ALIASES: dict[str, tuple[str, Any]] = {
    "Signer": ("InvoiceSigner", lambda: InvoiceSigner),
    "CsidCompliance": ("ComplianceCsid", lambda: ComplianceCsid),
    "CsidProduction": ("ProductionCsid", lambda: ProductionCsid),
}

__getattr__ = alias_getattr(__name__, DEPRECATED_ALIASES)
