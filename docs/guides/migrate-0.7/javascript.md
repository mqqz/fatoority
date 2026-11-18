---
title: Migrate a JavaScript or TypeScript Project from fatoority 0.6 to 0.7
description: Every name the fatoority npm package removed in 0.7, each with the 0.6 replacement to use instead.
---

# Migrate from 0.6 to 0.7: JavaScript and TypeScript

This page covers the `fatoority` npm package and its `fatoority/raw` entry
point. The [overview](index.md) lists the other languages.

```sh
npm install fatoority@0.7.0
```

0.7 removes every name marked `@deprecated` in the 0.6 TypeScript
declarations. Each replacement below already exists in 0.6, so you can switch
to it first and then upgrade. TypeScript reports each removed name as a
missing property; in JavaScript a call throws
`TypeError: ... is not a function`.

## Removed names

### `new_` constructors

The generated `new_` name of every Rust `new` constructor is removed from the
package root and from `fatoority/raw`. Call `create` with the same arguments.

| Removed | Use instead |
| --- | --- |
| `Address.new_(...)` | `Address.create(...)` |
| `AllowanceCharge.new_(...)` | `AllowanceCharge.create(...)` |
| `ComplianceCsid.new_(...)` | `ComplianceCsid.create(...)` |
| `Config.new_(env)` | `Config.create(env)` |
| `CsrProperties.new_(...)` | `CsrProperties.create(...)` |
| `InvoiceBuilder.new_(...)` | `InvoiceBuilder.create(...)` |
| `InvoiceLineItem.new_(...)` | `InvoiceLineItem.create(...)` |
| `LineItemBuilder.new_()` | `LineItemBuilder.create()` |
| `OriginalInvoiceRef.new_(...)` | `OriginalInvoiceRef.create(...)` |
| `PaymentMeans.new_(code, instruction)` | `PaymentMeans.create(code, instruction)` |
| `ProductionCsid.new_(...)` | `ProductionCsid.create(...)` |
| `TaxExemption.new_(code, reason)` | `TaxExemption.create(code, reason)` |
| `VatBreakdown.new_(...)` | `VatBreakdown.create(...)` |
| `ZatcaClient.new_(config)` (`fatoority/raw`) | `ZatcaClient.create(config)` (`fatoority/raw`), or `new ZatcaClient(config)` from the package root |

### Adjustments and VAT breakdowns

| Removed | Use instead |
| --- | --- |
| `Adjustment.percentage(base, percent)` | `Adjustment.percentOf(base, percent)` |
| `adjustment.withSuppliedAmount(value)` | `adjustment.suppliedAmount(value)` |
| `adjustment.percent()` (returned `Text \| null`) | `adjustment.percentage` property (`string \| null`) |
| `allowanceCharge.withoutVatRate()` | `allowanceCharge.omitVatRate()` |
| `vatBreakdown.withoutVatRate()` | `vatBreakdown.omitVatRate()` |

`adjustment.percentage` is a read-only property of an instance, while
`Adjustment.percentage(...)` was a static factory, so code that reads the
property needs no change.

## Clearance mode is required and must be a `ClearanceMode`

`ZatcaClient.reportSimplifiedInvoice` and `ZatcaClient.clearStandardInvoice`
no longer accept a boolean or the variant name (`'Enabled'`/`'Disabled'`) for
the `Clearance-Status` header, and the argument no longer has a default
(`false` for reporting, `true` for clearance). Pass `ClearanceMode.Disabled` or
`ClearanceMode.Enabled`, as every other binding does. Any other value, including
an omitted argument, rejects with a `TypeError`. The parameter is now named
`clearanceMode`, like the `fatoority/raw` `prepare*` methods, so it is not
confused with `response.clearanceStatus`.

| 0.6 | 0.7 |
| --- | --- |
| `client.reportSimplifiedInvoice(invoice, credentials)` | `client.reportSimplifiedInvoice(invoice, credentials, ClearanceMode.Disabled)` |
| `client.reportSimplifiedInvoice(invoice, credentials, false, ...)` | `client.reportSimplifiedInvoice(invoice, credentials, ClearanceMode.Disabled, ...)` |
| `client.clearStandardInvoice(invoice, credentials)` | `client.clearStandardInvoice(invoice, credentials, ClearanceMode.Enabled)` |
| `client.clearStandardInvoice(invoice, credentials, true, ...)` | `client.clearStandardInvoice(invoice, credentials, ClearanceMode.Enabled, ...)` |
| `'Enabled'` / `'Disabled'` | `ClearanceMode.Enabled` / `ClearanceMode.Disabled` |
| `ClearanceModeInput` type | `ClearanceMode` |
## Error classes

0.7 gives every language the same error hierarchy: a base `FatooraError`, one
`ErrorKind` enum, and one subclass per kind named `<Kind>Error`. See
[errors and diagnostics](../../reference/errors.md). These names change in 0.7
with no deprecated alias in 0.6, so update them while upgrading.

| 0.6 | 0.7 |
| --- | --- |
| `ErrorCode` (names to numbers) | `ErrorKind` (names to names: `ErrorKind.Network === 'Network'`); compare `error.kind` with it. `error.code` is still the number |
| `errorClassForCode(code)` | `FatooraError.fromCode(code, message, { details })` creates the error |
| `TransportError` | `NetworkError`, with the host error in `cause` |
| `transportError.httpStatus` | `error.details.http_status`; `details.type` is `'api_response_read'` when a response arrived and `'network'` when `fetch` failed |

`FatooraError`, `kind` and the subclasses (`InvalidInputError`, ...,
`ApiError`) are unchanged, so `instanceof NetworkError` checks keep working.
The TypeScript type `ErrorKind` is unchanged.

```javascript
// 0.6
if (error instanceof TransportError && error.httpStatus) retryLater();
if (error.code === ErrorCode.Unauthorized) renewCredentials();

// 0.7
if (error instanceof NetworkError && error.details.http_status) retryLater();
if (error.kind === ErrorKind.Unauthorized) renewCredentials();
```

## Example

```ts
// 0.6, with @deprecated warnings
import { Adjustment, Config, Environment, LineItemBuilder } from 'fatoority';

const config = Config.new_(Environment.NonProduction);
const line = LineItemBuilder.new_();
const rebate = Adjustment.percentage('250', '10').withSuppliedAmount('25');
const percent: string | undefined = rebate.percent()?.value();
```

```ts
// 0.7
import { Adjustment, Config, Environment, LineItemBuilder } from 'fatoority';

const config = Config.create(Environment.NonProduction);
const line = LineItemBuilder.create();
const rebate = Adjustment.percentOf('250', '10').suppliedAmount('25');
const percent: string | null = rebate.percentage;
```
