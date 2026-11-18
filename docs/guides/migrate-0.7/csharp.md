---
title: Migrate a C# Project from fatoority 0.6 to 0.7
description: Upgrade a C# project from the Fatoority 0.6 NuGet package to 0.7, covering removed members, TimeSpan timeouts, Bytes.ToArray and the renamed FatooraException.
---

# Migrate from 0.6 to 0.7: C# and .NET

This page covers the `Fatoority` NuGet package. The [overview](index.md) lists
the other languages.

```sh
dotnet add package Fatoority --version 0.7.0
```

0.7 removes every member that 0.6 marked `[Obsolete]`. Each replacement below
already exists in 0.6, so the simplest path is to turn obsolete warnings into
errors while still on 0.6, fix them, and then upgrade:

```xml
<PropertyGroup>
  <WarningsAsErrors>$(WarningsAsErrors);CS0618</WarningsAsErrors>
</PropertyGroup>
```

On 0.7 the same calls fail with `CS0117` (no static member `New`) or `CS1061`
(no instance member).

## Removed members

### `New` factory methods

The static `New` factory on each class below is removed. Call `Create` with the
same arguments.

| Removed | Use instead |
| --- | --- |
| `Address.New(...)` | `Address.Create(...)` |
| `AllowanceCharge.New(...)` | `AllowanceCharge.Create(...)` |
| `Config.New(env)` | `Config.Create(env)` |
| `CsrProperties.New(...)` | `CsrProperties.Create(...)` |
| `InvoiceBuilder.New(...)` | `InvoiceBuilder.Create(...)` |
| `InvoiceLineItem.New(...)` | `InvoiceLineItem.Create(...)` |
| `LineItemBuilder.New()` | `LineItemBuilder.Create()` |
| `OriginalInvoiceRef.New(...)` | `OriginalInvoiceRef.Create(...)` |
| `PaymentMeans.New(code, instruction)` | `PaymentMeans.Create(code, instruction)` |
| `TaxExemption.New(code, reason)` | `TaxExemption.Create(code, reason)` |
| `VatBreakdown.New(...)` | `VatBreakdown.Create(...)` |

`ZatcaClient`, `ComplianceCsid` and `ProductionCsid` already used `Create` in
0.5 and are unchanged.

### Adjustments and VAT rates

| Removed | Use instead |
| --- | --- |
| `adjustment.WithSuppliedAmount(amount)` | `adjustment.SuppliedAmount(amount)` |
| `adjustment.Percent` (property) | `adjustment.Percentage` (property) |
| `allowanceCharge.WithoutVatRate()` | `allowanceCharge.OmitVatRate()` |
| `vatBreakdown.WithoutVatRate()` | `vatBreakdown.OmitVatRate()` |

## Changed signatures

These changes have no 0.6 equivalent to migrate to first; update the call when
you upgrade.

### Client timeout

`ZatcaClient.WithTimeout` takes a `TimeSpan` instead of a `uint` millisecond
count. The timeout is rounded up to whole milliseconds; zero, negative
(including `Timeout.InfiniteTimeSpan`) and over-`uint.MaxValue`-millisecond
values throw `ArgumentOutOfRangeException`. Use `ZatcaClient.Create(config)`
for no client-side timeout.

| 0.6 | 0.7 |
| --- | --- |
| `ZatcaClient.WithTimeout(config, 30_000)` | `ZatcaClient.WithTimeout(config, TimeSpan.FromSeconds(30))` |

### Binary output

`Bytes.AsSlice()`, which returned a disposable `DiplomatBorrowedSpan<byte>`
view, is replaced by `Bytes.ToArray()`, which returns a new `byte[]`. The array
stays valid after the `Bytes` owner is disposed, so there is no view to dispose
and no `WithSpan()` read. The `Fatoora.Diplomat` byte-view types are no longer
public.

| 0.6 | 0.7 |
| --- | --- |
| `using var view = bytes.AsSlice(); byte[] der = view.Clone();` | `byte[] der = bytes.ToArray();` |
| `view.WithSpan(span => ...)` | `ReadOnlySpan<byte> span = bytes.ToArray();` |

### `Text` class

The `Fatoora.Text` class is removed. No public member returned it in 0.6:
optional text was already `string?`.

## Error classes

0.7 gives every language the same error hierarchy: a base `FatooraException`, one
`ErrorKind` enum, and one subclass per kind named `<Kind>Exception`. See
[errors and diagnostics](../../reference/errors.md). These names change in 0.7
with no deprecated alias in 0.6, so update them while upgrading.

| 0.6 | 0.7 |
| --- | --- |
| `FfiException` | `FatooraException`, which now carries `Code`, `Kind` and `DetailsJson` |
| `FfiErrorKind` | `ErrorKind` (same members and values) |
| `FfiException.Create(code, message, detailsJson)` | `FatooraException.FromCode(code, message, detailsJson)` |
| `new InvalidInputException(message, code, detailsJson)` (any subclass) | `new InvalidInputException(message, detailsJson)`; the code comes from the class |
| `new FatooraException(message)`, `new FatooraException(message, inner)` | `new FatooraException(message, code, detailsJson)` |

The subclasses (`InvalidInputException`, ..., `ApiException`) keep their names.
A `catch (FfiException error)` becomes `catch (FatooraException error)`.

## Example

=== "0.6 (obsolete)"

    ```csharp
    using var config = Config.New(ZatcaEnvironment.Simulation);
    using var rebate = Adjustment.PercentOf(250m, 10m);
    using var supplied = rebate.WithSuppliedAmount(25m);
    Console.WriteLine(rebate.Percent);
    ```

=== "0.7"

    ```csharp
    using var config = Config.Create(ZatcaEnvironment.Simulation);
    using var rebate = Adjustment.PercentOf(250m, 10m);
    using var supplied = rebate.SuppliedAmount(25m);
    Console.WriteLine(rebate.Percentage);
    ```
