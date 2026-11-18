---
title: Migrate a Rust Project from fatoora-core 0.6 to 0.7
description: Every fatoora-core item removed in 0.7, each with the 0.6 replacement to use instead.
---

# Migrate from 0.6 to 0.7: Rust

This page covers the `fatoora-core` crate. The [overview](index.md) lists the
other languages.

Update `Cargo.toml`:

```toml
[dependencies]
fatoora-core = "0.7.0"
```

0.7 removes every item deprecated in 0.6. Each replacement below already
exists in 0.6, so the simplest path is to add `#![deny(deprecated)]` to your
crate root, fix every error while still on 0.6, and then bump the version.

## Removed items

Paths are relative to `fatoora_core`.

### Configuration and API client

| Removed | Use instead |
| --- | --- |
| `config::EnvironmentType` | `config::Environment` |
| `ZatcaClient::post_csr_for_ccsid(csr, otp)` | `ZatcaClient::request_compliance_csid(csr, otp)` |
| `ZatcaClient::post_ccsid_for_pcsid(ccsid)` | `ZatcaClient::request_production_csid(ccsid)` |
| `ZatcaClient::prepare_post_csr_for_ccsid(csr, otp)` | `ZatcaClient::prepare_request_compliance_csid(csr, otp)` |
| `ZatcaClient::prepare_post_ccsid_for_pcsid(ccsid)` | `ZatcaClient::prepare_request_production_csid(ccsid)` |

### CSR properties

| Removed | Use instead |
| --- | --- |
| `CsrProperties::from_properties_str(text)` | `text.parse::<CsrProperties>()` (`FromStr`) |
| `CsrProperties::parse_csr_config(text)` | `text.parse::<CsrProperties>()` (`FromStr`) |
| `CsrProperties::parse_csr_config_file(path)` | `CsrProperties::from_file(path)` |

### Invoice builder, adjustments and payment means

| Removed | Use instead |
| --- | --- |
| `invoice::InvoiceTotalsData` | `invoice::InvoiceTotals` |
| `InvoiceBuilder::adjustment(entry)` | `allowance(DocumentAdjustment::from(entry))` or `charge(DocumentAdjustment::from(entry))`, by `entry.kind()` |
| `InvoiceBuilder::invoice_level_discount(amount)` | `allowance(DocumentAdjustment::new(Adjustment::amount(amount), category, rate))` |
| `InvoiceBuilder::invoice_level_charge(amount)` | `charge(DocumentAdjustment::new(Adjustment::amount(amount), category, rate))` |
| `InvoiceBuilder::allowance_reason(text)` | `DocumentAdjustment::with_reason(text)` on each entry |
| `InvoiceBuilder::payment_means_code(code)` | `InvoiceBuilder::payment_means(PaymentMeans::new(code))` |
| `PaymentMeans::instruction_note(text)` | `PaymentMeans::with_instruction_note(text)` |
| `Adjustment::allowance_charge_reason()` | `Adjustment::reason()` |
| `Adjustment::allowance_charge_reason_code()` | `Adjustment::reason_code()` |
| `Adjustment::calculation_base_amount()` | `Adjustment::base_amount()` |
| `Adjustment::calculation_percentage()` | `Adjustment::percentage()` |
| `VatBreakdown::without_vat_rate()` | `VatBreakdown::omit_vat_rate()` |

The aggregate setters replaced one allowance and one charge in the document VAT
category, with the rate taken from the first line in that category.
`allowance` and `charge` append one entry per call with the category and rate
you pass, so pass that category and rate to keep the same totals. Likewise,
`payment_means_code` changed the code of the first payment means, while
`payment_means` appends one.

The `InvoiceData` getters `invoice_level_discount()`, `invoice_level_charge()`
and `allowance_reason()` stay: they report the totals and shared reason of the
document-level entries, which are now the only place `InvoiceData` keeps them.

Invoice data serialized by 0.5 or 0.6 with an aggregate allowance or charge
still deserializes and produces the same XML: the amounts become entries in
`adjustments()` with the document VAT category, the rate of the single line
group in that category, and the old reason (`"discount"` or `"charge"` when none
was set). If no line group or more than one matches, deserialization fails, as
the 0.6 totals did. JSON written by 0.7 omits the aggregate fields and does not
load in 0.6; bincode keeps the 0.6 layout.

### Validation and hashing

| Removed | Use instead |
| --- | --- |
| `invoice::validation::validate_xml_invoice_from_str(xml, &config)` | `Validator::new().validate_xml(xml)` |
| `invoice::validation::validate_xml_invoice_report_from_str(xml, &config)` | `Validator::new().validate_xml_report(xml)` |
| `invoice::validation::validate_zatca_invoice_from_str(xml, &config, &options)` | `Validator::new().validate_zatca(xml, &options)` |
| `invoice::sign::invoice_hash_base64_from_xml_str(xml)` | `invoice::hash_xml(xml)` |
| `FinalizedInvoice::hash_base64()` | `FinalizedInvoice::hash()` |
| `SignedInvoice::hash_base64()` | `SignedInvoice::hash()` |

The replacements return an `InvoiceHash` instead of a `String`; call
`.as_str()` or `.to_string()` where you need the text.

## Zero request timeout is invalid input

`ZatcaClient::with_timeout(config, Duration::ZERO)` returns the new
`ZatcaError::InvalidTimeout`, classified as `ErrorKind::InvalidInput`, instead
of `ZatcaError::ClientState` (`ErrorKind::Internal`). Code that matched
`ClientState` or `ErrorKind::Internal` for this case should match the new
variant or kind. The bindings report it with the invalid-input error code (1)
and details type `invalid_timeout`.

## Example

```rust
// 0.6, with deprecation warnings
let invoice = builder
    .invoice_level_discount(discount)
    .allowance_reason("Loyalty")
    .payment_means_code("10")
    .build()?;
let hash: String = invoice.hash_base64()?;
```

```rust
// 0.7
use fatoora_core::invoice::{Adjustment, DocumentAdjustment, PaymentMeans, VatCategory};

let invoice = builder
    .allowance(
        DocumentAdjustment::new(Adjustment::amount(discount), VatCategory::Standard, rate)
            .with_reason("Loyalty"),
    )
    .payment_means(PaymentMeans::new("10"))
    .build()?;
let hash: String = invoice.hash()?.to_string();
```
