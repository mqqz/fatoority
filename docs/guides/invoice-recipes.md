---
title: Invoice Pricing, VAT, Credit Notes and Prepayments
description: Apply discounts and charges, use price-base quantities and mixed VAT, and model credit notes or prepayment adjustments with fatoority.
---

# Pricing and invoice recipes

Start with the runnable [invoice creation example](invoice-creation.md).
The Rust fragments below replace its line item or extend its builder. Use the
[advanced binding methods](../reference/invoice-model.md#advanced-building-in-the-bindings)
for the corresponding operations in other languages.

## Price per pack and a line discount

A price base expresses how much quantity the quoted price covers. Here, SAR 100
buys ten units. Selling 25 units gives SAR 250 before a SAR 10 line allowance:
line net is SAR 240, VAT is SAR 36, and line gross is SAR 276.

Replace the example's `let item = ...` with:

```rust
use fatoora_core::invoice::Adjustment;

let item = LineItem::builder()
    .item_name("Notebook")
    .invoiced_quantity("25".parse()?)
    .invoiced_quantity_unit_code("PCE")
    .item_net_price("100".parse()?)
    .item_price_base_quantity("10".parse()?)
    .vat_category(VatCategory::Standard)
    .vat_rate("15".parse()?)
    .allowance(Adjustment::amount("10".parse()?).reason("Volume discount"))
    .build()?;
```

Use `.charge(...)` for a line surcharge. An `Adjustment::percentage(base, percent)`
calculates an adjustment using the supplied base. For example, a 10% allowance on
SAR 250 is `Adjustment::percentage("250".parse()?, "10".parse()?)`.

For gross pricing, supply `.item_gross_price(...)` and `.item_price_discount(...)`.
The builder derives the net price and checks it if you also supply `.item_net_price(...)`.
Price-base division retains exact rational precision until the line net is rounded
at the calculation boundary.

## Document discounts and charges

Each document adjustment carries its VAT category and rate. To subtract SAR 10
from the creation example's SAR 100 taxable amount, add this call before `.build()`:

```rust
.allowance(fatoora_core::invoice::DocumentAdjustment::new(
    fatoora_core::invoice::Adjustment::amount("10".parse()?).reason("Loyalty discount"),
    VatCategory::Standard,
    "15".parse()?,
))
```

The resulting VAT is SAR 13.50 and payable is SAR 103.50. Use `.charge(...)` for
shipping or another document charge with its own tax treatment. Repeated calls
append adjustments.

The scalar `invoice_level_discount` and `invoice_level_charge` setters remain
available for a single VAT group. Use independent adjustment entries when you
need multiple groups; combining the two forms is rejected.

## Mixed VAT categories and rates

Assign each line its category and rate. The builder groups accepted line net
amounts by both values, applies each group's document adjustments, and calculates
the VAT breakdown. Document VAT is the sum of breakdown taxes.

For example, SAR 100 at 15% plus SAR 50 at 0% gives SAR 15 VAT and SAR 165 payable
before other adjustments. Use `VatCategory::Zero` for the zero-rated line.
`Exempt` and `OutOfScope` are also available; those categories use a zero rate.

Set exemption metadata where the invoice's tax treatment requires it. A line
builder accepts `.exemption(TaxExemption { reason_code, reason })`; document
breakdown metadata uses `.tax_exemption(category, rate, exemption)` on the invoice
builder. Both fields are optional strings in the model. Choose the applicable
code and explanation for the transaction, then run local business validation.

## Credit and debit notes

Use `InvoiceType::CreditNote(InvoiceSubType::Simplified)` or
`InvoiceType::DebitNote(InvoiceSubType::Simplified)` in place of the tax-invoice
type. Both also support `Standard`. Add the original invoice reference and a
payment instruction explaining the correction:

```rust
.billing_reference(fatoora_core::invoice::OriginalInvoiceRef::new("INV-1001"))
.payment_instruction("Return of goods")
```

Keep line quantities and amounts positive. The document type identifies the
correction. Set a new ID, UUID, issue timestamp, counter, and predecessor hash for
the note. Repeated `.billing_reference(...)` calls preserve multiple references.

## Prepayments and final payable amounts

Use `InvoiceType::Prepayment(subtype)` to create a prepayment invoice. On an invoice
that deducts an advance payment, `.prepaid_amount(...)` subtracts the supplied
amount from the tax-inclusive total. `.payable_rounding_amount(...)` applies a
signed rounding adjustment:

```text
Payable = tax-inclusive total − prepaid amount + payable rounding
```

A SAR 115 tax-inclusive total with SAR 23 prepaid has SAR 92 payable. This amount
also supplies QR Tag 4.

To represent an advance payment of SAR 20 plus SAR 3 VAT, insert this declaration
before `let invoice = ...` in the creation example:

```rust
use fatoora_core::invoice::{InvoiceTime, OriginalInvoiceRef, VatBreakdown};

let advance = LineItem::builder()
    .item_name("Advance payment adjustment")
    .invoiced_quantity("0".parse()?)
    .invoiced_quantity_unit_code("PCE")
    .item_net_price("0".parse()?)
    .vat_category(VatCategory::Standard)
    .vat_rate("15".parse()?)
    .document_reference(
        OriginalInvoiceRef::new("PRE-1001")
            .with_uuid("b2a43c49-3aab-4e3b-9d67-0da45a5e33cc")
            .with_issue_date_str("2026-10-25")?
            .with_issue_time(InvoiceTime::parse("09:00:00Z")?)
            .with_document_type_code("386"),
    )
    .tax_subtotal(VatBreakdown::new(
        VatCategory::Standard,
        "15".parse()?,
        "20".parse()?,
        "3".parse()?,
    )?)
    .build()?;
```

Keep the sale's `.line_item(item)` call and add these calls before the invoice's
`.build()`:

```rust
.line_item(advance)
.prepaid_amount("23".parse()?)
```

The result has SAR 15 document VAT and SAR 92 payable. Replace the sample reference
with the prepayment invoice's actual identity and issue date/time.

`with_document_type_code("386")` marks the reference as a prepayment. Without it,
the model treats the line as an ordinary sale. Adjustment lines must have zero
quantity, price, line net, and line VAT; the advance amounts belong in their VAT
subtotals. Their combined taxable and tax subtotals must equal `prepaid_amount`.
Setting that amount alone does not create references or adjustment lines. See the
[model contract](../development/invoice-model.md) for the calculation checks.

## Purchase orders, delivery and payment data

| Application data | Rust builder API |
| --- | --- |
| Purchase order or contract | `purchase_order_id(...)`, `contract_id(...)` |
| Actual/latest delivery dates | `delivery(Delivery { actual_date, latest_date })` |
| Payment code with instructions and account | `payment_means(PaymentMeans::new(code).instruction_note(text).payee_financial_account(account))` |
| Additional party identifiers | `seller_identifier(...)`, `buyer_identifier(...)` |
| Item identifiers | Line builder `item_seller_identifier(...)`, `item_buyer_identifier(...)`, `item_standard_identifier(...)` |
| Notes in different languages | Repeated `note(InvoiceNote::new(language, text))` calls |

Use collection getters when reading repeated values; convenience getters expose
the first entry. [XML import](../reference/xml.md) preserves the supported fields,
subject to the documented normalization and unsupported-content policy.
