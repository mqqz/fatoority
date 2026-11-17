---
title: Invoice Model Reference
description: Invoice builder methods and field requirements for fatoority, with language-specific signatures and object lifetimes.
---

# Invoice model

Build invoices and calculate their totals with the typed model. Start with the
[runnable creation guide](../guides/invoice-creation.md) or the
[pricing recipes](../guides/invoice-recipes.md). For an existing XML document, start with [XML parsing](xml.md). For signatures and certificate inputs, see [invoice signing](invoice-signing.md).

## Building an invoice

Create a builder, set the required fields, append one or more line items, then build a finalized invoice. Building checks supported field formats and amounts and calculates totals. It does not run schema validation, all ZATCA business rules, or signature verification; use [validation](invoice-validation.md) for those checks.

Scalar setters replace their previous value. Lines, notes, billing references, and
independent adjustments append in order. Rust `allowance(DocumentAdjustment)` and
`charge(DocumentAdjustment)` append document adjustments, and the generated
bindings mirror them as `add_allowance` and `add_charge`. In the generated
bindings, `set_*` methods replace a value and `add_*` methods append one.
Python's deprecated `set_allowance(reason, amount)` and aggregate setters are
applied as document adjustments when the invoice is built; use `add_allowance`
and `add_charge` instead.
Use collection getters to read every repeated entry.

## Fields and expected values

| Field | Required | Expected value and behavior |
| --- | --- | --- |
| Invoice type and subtype | Yes | Tax invoice, prepayment, credit note, or debit note; standard or simplified subtype. Supply original invoice references and correction reasons for credit/debit business validation. Original UUID and issue date are optional. |
| ID | Yes | Nonblank invoice identifier, such as `INV-1001`. |
| UUID | Yes | Invoice UUID text. The builder checks nonblank input, not UUID syntax. |
| Issue datetime | Yes | UTC `YYYY-MM-DDTHH:MM:SSZ`, for example `2026-10-15T09:30:00Z`. |
| Currency | Yes | Recognized ISO currency code, such as `SAR`. Trimmed and normalized to uppercase. |
| Previous invoice hash | Yes | Previous invoice hash text. The builder checks nonblank input; it does not verify the chain or decode the hash. |
| Invoice counter | Yes | Unsigned 64-bit integer. The model accepts zero; check applicable business rules separately. |
| Seller | Yes | Name, address, and VAT ID; optional other identification and scheme. |
| Buyer | No in builder | Name with optional address and identifiers. Identification requirements depend on the invoice type and are checked by business validation. |
| Payment means code | Yes | Nonblank code text, for example `10` for cash. The builder does not validate the complete payment code list. |
| VAT category | Yes | Standard, zero-rated, exempt, or out of scope. Each line also has its own category. |
| Line items | Yes | At least one. Nonblank description; nonnegative quantity and unit price. Unit code can be omitted; a supplied code must be nonblank. |
| VAT rate | Per line | Percentage in the inclusive range `0`–`100`, at most two decimal places. Categories other than standard require zero. |
| Invoice charge / discount | No | Nonnegative decimal amount, at most two decimal places. Defaults to zero. |
| Line pricing | No | Price-base quantity, gross price and price discount, supplied amounts, line allowances/charges. |
| Independent document adjustments | No | Repeated allowances/charges with their own VAT category and rate. |
| VAT breakdowns and exemptions | No | Supplied category/rate totals and independent exemption code/text. |
| Prepaid amount / payable rounding | No | Prepaid amount subtracts from payable; rounding is signed. |
| Business references | No | Repeated billing references, purchase order ID, contract ID. |
| Delivery | No | Independent actual/latest dates. |
| Payment entries | No | Ordered payment codes with optional instructions and financial account data. |
| Party and item identifiers | No | Multiple party IDs; seller, buyer, and standard item IDs. |
| Notes | No | Repeated language/text entries in insertion order. |
| Flags | No | Third party, nominal, export, summary, self-billed. Defaults to no flags. |

Address inputs are country code, city, street, building number, and postal code, with optional additional street, additional number, district, and province/state. Buyer addresses may be absent. Country accepts recognized ISO alpha-2 or alpha-3 values (`SA` or `SAU`) and is stored as alpha-3. A VAT ID wrapper checks only that trimmed text is nonempty; use ZATCA validation for country-specific requirements.

Credit/debit note original issue dates use `YYYY-MM-DD`. Optional values mean absence; an empty string is still a supplied value.

See [decimal numbers](numbers.md) for precision and rounding. Line totals are calculated from quantity and price. Explicitly supplied line net amounts have a `0.01` tolerance around the rounded calculation; imported amounts are not accepted merely because they parse.

## Returned data and totals

A finalized invoice contains invoice data and calculated totals. A signed invoice also stores the original signed XML alongside its QR payload and signature metadata. Signed import checks the signature and digests against the embedded certificate
and verifies QR consistency. Certificate issuer trust and gateway acceptance
remain separate checks.

| Result | Contents |
| --- | --- |
| Invoice data | Identity, type/subtype, parties, notes, original invoice reference, flags, line items, and invoice adjustments. |
| Line item | Description, quantity, unit code, unit price, line total, VAT rate, VAT amount, and category. Indexed access is zero-based and rejects out-of-range indexes. |
| Totals | Line extension, allowance total, charge total, taxable amount, tax amount, tax inclusive total, prepaid amount, payable rounding amount, and payable amount. |
| Computed hash | `hash()` hashes emitted XML for a finalized model and stored XML for a signed invoice. Signed import rejects digest mismatches and retains the original XML. |

## Advanced building in the bindings

The bindings expose `LineItemBuilder` for price bases and line adjustments. Build
the line, then append it with the invoice builder's `add_item` equivalent.
`add_line_item` remains the shortcut for a line with quantity and unit price.

| Operation | Python | JavaScript / TypeScript | C# | Kotlin / Java |
| --- | --- | --- | --- | --- |
| Create a line builder | `LineItemBuilder()` | `LineItemBuilder.create()` | `LineItemBuilder.New()` | `LineItemBuilder.create()` |
| Set price base | `set_item_price_base_quantity(value)` | `setItemPriceBaseQuantity(value)` | `SetItemPriceBaseQuantity(value)` | `setItemPriceBaseQuantity(value)` |
| Add line allowance | `add_allowance(adjustment)` | `addAllowance(adjustment)` | `AddAllowance(adjustment)` | `addAllowance(adjustment)` |
| Add line charge | `add_charge(adjustment)` | `addCharge(adjustment)` | `AddCharge(adjustment)` | `addCharge(adjustment)` |
| Finish line | `build()` | `build()` | `Build()` | `build()` |
| Append built line to invoice | `add_item(item)` | `addItem(item)` | `AddItem(item)` | `addItem(item)` |
| Append document allowance | `add_allowance(...)` | `addAllowance(...)` | `AddAllowance(...)` | `addAllowance(...)` |
| Append document charge | `add_charge(...)` | `addCharge(...)` | `AddCharge(...)` | `addCharge(...)` |
| Set prepaid amount | `set_prepaid_amount(value)` | `setPrepaidAmount(value)` | `SetPrepaidAmount(value)` | `setPrepaidAmount(value)` |
| Set payable rounding | `set_payable_rounding_amount(value)` | `setPayableRoundingAmount(value)` | `SetPayableRoundingAmount(value)` | `setPayableRoundingAmount(value)` |
| Append billing reference | `add_billing_reference(reference)` | `addBillingReference(reference)` | `AddBillingReference(reference)` | `addBillingReference(reference)` |
| Append payment entry | `add_payment_means(payment)` | `addPaymentMeans(payment)` | `AddPaymentMeans(payment)` | `addPaymentMeans(payment)` |

Document allowance and charge arguments are `(adjustment, category, rate, exemption)`,
mirroring Rust `allowance` and `charge`. In Python the category, rate and exemption
default to the fields of the `Adjustment` value, and the exemption to empty metadata;
the other bindings require a `TaxExemption` object. Category values and decimal
representations follow the language reference below.

For a prepayment adjustment line, use `add_document_reference` and
`add_tax_subtotal` on the Python line builder (`addDocumentReference` and
`addTaxSubtotal` in JavaScript/Kotlin/Java; PascalCase in C#).
`OriginalInvoiceRef` constructors accept ID, UUID, date, time, and document type
code in that order. Set the last argument to `"386"`; in Python it is the optional
`document_type_code` keyword. The [prepayment recipe](../guides/invoice-recipes.md#prepayments-and-final-payable-amounts)
shows which line amounts must be zero and how to supply the advance VAT subtotal.

For C, use `fatoora_line_item_builder_new(&line, &error)` and
`fatoora_line_item_builder_build(line, &item, &error)`. The setters above use the
`fatoora_line_item_builder_` prefix with snake_case names. Invoice operations use
`fatoora_invoice_builder_`, including `add_item`, `add_allowance`, `add_charge`, and
`set_prepaid_amount`. Each accepts the builder first and `FatooraError **` last,
and returns a status. Document allowances and charges take an adjustment handle,
`FatooraVatCategory category`, decimal rate text, and exemption handle. Free owned handles after
use, including consumed builders. See [C ownership](bindings/c.md).

### Python line discount example

In the [Python creation example](../guides/invoice-creation.md), build this line
first and replace the `.add_line_item(...)` call with `.add_item(line)`. It
produces the same SAR 276 payable amount as the Rust price-per-pack recipe:

```python
from fatoora import Adjustment, LineItemBuilder

line = (
    LineItemBuilder()
    .set_item_name("Notebook")
    .set_invoiced_quantity("25")
    .set_invoiced_quantity_unit_code("PCE")
    .set_item_net_price("100")
    .set_item_price_base_quantity("10")
    .set_vat_category(VatCategory.STANDARD)
    .set_vat_rate("15")
    .add_allowance(Adjustment("10", reason="Volume discount"))
    .build()
)
```

For a document discount instead, keep the creation example's original line and
add this call before `.build()`. The payable amount becomes SAR 103.50:

```python
    .add_allowance(Adjustment("10", reason="Loyalty discount"),
                   VatCategory.STANDARD, "15")
```

## Language reference

Choose your language for callable signatures and lifecycle rules. Argument names
correspond to the fields above.

=== "Rust"

    **Create and configure**

    Import model types from `fatoora_core::invoice`. `InvoiceBuilder::new(invoice_type: InvoiceType) -> InvoiceBuilder` accepts `InvoiceType::Tax(subtype)`, `Prepayment(subtype)`, `CreditNote(subtype)`, or `DebitNote(subtype)`. Subtype is `InvoiceSubType::Standard` or `Simplified`; add references with `.billing_reference(OriginalInvoiceRef::new(id))` and a reason with `.payment_instruction(reason)`. References support `OriginalInvoiceRef::new(id)` with optional `with_uuid(uuid: InvoiceUuid)`, `with_issue_date(date: InvoiceDate)` and `with_issue_time(time: InvoiceTime)`.

    Every setter consumes and returns `Self`: chain calls or reassign the result.

    | Call | Returns |
    | --- | --- |
    | `id(value: impl Into<String>)` | `Self` |
    | `uuid(value: InvoiceUuid)` | `Self` |
    | `issue_datetime(value: InvoiceTimestamp)` | `Self` |
    | `currency(value: CurrencyCode)` | `Self` |
    | `previous_invoice_hash(value: InvoiceHash)` | `Self` |
    | `invoice_counter(value: u64)` | `Self` |
    | `seller(value: Seller)` | `Self` |
    | `buyer(value: Buyer)` | `Self` |
    | `vat_category(value: VatCategory)` | `Self` |
    | `flags(value: InvoiceFlags)` | `Self` |
    | `note(value: InvoiceNote)` | `Self` |
    | `line_item(value: LineItem)` | `Self` |
    | `allowance(value: DocumentAdjustment)` | `Self` |
    | `charge(value: DocumentAdjustment)` | `Self` |
    | `billing_reference(value: OriginalInvoiceRef)` | `Self` |
    | `payment_instruction(value: impl Into<String>)` | `Self` |
    | `payment_means(value: PaymentMeans)` | `Self` |
    | `purchase_order_id(value: impl Into<String>)`, `contract_id(value: impl Into<String>)` | `Self` |
    | `delivery(value: Delivery)` | `Self` |
    | `prepaid_amount(value: Decimal)`, `payable_rounding_amount(value: Decimal)` | `Self` |
    | `tax_currency(currency: CurrencyCode, amount: Decimal)` | `Self` |
    | `tax_exemption(category: VatCategory, rate: Decimal, exemption: TaxExemption)` | `Self` |
    | `vat_breakdown(value: VatBreakdown)` | `Self` |
    | `build(self)` | `Result<FinalizedInvoice, InvoiceError>` |

    `payment_means` appends a payment means; use `.payment_means(PaymentMeans::new(code))` to set the payment means code. Add document-level allowances and charges with `.allowance(DocumentAdjustment)` and `.charge(DocumentAdjustment)`; `DocumentAdjustment` carries the amount or percentage calculation, `with_reason`, `with_reason_code`, VAT category and rate, `with_exemption`, and `omit_vat_rate()` for out-of-scope entries.

    Deprecated since 0.6.0 and removed in 0.7: `payment_means_code` (changes the code of the first payment means), `invoice_level_charge`, `invoice_level_discount` and `allowance_reason` (one aggregate allowance and charge in the document VAT category), and `adjustment(AllowanceCharge)` (convert with `DocumentAdjustment::from`). `VatBreakdown::without_vat_rate` is replaced by `omit_vat_rate`; `LineItem`, `VatBreakdown`, `AllowanceCharge` and `DocumentAdjustment` all report an omitted rate through `has_vat_rate()`.

    `build(self)` consumes the builder on success or failure. `VatCategory` variants are `Standard`, `Zero`, `Exempt`, and `OutOfScope`. Combine `InvoiceFlags::THIRD_PARTY`, `NOMINAL`, `EXPORT`, `SUMMARY`, and `SELF_BILLED` with `\|`.

    **Supporting values**

    `LineItem::new(description, quantity, unit_code, unit_price, vat_rate, vat_category)` takes text values for description/unit code, `Decimal` for quantities/amounts/rate, and `VatCategory`; it returns `Result<LineItem, InvoiceError>`. `LineItem::from_totals(...)` adds an explicit `total_amount` after unit price; `try_from_parts(...)` also adds `vat_amount` after VAT rate and checks both supplied amounts.

    Use `CountryCode::parse`, `CurrencyCode::parse`, `InvoiceTimestamp::parse`, `InvoiceDate::parse`, `InvoiceTime::parse`, `InvoiceUuid::parse`, `InvoiceHash::parse`, and `VatId::parse` for validated wrappers; each returns `Result<Wrapper, InvoiceError>`, and each wrapper also implements `FromStr` and `TryFrom<&str>`. With the `chrono` feature, `InvoiceTimestamp` and `InvoiceDate` convert from chrono values via `From`; with the `uuid` feature, `InvoiceUuid` converts from `uuid::Uuid`. `InvoiceNote::new(language, text)` returns a note. `OtherId::new(value)` creates an ID without a scheme; `OtherId::with_scheme(value, scheme_id)` supplies both.

    Construct `Address` with `Address::new(country_code: CountryCode, city, street, building_number, postal_code)` (text as `impl Into<String>`) and add the optional parts with `with_additional_street`, `with_additional_number`, `with_district` and `with_province`; read them back with the getters of the same names.

    `TaxExemption`, `PaymentAccount` and `Delivery` start from `Default` and take `with_*` setters (`with_reason_code`/`with_reason`, `with_payment_account_identifier`/`with_payment_terms`, `with_actual_date`/`with_latest_date`); `PaymentMeans::new(code)` takes `with_instruction_note` and `with_payee_financial_account`, and `code()` returns the code. Line and document adjustments use `Adjustment::amount(amount)` or `Adjustment::percent_of(base, percent)` with `with_reason`, `with_reason_code` and `with_base_amount`; `DocumentAdjustment::new(adjustment, category, rate)` adds `with_reason`, `with_reason_code`, `with_exemption` and `with_tax_category_count`. Fields are private; each has a getter without a `get_` prefix.

    `Seller::new(name: impl Into<String>, address: Address, vat_id: VatId, other_id: Option<OtherId>) -> Seller` requires a VAT ID. `Buyer::new(name: impl Into<String>, address: impl Into<Option<Address>>, vat_id: Option<VatId>, other_id: Option<OtherId>) -> Buyer` accepts an absent address and identifiers; business validation checks applicability.

    `LineItem::builder()` adds price-base quantities, gross pricing, line allowances
    and charges. It also accepts supplied amounts and supports item identifiers
    as well as prepayment references.
    See [pricing recipes](../guides/invoice-recipes.md) for construction examples
    and [model coverage](../development/invoice-model.md) for preservation rules.

    **Read invoice data**

    Both invoice types provide `data() -> &InvoiceData`, `totals() -> &InvoiceTotals`, and `hash() -> Result<InvoiceHash, SigningError>` (the deprecated `hash_base64()` returns the same value as `String`). Read fields through the returned data's accessor methods, including `id()`, `uuid() -> &InvoiceUuid`, `issue_date() -> InvoiceDate`, `issue_time() -> InvoiceTime`, `previous_invoice_hash() -> &InvoiceHash`, `seller()`, `buyer()`, `line_items()`, `flags()`, and `invoice_type()`. Decimal accessors return `Decimal`; optional fields return `Option`.

    `FinalizedInvoice::sign(self, signer: &InvoiceSigner) -> Result<SignedInvoice, SigningError>` consumes the finalized invoice even if signing fails. Signed metadata and XML access are described in [signing](invoice-signing.md) and [XML](xml.md).

=== "Python"

    **Create and configure**

    Import from `fatoora`. Enum values are `InvoiceTypeKind.TAX`, `PREPAYMENT`, `CREDIT_NOTE`, `DEBIT_NOTE`; `InvoiceSubType.STANDARD` / `SIMPLIFIED`; and `VatCategory.EXEMPT`, `STANDARD`, `ZERO`, `OUT_OF_SCOPE`. Decimal inputs accept `decimal.Decimal`, a decimal string, or an integer; binary floats are rejected. Timestamps accept a timezone-aware `datetime` (converted to UTC) or `"YYYY-MM-DDTHH:MM:SSZ"` text; dates accept `datetime.date` or `"YYYY-MM-DD"`; UUIDs accept `uuid.UUID` or text.

    ```python
    InvoiceBuilder(
        invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, *,
        id: str | None = None, uuid: UUID | str | None = None,
        issue_datetime: datetime | str | None = None, currency: str | None = None,
        previous_invoice_hash: str | None = None, invoice_counter: int | None = None,
        vat_category: VatCategory | None = None, flags: int | InvoiceFlag | None = None,
        original_invoice: OriginalInvoiceRef | None = None,
        original_invoice_reason: str | None = None,
        seller: Party | None = None, buyer: Party | None = None,
    )
    ```

    Pass the header fields as keyword arguments, call the setters, or mix both. Every setter returns the builder, so calls chain: `InvoiceBuilder(...).set_seller(...).add_line_item(...).build()`.

    | Call | Notes |
    | --- | --- |
    | `set_id(invoice_id: str)` | |
    | `set_uuid(uuid: UUID \| str)` | |
    | `set_issue_datetime(issue_datetime: datetime \| str)` | Naive `datetime` raises `ValueError`. |
    | `set_currency(currency_code: str)` | |
    | `set_previous_invoice_hash(previous_invoice_hash: str)` | |
    | `set_invoice_counter(invoice_counter: int)` | |
    | `set_vat_category(vat_category: VatCategory)` | |
    | `set_flags(flags: int \| InvoiceFlag)` | |
    | `set_seller(party: Party)` | The seller needs an address and a VAT ID. The first of `party.identifiers` is the other ID; the rest are additional identifiers. |
    | `set_seller(name: str, *, vat_id: str, address: Address \| None = None, country_code=..., city=..., street=..., building_number=..., postal_code=..., additional_street=None, additional_number=None, district=None, province=None, other_id=None, other_id_scheme=None)` | Pass `address=Address(...)` or the address fields, not both. |
    | `set_buyer(party: Party)` | The buyer's address and VAT ID are optional. |
    | `set_buyer(name: str, *, address: Address \| None = None, vat_id=None, other_id=None, other_id_scheme=None, ...address fields...)` | As `set_seller`; `vat_id` is optional. |
    | `set_buyer_without_address(name: str, vat_id=None, other_id=None, scheme=None)` | |
    | `add_line_item(description: str, quantity, unit_code: str, unit_price, vat_rate, vat_category: VatCategory, *, id=None, exemption=TaxExemption(), document_references=(), tax_subtotals=())` | `document_references` holds `OriginalInvoiceRef` values. |
    | `add_item(item: InvoiceLineItem)` | `total_amount` and `vat_amount` are calculated when `None` and checked when given. |
    | `add_note(text: str, *, language: str \| None = None)` | Appends a note. |
    | `add_payment_means(value: PaymentMeans)` | Appends a payment means. |
    | `add_allowance(value: Adjustment, category: VatCategory \| None = None, rate=None, exemption=None)` | Document-level allowance (discount). Omitted arguments come from the `Adjustment`. |
    | `add_charge(value: Adjustment, category: VatCategory \| None = None, rate=None, exemption=None)` | Document-level charge. |
    | `add_adjustment(value: Adjustment)` | Allowance or charge described entirely by the value (`charge`, `category`, `rate`, tax category count, omitted rate), such as one read from `invoice.adjustments`. |
    | `add_tax_exemption(category: VatCategory, rate, exemption: TaxExemption)` | |
    | `set_delivery(actual: date \| str \| None = None, latest: date \| str \| None = None)` | |
    | `build()` | Returns `FinalizedInvoice`. |

    `build()` consumes the builder's state on success or failure. Rejected setter inputs leave it available for correction. Close owned objects with `close()` or use a `with` block; closing a consumed builder is safe.

    **Read invoice data**

    `FinalizedInvoice` and `SignedInvoice` expose the invoice data as read-only properties:

    | Property | Value |
    | --- | --- |
    | `id`, `currency`, `previous_invoice_hash`, `payment_means_code` | `str` |
    | `uuid` | `str` (core keeps the UUID text verbatim) |
    | `issue_datetime` | timezone-aware `datetime` (UTC) |
    | `issue_date` | `date` |
    | `issue_time` | `str`, `HH:MM:SS` (AST) or `HH:MM:SSZ` (UTC) as written |
    | `invoice_counter`, `line_item_count`, `flags_raw` | `int` |
    | `vat_category` | `VatCategory` |
    | `invoice_type_kind` / `invoice_sub_type` | `InvoiceTypeKind` / `InvoiceSubType` |
    | `seller` / `buyer` | `Party` / `Optional[Party]` |
    | `notes`, `payment_means`, `billing_references` | lists of `InvoiceNote`, `PaymentMeans`, `OriginalInvoiceRef` |
    | `original_invoice_ref` / `original_invoice_reason` | `Optional[OriginalInvoiceRef]` / `Optional[str]` |
    | `actual_delivery_date` / `latest_delivery_date` | `Optional[date]` |
    | `line_items` | `list[InvoiceLineItem]` |
    | `adjustments` / `vat_breakdown` | `list[Adjustment]` / `list[VatBreakdown]` |
    | `totals` | `InvoiceTotals` |
    | `flags` | `set[InvoiceFlag]` |
    | `xml` | `str` |

    Methods: `line_item(index: int) -> InvoiceLineItem`, `hash() -> str` (recomputes the base64 invoice hash), and the queries `is_third_party()`, `is_nominal()`, `is_export()`, `is_summary()`, `is_self_billed()` and `is_simplified()`.

    The invoice data is read from the native invoice once, on first access, and cached; invoices do not change after `build()` or parsing.

    **Value models**

    Everything read from an invoice is a frozen dataclass: compare values with `==`, copy them with `dataclasses.replace`, and pass them back to the builder. They need no `close()`. Optional fields are `None` when absent.

    | Model | Fields |
    | --- | --- |
    | `Party(name, address=None, vat_id=None, identifiers=())` | `vat_id` and `identifiers` hold `Identifier` values (strings are accepted as input); `other_id` is the first identifier. |
    | `Identifier(value, scheme=None)` | An identifier and its `schemeID`, such as `Identifier("7003339333", "CRN")`. |
    | `Address(country_code, city, street, building_number, postal_code, additional_street=None, additional_number=None, district=None, province=None)` | Validated on construction; the country code becomes ISO alpha-3 (`"SA"` reads back as `"SAU"`). |
    | `InvoiceNote(text, language_id=None)` | `language` is `language_id` or `"en"`. |
    | `OriginalInvoiceRef(id, uuid=None, issue_date=None, issue_time=None, document_type_code=None)` | Billing references and line document references. `issue_date` is a `date`. |
    | `PaymentMeans(code, instruction_notes=(), account_identifier=None, payment_terms=None)` | `with_instruction_note(note)` and `with_account(id, terms)` return copies. |
    | `Adjustment(amount, reason=None, reason_code=None, base_amount=None, percentage=None, charge=False, category=None, rate=None, exemption=TaxExemption(), tax_category_count=1, omitted_rate=False)` | `Adjustment.percent_of(base, percent, **fields)` calculates `amount`. Line adjustments read back with their line's category and rate. |
    | `InvoiceLineItem(description, unit_code, quantity, unit_price, total_amount=None, vat_rate, vat_amount=None, vat_category, ...)` | `vat_rate` and `vat_category` are required; pass them by keyword when you leave out `total_amount`. |

    `FinalizedInvoice.sign(signer: InvoiceSigner) -> SignedInvoice` consumes the finalized invoice on success or signing failure; the signer remains reusable. See [signing](invoice-signing.md) for metadata and [XML](xml.md) for serialization.

    **Deprecated in 0.6 (removed in 0.7)**

    Getters became properties as a hard break in 0.6: drop the parentheses (`invoice.id()` becomes `invoice.id`). `InvoiceBuilder.new(...)` and `new_with_reference(...)` become the `InvoiceBuilder(...)` constructor; `set_note(language, text)` and `set_note_without_language(text)` become `add_note(text, language=...)`; `set_payment_means_code(code)` becomes `add_payment_means(PaymentMeans(code))`; `flags(value)` becomes `set_flags(value)`; `set_tax_exemption` becomes `add_tax_exemption`; `add_document_adjustment(charge, ...)` becomes `add_allowance(...)` / `add_charge(...)`. The aggregate setters `set_allowance`, `invoice_level_discount`, `invoice_level_charge` and `allowance_reason` keep their 0.5 behaviour (one allowance and one charge in the document VAT category, at the single VAT rate of that category's lines) but are deprecated in favour of `add_allowance` / `add_charge` with an explicit category and rate. The value models replace `Address.new`, `PaymentMeans.new` and `OriginalInvoiceRef.new` with their constructors, `AllowanceCharge` with `Adjustment` (`Adjustment.amount(value)` becomes `Adjustment(value)` and `Adjustment.percentage(base, percent)` becomes `Adjustment.percent_of(base, percent)`), `DocumentReference` with `OriginalInvoiceRef`, and `VatId`/`OtherId` and `(value, scheme)` tuples with `Identifier`. See the [Python migration table](bindings/python.md#migrate-from-05).

=== "JavaScript"

    Import from `fatoority`. All decimal arguments and amounts are strings; counters use `bigint`, indexes and the flag mask use `number`, and codes are enum classes. Pass `null` for absent optional arguments. Operations are synchronous and throw a `FatooraError` subclass on failure. Returned `Text` objects expose `.value()`.

    Enums: `InvoiceTypeKind` (`Tax`, `Prepayment`, `CreditNote`, `DebitNote`), `InvoiceSubType` (`Standard`, `Simplified`), and `VatCategory` (`Exempt`, `Standard`, `Zero`, `OutOfScope`); pass them as `InvoiceTypeKind.Tax`, `VatCategory.Standard`, and so on. A returned enum's `.value` is the variant name, such as `'Standard'`. Flags remain a `number` bitmask; build it from `InvoiceFlag` bits, for example `InvoiceFlag.Export.bit() | InvoiceFlag.Summary.bit()` (`1` third party, `2` nominal, `4` export, `8` summary, `16` self-billed). Use zero for no flags.

    **Create and configure**

    | Call | Returns |
    | --- | --- |
    | `create(kind: InvoiceTypeKind, subtype: InvoiceSubType, originalId, originalUuid, originalDate, reason)` | `InvoiceBuilder` |
    | `setId(value)` | `void` |
    | `setUuid(value)` | `void` |
    | `setIssueDatetime(value)` | `void` |
    | `setCurrency(value)` | `void` |
    | `setPreviousInvoiceHash(value)` | `void` |
    | `addPaymentMeans(payment)` | `void` |
    | `setInvoiceCounter(value)` | `void` |
    | `setVatCategory(value: VatCategory)` | `void` |
    | `setFlags(value)` | `void` |
    | `addNote(language, value)` | `void` |
    | `setSeller(name, address, vatId, otherId, scheme)` | `void` |
    | `setBuyer(name, address, vatId, otherId, scheme)` | `void` |
    | `addLineItem(description, quantity, unitCode, unitPrice, vatRate, category: VatCategory)` | `void` |
    | `build()` | `FinalizedInvoice` |

    **Address inputs**

    | Call | Returns |
    | --- | --- |
    | `create(countryCode, city, street, buildingNumber, postalCode, additionalStreet, additionalNumber, district)` | `Address` |

    Setters replace values; line-item calls append. Building consumes builder state even on validation failure. Signing through the signer consumes finalized invoice state. Further calls on consumed state fail.

    **Read invoice data**

    `invoice.data()` returns an owned `InvoiceData` snapshot; `invoice.totals()` returns owned `InvoiceTotals`. Both are independent of invoice state.

    | Call | Returns |
    | --- | --- |
    | `id()` | `string` |
    | `uuid()` | `string` |
    | `previousInvoiceHash()` | `string` |
    | `paymentMeansCode()` | `string` |
    | `currency()` | `string` |
    | `issueDatetime()` | `string` |
    | `invoiceLevelCharge()` | `string` |
    | `invoiceLevelDiscount()` | `string` |
    | `allowanceReason()` | `Text \| null` |
    | `invoiceCounter()` | `bigint` |
    | `vatCategory()` | `VatCategory` |
    | `flags()` | `number` |
    | `invoiceTypeKind()` | `InvoiceTypeKind` |
    | `invoiceSubType()` | `InvoiceSubType` |
    | `seller()` | `Party` |
    | `buyer()` | `Party \| null` |
    | `note()` | `InvoiceNote \| null` |
    | `lineItemsLen()` | `number` |
    | `lineItem(index)` | `InvoiceLineItem` |
    | `originalInvoiceRef()` | `OriginalInvoiceRef \| null` |
    | `originalInvoiceReason()` | `Text \| null` |

    **Totals**

    | Call | Returns |
    | --- | --- |
    | `taxInclusive()` | `string` |
    | `taxAmount()` | `string` |
    | `lineExtension()` | `string` |
    | `allowanceTotal()` | `string` |
    | `chargeTotal()` | `string` |
    | `taxableAmount()` | `string` |
    | `prepaidAmount()` | `string` |
    | `payableRoundingAmount()` | `string` |
    | `payableAmount()` | `string` |

    Optional object accessors return null when absent. Line-item objects provide description, quantity, unit code, unit price, total amount, VAT rate, VAT amount, and category. See [XML](xml.md) and [signing](invoice-signing.md) for document and signature operations.

=== "C#"

    Use the `Fatoora` namespace. Quantities, prices, rates and amounts are `decimal` in both directions; counters use `ulong`, indexes use `nuint`, codes use C# enums, and the flag mask uses `byte`. Pass `null` for absent optional arguments. Members throw on failure. Scalar getters are read-only properties and optional text is `string?`; getters returning owned objects stay methods. Dispose each owned object with `using`.

    Enums: `InvoiceTypeKind` (`Tax`, `Prepayment`, `CreditNote`, `DebitNote`), `InvoiceSubType` (`Standard`, `Simplified`), and `VatCategory` (`Exempt`, `Standard`, `Zero`, `OutOfScope`); pass them as `InvoiceTypeKind.Tax`, `VatCategory.Standard`, and so on. Flags remain a `byte` bitmask; build it by casting `InvoiceFlag` values, for example `(byte)(InvoiceFlag.Export | InvoiceFlag.Summary)` (`1` third party, `2` nominal, `4` export, `8` summary, `16` self-billed). Use zero for no flags.

    **Create and configure**

    | Call | Returns |
    | --- | --- |
    | `New(InvoiceTypeKind kind, InvoiceSubType subtype, string? originalId, string? originalUuid, string? originalDate, string? reason)` | `InvoiceBuilder` |
    | `SetId(string value)` | `void` |
    | `SetUuid(string value)` | `void` |
    | `SetIssueDatetime(string value)` | `void` |
    | `SetCurrency(string value)` | `void` |
    | `SetPreviousInvoiceHash(string value)` | `void` |
    | `AddPaymentMeans(PaymentMeans payment)` | `void` |
    | `SetInvoiceCounter(ulong value)` | `void` |
    | `SetVatCategory(VatCategory value)` | `void` |
    | `SetFlags(byte value)` | `void` |
    | `AddNote(string? language, string value)` | `void` |
    | `SetSeller(string name, Address address, string vatId, string? otherId, string? scheme)` | `void` |
    | `SetBuyer(string name, Address address, string? vatId, string? otherId, string? scheme)` | `void` |
    | `AddLineItem(string description, decimal quantity, string unitCode, decimal unitPrice, decimal vatRate, VatCategory category)` | `void` |
    | `Build()` | `FinalizedInvoice` |

    **Address inputs**

    | Call | Returns |
    | --- | --- |
    | `New(string countryCode, string city, string street, string buildingNumber, string postalCode, string? additionalStreet, string? additionalNumber, string? district)` | `Address` |

    Setters replace values; line-item calls append. Building consumes builder state even on validation failure. Signing through the signer consumes finalized invoice state. Further calls on consumed state fail.

    **Read invoice data**

    `invoice.Data()` returns an owned `InvoiceData` snapshot; `invoice.Totals()` returns owned `InvoiceTotals`. Both survive disposal of the invoice.

    | Member | Returns |
    | --- | --- |
    | `Id` | `string` |
    | `Uuid` | `string` |
    | `PreviousInvoiceHash` | `string` |
    | `PaymentMeansCode` | `string` |
    | `Currency` | `string` |
    | `IssueDatetime` | `string` |
    | `InvoiceLevelCharge` | `decimal` |
    | `InvoiceLevelDiscount` | `decimal` |
    | `AllowanceReason` | `string?` |
    | `InvoiceCounter` | `ulong` |
    | `VatCategory` | `VatCategory` |
    | `Flags` | `byte` |
    | `InvoiceTypeKind` | `InvoiceTypeKind` |
    | `InvoiceSubType` | `InvoiceSubType` |
    | `Seller()` | `Party` |
    | `Buyer()` | `Party?` |
    | `Note()` | `InvoiceNote?` |
    | `LineItemsLen` | `nuint` |
    | `LineItem(nuint index)` | `InvoiceLineItem` |
    | `OriginalInvoiceRef()` | `OriginalInvoiceRef?` |
    | `OriginalInvoiceReason` | `string?` |

    **Totals**

    All totals are `decimal` properties: `TaxInclusive`, `TaxAmount`,
    `LineExtension`, `AllowanceTotal`, `ChargeTotal`, `TaxableAmount`,
    `PrepaidAmount`, `PayableRoundingAmount` and `PayableAmount`.

    Optional object accessors return null when absent. Line-item objects provide description, quantity, unit code, unit price, total amount, VAT rate, VAT amount, and category. See [XML](xml.md) and [signing](invoice-signing.md) for document and signature operations.

=== "Kotlin"

    Use `com.fatoora`. Quantities, prices, rates and amounts are exact `java.math.BigDecimal` values; counters and indexes use `java.math.BigInteger`, codes use `com.fatoora` enums, and the flag mask uses `Int`. Optional values are nullable. Getters that return copied values are properties; getters that return native owners stay methods. Methods throw on failure. Close owned objects with `use`.

    Enums: `InvoiceTypeKind` (`Tax`, `Prepayment`, `CreditNote`, `DebitNote`), `InvoiceSubType` (`Standard`, `Simplified`), and `VatCategory` (`Exempt`, `Standard`, `Zero`, `OutOfScope`) in `com.fatoora`; pass them as `InvoiceTypeKind.Tax`, `VatCategory.Standard`, and so on. Flags remain an `Int` bitmask; build it from `InvoiceFlag` bits, for example `InvoiceFlag.Export.bit or InvoiceFlag.Summary.bit` (`1` third party, `2` nominal, `4` export, `8` summary, `16` self-billed). Use zero for no flags.

    **Create and configure**

    | Call | Returns |
    | --- | --- |
    | `create(kind: InvoiceTypeKind, subtype: InvoiceSubType, originalId: String? = null, originalUuid: String? = null, originalDate: String? = null, reason: String? = null)` | `InvoiceBuilder` |
    | `setId(value: String)` | `Unit` |
    | `setUuid(value: String)` | `Unit` |
    | `setIssueDatetime(value: String)` | `Unit` |
    | `setCurrency(value: String)` | `Unit` |
    | `setPreviousInvoiceHash(value: String)` | `Unit` |
    | `addPaymentMeans(payment: PaymentMeans)` | `Unit` |
    | `setInvoiceCounter(value: java.math.BigInteger)` | `Unit` |
    | `setVatCategory(value: VatCategory)` | `Unit` |
    | `setFlags(value: Int)` | `Unit` |
    | `addNote(language: String?, value: String)` | `Unit` |
    | `setSeller(name: String, address: Address, vatId: String, otherId: String? = null, scheme: String? = null)` | `Unit` |
    | `setBuyer(name: String, address: Address, vatId: String? = null, otherId: String? = null, scheme: String? = null)` | `Unit` |
    | `addLineItem(description: String, quantity: BigDecimal, unitCode: String, unitPrice: BigDecimal, vatRate: BigDecimal, category: VatCategory)` | `Unit` |
    | `build()` | `FinalizedInvoice` |

    **Address inputs**

    | Call | Returns |
    | --- | --- |
    | `create(countryCode: String, city: String, street: String, buildingNumber: String, postalCode: String, additionalStreet: String? = null, additionalNumber: String? = null, district: String? = null)` | `Address` |

    Setters replace values; line-item calls append. Building consumes builder state even on validation failure. Signing through the signer consumes finalized invoice state. Further calls on consumed state fail.

    **Read invoice data**

    `invoice.data()` returns an owned `InvoiceData` snapshot; `invoice.totals()` returns owned `InvoiceTotals`. Both are independent of invoice state.

    | Call | Returns |
    | --- | --- |
    | `id` | `String` |
    | `uuid` | `String` |
    | `previousInvoiceHash` | `String` |
    | `paymentMeansCode` | `String` |
    | `currency` | `String` |
    | `issueDatetime` | `String` |
    | `invoiceLevelCharge` | `BigDecimal` |
    | `invoiceLevelDiscount` | `BigDecimal` |
    | `allowanceReason` | `String?` |
    | `invoiceCounter` | `java.math.BigInteger` |
    | `vatCategory` | `VatCategory` |
    | `flags` | `Int` |
    | `invoiceTypeKind` | `InvoiceTypeKind` |
    | `invoiceSubType` | `InvoiceSubType` |
    | `seller()` | `Party` |
    | `buyer()` | `Party?` |
    | `note()` | `InvoiceNote?` |
    | `lineItemsLen` | `java.math.BigInteger` |
    | `lineItem(index: java.math.BigInteger)` | `InvoiceLineItem` |
    | `originalInvoiceRef()` | `OriginalInvoiceRef?` |
    | `originalInvoiceReason` | `String?` |

    **Totals**

    | Call | Returns |
    | --- | --- |
    | `taxInclusive` | `BigDecimal` |
    | `taxAmount` | `BigDecimal` |
    | `lineExtension` | `BigDecimal` |
    | `allowanceTotal` | `BigDecimal` |
    | `chargeTotal` | `BigDecimal` |
    | `taxableAmount` | `BigDecimal` |
    | `prepaidAmount` | `BigDecimal` |
    | `payableRoundingAmount` | `BigDecimal` |
    | `payableAmount` | `BigDecimal` |

    Optional object accessors return null when absent. Line-item objects provide description, quantity, unit code, unit price, total amount, VAT rate, VAT amount, and category. See [XML](xml.md) and [signing](invoice-signing.md) for document and signature operations.

=== "Java"

    Import `com.fatoora.*`. Quantities, prices, rates and amounts use `java.math.BigDecimal`; counters and indexes use `BigInteger`, codes use `com.fatoora` enums, and the flag mask uses `int`. Optional values accept `null`. Call static `InvoiceBuilder.create(...)` and `Address.create(...)` with all arguments, including optional nulls. Close owned objects with try-with-resources. Value getters use `getX()`, `isX()` or `hasX()`. The signatures below use Java parameter and return types.

    Enums: `InvoiceTypeKind` (`Tax`, `Prepayment`, `CreditNote`, `DebitNote`), `InvoiceSubType` (`Standard`, `Simplified`), and `VatCategory` (`Exempt`, `Standard`, `Zero`, `OutOfScope`) in `com.fatoora`; pass them as `InvoiceTypeKind.Tax`, `VatCategory.Standard`, and so on. Flags remain an `int` bitmask; build it from `InvoiceFlag` bits, for example `InvoiceFlag.Export.getBit() | InvoiceFlag.Summary.getBit()` (`1` third party, `2` nominal, `4` export, `8` summary, `16` self-billed). Use zero for no flags.

    **Create and configure**

    | Call | Returns |
    | --- | --- |
    | `create(InvoiceTypeKind kind, InvoiceSubType subtype, String originalId, String originalUuid, String originalDate, String reason)` | `InvoiceBuilder` |
    | `setId(String value)` | `void` |
    | `setUuid(String value)` | `void` |
    | `setIssueDatetime(String value)` | `void` |
    | `setCurrency(String value)` | `void` |
    | `setPreviousInvoiceHash(String value)` | `void` |
    | `addPaymentMeans(PaymentMeans payment)` | `void` |
    | `setInvoiceCounter(BigInteger value)` | `void` |
    | `setVatCategory(VatCategory value)` | `void` |
    | `setFlags(int value)` | `void` |
    | `addNote(String language, String value)` | `void` |
    | `setSeller(String name, Address address, String vatId, String otherId, String scheme)` | `void` |
    | `setBuyer(String name, Address address, String vatId, String otherId, String scheme)` | `void` |
    | `addLineItem(String description, BigDecimal quantity, String unitCode, BigDecimal unitPrice, BigDecimal vatRate, VatCategory category)` | `void` |
    | `build()` | `FinalizedInvoice` |

    **Address inputs**

    | Call | Returns |
    | --- | --- |
    | `create(String countryCode, String city, String street, String buildingNumber, String postalCode, String additionalStreet, String additionalNumber, String district)` | `Address` |

    Setters replace values; line-item calls append. Building consumes builder state even on validation failure. Signing through the signer consumes finalized invoice state. Further calls on consumed state fail.

    **Read invoice data**

    `invoice.data()` returns an owned `InvoiceData` snapshot; `invoice.totals()` returns owned `InvoiceTotals`. Both are independent of invoice state.

    | Call | Returns |
    | --- | --- |
    | `getId()` | `String` |
    | `getUuid()` | `String` |
    | `getPreviousInvoiceHash()` | `String` |
    | `getPaymentMeansCode()` | `String` |
    | `getCurrency()` | `String` |
    | `getIssueDatetime()` | `String` |
    | `getInvoiceLevelCharge()` | `BigDecimal` |
    | `getInvoiceLevelDiscount()` | `BigDecimal` |
    | `getAllowanceReason()` | `String` |
    | `getInvoiceCounter()` | `BigInteger` |
    | `getVatCategory()` | `VatCategory` |
    | `getFlags()` | `int` |
    | `getInvoiceTypeKind()` | `InvoiceTypeKind` |
    | `getInvoiceSubType()` | `InvoiceSubType` |
    | `seller()` | `Party` |
    | `buyer()` | `Party` |
    | `note()` | `InvoiceNote` |
    | `getLineItemsLen()` | `BigInteger` |
    | `lineItem(BigInteger index)` | `InvoiceLineItem` |
    | `originalInvoiceRef()` | `OriginalInvoiceRef` |
    | `getOriginalInvoiceReason()` | `String` |

    **Totals**

    | Call | Returns |
    | --- | --- |
    | `getTaxInclusive()` | `BigDecimal` |
    | `getTaxAmount()` | `BigDecimal` |
    | `getLineExtension()` | `BigDecimal` |
    | `getAllowanceTotal()` | `BigDecimal` |
    | `getChargeTotal()` | `BigDecimal` |
    | `getTaxableAmount()` | `BigDecimal` |
    | `getPrepaidAmount()` | `BigDecimal` |
    | `getPayableRoundingAmount()` | `BigDecimal` |
    | `getPayableAmount()` | `BigDecimal` |

    Optional object accessors return null when absent. Line-item objects provide description, quantity, unit code, unit price, total amount, VAT rate, VAT amount, and category. See [XML](xml.md) and [signing](invoice-signing.md) for document and signature operations.

=== "C"

    Include `fatoora.h`. Functions return `FatooraStatus`; read output pointers only after success. On failure, inspect and free `FatooraError`. Release returned strings with `fatoora_string_free()` and objects with their matching `_free()` function. Optional input strings use `NULL`, not empty strings.

    Enums: `FatooraInvoiceTypeKind` (`FATOORA_TAX`, `FATOORA_PREPAYMENT`, `FATOORA_CREDIT_NOTE`, `FATOORA_DEBIT_NOTE`), `FatooraInvoiceSubType` (`FATOORA_STANDARD`, `FATOORA_SIMPLIFIED`), and `FatooraVatCategory` (`FATOORA_VAT_EXEMPT`, `FATOORA_VAT_STANDARD`, `FATOORA_VAT_ZERO`, `FATOORA_VAT_OUT_OF_SCOPE`). The numeric values match the earlier codes (`0` tax, `1` prepayment, ...); a value that is not a declared enumerator returns `FATOORA_INVALID_INPUT`. Flags remain an integer bitmask; combine `FatooraInvoiceFlag` constants `FATOORA_FLAG_THIRD_PARTY` (`1`), `FATOORA_FLAG_NOMINAL` (`2`), `FATOORA_FLAG_EXPORT` (`4`), `FATOORA_FLAG_SUMMARY` (`8`), and `FATOORA_FLAG_SELF_BILLED` (`16`). Use zero for no flags.

    **Create and configure**

    ```c
    FatooraStatus fatoora_invoice_builder_new(
        const FatooraInvoiceOptions *options,
        FatooraInvoiceBuilder **out, FatooraError **error);
    FatooraStatus fatoora_invoice_builder_build(
        FatooraInvoiceBuilder *self,
        FatooraFinalizedInvoice **out, FatooraError **error);
    ```

    `FatooraInvoiceOptions` contains `FatooraInvoiceTypeKind kind`, `FatooraInvoiceSubType subtype`, and optional `const char *original_id`, `original_uuid`, `original_date`, `reason`. Credit/debit notes require original ID and reason. Set unused optional pointers to `NULL`.

    All setter functions take builder first and `FatooraError **error` last, return status, and have no output argument:

    | Function suffix after `fatoora_invoice_builder_` | Value arguments |
    | --- | --- |
    | `set_id`, `set_uuid`, `set_issue_datetime`, `set_currency`, `set_previous_invoice_hash` | `const char *value` |
    | `add_payment_means` | `const FatooraPaymentMeans *value` from `fatoora_payment_means_new(code, instruction, &means, &error)` |
    | `set_invoice_counter` | `uint64_t value` |
    | `set_vat_category` | `FatooraVatCategory value` |
    | `set_flags` | `int32_t value` (bitmask of `FATOORA_FLAG_*`) |
    | `add_note` | `const char *language` (`NULL` for none), `const char *value` |
    | `set_seller`, `set_buyer` | `const FatooraPartyOptions *options` |
    | `add_line_item` | `const FatooraLineItemOptions *options` |

    `FatooraPartyOptions` contains `name`, borrowed `const FatooraAddress *address`, `vat_id`, `other_id`, and `scheme`; text fields are `const char *`. Seller VAT is required; buyer needs VAT or an other ID. Create the address with `fatoora_address_new(&options, &address, &error)` using `FatooraAddressOptions`: country_code, city, street, building_number, postal_code, and optional additional_street/additional_number/district. Setters copy the address data.

    `FatooraLineItemOptions` contains text fields `description`, `quantity`, `unit_code`, `unit_price`, `vat_rate`, and `FatooraVatCategory category`. Numbers are decimal strings.

    Building consumes builder state on success or failure. Rejected setter inputs leave the builder available for correction. Always call `fatoora_invoice_builder_free()` afterward, including after consumption.

    **Read invoice data**

    `fatoora_finalized_invoice_data(invoice, &data, &error)` and `fatoora_signed_invoice_data(invoice, &data, &error)` return an owned `FatooraInvoiceData` snapshot; free it with `fatoora_invoice_data_free`. Snapshots survive invoice destruction.

    Data getters take `const FatooraInvoiceData *self`, the output pointer below, and `FatooraError **error`, and return status:

    | Suffix after `fatoora_invoice_data_` | Output |
    | --- | --- |
    | `id`, `uuid`, `issue_datetime`, `currency`, `previous_invoice_hash`, `payment_means_code` | Owned `char **out` |
    | `invoice_level_charge`, `invoice_level_discount` | Owned decimal text through `char **out` |
    | `allowance_reason`, `original_invoice_reason` | Optional owned text through `char **out`; `NULL` when absent |
    | `invoice_counter` | `uint64_t *out` |
    | `vat_category` | `FatooraVatCategory *out` |
    | `invoice_type_kind` | `FatooraInvoiceTypeKind *out` |
    | `invoice_sub_type` | `FatooraInvoiceSubType *out` |
    | `flags` | `uint8_t *out` bitmask of `FATOORA_FLAG_*` |
    | `seller`, `buyer` | Owned `FatooraParty **out`; buyer may be `NULL` |
    | `note` | Optional owned `FatooraInvoiceNote **out` |
    | `original_invoice_ref` | Optional owned `FatooraOriginalInvoiceRef **out` |
    | `line_items_len` | `size_t *out` |
    | `line_item` | Extra `size_t index` argument, then owned `FatooraInvoiceLineItem **out` |

    `fatoora_finalized_invoice_totals` and `fatoora_signed_invoice_totals` return owned `FatooraInvoiceTotals **out`. Its getters have prefix `fatoora_invoice_totals_` and suffix `tax_inclusive`, `tax_amount`, `line_extension`, `allowance_total`, `charge_total`, `taxable_amount`, `prepaid_amount`, `payable_rounding_amount`, or `payable_amount`. Each takes the totals handle, returns decimal text through `char **out`, and accepts `FatooraError **error`. Free the text and then the snapshot with `fatoora_invoice_totals_free()`.

    Use [XML](xml.md) for serialization and [signing](invoice-signing.md) for consuming signing operations and metadata.
