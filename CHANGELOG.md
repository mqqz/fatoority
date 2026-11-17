# Changelog

## Unreleased

Breaking 0.6 phase for the `fatoora-core` Rust crate. Renamed items keep their
old names as deprecated aliases until 0.7 (see "Deprecated"); items whose type,
signature or fields changed in place have no deprecation period (see "Changed
(breaking)"). The `fatoora-ffi` bridge and the generated bindings replace
raw integer codes and Booleans with enums (see "Bindings" under "Changed
(breaking)"); the Python `fatoora` package API is unchanged.

Migration guide with a before-and-after table of every change: [Migrate from 0.5 to 0.6 (Rust)](https://docs.fatoority.com/guides/migrate-0.6/).

### Added

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

- Bindings: FFI enums `Environment`, `InvoiceTypeKind`, `InvoiceSubType`,
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

### Changed

- `InvoiceBuilder::allowance(DocumentAdjustment)` and
  `charge(DocumentAdjustment)` are the one way to add document-level
  allowances and charges, and `payment_means(PaymentMeans::new(code))` the way
  to add payment means; the alternatives are deprecated (see "Deprecated").
- A `DocumentAdjustment` whose calculation is invalid is now reported with the
  other adjustment checks in `InvoiceBuilder::build`, after missing or invalid
  fields, instead of before them.

### Changed (breaking)

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
  `false`, or `ClearanceMode::from(flag)`. The FFI and language bindings still
  take a Boolean.
- Response status getters return the new enums instead of `Option<&str>`:
  `ValidationResponse::reporting_status()` returns `Option<&ReportingStatus>`,
  `ValidationResponse::clearance_status()` returns `Option<&ClearanceStatus>`,
  `ValidationResults::status()` and `ValidationMessage::status()` return
  `Option<&ValidationStatus>`, and `ValidationMessage::message_type()` returns
  `Option<&MessageType>`. Compare with the enum
  (`Some(&ReportingStatus::Reported)`) or call `.map(ReportingStatus::as_str)`
  for the previous `Option<&str>`. The JSON wire format and the `fatoora-ffi`
  getters, which still return text, are unchanged.

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
  The FFI `Adjustment.percentage` constructor is unchanged.
- `PaymentMeans::payee_financial_account(account)` is renamed
  `with_payee_financial_account(account)`; `payee_financial_account()` is now
  the getter.

**Bindings (`fatoora-ffi`, JavaScript, Kotlin/Java, C#, C/C++)**

The Python `fatoora` package API is unchanged. Enum discriminants for
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
  Boolean (or a `ClearanceMode`), and the Kotlin `*Async` helpers still take a
  Boolean.
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

### Deprecated

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
  `hash()`, which returns `InvoiceHash`. They still return `String`. The FFI
  `hash_base64` methods are unchanged.
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

The `fatoora-ffi` exported methods (`set_payment_means_code`,
`invoice_level_charge`, `invoice_level_discount`, `allowance_reason`,
`set_allowance`, `AllowanceCharge::without_vat_rate`,
`VatBreakdown::without_vat_rate`) are unchanged.

### Removed

- The `invoice::LineItems` alias (`Vec<LineItem>`). Use `Vec<LineItem>`.
- `InvoiceData::note()`, which returned only the first note. Use
  `notes().first()`.
- `OriginalInvoiceRef::with_issue_date_str`. Use
  `with_issue_date(InvoiceDate::parse(date)?)` or `with_issue_date(date.parse()?)`.

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
