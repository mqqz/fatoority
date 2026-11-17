---
title: Migrate from fatoority 0.5 to 0.6
description: What changes between fatoority 0.5 and 0.6 in Rust, Python, JavaScript, Kotlin/Java, C# and C/C++, with a migration page per language.
---

# Migrate from 0.5 to 0.6

0.6 is a breaking release for every package. The Rust core renames items for
consistency and replaces strings and Booleans with typed values; every language
package now uses those names, takes enums instead of integer codes, and drops
the `Config` argument from validation. Upgrade the language package and its
native library together: all packages are released as 0.6.0.

Pick your language:

| Language | Package | Deprecated 0.5 names | Migration page |
| --- | --- | --- | --- |
| Rust | `fatoora-core = "0.6.0"` | Renamed items warn until 0.7; retyped items break | [Rust](rust.md) |
| Python | `pip install "fatoority==0.6.0"` | Renamed names warn until 0.7; getters becoming properties breaks | [Python](python.md) |
| JavaScript / TypeScript | `npm install fatoority@0.6.0` | Only `new_` (use `create`); other renames break | [JavaScript/TypeScript](javascript.md) |
| Kotlin / Java | `com.fatoority:fatoority:0.6.0` | 0.5 getter functions and `String` amount overloads warn until 0.7; renames break | [Kotlin/Java](kotlin.md) |
| C# / .NET | `Fatoority` 0.6.0 | None; every change breaks | [C#/.NET](csharp.md) |
| C / C++ | Release archives | None; every change breaks | [C/C++](c.md) |

Where a language keeps deprecated names, they forward to the new ones and are
removed in 0.7. A good order of work is to fix the compile errors (or, in
Python, the property calls) first, then turn deprecation warnings into errors
and replace what remains.

## What changed everywhere

These changes come from the core and appear in every language. Each language
page shows them with its own before-and-after code.

| Change | Rust | Python | JavaScript | Kotlin / Java | C# | C |
| --- | --- | --- | --- | --- | --- | --- |
| Signer | `InvoiceSigner` | `InvoiceSigner` | `InvoiceSigner` | `InvoiceSigner` | `InvoiceSigner` | `FatooraInvoiceSigner` |
| CSID credentials | `ComplianceCsid`, `ProductionCsid` aliases | `ComplianceCsid`, `ProductionCsid` | `ComplianceCsid`, `ProductionCsid` | `ComplianceCsid`, `ProductionCsid` | `ComplianceCsid`, `ProductionCsid` | `FatooraComplianceCsid`, `FatooraProductionCsid` |
| Request CSIDs | `request_compliance_csid`, `request_production_csid` | same | `requestComplianceCsid`, `requestProductionCsid` | same as JavaScript | `RequestComplianceCsid`, `RequestProductionCsid` | `fatoora_zatca_client_request_*_csid` |
| Parse CSR properties | `text.parse()`, `CsrProperties::from_file` | `CsrProperties.from_str`, `from_file` | `CsrProperties.fromStr` | `CsrProperties.fromStr`, `fromFile` | `CsrProperties.FromStr`, `FromFile` | `fatoora_csr_properties_from_str`, `_from_file` |
| Environment | `Environment::NonProduction` | `Environment.NON_PRODUCTION` | `Environment.NonProduction` | `Environment.NonProduction` | `ZatcaEnvironment.NonProduction` | `FATOORA_NON_PRODUCTION` |
| Clearance header | `ClearanceMode::Disabled` | `ClearanceMode.DISABLED` | `ClearanceMode.Disabled` (or `false`) | `ClearanceMode.Disabled` | `ClearanceMode.Disabled` | `FATOORA_CLEARANCE_DISABLED` |
| Response status | `Option<&ReportingStatus>` enum | `ReportingStatus` `str` enum or `None`; unknown text kept verbatim | `ReportingStatus` enum or `null`; `reportingStatusRaw()` text | `ReportingStatus?`; `reportingStatusRaw` | `ReportingStatus?`; `ReportingStatusRaw` | enum plus `has_value`; `_raw` text |
| XSD validation | `Validator::new().validate_xml(xml)` | `validate_xml(xml)` | `Xml.validate(xml)` | `Xml.validate(xml)` | `Xml.Validate(xml)` | `fatoora_xml_validate(xml, ...)` |
| ZATCA validation | `Validator::new().validate_zatca(xml, &options)` | `validate_zatca(xml, previous_invoice_hash=...)` returning `ZatcaReport` | `Xml.validateZatca(xml, { previousInvoiceHash })` returning a plain object | `Xml.validateZatca(xml, optionsJson)` returning `ZatcaValidationReport` | `Xml.ValidateZatca(xml, optionsJson)` returning `ZatcaValidationReport` | `fatoora_xml_validate_zatca(xml, options, &report, ...)` |
| Invoice hash | `hash()` returning `InvoiceHash` | `hash()` | `hash()` | `hash` | `Hash` | `fatoora_*_invoice_hash` |
| Payment means | `payment_means(PaymentMeans::new(code))` | `add_payment_means(PaymentMeans(code))` | `addPaymentMeans(PaymentMeans.create(code, null))` | `addPaymentMeans(PaymentMeans.create(code))` | `AddPaymentMeans(PaymentMeans.New(code, null))` | `fatoora_invoice_builder_add_payment_means` |
| Document allowance | `allowance(DocumentAdjustment::new(..))` | `add_allowance(Adjustment(..), category, rate)` | `addAllowance(adjustment, category, rate, exemption)` | same as JavaScript | `AddAllowance(...)` | `fatoora_invoice_builder_add_allowance` |
| Request timeout (new) | `ZatcaClient::with_timeout(config, duration)` | `ZatcaClient(config, timeout=30)` | `AbortSignal` per request | `ZatcaClient.createWithTimeout(config, ms)` | `ZatcaClient.CreateWithTimeout(config, ms)` | `fatoora_zatca_client_create_with_timeout` |

In every language, builders use `set*` to replace a value and `add*` to append
one, so `flags`, `set_tax_exemption`, `set_note` and `set_payment_means_code`
become `set_flags`, `add_tax_exemption`, `add_note` and `add_payment_means`.
The 0.5 aggregate allowance setters (`invoice_level_discount`,
`invoice_level_charge`, `allowance_reason`, `set_allowance`) are replaced by
`allowance`/`charge` entries with an explicit VAT category and rate.

## What changed per language

- **Rust**: invoice inputs and getters use typed wrappers (`InvoiceUuid`,
  `InvoiceTimestamp`, `CurrencyCode`, `InvoiceHash`, `VatId`); the fields of
  invoice value types such as `Address` are private; `Environment` is
  `#[non_exhaustive]`; serialized adjustments use `"kind"` instead of
  `"charge"`. See [Rust](rust.md).
- **Python**: getters become properties; invoice data becomes frozen
  dataclasses (`Address`, `Party`, `Adjustment`, `Identifier`, ...); dates
  become `datetime`/`date`; keyword-only arguments are enforced. See
  [Python](python.md).
- **JavaScript/TypeScript**: library failures throw `FatooraError` subclasses;
  `validateZatca` returns a plain object; `new_` becomes `create`. See
  [JavaScript/TypeScript](javascript.md).
- **Kotlin/Java**: value getters become properties; amounts are `BigDecimal`;
  optional text is `String?`; failures throw `BindingException` subclasses.
  See [Kotlin/Java](kotlin.md).
- **C#/.NET**: scalar getters become properties; amounts are `decimal`;
  optional text is `string?`; `BindingErrorException` becomes `FfiException`
  and its subclasses. See [C#/.NET](csharp.md).
- **C/C++**: functions are renamed with the core types, enums are typed and
  ZATCA validation returns a report object. See [C/C++](c.md).

The CLI's commands and flags are unchanged; a blank `--previous-invoice-hash`
is now rejected as invalid options (exit code 3). The full list of
changes, including additions, is in the
[changelog](https://github.com/mqqz/fatoority/blob/main/CHANGELOG.md).
