---
title: Migrate a C# Project from fatoority 0.5 to 0.6
description: Upgrade C# code from the Fatoority 0.5 NuGet package to 0.6. Enums replace integer codes, getters become properties, amounts become decimal and failures throw typed exceptions.
---

# Migrate from 0.5 to 0.6: C# and .NET

The `Fatoority` NuGet package is an idiomatic C# facade in 0.6. Integer codes
and Booleans become enums, class and method names follow the Rust core, scalar
getters become read-only properties, amounts are `decimal`, optional text is
`string?`, and native failures throw one exception class per error code. C#
cannot overload a method with a property or by return type, so every change is
a hard break with no deprecated alias: work through the compiler errors.

```sh
dotnet add package Fatoority --version 0.6.0
```

The package bundles its native libraries. Native objects still implement
`IDisposable`; keep your `using` declarations.

## Upgrade checklist

1. Replace integer codes and Booleans with enums:
   `ZatcaEnvironment.NonProduction` for `0` (named to avoid clashing with
   `System.Environment`), `InvoiceTypeKind.Tax` and `InvoiceSubType.Simplified`
   for `0, 1`, `VatCategory.Standard` for `1`, `ClearanceMode.Disabled` for
   `false`. The enum values equal the old codes.
2. Rename classes and methods (see the [table below](#renamed-names)):
   `Signer` to `InvoiceSigner`, `CsidCompliance`/`CsidProduction` to
   `ComplianceCsid`/`ProductionCsid`, `PostCsrForCcsid`/`PostCcsidForPcsid` to
   `RequestComplianceCsid`/`RequestProductionCsid`, `FromPropertiesStr` to
   `FromStr`, `HashBase64()` to `Hash`.
3. Drop the parentheses from scalar getters: `address.City()` becomes
   `address.City`, `response.Outcome()` becomes `response.Outcome`. Getters that
   return a disposable snapshot stay methods: `invoice.Data()`,
   `invoice.Totals()`, `response.ValidationResults()`, `request.Body()`.
4. Pass and read amounts as `decimal` (`2m`, `50m`) instead of strings.
5. Replace `Text?` handling with `string?`, and `BindingErrorException` with
   `FfiException` or one of its subclasses.

## Build an invoice

=== "0.5"

    ```csharp
    using Fatoora;

    using var builder = InvoiceBuilder.New(0, 1, null, null, null, null);
    builder.SetId("INV-1001");
    builder.SetUuid(uuid);
    builder.SetIssueDatetime("2026-10-26T09:00:00Z");
    builder.SetCurrency("SAR");
    builder.SetInvoiceCounter(1);
    builder.SetPreviousInvoiceHash(previousHash);
    builder.SetPaymentMeansCode("10");
    builder.SetVatCategory(1);
    using var address = Address.New("SA", "Riyadh", "King Fahd Road", "1234", "12222", null, "5678", "Olaya");
    builder.SetSeller("Example Store", address, "399999999900003", "7003339333", "CRN");
    builder.AddLineItem("Notebook", "2", "PCE", "50", "15", 1);
    builder.InvoiceLevelDiscount("10");
    builder.AllowanceReason("Loyalty discount");
    using var invoice = builder.Build();
    File.WriteAllText("invoice.xml", invoice.Xml());
    using var totals = invoice.Totals();
    decimal payable = decimal.Parse(totals.PayableAmount(), CultureInfo.InvariantCulture);
    ```

=== "0.6"

    ```csharp
    using Fatoora;

    using var builder = InvoiceBuilder.New(InvoiceTypeKind.Tax, InvoiceSubType.Simplified, null, null, null, null);
    builder.SetId("INV-1001");
    builder.SetUuid(uuid);
    builder.SetIssueDatetime("2026-10-26T09:00:00Z");
    builder.SetCurrency("SAR");
    builder.SetInvoiceCounter(1);
    builder.SetPreviousInvoiceHash(previousHash);
    using (var means = PaymentMeans.New("10", null))
        builder.AddPaymentMeans(means);
    builder.SetVatCategory(VatCategory.Standard);
    using var address = Address.New("SA", "Riyadh", "King Fahd Road", "1234", "12222", null, "5678", "Olaya");
    builder.SetSeller("Example Store", address, "399999999900003", "7003339333", "CRN");
    builder.AddLineItem("Notebook", 2m, "PCE", 50m, 15m, VatCategory.Standard);
    using (var amount = Adjustment.Amount(10m))
    using (var discount = amount.WithReason("Loyalty discount", null))
    using (var exemption = TaxExemption.New(null, null))
        builder.AddAllowance(discount, VatCategory.Standard, 15m, exemption);
    using var invoice = builder.Build();
    File.WriteAllText("invoice.xml", invoice.Xml);
    using var totals = invoice.Totals();
    decimal payable = totals.PayableAmount;
    ```

`SetPaymentMeansCode`, `SetAllowance`, `AllowanceReason`, `InvoiceLevelCharge`
and `InvoiceLevelDiscount` are gone from the builder. `AddAllowance` and
`AddCharge` append one entry each in the VAT category and at the rate you pass.
Returned amounts are normalized (`1.50` comes back as `1.5m`); both compare
equal as `decimal`.

## Sign, validate and report

=== "0.5"

    ```csharp
    try
    {
        using var signer = Signer.FromPem(certificatePem, keyPem);
        using var signed = signer.Sign(invoice);
        string signedXml = signed.Xml();
        using var config = Config.New(0);
        string reportJson = Xml.ValidateZatca(config, signedXml, optionsJson);
        using var report = JsonDocument.Parse(reportJson);
        if (!report.RootElement.GetProperty("is_valid").GetBoolean())
            throw new InvalidOperationException("Local validation failed");

        using var client = ZatcaClient.Create(config);
        using var credentials = CsidProduction.Create(0, null, token, secret);
        using var response = client.ReportSimplifiedInvoice(signed, credentials, false, "en");
        using var status = response.ReportingStatus();
        Console.WriteLine(status?.Value());
        response.EnsureAccepted();
    }
    catch (BindingErrorException error)
    {
        using var inner = error.Inner;
        Console.Error.WriteLine($"{inner.Code()}: {inner.DetailsJson()}");
        throw;
    }
    ```

=== "0.6"

    ```csharp
    try
    {
        using var signer = InvoiceSigner.FromPem(certificatePem, keyPem);
        using var signed = signer.Sign(invoice);
        string signedXml = signed.Xml;
        using var report = Xml.ValidateZatca(signedXml, optionsJson);
        if (!report.IsValid)
            throw new InvalidOperationException("Local validation failed: " + report.ToJson());

        using var config = Config.New(ZatcaEnvironment.NonProduction);
        using var client = ZatcaClient.Create(config);
        using var credentials = ProductionCsid.Create(ZatcaEnvironment.NonProduction, null, token, secret);
        using var response = client.ReportSimplifiedInvoice(signed, credentials, ClearanceMode.Disabled, "en");
        ReportingStatus? status = response.ReportingStatus;
        Console.WriteLine(response.ReportingStatusRaw);
        response.EnsureAccepted();
    }
    catch (FfiException error) // InvalidInputException, NetworkException, ApiException, ...
    {
        Console.Error.WriteLine($"{error.Kind} ({error.Code}): {error.DetailsJson}");
        throw;
    }
    ```

To bound each request, create the client with
`ZatcaClient.CreateWithTimeout(config, 30_000)` (milliseconds); a timeout
throws `NetworkException`. The native client methods still block; for
cancellation, use a `Prepare*` method with your own `HttpClient`.

## Errors

`BindingErrorException` and the public `BindingError` are replaced by
`FatooraException` > `FfiException` > one class per error code:
`InvalidInputException`, `ValidationException`, `ParseException`,
`XmlException`, `CryptoException`, `IoException`, `NetworkException`,
`UnauthorizedException`, `InternalException` and `ApiException`. The exception
exposes `Code`, `Kind` (`FfiErrorKind`) and `DetailsJson` directly and holds no
native resources, so there is nothing to dispose. If you also import
`System.Xml` or `System.ComponentModel.DataAnnotations`, qualify
`Fatoora.XmlException` or `Fatoora.ValidationException`.

## Renamed names

| 0.5 | 0.6 |
| --- | --- |
| `byte` environment, invoice kind, subtype and VAT category arguments | `ZatcaEnvironment`, `InvoiceTypeKind`, `InvoiceSubType`, `VatCategory` enums |
| `bool` charge argument of `AllowanceCharge.New` | `AdjustmentKind.Allowance`/`Charge` |
| `bool clearanceStatus` in `ReportSimplifiedInvoice`, `ClearStandardInvoice` and `Prepare*` | `ClearanceMode.Disabled`/`Enabled` |
| `string` quantities, prices, rates and amounts | `decimal` (`decimal?` when optional) |
| `CsidCompliance`, `CsidProduction` | `ComplianceCsid`, `ProductionCsid` |
| `client.PostCsrForCcsid(csr, otp)`, `PostCcsidForPcsid(ccsid)` (and `PreparePost*`) | `client.RequestComplianceCsid(csr, otp)`, `RequestProductionCsid(ccsid)` (and `PrepareRequest*`) |
| `Signer` | `InvoiceSigner` |
| `CsrProperties.FromPropertiesStr(text)`, `ParseCsrConfigFile(path)` | `CsrProperties.FromStr(text)`, `FromFile(path)` |
| `invoice.HashBase64()` | `invoice.Hash` (`signed.InvoiceHash` is the stored digest) |
| `builder.Flags(mask)`, `data.FlagsRaw()` | `builder.SetFlags(mask)`, `data.Flags`; build the mask with `(byte)InvoiceFlag.Export` |
| `builder.SetTaxExemption(category, rate, exemption)` | `builder.AddTaxExemption(category, rate, exemption)` |
| `builder.AddDocumentAdjustment(charge, adjustment, category, rate, exemption)` | `builder.AddAllowance(adjustment, category, rate, exemption)` or `AddCharge(...)` |
| `builder.SetNote(language, text)`, `SetNoteWithoutLanguage(text)` | `builder.AddNote(language, text)`, with `null` for no language |
| `builder.SetPaymentMeansCode(code)` | `builder.AddPaymentMeans(PaymentMeans.New(code, null))` |
| `builder.SetAllowance(reason, amount)`, `AllowanceReason`, `InvoiceLevelCharge`, `InvoiceLevelDiscount` | `AddAllowance`/`AddCharge` with `Adjustment.Amount(amount).WithReason(reason, null)` |
| `charge.IsCharge()`, `item.AdjustmentIsCharge(i)` | `charge.Kind`, `item.AdjustmentKind(i)` returning `AdjustmentKind` |
| `response.ReportingStatus()` and the other status getters returning `Text?` | `ReportingStatus?`, `ClearanceStatus?`, `ValidationStatus?`, `MessageType?`; `*Raw` properties return the text as `string?` |
| `credentials.RequestId()` and other optional text getters returning `Text?` | `string?` properties, `null` when absent |
| `Xml.Validate(config, xml)` | `Xml.Validate(xml)` |
| `Xml.ValidateZatca(config, xml, optionsJson)` returning JSON text | `Xml.ValidateZatca(xml, optionsJson)` returning `ZatcaValidationReport`; `ToJson()` gives the old text |
| `BindingErrorException` (`e.Inner.Code()`, `e.Inner.DetailsJson()`) | `FfiException` (`e.Code`, `e.Kind`, `e.DetailsJson`) |
