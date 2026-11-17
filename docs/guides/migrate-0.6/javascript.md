---
title: Migrate a JavaScript or TypeScript Project from fatoority 0.5 to 0.6
description: Upgrade JavaScript and TypeScript code from fatoority 0.5 to 0.6. Enums replace integer codes, names mirror the Rust core, errors are typed and validation returns a plain report.
---

# Migrate from 0.5 to 0.6: JavaScript and TypeScript

The `fatoority` npm package keeps its shape in 0.6: getters are still methods,
amounts are still decimal strings, and optional text is still a `Text` object
or `null`. What changes is that integer codes become enums, class and method
names follow the Rust core, failures throw typed errors, and local validation
returns a plain report object. Except for `new_` and the
[renamed adjustment members](#renamed-members-with-deprecated-aliases), the
renamed 0.5 names are removed, not deprecated, so TypeScript reports each one.

```sh
npm install fatoority@0.6.0
```

The WebAssembly module ships inside the package.

## Upgrade checklist

1. Replace integer codes and Booleans with enums: `Environment.NonProduction`
   for `0`, `InvoiceTypeKind.Tax` and `InvoiceSubType.Simplified` for `0, 1`,
   `VatCategory.Standard` for `1`, `AdjustmentKind.Charge` for `true`.
   The enum discriminants equal the old codes.
2. Rename classes and methods: `Signer` to `InvoiceSigner`,
   `CsidCompliance`/`CsidProduction` to `ComplianceCsid`/`ProductionCsid`,
   `postCsrForCcsid`/`postCcsidForPcsid` to
   `requestComplianceCsid`/`requestProductionCsid`, `fromPropertiesStr` to
   `fromStr`, `hashBase64()` to `hash()`. The [full table](#renamed-names)
   is below.
3. Replace `X.new_(...)` with `X.create(...)`. `new_` still works and is marked
   `@deprecated` in the TypeScript declarations; it has no runtime warning.
4. Drop the `Config` argument from `Xml.validate` and `Xml.validateZatca`, and
   read the report as an object instead of parsing JSON.
5. Catch `FatooraError` (or a subclass such as `InvalidInputError`) instead of
   checking `error.cause instanceof BindingError`.

## Build an invoice

=== "0.5"

    ```ts
    import { Address, InvoiceBuilder } from 'fatoority';

    const builder = InvoiceBuilder.new_(0, 1, null, null, null, null); // Tax, simplified.
    builder.setId('INV-1001');
    builder.setUuid(uuid);
    builder.setIssueDatetime('2026-10-26T09:00:00Z');
    builder.setCurrency('SAR');
    builder.setInvoiceCounter(1n);
    builder.setPreviousInvoiceHash(previousHash);
    builder.setPaymentMeansCode('10');
    builder.setVatCategory(1); // Standard VAT category.
    const address = Address.new_('SA', 'Riyadh', 'King Fahd Road', '1234', '12222', null, '5678', 'Olaya');
    builder.setSeller('Example Store', address, '399999999900003', '7003339333', 'CRN');
    builder.addLineItem('Notebook', '2', 'PCE', '50', '15', 1);
    builder.invoiceLevelDiscount('10');
    builder.allowanceReason('Loyalty discount');
    const invoice = builder.build();
    ```

=== "0.6"

    ```ts
    import {
      Address, Adjustment, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind, PaymentMeans,
      TaxExemption, VatCategory,
    } from 'fatoority';

    const builder = InvoiceBuilder.create(InvoiceTypeKind.Tax, InvoiceSubType.Simplified, null, null, null, null);
    builder.setId('INV-1001');
    builder.setUuid(uuid);
    builder.setIssueDatetime('2026-10-26T09:00:00Z');
    builder.setCurrency('SAR');
    builder.setInvoiceCounter(1n);
    builder.setPreviousInvoiceHash(previousHash);
    builder.addPaymentMeans(PaymentMeans.create('10', null));
    builder.setVatCategory(VatCategory.Standard);
    const address = Address.create('SA', 'Riyadh', 'King Fahd Road', '1234', '12222', null, '5678', 'Olaya');
    builder.setSeller('Example Store', address, '399999999900003', '7003339333', 'CRN');
    builder.addLineItem('Notebook', '2', 'PCE', '50', '15', VatCategory.Standard);
    builder.addAllowance(
      Adjustment.amount('10').withReason('Loyalty discount', null),
      VatCategory.Standard, '15', TaxExemption.create(null, null),
    );
    const invoice = builder.build();
    ```

`setPaymentMeansCode`, `setAllowance`, `allowanceReason`, `invoiceLevelCharge`
and `invoiceLevelDiscount` are gone from the builder. `addAllowance` and
`addCharge` append one entry each in the VAT category and at the rate you pass,
where the 0.5 aggregate setters produced a single allowance and charge. The
totals getters `invoiceLevelDiscount()`, `invoiceLevelCharge()` and
`allowanceReason()` on `InvoiceTotals` are unchanged.

## Sign, validate and report

=== "0.5"

    ```ts
    import { BindingError, Config, CsidProduction, Signer, Xml, ZatcaClient } from 'fatoority';

    try {
      const signed = Signer.fromPem(certificatePem, keyPem).sign(invoice);
      const config = Config.new_(0);
      const report = JSON.parse(Xml.validateZatca(config, signed.xml(), JSON.stringify({
        previous_invoice_hash: previousHash,
      })));
      if (!report.is_valid) throw new Error('Local validation failed');

      const client = new ZatcaClient(config);
      const credentials = CsidProduction.create(0, null, token, secret);
      const response = await client.reportSimplifiedInvoice(signed, credentials, false, 'en');
      console.log(response.reportingStatus()?.value());
      response.ensureAccepted();
    } catch (error) {
      if (error instanceof Error && error.cause instanceof BindingError) {
        console.error(error.cause.code(), JSON.parse(error.cause.detailsJson()));
      }
      throw error;
    }
    ```

=== "0.6"

    ```ts
    import {
      ClearanceMode, Config, Environment, FatooraError, InvoiceSigner, ProductionCsid, Xml,
      ZatcaClient,
    } from 'fatoority';

    try {
      const signed = InvoiceSigner.fromPem(certificatePem, keyPem).sign(invoice);
      const report = Xml.validateZatca(signed.xml(), { previousInvoiceHash: previousHash });
      for (const stage of report.stages) {
        for (const finding of stage.findings) {
          console.log(`${finding.severity} ${stage.stage}/${finding.code}: ${finding.message}`);
        }
      }
      if (!report.isValid) throw new Error('Local validation failed');

      const client = new ZatcaClient(Config.create(Environment.NonProduction));
      const credentials = ProductionCsid.create(Environment.NonProduction, null, token, secret);
      const response = await client.reportSimplifiedInvoice(signed, credentials, ClearanceMode.Disabled, 'en');
      console.log(response.reportingStatus()?.value, response.reportingStatusRaw()?.value());
      response.ensureAccepted(); // Throws ApiError when not accepted.
    } catch (error) {
      if (error instanceof FatooraError) {
        console.error(error.code, error.kind, error.message, error.details);
      }
      throw error;
    }
    ```

`reportSimplifiedInvoice` and `clearStandardInvoice` on the async
`ZatcaClient` still accept `false`/`true` as well as `ClearanceMode`; the
`prepare*` methods on `fatoority/raw` require a `ClearanceMode`. The JavaScript
client has no request timeout option; pass an `AbortSignal` in the last
`options` argument (`{ signal }`) to cancel a request.

### Validation report

From the package root, `Xml.validateZatca(xml, options?)` returns plain data
with camelCase fields (`isValid`, `isComplete`, `hasErrors`, `evaluatedAt`,
`stages[].findings[]`). Enumerated fields are variant-name strings such as
`'Error'` or `'BusinessRules'`. `options` is `{ previousInvoiceHash, evaluatedAt }`
or the 0.5 options JSON string. The report can be passed to `JSON.stringify`
directly. The snake_case JSON of 0.5 is still available from the native report
on `fatoority/raw`, whose `Xml.validateZatca(xml, optionsJson)` returns a
`ZatcaValidationReport` with `toJson()`; unlike the package root, it requires an
explicit `evaluated_at` in the options.

### Response statuses

`reportingStatus()`, `clearanceStatus()`, `results.status()`,
`message.messageType()` and `message.status()` return an enum (with `Unknown`
for values this version does not recognise) or `null`, instead of a `Text`. The
new `reportingStatusRaw()`, `clearanceStatusRaw()`, `statusRaw()` and
`messageTypeRaw()` return the 0.5 `Text` with the exact wire value.

### Errors

Every library failure throws a `FatooraError` subclass chosen by error code:
`InvalidInputError`, `ValidationError`, `ParseError`, `XmlError`,
`CryptoError`, `IoError`, `NetworkError`, `UnauthorizedError`,
`InternalError` or `ApiError`. Each has `code`, `kind`, `message` and `details`
(parsed JSON). `error.cause` is still the `BindingError`, so
`error.cause.code()` keeps working. `TransportError`, thrown for network and
abort failures, now extends `NetworkError`.

## Renamed members with deprecated aliases

These members are renamed to the Rust core names. The 0.5 names still work, are
marked `@deprecated` in the TypeScript declarations and are removed in 0.7.

| 0.5 | 0.6 |
| --- | --- |
| `Adjustment.percentage(base, percent)` | `Adjustment.percentOf(base, percent)` |
| `adjustment.withSuppliedAmount(amount)` | `adjustment.suppliedAmount(amount)` |
| `adjustment.percent()` | `adjustment.percentage()` |
| `allowanceCharge.withoutVatRate()`, `vatBreakdown.withoutVatRate()` | `omitVatRate()` |

## Renamed names

| 0.5 | 0.6 |
| --- | --- |
| `Config.new_(0)` | `Config.create(Environment.NonProduction)` |
| `X.new_(...)` on `Address`, `InvoiceBuilder`, `LineItemBuilder`, `PaymentMeans`, `TaxExemption`, `CsrProperties`, `OriginalInvoiceRef`, `AllowanceCharge`, `InvoiceLineItem`, `VatBreakdown` | `X.create(...)` (`new_` deprecated) |
| `CsidCompliance`, `CsidProduction` | `ComplianceCsid`, `ProductionCsid` |
| `client.postCsrForCcsid(csr, otp)`, `postCcsidForPcsid(ccsid)` | `client.requestComplianceCsid(csr, otp)`, `requestProductionCsid(ccsid)` |
| `Signer` | `InvoiceSigner` |
| `CsrProperties.fromPropertiesStr(text)` | `CsrProperties.fromStr(text)` |
| `invoice.hashBase64()` | `invoice.hash()` (`signed.invoiceHash()` is the stored digest) |
| `builder.flags(mask)`, `data.flagsRaw()` | `builder.setFlags(mask)`, `data.flags()` |
| `builder.setTaxExemption(category, rate, exemption)` | `builder.addTaxExemption(category, rate, exemption)` |
| `builder.addDocumentAdjustment(charge, adjustment, category, rate, exemption)` | `builder.addAllowance(adjustment, category, rate, exemption)` or `addCharge(...)` |
| `builder.setNote(language, text)`, `setNoteWithoutLanguage(text)` | `builder.addNote(language, text)`, with `null` for no language |
| `builder.setPaymentMeansCode(code)` | `builder.addPaymentMeans(PaymentMeans.create(code, null))` |
| `builder.setAllowance(reason, amount)`, `allowanceReason`, `invoiceLevelCharge`, `invoiceLevelDiscount` | `addAllowance`/`addCharge` with `Adjustment.amount(amount).withReason(reason, null)` |
| `charge.isCharge()`, `item.adjustmentIsCharge(i)` | `charge.kind()`, `item.adjustmentKind(i)` returning `AdjustmentKind` |
| `config.env()`, `data.vatCategory()`, ... returning numbers | Return the enums |
| `Xml.validate(config, xml)` | `Xml.validate(xml)` |
| `Xml.validateZatca(config, xml, optionsJson)` returning JSON text | `Xml.validateZatca(xml, options)` returning a plain report |

Credential `requestId()` returns `null` when the credentials have no request
ID. Boolean getters such as `PaymentMeans.hasAccount()` and
`VatBreakdown.hasVatRate()` now return `true`/`false` instead of `1`/`0`.

Each enum also has a string-literal union type named with a `Value` suffix
(`EnvironmentValue`, `VatCategoryValue`, `SeverityValue`, ...), matching the
enum's `.value`, for typing plain data such as the validation report.
