# Changelog

## Unreleased

### Changed (breaking)

- Python: drop support for Python 3.9, which reached end of life in October
  2025. The package now requires Python 3.10 or newer and no longer ships
  `cp39` wheels.
- C#: `Bytes.AsSlice()` returns a `RustVec` holding a native copy of the bytes
  instead of a `DiplomatBorrowedSpan<byte>` view, so it stays valid after its
  owner is disposed. `Clone()`, `WithSpan()`, `Length` and `using` work as
  before.
- Bindings: `Address.province` and `ZatcaFinding.assertion_site` return a
  plain nullable string in every language instead of a `Text` handle. This
  changes the raw JavaScript methods and the deprecated Kotlin
  `province()`/`assertionSite()` methods, which returned `Text?`.

### Internal

- Bindings are generated with Diplomat 0.17.0 (`diplomat` 0.17.0,
  `diplomat-runtime` 0.16.1). The C# generator moves to current upstream
  Diplomat plus the opt-in `diplomat_str_as_string` option; the Kotlin generator
  moves to the `v0.17.0` tag.
- The C# and Kotlin generators support optional string returns (a
  `DiplomatWrite` method returning `Option<()>`); the Kotlin patch frees the
  write buffer when the result is absent. The C facade rejects any string
  output containing NUL.

### Removed

Rust (`fatoora-core`): every item deprecated in 0.6 is removed. Each had a
replacement in 0.6; see the
[Rust migration guide](https://docs.fatoority.com/guides/migrate-0.7/rust/).
The `fatoora-ffi` bridge, the CLI and the generated C, C++ and Diplomat
bindings had no deprecated items and are unchanged.

- Configuration and API client: `config::EnvironmentType` (use
  `Environment`); `ZatcaClient::post_csr_for_ccsid`, `post_ccsid_for_pcsid`,
  `prepare_post_csr_for_ccsid` and `prepare_post_ccsid_for_pcsid` (use
  `request_compliance_csid`, `request_production_csid` and their `prepare_*`
  forms).
- CSR: `CsrProperties::from_properties_str` and `parse_csr_config` (use
  `str::parse`), `parse_csr_config_file` (use `CsrProperties::from_file`).
- Invoice builder: `adjustment` (use `allowance`/`charge` with
  `DocumentAdjustment::from`), the aggregate setters `invoice_level_discount`,
  `invoice_level_charge` and `allowance_reason` (use `allowance`/`charge`
  entries with an explicit VAT category and rate, and
  `DocumentAdjustment::with_reason`), and `payment_means_code` (use
  `payment_means(PaymentMeans::new(code))`). The `InvoiceData` getters of the
  same names stay, and serialized invoice data with aggregate amounts still
  loads.
- Invoice values: `invoice::InvoiceTotalsData` (use `InvoiceTotals`),
  `PaymentMeans::instruction_note` (use `with_instruction_note`),
  `Adjustment::allowance_charge_reason`, `allowance_charge_reason_code`,
  `calculation_base_amount` and `calculation_percentage` (use `reason`,
  `reason_code`, `base_amount` and `percentage`), and
  `VatBreakdown::without_vat_rate` (use `omit_vat_rate`).
- Validation and hashing: `validation::validate_xml_invoice_from_str`,
  `validate_xml_invoice_report_from_str` and `validate_zatca_invoice_from_str`
  (use `Validator::validate_xml`, `validate_xml_report` and `validate_zatca`);
  `sign::invoice_hash_base64_from_xml_str` (use `invoice::hash_xml`); and
  `FinalizedInvoice::hash_base64` / `SignedInvoice::hash_base64` (use `hash`,
  which returns an `InvoiceHash`).

## [0.6.0] - 2026-11-18

The breaking 0.6 release. `fatoora-core` renames items for consistency and
replaces strings and Booleans with typed values; the `fatoora-ffi` bridge and
every language package now mirror the core names and take enums instead of
integer codes, and validation no longer takes a `Config`. Upgrade language
packages and native libraries together.

Migration guides with before-and-after code for each language:
[overview](https://docs.fatoority.com/guides/migrate-0.6/),
[Rust](https://docs.fatoority.com/guides/migrate-0.6/rust/),
[Python](https://docs.fatoority.com/guides/migrate-0.6/python/),
[JavaScript/TypeScript](https://docs.fatoority.com/guides/migrate-0.6/javascript/),
[Kotlin/Java](https://docs.fatoority.com/guides/migrate-0.6/kotlin/),
[C#/.NET](https://docs.fatoority.com/guides/migrate-0.6/csharp/) and
[C/C++](https://docs.fatoority.com/guides/migrate-0.6/c/).

In `fatoora-core`, renamed items keep their old names as deprecated aliases
until 0.7 (see "Deprecated"); items whose type, signature or fields changed in
place have no deprecation period (see "Changed (breaking)"). The Python package
keeps its renamed 0.5 names as deprecated aliases until 0.7, and the Kotlin/Java
facade keeps its 0.5 getter functions and `String` amount overloads, and the
JavaScript package keeps `new_` as an alias of `create` (its getters becoming
properties is a hard break, as in Python). The JavaScript,
Kotlin/Java and C# packages also keep deprecated aliases for the bridge members
renamed to the core names (`percentage` → `percent_of`,
`with_supplied_amount` → `supplied_amount`, `percent` → `percentage`,
`without_vat_rate` → `omit_vat_rate`), and C# keeps `New` as an alias of
`Create`. The C facade changes without aliases.

### Rust (`fatoora-core`)

#### Added

- `api::{ComplianceCsid, ProductionCsid}` aliases for
  `CsidCredentials<Compliance>` and `CsidCredentials<Production>`.
- `api::ClearanceMode::{Enabled, Disabled}` for the `Clearance-Status`
  request header (`1`/`0`), with `From<bool>`.
- `api::{ReportingStatus, ClearanceStatus, ValidationStatus, MessageType}`:
  non-exhaustive enums for ZATCA response status values. Each has an
  `Other(String)` variant that keeps unrecognized values verbatim, `as_str()`,
  `Display`, infallible `From<&str>`/`From<String>`/`FromStr`, and serializes
  as the plain wire string.
- `impl FromStr for CsrProperties` (`text.parse::<CsrProperties>()`) and
  `CsrProperties::from_file(path)`.
- `invoice::validation::Validator`, the single entry point for local
  validation: `validate_xml(xml)` (UBL XSD only), `validate_xml_report(xml)`
  (XSD findings as a `ValidationReport`) and `validate_zatca(xml, options)`
  (full local ZATCA profile). It needs no `Config`; validation does not depend
  on the ZATCA environment.
- `invoice::hash_xml(xml)`, returning the invoice hash as an `InvoiceHash`
  (the base64 SHA-256 of the canonicalised invoice, ready to pass on as the
  next invoice's previous invoice hash).
- `FinalizedInvoice::hash()` and `SignedInvoice::hash()`, returning
  `Result<InvoiceHash, SigningError>`: the same digest as `hash_base64()`, typed
  so it can be passed straight to `InvoiceBuilder::previous_invoice_hash` or
  `ZatcaValidationOptions::previous_invoice_hash`.
- `AsRef<str>`, `FromStr`, `TryFrom<&str>` and `TryFrom<String>` for
  `InvoiceTime`, matching the other validated wrappers.
- `invoice::AdjustmentKind::{Allowance, Charge}`, with `is_charge()`,
  `is_allowance()`, `from_charge_indicator(bool)` and `charge_indicator()`
  mapping to UBL `cbc:ChargeIndicator`.
- `DocumentAdjustment::{omit_vat_rate(), has_vat_rate(),
  with_tax_category_count(count)}` and `impl From<AllowanceCharge> for
  DocumentAdjustment`, so `InvoiceBuilder::allowance`/`charge` can express
  everything `adjustment(AllowanceCharge)` could.
- `omit_vat_rate()` as the one name for omitting a VAT rate: new on
  `LineItem` (returns `Result`), `VatBreakdown` (returns `Result`; replaces
  `without_vat_rate`), `AllowanceCharge` (returns `Self`; `build` checks it)
  and `DocumentAdjustment`. `AllowanceCharge` and `DocumentAdjustment` gain
  `has_vat_rate()`, which `LineItem` and `VatBreakdown` already had.
- Constructors, getters and `with_*` setters for the invoice value types whose
  fields are now private (see "Changed (breaking)"):
  - `Address::new(country_code, city, street, building_number, postal_code)`
    with `with_province`, `with_additional_street`, `with_additional_number`
    and `with_district`. The existing getters are unchanged.
  - `TaxExemption::{with_reason_code, with_reason}` (start from
    `TaxExemption::default()`) and getters `reason_code()`, `reason()`.
  - `PaymentMeans::{with_instruction_note, with_payee_financial_account}` and
    getters `code()`, `instruction_notes()`, `payee_financial_account()`.
  - `PaymentAccount::{with_payment_account_identifier, with_payment_terms}`
    (start from `PaymentAccount::default()`) and getters of the same names
    without `with_`.
  - `Delivery::{with_actual_date, with_latest_date}` (start from
    `Delivery::default()`) and getters `actual_date()`, `latest_date()`.
  - `AllowanceCharge::new(kind, amount, category, rate)` with
    `with_base_amount`, `with_percentage`, `with_reason`, `with_reason_code`,
    `with_exemption` and `with_tax_category_count`, and getters `kind()`,
    `amount()`, `base_amount()`, `percentage()`, `reason()`, `reason_code()`,
    `category()`, `rate()`, `exemption()`, `tax_category_count()`.
  - `DocumentAdjustment::{with_reason, with_reason_code, with_exemption}` and
    getters `adjustment()`, `reason()`, `reason_code()`, `category()`,
    `rate()`, `exemption()`, `tax_category_count()`.
  - `LineAdjustment::{kind(), adjustment()}`.
  - `Adjustment::{reason(), reason_code(), base_amount(), percentage()}`
    getters.

- `api::ZatcaClient::with_timeout(config, Duration)`: a client whose requests
  fail after the given timeout, covering connect through reading the response
  body. A timeout surfaces as `ZatcaError::Http` (`ZatcaError::ResponseRead`
  if it expires mid-body), both `ErrorKind::Network`; a zero timeout is
  rejected. `ZatcaClient::new` is unchanged and applies no timeout. Ignored on
  `wasm32`, where the client does not send requests.

#### Changed

- `InvoiceBuilder::allowance(DocumentAdjustment)` and
  `charge(DocumentAdjustment)` are the one way to add document-level
  allowances and charges, and `payment_means(PaymentMeans::new(code))` the way
  to add payment means; the alternatives are deprecated (see "Deprecated").
- A `DocumentAdjustment` whose calculation is invalid is now reported with the
  other adjustment checks in `InvoiceBuilder::build`, after missing or invalid
  fields, instead of before them.

#### Changed (breaking)

These change a type, field, parameter or return type in place, so they have no
deprecation period.

**Configuration**

- `config::Environment` is `#[non_exhaustive]`. Matches on it outside
  `fatoora-core` need a wildcard arm. `VatCategory`, `InvoiceType` and
  `InvoiceSubType` stay exhaustive.

**API client and responses**

- `ZatcaClient::{report_simplified_invoice, clear_standard_invoice,
  prepare_report_simplified_invoice, prepare_clear_standard_invoice}` take
  `clearance_mode: ClearanceMode` instead of `clearance_status: bool`. Pass
  `ClearanceMode::Enabled` for `true` and `ClearanceMode::Disabled` for
  `false`, or `ClearanceMode::from(flag)`. The bindings take a `ClearanceMode`
  too, except Python, which keeps a Boolean (see "Bindings").
- Response status getters return the new enums instead of `Option<&str>`:
  `ValidationResponse::reporting_status()` returns `Option<&ReportingStatus>`,
  `ValidationResponse::clearance_status()` returns `Option<&ClearanceStatus>`,
  `ValidationResults::status()` and `ValidationMessage::status()` return
  `Option<&ValidationStatus>`, and `ValidationMessage::message_type()` returns
  `Option<&MessageType>`. Compare with the enum
  (`Some(&ReportingStatus::Reported)`) or call `.map(ReportingStatus::as_str)`
  for the previous `Option<&str>`. The JSON wire format is unchanged.

**Typed invoice inputs and getters**

Parse strings with the wrapper's `parse`, `FromStr` or `TryFrom<&str>` (for
example `"SAR".parse()?`). Invalid values now fail when the wrapper is parsed
instead of in `InvoiceBuilder::build`, so `build` reports the UUID, issue
date-time, currency and previous invoice hash only as missing (never as empty
or invalid). Every wrapper has `as_str()` and `Display`.

- `InvoiceBuilder::uuid` takes `InvoiceUuid`, `issue_datetime` takes
  `InvoiceTimestamp`, `currency` takes `CurrencyCode` and
  `previous_invoice_hash` takes `InvoiceHash` (each was `impl Into<String>`).
- `InvoiceData::uuid` returns `&InvoiceUuid`, `issue_date` returns
  `InvoiceDate`, `issue_time` returns `InvoiceTime` and
  `previous_invoice_hash` returns `&InvoiceHash` (each was `&str`).
- `SignedInvoice::uuid` returns `&InvoiceUuid`, and
  `SignedInvoice::invoice_hash` and `SignedProperties::invoice_hash` return
  `&InvoiceHash` (each was `&str`).
- `SignedProperties::signing_time` returns `&InvoiceTimestamp` (was `&str`).
  Its `as_str()` is the canonical `YYYY-MM-DDTHH:MM:SSZ` form; the XAdES
  `SigningTime` element in the XML is unchanged and still has no `Z`.
- `OriginalInvoiceRef::with_uuid` takes `InvoiceUuid` (was
  `impl Into<String>`), and `OriginalInvoiceRef::uuid` returns
  `Option<&InvoiceUuid>` (was `Option<&str>`).
- `Seller::new` (`Party::<SellerRole>::new`) takes `name: impl Into<String>`
  and `vat_id: VatId` (was `String` and `impl Into<String>`) and returns
  `Seller` (was `Result<Seller, InvoiceError>`); it can no longer fail.
- `Buyer::new` (`Party::<BuyerRole>::new`) takes `name: impl Into<String>` and
  `vat_id: Option<VatId>` (was `String` and `Option<String>`) and returns
  `Buyer` (was `Result<Buyer, InvoiceError>`).
- `ZatcaValidationOptions::previous_invoice_hash` is `Option<InvoiceHash>`
  (was `Option<String>`). Set it from `hash()`, `hash_xml` or `text.parse()?`.
  Blank text is now rejected when the hash is parsed (or when options are
  deserialized), and surrounding whitespace is trimmed; `validate_zatca` still
  rejects a malformed digest as `InvalidContext`. The CLI
  `--previous-invoice-hash` flag and the FFI options JSON are unchanged; a
  blank value now fails as invalid options (CLI exit code 3, FFI error code 1,
  as before).

**Private fields**

- The fields of `Address`, `TaxExemption`, `PaymentMeans`, `PaymentAccount`,
  `Delivery`, `AllowanceCharge`, `DocumentAdjustment` and `LineAdjustment` are
  private, so these types can gain fields without a breaking change. Replace
  struct literals with the constructor and `with_*` setters, and field reads
  with the getter of the same name (`address.city` becomes `address.city()`):
  - `Address { .. }`: `Address::new(country_code, city, street,
    building_number, postal_code)` plus `with_province`,
    `with_additional_street`, `with_additional_number`, `with_district`.
  - `TaxExemption { reason_code, reason }`:
    `TaxExemption::default().with_reason_code(code).with_reason(text)`.
  - `PaymentAccount { .. }`: `PaymentAccount::default()
    .with_payment_account_identifier(id).with_payment_terms(terms)`.
  - `Delivery { actual_date, latest_date }`:
    `Delivery::default().with_actual_date(date).with_latest_date(date)`.
  - `AllowanceCharge { .. }`: `AllowanceCharge::new(kind, amount, category,
    rate)` plus `with_base_amount`, `with_percentage`, `with_reason`,
    `with_reason_code`, `with_exemption`, `with_tax_category_count` and
    `omit_vat_rate()`; read `omitted_rate` as `!has_vat_rate()`.
  - `DocumentAdjustment { .. }`: `DocumentAdjustment::new(adjustment,
    category, rate)` plus the `with_*` setters and `omit_vat_rate()`.
  - `LineAdjustment` is read-only: use `kind()` and `adjustment()`. Line
    adjustments are added with `LineItemBuilder::allowance`/`charge`.
  - `PaymentMeans::payment_means_type_code` is read with `code()`, matching
    `PaymentMeans::new(code)`; `instruction_notes` and
    `payee_financial_account` with the getters of the same names.

  Apart from the `charge` to `kind` change below, the serde field names and
  formats of these types are unchanged.

**Allowance or charge kind**

- `AllowanceCharge::charge: bool` and `LineAdjustment::charge: bool` are
  replaced by `kind: AdjustmentKind`, read with `kind()`.
  `AdjustmentKind::from_charge_indicator(flag)` converts a `charge` flag
  (`true` is `Charge`). Serialization now writes a `"kind"` field with the
  values `"allowance"`/`"charge"` instead of `"charge"`, so serialized
  `InvoiceData` and `LineItem` values change too. JSON written by 0.5 is still
  accepted on read: a legacy `"charge": true`/`false` deserializes as `Charge`/
  `Allowance`. Input with both fields is accepted if they agree and rejected
  if they conflict.

**Setters renamed so the plain name is the getter**

- `Adjustment::{reason, reason_code, base_amount}(value)` are renamed
  `with_reason`, `with_reason_code`, `with_base_amount`;
  `DocumentAdjustment::{reason, reason_code, exemption}(value)` are renamed
  `with_reason`, `with_reason_code`, `with_exemption`. `reason()`,
  `reason_code()`, `base_amount()` and `exemption()` are now getters.
- `Adjustment::percentage(base, percent)` is renamed
  `Adjustment::percent_of(base, percent)`; `percentage()` is now the getter.
  The FFI follows (see "Bindings").
- `PaymentMeans::payee_financial_account(account)` is renamed
  `with_payee_financial_account(account)`; `payee_financial_account()` is now
  the getter.

#### Deprecated

Each of these still compiles with a deprecation warning, forwards to its
replacement, and will be removed in 0.7.

- `config::EnvironmentType`: renamed `config::Environment`.
- `ZatcaClient::post_csr_for_ccsid` and `post_ccsid_for_pcsid`: renamed
  `request_compliance_csid` and `request_production_csid`.
  `prepare_post_csr_for_ccsid` and `prepare_post_ccsid_for_pcsid`: renamed
  `prepare_request_compliance_csid` and `prepare_request_production_csid`.
- `CsrProperties::{from_properties_str, parse_csr_config}`: use
  `text.parse::<CsrProperties>()`. `CsrProperties::parse_csr_config_file`: use
  `CsrProperties::from_file`.
- `invoice::InvoiceTotalsData`: renamed `invoice::InvoiceTotals`.
- `invoice::validation::{validate_xml_invoice_from_str,
  validate_xml_invoice_report_from_str, validate_zatca_invoice_from_str}`: use
  `Validator::{validate_xml, validate_xml_report, validate_zatca}`. They
  ignore their `Config` argument.
- `invoice::sign::invoice_hash_base64_from_xml_str`: use `invoice::hash_xml`.
  It still returns `String`.
- `FinalizedInvoice::hash_base64()` and `SignedInvoice::hash_base64()`: use
  `hash()`, which returns `InvoiceHash`. They still return `String`.
- `InvoiceBuilder::adjustment(AllowanceCharge)`: forwards to `allowance` or
  `charge` by the entry's `kind`. Use
  `allowance(DocumentAdjustment::from(entry))` (or `charge`).
- `InvoiceBuilder::{invoice_level_discount, invoice_level_charge,
  allowance_reason}`: these keep their behaviour. Each call replaces one
  aggregate allowance or charge in the document VAT category, with a rate
  taken from the first line in that category, and the reason is shared by
  both (on its own it emits a zero-amount allowance). Use
  `allowance`/`charge` with `DocumentAdjustment::new(Adjustment::amount(amount),
  category, rate).with_reason(reason)`; each call appends an entry with an
  explicit VAT category and rate.
- `InvoiceBuilder::payment_means_code(code)`: keeps its behaviour of changing
  the code of the first payment means, creating one if there is none. Use
  `payment_means(PaymentMeans::new(code))`, which appends.
- `VatBreakdown::without_vat_rate()`: renamed `omit_vat_rate()`.
- `Adjustment::{allowance_charge_reason, allowance_charge_reason_code,
  calculation_base_amount, calculation_percentage}`: renamed `reason()`,
  `reason_code()`, `base_amount()` and `percentage()`.
- `PaymentMeans::instruction_note(text)`: renamed
  `with_instruction_note(text)`.

The FFI builder setters
`set_payment_means_code`, `invoice_level_charge`, `invoice_level_discount`,
`allowance_reason` and `set_allowance` are replaced without aliases (see
"Bindings").

#### Removed

- The `invoice::LineItems` alias (`Vec<LineItem>`). Use `Vec<LineItem>`.
- `InvoiceData::note()`, which returned only the first note. Use
  `notes().first()`.
- `OriginalInvoiceRef::with_issue_date_str`. Use
  `with_issue_date(InvoiceDate::parse(date)?)` or `with_issue_date(date.parse()?)`.

### Bindings (`fatoora-ffi`, C and C++)

Changes to the bridge that every generated binding inherits. The language
sections below list what each package adds on top.

#### Added

- FFI enums `Environment`, `InvoiceTypeKind`, `InvoiceSubType`,
  `VatCategory`, `AdjustmentKind`, `ClearanceMode` and `InvoiceFlag` (named
  bits of the flag mask, with `bit()` where the generator emits enum methods),
  plus `ReportingStatus`, `ClearanceStatus`, `ValidationStatus` and
  `MessageType`, each with an `Unknown` variant for values this binding
  version does not recognise. New `ValidationResponse::{reporting_status_raw,
  clearance_status_raw}`, `ValidationResults::status_raw` and
  `ValidationMessage::{message_type_raw, status_raw}` return the exact wire
  text. The C facade (`fatoora.h`) adds typed enums (`FatooraEnvironment`,
  `FatooraVatCategory`, ...) and the constants `FATOORA_ADJUSTMENT_*`,
  `FATOORA_CLEARANCE_{DISABLED,ENABLED}`, `FATOORA_REPORTING_STATUS_*`,
  `FATOORA_CLEARANCE_STATUS_*`, `FATOORA_VALIDATION_STATUS_*` and
  `FATOORA_MESSAGE_TYPE_*`.
- `ZatcaValidationReport` mirrors the core report, with
  `schema_version`, `profile`, `evaluated_at`, `is_complete`, `has_errors`,
  `is_valid`, `stages_len`, `stage(index)` and `to_json`. Its
  `ZatcaStageReport` (`stage`, `status`, `provenance`, evaluated assertions and
  findings), `ZatcaRuleSource` and `ZatcaFinding` (`assertion_site`, `layer`,
  `code`, `severity`, `message`, `location_kind`, `location_path`,
  `location_line`, `location_column`) come with the enums `ZatcaStage`,
  `ZatcaStageStatus`, `ValidationLayer`, `Severity` and
  `ValidationLocationKind`. The C facade adds the matching
  `FATOORA_ZATCA_STAGE_*`, `FATOORA_ZATCA_STAGE_STATUS_*`,
  `FATOORA_VALIDATION_LAYER_*`, `FATOORA_SEVERITY_*` and `FATOORA_LOCATION_*`
  constants.

- `ZatcaClient.with_timeout(config, timeout_ms: u32)` in the
  FFI, named like the core `ZatcaClient::with_timeout` (`withTimeout` in
  Kotlin/Java, `WithTimeout` in C#, `fatoora_zatca_client_with_timeout` in C).
  Not exposed to JavaScript, whose client sends through the caller's `fetch`.

#### Changed (breaking)

The Python `fatoora` package changes are listed under "Python". Enum discriminants for
`Environment`, `InvoiceTypeKind`, `InvoiceSubType` and `VatCategory` equal the
old integer codes.

- Environment, invoice kind/subtype, VAT category and adjustment kind
  arguments take enums instead of integer codes or Booleans: `Config::new`,
  `CsidCompliance::create`, `CsidProduction::create`, `CsrProperties::build`,
  `InvoiceBuilder::{new, new_with_reference, set_vat_category, add_line_item,
  set_tax_exemption, add_document_adjustment}`,
  `LineItemBuilder::set_vat_category`, `InvoiceLineItem::new`,
  `VatBreakdown::new` and `AllowanceCharge::new`. Out-of-range-code errors are
  gone. The matching getters (`Config::env`, the credential `env()`,
  `InvoiceData::{vat_category, invoice_type_kind, invoice_sub_type}`, and the
  `category()`/`vat_category()` getters) return enums; `env()` is now fallible
  (code 9 only if the core gains an environment the bindings do not know).
- `AllowanceCharge::is_charge()` is replaced by `kind() -> AdjustmentKind`,
  and `InvoiceLineItem::adjustment_is_charge(index)` by
  `adjustment_kind(index)`. In C: `fatoora_allowance_charge_kind` and
  `fatoora_invoice_line_item_adjustment_kind`.
- `ZatcaClient::{report_simplified_invoice, clear_standard_invoice}` and their
  `prepare_*` forms take `clearance_mode: ClearanceMode` instead of
  `clearance_status: bool`. The JavaScript async `ZatcaClient` still accepts a
  Boolean (or a `ClearanceMode`). The Kotlin `reportSimplifiedInvoiceAsync` and
  `clearStandardInvoiceAsync` helpers take a `ClearanceMode` (see "Kotlin/Java
  facade" below).
- `ValidationResponse::{reporting_status, clearance_status}`,
  `ValidationResults::status` and `ValidationMessage::{message_type, status}`
  return an optional enum instead of optional text; use the new `*_raw`
  getters for the previous text.
- .NET names the environment enum `ZatcaEnvironment` to avoid clashing with
  `System.Environment`, and the flag mask is built by casting
  (`(byte)InvoiceFlag.Export`).
- C facade: enum parameters and outputs use the typed enums (for example
  `FatooraVatCategory *out` instead of `uint8_t *out`); undeclared values
  return `FATOORA_INVALID_INPUT`. Optional status getters write the enum plus
  `bool *has_value`, and the `_raw` variants write `char **` (`NULL` when
  absent). The raw Diplomat headers do not validate enum values.
- The bridge mirrors the core names, with no aliases. Builders use `set_*` to
  replace a value and `add_*` to append one; getters keep plain names:

  | 0.5 | 0.6 |
  | --- | --- |
  | `CsidCompliance`, `CsidProduction` | `ComplianceCsid`, `ProductionCsid` |
  | `ZatcaClient::post_csr_for_ccsid`, `post_ccsid_for_pcsid` | `request_compliance_csid`, `request_production_csid` |
  | `ZatcaClient::prepare_post_csr_for_ccsid`, `prepare_post_ccsid_for_pcsid` | `prepare_request_compliance_csid`, `prepare_request_production_csid` |
  | `Signer` | `InvoiceSigner` |
  | `CsrProperties::from_properties_str`, `parse_csr_config_file` | `from_str`, `from_file` |
  | `FinalizedInvoice::hash_base64`, `SignedInvoice::hash_base64` | `hash` (`SignedInvoice::invoice_hash` stays the stored digest) |
  | `InvoiceBuilder::flags(mask)` | `set_flags(mask)` |
  | `InvoiceData::flags_raw` | `flags` |
  | `InvoiceBuilder::set_tax_exemption` | `add_tax_exemption` (it appends) |
  | `InvoiceBuilder::add_document_adjustment(kind, ...)` | `add_allowance(...)` or `add_charge(...)` |
  | `InvoiceBuilder::{set_note(language, text), set_note_without_language(text)}` | `add_note(language or null, text)` |
  | `InvoiceBuilder::set_payment_means_code(code)` | `add_payment_means(PaymentMeans::new(code, null))` |
  | `InvoiceBuilder::{set_allowance, allowance_reason, invoice_level_charge, invoice_level_discount}` | `add_allowance`/`add_charge` with an `Adjustment` carrying the reason |
  | `Xml::validate(config, xml)` | `Xml::validate(xml)` |
  | `Xml::validate_zatca(config, xml, options_json)` returning JSON text | `Xml::validate_zatca(xml, options_json)` returning `ZatcaValidationReport` (`to_json()` gives the old text) |

  | `ZatcaClient::create`, `CsidCompliance::create`, `CsidProduction::create` | `ZatcaClient::new`, `ComplianceCsid::new`, `ProductionCsid::new`, so every constructor is `new` like the core (C: `fatoora_zatca_client_new`, `fatoora_compliance_csid_new`, `fatoora_production_csid_new`) |
  | `Adjustment::percentage(base, percent)` | `percent_of(base, percent)` |
  | `Adjustment::with_supplied_amount(value)` | `supplied_amount(value)` |
  | `Adjustment::percent()` | `percentage()` (the old constructor name is now the getter) |
  | `AllowanceCharge::without_vat_rate`, `VatBreakdown::without_vat_rate` | `omit_vat_rate` |

  Validation drops the `Config` argument because the core `Validator` is
  stateless. The bridge no longer calls deprecated core APIs. The credential
  `request_id()` getters return null when absent in every language.
- JavaScript: the async `ZatcaClient` methods `postCsrForCcsid` and
  `postCcsidForPcsid` are renamed `requestComplianceCsid` and
  `requestProductionCsid`, and `fatoority/raw` `Xml.validateZatca(xml,
  optionsJson)` returns a `ZatcaValidationReport` (the package root returns a
  plain object; see "JavaScript / TypeScript"). Kotlin: the coroutine helpers are renamed
  `requestComplianceCsidAsync` and `requestProductionCsidAsync`. C facade:
  functions follow the renames (`fatoora_compliance_csid_*`,
  `fatoora_production_csid_*`, `fatoora_invoice_signer_*`,
  `fatoora_zatca_client_request_compliance_csid`, `fatoora_*_invoice_hash`,
  `fatoora_xml_validate(xml, ...)`, `fatoora_xml_validate_zatca(xml, options,
  FatooraZatcaValidationReport **out, ...)`).

### Python (`fatoority` on PyPI)

The `fatoora` package gets a Pythonic surface that mirrors the core names.
Getters becoming properties is a hard break; every renamed 0.5 name keeps
working until 0.7 and emits a `DeprecationWarning` naming its replacement (see [the Python reference](https://docs.fatoority.com/reference/bindings/python/#migrate-from-05)).

- Breaking: getters are read-only properties (`invoice.id`, `address.city`,
  `response.reporting_status`, `config.env`, `signed.qr_code`, ...). Drop the
  parentheses; `invoice.id()` now fails with `TypeError` like calling the
  value.
- `InvoiceBuilder(invoice_type, invoice_subtype, *, id=, uuid=, issue_datetime=,
  currency=, previous_invoice_hash=, invoice_counter=, vat_category=, flags=,
  original_invoice=, original_invoice_reason=)` constructor; `InvoiceBuilder.new`
  and `new_with_reference` are deprecated. Builder and `LineItemBuilder` setters
  return the builder so calls chain; `LineItemBuilder()` replaces
  `LineItemBuilder.new()`.
- Timestamps accept a timezone-aware `datetime` (naive ones raise `ValueError`),
  dates accept `datetime.date`, and UUIDs accept `uuid.UUID`; strings are still
  accepted. `issue_datetime`, `signing_time` and `ZatcaReport.evaluated_at` return
  aware UTC `datetime`s, and `issue_date`, the delivery dates and
  `OriginalInvoiceRef.issue_date` return `date`s. `uuid` stays `str` because core
  does not check UUID syntax.
- Keyword-only arguments, warned about in 0.5.1, are now enforced (positional
  use raises `TypeError`): everything after `name` in `set_seller`/`set_buyer`,
  every `CsrProperties.new` field, `language` in `add_note(text, *, language=None)`,
  and `clearance_status`/`accept_language` in `report_simplified_invoice` and
  `clear_standard_invoice`. `set_seller`/`set_buyer` also accept
  `address=Address.new(...)`.
- `validate_zatca(xml, *, previous_invoice_hash=None, evaluated_at=None)` returns
  a frozen `ZatcaReport` dataclass (stages, statuses, findings with severity,
  layer, code, message and location; `to_json()`/`to_dict()`), with string enums
  `ZatcaStage`, `ZatcaStageStatus`, `Severity`, `ValidationLayer` and
  `ValidationLocationKind`. `validate_xml(xml)` and `hash_xml(xml)` replace
  `validate_xml_invoice_from_str(config, xml)` and
  `invoice_hash_base64_from_xml_str(xml)`; `validate_zatca_invoice_from_str`
  (returning the dictionary) is deprecated.
- `FinalizedInvoice.from_xml`/`from_file` and `SignedInvoice.from_xml`/`from_file`
  replace the deprecated `parse_*_invoice_xml[_file]` functions.
- Renamed with deprecated aliases: `Signer` → `InvoiceSigner`,
  `CsidCompliance`/`CsidProduction` → `ComplianceCsid`/`ProductionCsid`,
  `ZatcaClient.post_csr_for_ccsid`/`post_ccsid_for_pcsid` →
  `request_compliance_csid`/`request_production_csid`,
  `CsrProperties.from_properties_str`/`parse_csr_config`/`parse_csr_config_file`
  → `from_str`/`from_file`, `hash_base64()` → `hash()` (`invoice_hash` is the
  stored hash), `to_xml()`/`into_xml()` → the `xml` property, `builder.flags()` →
  `set_flags()`, `set_tax_exemption` → `add_tax_exemption`,
  `set_note`/`set_note_without_language` → `add_note`,
  `set_payment_means_code(code)` → `add_payment_means(PaymentMeans.new(code))`.
- `InvoiceBuilder.add_allowance(value, category, rate, exemption=...)` and
  `add_charge(...)` are the document-level adjustment setters;
  `add_document_adjustment(charge, ...)` and the aggregate setters
  (`set_allowance`, `invoice_level_discount`, `invoice_level_charge`,
  `allowance_reason`) are deprecated.
- Invoice data models are frozen dataclasses: `Party`, `Address`,
  `InvoiceNote`, `OriginalInvoiceRef`, `PaymentMeans`, `Adjustment` and the new
  `Identifier(value, scheme)`, alongside `InvoiceLineItem`, `InvoiceTotals`,
  `VatBreakdown` and `TaxExemption`. Values read from an invoice compare with
  `==` and can be passed back to the builder; they need no `close()`. Construct
  them directly: `Address(...)`, `PaymentMeans(code, instruction_notes=(), ...)`
  and `OriginalInvoiceRef(id, uuid=, issue_date=, issue_time=, document_type_code=)`
  replace the deprecated `.new(...)` classmethods, and `close()`/`with` on these
  values are deprecated no-ops.
- `FinalizedInvoice`/`SignedInvoice` read their data once, on first access, and
  cache it instead of crossing the FFI on every property read. Signing (or the
  deprecated `into_xml()`) still consumes the invoice and drops the cache.
- `Adjustment` is the one allowance/charge model: `Adjustment(amount, reason=,
  reason_code=, ...)` or `Adjustment.percent_of(base, percent, ...)` for line
  and document adjustments, and the type of `invoice.adjustments`, with
  `charge`, `category`, `rate`, `exemption`, `tax_category_count` and
  `omitted_rate` so `add_adjustment(entry)` round-trips them.
  `add_allowance`/`add_charge` take the category, rate and exemption from the
  value when omitted. Deprecated: `AllowanceCharge(charge, amount, category,
  rate, ...)` (returns an `Adjustment`), `Adjustment.amount(value)` →
  `Adjustment(value)`, `Adjustment.percentage(base, percent)` →
  `Adjustment.percent_of(base, percent)`, `.value` → `.amount`, `.percent` →
  `.percentage`, `with_supplied_amount(value)` → `percent_of(..., amount=value)`.
- `DocumentReference` is merged into `OriginalInvoiceRef` (deprecated alias).
  Breaking: line `document_references` entries have `issue_date` as a
  `datetime.date`, like `OriginalInvoiceRef`.
- `Identifier(value, scheme)` replaces the `VatId`/`OtherId` handles (deprecated
  aliases) and the `(value, scheme)` tuples of `item_seller_identifier`,
  `item_buyer_identifier` and `item_standard_identifier`; unpacking or indexing
  an `Identifier`, or passing a tuple, warns. `party.vat_id.value` and
  `party.identifiers[0].value` keep working. Breaking: `party.identifiers` and
  `payment_means.instruction_notes` are tuples instead of lists.
- `InvoiceLineItem` input no longer requires `total_amount` and `vat_amount`:
  `add_item` lets the core calculate them when `None` and checks them when given.
  Its constructor keeps the 0.5 positional order; pass `vat_rate` and
  `vat_category` by keyword when leaving the amounts out.
- `InvoiceBuilder(..., seller=Party(...), buyer=Party(...))`, and `set_seller`/
  `set_buyer` accept a `Party`. Identifier setters accept an `Identifier`.
- Invalid decimal text in `Adjustment` and `InvoiceLineItem` fields, and in the
  deprecated aggregate setters, raises `ValueError` on construction instead of
  `InvalidInputError` when the value reaches the native library.
- `tests/public_api.json` records the public surface, including the deprecated
  names, and is regenerated with `python tests/test_public_api.py --update`.
- `ZatcaClient(config, *, timeout=None)`: `timeout` is seconds (`int`/`float`)
  or a `datetime.timedelta` bounding each request; exceeding it raises
  `NetworkError`. There is no async client in 0.6.
- `ClearanceMode` (`DISABLED`, `ENABLED`) sets the `Clearance-Status` header, as
  in the other languages: `report_simplified_invoice` and
  `clear_standard_invoice` take a required keyword-only
  `clearance=ClearanceMode...`. The 0.5 `clearance_status=False`/`True` is
  deprecated and still works; passing both raises `TypeError`.
- Response statuses are `str` enums instead of plain strings:
  `reporting_status` is a `ReportingStatus` (`REPORTED`, `NOT_REPORTED`),
  `clearance_status` a `ClearanceStatus` (`CLEARED`, `NOT_CLEARED`),
  `ValidationResults.status` and `ValidationMessage.status` a `ValidationStatus`
  (`PASS`, `WARNING`, `ERROR`) and `ValidationMessage.message_type` a
  `MessageType` (`INFO`, `WARNING`, `ERROR`). Their values are the wire text,
  so `== "REPORTED"`, `str()`, f-strings and `json.dumps` behave as before. A
  value this version does not recognize is kept verbatim, like the core's
  `Other(String)`: an instance of the enum named `OTHER` whose `value` is the
  text ZATCA sent. `qr_seller_status`/`qr_buyer_status` stay strings, as in the
  core.
- Every type you create directly has a constructor: `CsrProperties(*, ...)`,
  `ComplianceCsid(env, *, token, secret, request_id=None)` and
  `ProductionCsid(...)` join `Config`, `ZatcaClient`, the builders and the
  invoice dataclasses. Deprecated: `CsrProperties.new(*, ...)` and
  `ComplianceCsid.new`/`ProductionCsid.new`, which keep their 0.5 positional
  order `(env, token, secret, request_id=None)`; the keyword-only constructor
  arguments remove the mismatch with the core's `(env, request_id, token,
  secret)` order. Types obtained from data or another call
  (`InvoiceSigner`, `SigningKey`, `Csr`, invoices and responses) keep named
  factories, and calling the class raises a `TypeError` that names them.

- The deprecated 0.5 names (`Signer`, `CsidCompliance`, `post_csr_for_ccsid`,
  `hash_base64`, `set_payment_means_code`, the aggregate adjustment setters and
  the `validate_zatca_invoice_from_str` dictionary) work on top of the renamed
  bridge. `ComplianceCsid.request_id` and `ProductionCsid.request_id`
  are `None` instead of `""` when absent. `set_payment_means_code` appends
  a payment means instead of rewriting the code of an existing one, and the
  aggregate setters (`set_allowance`, `invoice_level_discount`,
  `invoice_level_charge`, `allowance_reason`) are applied by `build()` as one
  allowance and one charge in the document VAT category; `build()` raises
  `ValueError` when that category has more than one line VAT rate or when they
  are combined with `add_document_adjustment`/`add_adjustment`.

### JavaScript / TypeScript (`fatoority` on npm)

Changed (breaking):

- Value getters are read-only properties, as in the Kotlin and C# packages:
  every zero-argument getter returning a string, number, `bigint`, boolean,
  enum or optional text (`invoice.xml`, `data.id`, `address.city`,
  `totals.taxAmount`, `response.reportingStatus`, `request.url`,
  `InvoiceFlag.Export.bit`). Members that take arguments, return another
  native object (`invoice.data()`, `response.validationResults()`), convert or
  consume (`toPem()`, `intoXml()`, `withReason(...)`) stay methods, as do
  `len()`, `isEmpty()` and `BindingError`'s methods. This applies to
  `fatoority/raw` too, whose declarations are now `raw.d.mts`. There is no
  deprecation period: a property cannot also be called, so `invoice.xml()`
  throws `TypeError` and TypeScript reports each call.
- Optional text results are `string | null` instead of a `Text` object, and
  methods with arguments that returned `Text` (`instructionNoteAt(i)`,
  `evaluatedAssertion(i)`) return `string`. Amounts remain exact decimal
  strings.
- Library failures throw one `FatooraError` subclass per error code
  (`InvalidInputError`, `ValidationError`, `ParseError`, `XmlError`,
  `CryptoError`, `IoError`, `NetworkError`, `UnauthorizedError`,
  `InternalError`, `ApiError`) instead of a plain `Error`. Each has `code`,
  `kind`, `details` (parsed JSON) and `message` (the library message instead
  of `"BindingError: [object Object]"`). `cause` is still the `BindingError`,
  so `error.cause.code()` keeps working. `fatoority/raw` throws the same
  classes. `TransportError` now extends `NetworkError`.
- `Xml.validateZatca` from the package root returns the report as a plain
  object (`isValid`, `isComplete`, `hasErrors`, `evaluatedAt`,
  `stages[].findings[]` with `severity`, `layer`, `code`, `message`,
  `assertionSite`, `location`) instead of the native `ZatcaValidationReport`.
  Enumerated fields are variant-name strings typed as unions. It also accepts
  `{ previousInvoiceHash, evaluatedAt }` as well as the options JSON string.
  The native report classes (`ZatcaValidationReport`, `ZatcaStageReport`,
  `ZatcaFinding`, `ZatcaRuleSource`) are no longer exported from the package
  root; `fatoority/raw` still returns the native report with `toJson()`.
- The root `Xml` class no longer extends the generated `Xml`. The root
  declarations make the internal constructors of `Xml` and the classes with
  `create` (below) `private`; they never produced a usable object.

Added:

- `create` for every generated `new_` constructor (`Config.create`,
  `Address.create`, `InvoiceBuilder.create`, `LineItemBuilder.create`,
  `PaymentMeans.create`, `TaxExemption.create`, `CsrProperties.create`,
  `OriginalInvoiceRef.create`, `AllowanceCharge.create`,
  `InvoiceLineItem.create`, `VatBreakdown.create`).
- String-literal union types for every enum, named with a `Value` suffix
  (`EnvironmentValue`, `SeverityValue`, `ZatcaStageValue`, ...), matching each
  enum's `.value`. `ZatcaClient` also accepts `'Enabled'`/`'Disabled'` for
  `clearanceStatus`.
- `ErrorCode`, `ErrorKind` and `errorClassForCode(code)`.

Fixed:

- Boolean getters such as `PaymentMeans.hasAccount` and
  `VatBreakdown.hasVatRate` returned `0`/`1`; they are booleans.

Deprecated:

- `new_` on the classes above: use `create`. It is marked `@deprecated` in
  the TypeScript declarations and still works without a runtime warning.
  `ComplianceCsid`, `ProductionCsid` and `fatoority/raw`'s `ZatcaClient` keep
  `create` and gain `new_` too, as the bridge now names every constructor
  `new`.
- `Adjustment.percentage(base, percent)`, `adjustment.withSuppliedAmount`,
  `adjustment.percent()` and `withoutVatRate()` on `AllowanceCharge` and
  `VatBreakdown`, renamed in the bridge: use `Adjustment.percentOf`,
  `suppliedAmount`, the `percentage` property and `omitVatRate()`; `percent()`
  still returns the 0.5 `Text`. Removed in 0.7.

### Kotlin / Java (`com.fatoora`)

Changed (breaking):

- Quantities, prices, rates and amounts are `java.math.BigDecimal` instead of
  `String`, for arguments (`InvoiceBuilder.addLineItem`, `setPrepaidAmount`,
  `addAllowance`/`addCharge`/`addTaxExemption` rates, `LineItemBuilder` price
  and amount setters, `Adjustment.amount`/`percentage`, and the
  `InvoiceLineItem`, `VatBreakdown` and `AllowanceCharge` factories) and for
  results (`InvoiceTotals`, `VatBreakdown`, `InvoiceLineItem`,
  `AllowanceCharge`, `Adjustment` and `InvoiceData` amounts). Conversion is
  exact: arguments are sent with `toPlainString()` and results parsed with
  `BigDecimal(String)`. Results are normalized (`10.50` reads back as `10.5`).
- Zero-argument getters that return copied values (text, amounts, numbers,
  enums, booleans) are Kotlin `val` properties, which Java sees as `getX()`;
  `isX` properties keep Java `isX()` and `hasX` properties keep Java `hasX()`.
  `InvoiceFlag.bit` is a property too. Getters that return native owners, such
  as `invoice.totals()` or `response.validationResults()`, and conversions
  (`toPem()`, `toJson()`, `intoXml()`, `asSlice()`) stay methods.
- Optional text results are `String?` instead of a closeable `Text`
  (for example `address.district`, `credentials.requestId`,
  `response.clearedInvoiceXml`). `PaymentMeans.instructionNoteAt(index)` and
  `ZatcaStageReport.evaluatedAssertion(index)` return `String`.
- Native failures throw a `BindingException` subclass per error code:
  `InvalidInputException`, `ValidationException`, `ParseException`,
  `XmlException`, `CryptoException`, `IoException`, `NetworkException`,
  `UnauthorizedException`, `InternalException` and `ApiException` (codes 1-10,
  matching the Python `FfiError` subclasses). `BindingException` is now `open`
  and gains `kind: ErrorKind?`; unknown codes still throw `BindingException`.
- `reportSimplifiedInvoiceAsync` and `clearStandardInvoiceAsync` take
  `clearanceMode: ClearanceMode`, like the blocking methods.
- Kotlin callers of `isValid()`, `isComplete()`, `hasErrors()`, `hasVatRate()`
  and the other `is*`/`has*` getters must drop the parentheses; there is no
  deprecated function because it would clash with the Java getter. Java
  callers are unaffected.

Deprecated, removed in 0.7:

- The getter functions replaced by properties, such as `address.city()` or
  `totals.taxAmount()`, keep their 0.5 return types (`String` amounts and
  closeable `Text?`) and carry `ReplaceWith` hints.
- The `String` overloads of the amount-taking methods listed above. They keep
  the 0.5 native parsing; use the `BigDecimal` overloads.
- The `Boolean` overloads of `reportSimplifiedInvoiceAsync` and
  `clearStandardInvoiceAsync`.
- The members renamed in the bridge to the core names:
  `Adjustment.percentage(base, percent)` (use `percentOf`),
  `withSuppliedAmount` (use `suppliedAmount`), `percent` (use the `percentage`
  property) and `withoutVatRate()` on `AllowanceCharge` and `VatBreakdown` (use
  `omitVatRate()`). `ZatcaClient.withTimeout` replaces
  `createWithTimeout`, which was never released.

### .NET (`Fatoority` NuGet package)

The C# API is now an idiomatic facade generated over the Diplomat output by
`scripts/generate_dotnet.py`. Native objects still implement `IDisposable`.
These are hard breaks with no deprecation period, because C# cannot overload a
method with a property or by return type:

- Quantities, prices, rates and amounts are `decimal` instead of strings, as
  arguments (`InvoiceBuilder.AddLineItem("Pen", 2m, "PCE", 50m, 15m, ...)`) and
  as results (`totals.PayableAmount` is `decimal`; optional amounts are
  `decimal?`). Values are converted exactly with the invariant culture. Returned
  values are normalized (`1.50` comes back as `1.5m`).
- Scalar getters are read-only properties: `address.City`, `config.Env`,
  `signed.Hash`, `signed.Xml`, `response.Outcome`, `report.IsValid`,
  `request.Method`, and so on. Getters that return a native-owned snapshot stay
  methods (`invoice.Data()`, `invoice.Totals()`, `response.ValidationResults()`,
  `request.Body()`), as do indexed accessors and `To*`/`Into*` conversions.
- Optional text getters return `string?` instead of a disposable `Text?`
  (`response.ReportingStatusRaw`, `credentials.RequestId`, `address.District`).
- `BindingErrorException` is replaced by one exception class per error code,
  matching the Python hierarchy: `FatooraException` > `FfiException` >
  `InvalidInputException`, `ValidationException`, `ParseException`,
  `XmlException`, `CryptoException`, `IoException`, `NetworkException`,
  `UnauthorizedException`, `InternalException`, `ApiException`. The exception
  exposes `Code`, `Kind` (`FfiErrorKind`) and `DetailsJson` directly and holds
  no native resources; `BindingError` is no longer public.

| 0.5 | 0.6 |
| --- | --- |
| `builder.AddLineItem("Pen", "2", "PCE", "50", "15", VatCategory.Standard)` | `builder.AddLineItem("Pen", 2m, "PCE", 50m, 15m, VatCategory.Standard)` |
| `decimal.Parse(totals.PayableAmount(), CultureInfo.InvariantCulture)` | `totals.PayableAmount` |
| `address.City()` | `address.City` |
| `using var id = credentials.RequestId(); id?.Value()` | `credentials.RequestId` |
| `catch (BindingErrorException e) { using var inner = e.Inner; inner.Code(); }` | `catch (FfiException e) { e.Code; e.Kind; e.DetailsJson; }` |

- Constructors are `Create` factory methods, matching Kotlin's and
  JavaScript's `create`: `Config.Create`, `Address.Create`,
  `InvoiceBuilder.Create`, and so on (`ZatcaClient.Create` and the credential
  `Create` methods are unchanged). `Adjustment.Percentage(base, percent)` is
  renamed `PercentOf`, because `Percentage` is now the property that reads the
  percentage. `ZatcaClient.WithTimeout` replaces `CreateWithTimeout`, which was
  never released.

Deprecated with `[Obsolete]`, removed in 0.7:

- `New` on the classes above: use `Create`.
- `adjustment.WithSuppliedAmount` (use `SuppliedAmount`), the `Percent`
  property (use `Percentage`) and `WithoutVatRate()` on `AllowanceCharge` and
  `VatBreakdown` (use `OmitVatRate()`).

## [0.5.1] - 2026-11-17

A compatible patch release: no public signature changed, and
`cargo semver-checks` against v0.5.0 passes for `fatoora-core` and
`fatoora-ffi`. Upgrade language packages and native libraries together.

### Security

- `CsidCredentials` no longer prints its binary security token or secret in
  `Debug` output (`{:?}`, `tracing`); both show as `<redacted>`. Python
  `CsidCompliance`/`CsidProduction` reprs redact them too.

### Added

- `InvoiceBuilder::{notes, line_items, billing_references, seller_identifiers,
  buyer_identifiers}` append every item from an iterator, like repeated calls
  to the singular methods.
- `InvoiceUuid` and `InvoiceHash` validated newtypes (not yet used by the builder).
- Optional `uuid` feature: `From<uuid::Uuid> for InvoiceUuid`.
- Optional `chrono` feature: `From<chrono::DateTime<Utc>> for InvoiceTimestamp`
  and `From<chrono::NaiveDate> for InvoiceDate`.
- `Display` for `CountryCode`, `CurrencyCode`, `InvoiceTimestamp`, `InvoiceDate`,
  `InvoiceTime`, `VatId` and `OtherId` (value only), printing `as_str()`.
- Every public `fatoora-core` item is documented, including `# Errors` and
  `# Panics` sections; `missing_docs` and the Clippy error/panic doc lints are
  now enforced for the crate.
- Python: the package ships `py.typed`; file-path parameters accept
  `os.PathLike`; handle objects have a `repr`; `InvalidInputError` is also a
  `ValueError` and `IoError` an `OSError`; `InvoiceFlag` is an `IntFlag`, so
  flags combine with `|`.

### Deprecated

- Python, changing in 0.6 (each emits `DeprecationWarning`): positional
  arguments after `name` in `InvoiceBuilder.set_seller`/`set_buyer`, positional
  `language` in `add_note`, positional `clearance_status`/`accept_language` in
  `ZatcaClient.report_simplified_invoice`/`clear_standard_invoice`, positional
  `CsrProperties.new`, and `set_note`/`set_note_without_language` (use
  `add_note(text, language=...)`).

### Changed

- Dependencies: rustls 0.23.46 (with aws-lc-sys 0.47); Playwright 1.64.1 for
  the browser test, which supports Ubuntu 26.04 CI runners.
- CI: `cargo semver-checks` guards `main` against breaking changes, and the
  required Rust checks report on every pull request.

## [0.5.0] - 2026-10-19

Upgrade language packages and their native libraries together. See the
[invoice-model migration notes](docs/development/invoice-model.md) and
[QR migration notes](docs/development/qr-correctness-evidence.md).

### Breaking changes

- `SignedInvoice::hash_base64()` and all language bindings
  now hash stored signed XML instead of reserializing its parsed model.
  CLI `generate-hash` likewise hashes the input XML directly for signed and
  unsigned documents, without requiring model conversion. Imported documents
  can therefore produce different results. `FinalizedInvoice::hash_base64()`
  still hashes emitted XML; `invoice_hash()` returns the embedded digest.
- Credit/debit invoice variants now take only the subtype; supply references and
  reasons through the invoice builder. Party addresses are optional, address
  struct literals include `province`, and identifiers and payment instructions
  use collections. Update Rust callers and rebuild native binding consumers.
- Signed XML imports now verify signature, digest, certificate metadata and QR
  consistency. Previously accepted malformed or inconsistent documents can fail
  import. Original XML and its embedded QR are retained without reconstruction.

### Added and fixed

- Preserve supported invoice fields across XML, dictionary and language-binding
  round trips, including buyer data, references, delivery dates, payment means,
  adjustments, tax-field presence and price-base quantities. Add `LineItemBuilder`.
  Retain accepted monetary values and distinguish omitted fields from empty input.
- Use subtype-specific QR tags for tax invoices and notes: standard stamps use
  Tags 1–8, simplified stamps use Tags 1–9. Preserve source decimal spellings and
  compare monetary values with bounded exact arithmetic. Reject malformed signing
  material and TLV framing through structured errors.
- Preserve the embedded QR in signed CLI workflows and reject tampered signed
  input. Accept XML whitespace in embedded signing certificates within size limits.
- Record pinned SDK and development compliance API comparisons, including known
  SDK and service validation gaps. These checks do not establish production
  clearance or certificate trust.
- Synchronize Rust, Python, npm, NuGet and JVM packages at 0.5.0.

## [0.4.4] - 2026-10-17

- Use `PayableAmount` (BT-115) for QR tag 4 in the library and CLI, including
  prepayment and payable rounding adjustments. Signing and SDK-profile validation
  now check the same amount.
- Add regression coverage for the public QR accessor and signed and unsigned CLI
  inputs. Preserve the recorded SDK corpus and document the corrected behavior.
- Synchronize package versions at 0.4.4.

## [0.4.3] - 2026-10-15

- Add Kotlin/JVM bindings with a Java-friendly API, deterministic ownership,
  coroutine helpers, and Android AAR packaging for arm64 and x86-64.
- Publish JVM and Android packages under `com.fatoority` on Maven Central, with
  signed artifacts, sources, API documentation, and package-consumer tests.
- Support GPG signing subkeys and increase Android emulator memory for the
  16 KiB page-size tests.
- Replace prerelease crypto dependencies with stable versions so fresh Cargo
  consumers build successfully; add a fresh-consumer CI check.
- Synchronize Rust, Python, npm, NuGet, and Maven packages at 0.4.3.

## [0.4.2] - 2026-10-12

- Add C#/.NET 8 bindings with deterministic disposal, strict string conversion,
  and prepared requests for asynchronous HTTP through caller-owned transports.
- Package native libraries for Linux x64 (glibc 2.28+), Windows x64, and macOS
  arm64; publish tested NuGet packages on version tags using trusted publishing.
- Include the bundled ZATCA notices and license texts in the NuGet package.
- Add the official .NET SDK comparison harness, recorded hashing/signing/validation
  benchmarks, and reproduction instructions.
- Document C# support in the README and synchronize Rust, Python, npm, and NuGet
  package versions at 0.4.2.

## [0.4.1] - 2026-10-11

- Add Diplomat JavaScript/TypeScript bindings for Node and browsers, with async
  fetch APIs backed by Rust request preparation and response decoding.
- Build a size-optimized WASM artifact with losslessly compressed UBL schemas,
  cryptographic host randomness and explicit validation time.
- Add JS/TS documentation, runtime/browser/package tests and artifact size limits.
- Publish the tested npm tarball on version tags using the JavaScript release workflow.
- Preserve Rust validation errors for invalid JS options and document the compiled
  SDK-derived rules and their upstream licenses.
- Synchronize Rust, Python and npm packages at 0.4.1.

## [0.4.0] - 2026-10-11

### Breaking changes

- Rename the project and repository to `fatoority`, the Python distribution to
  `fatoority`, and the CLI crate to `fatoora-cli` with executable `fatoora`.
  Install Python with `pip install fatoority` and the CLI with
  `cargo install fatoora-cli`. Python still imports as `fatoora`; Rust crate
  names and C/C++ interfaces remain unchanged. Documentation now lives at
  https://docs.fatoority.com.
- Before switching Python distributions, uninstall the previous distribution,
  then install `fatoority`; both distributions provide the same `fatoora` import package.
- The CLI executable is now `fatoora`. It shares its name with the official
  ZATCA SDK executable; use an explicit path when both are installed.

### Distribution

- Publish the first `fatoority` Python distribution and `fatoora-cli` crate.
  Existing packages remain available under their historical names.
- Synchronize `fatoora-core`, `fatoora-derive`, `fatoora-ffi`, `fatoora-cli`, and
  the Python distribution at 0.4.0.

## [0.3.0] - 2026-10-11

Upgrade bindings and their native library together. This release replaces the
C ABI and Python native loader, adds complete local ZATCA validation, and
removes libxml2 from ordinary builds and distributed artifacts.

### Breaking changes

- Replace handwritten C exports with Diplomat-generated `fatoora_Type_method`
  symbols. Recompile C/C++ clients against the matching generated headers.
  Strings are length-delimited UTF-8 views, numeric getters write to
  `DiplomatWrite`, and fallible results contain an owned `BindingError`.
  See the [binding migration guide](docs/development/diplomat.md).
- Python keeps its high-level API but removes `FfiLibrary`, the CFFI loader,
  and custom native-library/header environment overrides. Install the matching
  wheel, which bundles its Nanobind extension and native library.
- XML entrypoints require UTF-8 XML 1.0 without DTDs and enforce an 8 MiB limit
  for complete documents, including signed output. Signing rejects mismatched
  keys, different existing certificates, invalid reference bindings, and
  externally owned or ambiguous signatures.

### Added

- Complete local ZATCA validation through Rust, CLI, C/C++, and Python: XSD,
  all 257 CEN/KSA assertions, and applicable signature/QR integrity checks.
  Reports distinguish rejection, missing context, and execution failure.
  See [local validation](docs/reference/invoice-validation.md).
- Generated C++ ownership wrappers and Python native bindings for invoices,
  signing, keys, CSRs, credentials, and HTTP responses.
- Seeded property tests, bounded XML/QR/signing fuzz targets, and independent
  canonicalization, signature-tampering, and schema-negative regressions.

### Fixed

- Preserve unrelated XML bytes, comments, processing instructions, and extension
  content during signing. Re-signing with the same certificate and preserved
  signing time produces identical bytes and verifies the complete result.
- Align hash exclusions with exact QR identifiers and preserve valid text across
  excluded subtree boundaries. Preserve the certificate's original SPKI encoding
  in QR tag 8 and escape issuer text consistently with SignedProperties digests.
- Preserve invoice flags and free-text values, including CR/CRLF, across XML
  import and output. Reject truncated or duplicate QR TLV tags.
- Preserve literal CSR distinguished-name values and improve binding ownership,
  failure recovery, HTTP, and CLI regression coverage.
- Validate large UBL decimal values with a narrowly scoped compatibility shim;
  retain lexical, declared-type, attribute, ordering, and namespace constraints.
  Resolve bundled schema paths consistently across platforms.

### Distribution

- Synchronize all four Rust crates and the Python package at 0.3.0.
- Use roxmltree for XML interpretation, xml-sec for C14N 1.1, and xsd-schema for
  XSD validation. libxml2 remains only in optional reference tools.
- Remove libxml2 installation, linking, and bundling from normal builds,
  Linux/macOS/Windows CLI and FFI archives, and Python wheels. Verify the actual
  artifact dependency closure in CI.
- Ship generated C/C++ headers with FFI archives and retain the bundled native
  library layout in repaired Python wheels.

## [0.2.0] - 2026-09-18

This release breaks compatibility with 0.1.x in Rust, the C ABI, Python, and
serialized invoice data. Upgrade bindings and their native library together.

### Breaking changes

- Invoice numbers use the crate-owned `Decimal` type. Rust callers construct
  values from strings or integers; JSON encodes them as decimal strings. Python
  accepts `decimal.Decimal`, `str`, or `int` and rejects floats. C numeric inputs
  are UTF-8 strings, and numeric getters return owned strings. See the
  [numeric migration guide](docs/reference/numbers.md#bindings-and-migration).
- Rust `InvoiceBuilder` configuration methods consume and return the builder.
  Chain calls or assign their result back. `build()` consumes the builder on
  success and failure. C and Python retain mutable setters, but their builder
  handles are consumed by `build()` as well.
- `SignedInvoice` exposes `xml()` for borrowing and `into_xml()` for consuming
  its exact signed XML. C and Python expose copied and consuming XML accessors;
  discard a handle after using the consuming accessor.
- Public errors retain typed causes and structured details using crate-owned
  types. C errors are opaque handles with accessor functions and an explicit
  free function. Rebuild C/C++ callers against the generated 0.2.0 headers.
  See [error ownership and classification](docs/reference/errors.md).
- Invoice HTTP responses retain their status and body alongside typed validation
  results. Use `outcome()` or `ensure_accepted()` to distinguish invoice
  acceptance from a successful HTTP request. Handle rejected and unknown
  outcomes explicitly. Cleared invoice XML is decoded through the response
  accessor. See the [API client guide](docs/reference/api-client.md).
- XML generation uses inherent `FinalizedInvoice::to_xml()` and
  `to_xml_with_format()` methods. Remove `ToXml` imports and replace
  `to_xml_pretty()` calls with `to_xml()`. `InvoiceView` and `InvoiceXml` are
  internal, and `PartyRole` is sealed against downstream implementations.
- Addresses use `district` for XML `CitySubdivisionName`; the redundant
  `subdivision` field and constructor argument have been removed in every
  binding. See the [public API migration](docs/development/api-audit.md#migration).

### Fixes and additions

- Validate decimal amounts and totals without floating-point conversions,
  including half-up rounding, VAT category aggregation, supplied line VAT,
  prepayments, and payable-rounding adjustments.
- Enforce validated wrapper invariants during deserialization and expose
  structured invoice validation reports. Rust XML validation covers XSD;
  complete ZATCA business-rule validation remains separate work.
- Preserve optional address fields during XML import and output.
- Preserve HTTP error status and bodies, reject unexpected redirects, and
  report malformed success responses without treating them as acceptance.
- Correct Arabic CSR property decoding, certificate-template ASN.1 encoding,
  CSR name ordering, and validation of CSR capability flags.
- Align signing and QR output with recorded SDK behavior, including the QR
  timestamp and SignedProperties digest when re-signing formatted XML.
- Add a frozen 53-case ZATCA SDK `238-R3.4.8` corpus and offline regressions for
  canonical bytes, hashes, signatures, QR fields, XSD results, and CSRs. The
  [SDK compatibility notes](docs/development/sdk-parity.md) describe coverage
  limits and the recorded QR rounding and foreign-currency differences.
- License original project code under `MIT OR Apache-2.0` and include
  [third-party notices](THIRD_PARTY_NOTICES.md). Upstream materials retain their
  own terms; the notice records unresolved provenance separately.

### Distribution

- Report the CLI package version through the `--version` flag.
- Synchronize Rust crates and the Python package at 0.2.0.
- Validate versions and publication tags in CI and dry-run Cargo packaging.
- Build wheels and native assets on release PRs before tagging. Native GitHub
  assets upload only after every native build succeeds.
- Use the Cargo lockfile for native builds. Test installed wheels by calling
  the native library, and restrict Windows wheels to the supported AMD64 target.
- Bundle the generated ABI header in Python wheels so address constructors and
  the complete binding declarations are available outside a source checkout.
- Repair Windows wheels for runtime-loaded DLL dependencies. Windows CLI and
  FFI ZIP archives include vcpkg DLLs and their installed license material.
  Windows CLI users should extract the complete archive before running it.
- Require CFFI 1.17+ so Windows loads repaired wheel dependencies through the
  registered DLL search directories.

[0.2.0]: https://github.com/mqqz/fatoority/compare/v0.1.3...v0.2.0

[0.3.0]: https://github.com/mqqz/fatoority/compare/v0.2.0...v0.3.0

[0.4.0]: https://github.com/mqqz/fatoority/compare/v0.3.0...v0.4.0
