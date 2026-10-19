from __future__ import annotations

from contextlib import ExitStack
from dataclasses import dataclass
from decimal import Decimal
from enum import IntEnum
import json
from threading import RLock
from typing import Any, Optional

from . import _native
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

class InvoiceFlag(IntEnum):
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

@dataclass(frozen=True)
class VatBreakdown:
    category: VatCategory
    rate: Decimal
    taxable_amount: Decimal
    tax_amount: Decimal
    exemption: TaxExemption

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


class _Owned:
    """Own a generated object; serialize calls that could consume its state."""
    def __init__(self, handle):
        self._handle = handle
        self._lock = RLock()

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


def _convert(value):
    if isinstance(value, _native.Text):
        return _call(value.value)
    if isinstance(value, _native.Bytes):
        return value._copy()
    if isinstance(value, _native.BytesList):
        return [_call(value.get, i) for i in range(value.len())]
    return value


def _decimal(value: Decimal | str | int) -> str:
    if isinstance(value, (float, bool)) or not isinstance(value, (Decimal, str, int)):
        raise TypeError("decimal values must be Decimal, str, or int")
    return format(value, "f") if isinstance(value, Decimal) else str(value)

def _optional_decimal(value):
    return None if value is None else Decimal(value)

def _identifier(value):
    return None if value is None else (_call(value.value), _call(value.scheme))

def _optional(cls, value):
    return None if value is None else cls(value)

class Config(_Owned):
    def __init__(self, env: Environment = Environment.NON_PRODUCTION):
        super().__init__(_call(_native.Config.new, int(env)))

    def env(self) -> Environment:
        return Environment(self._invoke("env"))

class Signer(_Owned):
    def sign_xml(self, xml: str) -> str:
        return self._invoke("sign_xml", xml)

    @classmethod
    def from_pem(cls, cert_pem: str, key_pem: str) -> 'Signer':
        return Signer(_call(_native.Signer.from_pem, cert_pem, key_pem))

    @classmethod
    def from_der(cls, cert_der: bytes, key_der: bytes) -> 'Signer':
        return Signer(_call(_native.Signer.from_der, cert_der, key_der))

    def certificate_der(self) -> bytes:
        return self._invoke("certificate_der")

    def certificate_pem(self) -> str:
        return self._invoke("certificate_pem")

class SigningKey(_Owned):
    @classmethod
    def from_pem(cls, pem: str) -> 'SigningKey':
        return SigningKey(_call(_native.SigningKey.from_pem, pem))

    @classmethod
    def from_der(cls, der: bytes) -> 'SigningKey':
        return SigningKey(_call(_native.SigningKey.from_der, der))

    @classmethod
    def generate(cls) -> 'SigningKey':
        return SigningKey(_call(_native.SigningKey.generate))

    def to_pem(self) -> str:
        return self._invoke("to_pem")

    def to_der(self) -> bytes:
        return self._invoke("to_der")

class CsrProperties(_Owned):
    @classmethod
    def new(cls, common_name: str, serial_number: str, organization_identifier: str, organization_unit_name: str, organization_name: str, country_name: str, invoice_type: str, location_address: str, industry_business_category: str) -> 'CsrProperties':
        return CsrProperties(_call(_native.CsrProperties.new, common_name, serial_number, organization_identifier, organization_unit_name, organization_name, country_name, invoice_type, location_address, industry_business_category))

    @classmethod
    def from_properties_str(cls, properties: str) -> 'CsrProperties':
        return CsrProperties(_call(_native.CsrProperties.from_properties_str, properties))

    @classmethod
    def parse_csr_config(cls, properties: str) -> 'CsrProperties':
        return CsrProperties(_call(_native.CsrProperties.from_properties_str, properties))

    @classmethod
    def parse_csr_config_file(cls, path: str) -> 'CsrProperties':
        return CsrProperties(_call(_native.CsrProperties.parse_csr_config_file, path))

    def build(self, key: SigningKey, env: Environment) -> 'Csr':
        return Csr(self._invoke("build", key, int(env)))

class Csr(_Owned):
    @classmethod
    def from_der(cls, der: bytes) -> 'Csr':
        return Csr(_call(_native.Csr.from_der, der))

    def to_base64(self) -> str:
        return self._invoke("to_base64")

    def to_pem_base64(self) -> str:
        return self._invoke("to_pem_base64")

    def to_der(self) -> bytes:
        return self._invoke("to_der")

    def to_pem(self) -> str:
        return self._invoke("to_pem")

    def subject_string(self) -> str:
        return self._invoke("subject_string")

    def extension_values_der(self) -> list[bytes]:
        return self._invoke("extension_values_der")

class CsidCompliance(_Owned):
    @classmethod
    def new(cls, env: Environment, token: str, secret: str, request_id: Optional[str]=None) -> 'CsidCompliance':
        return CsidCompliance(_call(_native.CsidCompliance.create, int(env), request_id, token, secret))

    def request_id(self) -> str:
        return self._invoke("request_id") or ""

    def env(self) -> Environment:
        return Environment(self._invoke("env"))

    def binary_security_token(self) -> str:
        return self._invoke("binary_security_token")

    def secret(self) -> str:
        return self._invoke("secret")

class CsidProduction(_Owned):
    @classmethod
    def new(cls, env: Environment, token: str, secret: str, request_id: Optional[str]=None) -> 'CsidProduction':
        return CsidProduction(_call(_native.CsidProduction.create, int(env), request_id, token, secret))

    def request_id(self) -> str:
        return self._invoke("request_id") or ""

    def env(self) -> Environment:
        return Environment(self._invoke("env"))

    def binary_security_token(self) -> str:
        return self._invoke("binary_security_token")

    def secret(self) -> str:
        return self._invoke("secret")

class ValidationMessage(_Owned):
    def message_type(self) -> Optional[str]:
        return self._invoke("message_type")

    def code(self) -> Optional[str]:
        return self._invoke("code")

    def category(self) -> Optional[str]:
        return self._invoke("category")

    def message(self) -> Optional[str]:
        return self._invoke("message")

    def status(self) -> Optional[str]:
        return self._invoke("status")

class ValidationResults(_Owned):
    def status(self) -> Optional[str]:
        return self._invoke("status")

    def info_messages(self) -> list[ValidationMessage]:
        return [ValidationMessage(self._invoke("info_message", i)) for i in range(self._invoke("info_len"))]

    def warning_messages(self) -> list[ValidationMessage]:
        return [ValidationMessage(self._invoke("warning_message", i)) for i in range(self._invoke("warning_len"))]

    def error_messages(self) -> list[ValidationMessage]:
        return [ValidationMessage(self._invoke("error_message", i)) for i in range(self._invoke("error_len"))]

class ValidationResponse(_Owned):
    def http_status(self) -> Optional[int]:
        return self._invoke("http_status")

    def outcome(self) -> InvoiceOutcome:
        value = self._invoke("outcome")
        if value == _native.InvoiceOutcome.Accepted:
            return InvoiceOutcome.ACCEPTED
        if value == _native.InvoiceOutcome.Rejected:
            return InvoiceOutcome.REJECTED
        return InvoiceOutcome.UNKNOWN

    def ensure_accepted(self) -> None:
        return self._invoke("ensure_accepted")

    def cleared_invoice_base64(self) -> Optional[str]:
        return self._invoke("cleared_invoice_base64")

    def cleared_invoice_xml(self) -> Optional[str]:
        return self._invoke("cleared_invoice_xml")

    def reporting_status(self) -> Optional[str]:
        return self._invoke("reporting_status")

    def clearance_status(self) -> Optional[str]:
        return self._invoke("clearance_status")

    def qr_seller_status(self) -> Optional[str]:
        return self._invoke("qr_seller_status")

    def qr_buyer_status(self) -> Optional[str]:
        return self._invoke("qr_buyer_status")

    def validation_results(self) -> ValidationResults:
        return ValidationResults(self._invoke("validation_results"))

class VatId(_Owned):
    def value(self) -> str:
        return self._invoke("value")

class OtherId(_Owned):
    def value(self) -> str:
        return self._invoke("value")

    def scheme(self) -> Optional[str]:
        return self._invoke("scheme")

class Address(_Owned):
    def with_province(self, province: str) -> "Address":
        return Address(self._invoke("with_province", province))

    def province(self) -> Optional[str]:
        return self._invoke("province")

    @classmethod
    def new(cls, country_code: str, city: str, street: str, building_number: str, postal_code: str, additional_street: Optional[str]=None, additional_number: Optional[str]=None, district: Optional[str]=None) -> 'Address':
        return Address(_call(_native.Address.new, country_code, city, street, building_number, postal_code, additional_street, additional_number, district))

    def country_code(self) -> str:
        return self._invoke("country_code")

    def city(self) -> str:
        return self._invoke("city")

    def street(self) -> str:
        return self._invoke("street")

    def additional_street(self) -> Optional[str]:
        return self._invoke("additional_street")

    def building_number(self) -> str:
        return self._invoke("building_number")

    def additional_number(self) -> Optional[str]:
        return self._invoke("additional_number")

    def postal_code(self) -> str:
        return self._invoke("postal_code")

    def district(self) -> Optional[str]:
        return self._invoke("district")

class Party(_Owned):
    def identifiers(self) -> list[OtherId]:
        return [OtherId(self._invoke("identifier", i)) for i in range(self._invoke("identifiers_len"))]

    def name(self) -> str:
        return self._invoke("name")

    def address(self) -> Optional[Address]:
        return _optional(Address, self._invoke("address"))

    def vat_id(self) -> Optional[VatId]:
        return _optional(VatId, self._invoke("vat_id"))

    def other_id(self) -> Optional[OtherId]:
        return _optional(OtherId, self._invoke("other_id"))

class InvoiceNote(_Owned):
    def language_id(self) -> Optional[str]:
        return self._invoke("language_id")

    def language(self) -> str:
        return self._invoke("language")

    def text(self) -> str:
        return self._invoke("text")

class OriginalInvoiceRef(_Owned):
    @classmethod
    def new(cls, id: str, uuid: Optional[str] = None, date: Optional[str] = None, time: Optional[str] = None, type_code: Optional[str] = None) -> 'OriginalInvoiceRef':
        return cls(_call(_native.OriginalInvoiceRef.new, id, uuid, date, time, type_code))

    def issue_time(self) -> Optional[str]:
        return self._invoke("issue_time")

    def document_type_code(self) -> Optional[str]:
        return self._invoke("document_type_code")

    def id(self) -> str:
        return self._invoke("id")

    def uuid(self) -> Optional[str]:
        return self._invoke("uuid")

    def issue_date(self) -> Optional[str]:
        return self._invoke("issue_date")

class ZatcaClient(_Owned):
    def __init__(self, config: Config):
        super().__init__(_call(_native.ZatcaClient.create, config))

    def post_csr_for_ccsid(self, csr: Csr, otp: str) -> CsidCompliance:
        return CsidCompliance(self._invoke("_blocking_post_csr_for_ccsid", csr, otp))

    def post_ccsid_for_pcsid(self, ccsid: CsidCompliance) -> CsidProduction:
        return CsidProduction(self._invoke("_blocking_post_ccsid_for_pcsid", ccsid))

    def renew_csid(self, pcsid: CsidProduction, csr: Csr, otp: str, accept_language: Optional[str]=None) -> CsidProduction:
        return CsidProduction(self._invoke("_blocking_renew_csid", pcsid, csr, otp, accept_language))

    def check_invoice_compliance(self, invoice: 'SignedInvoice', ccsid: CsidCompliance) -> 'ValidationResponse':
        return ValidationResponse(self._invoke("_blocking_check_invoice_compliance", invoice, ccsid))

    def report_simplified_invoice(self, invoice: 'SignedInvoice', pcsid: CsidProduction, clearance_status: bool, accept_language: Optional[str]=None) -> 'ValidationResponse':
        return ValidationResponse(self._invoke("_blocking_report_simplified_invoice", invoice, pcsid, clearance_status, accept_language))

    def clear_standard_invoice(self, invoice: 'SignedInvoice', pcsid: CsidProduction, clearance_status: bool, accept_language: Optional[str]=None) -> 'ValidationResponse':
        return ValidationResponse(self._invoke("_blocking_clear_standard_invoice", invoice, pcsid, clearance_status, accept_language))

class _Invoice(_Owned):
    def issue_time(self) -> str:
        return self._data("issue_time")

    def issue_date(self) -> str:
        return self._data("issue_date")

    def _data(self, name, *args):
        data = self._invoke("data")
        return _call(getattr(data, name), *args)

    def id(self) -> str:
        return self._data("id")

    def uuid(self) -> str:
        return self._data("uuid")

    def issue_datetime(self) -> str:
        return self._data("issue_datetime")

    def currency(self) -> str:
        return self._data("currency")

    def previous_invoice_hash(self) -> str:
        return self._data("previous_invoice_hash")

    def invoice_counter(self) -> int:
        return self._data("invoice_counter")

    def payment_means_code(self) -> str:
        return self._data("payment_means_code")

    def vat_category(self) -> VatCategory:
        return VatCategory(self._data("vat_category"))

    def invoice_level_charge(self) -> Decimal:
        return Decimal(self._data("invoice_level_charge"))

    def invoice_level_discount(self) -> Decimal:
        return Decimal(self._data("invoice_level_discount"))

    def allowance_reason(self) -> Optional[str]:
        return self._data("allowance_reason")

    def seller(self) -> Party:
        return Party(self._data("seller"))

    def buyer(self) -> Optional[Party]:
        return _optional(Party, self._data("buyer"))

    def note(self) -> Optional[InvoiceNote]:
        return _optional(InvoiceNote, self._data("note"))

    def invoice_type_kind(self) -> InvoiceTypeKind:
        return InvoiceTypeKind(self._data("invoice_type_kind"))

    def invoice_sub_type(self) -> InvoiceSubType:
        return InvoiceSubType(self._data("invoice_sub_type"))

    def original_invoice_ref(self) -> Optional[OriginalInvoiceRef]:
        return _optional(OriginalInvoiceRef, self._data("original_invoice_ref"))

    def original_invoice_reason(self) -> Optional[str]:
        return self._data("original_invoice_reason")

    def actual_delivery_date(self) -> Optional[str]:
        return self._data("actual_delivery_date")

    def latest_delivery_date(self) -> Optional[str]:
        return self._data("latest_delivery_date")

    def payment_instruction(self) -> Optional[str]:
        return self._data("payment_instruction")

    def tax_currency(self) -> Optional[str]:
        return self._data("tax_currency")

    def tax_currency_amount(self) -> Optional[Decimal]:
        value = self._data("tax_currency_amount")
        return None if value is None else Decimal(value)

    def notes(self) -> list[InvoiceNote]:
        data = self._invoke("data")
        return [InvoiceNote(_call(data.note_at, i)) for i in range(_call(data.notes_len))]

    def billing_references(self) -> list[OriginalInvoiceRef]:
        data = self._invoke("data")
        return [OriginalInvoiceRef(_call(data.billing_reference, i)) for i in range(_call(data.billing_references_len))]

    def payment_means(self) -> list[PaymentMeans]:
        data = self._invoke("data")
        return [PaymentMeans(_call(data.payment_means, i)) for i in range(_call(data.payment_means_len))]

    def purchase_order_id(self) -> Optional[str]:
        return _call(self._invoke("data").purchase_order_id)

    def contract_id(self) -> Optional[str]:
        return _call(self._invoke("data").contract_id)

    def adjustments(self) -> list[AllowanceCharge]:
        data = self._invoke("data")
        result = []
        for i in range(_call(data.adjustments_len)):
            a = _call(data.adjustment, i)
            result.append(AllowanceCharge(_call(a.is_charge), Decimal(_call(a.amount)), VatCategory(_call(a.category)), Decimal(_call(a.rate)), _call(a.reason), _call(a.reason_code), TaxExemption._from_native(_call(a.exemption)), _call(a.tax_category_count), _optional_decimal(_call(a.base_amount)), _optional_decimal(_call(a.percentage))))
        return result

    def vat_breakdown(self) -> list[VatBreakdown]:
        totals = self._invoke("totals")
        result = []
        for i in range(_call(totals.vat_breakdown_len)):
            group = _call(totals.vat_breakdown, i)
            result.append(VatBreakdown(VatCategory(_call(group.category)), Decimal(_call(group.rate)), Decimal(_call(group.taxable_amount)), Decimal(_call(group.tax_amount)), TaxExemption._from_native(_call(group.exemption))))
        return result

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
            subtotals.append(VatBreakdown(VatCategory(_call(tax.category)), Decimal(_call(tax.rate)), Decimal(_call(tax.taxable_amount)), Decimal(_call(tax.tax_amount)), TaxExemption._from_native(_call(tax.exemption))))
        return InvoiceLineItem(description=(_call(item.description)), unit_code=(_call(item.unit_code)), quantity=Decimal(_call(item.quantity)), unit_price=Decimal(_call(item.unit_price)), total_amount=Decimal(_call(item.total_amount)), vat_rate=Decimal(_call(item.vat_rate)), vat_amount=Decimal(_call(item.vat_amount)), vat_category=VatCategory(_call(item.vat_category)), id=_call(item.id), exemption=TaxExemption._from_native(_call(item.exemption)), document_references=tuple(references), tax_subtotals=tuple(subtotals),
            item_price_base_quantity=_optional_decimal(_call(item.item_price_base_quantity)),
            item_price_base_quantity_unit_code=_call(item.item_price_base_quantity_unit_code),
            item_gross_price=_optional_decimal(_call(item.item_gross_price)),
            item_price_discount=_optional_decimal(_call(item.item_price_discount)),
            adjustments=tuple(AllowanceCharge(_call(item.adjustment_is_charge, i), Decimal(_call(a.value)), VatCategory(_call(item.vat_category)), Decimal(_call(item.vat_rate)), _call(a.reason), _call(a.reason_code), base_amount=_optional_decimal(_call(a.base_amount)), percentage=_optional_decimal(_call(a.percent))) for i in range(_call(item.adjustments_len)) for a in [_call(item.adjustment, i)]),
            item_seller_identifier=_identifier(_call(item.item_seller_identifier)),
            item_buyer_identifier=_identifier(_call(item.item_buyer_identifier)),
            item_standard_identifier=_identifier(_call(item.item_standard_identifier)))

    def line_items(self) -> list[InvoiceLineItem]:
        data = self._invoke("data")
        return [self._line_item_value(_call(data.line_item, i))
                for i in range(_call(data.line_items_len))]

    def totals(self) -> InvoiceTotals:
        totals = self._invoke("totals")
        return InvoiceTotals(**{name: Decimal(_call(getattr(totals, name))) for name in InvoiceTotals.__dataclass_fields__})

    def flags_raw(self) -> int:
        return self._data("flags_raw")

    def flags(self) -> set[InvoiceFlag]:
        bits = self.flags_raw()
        return {flag for flag in InvoiceFlag if bits & flag.value}

    def is_third_party(self) -> bool:
        return InvoiceFlag.THIRD_PARTY in self.flags()

    def is_nominal(self) -> bool:
        return InvoiceFlag.NOMINAL in self.flags()

    def is_export(self) -> bool:
        return InvoiceFlag.EXPORT in self.flags()

    def is_summary(self) -> bool:
        return InvoiceFlag.SUMMARY in self.flags()

    def is_self_billed(self) -> bool:
        return InvoiceFlag.SELF_BILLED in self.flags()

    def is_simplified(self) -> bool:
        return self.invoice_sub_type() == InvoiceSubType.SIMPLIFIED

    def xml(self) -> str:
        return self._invoke("xml")

    def to_xml(self) -> str:
        return self.xml()

    def hash_base64(self) -> str:
        return self._invoke("hash_base64")


class FinalizedInvoice(_Invoice):
    def sign(self, signer: Signer) -> SignedInvoice:
        return SignedInvoice(signer._invoke("sign", self))


class SignedInvoice(_Invoice):
    def into_xml(self) -> str:
        return self._invoke("into_xml")

    def to_xml_base64(self) -> str:
        return self._invoke("to_xml_base64")

    def qr_code(self) -> str:
        return self._invoke("qr_code")

    def invoice_hash(self) -> str:
        return self._invoke("invoice_hash")

    def signature(self) -> str:
        return self._invoke("signature")

    def public_key(self) -> str:
        return self._invoke("public_key")

    def zatca_key_signature(self) -> Optional[str]:
        return self._invoke("zatca_key_signature")

    def cert_hash(self) -> str:
        return self._invoke("cert_hash")

    def signed_props_hash(self) -> str:
        return self._invoke("signed_props_hash")

    def signing_time(self) -> str:
        return self._invoke("signing_time")

    def issuer(self) -> str:
        return self._invoke("issuer")

    def serial(self) -> str:
        return self._invoke("serial")

class Adjustment(_Owned):
    @classmethod
    def amount(cls, value: Decimal | str | int) -> "Adjustment":
        return cls(_call(_native.Adjustment.amount, _decimal(value)))

    @classmethod
    def percentage(cls, base: Decimal | str | int, percent: Decimal | str | int) -> "Adjustment":
        return cls(_call(_native.Adjustment.percentage, _decimal(base), _decimal(percent)))

    def with_supplied_amount(self, value: Decimal | str | int) -> "Adjustment":
        return Adjustment(self._invoke("with_supplied_amount", _decimal(value)))

    def with_reason(self, reason: Optional[str] = None, code: Optional[str] = None) -> "Adjustment":
        return Adjustment(self._invoke("with_reason", reason, code))

    def value(self) -> Decimal:
        return Decimal(self._invoke("value"))

    def base_amount(self) -> Optional[Decimal]:
        value = self._invoke("base_amount")
        return None if value is None else Decimal(value)

    def percent(self) -> Optional[Decimal]:
        value = self._invoke("percent")
        return None if value is None else Decimal(value)

    def reason(self) -> Optional[str]:
        return self._invoke("reason")

    def reason_code(self) -> Optional[str]:
        return self._invoke("reason_code")

class PaymentMeans(_Owned):
    def with_instruction_note(self, value: str) -> "PaymentMeans":
        return PaymentMeans(self._invoke("with_instruction_note", value))

    def instruction_notes(self) -> list[str]:
        return [self._invoke("instruction_note_at", i) for i in range(self._invoke("instruction_notes_len"))]

    @classmethod
    def new(cls, code: str, instruction: Optional[str] = None) -> "PaymentMeans":
        return cls(_call(_native.PaymentMeans.new, code, instruction))

    def with_account(self, id: Optional[str] = None, terms: Optional[str] = None) -> "PaymentMeans":
        return PaymentMeans(self._invoke("with_account", id, terms))

    def code(self) -> str:
        return self._invoke("code")

    def instruction_note(self) -> Optional[str]:
        return self._invoke("instruction_note")

    def has_account(self) -> bool:
        return self._invoke("has_account")

    def account_identifier(self) -> Optional[str]:
        return self._invoke("account_identifier")

    def payment_terms(self) -> Optional[str]:
        return self._invoke("payment_terms")

class LineItemBuilder(_Owned):
    @classmethod
    def new(cls) -> "LineItemBuilder":
        return cls(_call(_native.LineItemBuilder.new))

    def build(self) -> InvoiceLineItem:
        return FinalizedInvoice._line_item_value(self._invoke("build"))

    def set_item_name(self, value: str) -> None:
        self._invoke("set_item_name", value)

    def set_invoiced_quantity_unit_code(self, value: str) -> None:
        self._invoke("set_invoiced_quantity_unit_code", value)

    def set_item_price_base_quantity_unit_code(self, value: str) -> None:
        self._invoke("set_item_price_base_quantity_unit_code", value)

    def set_id(self, value: str) -> None:
        self._invoke("set_id", value)

    def set_invoiced_quantity(self, value: Decimal | str | int) -> None:
        self._invoke("set_invoiced_quantity", _decimal(value))

    def set_item_net_price(self, value: Decimal | str | int) -> None:
        self._invoke("set_item_net_price", _decimal(value))

    def set_item_gross_price(self, value: Decimal | str | int) -> None:
        self._invoke("set_item_gross_price", _decimal(value))

    def set_item_price_discount(self, value: Decimal | str | int) -> None:
        self._invoke("set_item_price_discount", _decimal(value))

    def set_item_price_base_quantity(self, value: Decimal | str | int) -> None:
        self._invoke("set_item_price_base_quantity", _decimal(value))

    def set_line_net_amount(self, value: Decimal | str | int) -> None:
        self._invoke("set_line_net_amount", _decimal(value))

    def set_line_vat_amount(self, value: Decimal | str | int) -> None:
        self._invoke("set_line_vat_amount", _decimal(value))

    def set_line_amount_with_vat(self, value: Decimal | str | int) -> None:
        self._invoke("set_line_amount_with_vat", _decimal(value))

    def set_vat_rate(self, value: Decimal | str | int) -> None:
        self._invoke("set_vat_rate", _decimal(value))

    def set_vat_category(self, value: VatCategory) -> None:
        self._invoke("set_vat_category", int(value))

    def set_exemption(self, value: TaxExemption) -> None:
        self._invoke("set_exemption", value._native_value())

    def add_allowance(self, value: Adjustment) -> None:
        self._invoke("add_allowance", value)

    def add_charge(self, value: Adjustment) -> None:
        self._invoke("add_charge", value)

    def add_document_reference(self, value: OriginalInvoiceRef) -> None:
        self._invoke("add_document_reference", value)

    def add_tax_subtotal(self, value: VatBreakdown) -> None:
        native = _call(_native.VatBreakdown.new, int(value.category), _decimal(value.rate), _decimal(value.taxable_amount), _decimal(value.tax_amount), value.exemption._native_value())
        self._invoke("add_tax_subtotal", native)

    def set_item_seller_identifier(self, value: str, scheme: Optional[str] = None) -> None:
        self._invoke("set_item_seller_identifier", value, scheme)

    def set_item_buyer_identifier(self, value: str, scheme: Optional[str] = None) -> None:
        self._invoke("set_item_buyer_identifier", value, scheme)

    def set_item_standard_identifier(self, value: str, scheme: Optional[str] = None) -> None:
        self._invoke("set_item_standard_identifier", value, scheme)

class InvoiceBuilder(_Owned):
    def add_seller_identifier(self, value: str, scheme: Optional[str] = None) -> None:
        self._invoke("add_seller_identifier", value, scheme)

    def add_buyer_identifier(self, value: str, scheme: Optional[str] = None) -> None:
        self._invoke("add_buyer_identifier", value, scheme)

    def add_vat_breakdown(self, value: VatBreakdown) -> None:
        native = _call(_native.VatBreakdown.new, int(value.category), _decimal(value.rate), _decimal(value.taxable_amount), _decimal(value.tax_amount), value.exemption._native_value())
        self._invoke("add_vat_breakdown", native)

    def add_payment_means(self, value: PaymentMeans) -> None:
        self._invoke("add_payment_means", value)

    def add_note(self, text: str, language: Optional[str] = None) -> None:
        self._invoke("add_note", language, text)

    def add_billing_reference(self, value: OriginalInvoiceRef) -> None:
        self._invoke("add_billing_reference", value)

    def set_purchase_order_id(self, value: str) -> None:
        self._invoke("set_purchase_order_id", value)

    def set_contract_id(self, value: str) -> None:
        self._invoke("set_contract_id", value)

    def set_buyer_without_address(self, name: str, vat_id: Optional[str] = None, other_id: Optional[str] = None, scheme: Optional[str] = None) -> None:
        self._invoke("set_buyer_without_address", name, vat_id, other_id, scheme)

    def add_document_adjustment(self, charge: bool, value: Adjustment, category: VatCategory, rate: Decimal | str | int, exemption: TaxExemption = TaxExemption()) -> None:
        self._invoke("add_document_adjustment", charge, value, int(category), _decimal(rate), exemption._native_value())

    def add_item(self, item: InvoiceLineItem) -> None:
        b = LineItemBuilder.new()
        b.set_item_name(item.description)
        b.set_invoiced_quantity(item.quantity)
        if item.unit_code:
            b.set_invoiced_quantity_unit_code(item.unit_code)
        b.set_item_net_price(item.unit_price)
        b.set_vat_rate(item.vat_rate)
        b.set_vat_category(item.vat_category)
        b.set_line_net_amount(item.total_amount)
        b.set_line_vat_amount(item.vat_amount)
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


    @classmethod
    def new(cls, invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, original_invoice_id: Optional[str]=None, original_invoice_uuid: Optional[str]=None, original_invoice_issue_date: Optional[str]=None, original_invoice_reason: Optional[str]=None) -> 'InvoiceBuilder':
        return cls(_call(_native.InvoiceBuilder.new, int(invoice_type), int(invoice_subtype), original_invoice_id, original_invoice_uuid, original_invoice_issue_date, original_invoice_reason))

    @classmethod
    def new_with_reference(cls, invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, reference: OriginalInvoiceRef, reason: str) -> 'InvoiceBuilder':
        return cls(_call(_native.InvoiceBuilder.new_with_reference, int(invoice_type), int(invoice_subtype), reference, reason))

    def set_note_without_language(self, text: str) -> None:
        self._invoke("set_note_without_language", text)

    def set_delivery(self, actual: Optional[str] = None, latest: Optional[str] = None) -> None:
        self._invoke("set_delivery", actual, latest)

    def set_tax_currency(self, currency: str, amount: Decimal | str | int) -> None:
        self._invoke("set_tax_currency", currency, _decimal(amount))

    def set_tax_exemption(self, category: VatCategory, rate: Decimal | str | int, exemption: TaxExemption) -> None:
        self._invoke("set_tax_exemption", int(category), _decimal(rate), exemption._native_value())

    def set_payment_instruction(self, instruction: str) -> None:
        self._invoke("set_payment_instruction", instruction)

    def set_prepaid_amount(self, amount: Decimal | str | int) -> None:
        self._invoke("set_prepaid_amount", _decimal(amount))

    def set_payable_rounding_amount(self, amount: Decimal | str | int) -> None:
        self._invoke("set_payable_rounding_amount", _decimal(amount))

    def add_adjustment(self, adjustment: AllowanceCharge) -> None:
        if adjustment.percentage is not None:
            value = Adjustment.percentage(adjustment.base_amount, adjustment.percentage).with_supplied_amount(adjustment.amount).with_reason(adjustment.reason, adjustment.reason_code)
            self.add_document_adjustment(adjustment.charge, value, adjustment.category, adjustment.rate, adjustment.exemption)
            return
        native = _call(_native.AllowanceCharge.new, adjustment.charge, _decimal(adjustment.amount), adjustment.reason, adjustment.reason_code, int(adjustment.category), _decimal(adjustment.rate), adjustment.exemption._native_value())
        native = _call(native.with_tax_category_count, adjustment.tax_category_count)
        self._invoke("add_adjustment", native)

    def set_id(self, invoice_id: str) -> None:
        self._invoke("set_id", invoice_id)

    def set_uuid(self, uuid: str) -> None:
        self._invoke("set_uuid", uuid)

    def set_issue_datetime(self, issue_datetime: str) -> None:
        self._invoke("set_issue_datetime", issue_datetime)

    def set_currency(self, currency_code: str) -> None:
        self._invoke("set_currency", currency_code)

    def set_previous_invoice_hash(self, previous_invoice_hash: str) -> None:
        self._invoke("set_previous_invoice_hash", previous_invoice_hash)

    def set_invoice_counter(self, invoice_counter: int) -> None:
        self._invoke("set_invoice_counter", invoice_counter)

    def set_payment_means_code(self, payment_means_code: str) -> None:
        self._invoke("set_payment_means_code", payment_means_code)

    def set_vat_category(self, vat_category: VatCategory) -> None:
        self._invoke("set_vat_category", int(vat_category))

    def set_seller(self, name: str, country_code: str, city: str, street: str, building_number: str, postal_code: str, vat_id: str, additional_street: Optional[str]=None, additional_number: Optional[str]=None, district: Optional[str]=None, other_id: Optional[str]=None, other_id_scheme: Optional[str]=None) -> None:
        address = Address.new(country_code, city, street, building_number, postal_code, additional_street, additional_number, district)
        self._invoke("set_seller", name, address, vat_id, other_id, other_id_scheme)

    def add_line_item(self, description: str, quantity: Decimal | str | int, unit_code: str, unit_price: Decimal | str | int, vat_rate: Decimal | str | int, vat_category: VatCategory, *, id: Optional[str] = None, exemption: TaxExemption = TaxExemption(), document_references: tuple[DocumentReference, ...] = (), tax_subtotals: tuple[VatBreakdown, ...] = ()) -> None:
        item = _call(_native.InvoiceLineItem.new, description, _decimal(quantity), unit_code, _decimal(unit_price), _decimal(vat_rate), int(vat_category))
        item = _call(item.with_metadata, id, exemption._native_value())
        for ref in document_references:
            native = _call(_native.OriginalInvoiceRef.new, ref.id, ref.uuid, ref.issue_date, ref.issue_time, ref.document_type_code)
            item = _call(item.with_document_reference, native)
        for tax in tax_subtotals:
            native = _call(_native.VatBreakdown.new, int(tax.category), _decimal(tax.rate), _decimal(tax.taxable_amount), _decimal(tax.tax_amount), tax.exemption._native_value())
            item = _call(item.with_tax_subtotal, native)
        self._invoke("add_item", item)

    def set_buyer(self, name: str, country_code: str, city: str, street: str, building_number: str, postal_code: str, vat_id: Optional[str]=None, other_id: Optional[str]=None, other_id_scheme: Optional[str]=None, additional_street: Optional[str]=None, additional_number: Optional[str]=None, district: Optional[str]=None) -> None:
        address = Address.new(country_code, city, street, building_number, postal_code, additional_street, additional_number, district)
        self._invoke("set_buyer", name, address, vat_id, other_id, other_id_scheme)

    def set_note(self, language: str, text: str) -> None:
        self._invoke("set_note", language, text)

    def set_allowance(self, reason: str, amount: Decimal | str | int) -> None:
        self._invoke("set_allowance", reason, _decimal(amount))

    def invoice_level_charge(self, charge: Decimal | str | int) -> None:
        self._invoke("invoice_level_charge", _decimal(charge))

    def invoice_level_discount(self, discount: Decimal | str | int) -> None:
        self._invoke("invoice_level_discount", _decimal(discount))

    def allowance_reason(self, reason: str) -> None:
        self._invoke("allowance_reason", reason)

    def flags(self, flags: int) -> None:
        self._invoke("flags", flags)

    def build(self) -> FinalizedInvoice:
        return FinalizedInvoice(self._invoke("build"))

def parse_finalized_invoice_xml(xml: str) -> FinalizedInvoice:
    return FinalizedInvoice(_call(_native.FinalizedInvoice.from_xml, xml))


def parse_finalized_invoice_xml_file(path: str) -> FinalizedInvoice:
    return FinalizedInvoice(_call(_native.FinalizedInvoice.from_file, str(path)))


def parse_signed_invoice_xml(xml: str) -> SignedInvoice:
    return SignedInvoice(_call(_native.SignedInvoice.from_xml, xml))


def parse_signed_invoice_xml_file(path: str) -> SignedInvoice:
    return SignedInvoice(_call(_native.SignedInvoice.from_file, str(path)))


def validate_xml_invoice_from_str(config: Config, xml: str) -> bool:
    return _call(_native.Xml.validate, config, xml)


def invoice_hash_base64_from_xml_str(xml: str) -> str:
    return _call(_native.Xml.hash, xml)


def validate_zatca_invoice_from_str(
    config: Config,
    xml: str,
    *,
    previous_invoice_hash: Optional[str] = None,
    evaluated_at: Optional[str] = None,
) -> dict[str, Any]:
    """Return the local ZATCA report, including rejection or incomplete coverage.

    Inspect ``is_valid`` before accepting an invoice. Missing predecessor context
    makes ``is_complete`` false. ``evaluated_at`` accepts RFC3339 text; defaults
    use one UTC clock snapshot. Execution failures raise FatooraError with the
    partial report in ``error.details['report']``. Local integrity does not
    establish certificate issuer trust or remote acceptance.
    """
    if not isinstance(xml, str):
        raise TypeError("xml must be a string")
    for name, value in (("xml", xml), ("previous_invoice_hash", previous_invoice_hash),
                        ("evaluated_at", evaluated_at)):
        if value is not None and not isinstance(value, str):
            raise TypeError(f"{name} must be a string or None")
        if value is not None and "\x00" in value:
            raise ValueError(f"{name} contains interior NUL")
    options = json.dumps({"previous_invoice_hash": previous_invoice_hash,
                          "evaluated_at": evaluated_at})
    return json.loads(_call(_native.Xml.validate_zatca, config, xml, options))
