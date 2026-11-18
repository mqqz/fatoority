---
title: Migrate a Python Project from fatoority 0.6 to 0.7
description: Every Python name removed in fatoority 0.7, each with the 0.6 replacement to use instead.
---

# Migrate from 0.6 to 0.7: Python

This page covers the `fatoority` package (imported as `fatoora`). The
[overview](index.md) lists the other languages.

```sh
pip install "fatoority==0.7.0"
```

0.7 removes every name that 0.6 kept with a `DeprecationWarning`, with no
compatibility aliases left behind. Each replacement below already exists in
0.6, so the simplest path is to run your tests on 0.6 with
`python -W error::DeprecationWarning` (or pytest's
`filterwarnings = ["error::DeprecationWarning"]`), replace every name the
warnings point at, and then upgrade. On 0.7 a removed name fails with
`ImportError`, `AttributeError` or `TypeError`.

## Removed names

### Module names

These are no longer importable from `fatoora`, `fatoora.invoice`,
`fatoora.sign` or `fatoora.validation`.

| Removed | Use instead |
| --- | --- |
| `Signer` | `InvoiceSigner` |
| `CsidCompliance` / `CsidProduction` | `ComplianceCsid` / `ProductionCsid` |
| `AllowanceCharge(charge, amount, category, rate, ...)` | `Adjustment(amount, charge=..., category=..., rate=..., ...)` |
| `DocumentReference(...)` | `OriginalInvoiceRef(...)`, with the same arguments |
| `VatId(value)`, `OtherId(value, scheme)` | `Identifier(value)`, `Identifier(value, scheme)` |
| `parse_finalized_invoice_xml(xml)` / `parse_finalized_invoice_xml_file(path)` | `FinalizedInvoice.from_xml(xml)` / `FinalizedInvoice.from_file(path)` |
| `parse_signed_invoice_xml(xml)` / `parse_signed_invoice_xml_file(path)` | `SignedInvoice.from_xml(xml)` / `SignedInvoice.from_file(path)` |
| `invoice_hash_base64_from_xml_str(xml)` | `hash_xml(xml)` |
| `validate_xml_invoice_from_str(config, xml)` | `validate_xml(xml)` |
| `validate_zatca_invoice_from_str(config, xml, ...)` | `validate_zatca(xml, ...)`, which returns a `ZatcaReport`; `report.to_dict()` gives the old dictionary |

### Factory classmethods

| Removed | Use instead |
| --- | --- |
| `InvoiceBuilder.new(type, subtype, ...)` | `InvoiceBuilder(type, subtype, ...)` |
| `InvoiceBuilder.new_with_reference(type, subtype, ref, reason)` | `InvoiceBuilder(type, subtype, original_invoice=ref, original_invoice_reason=reason)` |
| `LineItemBuilder.new()` | `LineItemBuilder()` |
| `CsrProperties.new(*, ...)` | `CsrProperties(*, ...)` |
| `ComplianceCsid.new(env, token, secret, request_id=None)` / `ProductionCsid.new(...)` | `ComplianceCsid(env, token=..., secret=..., request_id=None)` / `ProductionCsid(...)` |
| `Address.new(...)` | `Address(...)` |
| `PaymentMeans.new(code, instruction=None)` | `PaymentMeans(code)` or `PaymentMeans(code, (instruction,))` |
| `OriginalInvoiceRef.new(id, uuid, date, time, type_code)` | `OriginalInvoiceRef(id, uuid=..., issue_date=..., issue_time=..., document_type_code=...)` |
| `Adjustment.amount(value)` | `Adjustment(value)` |
| `Adjustment.percentage(base, percent)` | `Adjustment.percent_of(base, percent)` |

`amount` and `percentage` remain `Adjustment` fields; only the class-level
factories are gone.

### CSR properties

| Removed | Use instead |
| --- | --- |
| `CsrProperties.from_properties_str(text)` | `CsrProperties.from_str(text)` |
| `CsrProperties.parse_csr_config(text)` | `CsrProperties.from_str(text)` |
| `CsrProperties.parse_csr_config_file(path)` | `CsrProperties.from_file(path)` |

### Invoice builder

| Removed | Use instead |
| --- | --- |
| `builder.flags(value)` | `builder.set_flags(value)` |
| `set_note(language, text)` | `add_note(text, language=language)` |
| `set_note_without_language(text)` | `add_note(text)` |
| `set_payment_means_code(code)` | `add_payment_means(PaymentMeans(code))` |
| `set_tax_exemption(category, rate, exemption)` | `add_tax_exemption(category, rate, exemption)` |
| `add_document_adjustment(charge, value, category, rate, exemption)` | `add_charge(value, category, rate, exemption)` when `charge` is true, otherwise `add_allowance(...)` |
| `invoice_level_discount(amount)` | `add_allowance(Adjustment(amount), category, rate)` |
| `invoice_level_charge(amount)` | `add_charge(Adjustment(amount), category, rate)` |
| `set_allowance(reason, amount)` | `add_allowance(Adjustment(amount, reason=reason), category, rate)` |
| `allowance_reason(reason)` | `Adjustment(amount, reason=reason)` on each entry |

The aggregate setters (`set_allowance`, `invoice_level_discount`,
`invoice_level_charge`, `allowance_reason`) added one allowance and one charge
in the document VAT category, at the single VAT rate of that category's lines,
when `build()` ran. `add_allowance` and `add_charge` append one entry per call
in the category and at the rate you pass, so pass the document category and the
line rate to keep the same totals.

The invoice properties `invoice_level_discount`, `invoice_level_charge` and
`allowance_reason` stay: they report the totals and shared reason of the
document-level entries.

### Invoice values

| Removed | Use instead |
| --- | --- |
| `adjustment.value` | `adjustment.amount` |
| `adjustment.percent` | `adjustment.percentage` |
| `adjustment.with_supplied_amount(value)` | `Adjustment.percent_of(base, percent, amount=value)` |
| `value, scheme = identifier`, `identifier[0]` | `identifier.value`, `identifier.scheme` |
| A `(value, scheme)` tuple for `Party.vat_id`, `Party.identifiers` or an item identifier | `Identifier(value, scheme)` (a plain `str` value still works) |
| `close()` or `with` on `Address`, `Party`, `Identifier`, `InvoiceNote`, `OriginalInvoiceRef`, `PaymentMeans` or `Adjustment` | Nothing: they are plain frozen dataclasses with no cleanup |

Passing a tuple where an identifier is expected now raises `TypeError`.
Native owners such as `Config`, `ZatcaClient`, `InvoiceBuilder` and the invoice
classes still support `with` and `close()`.

### Invoices

| Removed | Use instead |
| --- | --- |
| `invoice.to_xml()` | `invoice.xml` |
| `signed.into_xml()` | `signed.xml`, then `signed.close()` when done |
| `invoice.hash_base64()` | `invoice.hash()` |

### ZATCA API client

| Removed | Use instead |
| --- | --- |
| `client.post_csr_for_ccsid(csr, otp)` | `client.request_compliance_csid(csr, otp)` |
| `client.post_ccsid_for_pcsid(ccsid)` | `client.request_production_csid(ccsid)` |
| `clearance_status=False` / `True` on `report_simplified_invoice` and `clear_standard_invoice` | `clearance=ClearanceMode.DISABLED` / `ClearanceMode.ENABLED` |

`clearance` is now a required keyword-only argument of both methods.

## Error classes

0.7 gives every language the same error hierarchy: a base `FatooraError`, one
`ErrorKind` enum, and one subclass per kind named `<Kind>Error`. See
[errors and diagnostics](../../reference/errors.md). These names change in 0.7
with no deprecated alias in 0.6, so update them while upgrading.

| 0.6 | 0.7 |
| --- | --- |
| `FfiError` | `FatooraError`; every library failure is a subclass of it |
| `FfiErrorKind` | `ErrorKind` (same members and values) |
| `fatoora.errors.error_class_for_code(code)(message, code, details)` | `FatooraError.from_code(code, message, details)` |
| `InvalidInputError(message, code, details)` (any subclass) | `InvalidInputError(message, details=details)`; the code comes from the class, and `code` and `details` are keyword-only |

The subclasses (`InvalidInputError`, `ValidationError`, ..., `ApiError`) keep
their names and their `ValueError`/`OSError` bases. A code the package does not
recognize now raises plain `FatooraError` instead of `FfiError`; its `.kind` is
still `None`. Errors also gain `.message`.

```python
# 0.6
from fatoora.errors import FfiError, FfiErrorKind
except FfiError as error:
    if error.kind == FfiErrorKind.NETWORK: ...

# 0.7
from fatoora import ErrorKind, FatooraError
except FatooraError as error:
    if error.kind == ErrorKind.NETWORK: ...
```

## New in 0.7

`AsyncZatcaClient` is an `asyncio` client with the methods of `ZatcaClient`,
as coroutines. It needs `httpx`, from the optional `async` extra
(`pip install "fatoority[async]"`); the base package still has no HTTP
dependency. See [asynchronous client](../../reference/bindings/python.md#asynchronous-client).

```python
async with AsyncZatcaClient(config, timeout=30) as client:
    response = await client.report_simplified_invoice(signed, pcsid, clearance=ClearanceMode.DISABLED)
```

## Example

```python
# 0.6, with DeprecationWarnings
from fatoora import (InvoiceBuilder, InvoiceSubType, InvoiceTypeKind, VatCategory,
                     parse_signed_invoice_xml)

builder = InvoiceBuilder.new(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
builder.set_vat_category(VatCategory.STANDARD)
builder.invoice_level_discount("10")
builder.allowance_reason("Loyalty discount")
builder.set_payment_means_code("10")

signed = parse_signed_invoice_xml(xml)
response = client.report_simplified_invoice(signed, pcsid, clearance_status=False)
print(signed.hash_base64(), signed.to_xml())
```

```python
# 0.7
from fatoora import (Adjustment, ClearanceMode, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                     PaymentMeans, SignedInvoice, VatCategory)

builder = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
builder.set_vat_category(VatCategory.STANDARD)
builder.add_allowance(Adjustment("10", reason="Loyalty discount"), VatCategory.STANDARD, "15")
builder.add_payment_means(PaymentMeans("10"))

signed = SignedInvoice.from_xml(xml)
response = client.report_simplified_invoice(signed, pcsid, clearance=ClearanceMode.DISABLED)
print(signed.hash(), signed.xml)
```
