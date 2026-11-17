---
title: Migrate a Rust Project from fatoora-core 0.5 to 0.6
description: Every renamed, retyped, removed and deprecated fatoora-core item between 0.5 and 0.6, with before-and-after Rust code for the common migrations.
---

# Migrate from 0.5 to 0.6 (Rust)

This page covers the `fatoora-core` Rust crate only. The language bindings
(Python, JavaScript/TypeScript, Kotlin/Java, C#/.NET and C/C++) keep their 0.5
API until the Phase 2 FFI release, which will come with its own migration
guides.

0.6 renames items for consistency, replaces strings and Booleans with typed
values, and makes the fields of invoice value types private. Each change below
is one of two kinds:

- **Deprecated**: the old name still compiles, with a deprecation warning, and
  forwards to the new one. It will be removed in 0.7.
- **Breaking**: the old form no longer compiles. The type, signature or field
  changed in place, so there is no deprecation period.

A practical order of work: fix the breaking errors until the project compiles,
then run `cargo build` (or `cargo clippy`) and work through the `deprecated`
warnings, each of which names its replacement. Adding `#![deny(deprecated)]`
to your crate root while you migrate turns the remaining old names into errors.

The full list of changes, including additions, is in the
[changelog](https://github.com/mqqz/fatoority/blob/main/CHANGELOG.md).

## Changed items

Paths are relative to `fatoora_core`. Status is **Deprecated** (compiles with a
warning until 0.7) or **Breaking** (fails to compile).

### Configuration

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `config::EnvironmentType` | `config::Environment` | Deprecated |
| Exhaustive `match` on `EnvironmentType` | `Environment` is `#[non_exhaustive]`; add a `_ =>` arm | Breaking |

`Config::new`, `Config::env`, `CsidCredentials::new`, `CsidCredentials::env`
and `CsrProperties::build` now name `Environment`; the deprecated alias keeps
code that names `EnvironmentType` compiling.

### API client and responses

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `ZatcaClient::post_csr_for_ccsid(csr, otp)` | `ZatcaClient::request_compliance_csid(csr, otp)` | Deprecated |
| `ZatcaClient::post_ccsid_for_pcsid(ccsid)` | `ZatcaClient::request_production_csid(ccsid)` | Deprecated |
| `ZatcaClient::prepare_post_csr_for_ccsid(csr, otp)` | `ZatcaClient::prepare_request_compliance_csid(csr, otp)` | Deprecated |
| `ZatcaClient::prepare_post_ccsid_for_pcsid(ccsid)` | `ZatcaClient::prepare_request_production_csid(ccsid)` | Deprecated |
| `ZatcaClient::report_simplified_invoice(.., clearance_status: bool, ..)` | `report_simplified_invoice(.., clearance_mode: ClearanceMode, ..)` | Breaking |
| `ZatcaClient::clear_standard_invoice(.., clearance_status: bool, ..)` | `clear_standard_invoice(.., clearance_mode: ClearanceMode, ..)` | Breaking |
| `ZatcaClient::prepare_report_simplified_invoice(.., bool, ..)` | `prepare_report_simplified_invoice(.., ClearanceMode, ..)` | Breaking |
| `ZatcaClient::prepare_clear_standard_invoice(.., bool, ..)` | `prepare_clear_standard_invoice(.., ClearanceMode, ..)` | Breaking |
| `ValidationResponse::reporting_status() -> Option<&str>` | `-> Option<&ReportingStatus>` | Breaking |
| `ValidationResponse::clearance_status() -> Option<&str>` | `-> Option<&ClearanceStatus>` | Breaking |
| `ValidationResults::status() -> Option<&str>` | `-> Option<&ValidationStatus>` | Breaking |
| `ValidationMessage::message_type() -> Option<&str>` | `-> Option<&MessageType>` | Breaking |
| `ValidationMessage::status() -> Option<&str>` | `-> Option<&ValidationStatus>` | Breaking |

`true` becomes `ClearanceMode::Enabled` and `false` becomes
`ClearanceMode::Disabled`; `ClearanceMode::from(flag)` converts an existing
Boolean. The status enums keep any value they do not recognise in an
`Other(String)` variant, and `as_str()` returns the wire text, so
`.map(ReportingStatus::as_str)` gives back the 0.5 `Option<&str>`. The
`api::ComplianceCsid` and `api::ProductionCsid` aliases are new shorthands for
`CsidCredentials<Compliance>` and `CsidCredentials<Production>`.

### CSR

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `CsrProperties::from_properties_str(text)` | `text.parse::<CsrProperties>()` (`FromStr`) | Deprecated |
| `CsrProperties::parse_csr_config(text)` | `text.parse::<CsrProperties>()` (`FromStr`) | Deprecated |
| `CsrProperties::parse_csr_config_file(path)` | `CsrProperties::from_file(path)` | Deprecated |

### Invoice building

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `InvoiceBuilder::uuid(impl Into<String>)` | `InvoiceBuilder::uuid(InvoiceUuid)` | Breaking |
| `InvoiceBuilder::issue_datetime(impl Into<String>)` | `InvoiceBuilder::issue_datetime(InvoiceTimestamp)` | Breaking |
| `InvoiceBuilder::currency(impl Into<String>)` | `InvoiceBuilder::currency(CurrencyCode)` | Breaking |
| `InvoiceBuilder::previous_invoice_hash(impl Into<String>)` | `InvoiceBuilder::previous_invoice_hash(InvoiceHash)` | Breaking |
| `Seller::new(name: String, address, vat_id: impl Into<String>, other_id) -> Result<Seller>` | `Seller::new(name: impl Into<String>, address, vat_id: VatId, other_id) -> Seller` | Breaking |
| `Buyer::new(name: String, address, vat_id: Option<String>, other_id) -> Result<Buyer>` | `Buyer::new(name: impl Into<String>, address, vat_id: Option<VatId>, other_id) -> Buyer` | Breaking |
| `OriginalInvoiceRef::with_uuid(impl Into<String>)` | `OriginalInvoiceRef::with_uuid(InvoiceUuid)` | Breaking |
| `OriginalInvoiceRef::with_issue_date_str(text)?` | `OriginalInvoiceRef::with_issue_date(text.parse()?)` | Breaking (removed) |
| `Address { country_code, city, street, building_number, postal_code, .. }` | `Address::new(country_code, city, street, building_number, postal_code)` plus `with_province`, `with_additional_street`, `with_additional_number`, `with_district` | Breaking |
| `address.city` (and the other `Address` fields) | `address.city()` (getters of the same name) | Breaking |
| `TaxExemption { reason_code, reason }` | `TaxExemption::default().with_reason_code(code).with_reason(text)` | Breaking |
| `exemption.reason_code`, `exemption.reason` | `exemption.reason_code()`, `exemption.reason()` | Breaking |
| `Delivery { actual_date, latest_date }` | `Delivery::default().with_actual_date(date).with_latest_date(date)` | Breaking |
| `delivery.actual_date`, `delivery.latest_date` | `delivery.actual_date()`, `delivery.latest_date()` | Breaking |
| `invoice::LineItems` | `Vec<LineItem>` | Breaking (removed) |

Parse a typed value with `"SAR".parse()?`, `CurrencyCode::parse("SAR")?` or
`CurrencyCode::try_from("SAR")?`; `InvoiceUuid`, `InvoiceTimestamp`,
`InvoiceHash`, `VatId`, `InvoiceDate` and `InvoiceTime` work the same way.

### Invoice data getters

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `InvoiceData::uuid() -> &str` | `-> &InvoiceUuid` | Breaking |
| `InvoiceData::issue_date() -> &str` | `-> InvoiceDate` | Breaking |
| `InvoiceData::issue_time() -> &str` | `-> InvoiceTime` | Breaking |
| `InvoiceData::previous_invoice_hash() -> &str` | `-> &InvoiceHash` | Breaking |
| `InvoiceData::note()` | `InvoiceData::notes().first()` | Breaking (removed) |
| `SignedInvoice::uuid() -> &str` | `-> &InvoiceUuid` | Breaking |
| `SignedInvoice::invoice_hash() -> &str` | `-> &InvoiceHash` | Breaking |
| `SignedProperties::invoice_hash() -> &str` | `-> &InvoiceHash` | Breaking |
| `SignedProperties::signing_time() -> &str` (`YYYY-MM-DDTHH:MM:SS`) | `-> &InvoiceTimestamp` (`as_str()` is `YYYY-MM-DDTHH:MM:SSZ`) | Breaking |
| `OriginalInvoiceRef::uuid() -> Option<&str>` | `-> Option<&InvoiceUuid>` | Breaking |
| `invoice::InvoiceTotalsData` (returned by `totals()`) | `invoice::InvoiceTotals` | Deprecated |

Every typed value has `as_str()` and `Display`, so appending `.as_str()`
restores the 0.5 string. The XAdES `SigningTime` element in signed XML is
unchanged; only the value returned by `signing_time()` gains the `Z`.

### Adjustments and payment means

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `InvoiceBuilder::adjustment(AllowanceCharge)` | `allowance(DocumentAdjustment::from(entry))` or `charge(..)` | Deprecated |
| `InvoiceBuilder::invoice_level_discount(amount)` | `allowance(DocumentAdjustment::new(Adjustment::amount(amount), category, rate))` | Deprecated |
| `InvoiceBuilder::invoice_level_charge(amount)` | `charge(DocumentAdjustment::new(Adjustment::amount(amount), category, rate))` | Deprecated |
| `InvoiceBuilder::allowance_reason(text)` | `DocumentAdjustment::with_reason(text)` on each entry | Deprecated |
| `InvoiceBuilder::payment_means_code(code)` | `InvoiceBuilder::payment_means(PaymentMeans::new(code))` | Deprecated |
| `AllowanceCharge { charge: bool, .. }` / `entry.charge` | `AllowanceCharge::new(kind, amount, category, rate)` / `entry.kind()` returning `AdjustmentKind` | Breaking |
| `AllowanceCharge` struct literal and field reads | `AllowanceCharge::new(..)` plus `with_base_amount`, `with_percentage`, `with_reason`, `with_reason_code`, `with_exemption`, `with_tax_category_count`; getters of the same names | Breaking |
| `entry.omitted_rate = true` / `entry.omitted_rate` | `entry.omit_vat_rate()` / `!entry.has_vat_rate()` | Breaking |
| `LineAdjustment::charge: bool` | `LineAdjustment::kind() -> AdjustmentKind` | Breaking |
| `LineAdjustment::adjustment` field | `LineAdjustment::adjustment()` | Breaking |
| `DocumentAdjustment { adjustment, category, rate, exemption }` field reads | `adjustment()`, `category()`, `rate()`, `exemption()` (build with `DocumentAdjustment::new`) | Breaking |
| `DocumentAdjustment::reason(text)` setter | `DocumentAdjustment::with_reason(text)`; `reason()` is now the getter | Breaking |
| `DocumentAdjustment::reason_code(code)` setter | `DocumentAdjustment::with_reason_code(code)`; `reason_code()` is now the getter | Breaking |
| `DocumentAdjustment::exemption(value)` setter | `DocumentAdjustment::with_exemption(value)`; `exemption()` is now the getter | Breaking |
| `Adjustment::percentage(base, percent)` | `Adjustment::percent_of(base, percent)`; `percentage()` is now the getter | Breaking |
| `Adjustment::reason(text)` setter | `Adjustment::with_reason(text)`; `reason()` is now the getter | Breaking |
| `Adjustment::reason_code(code)` setter | `Adjustment::with_reason_code(code)`; `reason_code()` is now the getter | Breaking |
| `Adjustment::base_amount(amount)` setter | `Adjustment::with_base_amount(amount)`; `base_amount()` is now the getter | Breaking |
| `Adjustment::allowance_charge_reason()` | `Adjustment::reason()` | Deprecated |
| `Adjustment::allowance_charge_reason_code()` | `Adjustment::reason_code()` | Deprecated |
| `Adjustment::calculation_base_amount()` | `Adjustment::base_amount()` | Deprecated |
| `Adjustment::calculation_percentage()` | `Adjustment::percentage()` | Deprecated |
| `VatBreakdown::without_vat_rate()` | `VatBreakdown::omit_vat_rate()` | Deprecated |
| `PaymentMeans { payment_means_type_code, instruction_notes, payee_financial_account }` field reads | `code()`, `instruction_notes()`, `payee_financial_account()` (build with `PaymentMeans::new(code)`) | Breaking |
| `PaymentMeans::instruction_note(text)` | `PaymentMeans::with_instruction_note(text)` | Deprecated |
| `PaymentMeans::payee_financial_account(account)` setter | `PaymentMeans::with_payee_financial_account(account)`; `payee_financial_account()` is now the getter | Breaking |
| `PaymentAccount { payment_account_identifier, payment_terms }` | `PaymentAccount::default().with_payment_account_identifier(id).with_payment_terms(terms)`; getters of the same names | Breaking |

`AdjustmentKind::from_charge_indicator(flag)` converts a 0.5 `charge` Boolean
and `kind.is_charge()` reads one back.

The deprecated `invoice_level_discount`, `invoice_level_charge` and
`allowance_reason` keep their 0.5 behaviour: each call replaces a single
aggregate entry in the document VAT category, with the rate taken from the
first line in that category. `allowance` and `charge` append one entry per
call with the VAT category and rate you pass, so a migrated invoice gets the
same totals only if you pass that category and rate. Likewise,
`payment_means_code` changes the code of the first payment means, while
`payment_means` appends one.

### Validation and hashing

| 0.5 | 0.6 | Status |
| --- | --- | --- |
| `invoice::validation::validate_xml_invoice_from_str(xml, &config)` | `Validator::new().validate_xml(xml)` | Deprecated |
| `invoice::validation::validate_xml_invoice_report_from_str(xml, &config)` | `Validator::new().validate_xml_report(xml)` | Deprecated |
| `invoice::validation::validate_zatca_invoice_from_str(xml, &config, &options)` | `Validator::new().validate_zatca(xml, &options)` | Deprecated |
| `invoice::sign::invoice_hash_base64_from_xml_str(xml) -> Result<String, _>` | `invoice::hash_xml(xml) -> Result<InvoiceHash, _>` | Deprecated |
| `FinalizedInvoice::hash_base64() -> Result<String, SigningError>` | `FinalizedInvoice::hash() -> Result<InvoiceHash, SigningError>` | Deprecated |
| `SignedInvoice::hash_base64() -> Result<String, SigningError>` | `SignedInvoice::hash() -> Result<InvoiceHash, SigningError>` | Deprecated |
| `ZatcaValidationOptions { previous_invoice_hash: Option<String>, .. }` | `previous_invoice_hash: Option<InvoiceHash>` | Breaking |

Validation never depended on the environment, so `Validator` takes no
`Config`. The deprecated functions ignore their `Config` argument, and
`invoice_hash_base64_from_xml_str` and the deprecated `hash_base64` methods
still return a `String`.

Set `previous_invoice_hash` from an `InvoiceHash`: the value returned by
`hash()` or `hash_xml`, `data.previous_invoice_hash().clone()`, or
`text.parse()?`. Parsing trims surrounding whitespace and rejects blank text;
`validate_zatca` still checks the digest format. Deserialized options (for
example from JSON) go through the same parsing.

## Behaviour and data format changes

These compile unchanged but can change what your program sees:

- `InvoiceBuilder::build` no longer reports `Empty` or `InvalidFormat` issues
  for the UUID, issue date-time, currency or previous invoice hash. An invalid
  value now fails earlier, where you parse it into its typed wrapper; `build`
  only reports these four fields as `Missing`.
- A `DocumentAdjustment` whose amount calculation is invalid is reported with
  the other adjustment checks in `build`, after missing or invalid fields,
  instead of before them.
- Serialized `AllowanceCharge` and `LineAdjustment` values, and therefore
  serialized `InvoiceData` and `LineItem` values, use a `"kind"` field with the
  values `"allowance"`/`"charge"` instead of the Boolean `"charge"` field.
  JSON written by 0.5 is still accepted on read: a legacy `"charge": true`
  reads as a charge and `"charge": false` as an allowance. Serialization writes
  only `"kind"`, so data saved again is in the 0.6 format. Input with both
  fields is accepted when they agree and rejected when they conflict. The other
  serde field names are unchanged.
- The JSON of ZATCA responses and the `InvoiceOutcome` returned by
  `ValidationResponse::outcome()` are unchanged.

## Common migrations

### Build an invoice with typed inputs and an allowance

=== "0.5"

    ```rust
    use fatoora_core::invoice::{
        Address, CountryCode, FinalizedInvoice, InvoiceBuilder, InvoiceSubType, InvoiceType,
        LineItem, Party, SellerRole, VatCategory,
    };

    fn build_invoice() -> Result<FinalizedInvoice, Box<dyn std::error::Error>> {
        let seller = Party::<SellerRole>::new(
            "Example Store".to_string(),
            Address {
                country_code: CountryCode::parse("SA")?,
                city: "Riyadh".into(),
                street: "King Fahd Road".into(),
                building_number: "1234".into(),
                postal_code: "12222".into(),
                district: Some("Olaya".into()),
                additional_number: None,
                additional_street: None,
                province: None,
            },
            "399999999900003",
            None,
        )?;
        let item = LineItem::new(
            "Notebook",
            "2".parse()?,
            "PCE",
            "50".parse()?,
            "15".parse()?,
            VatCategory::Standard,
        )?;
        let invoice = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
            .id("INV-1001")
            .uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
            .issue_datetime("2026-10-26T09:00:00Z")
            .currency("SAR")
            .previous_invoice_hash("NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==")
            .invoice_counter(1)
            .seller(seller)
            .payment_means_code("10")
            .vat_category(VatCategory::Standard)
            .line_item(item)
            .invoice_level_discount("10".parse()?)
            .allowance_reason("Loyalty discount")
            .build()?;

        let data = invoice.data();
        println!("{} issued {}", data.uuid(), data.issue_date());
        if let Some(note) = data.note() {
            println!("{note:?}");
        }
        Ok(invoice)
    }
    ```

=== "0.6"

    ```rust
    use fatoora_core::invoice::{
        Address, Adjustment, CountryCode, DocumentAdjustment, FinalizedInvoice, InvoiceBuilder,
        InvoiceSubType, InvoiceType, LineItem, Party, PaymentMeans, SellerRole, VatCategory, VatId,
    };

    fn build_invoice() -> Result<FinalizedInvoice, Box<dyn std::error::Error>> {
        let seller = Party::<SellerRole>::new(
            "Example Store",
            Address::new(CountryCode::parse("SA")?, "Riyadh", "King Fahd Road", "1234", "12222")
                .with_district("Olaya"),
            VatId::parse("399999999900003")?,
            None,
        );
        let item = LineItem::new(
            "Notebook",
            "2".parse()?,
            "PCE",
            "50".parse()?,
            "15".parse()?,
            VatCategory::Standard,
        )?;
        let invoice = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
            .id("INV-1001")
            .uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d".parse()?)
            .issue_datetime("2026-10-26T09:00:00Z".parse()?)
            .currency("SAR".parse()?)
            .previous_invoice_hash("NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==".parse()?)
            .invoice_counter(1)
            .seller(seller)
            .payment_means(PaymentMeans::new("10"))
            .vat_category(VatCategory::Standard)
            .line_item(item)
            .allowance(
                DocumentAdjustment::new(
                    Adjustment::amount("10".parse()?),
                    VatCategory::Standard,
                    "15".parse()?,
                )
                .with_reason("Loyalty discount"),
            )
            .build()?;

        let data = invoice.data();
        println!("{} issued {}", data.uuid(), data.issue_date());
        if let Some(note) = data.notes().first() {
            println!("{note:?}");
        }
        Ok(invoice)
    }
    ```

### Request CSIDs and report an invoice

=== "0.5"

    ```rust
    use fatoora_core::api::ZatcaClient;
    use fatoora_core::config::{Config, EnvironmentType};
    use fatoora_core::csr::{CsrProperties, SigningKey};
    use fatoora_core::invoice::SignedInvoice;

    async fn onboard_and_report(
        otp: &str,
        invoice: &SignedInvoice,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let env = EnvironmentType::Simulation;
        let client = ZatcaClient::new(Config::new(env))?;
        let csr = CsrProperties::parse_csr_config_file("csr.properties")?
            .build(&SigningKey::generate(), env)?;

        let ccsid = client.post_csr_for_ccsid(&csr, otp).await?;
        let pcsid = client.post_ccsid_for_pcsid(&ccsid).await?;
        let response = client
            .report_simplified_invoice(invoice, &pcsid, false, Some("en"))
            .await?;
        response.ensure_accepted()?;

        let label = match env {
            EnvironmentType::Production => "live",
            EnvironmentType::Simulation | EnvironmentType::NonProduction => "test",
        };
        println!("reported to {label}");
        Ok(())
    }
    ```

=== "0.6"

    ```rust
    use fatoora_core::api::{ClearanceMode, ComplianceCsid, ProductionCsid, ZatcaClient};
    use fatoora_core::config::{Config, Environment};
    use fatoora_core::csr::{CsrProperties, SigningKey};
    use fatoora_core::invoice::SignedInvoice;

    async fn onboard_and_report(
        otp: &str,
        invoice: &SignedInvoice,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let env = Environment::Simulation;
        let client = ZatcaClient::new(Config::new(env))?;
        let csr = CsrProperties::from_file("csr.properties")?
            .build(&SigningKey::generate(), env)?;

        let ccsid: ComplianceCsid = client.request_compliance_csid(&csr, otp).await?;
        let pcsid: ProductionCsid = client.request_production_csid(&ccsid).await?;
        let response = client
            .report_simplified_invoice(invoice, &pcsid, ClearanceMode::Disabled, Some("en"))
            .await?;
        response.ensure_accepted()?;

        // `Environment` is non-exhaustive: keep a wildcard arm.
        let label = match env {
            Environment::Production => "live",
            _ => "test",
        };
        println!("reported to {label}");
        Ok(())
    }
    ```

### Read response statuses

=== "0.5"

    ```rust
    use fatoora_core::api::ValidationResponse;

    fn summarize(response: &ValidationResponse) {
        match response.reporting_status() {
            Some("REPORTED") => println!("reported"),
            Some("NOT_REPORTED") => println!("not reported"),
            Some(other) => println!("unrecognised status {other}"),
            None => {}
        }
        if response.clearance_status() == Some("CLEARED") {
            println!("cleared");
        }
        let status: Option<&str> = response.validation_results().status();
        println!("validation status: {status:?}");
        for message in response.validation_results().warning_messages() {
            if message.message_type() == Some("WARNING") {
                println!("{:?}", message.message());
            }
        }
    }
    ```

=== "0.6"

    ```rust
    use fatoora_core::api::{
        ClearanceStatus, MessageType, ReportingStatus, ValidationResponse, ValidationStatus,
    };

    fn summarize(response: &ValidationResponse) {
        match response.reporting_status() {
            Some(ReportingStatus::Reported) => println!("reported"),
            Some(ReportingStatus::NotReported) => println!("not reported"),
            Some(other) => println!("unrecognised status {other}"),
            None => {}
        }
        if response.clearance_status() == Some(&ClearanceStatus::Cleared) {
            println!("cleared");
        }
        let status: Option<&str> = response
            .validation_results()
            .status()
            .map(ValidationStatus::as_str);
        println!("validation status: {status:?}");
        for message in response.validation_results().warning_messages() {
            if message.message_type() == Some(&MessageType::Warning) {
                println!("{:?}", message.message());
            }
        }
    }
    ```

### Validate and hash an invoice

=== "0.5"

    ```rust
    use fatoora_core::config::{Config, EnvironmentType};
    use fatoora_core::invoice::sign::invoice_hash_base64_from_xml_str;
    use fatoora_core::invoice::validation::{
        validate_xml_invoice_from_str, validate_zatca_invoice_from_str, ZatcaValidationOptions,
    };

    fn check(xml: &str, previous_hash: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = Config::new(EnvironmentType::NonProduction);
        validate_xml_invoice_from_str(xml, &config)?;

        let options = ZatcaValidationOptions {
            previous_invoice_hash: Some(previous_hash.to_string()),
            ..Default::default()
        };
        let report = validate_zatca_invoice_from_str(xml, &config, &options)?;
        if !report.is_valid() {
            return Err("invoice failed local ZATCA validation".into());
        }

        // Pass to the next invoice's `previous_invoice_hash(..)`.
        let hash: String = invoice_hash_base64_from_xml_str(xml)?;
        Ok(hash)
    }
    ```

=== "0.6"

    ```rust
    use fatoora_core::invoice::validation::{Validator, ZatcaValidationOptions};
    use fatoora_core::invoice::{hash_xml, InvoiceHash};

    fn check(xml: &str, previous_hash: &InvoiceHash) -> Result<InvoiceHash, Box<dyn std::error::Error>> {
        let validator = Validator::new();
        validator.validate_xml(xml)?;

        let options = ZatcaValidationOptions {
            previous_invoice_hash: Some(previous_hash.clone()),
            ..Default::default()
        };
        let report = validator.validate_zatca(xml, &options)?;
        if !report.is_valid() {
            return Err("invoice failed local ZATCA validation".into());
        }

        // Pass to the next invoice's `previous_invoice_hash(..)`.
        let hash: InvoiceHash = hash_xml(xml)?;
        Ok(hash)
    }
    ```
