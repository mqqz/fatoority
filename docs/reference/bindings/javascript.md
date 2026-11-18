---
title: JavaScript and TypeScript ZATCA E-Invoicing Library
description: JavaScript package reference for fatoority. Learn supported runtimes, exact decimal values, validation results, errors, and asynchronous ZATCA API requests.
---

# JavaScript package reference

Use `fatoority` in Node.js or a browser to build, sign, and validate invoices and
send ZATCA API requests. The package includes WebAssembly and TypeScript declarations.
For a complete task walkthrough, start with the [guides](../../guides/validation.md).

## Install

```sh
npm install fatoority
```

Import from the package root for asynchronous API requests and validation with an
automatically supplied clock:

```js
import { Config, Environment, SigningKey, Xml, ZatcaClient } from 'fatoority';

const config = Config.create(Environment.NonProduction);
const key = SigningKey.generate();
const privateKeyPem = key.toPem();
```

The private key is secret material; store it securely. Environment values are
`Environment.NonProduction`, `Environment.Simulation`, and `Environment.Production`.
Configuration, CSR generation, and credentials must agree.

Node.js 22+ and modern browsers are supported. Browser execution requires
WebAssembly, Web Crypto, `FinalizationRegistry`, and top-level await. TypeScript
users can use `NodeNext` or `bundler` module resolution; no separate `@types`
package is needed.

## Values, ownership and errors

| Value | JavaScript representation and behavior |
| --- | --- |
| Text | `string`; embedded NUL is rejected |
| Optional text input | `string` or `null`; `""` is a present empty value |
| Optional text output | `string` or `null` |
| Decimal amount or quantity | Exact decimal `string`, such as `"15.25"`, in arguments and results; do not pass a floating-point number. Use a decimal library such as `decimal.js` for arithmetic |
| Invoice counter | `bigint`, within the unsigned 64-bit range |
| Binary output | `Bytes.asSlice()` returns a copied `number[]`; use `Uint8Array.from(...)` for a typed byte array |
| Native object | Object managed by `FinalizationRegistry` |
| Invoice outcome | `InvoiceOutcome.Accepted`, `Rejected`, or `Unknown` |
| Enumerated code | Enum class static, such as `Environment.NonProduction`, `InvoiceTypeKind.Tax`, `InvoiceSubType.Simplified`, `VatCategory.Standard`, `AdjustmentKind.Charge`, or `ClearanceMode.Disabled`; `.value` is the variant name string, such as `'Standard'`. Each enum has a matching string-literal union type named with a `Value` suffix, such as `VatCategoryValue` |
| Response status | `ReportingStatus`, `ClearanceStatus`, `ValidationStatus`, or `MessageType` enum, or `null` when absent; `Unknown` means present but not recognized by this version, and the matching `*Raw` property returns the exact text |
| Invoice flags | `number` bitmask; combine the `bit` property of `InvoiceFlag.ThirdParty`, `Nominal`, `Export`, `Summary`, and `SelfBilled` bits with bitwise OR |

Value getters are read-only properties: `address.city`, `invoice.xml`,
`totals.taxAmount`, `response.reportingStatus`. A getter is a property when it
takes no arguments and returns a string, number, `bigint`, boolean, enum or
optional text. Members that take arguments, return another native object
(`invoice.data()`, `invoice.totals()`, `response.validationResults()`) or
convert or consume (`toPem()`, `intoXml()`, `withReason(...)`, `build()`) stay
methods, as do `len()` and `isEmpty()`. Assigning to a property throws a
`TypeError`.

Method names use camelCase. Constructors named `new` in the underlying API are
`create`, such as `Config.create(Environment.NonProduction)`; the generated name
`new_` was removed in 0.7 (see [migrating to 0.7](../../guides/migrate-0.7/javascript.md)). Generated
constructors (`new Config()`) are internal. Read the [number reference](../numbers.md) for
accepted decimal syntax and rounding rules.

`InvoiceBuilder.build()`, `InvoiceSigner.sign()`, and `SignedInvoice.intoXml()` consume
the object's inner state. Build and signing failures can also consume state;
create a new input object before retrying. Subsequent use of consumed state
throws an error. Returned invoice data, addresses, totals, and response children
are independent snapshots.

Objects belong to the WebAssembly instance that created them. Each worker needs
its own instance and objects; do not transfer internal pointers between workers.
Garbage collection does not guarantee immediate cleanup or erasure of secrets.

A library failure throws one `FatooraError` subclass per error code:

| Code | Class | Meaning |
| --- | --- | --- |
| 1 | `InvalidInputError` | A caller-supplied value is malformed or out of range |
| 2 | `ValidationError` | Input fails invoice, CSR, or schema rules |
| 3 | `ParseError` | Structured input, such as an API response body, could not be parsed |
| 4 | `XmlError` | XML could not be serialized, parsed, or processed |
| 5 | `CryptoError` | Key, signing, hashing, or certificate processing failed |
| 6 | `IoError` | An I/O operation failed |
| 7 | `NetworkError` | The HTTP transport failed |
| 8 | `UnauthorizedError` | ZATCA rejected the credentials |
| 9 | `InternalError` | Internal invariant or consumed-state error |
| 10 | `ApiError` | ZATCA returned an error or non-accepted response |

Each error has `code`, `kind` (an `ErrorKind` value such as
`ErrorKind.InvalidInput`, which is the string `'InvalidInput'`, or `null` for a
code this version does not know), `message`, and `details`, the parsed JSON
object with structured information such as validation findings or an HTTP
response body. `cause` is the underlying `BindingError`.
`FatooraError.fromCode(code, message, { details })` creates the class for a
code. Typed errors apply to
`fatoority/raw` too, because it shares the generated classes.
See [error codes and details](../errors.md).

```js
import { Address, FatooraError, InvalidInputError } from 'fatoority';

try {
  Address.create('XX', 'Riyadh', 'King Fahd Road', '1234', '12345', null, null, null);
} catch (error) {
  if (error instanceof InvalidInputError) console.error('Fix the address:', error.message);
  else if (error instanceof FatooraError) console.error(error.code, error.kind, error.details);
  else throw error;
}
```

Network, cancellation, and response-body read failures throw `NetworkError`
with the original host error in `cause`. Its `details` follow the library
schema: `{ type: 'network', diagnostics }` when `fetch` fails, or
`{ type: 'api_response_read', http_status, message }` when a response arrived
but its body could not be read. HTTP rejection decoded by the library uses the class for its code instead,
such as `UnauthorizedError` or `ApiError`.

## Local validation

| Method | Inputs | Returns |
| --- | --- | --- |
| `Xml.validate(xml)` | Invoice XML string | `true` on schema success; throws on invalid XML or schema rejection |
| `Xml.validateZatca(xml, options?)` | Invoice XML; optional `{ previousInvoiceHash, evaluatedAt }`, options JSON string, or `null` | `ZatcaValidationReport` plain object, including rejection or incomplete coverage |
| `Xml.hash(xml)` | Invoice XML string | Base64 invoice hash; throws when XML cannot be processed |

For Node.js, save your invoice as `invoice.xml` and read it as UTF-8:

```js
import { readFile } from 'node:fs/promises';
import { Xml } from 'fatoority';

const invoiceXml = await readFile('invoice.xml', 'utf8');
const report = Xml.validateZatca(invoiceXml);
console.log(report.isValid, report.isComplete);
for (const stage of report.stages) {
  for (const finding of stage.findings) {
    console.log(stage.stage, finding.severity, finding.code, finding.message, finding.location);
  }
}
```

Pass `{ previousInvoiceHash }` to supply invoice-history context.
`evaluatedAt` is an RFC3339 timestamp or a `Date`; the package root supplies the
current time once if omitted. Set it explicitly for repeatable checks. A JSON
string with `previous_invoice_hash` and `evaluated_at` is also accepted. The
`fatoority/raw` validation API requires an explicit timestamp.

The report is plain data, copied out of WebAssembly before `validateZatca`
returns: it can be serialized with `JSON.stringify`, posted to a worker, and
kept without holding native memory. Its TypeScript interfaces are
`ZatcaValidationReport`, `ZatcaStageReport`, `ZatcaFinding`,
`ZatcaFindingLocation`, and `ZatcaRuleSource`.

| Field | Type |
| --- | --- |
| `schemaVersion`, `profile`, `evaluatedAt` | `number`, `string`, RFC3339 `string` |
| `isComplete`, `hasErrors`, `isValid` | `boolean` |
| `stages[]` | `{ stage, status, provenance, evaluatedAssertions, findings }`; `stage` is a `ZatcaStageValue` such as `'Ksa'`, `status` a `ZatcaStageStatusValue` such as `'Completed'`, `provenance` is `{ stylesheet, sha256 }` or `null` |
| `stages[].findings[]` | `{ code, severity, layer, message, assertionSite, location }`; `severity` is `'Warning'` or `'Error'`, `layer` a `ValidationLayerValue` such as `'BusinessRules'` |
| `location` | `null`, `{ kind: 'Field', path }` or `{ kind: 'XPath', path }`, `{ kind: 'Xml', line, column }`, or `{ kind: 'Unknown' }` |

The snake_case JSON form documented in the [validation reference](../invoice-validation.md)
comes from the native report's `toJson()` in `fatoority/raw`.

A returned report does not imply validity: inspect `isValid`. Missing predecessor
context produces incomplete coverage. Execution failures throw and may include a
partial report in `error.details.report`. See the [validation reference](../invoice-validation.md)
for fields, stage statuses, and limits. Local validation does not establish
remote acceptance or certificate issuer trust. Read files using Node.js or browser
APIs and pass their contents; filesystem helpers are not available in WebAssembly.

## API requests

Create `new ZatcaClient(config)` to use the global `fetch`, or
`new ZatcaClient(config, { fetch: customFetch })` to provide a compatible transport.
All methods below return promises. Optional `options` accepts `{ signal }`, where
`signal` is an `AbortSignal`.

| Method | Inputs | Promise result |
| --- | --- | --- |
| `requestComplianceCsid(csr, otp, options?)` | `Csr`, OTP string | `ComplianceCsid` |
| `requestProductionCsid(credentials, options?)` | `ComplianceCsid` | `ProductionCsid` |
| `renewCsid(credentials, csr, otp, acceptLanguage?, options?)` | `ProductionCsid`, `Csr`, OTP, optional language string | `ProductionCsid` |
| `checkInvoiceCompliance(invoice, credentials, options?)` | `SignedInvoice`, `ComplianceCsid` | `ValidationResponse` |
| `reportSimplifiedInvoice(invoice, credentials, clearanceMode, acceptLanguage?, options?)` | `SignedInvoice`, `ProductionCsid`, `ClearanceMode`; language defaults to `null` | `ValidationResponse` |
| `clearStandardInvoice(invoice, credentials, clearanceMode, acceptLanguage?, options?)` | `SignedInvoice`, `ProductionCsid`, `ClearanceMode`; language defaults to `null` | `ValidationResponse` |

`clearanceMode` sets the `Clearance-Status` request header; it is not the
result of the operation. It is required and must be `ClearanceMode.Enabled` or
`ClearanceMode.Disabled`, as in every other binding and in the prepared-request
methods on `fatoority/raw`; any other value, including a boolean or the variant
name, rejects with a `TypeError`. `acceptLanguage` controls the language request header.
Pass optional positional arguments explicitly when supplying a later argument.

HTTP success alone does not establish invoice acceptance. Inspect `outcome` or
call `ensureAccepted()` on a `ValidationResponse`. For clearance,
`clearedInvoiceXml` is the cleared document as a string, or `null`; save it
when present. See [API requests and responses](../api-client.md) for
credential requirements and response fields, and the [API guide](../../guides/api.md)
for a complete workflow.

### Prepared requests

For your own HTTP integration, import `ZatcaClient` from `fatoority/raw`, construct
it with `ZatcaClient.create(config)`, and call its `prepare*` methods. These return
`ComplianceRequest`, `ProductionRequest`, or `InvoiceRequest` objects.

| Request member | Value |
| --- | --- |
| `method` | HTTP method string |
| `url` | Complete request URL string |
| `headersJson` | JSON array of header-name/value pairs |
| `body()` | `Bytes`; use `asSlice()` for a copied request body |
| `decode(status, bytes)` | Credentials or `ValidationResponse`, depending on request type; may throw |

Prepared requests retain their data and decoding context independently of the
original invoice and credentials. Copy the outgoing body before awaiting HTTP.
Pass every HTTP status and its response bytes to `decode`, including non-2xx
responses. Disable redirects. Headers contain credentials and should not be logged.

Alternatively, `sendPrepared(request, { fetch, signal })` from `fatoority` performs
these steps and returns a promise for the decoded result. It copies the body
before awaiting, refuses redirects, and forwards HTTP error responses to decoding.

## Browser deployment

The loader resolves `fatoora_ffi.wasm` relative to its package modules. Deploy the
modules and WASM together, or configure your bundler to copy WASM to that URL.
Serve the application over HTTP(S), with WebAssembly execution and Web Crypto
available.

Direct ZATCA requests depend on gateway CORS. Use a server-side transport when
needed and decide where credentials and private keys should reside. Local
validation runs without Java, external SDK files, or network access.

## Licensing and source builds

The package contains ZATCA-derived rules and messages with additional terms in
[third-party notices](https://github.com/mqqz/fatoority/blob/main/THIRD_PARTY_NOTICES.md).
TypeScript declarations ship with the package. For source-build instructions,
see the [binding development documentation](../../development/diplomat.md).
