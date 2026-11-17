---
title: C# and .NET ZATCA E-Invoicing Library
description: C# package reference for Fatoority on .NET 8. Learn supported platforms, input values, returned objects, disposal, errors, validation, and ZATCA API requests.
---

# C# package reference

Use the `Fatoority` NuGet package to build, sign, and validate invoices and send
ZATCA API requests from .NET 8 or later. Public classes use the `Fatoora` namespace.
The package includes the native library for its supported platforms.

## Install from NuGet

In your application project, install the published package:

```sh
dotnet add package Fatoority
```

Use the `Fatoora` namespace. Check the supported native platforms below before
deploying your application.

## Supported platforms

| Runtime identifier | Platform |
| --- | --- |
| `linux-x64` | Linux x86-64, glibc 2.28 or later |
| `win-x64` | Windows x86-64 |
| `osx-arm64` | macOS Apple Silicon |

Match your application's OS and architecture to a packaged runtime identifier.
NuGet supplies the native asset at runtime; no library search-path override is
needed. Linux musl/Alpine is not included. A package built locally inherits its
build host's libc requirements.

## Example

Import `Fatoora` and dispose native objects when finished:

```csharp
using Fatoora;

using var config = Config.New(ZatcaEnvironment.NonProduction);
using var key = SigningKey.Generate();
using var bytes = key.ToDer();
using var view = bytes.AsSlice();
byte[] privateKey = view.Clone();
```

`privateKey` is an independent managed copy. Keep it secure; disposing the native
objects does not erase that array. Follow the [CSR guide](../../guides/csr.md)
for a complete credential-generation workflow.

Use `ZatcaEnvironment.NonProduction`, `ZatcaEnvironment.Simulation`, or
`ZatcaEnvironment.Production`. The enum is named `ZatcaEnvironment` so it does not
clash with `System.Environment` under implicit usings. Configuration, CSR generation, and credentials must agree on the environment.
See the [CSR](../../guides/csr.md), [signing](../../guides/invoice-signing.md),
[validation](../../guides/validation.md), and [QR](../../guides/qr.md) guides for
complete C# examples.

## Values and ownership

| Value | C# representation |
| --- | --- |
| Text input | `string`; invalid UTF-16 and embedded NUL are rejected |
| Optional text input | `string?`; `null` and `""` remain distinct |
| Optional text output | `Text?`; read `Value()` and dispose the owner when present |
| Decimal amount | Invariant decimal string; use `CultureInfo.InvariantCulture` for conversion |
| Invoice counter | `ulong` |
| Owned native object | An `IDisposable` class; use `using` or call `Dispose()` |
| Owned binary output | `Bytes`; call `AsSlice()` and copy with `Clone()` when needed |
| Invoice acceptance | `InvoiceOutcome.Accepted`, `Rejected`, or `Unknown` |
| Enumerated code | C# enum: `ZatcaEnvironment`, `InvoiceTypeKind`, `InvoiceSubType`, `VatCategory`, `AdjustmentKind`, `ClearanceMode` |
| Response status | `ReportingStatus?`, `ClearanceStatus?`, `ValidationStatus?`, or `MessageType?`; `null` when absent, `Unknown` when present but not recognized by this version. The `*Raw()` getter returns the exact text as `Text?` |
| Invoice flags | `byte` bitmask taken by `InvoiceBuilder.Flags` and returned by `Flags()`; cast `InvoiceFlag` members, such as `(byte)InvoiceFlag.Export`, and combine bits with bitwise OR |

Native object classes implement `IDisposable`, including returned snapshots and
`BindingError`. Disposal is idempotent; later calls on that wrapper throw
`ObjectDisposedException`. Finalization provides cleanup if disposal is omitted,
but its timing is not deterministic. Releasing an owner does not guarantee
zeroization of secrets or copies exported to managed strings and arrays.

`InvoiceBuilder.Build()`, `InvoiceSigner.Sign()`, and `SignedInvoice.IntoXml()` consume
inner state. The outer wrapper remains disposable, but reusing consumed state
raises `BindingErrorException`. Build and signing failures can also consume
state. Returned invoice data, totals, addresses, and response children are owned
snapshots and remain usable after their parents are disposed.

For binary output, dispose both the owner and its borrowed view:

```csharp
using Fatoora;

using var key = SigningKey.Generate();
using var bytes = key.ToDer();
using var view = bytes.AsSlice();
byte[] der = view.Clone(); // Independent managed copy.
```

`WithSpan()` provides a scoped, read-only view without copying. An active read
retains native memory through concurrent source disposal. Later reads fail after
the source is disposed. Disposing the view while a read is active throws
`InvalidOperationException`; retry once the read has finished. Conflicting calls
that require mutable access also throw `InvalidOperationException`.

## Errors

```csharp
--8<-- "bindings/dotnet/examples/SdkExamples.cs:errors"
```

`BindingErrorException.Message` contains the native message. `Inner.Code()` uses
the shared [error codes](../errors.md), and `Inner.DetailsJson()` preserves
structured details such as HTTP status and response body. Dispose `Inner` after
reading it. Invalid UTF-16 fails before the native call with
`EncoderFallbackException`; invalid native inputs, including embedded NUL, use
`BindingErrorException`.

## API requests

Create a client with `ZatcaClient.Create(config)`. Dispose the client, credentials,
and responses with `using`. The following methods return owned objects:

| Method | Inputs | Returns |
| --- | --- | --- |
| `RequestComplianceCsid(csr, otp)` | `Csr`, OTP string | `ComplianceCsid` |
| `RequestProductionCsid(credentials)` | `ComplianceCsid` | `ProductionCsid` |
| `RenewCsid(credentials, csr, otp, acceptLanguage)` | `ProductionCsid`, `Csr`, OTP, nullable language string | `ProductionCsid` |
| `CheckInvoiceCompliance(invoice, credentials)` | `SignedInvoice`, `ComplianceCsid` | `ValidationResponse` |
| `ReportSimplifiedInvoice(invoice, credentials, clearanceMode, acceptLanguage)` | `SignedInvoice`, `ProductionCsid`, `ClearanceMode`, nullable language string | `ValidationResponse` |
| `ClearStandardInvoice(invoice, credentials, clearanceMode, acceptLanguage)` | `SignedInvoice`, `ProductionCsid`, `ClearanceMode`, nullable language string | `ValidationResponse` |

All listed arguments are required, including nullable arguments: pass `null` when
no language override is needed. `clearanceMode` sets the clearance-status
request header; use `ClearanceMode.Disabled` for ordinary reporting and
`ClearanceMode.Enabled` for clearance.
It does not describe the result of the operation.

The native client methods block the calling thread. HTTP success alone does not
establish invoice acceptance: inspect `Outcome()` or call `EnsureAccepted()`.
For clearance, `ClearedInvoiceXml()` returns `Text?`: when present, read `Value()`,
save the text, and dispose the returned owner. See the
[API guide](../../guides/api.md#response-receipt-and-acceptance) for rejection,
unknown outcomes, and cleared-document semantics.

### Prepared requests

| Blocking method | Prepared equivalent | Credential scope / result |
| --- | --- | --- |
| `RequestComplianceCsid` | `PrepareRequestComplianceCsid` | CSR and OTP to compliance credentials |
| `RequestProductionCsid` | `PrepareRequestProductionCsid` | Compliance to production credentials |
| `RenewCsid` | `PrepareRenewCsid` | Production credentials, CSR and OTP to renewed credentials |
| `CheckInvoiceCompliance` | `PrepareCheckInvoiceCompliance` | Compliance credentials; validation response |
| `ReportSimplifiedInvoice` | `PrepareReportSimplifiedInvoice` | Production credentials; reporting response |
| `ClearStandardInvoice` | `PrepareClearStandardInvoice` | Production credentials; clearance response |

`PrepareRequestComplianceCsid` returns `ComplianceRequest`; credential exchange and
renewal return `ProductionRequest`; invoice operations return `InvoiceRequest`.
Their arguments match the corresponding blocking method.

For asynchronous HTTP or cancellation, use a `Prepare*` method and your own
`HttpClient`. The request exposes `Method()`, `Url()`, `HeadersJson()` (a JSON
array of header pairs), and an owned `Body()`. Copy its bytes before awaiting the
transport. Pass the HTTP status and response bytes to `Decode(ushort, byte[])`,
including non-2xx responses, so the library can classify gateway errors.
`Decode` returns an owned `ComplianceCsid`, `ProductionCsid`, or
`ValidationResponse` matching the request type; dispose it after use. Configure the
transport to refuse redirects, and pass your cancellation token to send/read
operations. The generated blocking methods do not accept a cancellation token.

Prepared requests own their wire data, credentials, and decoding context; keep
the request alive until decoding completes. Headers contain credentials and
should not be logged.

## Local validation

| Method | Inputs | Returns |
| --- | --- | --- |
| `Xml.Validate(xml)` | Invoice XML string | `true` on schema success; throws on malformed XML or schema rejection |
| `Xml.ValidateZatca(xml, optionsJson)` | Invoice XML string, nullable options JSON string | Owned `ZatcaValidationReport`, including rejection or incomplete coverage |
| `Xml.Hash(xml)` | Invoice XML string | Base64 invoice hash; throws when XML cannot be processed |

Pass `null` explicitly for default validation options. Otherwise, serialize an
object containing `previous_invoice_hash` and/or `evaluated_at`; unknown option
fields are rejected. The evaluation timestamp uses RFC3339 format.

```csharp
using Fatoora;
using System.IO;

string invoiceXml = File.ReadAllText("invoice.xml");
using var report = Xml.ValidateZatca(invoiceXml, null);
bool valid = report.IsValid();
string json = report.ToJson(); // the same report as JSON
```

`Xml.ValidateZatca(xml, optionsJson)` returns an owned `ZatcaValidationReport`. Inspect
`IsValid()`, `IsComplete()` and `Stage(i)`; a rejected document returns a report, while an execution
failure throws an exception that may carry a partial report in its details.
Provide the predecessor hash from invoice history and set `evaluated_at` for
repeatable checks. Omitting the timestamp uses the native clock.

All bundled UBL schemas are embedded. Installed packages do not need the source
checkout, Java, or an external SDK installation. Local validation does not
establish gateway acceptance or certificate issuer trust. See the
[validation reference](../invoice-validation.md).

## Licensing

The native library includes SDK-derived rules and messages subject to the
upstream terms in the bundled
[third-party notices](https://github.com/mqqz/fatoority/blob/main/THIRD_PARTY_NOTICES.md).
Those terms apply to the native component distributed with the C# package, as
they do to other SDK distributions.

For source-build instructions, see the [binding development documentation](../../development/diplomat.md).
