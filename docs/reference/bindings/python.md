---
title: Python ZATCA E-Invoicing Library
description: Python package reference for fatoority. Learn accepted values, returned objects, context managers, errors, validation reports, and ZATCA API methods.
---

# Python package reference

The published wheels support CPython 3.9–3.13. Use CPython 3.13 for the
[complete sandbox walkthrough](../../guides/sandbox-walkthrough.md). Install
`fatoority` and import `fatoora`; this page describes Python-specific values and behavior.

## Install and import

```sh
python -m pip install fatoority
```

```python
from fatoora import Config, Environment, SigningKey

with Config(Environment.NON_PRODUCTION) as config:
    with SigningKey.generate() as key:
        private_key_pem = key.to_pem()
```

Keep the private key secure. Exported strings and bytes remain in Python memory
after the native owner closes.

The default `Config()` environment is `Environment.NON_PRODUCTION`. Choose
`Environment.SIMULATION` or `Environment.PRODUCTION` explicitly when needed.
Configuration, CSR generation, and credentials must agree on the environment.

## Values and return types

| Value | Python representation |
| --- | --- |
| Text | `str`; embedded NUL is rejected |
| Optional text | `str` or `None`; `""` remains a present empty string |
| Decimal input | `decimal.Decimal`, decimal `str`, or `int`; `float` and `bool` are rejected |
| Decimal output | `decimal.Decimal` |
| Binary input/output | `bytes` |
| Invoice counter | `int` in the unsigned 64-bit range |
| Timestamp input | timezone-aware `datetime.datetime` (converted to UTC) or `"YYYY-MM-DDTHH:MM:SSZ"`; a naive `datetime` raises `ValueError` |
| Timestamp output | timezone-aware `datetime.datetime` in UTC (`issue_datetime`, `signing_time`, `evaluated_at`) |
| Date input / output | `datetime.date` or `"YYYY-MM-DD"` / `datetime.date` |
| UUID input / output | `uuid.UUID` or `str` / `str` (core keeps the UUID text verbatim) |
| Local ZATCA report | frozen `ZatcaReport` dataclass |
| Line item / totals snapshot | `InvoiceLineItem` / `InvoiceTotals` dataclass |

Use exact decimal text such as `"0.3333"`, rather than constructing a `Decimal`
from a binary float. See [decimal numbers and rounding](../numbers.md).

Public choices use `IntEnum` members. Prefer their names to raw integers:

| Enum | Members |
| --- | --- |
| `Environment` | `NON_PRODUCTION`, `SIMULATION`, `PRODUCTION` |
| `InvoiceTypeKind` | `TAX`, `PREPAYMENT`, `CREDIT_NOTE`, `DEBIT_NOTE` |
| `InvoiceSubType` | `STANDARD`, `SIMPLIFIED` |
| `VatCategory` | `EXEMPT`, `STANDARD`, `ZERO`, `OUT_OF_SCOPE` |
| `InvoiceFlag` | `THIRD_PARTY`, `NOMINAL`, `EXPORT`, `SUMMARY`, `SELF_BILLED` |
| `InvoiceOutcome` | `UNKNOWN`, `ACCEPTED`, `REJECTED` |

Country/currency values are strings; their constructors validate and normalize
supported values. See the [invoice model](../invoice-model.md).

## Properties, constructors and chaining

Values you read are properties: `invoice.id`, `invoice.seller.address.city`,
`response.reporting_status`, `config.env`. They are read-only; optional values
are `None` when absent. Operations that compute, convert or consume stay methods:
`hash()`, `to_xml_base64()`, `line_item(index)`, `is_simplified()`,
`ensure_accepted()`, `build()`, `sign(signer)`.

`InvoiceBuilder(invoice_type, invoice_subtype, *, id=..., uuid=..., ...)` takes the
header fields as keyword arguments, and every setter returns the builder, so a
whole invoice is one expression:

```python
from datetime import datetime, timezone
from uuid import uuid4
from fatoora import (Address, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                     PaymentMeans, VatCategory)

invoice = (
    InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
                   id="INV-1", uuid=uuid4(), issue_datetime=datetime.now(timezone.utc),
                   currency="SAR", previous_invoice_hash=previous_hash,
                   invoice_counter=1, vat_category=VatCategory.STANDARD)
    .set_seller("Acme", vat_id="399999999900003",
                address=Address.new("SA", "Riyadh", "King Fahd", "1234", "12222"))
    .add_payment_means(PaymentMeans.new("10"))
    .add_line_item("Item", 1, "PCE", "100", "15", VatCategory.STANDARD)
    .build()
)
```

Arguments after the first are keyword-only for `set_seller`, `set_buyer`,
`add_note(text, *, language=None)`, `CsrProperties.new(*, ...)`, and the
`clearance_status`/`accept_language` arguments of the report and clear methods.
`set_seller` and `set_buyer` take either `address=Address.new(...)` or the address
fields, not both. Parse invoices with `FinalizedInvoice.from_xml(xml)` /
`from_file(path)` and `SignedInvoice.from_xml(xml)` / `from_file(path)`.

## Object lifetime

Native owners support `with` and `close()`. Close objects returned by methods and
properties as well as ones you construct. Closing is idempotent; later calls on,
or property reads from, a closed owner raise `InvalidInputError`. Calls involving the same owners are synchronized.
Independent owners can be used independently.

`InvoiceBuilder.build()` consumes the builder's inner state even when final
validation fails. `FinalizedInvoice.sign(signer)` consumes the finalized invoice
(as does the deprecated `SignedInvoice.into_xml()` for a signed one). The wrapper remains safe
to close, but further use of consumed state fails. Create a new input object to
retry after a consuming operation fails.

Plain strings, bytes, decimal values, and dataclass snapshots do not need
`close()`. Returned object snapshots are independent of their parent, but still
need their own cleanup.

## Errors

Native failures raise subclasses of `FfiError`, which inherits `FatooraError`.
Use `str(error)` for the message, `.code` for its numeric classification,
`.kind` for `FfiErrorKind`, and `.details` for structured data.

```python
from fatoora import SigningKey
from fatoora.errors import FfiError

try:
    SigningKey.from_pem("invalid key")
except FfiError as error:
    print(error.code, str(error))
    print(error.details)
```

| Category | Exception |
| --- | --- |
| Invalid input | `InvalidInputError` |
| Validation rejection | `ValidationError` |
| Parsing | `ParseError` |
| XML | `XmlError` |
| Cryptography | `CryptoError` |
| File access | `IoError` |
| Network | `NetworkError` |
| Authorization | `UnauthorizedError` |
| Internal failure | `InternalError` |
| API response | `ApiError` |

Unknown native codes produce `FfiError` with `.kind` set to `None`; `.code` and
`.details` remain available. Python argument checks can raise `TypeError` or
`ValueError` before a native call. Do not parse message text to identify an error;
use [error codes and structured details](../errors.md).

## Local validation

| Function | Inputs | Returns |
| --- | --- | --- |
| `validate_xml(xml)` | XML `str` | `True` on schema success; raises on malformed XML or schema rejection |
| `validate_zatca(xml, *, previous_invoice_hash=None, evaluated_at=None)` | XML `str`, optional keyword-only context | `ZatcaReport`, including rejection or incomplete coverage |
| `hash_xml(xml)` | XML `str` | Base64 invoice hash `str` |

```python
from pathlib import Path
from fatoora import Severity, validate_zatca

xml = Path("invoice.xml").read_text(encoding="utf-8")
report = validate_zatca(xml)
print(report.is_valid, report.is_complete)
for finding in report.findings:
    if finding.severity is Severity.ERROR:
        print(finding.code, finding.message, finding.location)
```

`previous_invoice_hash` supplies trusted invoice-history context. `evaluated_at`
accepts a timezone-aware `datetime` or RFC 3339 text; omission uses one UTC clock
snapshot. Missing predecessor context makes coverage incomplete. A returned report
does not mean the invoice passed: inspect `is_valid` and the stage findings.

`ZatcaReport` is a frozen dataclass with `is_valid`, `is_complete`, `has_errors`,
`stages`, `evaluated_at` (`datetime`), `profile` and `schema_version`, plus
`findings`, `errors` and `warnings` (all stages, in order), `stage(ZatcaStage.KSA)`,
`to_json()` and `to_dict()`. Each `ZatcaStageReport` has `stage`, `status`,
`findings`, `evaluated_assertions` and `provenance` (`ZatcaRuleSource` with
`stylesheet` and `sha256`, or `None`). Each `ZatcaFinding` has `severity`, `layer`,
`code`, `message`, `location` (`ValidationLocation` with `kind`, `path`, `line`,
`column`, or `None`) and `assertion_site`. `ZatcaStage`, `ZatcaStageStatus`,
`Severity`, `ValidationLayer` and `ValidationLocationKind` are string enums whose
values are the JSON names, so `finding.severity == "error"` also holds.

Execution failures raise an exception and may include a partial report, as a
dictionary, at `error.details["report"]`. Argument failures can have no report. All schemas are
bundled; validation needs no network or external SDK. It does not establish
remote acceptance or certificate issuer trust. See the
[validation reference](../invoice-validation.md).

## ZATCA API methods

Create `ZatcaClient(config)` with a live `Config`. Client methods block the
calling thread and return owned objects:

| Method | Inputs | Returns |
| --- | --- | --- |
| `request_compliance_csid(csr, otp)` | `Csr`, OTP `str` | `ComplianceCsid` |
| `request_production_csid(ccsid)` | `ComplianceCsid` | `ProductionCsid` |
| `renew_csid(pcsid, csr, otp, accept_language=None)` | `ProductionCsid`, `Csr`, OTP, optional language | `ProductionCsid` |
| `check_invoice_compliance(invoice, ccsid)` | `SignedInvoice`, `ComplianceCsid` | `ValidationResponse` |
| `report_simplified_invoice(invoice, pcsid, *, clearance_status, accept_language=None)` | `SignedInvoice`, `ProductionCsid`, keyword-only Boolean and optional language | `ValidationResponse` |
| `clear_standard_invoice(invoice, pcsid, *, clearance_status, accept_language=None)` | `SignedInvoice`, `ProductionCsid`, keyword-only Boolean and optional language | `ValidationResponse` |

`clearance_status` is a required Boolean controlling the request header; use
`False` for ordinary reporting and `True` for clearance. It does not describe the
result. `accept_language` is a string or `None` controlling the language header.
These methods do not expose an asynchronous or cancellation-token parameter.

A successful HTTP request does not guarantee acceptance. Read `outcome` for the
`InvoiceOutcome`, or call `ensure_accepted()` to raise for rejected/unknown
outcomes. `cleared_invoice_xml` is `str | None`; save that XML for an accepted
clearance when present. `reporting_status` and `clearance_status` are also
optional strings. See [API request and response contracts](../api-client.md)
and the [API guide](../../guides/api.md).

## Signed invoice metadata

| Property or method | Value |
| --- | --- |
| `signature` | Encoded signature `str` |
| `public_key` | Encoded public key `str` |
| `cert_hash` | Encoded certificate hash `str` |
| `signed_props_hash` | Encoded signed-properties hash `str` |
| `signing_time` | Timezone-aware (UTC) `datetime` |
| `invoice_hash` | The invoice hash stored in the signature, `str` |
| `hash()` | The invoice hash recomputed from the XML, `str` |

Parsing a signed invoice checks local signature, digest and QR integrity.
Run explicit validation for schema and business-rule checks; issuer trust and clearance require separate evidence. See the
[signing reference](../invoice-signing.md) for key/certificate formats and signing operations.

## Migrate from 0.5

Getters became read-only properties in 0.6. This is a hard break: drop the
parentheses (`invoice.id()` becomes `invoice.id`); calling a property now fails
like calling its value (`TypeError: 'str' object is not callable`). Rows marked
"enforced" were warnings in 0.5.1 and are now `TypeError`s too. Every other 0.5
name below still works in 0.6 and emits a `DeprecationWarning` that names its
replacement; they are removed in 0.7. Run your tests with
`python -W error::DeprecationWarning` (or pytest's
`filterwarnings = ["error::DeprecationWarning"]`) to find the remaining uses.

| 0.5 | 0.6 |
| --- | --- |
| Getter methods: `invoice.id()`, `address.city()`, `response.reporting_status()`, ... (hard break) | Properties: `invoice.id`, `address.city`, `response.reporting_status` |
| `invoice.issue_datetime()`, `signed.signing_time()` return `str` (hard break) | `invoice.issue_datetime`, `signed.signing_time` are aware `datetime`s |
| `issue_date()`, `actual_delivery_date()`, `latest_delivery_date()`, `OriginalInvoiceRef.issue_date()` return `str` (hard break) | Properties of type `datetime.date` |
| `InvoiceBuilder.new(type, subtype, ...)` / `new_with_reference(type, subtype, ref, reason)` | `InvoiceBuilder(type, subtype, *, id=..., ..., original_invoice=ref, original_invoice_reason=reason)` |
| Setters return `None` | Setters return the builder |
| `set_seller(name, country_code, ...)` positional (enforced) | `set_seller(name, *, vat_id=..., address=Address.new(...))` or the address keywords |
| `set_buyer(name, country_code, ...)` positional (enforced) | `set_buyer(name, *, address=..., ...)` |
| `add_note(text, language)` positional (enforced) | `add_note(text, language=...)` |
| `set_note(language, text)` / `set_note_without_language(text)` | `add_note(text, language=...)` / `add_note(text)` |
| `set_payment_means_code(code)` | `add_payment_means(PaymentMeans.new(code))` |
| `builder.flags(value)` | `builder.set_flags(value)` |
| `set_tax_exemption(category, rate, exemption)` | `add_tax_exemption(category, rate, exemption)` |
| `add_document_adjustment(charge, value, category, rate)` | `add_allowance(value, category, rate)` / `add_charge(value, category, rate)` |
| `set_allowance`, `invoice_level_discount`, `invoice_level_charge`, `allowance_reason` | `add_allowance` / `add_charge` with an explicit category and rate, and `Adjustment.with_reason` |
| `LineItemBuilder.new()` | `LineItemBuilder()` |
| `CsrProperties.new(...)` positional (enforced) | `CsrProperties.new(*, ...)` |
| `CsrProperties.from_properties_str(text)` / `parse_csr_config(text)` | `CsrProperties.from_str(text)` |
| `CsrProperties.parse_csr_config_file(path)` | `CsrProperties.from_file(path)` |
| `Signer` | `InvoiceSigner` |
| `CsidCompliance` / `CsidProduction` | `ComplianceCsid` / `ProductionCsid` |
| `client.post_csr_for_ccsid(csr, otp)` / `post_ccsid_for_pcsid(ccsid)` | `client.request_compliance_csid(csr, otp)` / `request_production_csid(ccsid)` |
| `report_simplified_invoice(invoice, pcsid, False)` positional flags (enforced) | `report_simplified_invoice(invoice, pcsid, clearance_status=False)` |
| `parse_finalized_invoice_xml(xml)` / `parse_finalized_invoice_xml_file(path)` | `FinalizedInvoice.from_xml(xml)` / `FinalizedInvoice.from_file(path)` |
| `parse_signed_invoice_xml(xml)` / `parse_signed_invoice_xml_file(path)` | `SignedInvoice.from_xml(xml)` / `SignedInvoice.from_file(path)` |
| `invoice.xml()` (hard break), `to_xml()`, `into_xml()` | `invoice.xml` |
| `invoice.hash_base64()` | `invoice.hash()` |
| `invoice_hash_base64_from_xml_str(xml)` | `hash_xml(xml)` |
| `validate_xml_invoice_from_str(config, xml)` | `validate_xml(xml)` |
| `validate_zatca_invoice_from_str(config, xml, ...)` returns a `dict` | `validate_zatca(xml, ...)` returns a `ZatcaReport`; `report.to_dict()` gives the 0.5 dictionary |
