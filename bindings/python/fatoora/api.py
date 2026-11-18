from __future__ import annotations

from contextlib import ExitStack
import dataclasses
from dataclasses import dataclass, field
from datetime import date, datetime, timedelta, timezone
from decimal import Decimal
from enum import Enum, IntEnum, IntFlag
import json
import math
import os
import re
from threading import RLock
from typing import Any, Optional, Union
import uuid as _uuid

from . import _native
from .errors import FatooraError, InvalidInputError

class Environment(IntEnum):
    NON_PRODUCTION = 0
    SIMULATION = 1
    PRODUCTION = 2

class InvoiceOutcome(IntEnum):
    """Outcome of the invoked operation; compliance acceptance only means the check passed."""
    UNKNOWN = 0
    ACCEPTED = 1
    REJECTED = 2

class ClearanceMode(IntEnum):
    """The ``Clearance-Status`` request header sent when reporting or clearing an invoice.

    ``DISABLED`` sends ``0`` and ``ENABLED`` sends ``1``. It configures the
    request; it does not describe the result.
    """
    DISABLED = 0
    ENABLED = 1


class _WireStatus(str, Enum):
    """A ZATCA response status; the value is the text ZATCA sent.

    Members are ``str`` subclasses, so ``status == "REPORTED"``, ``str(status)``,
    f-strings and ``json.dumps`` behave as they do for plain strings.
    A value this version does not recognise is preserved verbatim, like the
    core's ``Other(String)`` variant: it comes back as a member named
    ``"OTHER"`` whose ``value`` (and string form) is the text ZATCA sent.
    """

    def __str__(self) -> str:
        return str.__str__(self)

    def __format__(self, spec: str) -> str:
        return str.__format__(self, spec)

    def __hash__(self) -> int:
        return str.__hash__(self)

    @classmethod
    def _missing_(cls, value):
        if not isinstance(value, str):
            return None
        member = str.__new__(cls, value)
        member._name_ = "OTHER"
        member._value_ = value
        return member


class ReportingStatus(_WireStatus):
    """``reportingStatus`` of a reporting response."""
    REPORTED = "REPORTED"
    NOT_REPORTED = "NOT_REPORTED"


class ClearanceStatus(_WireStatus):
    """``clearanceStatus`` of a clearance response."""
    CLEARED = "CLEARED"
    NOT_CLEARED = "NOT_CLEARED"


class ValidationStatus(_WireStatus):
    """Overall ``validationResults.status``, or the ``status`` of one validation message."""
    PASS = "PASS"
    WARNING = "WARNING"
    ERROR = "ERROR"


class MessageType(_WireStatus):
    """The ``type`` of a validation message."""
    INFO = "INFO"
    WARNING = "WARNING"
    ERROR = "ERROR"


def _wire_status(cls, text: Optional[str]):
    return None if text is None else cls(text)


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
class VatBreakdown:
    category: VatCategory
    rate: Decimal
    taxable_amount: Decimal
    tax_amount: Decimal
    exemption: TaxExemption
    omitted_rate: bool = False

    def _native_value(self):
        value = _call(_native.VatBreakdown.new, int(self.category), _decimal(self.rate), _decimal(self.taxable_amount), _decimal(self.tax_amount), self.exemption._native_value())
        return _call(value.omit_vat_rate) if self.omitted_rate else value

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

    # How to obtain an instance of a type without a public constructor.
    _factories: str = ""

    def __init__(self, *args, **kwargs):
        name = type(self).__name__
        raise TypeError(f"{name} has no public constructor; use {self._factories or 'the methods that return it'}")

    def _attach(self, handle) -> None:
        self._handle = handle
        self._lock = RLock()

    @classmethod
    def _wrap(cls, handle):
        """Wrap a native handle without running a public ``__init__``."""
        obj = cls.__new__(cls)
        _Owned._attach(obj, handle)
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
            raise InvalidInputError("object is closed", details={"type": "binding_error"})
        return self._handle

    def _invoke(self, name, *args):
        return _call(lambda owner, *values: getattr(owner, name)(*values), self, *args)

    def _invoke_optional(self, name):
        # A fallible optional string crosses the FFI as an `x_present`/`x` pair.
        def read(owner):
            return getattr(owner, name)() if getattr(owner, f"{name}_present")() else None

        return _call(read, self)

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
            raise FatooraError.from_code(code, error.message(), json.loads(error.details_json())) from None
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
    # Booleans map onto two-variant native enums (AdjustmentKind) whose
    # discriminant 1 is the "true" case.
    if not isinstance(value, bool):
        raise TypeError(f"{name} must be a bool")
    return int(value)


def _decimal(value: _DecimalIn) -> str:
    if isinstance(value, (float, bool)) or not isinstance(value, (Decimal, str, int)):
        raise TypeError("decimal values must be Decimal, str, or int")
    return format(value, "f") if isinstance(value, Decimal) else str(value)

def _optional_decimal(value):
    return None if value is None else Decimal(value)


def _to_decimal(value: _DecimalIn, name: str) -> Decimal:
    """Validate a decimal input (``Decimal``, ``str`` or ``int``) and return it as ``Decimal``."""
    try:
        text = _decimal(value)
    except TypeError:
        raise TypeError(f"{name} must be a Decimal, str or int") from None
    try:
        result = Decimal(text)
    except ArithmeticError:
        result = None
    if result is None or not result.is_finite():
        raise ValueError(f"{name} is not a decimal number: {text!r}")
    return result


def _to_optional_decimal(value: Optional[_DecimalIn], name: str) -> Optional[Decimal]:
    return None if value is None else _to_decimal(value, name)


def _check_text(value, name: str, optional: bool = False) -> None:
    if value is None and optional:
        return
    if not isinstance(value, str):
        raise TypeError(f"{name} must be a str" + (" or None" if optional else ""))


def _frozen(cls, **values):
    """Build a frozen dataclass from trusted native data, skipping ``__post_init__``."""
    obj = object.__new__(cls)
    for name, value in values.items():
        object.__setattr__(obj, name, value)
    return obj


def _set(obj, **values) -> None:
    """Normalize fields of a frozen dataclass from ``__post_init__``."""
    for name, value in values.items():
        object.__setattr__(obj, name, value)


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
    # Core may report nanoseconds; Python 3.10 only parses 3 or 6 fraction
    # digits, and ``datetime`` holds microseconds anyway.
    value = re.sub(r"\.(\d+)", lambda m: "." + m.group(1)[:6].ljust(6, "0"), value, count=1)
    parsed = datetime.fromisoformat(value)
    return parsed if parsed.tzinfo is not None else parsed.replace(tzinfo=timezone.utc)


class Config(_Owned):
    _repr_fields = ('env',)

    def __init__(self, env: Environment = Environment.NON_PRODUCTION):
        self._attach(_call(_native.Config.new, int(env)))

    @property
    def env(self) -> Environment:
        return Environment(self._invoke("env"))

class InvoiceSigner(_Owned):
    """Signs invoices with a certificate and its private key."""
    _factories = "InvoiceSigner.from_pem(cert_pem, key_pem) or InvoiceSigner.from_der(cert_der, key_der)"

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
    _factories = "SigningKey.generate(), SigningKey.from_pem(pem) or SigningKey.from_der(der)"

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
    """The subject and extension fields of a ZATCA CSR (all keyword-only)."""

    def __init__(self, *, common_name: str, serial_number: str, organization_identifier: str,
                 organization_unit_name: str, organization_name: str, country_name: str,
                 invoice_type: str, location_address: str, industry_business_category: str) -> None:
        self._attach(_call(_native.CsrProperties.new, common_name, serial_number, organization_identifier,
                           organization_unit_name, organization_name, country_name, invoice_type,
                           location_address, industry_business_category))

    @classmethod
    def from_str(cls, properties: str) -> 'CsrProperties':
        """Parse CSR properties text (the ZATCA ``csr.*=`` properties format)."""
        return cls._wrap(_call(_native.CsrProperties.from_str, properties))

    @classmethod
    def from_file(cls, path: _StrPath) -> 'CsrProperties':
        """Read and parse a CSR properties file."""
        return cls._wrap(_call(_native.CsrProperties.from_file, os.fspath(path)))

    def build(self, key: SigningKey, env: Environment) -> 'Csr':
        return Csr._wrap(self._invoke("build", key, int(env)))

class Csr(_Owned):
    _repr_fields = ('subject_string',)
    _factories = "CsrProperties.build(key, env) or Csr.from_der(der)"

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

    def __init__(self, env: Environment, *, token: str, secret: str, request_id: Optional[str] = None) -> None:
        """Credentials from stored values: the ``binary_security_token`` and ``secret``
        ZATCA issued, and its optional request ID."""
        self._attach(_call(self._native_type.new, int(env), request_id, token, secret))

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

class ProductionCsid(_Csid):
    """Production CSID credentials, returned by ``ZatcaClient.request_production_csid``."""
    _native_type = _native.ProductionCsid

class ValidationMessage(_Owned):
    _repr_fields = ('message_type', 'code', 'status')
    _factories = "ValidationResults.info_messages, warning_messages or error_messages"

    @property
    def message_type(self) -> Optional[MessageType]:
        return _wire_status(MessageType, self._invoke("message_type_raw"))

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
    def status(self) -> Optional[ValidationStatus]:
        return _wire_status(ValidationStatus, self._invoke("status_raw"))

class ValidationResults(_Owned):
    _repr_fields = ('status',)
    _factories = "ValidationResponse.validation_results"

    @property
    def status(self) -> Optional[ValidationStatus]:
        return _wire_status(ValidationStatus, self._invoke("status_raw"))

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
    _factories = "the ZatcaClient methods that return it"

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
        return self._invoke_optional("cleared_invoice_xml")

    @property
    def reporting_status(self) -> Optional[ReportingStatus]:
        return _wire_status(ReportingStatus, self._invoke("reporting_status_raw"))

    @property
    def clearance_status(self) -> Optional[ClearanceStatus]:
        return _wire_status(ClearanceStatus, self._invoke("clearance_status_raw"))

    @property
    def qr_seller_status(self) -> Optional[str]:
        return self._invoke("qr_seller_status")

    @property
    def qr_buyer_status(self) -> Optional[str]:
        return self._invoke("qr_buyer_status")

    @property
    def validation_results(self) -> ValidationResults:
        return ValidationResults._wrap(self._invoke("validation_results"))

# --- invoice value models -------------------------------------------------------------
#
# Frozen dataclasses for the data an invoice holds. Their constructors validate
# their input; values read from an invoice are built straight from native data.

@dataclass(frozen=True)
class Identifier:
    """An identifier and its optional scheme (``schemeID``), such as ``Identifier("7003339333", "CRN")``."""
    value: str
    scheme: Optional[str] = None

    def __post_init__(self) -> None:
        _check_text(self.value, "value")
        _check_text(self.scheme, "scheme", optional=True)

    @classmethod
    def _from_native(cls, native) -> "Identifier":
        return _frozen(cls, value=_call(native.value), scheme=_call(native.scheme))


def _identifier_in(value, name: str) -> Optional[Identifier]:
    """Accept an ``Identifier`` or a bare value string."""
    if value is None or isinstance(value, Identifier):
        return value
    if isinstance(value, str):
        return Identifier(value)
    raise TypeError(f"{name} must be an Identifier or str")


def _optional_identifier(native) -> Optional[Identifier]:
    return None if native is None else Identifier._from_native(native)


def _vat_breakdown_value(native) -> VatBreakdown:
    return VatBreakdown(VatCategory(_call(native.category)), Decimal(_call(native.rate)),
                        Decimal(_call(native.taxable_amount)), Decimal(_call(native.tax_amount)),
                        TaxExemption._from_native(_call(native.exemption)), not _call(native.has_vat_rate))


def _address_fields(native) -> dict[str, Any]:
    return {name: _call(getattr(native, name)) for name in (
        "country_code", "city", "street", "building_number", "postal_code",
        "additional_street", "additional_number", "district", "province")}


@dataclass(frozen=True)
class Address:
    """A postal address.

    The constructor validates the address and normalizes the country code to
    its ISO 3166-1 alpha-3 form (``"SA"`` becomes ``"SAU"``).
    """
    country_code: str
    city: str
    street: str
    building_number: str
    postal_code: str
    additional_street: Optional[str] = None
    additional_number: Optional[str] = None
    district: Optional[str] = None
    province: Optional[str] = None

    def __post_init__(self) -> None:
        _set(self, **_address_fields(self._native_value()))

    def _native_value(self):
        native = _call(_native.Address.new, self.country_code, self.city, self.street,
                       self.building_number, self.postal_code, self.additional_street,
                       self.additional_number, self.district)
        return native if self.province is None else _call(native.with_province, self.province)

    @classmethod
    def _from_native(cls, native) -> "Address":
        return _frozen(cls, **_address_fields(native))

    def with_province(self, province: str) -> "Address":
        """Return a copy with ``province`` (the country subdivision) set."""
        return dataclasses.replace(self, province=province)

@dataclass(frozen=True)
class Party:
    """A seller or buyer.

    ``vat_id`` and the entries of ``identifiers`` accept an ``Identifier`` or a
    plain string. The first of ``identifiers`` is the party's main other
    identifier (``other_id``); the rest are additional identifiers.
    """
    name: str
    address: Optional[Address] = None
    vat_id: Optional[Identifier] = None
    identifiers: tuple[Identifier, ...] = ()

    def __post_init__(self) -> None:
        _check_text(self.name, "name")
        if self.address is not None and not isinstance(self.address, Address):
            raise TypeError("address must be an Address or None")
        if isinstance(self.identifiers, (str, Identifier)):
            raise TypeError("identifiers must be a sequence of Identifier")
        _set(self, vat_id=_identifier_in(self.vat_id, "vat_id"),
             identifiers=tuple(_identifier_in(v, "identifiers") for v in self.identifiers))

    @property
    def other_id(self) -> Optional[Identifier]:
        """The first identifier, or ``None``."""
        return self.identifiers[0] if self.identifiers else None

    @classmethod
    def _from_native(cls, native) -> "Party":
        address = _call(native.address)
        vat_id = _call(native.vat_id)
        return _frozen(
            cls, name=_call(native.name),
            address=None if address is None else Address._from_native(address),
            vat_id=None if vat_id is None else _frozen(Identifier, value=_call(vat_id.value), scheme=None),
            identifiers=tuple(Identifier._from_native(_call(native.identifier, i))
                              for i in range(_call(native.identifiers_len))))


@dataclass(frozen=True)
class InvoiceNote:
    """An invoice note; ``language_id`` is ``None`` when the note has no language."""
    text: str
    language_id: Optional[str] = None

    def __post_init__(self) -> None:
        _check_text(self.text, "text")
        _check_text(self.language_id, "language_id", optional=True)

    @property
    def language(self) -> str:
        """The note language, ``"en"`` when the note has none."""
        return "en" if self.language_id is None else self.language_id

    @classmethod
    def _from_native(cls, native) -> "InvoiceNote":
        return _frozen(cls, text=_call(native.text), language_id=_call(native.language_id))


def _reference_fields(native) -> dict[str, Any]:
    return {"id": _call(native.id), "uuid": _call(native.uuid),
            "issue_date": _parse_date(_call(native.issue_date)),
            "issue_time": _call(native.issue_time),
            "document_type_code": _call(native.document_type_code)}


@dataclass(frozen=True)
class OriginalInvoiceRef:
    """A reference to another document: the billing reference of a credit or
    debit note, or a line's document reference (a prepayment invoice).

    ``uuid`` accepts ``uuid.UUID`` or text and ``issue_date`` a ``datetime.date``
    or ``"YYYY-MM-DD"``; ``issue_time`` is ``HH:MM:SS`` (AST) or ``HH:MM:SSZ`` (UTC).
    """
    id: str
    uuid: Optional[str] = None
    issue_date: Optional[date] = None
    issue_time: Optional[str] = None
    document_type_code: Optional[str] = None

    def __post_init__(self) -> None:
        _set(self, **_reference_fields(self._native_value()))

    def _native_value(self):
        return _call(_native.OriginalInvoiceRef.new, self.id, _uuid_text(self.uuid),
                     _date_text(self.issue_date, "issue_date"), self.issue_time, self.document_type_code)

    @classmethod
    def _from_native(cls, native) -> "OriginalInvoiceRef":
        return _frozen(cls, **_reference_fields(native))

@dataclass(frozen=True)
class PaymentMeans:
    """A payment means: the UNTDID 4461 ``code`` (``"10"`` cash, ``"30"`` credit
    transfer, ...), instruction notes, and an optional payee account."""
    code: str
    instruction_notes: tuple[str, ...] = ()
    account_identifier: Optional[str] = None
    payment_terms: Optional[str] = None

    def __post_init__(self) -> None:
        if isinstance(self.instruction_notes, str):
            raise TypeError("instruction_notes must be a sequence of str, not a str")
        _set(self, instruction_notes=tuple(self.instruction_notes))
        for note in self.instruction_notes:
            _check_text(note, "instruction note")
        self._native_value()

    def _native_value(self):
        native = _call(_native.PaymentMeans.new, self.code, None)
        for note in self.instruction_notes:
            native = _call(native.with_instruction_note, note)
        if self.has_account():
            native = _call(native.with_account, self.account_identifier, self.payment_terms)
        return native

    @classmethod
    def _from_native(cls, native) -> "PaymentMeans":
        return _frozen(cls, code=_call(native.code),
                       instruction_notes=tuple(_call(native.instruction_note_at, i)
                                               for i in range(_call(native.instruction_notes_len))),
                       account_identifier=_call(native.account_identifier),
                       payment_terms=_call(native.payment_terms))

    @property
    def instruction_note(self) -> Optional[str]:
        """The first instruction note, or ``None``."""
        return self.instruction_notes[0] if self.instruction_notes else None

    def has_account(self) -> bool:
        """Whether a payee account identifier or payment terms are set."""
        return self.account_identifier is not None or self.payment_terms is not None

    def with_instruction_note(self, value: str) -> "PaymentMeans":
        """Return a copy with ``value`` appended to the instruction notes."""
        return dataclasses.replace(self, instruction_notes=self.instruction_notes + (value,))

    def with_account(self, id: Optional[str] = None, terms: Optional[str] = None) -> "PaymentMeans":
        """Return a copy with the payee account identifier and payment terms set."""
        return dataclasses.replace(self, account_identifier=id, payment_terms=terms)

@dataclass(frozen=True)
class Adjustment:
    """An allowance or charge, on the document or on a line.

    Give a fixed ``amount``, or use :meth:`percent_of` for a percentage of a
    base amount. ``reason``/``reason_code`` describe it. The other fields
    apply to document-level entries: ``charge`` (``False`` for an allowance),
    the VAT ``category`` and ``rate`` (or ``omitted_rate=True`` for an
    out-of-scope entry without a rate), the ``exemption`` and the number of
    identical ``cac:TaxCategory`` elements (``tax_category_count``).
    ``InvoiceBuilder.add_allowance``/``add_charge`` take the category and rate
    as arguments instead; values read from an invoice carry all fields (line
    adjustments report their line's category and rate).
    """
    amount: Decimal
    reason: Optional[str] = None
    reason_code: Optional[str] = None
    base_amount: Optional[Decimal] = None
    percentage: Optional[Decimal] = None
    charge: bool = False
    category: Optional[VatCategory] = None
    rate: Optional[Decimal] = None
    exemption: TaxExemption = TaxExemption()
    tax_category_count: int = 1
    omitted_rate: bool = False

    def __post_init__(self) -> None:
        if (self.base_amount is None) != (self.percentage is None):
            raise ValueError("base_amount and percentage must be given together")
        if not isinstance(self.charge, bool):
            raise TypeError("charge must be a bool")
        if not isinstance(self.omitted_rate, bool):
            raise TypeError("omitted_rate must be a bool")
        if isinstance(self.tax_category_count, bool) or not isinstance(self.tax_category_count, int):
            raise TypeError("tax_category_count must be an int")
        if not isinstance(self.exemption, TaxExemption):
            raise TypeError("exemption must be a TaxExemption")
        _check_text(self.reason, "reason", optional=True)
        _check_text(self.reason_code, "reason_code", optional=True)
        _set(self, amount=_to_decimal(self.amount, "amount"),
             base_amount=_to_optional_decimal(self.base_amount, "base_amount"),
             percentage=_to_optional_decimal(self.percentage, "percentage"),
             category=None if self.category is None else VatCategory(self.category),
             rate=_to_optional_decimal(self.rate, "rate"))

    @classmethod
    def percent_of(cls, base: _DecimalIn, percent: _DecimalIn, **fields: Any) -> "Adjustment":
        """``percent`` percent of ``base``, for example ``Adjustment.percent_of(250, 10)``.

        The amount is calculated as the core does; pass ``amount=`` to supply
        it instead (it must then match). Other keyword arguments set the
        remaining fields.
        """
        base_text, percent_text = _decimal(base), _decimal(percent)
        if fields.get("amount") is None:
            native = _call(_native.Adjustment.percent_of, base_text, percent_text)
            fields["amount"] = _call(native.value)
        return cls(base_amount=Decimal(base_text), percentage=Decimal(percent_text), **fields)

    def with_reason(self, reason: Optional[str] = None, code: Optional[str] = None) -> "Adjustment":
        """Return a copy with the reason text and/or reason code set."""
        return dataclasses.replace(self, reason=self.reason if reason is None else reason,
                                   reason_code=self.reason_code if code is None else code)

    def _line_native(self):
        """The native line adjustment (category and rate come from the line)."""
        base, percentage = self.base_amount, self.percentage
        if base is None or percentage is None:
            native = _call(_native.Adjustment.amount, _decimal(self.amount))
        else:
            native = _call(_native.Adjustment.percent_of, _decimal(base), _decimal(percentage))
            native = _call(native.supplied_amount, _decimal(self.amount))
        if self.reason is not None or self.reason_code is not None:
            native = _call(native.with_reason, self.reason, self.reason_code)
        return native

    def _document_native(self, charge: bool, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption):
        """The native document-level entry, keeping the tax category count and an omitted rate."""
        native = _call(_native.AllowanceCharge.new, _flag(charge, "charge"), _decimal(self.amount),
                       self.reason, self.reason_code, int(category), _decimal(rate), exemption._native_value())
        native = _call(native.with_tax_category_count, self.tax_category_count)
        if self.base_amount is not None and self.percentage is not None:
            native = _call(native.with_percentage, _decimal(self.base_amount), _decimal(self.percentage))
        if self.omitted_rate:
            native = _call(native.omit_vat_rate)
        return native

    @classmethod
    def _from_native_entry(cls, native) -> "Adjustment":
        return _frozen(
            cls, amount=Decimal(_call(native.amount)), reason=_call(native.reason),
            reason_code=_call(native.reason_code), base_amount=_optional_decimal(_call(native.base_amount)),
            percentage=_optional_decimal(_call(native.percentage)), charge=_call(native.kind) == _CHARGE,
            category=VatCategory(_call(native.category)), rate=Decimal(_call(native.rate)),
            exemption=TaxExemption._from_native(_call(native.exemption)),
            tax_category_count=_call(native.tax_category_count), omitted_rate=not _call(native.has_vat_rate))

    @classmethod
    def _from_native_line(cls, native, charge: bool, category: VatCategory, rate: Decimal) -> "Adjustment":
        return _frozen(
            cls, amount=Decimal(_call(native.value)), reason=_call(native.reason),
            reason_code=_call(native.reason_code), base_amount=_optional_decimal(_call(native.base_amount)),
            percentage=_optional_decimal(_call(native.percentage)), charge=charge, category=category, rate=rate,
            exemption=TaxExemption(), tax_category_count=1, omitted_rate=False)



@dataclass(frozen=True)
class InvoiceLineItem:
    """A line item, as read from an invoice or added with ``InvoiceBuilder.add_item``.

    As input only ``description``, ``unit_code``, ``quantity``, ``unit_price``,
    ``vat_rate`` and ``vat_category`` are required: the line net amount
    (``total_amount``) and VAT (``vat_amount``) are calculated when omitted
    and checked when given. Values read from an invoice always carry them.
    """
    description: str
    unit_code: str
    quantity: Decimal
    unit_price: Decimal
    total_amount: Optional[Decimal] = None
    vat_rate: Decimal = field(default=None)  # type: ignore[assignment]  # required; defaulted to follow total_amount
    vat_amount: Optional[Decimal] = None
    vat_category: VatCategory = field(default=None)  # type: ignore[assignment]  # required
    id: Optional[str] = None
    exemption: TaxExemption = TaxExemption()
    document_references: tuple[OriginalInvoiceRef, ...] = ()
    tax_subtotals: tuple[VatBreakdown, ...] = ()
    item_price_base_quantity: Optional[Decimal] = None
    item_price_base_quantity_unit_code: Optional[str] = None
    item_gross_price: Optional[Decimal] = None
    item_price_discount: Optional[Decimal] = None
    adjustments: tuple[Adjustment, ...] = ()
    item_seller_identifier: Optional[Identifier] = None
    item_buyer_identifier: Optional[Identifier] = None
    item_standard_identifier: Optional[Identifier] = None
    omitted_vat_rate: bool = False
    omitted_tax_amount: bool = False
    omitted_gross_amount: bool = False

    def __post_init__(self) -> None:
        for name in ("vat_rate", "vat_category"):
            if getattr(self, name) is None:
                raise TypeError(f"InvoiceLineItem() missing required argument: {name!r}")
        _check_text(self.description, "description")
        _check_text(self.unit_code, "unit_code")
        if not isinstance(self.exemption, TaxExemption):
            raise TypeError("exemption must be a TaxExemption")
        values: dict[str, Any] = {"vat_category": VatCategory(self.vat_category)}
        for name in ("quantity", "unit_price", "vat_rate"):
            values[name] = _to_decimal(getattr(self, name), name)
        for name in ("total_amount", "vat_amount", "item_price_base_quantity", "item_gross_price",
                     "item_price_discount"):
            values[name] = _to_optional_decimal(getattr(self, name), name)
        for name, kind in (("document_references", OriginalInvoiceRef), ("tax_subtotals", VatBreakdown),
                           ("adjustments", Adjustment)):
            items = tuple(getattr(self, name))
            if not all(isinstance(item, kind) for item in items):
                raise TypeError(f"{name} must contain {kind.__name__} values")
            values[name] = items
        for name in ("item_seller_identifier", "item_buyer_identifier", "item_standard_identifier"):
            values[name] = _identifier_in(getattr(self, name), name)
        _set(self, **values)


_MAX_TIMEOUT_MS = 2**32 - 1


def _timeout_ms(value: Union[float, timedelta]) -> int:
    """Seconds (``int``/``float``) or ``timedelta`` -> whole milliseconds, rounded up."""
    if isinstance(value, timedelta):
        seconds = value.total_seconds()
    elif isinstance(value, (int, float)) and not isinstance(value, bool):
        seconds = float(value)
    else:
        raise TypeError("timeout must be a number of seconds, a datetime.timedelta, or None")
    if not math.isfinite(seconds) or seconds <= 0:
        raise ValueError("timeout must be a positive, finite duration")
    milliseconds = math.ceil(seconds * 1000)
    if milliseconds > _MAX_TIMEOUT_MS:
        raise ValueError(f"timeout must be at most {_MAX_TIMEOUT_MS / 1000:g} seconds")
    return milliseconds


class ZatcaClient(_Owned):
    """Blocking ZATCA gateway client.

    ``timeout`` bounds each request, from connecting until the response body is
    read, in seconds (``int``/``float``) or as a ``datetime.timedelta``; it is
    rounded up to whole milliseconds. A request that exceeds it raises
    ``NetworkError``. The default ``None`` applies no client-side timeout.
    """

    def __init__(self, config: Config, *, timeout: Optional[Union[float, timedelta]] = None):
        if timeout is None:
            handle = _call(_native.ZatcaClient.new, config)
        else:
            handle = _call(_native.ZatcaClient.with_timeout, config, _timeout_ms(timeout))
        self._attach(handle)

    def request_compliance_csid(self, csr: Csr, otp: str) -> ComplianceCsid:
        """Exchange a CSR and OTP for compliance CSID credentials."""
        return ComplianceCsid._wrap(self._invoke("_blocking_request_compliance_csid", csr, otp))

    def request_production_csid(self, ccsid: ComplianceCsid) -> ProductionCsid:
        """Exchange compliance credentials for production CSID credentials."""
        return ProductionCsid._wrap(self._invoke("_blocking_request_production_csid", ccsid))

    def renew_csid(self, pcsid: ProductionCsid, csr: Csr, otp: str, accept_language: Optional[str]=None) -> ProductionCsid:
        return ProductionCsid._wrap(self._invoke("_blocking_renew_csid", pcsid, csr, otp, accept_language))

    def check_invoice_compliance(self, invoice: 'SignedInvoice', ccsid: ComplianceCsid) -> 'ValidationResponse':
        return ValidationResponse._wrap(self._invoke("_blocking_check_invoice_compliance", invoice, ccsid))

    def report_simplified_invoice(self, invoice: 'SignedInvoice', pcsid: ProductionCsid, *,
                                  clearance: ClearanceMode, accept_language: Optional[str] = None) -> 'ValidationResponse':
        """Report a simplified invoice. ``clearance`` sets the ``Clearance-Status`` header."""
        mode = _clearance_mode(clearance)
        return ValidationResponse._wrap(self._invoke("_blocking_report_simplified_invoice", invoice, pcsid, mode, accept_language))

    def clear_standard_invoice(self, invoice: 'SignedInvoice', pcsid: ProductionCsid, *,
                               clearance: ClearanceMode, accept_language: Optional[str] = None) -> 'ValidationResponse':
        """Clear a standard invoice. ``clearance`` sets the ``Clearance-Status`` header."""
        mode = _clearance_mode(clearance)
        return ValidationResponse._wrap(self._invoke("_blocking_clear_standard_invoice", invoice, pcsid, mode, accept_language))


def _clearance_mode(clearance: ClearanceMode) -> int:
    """The native ``ClearanceMode`` discriminant for ``clearance=``."""
    if isinstance(clearance, bool) or not isinstance(clearance, int):
        raise TypeError("clearance must be a ClearanceMode")
    return int(ClearanceMode(clearance))

class _Invoice(_Owned):
    _repr_fields = ('id', 'uuid')

    @classmethod
    def from_xml(cls, xml: str):
        return cls._wrap(_call(cls._native_type.from_xml, xml))

    @classmethod
    def from_file(cls, path: _StrPath):
        return cls._wrap(_call(cls._native_type.from_file, os.fspath(path)))

    # Invoices are immutable once built or parsed, so their data is read from
    # the native object once, on first access, and cached. ``close()`` drops it.

    def close(self):
        with self._lock:
            self._cache = None
            super().close()

    def _get(self, name: str):
        with self._lock:
            self._require_open()
            cache = getattr(self, "_cache", None)
            if cache is None:
                cache = self._cache = self._read()
            return cache[name]

    def _read(self) -> dict[str, Any]:
        data = self._invoke("data")
        totals = self._invoke("totals")

        def get(name, *args):
            return _call(getattr(data, name), *args)

        def items(name, length, convert):
            return tuple(convert(get(name, i)) for i in range(get(length)))

        buyer, reference = get("buyer"), get("original_invoice_ref")
        values = {name: get(name) for name in (
            "id", "uuid", "issue_time", "currency", "previous_invoice_hash", "invoice_counter",
            "payment_means_code", "allowance_reason", "original_invoice_reason", "payment_instruction",
            "tax_currency", "purchase_order_id", "contract_id", "flags")}
        values.update(
            issue_date=_parse_date(get("issue_date")),
            issue_datetime=_parse_timestamp(get("issue_datetime")),
            vat_category=VatCategory(get("vat_category")),
            invoice_type_kind=InvoiceTypeKind(get("invoice_type_kind")),
            invoice_sub_type=InvoiceSubType(get("invoice_sub_type")),
            invoice_level_charge=Decimal(get("invoice_level_charge")),
            invoice_level_discount=Decimal(get("invoice_level_discount")),
            actual_delivery_date=_parse_date(get("actual_delivery_date")),
            latest_delivery_date=_parse_date(get("latest_delivery_date")),
            tax_currency_amount=_optional_decimal(get("tax_currency_amount")),
            seller=Party._from_native(get("seller")),
            buyer=None if buyer is None else Party._from_native(buyer),
            original_invoice_ref=None if reference is None else OriginalInvoiceRef._from_native(reference),
            notes=items("note_at", "notes_len", InvoiceNote._from_native),
            billing_references=items("billing_reference", "billing_references_len", OriginalInvoiceRef._from_native),
            payment_means=items("payment_means", "payment_means_len", PaymentMeans._from_native),
            adjustments=items("adjustment", "adjustments_len", Adjustment._from_native_entry),
            line_items=items("line_item", "line_items_len", self._line_item_value),
            totals=InvoiceTotals(**{name: Decimal(_call(getattr(totals, name)))
                                    for name in InvoiceTotals.__dataclass_fields__}),
            vat_breakdown=tuple(_vat_breakdown_value(_call(totals.vat_breakdown, i))
                                for i in range(_call(totals.vat_breakdown_len))),
        )
        return values

    @property
    def issue_time(self) -> str:
        """``HH:MM:SS`` (AST) or ``HH:MM:SSZ`` (UTC), as written in the invoice."""
        return self._get("issue_time")

    @property
    def issue_date(self) -> date:
        return self._get("issue_date")

    @property
    def id(self) -> str:
        return self._get("id")

    @property
    def uuid(self) -> str:
        """The invoice UUID text (core keeps it verbatim; it may not be RFC 4122)."""
        return self._get("uuid")

    @property
    def issue_datetime(self) -> datetime:
        """Issue timestamp as a timezone-aware (UTC) ``datetime``."""
        return self._get("issue_datetime")

    @property
    def currency(self) -> str:
        return self._get("currency")

    @property
    def previous_invoice_hash(self) -> str:
        return self._get("previous_invoice_hash")

    @property
    def invoice_counter(self) -> int:
        return self._get("invoice_counter")

    @property
    def payment_means_code(self) -> str:
        return self._get("payment_means_code")

    @property
    def vat_category(self) -> VatCategory:
        return self._get("vat_category")

    @property
    def invoice_level_charge(self) -> Decimal:
        return self._get("invoice_level_charge")

    @property
    def invoice_level_discount(self) -> Decimal:
        return self._get("invoice_level_discount")

    @property
    def allowance_reason(self) -> Optional[str]:
        return self._get("allowance_reason")

    @property
    def seller(self) -> Party:
        return self._get("seller")

    @property
    def buyer(self) -> Optional[Party]:
        return self._get("buyer")

    @property
    def note(self) -> Optional[InvoiceNote]:
        """The first note, or ``None``."""
        notes = self._get("notes")
        return notes[0] if notes else None

    @property
    def invoice_type_kind(self) -> InvoiceTypeKind:
        return self._get("invoice_type_kind")

    @property
    def invoice_sub_type(self) -> InvoiceSubType:
        return self._get("invoice_sub_type")

    @property
    def original_invoice_ref(self) -> Optional[OriginalInvoiceRef]:
        return self._get("original_invoice_ref")

    @property
    def original_invoice_reason(self) -> Optional[str]:
        return self._get("original_invoice_reason")

    @property
    def actual_delivery_date(self) -> Optional[date]:
        return self._get("actual_delivery_date")

    @property
    def latest_delivery_date(self) -> Optional[date]:
        return self._get("latest_delivery_date")

    @property
    def payment_instruction(self) -> Optional[str]:
        return self._get("payment_instruction")

    @property
    def tax_currency(self) -> Optional[str]:
        return self._get("tax_currency")

    @property
    def tax_currency_amount(self) -> Optional[Decimal]:
        return self._get("tax_currency_amount")

    @property
    def notes(self) -> list[InvoiceNote]:
        return list(self._get("notes"))

    @property
    def billing_references(self) -> list[OriginalInvoiceRef]:
        return list(self._get("billing_references"))

    @property
    def payment_means(self) -> list[PaymentMeans]:
        return list(self._get("payment_means"))

    @property
    def purchase_order_id(self) -> Optional[str]:
        return self._get("purchase_order_id")

    @property
    def contract_id(self) -> Optional[str]:
        return self._get("contract_id")

    @property
    def adjustments(self) -> list[Adjustment]:
        """Document-level allowances and charges."""
        return list(self._get("adjustments"))

    @property
    def vat_breakdown(self) -> list[VatBreakdown]:
        return list(self._get("vat_breakdown"))

    @property
    def line_item_count(self) -> int:
        return len(self._get("line_items"))

    def line_item(self, index: int) -> InvoiceLineItem:
        line_items = self._get("line_items")
        if not 0 <= index < len(line_items):
            raise InvalidInputError("line item index out of range", details={"type": "binding_error"})
        return line_items[index]

    @staticmethod
    def _line_item_value(item) -> InvoiceLineItem:
        category = VatCategory(_call(item.vat_category))
        rate = Decimal(_call(item.vat_rate))
        return _frozen(
            InvoiceLineItem,
            description=_call(item.description), unit_code=_call(item.unit_code),
            quantity=Decimal(_call(item.quantity)), unit_price=Decimal(_call(item.unit_price)),
            total_amount=Decimal(_call(item.total_amount)), vat_rate=rate,
            vat_amount=Decimal(_call(item.vat_amount)), vat_category=category,
            id=_call(item.id), exemption=TaxExemption._from_native(_call(item.exemption)),
            document_references=tuple(OriginalInvoiceRef._from_native(_call(item.document_reference, i))
                                      for i in range(_call(item.document_references_len))),
            tax_subtotals=tuple(_vat_breakdown_value(_call(item.tax_subtotal, i))
                                for i in range(_call(item.tax_subtotals_len))),
            item_price_base_quantity=_optional_decimal(_call(item.item_price_base_quantity)),
            item_price_base_quantity_unit_code=_call(item.item_price_base_quantity_unit_code),
            item_gross_price=_optional_decimal(_call(item.item_gross_price)),
            item_price_discount=_optional_decimal(_call(item.item_price_discount)),
            adjustments=tuple(Adjustment._from_native_line(_call(item.adjustment, i),
                                                           _call(item.adjustment_kind, i) == _CHARGE,
                                                           category, rate)
                              for i in range(_call(item.adjustments_len))),
            item_seller_identifier=_optional_identifier(_call(item.item_seller_identifier)),
            item_buyer_identifier=_optional_identifier(_call(item.item_buyer_identifier)),
            item_standard_identifier=_optional_identifier(_call(item.item_standard_identifier)),
            omitted_vat_rate=not _call(item.has_vat_rate),
            omitted_tax_amount=not _call(item.has_tax_total),
            omitted_gross_amount=not _call(item.has_gross_amount))

    @property
    def line_items(self) -> list[InvoiceLineItem]:
        return list(self._get("line_items"))

    @property
    def totals(self) -> InvoiceTotals:
        return self._get("totals")

    @property
    def flags_raw(self) -> int:
        return self._get("flags")

    @property
    def flags(self) -> set[InvoiceFlag]:
        bits = self._get("flags")
        # Use the declared single-bit members rather than version-specific flag iteration.
        return {flag for flag in InvoiceFlag.__members__.values() if bits & flag.value}

    def _has_flag(self, flag: InvoiceFlag) -> bool:
        return bool(self._get("flags") & flag.value)

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
        return self._get("invoice_sub_type") == InvoiceSubType.SIMPLIFIED

    @property
    def xml(self) -> str:
        """The invoice XML."""
        return self._invoke("xml")

    def hash(self) -> str:
        """Compute the base64 invoice hash (the next invoice's ``previous_invoice_hash``)."""
        return self._invoke("hash")

class FinalizedInvoice(_Invoice):
    _native_type = _native.FinalizedInvoice
    _factories = "InvoiceBuilder.build(), FinalizedInvoice.from_xml(xml) or FinalizedInvoice.from_file(path)"

    @classmethod
    def from_xml(cls, xml: str) -> 'FinalizedInvoice':
        """Parse unsigned invoice XML."""
        return super().from_xml(xml)

    @classmethod
    def from_file(cls, path: _StrPath) -> 'FinalizedInvoice':
        """Read and parse an unsigned invoice XML file."""
        return super().from_file(path)

    def sign(self, signer: InvoiceSigner) -> SignedInvoice:
        with self._lock:
            try:
                return SignedInvoice._wrap(signer._invoke("sign", self))
            finally:
                self._cache = None  # consumed: later reads fail like other uses


class SignedInvoice(_Invoice):
    _native_type = _native.SignedInvoice
    _factories = "FinalizedInvoice.sign(signer), SignedInvoice.from_xml(xml) or SignedInvoice.from_file(path)"

    @classmethod
    def from_xml(cls, xml: str) -> 'SignedInvoice':
        """Parse signed invoice XML, checking its local signature, digest and QR integrity."""
        return super().from_xml(xml)

    @classmethod
    def from_file(cls, path: _StrPath) -> 'SignedInvoice':
        """Read and parse a signed invoice XML file."""
        return super().from_file(path)

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
        return self._invoke_optional("zatca_key_signature")

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

class LineItemBuilder(_Owned):
    """Builds one line item for ``InvoiceBuilder.add_item``-style use; setters chain."""

    def __init__(self) -> None:
        self._attach(_call(_native.LineItemBuilder.new))

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
        """Add a line allowance; its VAT category and rate are the line's."""
        return self._set("add_allowance", _adjustment_in(value, "add_allowance")._line_native())

    def add_charge(self, value: Adjustment) -> "LineItemBuilder":
        """Add a line charge; its VAT category and rate are the line's."""
        return self._set("add_charge", _adjustment_in(value, "add_charge")._line_native())

    def add_document_reference(self, value: OriginalInvoiceRef) -> "LineItemBuilder":
        return self._set("add_document_reference", _reference_in(value)._native_value())

    def add_tax_subtotal(self, value: VatBreakdown) -> "LineItemBuilder":
        return self._set("add_tax_subtotal", value._native_value())

    def set_item_seller_identifier(self, value: Union[str, Identifier], scheme: Optional[str] = None) -> "LineItemBuilder":
        return self._set("set_item_seller_identifier", *_identifier_args(value, scheme))

    def set_item_buyer_identifier(self, value: Union[str, Identifier], scheme: Optional[str] = None) -> "LineItemBuilder":
        return self._set("set_item_buyer_identifier", *_identifier_args(value, scheme))

    def set_item_standard_identifier(self, value: Union[str, Identifier], scheme: Optional[str] = None) -> "LineItemBuilder":
        return self._set("set_item_standard_identifier", *_identifier_args(value, scheme))


def _identifier_args(value: Union[str, Identifier], scheme: Optional[str]) -> tuple[str, Optional[str]]:
    if isinstance(value, Identifier):
        if scheme is not None:
            raise TypeError("pass the scheme in the Identifier, not as a separate argument")
        return value.value, value.scheme
    return value, scheme


def _adjustment_in(value: Adjustment, method: str) -> Adjustment:
    if not isinstance(value, Adjustment):
        raise TypeError(f"{method}() takes an Adjustment")
    return value


def _party_only(method: str, others: dict[str, Any]) -> None:
    given = sorted(name for name, value in others.items() if value is not None)
    if given:
        raise TypeError(f"{method}(): pass either a Party or the party fields, not both (got {', '.join(given)})")


def _reference_in(value: OriginalInvoiceRef) -> OriginalInvoiceRef:
    if not isinstance(value, OriginalInvoiceRef):
        raise TypeError("expected an OriginalInvoiceRef")
    return value


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
                        "(or pass address=Address(...))")
    return Address(**fields, province=province)  # type: ignore[arg-type]


class InvoiceBuilder(_Owned):
    """Builds an invoice. Setters return the builder, so calls chain.

    Pass the header fields as keyword arguments, use the setters, or mix both::

        invoice = (InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
                                  id="INV-1", uuid=uuid4(), issue_datetime=now,
                                  currency="SAR", previous_invoice_hash=seed,
                                  invoice_counter=1, vat_category=VatCategory.STANDARD)
                   .set_seller(Party("Acme", address, vat_id="399999999900003"))
                   .add_line_item("Item", 1, "PCE", 100, 15, VatCategory.STANDARD)
                   .build())

    ``seller=`` and ``buyer=`` take a :class:`Party` (see ``set_seller``/``set_buyer``).
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
                 original_invoice_reason: Optional[str] = None,
                 seller: Optional[Party] = None,
                 buyer: Optional[Party] = None) -> None:
        if original_invoice is not None:
            if original_invoice_reason is None:
                raise TypeError("InvoiceBuilder(): original_invoice requires original_invoice_reason")
            handle = _call(_native.InvoiceBuilder.new_with_reference, int(invoice_type), int(invoice_subtype),
                           _reference_in(original_invoice)._native_value(), original_invoice_reason)
        else:
            handle = _call(_native.InvoiceBuilder.new, int(invoice_type), int(invoice_subtype),
                           None, None, None, original_invoice_reason)
        self._attach(handle)
        for name, value in (("set_id", id), ("set_uuid", uuid), ("set_issue_datetime", issue_datetime),
                            ("set_currency", currency), ("set_previous_invoice_hash", previous_invoice_hash),
                            ("set_invoice_counter", invoice_counter), ("set_vat_category", vat_category),
                            ("set_flags", flags), ("set_seller", seller), ("set_buyer", buyer)):
            if value is not None:
                getattr(self, name)(value)

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
        return self._set("set_vat_category", int(vat_category))

    def set_flags(self, flags: Union[int, InvoiceFlag]) -> "InvoiceBuilder":
        return self._set("set_flags", int(flags))

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

    def add_billing_reference(self, value: OriginalInvoiceRef) -> "InvoiceBuilder":
        return self._set("add_billing_reference", _reference_in(value)._native_value())

    def add_payment_means(self, value: PaymentMeans) -> "InvoiceBuilder":
        if not isinstance(value, PaymentMeans):
            raise TypeError("add_payment_means() takes a PaymentMeans")
        return self._set("add_payment_means", value._native_value())

    # --- parties ----------------------------------------------------------------------

    def set_seller(self, name: Union[str, Party], *, vat_id: Optional[str] = None,
                   address: Optional[Address] = None,
                   country_code: Optional[str] = None, city: Optional[str] = None,
                   street: Optional[str] = None, building_number: Optional[str] = None,
                   postal_code: Optional[str] = None, additional_street: Optional[str] = None,
                   additional_number: Optional[str] = None, district: Optional[str] = None,
                   province: Optional[str] = None, other_id: Optional[str] = None,
                   other_id_scheme: Optional[str] = None) -> "InvoiceBuilder":
        """Set the seller from a :class:`Party` (with an address and VAT ID), or from
        ``name``, ``vat_id=`` and ``address=Address(...)`` or the address fields."""
        fields = dict(country_code=country_code, city=city, street=street, building_number=building_number,
                      postal_code=postal_code, additional_street=additional_street,
                      additional_number=additional_number, district=district)
        if isinstance(name, Party):
            _party_only("set_seller", dict(fields, vat_id=vat_id, address=address, province=province,
                                           other_id=other_id, other_id_scheme=other_id_scheme))
            return self._apply_party(True, name)
        if vat_id is None:
            raise TypeError("set_seller() missing required keyword argument: 'vat_id'")
        resolved = _party_address("set_seller", address, fields, province)
        return self._set("set_seller", name, resolved._native_value(), vat_id, other_id, other_id_scheme)

    def set_buyer(self, name: Union[str, Party], *, address: Optional[Address] = None,
                  country_code: Optional[str] = None, city: Optional[str] = None,
                  street: Optional[str] = None, building_number: Optional[str] = None,
                  postal_code: Optional[str] = None, vat_id: Optional[str] = None,
                  other_id: Optional[str] = None, other_id_scheme: Optional[str] = None,
                  additional_street: Optional[str] = None, additional_number: Optional[str] = None,
                  district: Optional[str] = None, province: Optional[str] = None) -> "InvoiceBuilder":
        """Set the buyer from a :class:`Party` (its address is optional), or from
        ``name`` and ``address=Address(...)`` or the address fields."""
        fields = dict(country_code=country_code, city=city, street=street, building_number=building_number,
                      postal_code=postal_code, additional_street=additional_street,
                      additional_number=additional_number, district=district)
        if isinstance(name, Party):
            _party_only("set_buyer", dict(fields, vat_id=vat_id, address=address, province=province,
                                          other_id=other_id, other_id_scheme=other_id_scheme))
            return self._apply_party(False, name)
        resolved = _party_address("set_buyer", address, fields, province)
        return self._set("set_buyer", name, resolved._native_value(), vat_id, other_id, other_id_scheme)

    def _apply_party(self, seller: bool, party: Party) -> "InvoiceBuilder":
        role = "seller" if seller else "buyer"
        vat_id = None if party.vat_id is None else party.vat_id.value
        other = party.other_id
        other_args = (None, None) if other is None else (other.value, other.scheme)
        if seller:
            if party.address is None or vat_id is None:
                raise ValueError("the seller Party needs an address and a vat_id")
            self._invoke("set_seller", party.name, party.address._native_value(), vat_id, *other_args)
        elif party.address is None:
            self._invoke("set_buyer_without_address", party.name, vat_id, *other_args)
        else:
            self._invoke("set_buyer", party.name, party.address._native_value(), vat_id, *other_args)
        for identifier in party.identifiers[1:]:
            self._invoke(f"add_{role}_identifier", identifier.value, identifier.scheme)
        return self

    def set_buyer_without_address(self, name: str, vat_id: Optional[str] = None, other_id: Optional[str] = None, scheme: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("set_buyer_without_address", name, vat_id, other_id, scheme)

    def add_seller_identifier(self, value: Union[str, Identifier], scheme: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("add_seller_identifier", *_identifier_args(value, scheme))

    def add_buyer_identifier(self, value: Union[str, Identifier], scheme: Optional[str] = None) -> "InvoiceBuilder":
        return self._set("add_buyer_identifier", *_identifier_args(value, scheme))

    # --- lines ------------------------------------------------------------------------

    def add_line_item(self, description: str, quantity: _DecimalIn, unit_code: str, unit_price: _DecimalIn, vat_rate: _DecimalIn, vat_category: VatCategory, *, id: Optional[str] = None, exemption: TaxExemption = TaxExemption(), document_references: tuple[OriginalInvoiceRef, ...] = (), tax_subtotals: tuple[VatBreakdown, ...] = ()) -> "InvoiceBuilder":
        item = _call(_native.InvoiceLineItem.new, description, _decimal(quantity), unit_code, _decimal(unit_price), _decimal(vat_rate), int(vat_category))
        item = _call(item.with_metadata, id, exemption._native_value())
        for ref in document_references:
            item = _call(item.with_document_reference, _reference_in(ref)._native_value())
        for tax in tax_subtotals:
            native = tax._native_value()
            item = _call(item.with_tax_subtotal, native)
        return self._set("add_item", item)

    def add_item(self, item: InvoiceLineItem) -> "InvoiceBuilder":
        """Add a line item. ``total_amount`` and ``vat_amount`` are calculated when ``None``."""
        if not isinstance(item, InvoiceLineItem):
            raise TypeError("add_item() takes an InvoiceLineItem")
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
        if item.total_amount is not None:
            b.set_line_net_amount(item.total_amount)
        if item.vat_amount is not None:
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
                getattr(b, "set_" + name)(value)
        for adjustment in item.adjustments:
            (b.add_charge if adjustment.charge else b.add_allowance)(adjustment)
        for ref in item.document_references:
            b.add_document_reference(ref)
        for tax in item.tax_subtotals:
            b.add_tax_subtotal(tax)
        return self._set("add_item", b._invoke("build"))

    # --- document totals --------------------------------------------------------------

    def add_vat_breakdown(self, value: VatBreakdown) -> "InvoiceBuilder":
        return self._set("add_vat_breakdown", value._native_value())

    def add_tax_exemption(self, category: VatCategory, rate: _DecimalIn, exemption: TaxExemption) -> "InvoiceBuilder":
        return self._set("add_tax_exemption", int(category), _decimal(rate), exemption._native_value())

    def add_allowance(self, value: Adjustment, category: Optional[VatCategory] = None,
                      rate: Optional[_DecimalIn] = None, exemption: Optional[TaxExemption] = None) -> "InvoiceBuilder":
        """Append a document-level allowance (discount) in ``category`` at ``rate``.

        ``category``, ``rate`` and ``exemption`` default to the adjustment's own fields.
        """
        return self._add_document_adjustment(False, value, category, rate, exemption, "add_allowance")

    def add_charge(self, value: Adjustment, category: Optional[VatCategory] = None,
                   rate: Optional[_DecimalIn] = None, exemption: Optional[TaxExemption] = None) -> "InvoiceBuilder":
        """Append a document-level charge in ``category`` at ``rate``.

        ``category``, ``rate`` and ``exemption`` default to the adjustment's own fields.
        """
        return self._add_document_adjustment(True, value, category, rate, exemption, "add_charge")

    def add_adjustment(self, adjustment: Adjustment) -> "InvoiceBuilder":
        """Append a document-level allowance or charge described entirely by ``adjustment``:
        its ``charge`` flag, VAT ``category`` and ``rate`` (all required here), exemption,
        tax category count and omitted rate. Values read from ``invoice.adjustments``
        round-trip through it unchanged."""
        return self._add_document_adjustment(adjustment.charge if isinstance(adjustment, Adjustment) else False,
                                             adjustment, None, None, None, "add_adjustment")

    def _add_document_adjustment(self, charge: bool, value: Adjustment, category: Optional[VatCategory],
                                 rate: Optional[_DecimalIn], exemption: Optional[TaxExemption],
                                 method: str) -> "InvoiceBuilder":
        if not isinstance(value, Adjustment):
            raise TypeError(f"{method}() takes an Adjustment")
        category = value.category if category is None else category
        rate = value.rate if rate is None else rate
        if category is None or rate is None:
            raise TypeError(f"{method}() needs a VAT category and rate: pass them as arguments "
                            "or set them on the Adjustment")
        exemption = value.exemption if exemption is None else exemption
        return self._set("add_adjustment", value._document_native(charge, VatCategory(category), rate, exemption))

    def build(self) -> FinalizedInvoice:
        with self._lock:
            self._require_open()
            return FinalizedInvoice._wrap(self._invoke("build"))


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
        """The report as plain JSON-compatible data."""
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
