# Invoice model XML coverage

The XML importer reads a typed subset of ZATCA UBL. It does not preserve arbitrary
UBL documents. Use the original XML for raw hashing, signing, storage, or forwarding.
`SignedInvoice::xml()` and `into_xml()` retain that original signed XML exactly.
`FinalizedInvoice::to_xml()` generates unsigned XML from the model; it cannot reuse
an imported signature.

The importer checks typed values, modeled cardinalities and supplied monetary
consistency. Successful import does not establish XSD conformance, ZATCA business-rule
compliance, certificate trust, or signature authenticity. Use the existing XSD and
ZATCA validation APIs for those separate checks. In particular, exemption rules,
conditional buyer identification, delivery-date ordering and latest-date requirements
remain in the existing business-rule validator. Import never supplies exemption reasons.

## Coverage matrix

Namespaces: `ubl` = `urn:oasis:names:specification:ubl:schema:xsd:Invoice-2`,
`cac` = `urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2`,
`cbc` = `urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2`.
All paths below start at `/ubl:Invoice`. Braces enumerate individual fields with the
same implementation and contract. Party rows apply separately to supplier and customer.

Implementation abbreviations:

- **P**: `invoice/xml/parse.rs::parse_finalized_invoice_doc`.
- **L**: `parse_line_items`; **A**: `parse_address`; **R**: `parse_reference`.
- **X**: `invoice/xml.rs::InvoiceXml`; other serializer names are in that file.
- **F**: every bundled invoice fixture in `tests/parse_fixtures.rs`, compared as
  `XML → model → XML → model`, with regenerated XSD validation and original signed XML/hash checks.
- **S**: `tests/semantic_roundtrip.rs`; **B**: `tests/parse_boundaries.rs`;
  **I**: `tests/parse_invoice.rs`; **PBT**: `tests/invoice_properties.rs`.

| XML path | Parser | Model / builder input | Serializer | Validation | Tests | Status |
|---|---|---|---|---|---|---|
| `cbc:ProfileID` | structural guard | fixed `reporting:1.0` | X | different supplied profile rejected | F | Normalized constant |
| `cbc:ID`, `cbc:UUID` | P required text | `InvoiceData`; `id`, `uuid` | X | nonempty | F, PBT, B | Preserved; surrounding token whitespace normalized |
| `cbc:IssueDate`, `cbc:IssueTime` | P + `parse_datetime` | `InvoiceTimestamp`; `issue_datetime` | X | valid date/time, supported seconds format | F, B, PBT | Normalized to UTC timestamp/seconds |
| `cbc:InvoiceTypeCode`, `@name` | `parse_invoice_type` | `InvoiceType`, `InvoiceFlags`; `new`, `flags` | `InvoiceTypeView` | 388/386/381/383, 01/02 + five binary flags | F, I, B, PBT | Preserved |
| `cbc:Note`, `@languageID` | P | `InvoiceNote`; `note`, `without_language` | `NoteXml` | singleton; free text preserved | S, B, PBT | Preserved, including absent language attribute; legacy `language()` returns en when absent |
| `cbc:DocumentCurrencyCode` | P | `CurrencyCode`; `currency` | X | ISO currency wrapper | F, B, PBT | Normalized uppercase |
| `cbc:TaxCurrencyCode`, accounting `cac:TaxTotal/cbc:TaxAmount/@currencyID` | P | `tax_currency`, `tax_currency_amount`; `tax_currency` | X | separate foreign tax amount, two decimals; one foreign total without subtotals | F, S | Preserved foreign currency/amount; same-currency code defaults to document currency |
| `cac:BillingReference/cac:InvoiceDocumentReference/cbc:{ID,UUID,IssueDate,IssueTime,DocumentTypeCode}` | R for credit/debit | `OriginalInvoiceRef`; `with_*`, invoice type | `BillingReferenceXml` / `InvoiceDocumentReferenceXml` | ID required; typed date/time | F, I | Preserved for credit/debit types |
| `cac:AdditionalDocumentReference[cbc:ID='ICV']/cbc:UUID` | P | `invoice_counter`; same setter | reference serializer | u64, duplicate reference rejected | F, B, PBT | Normalized integer |
| `cac:AdditionalDocumentReference[cbc:ID='PIH']/cac:Attachment/cbc:EmbeddedDocumentBinaryObject` | P | `previous_invoice_hash`; same setter | reference serializer | required nonempty; duplicate rejected | F, B, PBT | Preserved text; MIME normalized to text/plain |
| `cac:Accounting{Supplier,Customer}Party/cac:Party/cac:PartyIdentification/cbc:ID`, `@schemeID` | `parse_seller`, `parse_buyer` | `Party.other_id`, `OtherId`; party constructors / seller, buyer | `PartyIdentificationXml` | no seller-ID requirement applied to buyer; malformed present ID rejected | F, S, PBT | Preserved optional value and scheme |
| party `cac:PartyTaxScheme/cbc:CompanyID` | seller/buyer helpers | `VatId`; party constructors | `VatSchemeXml` | nonempty VAT when supplied; buyer VAT optional | F, S | Preserved; VAT lexical wrapper normalization |
| party `cac:PartyTaxScheme/cac:TaxScheme/cbc:ID` | structural guard | VAT constant | `TaxSchemeXml` | non-VAT rejected; full document rules separate | F | Normalized VAT scheme attributes |
| party `cac:PartyLegalEntity/cbc:RegistrationName` | seller/buyer helpers | `Party.name`; party constructors | `PartyLegalEntityXml` | nonempty when party exists | F, S, PBT | Preserved free text |
| party `cac:PostalAddress/cbc:{StreetName,AdditionalStreetName,BuildingNumber,PlotIdentification,CitySubdivisionName,CityName,PostalZone}` | A | `Address`; party constructors | `AddressXml` | seller required strings; buyer may omit street/city/building/postal fields | F, S, PBT | Preserved; absent buyer string fields represented by empty strings and omitted on output |
| party `cac:PostalAddress/cac:Country/cbc:IdentificationCode` | A | `CountryCode`; Address | `CountryXml` | ISO country required for modeled address | F, S, B | Normalized to alpha-2 XML / alpha-3 Rust |
| `cac:Delivery/cbc:{ActualDeliveryDate,LatestDeliveryDate}` | P | `Delivery`, `InvoiceDate`; `delivery` | `DeliveryXml`, before PaymentMeans | invalid supplied dates rejected; ordering/conditional presence checked by unchanged KSA rules | F, S | Preserved independently, including empty delivery container |
| `cac:PaymentMeans/cbc:PaymentMeansCode` | P | `payment_means_code`; same setter | `payment_means` | required nonempty | F, B, PBT | Preserved |
| `cac:PaymentMeans/cbc:InstructionNote` | P | `payment_instruction`; same setter; credit/debit reason | `payment_means` | no invented missing reason | F, I, S | Preserved; historical Note fallback for credit/debit remains |
| `cac:AllowanceCharge/cbc:ChargeIndicator` | P, every occurrence | `AllowanceCharge.charge`; `adjustment` | `AdjustmentXml` | XML boolean true/false/1/0 | F, S, B | Normalized boolean; occurrence/order preserved |
| `cac:AllowanceCharge/cbc:Amount`, `@currencyID` | P | `AllowanceCharge.amount`; `adjustment` | `AdjustmentXml` | nonnegative, two decimals; currency consistency | F, S, B | Preserved amount |
| `cac:AllowanceCharge/cbc:{AllowanceChargeReason,AllowanceChargeReasonCode}` | P | individual optional reason/code; `adjustment` | `AdjustmentXml` | business rules separate | F, S | Preserved independently for each adjustment |
| `cac:AllowanceCharge/cac:TaxCategory/cbc:{ID,Percent,TaxExemptionReasonCode,TaxExemptionReason}` | P | individual category/rate/`TaxExemption`; `adjustment` | `TaxCategoryXml` | category/rate checks; identical repeated classifications retained; conflicting classifications rejected | F, S, B | Preserved; no collapsing across adjustments |
| `cac:TaxTotal/cbc:TaxAmount` in document currency | P | `InvoiceTotalsData.tax_amount`; calculated | `tax_total` | supplied total must match grouped VAT | F, B, PBT | Preserved value; total-container count/order normalized |
| `cac:TaxTotal/cac:TaxSubtotal/cbc:{TaxableAmount,TaxAmount}` | P, every group | `VatBreakdown`; calculated by category AND rate | `tax_subtotal` | supplied group amounts checked; duplicate/unknown group rejected | F, B, PBT | Preserved; absent calculated zero groups may be emitted |
| subtotal `cac:TaxCategory/cbc:{ID,Percent,TaxExemptionReasonCode,TaxExemptionReason}` | P | `VatBreakdown.exemption`; `tax_exemption(category,rate,...)` | `TaxCategoryXml` | no global reason; independent code/description; business rules separate | F, S | Preserved per group |
| every modeled tax category `cac:TaxScheme/cbc:ID`; ID scheme/agency attributes | guard / constants | VAT + S/Z/E/O | `TaxCategoryXml`, `TaxSchemeXml` | non-VAT rejected; category recognized | F, B | Normalized fixed scheme/agency attributes; arbitrary attributes unsupported |
| `cac:LegalMonetaryTotal/cbc:{LineExtensionAmount,TaxExclusiveAmount,TaxInclusiveAmount,AllowanceTotalAmount,ChargeTotalAmount}` | P | `InvoiceTotalsData`; lines/adjustments | `legal_monetary_total` | supplied values checked exactly at money precision | F, B, PBT | Preserved values; optional zero totals emitted |
| `cac:LegalMonetaryTotal/cbc:{PrepaidAmount,PayableRoundingAmount,PayableAmount}` | P | totals; `prepaid_amount`, `payable_rounding_amount` | `legal_monetary_total` | nonnegative prepaid, signed rounding; payable equation checked | F, B, S, PBT | Preserved values; absent amounts normalize to zero |
| `cac:InvoiceLine/cbc:ID` | L | `LineItem.id`; `with_id` | `InvoiceLineXml` | required on import; builder assigns missing sequential IDs | F, S, PBT | Preserved supplied ID |
| line `cbc:InvoicedQuantity`, `@unitCode` | L | quantity/unit; line constructors | `quantity_with_unit` | nonnegative decimal; required unit | F, B, PBT | Preserved exact numeric value and unit; zero emits 0.00 for SDK BR-22 |
| line `cbc:LineExtensionAmount` | L | `LineItem.total_amount`; constructors | `InvoiceLineXml` | rounded quantity × unit price, at most two decimals | F, B, PBT | Preserved when consistent; mismatch rejected |
| line `cac:Item/cbc:Name` | L | description; constructors | `invoice_item` | nonempty; free text not trimmed | F, S, PBT | Preserved escaped/Arabic text |
| line `cac:Item/cac:ClassifiedTaxCategory/cbc:{ID,Percent,TaxExemptionReasonCode,TaxExemptionReason}` | L | category/rate/exemption; constructors, `with_exemption` | `TaxCategoryXml` | rate 0–100; nonstandard rate zero; business exemption checks separate | F, S, B | Preserved per line, independently of document breakdown |
| line `cac:Price/cbc:PriceAmount` | L | unit_price; constructors | `ExactPrice` | exact decimal, no forced money rounding | F, S, PBT | Preserved |
| line `cac:TaxTotal/cbc:{TaxAmount,RoundingAmount}` | L | vat_amount; constructors / importer | `invoice_line_tax_total` | supplied VAT retained; gross must equal line net + supplied VAT; scale checked | F, B | Preserved; absent gross reconstructed |
| line `cac:DocumentReference/cbc:{ID,UUID,IssueDate,IssueTime,DocumentTypeCode}` | L + R, every reference | `OriginalInvoiceRef`; `LineItem.document_reference` | `InvoiceDocumentReferenceXml` before TaxTotal | typed date/time; ID required | F | Preserved prepayment references |
| line `cac:TaxTotal/cac:TaxSubtotal/cbc:{TaxableAmount,TaxAmount}` and `cac:TaxCategory/cbc:{ID,Percent,TaxExemptionReasonCode,TaxExemptionReason}` | L, every subtotal | `VatBreakdown`; `LineItem.tax_subtotal` | `tax_subtotal` after RoundingAmount | supplied nonnegative money/valid rate; full prepayment rules separate | F | Preserved prepayment amounts/category metadata independently of line gross |
| modeled monetary `@currencyID` | guard | document or accounting currency | monetary helpers | missing/inconsistent currency rejected | F, S | Preserved semantic currency |
| signature extensions, QR reference | signed parser extracts material; stores original string | `SignedInvoice` | original `xml()` / `into_xml()` only | parse is not verification | F, signing/QR/SDK suites | Original signed XML preserved; excluded from regenerated unsigned model |

## Additional defects resolved

Besides buyer, exemption and delivery loss, the audit found replaced line IDs,
collapsed repeated adjustments, shared adjustment reasons, discarded adjustment
reason codes, discarded accounting-currency tax totals, synthesized note language
attributes, discarded payment instructions on ordinary invoices, invented missing
credit/debit reasons, and discarded prepayment references/subtotals. Decimal zero quantities also now retain an SDK-compatible
`0.00` spelling instead of becoming the BR-22-rejected `0`. These now have
typed preservation or explicit normalized behavior in the matrix above.

Ambiguous repeated modeled singleton elements no longer silently select the first.
Duplicate ICV/PIH/QR references and document tax groups are rejected. Line/price
allowances and non-unit price basis quantities are rejected rather than imported
using an incorrect quantity × price calculation.

## Unsupported content and limits

This is semantic preservation of the listed fields, **not lossless UBL editing**.
Namespace prefixes, whitespace, comments, processing instructions, XML declaration,
attribute ordering and numeric lexical spelling are not model data. Fixed VAT
scheme attributes are regenerated. Unlisted optional attributes are not retained.

The model has one note, payment means, delivery, party identifier, postal address,
legal entity and VAT registration per relevant party. Multiple occurrences are
rejected rather than truncated. Multiple distinct tax classifications on one
adjustment are rejected; identical repeats in the SDK fixtures retain their count.
Different adjustments may use different categories/rates/reasons.

A modeled buyer requires a legal name and an address country. An absent/empty buyer
container becomes `None`; a nonempty malformed buyer fails. The current Address
API represents missing buyer street/city/building/postal values as empty strings;
optional additional street, plot and district retain `Option` values. Buyers do
not inherit seller VAT/identification requirements. `Buyer::new` now permits both
identifiers to be absent; full conditional ZATCA rules belong to XML validation.

Unlisted UBL structures are outside the typed contract and are not emitted. Examples:
contacts, endpoints, payment accounts/terms, order/contract/despatch/receipt/project
references, extra attachments, additional commodity classifications, item properties,
prices with a non-unit basis, line/price allowance calculation models, adjustment
base amounts/multipliers, arbitrary tax schemes, arbitrary extensions and customs
metadata. Billing references are modeled for credit/debit types. Raw XML APIs remain
the preservation path for documents using these structures. The importer is not an
unsupported-content inventory API; unknown extension content can be ignored.

Same-currency tax code/amount, profile, zero totals and gross amounts have documented
reconstruction defaults. An omitted document tax breakdown is calculated from lines
and adjustments. Do not compare original and regenerated XML byte-for-byte or reuse
the original signature/hash for generated XML.

## API compatibility and bindings

Existing builder setters and line constructors remain available. New additive APIs
cover `Delivery`, `TaxExemption`, individual `AllowanceCharge` entries, line IDs,
reference time/type metadata, prepayment subtotals, accounting currency, payment
instructions, prepaid amounts and rounding. Legacy aggregate adjustment setters
cannot be mixed with individual adjustments. `allowance_reason()` remains a legacy
view; use `adjustments()` for differing reasons. `vat_category()` is the legacy
adjustment/default category; use line and breakdown categories for mixed invoices.

New serialized fields use `serde(default)` (identical classification count defaults
to one). Old serialized models remain readable. Python line metadata arguments are
optional keyword-only additions. C, C++, Python native, JavaScript, C# and Kotlin
bindings are regenerated from the shared Diplomat API; the Python facade also
exposes typed metadata and collections.

## Verification

`semantic_roundtrip` covers buyers with/without identifiers, optional address strings,
all delivery-date combinations and invalid dates, mixed S/E/Z/O categories,
independent exemption code/description, Arabic/escaped text, repeated adjustments,
foreign accounting currency, note-language absence, payment instructions, legacy
serde defaults and cardinality failures. Compliant standard/exempt/export fixtures
are checked against XSD and the unchanged CEN/KSA rules. Invalid delivery ordering
is retained and reported as BR-KSA-36 at the validator's existing severity.

`parse_fixtures` covers all bundled invoice families, including advance-payment
references and subtotals, with complete imported-model equality after regeneration.
Python tests exercise new metadata across the native ABI, serialization, reimport,
and signed-source/hash retention. Rust binding contract tests cover the same metadata
through owned snapshots, including optional values, invalid inputs and indexed
access errors. Serializer tests inject field and output failures to verify error
propagation. Existing property tests compare builder-created invoice fields,
including buyer and line metadata.

Validation run for this change:

- `cargo test --workspace --offline`: passed, including signing, QR, SDK parity,
  property tests and all new model regressions. The live ZATCA credential example
  and placeholder doctests remain intentionally ignored. The ignored submission
  worker is exercised by its parent test in a subprocess.
- `cargo clippy --workspace --all-targets --offline -- -D warnings`, Rust formatting,
  and `git diff --check`: passed.
- Installed native Python package: **60 tests passed**.
- `scripts/check_native_bindings.py`: C, C++, public C ownership/allocation,
  examples and ZATCA reporting contracts passed.
- `scripts/generate_bindings.py --check` with the repository-pinned generators:
  passed for all generated languages.
- Strict MkDocs build: passed.

C#, Kotlin and JavaScript sources were regenerated and checked for reproducibility;
their runtime suites and non-Linux platform builds were not run in this change.
Local validation does not establish remote ZATCA acceptance.
