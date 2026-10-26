---
title: Decimal numbers and invoice rounding
description: Exact decimal inputs, supported precision, VAT rounding and invoice total calculations in fatoority.
---

# Decimal numbers and rounding

Use decimal strings or integers for invoice amounts. Parsing never silently rounds an input.

## Representation

Accepted syntax is `[+-]digits[.digits]`, using ASCII digits. Whitespace,
exponents, separators, infinities and NaN are rejected. Parsing never rounds.
The range is a signed 96-bit coefficient with a scale from 0 through 28.
Values outside that range fail with an out-of-range error.

Equality compares numeric values: `1.0` equals `1.00`. Display and JSON use
canonical decimal strings, without exponents or unnecessary trailing zeros.
Negative zero is displayed as `0`. XML monetary fields use two decimal places;
quantities and unit prices retain their supported precision.

Arithmetic is checked. Values outside the supported decimal range return an
error. Price-base division uses integer rational arithmetic until rounding, so a
non-terminating per-unit price does not lose precision before the line calculation.

## Calculation boundaries

- Line net: round quantity × net price / price base to cents, subtract line
  allowances, and add line charges. The default price base is one.
- Calculated line VAT: round the accepted line net × rate / 100 to cents.
  Supplied line VAT is retained after its precision and amount checks.
- Line gross: accepted line net plus accepted line VAT.
- Group taxable amount: sum accepted line nets by VAT category and rate,
  subtract that group's document allowances, and add its document charges.
- Calculated category VAT: round the group's taxable amount × rate / 100 to
  cents. Document VAT is the sum of accepted category taxes.
- Payable: tax-inclusive amount minus prepaid amount plus signed payable rounding.

Default calculated amounts use half-up rounding. For three lines of `0.03` at
15%, each calculated line VAT is `0.00`, while document VAT is `0.01`.
XML output and QR payloads use the accepted totals without recalculating them.

Rates must be between 0 and 100 with at most two decimal places. Categories other
than standard require a zero rate. Supplied monetary totals must already have at
most two decimal places. Negative line quantities, prices, discounts, and charges
are rejected; credit notes use positive amounts with the appropriate document
type. Payable rounding can be signed. Price bases must be positive.

Use `Adjustment::amount` or `Adjustment::percentage` for line adjustments, and
`DocumentAdjustment` for invoice adjustments with independent VAT categories and
rates. Scalar invoice discount/charge setters require an unambiguous VAT group;
combining them with independent adjustment entries is rejected. See
[pricing recipes](../guides/invoice-recipes.md) for examples.

## Calculated and supplied values

The model checks supplied amounts and retains accepted values. A mismatch reports
both the expected and supplied amounts. Acceptance depends on the field:

| Supplied value | Check |
| --- | --- |
| Net price | Exact agreement with gross price minus price discount, when both are supplied. |
| Line net | Within `0.01` of the calculated line net. |
| Line gross | Exact sum of accepted line net and line VAT. |
| Adjustment amount and percentage | Exact agreement with the rounded base × percentage / 100 calculation. |
| Standard-category breakdown tax | Within invoice-line count × `0.01` of the half-even rounded exact tax calculation. |
| Other category tax | Zero. |
| Document VAT and totals | Consistent with accepted breakdowns and the document total equations. |

Default construction and supplied-tax tolerance checks use different rounding
rules. For example, `53.30` at 5% produces `2.67` during construction. The
standard-category supplied-tax check centers its tolerance on `2.66`; with one
invoice line, it accepts `2.65` through `2.67`.

Prepayment adjustment lines contribute their taxable and tax subtotals to the
prepaid amount, without increasing ordinary taxable sales. Imported XML also
preserves supported optional-amount presence, including payable rounding.

These checks are separate from schema and business-rule validation. The
[model contract](../development/invoice-model.md) documents the field dependencies
and differences between the published standard and the pinned SDK's executable
rules. Run [local validation](../guides/validation.md) for that rule profile.


## Values in your language

=== "Rust"

    `fatoora_core::Decimal` accepts decimal strings through `parse()` and integers through `Decimal::from`. Parsing returns `DecimalError` on invalid syntax or range. Numeric getters return `Decimal`.

    ```rust
    let price: fatoora_core::Decimal = "0.3333".parse()?;
    ```

=== "Python"

    Inputs accept `decimal.Decimal`, `str`, or `int`. Both `float` and `bool` are rejected. Numeric getters return `decimal.Decimal`.

    ```python
    from decimal import Decimal
    price = Decimal("0.3333")
    ```

=== "JavaScript"

    Pass quantities, prices, rates and amounts as decimal strings, such as `"0.3333"`. Numeric getters return strings. Avoid converting financial values to `Number`, which can lose precision.

=== "C#"

    Pass quantities, prices, rates and amounts as invariant decimal strings, such as `"0.3333"`. Numeric getters return `string`. If formatting a `decimal`, use `CultureInfo.InvariantCulture`.

=== "Kotlin"

    Pass quantities, prices, rates and amounts as decimal strings. Numeric getters return `String`. Use `BigDecimal.toPlainString()` when converting a value; scientific notation is not accepted.

=== "Java"

    Pass quantities, prices, rates and amounts as decimal strings. Numeric getters return `String`. Use `BigDecimal.toPlainString()` when converting a value; scientific notation is not accepted.

=== "C"

    Inputs are NUL-terminated UTF-8 decimal strings, such as `"0.3333"`. Numeric getters return owned text through `char **out`; release it with `fatoora_string_free()`. Check the returned `FatooraStatus` before reading the output.
