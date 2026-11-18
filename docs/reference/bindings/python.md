---
title: Python ZATCA E-Invoicing Library
description: Python package reference for fatoority. Learn accepted values, returned objects, context managers, errors, validation reports, and ZATCA API methods.
---

# Python package reference

The published wheels support CPython 3.10–3.13. Use CPython 3.13 for the
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
| ZATCA response status | `str` enum (`ReportingStatus`, `ClearanceStatus`, `ValidationStatus`, `MessageType`) or `None` |
| Invoice data (parties, addresses, notes, references, payment means, adjustments, line items, totals) | frozen dataclasses: `Party`, `Address`, `Identifier`, `InvoiceNote`, `OriginalInvoiceRef`, `PaymentMeans`, `Adjustment`, `InvoiceLineItem`, `InvoiceTotals`, `VatBreakdown`, `TaxExemption` |

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
| `ClearanceMode` | `DISABLED`, `ENABLED` |

Statuses read from a ZATCA response are `str` enums whose values are the wire
text, so `response.reporting_status == "REPORTED"`, `str()`, f-strings and
`json.dumps` give the text ZATCA sent:

| Enum | Members |
| --- | --- |
| `ReportingStatus` | `REPORTED`, `NOT_REPORTED` |
| `ClearanceStatus` | `CLEARED`, `NOT_CLEARED` |
| `ValidationStatus` | `PASS`, `WARNING`, `ERROR` |
| `MessageType` | `INFO`, `WARNING`, `ERROR` |

A status this version does not recognize is kept verbatim, as the core's
`Other(String)` variant keeps it: it is still an instance of the enum, its
`name` is `"OTHER"` and its `value` is the text ZATCA sent. Test
`status.name == "OTHER"` to detect one.

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
from fatoora import (Address, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind, Party,
                     PaymentMeans, VatCategory)

seller = Party("Acme", Address("SA", "Riyadh", "King Fahd", "1234", "12222"),
               vat_id="399999999900003")
invoice = (
    InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
                   id="INV-1", uuid=uuid4(), issue_datetime=datetime.now(timezone.utc),
                   currency="SAR", previous_invoice_hash=previous_hash,
                   invoice_counter=1, vat_category=VatCategory.STANDARD, seller=seller)
    .add_payment_means(PaymentMeans("10"))
    .add_line_item("Item", 1, "PCE", "100", "15", VatCategory.STANDARD)
    .build()
)
```

Every type you create directly has a constructor: `Config(env)`,
`ZatcaClient(config)`, `CsrProperties(*, ...)`,
`ComplianceCsid(env, *, token, secret, request_id=None)`,
`ProductionCsid(...)`, `InvoiceBuilder(...)`, `LineItemBuilder()` and the
invoice dataclasses. Types you obtain from data or from another call use named
factories (`InvoiceSigner.from_pem`, `SigningKey.generate`, `Csr.from_der`,
`SignedInvoice.from_xml`, ...); calling such a class raises `TypeError` naming
them.

Arguments after the first are keyword-only for `set_seller`, `set_buyer`,
`add_note(text, *, language=None)`, `CsrProperties(*, ...)`, the credentials
after `env`, and the `clearance`/`accept_language` arguments of the report and
clear methods.
`set_seller` and `set_buyer` take a `Party`, or a name with either
`address=Address(...)` or the address fields, not both. Parse invoices with `FinalizedInvoice.from_xml(xml)` /
`from_file(path)` and `SignedInvoice.from_xml(xml)` / `from_file(path)`.

## Object lifetime

Native owners support `with` and `close()`. Close objects returned by methods and
properties as well as ones you construct. Closing is idempotent; later calls on,
or property reads from, a closed owner raise `InvalidInputError`. Calls involving the same owners are synchronized.
Independent owners can be used independently.

`InvoiceBuilder.build()` consumes the builder's inner state even when final
validation fails. `FinalizedInvoice.sign(signer)` consumes the finalized invoice
The wrapper remains safe
to close, but further use of consumed state fails. Create a new input object to
retry after a consuming operation fails.

Plain strings, bytes, decimal values, and the dataclass models do not need
`close()`. Returned owners (such as a `ValidationResponse` and its
`validation_results`) are independent of their parent, but still need their own
cleanup.

## Invoice data models

Everything read from an invoice is a frozen dataclass. Values compare with `==`,
copy with `dataclasses.replace`, and go straight back into a builder, so an
invoice read from XML can be rebuilt from its parts. An invoice reads its data
from the native object once, on first access, and caches it.

| Model | Constructor and notes |
| --- | --- |
| `Party` | `Party(name, address=None, vat_id=None, identifiers=())`. `vat_id` and each identifier take an `Identifier` or a string. `other_id` is the first identifier. |
| `Identifier` | `Identifier(value, scheme=None)`: replaces the `VatId`/`OtherId` handles and the `(value, scheme)` tuples of line item identifiers. |
| `Address` | `Address(country_code, city, street, building_number, postal_code, additional_street=None, additional_number=None, district=None, province=None)`. Validated on construction; `"SA"` reads back as `"SAU"`. |
| `InvoiceNote` | `InvoiceNote(text, language_id=None)`; `language` defaults to `"en"`. |
| `OriginalInvoiceRef` | `OriginalInvoiceRef(id, uuid=None, issue_date=None, issue_time=None, document_type_code=None)`: billing references, the referenced original invoice, and line document references. |
| `PaymentMeans` | `PaymentMeans(code, instruction_notes=(), account_identifier=None, payment_terms=None)`. |
| `Adjustment` | `Adjustment(amount, reason=None, reason_code=None, ...)` or `Adjustment.percent_of(base, percent, ...)`: line and document allowances and charges. Document entries read back with `charge`, `category`, `rate`, `exemption`, `tax_category_count` and `omitted_rate`, and round-trip through `builder.add_adjustment(entry)`. |
| `InvoiceLineItem` | As input to `add_item`, `total_amount` and `vat_amount` are optional: they are calculated when `None` and checked when given. |

```python
from fatoora import Adjustment, FinalizedInvoice, InvoiceBuilder, InvoiceLineItem, VatCategory

parsed = FinalizedInvoice.from_file("invoice.xml")
print(parsed.seller.name, parsed.seller.vat_id.value, parsed.seller.address.city)

line = InvoiceLineItem("Notebook", "PCE", quantity=2, unit_price="50", vat_rate=15,
                       vat_category=VatCategory.STANDARD,
                       adjustments=(Adjustment(10, reason="Volume discount"),))
builder = InvoiceBuilder(parsed.invoice_type_kind, parsed.invoice_sub_type,
                         seller=parsed.seller, buyer=parsed.buyer)
builder.add_item(line)
builder.add_charge(Adjustment.percent_of(90, 5, reason="Delivery"), VatCategory.STANDARD, 15)
```

## Errors

Library failures raise a subclass of `FatooraError`, one per `ErrorKind`.
Use `str(error)` or `.message` for the message, `.code` for its numeric
classification, `.kind` for the `ErrorKind`, and `.details` for structured data
as a `dict`.

```python
from fatoora import FatooraError, SigningKey

try:
    SigningKey.from_pem("invalid key")
except FatooraError as error:
    print(error.kind, error.code, error.message)
    print(error.details)
```

| Code | `ErrorKind` | Exception |
| --- | --- | --- |
| 1 | `INVALID_INPUT` | `InvalidInputError` (also a `ValueError`) |
| 2 | `VALIDATION` | `ValidationError` |
| 3 | `PARSE` | `ParseError` |
| 4 | `XML` | `XmlError` |
| 5 | `CRYPTO` | `CryptoError` |
| 6 | `IO` | `IoError` (also an `OSError`) |
| 7 | `NETWORK` | `NetworkError` |
| 8 | `UNAUTHORIZED` | `UnauthorizedError` |
| 9 | `INTERNAL` | `InternalError` |
| 10 | `API` | `ApiError` |

`ErrorKind` is an `IntEnum`, so `error.kind == error.code` for a known code.
Unknown native codes raise plain `FatooraError` with `.kind` set to `None`;
`.code` and `.details` remain available. Python argument checks can raise `TypeError` or
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

Create `ZatcaClient(config, *, timeout=None)` with a live `Config`. Client
methods block the calling thread and return owned objects.

`timeout` bounds each request, from connecting until the response body is read.
Pass seconds as an `int`/`float` or a `datetime.timedelta`; it is rounded up to
whole milliseconds and must be positive and at most about 49.7 days (otherwise
`ValueError`; a non-numeric value raises `TypeError`). A request that exceeds it
raises `NetworkError`. The default `None` applies no client-side timeout.

```python
from datetime import timedelta
from fatoora import Config, Environment, ZatcaClient

client = ZatcaClient(Config(Environment.SIMULATION), timeout=30)
client = ZatcaClient(Config(Environment.SIMULATION), timeout=timedelta(seconds=30))
```


| Method | Inputs | Returns |
| --- | --- | --- |
| `request_compliance_csid(csr, otp)` | `Csr`, OTP `str` | `ComplianceCsid` |
| `request_production_csid(ccsid)` | `ComplianceCsid` | `ProductionCsid` |
| `renew_csid(pcsid, csr, otp, accept_language=None)` | `ProductionCsid`, `Csr`, OTP, optional language | `ProductionCsid` |
| `check_invoice_compliance(invoice, ccsid)` | `SignedInvoice`, `ComplianceCsid` | `ValidationResponse` |
| `report_simplified_invoice(invoice, pcsid, *, clearance, accept_language=None)` | `SignedInvoice`, `ProductionCsid`, keyword-only `ClearanceMode` and optional language | `ValidationResponse` |
| `clear_standard_invoice(invoice, pcsid, *, clearance, accept_language=None)` | `SignedInvoice`, `ProductionCsid`, keyword-only `ClearanceMode` and optional language | `ValidationResponse` |

`clearance` is a required `ClearanceMode` controlling the `Clearance-Status`
request header: `ClearanceMode.DISABLED` sends `0`, `ClearanceMode.ENABLED`
sends `1`. It does not describe the result. `accept_language` is a string or `None` controlling the language header.
These methods do not expose an asynchronous or cancellation-token parameter;
use `timeout` to bound how long a call can block.

A successful HTTP request does not guarantee acceptance. Read `outcome` for the
`InvoiceOutcome`, or call `ensure_accepted()` to raise for rejected/unknown
outcomes. `cleared_invoice_xml` is `str | None`; save that XML for an accepted
clearance when present. `reporting_status` is an optional `ReportingStatus` and
`clearance_status` an optional `ClearanceStatus`. See [API request and response contracts](../api-client.md)
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

## Migrate from earlier versions

The migration guides walk through each upgrade with before-and-after code:
[0.5 to 0.6](../../guides/migrate-0.6/python.md) and
[0.6 to 0.7](../../guides/migrate-0.7/python.md). 0.7 removed every name that
0.6 deprecated; the 0.7 guide lists each one with its replacement.

The 0.6 changes that were hard breaks, with no deprecation period, are below.
Getters became read-only properties: drop the parentheses (`invoice.id()`
becomes `invoice.id`); calling a property fails like calling its value
(`TypeError: 'str' object is not callable`). Rows marked "enforced" were
warnings in 0.5.1 and became `TypeError`s in 0.6.

| 0.5 | 0.6 and later |
| --- | --- |
| Getter methods: `invoice.id()`, `address.city()`, `response.reporting_status()`, `invoice.xml()`, ... | Properties: `invoice.id`, `address.city`, `response.reporting_status`, `invoice.xml` |
| `invoice.issue_datetime()`, `signed.signing_time()` return `str` | `invoice.issue_datetime`, `signed.signing_time` are aware `datetime`s |
| `issue_date()`, `actual_delivery_date()`, `latest_delivery_date()`, `OriginalInvoiceRef.issue_date()` return `str` | Properties of type `datetime.date` |
| Setters return `None` | Setters return the builder |
| `set_seller(name, country_code, ...)` positional (enforced) | `set_seller(Party(...))`, or `set_seller(name, *, vat_id=..., address=Address(...))` or the address keywords |
| `set_buyer(name, country_code, ...)` positional (enforced) | `set_buyer(name, *, address=..., ...)` |
| `add_note(text, language)` positional (enforced) | `add_note(text, language=...)` |
| `CsrProperties.new(...)` positional (enforced) | `CsrProperties(*, ...)` |
| `report_simplified_invoice(invoice, pcsid, False)` positional flags (enforced) | `report_simplified_invoice(invoice, pcsid, clearance=ClearanceMode.DISABLED)` |
| Line `document_references` hold `DocumentReference` values with a `str` `issue_date` | `OriginalInvoiceRef` values whose `issue_date` is a `date` |
| `party.identifiers`, `payment_means.instruction_notes` return lists | Tuples |
| `reporting_status`, `clearance_status`, `results.status`, `message.message_type`, `message.status` return `str` | `str` enums `ReportingStatus`, `ClearanceStatus`, `ValidationStatus`, `MessageType`; comparisons with strings still hold, `type(x) is str` does not |
