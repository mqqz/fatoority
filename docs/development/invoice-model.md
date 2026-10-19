# Invoice model and preservation contract

The supported invoice profile is the ZATCA data dictionary dated 19 May 2023 and XML implementation standard 1.2. The source URLs, SHA-256 digests, profile applicability and all 150 dictionary rows are recorded in `fatoora-core/tests/fixtures/model-coverage.json`. The dictionary's spelling errors in XML paths are corrected in the executable selectors (AdditionalDocumentReference, ChargeIndicator and currencyID).

`dictionary_source_to_output_coverage` extracts source XML values directly and compares them with regenerated XML. It fails if a dictionary row has no exercised source occurrence. The corpus includes official invoice-family examples, advance adjustments, mixed categories, and a constructed invoice containing optional pricing and metadata fields. This is field coverage, not exhaustive coverage of all combinations.

## API inventory

| Area | Representation and API |
| --- | --- |
| Issue time | UTC builder timestamps serialize with `Z`; imported AST dates/times retain their clock representation while `issue_datetime()` represents the corresponding UTC instant. Reference times also preserve `Z`. |
| Invoice kind | `InvoiceType::{Tax, Prepayment, CreditNote, DebitNote}(InvoiceSubType)`; credit/debit constructors no longer embed references or reasons |
| References and reasons | Append `billing_reference(OriginalInvoiceRef)` and `payment_means(PaymentMeans)`; instructions belong to their payment entry |
| Notes | `note(InvoiceNote)` appends in order; `notes()` exposes all entries; `note()` reads the first |
| Line construction | Existing `LineItem::new` plus `LineItem::builder()`; named quantity, price, base quantity, gross price, price discount, supplied amounts, identifiers, allowances and charges |
| Adjustments | `Adjustment::amount` or `percentage`; line `.allowance`/`.charge`; invoice `.allowance(DocumentAdjustment)`/`.charge(DocumentAdjustment)` specify VAT independently |
| VAT breakdowns | `vat_breakdown(VatBreakdown)` accepts checked supplied amounts; `tax_exemption` assigns document breakdown metadata independently of line metadata |
| Delivery | `Delivery` has independent actual/latest dates, and preserves an explicitly present empty container |
| Party identifiers | `identifier(OtherId)` appends, `identifiers()` reads all schemes in source order; first-ID getters remain available |
| Buyer | An optional party and optional address; no address is invented on import |
| Address | Province/state is an additional optional field; empty basic address strings represent omitted components in the existing address API |
| Payments | Ordered payment entries, each with ordered optional instructions and an optional account; account identifier and payment terms are independent |
| Totals | Prepaid amount and signed payable rounding amount are builder inputs |
| Bindings | Mutable `set_*` and `add_*` methods; C/C++/Python/JS/C#/Kotlin generated from the shared FFI; Python also has a handwritten facade |

Adding `province` to address struct literals, making `Party::address()` optional, and removing embedded credit/debit reference fields are intentional Rust breaking changes. Party identifiers and payment instructions now use collections. Imported AST timestamps are converted to UTC for `issue_datetime()`; `issue_date()` and `issue_time()` preserve their XML representation. Rust tests and FFI callers are updated together. Existing aggregate discount/charge setters remain available for callers using one category; combining them with independent adjustment entries is rejected. First-entry convenience getters remain available, but collection getters are required to inspect all supplied values.

## Monetary dependencies

Accepted supplied values are stored once. Expected values are calculated transiently for checks. Serialized values are the same values used downstream.

| Value | Input and check | Downstream use |
| --- | --- | --- |
| Item net price | Supplied net or gross minus price discount; supplied net must agree exactly | Exact rational quantity × net price / price base |
| Line net | Round the complete rational price calculation to cents, subtract line allowances and add charges; accepted supplied net may differ by at most 0.01 | Document line extension and category/rate taxable sums |
| Line VAT | Retain supplied cents; when absent in construction, round accepted line net × rate / 100 | Line gross only; never sum line VAT to obtain document VAT |
| Line gross | Accepted line net plus accepted line VAT, exact equality | Serialized line RoundingAmount |
| Adjustment | Supplied cents, or base × percentage / 100 rounded to cents; when both supplied, require exact equality | Line net or the adjustment's document category/rate |
| Breakdown taxable | Accepted line nets in that category/rate, minus its document allowances plus charges | Breakdown tax expectation |
| Breakdown tax | Retain accepted supplied cents; standard-category tolerance is invoice-line count × 0.01 around the half-even rounded exact product; other categories require zero | Sum of document breakdown taxes |
| Document VAT | Sum accepted breakdown taxes, exact equality with supplied document tax | Tax-inclusive total; same-currency accounting tax |
| Tax-inclusive | Tax-exclusive plus document VAT, exact equality | Payable equation |
| Prepaid | Supplied cents; when type-386 adjustment lines exist, equals their taxable plus tax subtotals | Subtract from payable; adjustment lines never add ordinary taxable sales |
| Payable | Tax-inclusive minus prepaid plus payable rounding, exact equality | XML and QR tag 4 |

Decimal input remains decimal, never binary floating point. Price-basis division uses integer rational arithmetic until the final cent rounding, so non-terminating per-unit prices do not lose precision prematurely. Zero or negative price bases fail. Supplied sub-cent monetary totals fail. No global epsilon or import-repair mode is introduced.

## Specification and executable rule precedence

The published standard controls field ownership and representation. The existing business validator retains the pinned SDK 238-R3.4.8 executable semantics; its fixture manifest pins the executable hashes. Neither artifact is silently replaced on a discrepancy.

- BR-KSA-50 was removed in standard 1.2. Import therefore does not reject supplied line VAT simply for differing from quantity × unrounded price × rate. Gross and document equations remain checked.
- BR-S-08 describes category-and-rate allocation. Typed calculations use that allocation. The pinned executable validator has category-wide behavior for some mixed-rate cases; business-rule results remain independent of model acceptance.
- BR-CO-17/BR-S-09 executable tolerances apply to standard-category breakdown tax, including their half-even midpoint policy. For taxable 53.30 at 5%, the tolerance center is 2.66; with one line, 2.65–2.67 are accepted and 2.68 is rejected. Default construction uses the published commercial rounding convention (2.67 in this case). Typed checks use the exact decimal product rather than the SDK binary-float intermediate; business validation retains that executable behavior.
- Price-base calculations retain exact rational arithmetic. The SDK's executable line calculation uses an intermediate three-decimal truncation; this difference must be considered when evaluating cent boundaries.

## XML and validation boundaries

Optional structures distinguish absence from presence. Present PaymentMeans requires a code; present item identification requires an ID; present adjustments require their indicator and amount; a present price base must be positive. Delivery dates are independent. Date lexical errors fail import, while date-order business constraints are reported by business validation. Buyer addresses can be absent; a present address requires a valid country. Partial buyer street/building/city/postal components use the existing empty-string omission convention; supplied blank XML values fail import rather than disappearing. Price discount containers require both discount and gross amounts.

Numeric lexical formatting, scalar surrounding whitespace, ISO country/currency spelling, namespace prefixes and XML whitespace may normalize. Repeated supported values retain order. Optional line VAT/rate/gross and document allowance/charge/prepaid/rounding amount presence is retained. Schema validity, business-rule validity, and typed-model acceptance are distinct test outcomes. A preservation test does not certify compliance.

Content outside this typed profile follows the existing permissive import policy and may be dropped from regenerated XML. This is not arbitrary UBL lossless round-tripping. Signed import retains original XML verbatim; reconstructed XML is unsigned and does not inherit a hash or signature authenticating another document. Live ZATCA acceptance is a separate operation.
