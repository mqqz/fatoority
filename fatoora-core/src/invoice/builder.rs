//! Invoice builder and view types.
use super::{
    Buyer, CurrencyCode, InvoiceData, InvoiceError, InvoiceField, InvoiceFlags, InvoiceHash,
    InvoiceNote, InvoiceTimestamp, InvoiceTotals, InvoiceType, InvoiceUuid, LineItems, QrCodeError,
    QrResult, Seller, ValidationError, ValidationIssue, ValidationKind, VatCategory,
};
use crate::Decimal;
use crate::invoice::sign::{
    InvoiceSigner, SignedProperties, SigningError, invoice_hash_base64_from_xml,
};

/// An invoice with computed totals and the implemented builder field checks.
///
/// XML imports also check the supported supplied amounts against computed values.
/// This type does not establish XSD conformance, full business-rule compliance,
/// or signature authenticity.
#[derive(Debug, Clone, PartialEq)]
pub struct FinalizedInvoice {
    data: InvoiceData,
    pub(crate) totals: InvoiceTotals,
}

/// An invoice carrying signature material, QR payload, and signed XML.
///
/// Produced by local signing or imported with local signature and QR integrity checks.
/// It does not establish certificate issuer trust, clearance, or business-rule compliance.
///
/// Formatting signed XML is intentionally unavailable:
/// ```compile_fail
/// use fatoora_core::invoice::{SignedInvoice, xml::XmlFormat};
/// fn reformat(invoice: &SignedInvoice) {
///     invoice.to_xml_with_format(XmlFormat::Compact);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SignedInvoice {
    finalized: FinalizedInvoice,
    signed_properties: SignedProperties,
    qr_code: String,
    signed_xml: String,
}

/// Builder that checks selected fields and computes invoice totals.
///
/// Configuration methods consume and return the builder. Chain them or reassign
/// the result when configuring conditionally. `build(self)` moves owned fields
/// into the invoice and consumes the builder even when validation fails.
///
/// # Examples
/// ```rust,no_run
/// use fatoora_core::invoice::{
///     InvoiceBuilder, InvoiceSubType, InvoiceType, LineItem, PaymentMeans, VatCategory, Seller,
/// };
///
/// let seller: Seller = unimplemented!();
/// let invoice = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
///     .id("INV-1")
///     .uuid("uuid-1".parse()?)
///     .issue_datetime("2024-01-01T12:30:00Z".parse()?)
///     .currency("SAR".parse()?)
///     .previous_invoice_hash("hash".parse()?)
///     .invoice_counter(1)
///     .seller(seller)
///     .payment_means(PaymentMeans::new("10"))
///     .vat_category(VatCategory::Standard)
///     .line_item(LineItem::new("Item", fatoora_core::Decimal::parse("1.0").unwrap(), "PCE", fatoora_core::Decimal::parse("100.0").unwrap(), fatoora_core::Decimal::parse("15.0").unwrap(), VatCategory::Standard).unwrap())
///     .build()?;
/// # let _ = invoice;
/// use fatoora_core::invoice::InvoiceError;
/// # Ok::<(), InvoiceError>(())
/// ```
///
/// A configuration call moves the builder:
/// ```compile_fail
/// use fatoora_core::invoice::{InvoiceBuilder, InvoiceType, InvoiceSubType};
/// let builder = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified));
/// let configured = builder.id("INV-1");
/// let invoice = builder.build(); // builder was moved into id()
/// ```
#[must_use = "builder methods return the updated builder"]
#[derive(Debug, Clone)]
pub struct InvoiceBuilder {
    invoice_type: InvoiceType,
    id: Option<String>,
    uuid: Option<InvoiceUuid>,
    issue_datetime: Option<InvoiceTimestamp>,
    currency: Option<CurrencyCode>,
    tax_currency: Option<super::CurrencyCode>,
    tax_currency_amount: Option<Decimal>,
    previous_invoice_hash: Option<InvoiceHash>,
    invoice_counter: Option<u64>,
    notes: Vec<InvoiceNote>,
    billing_references: Vec<super::OriginalInvoiceRef>,
    purchase_order_id: Option<String>,
    contract_id: Option<String>,
    seller: Option<Seller>,
    buyer: Option<Buyer>,
    delivery: Option<super::Delivery>,
    line_items: LineItems,
    payment_means: Vec<super::PaymentMeans>,
    payment_instruction_placeholder: bool,
    prepaid_amount: Decimal,
    payable_rounding_amount: Decimal,
    vat_category: Option<VatCategory>,
    flags: InvoiceFlags,
    invoice_level_charge: Decimal,
    invoice_level_discount: Decimal,
    pub(crate) adjustment_vat_rate: Option<Decimal>,
    allowance_reason: Option<String>,
    adjustments: Vec<(super::AdjustmentKind, super::DocumentAdjustment)>,
    exemptions: Vec<(VatCategory, Decimal, super::TaxExemption)>,
    supplied_breakdowns: Vec<super::VatBreakdown>,
    party_identifiers: Vec<(bool, super::OtherId)>,
}

impl InvoiceBuilder {
    /// Append an additional seller identifier, applied to the seller during `build`.
    ///
    /// Appends; earlier identifiers are kept in call order.
    pub fn seller_identifier(mut self, value: super::OtherId) -> Self {
        self.party_identifiers.push((true, value));
        self
    }
    /// Append an additional buyer identifier, applied to the buyer during `build`.
    ///
    /// Appends; earlier identifiers are kept in call order. `build` fails if no
    /// buyer is set.
    pub fn buyer_identifier(mut self, value: super::OtherId) -> Self {
        self.party_identifiers.push((false, value));
        self
    }

    /// Append several seller identifiers, preserving iteration order.
    ///
    /// Appends; equivalent to calling [`seller_identifier`](Self::seller_identifier)
    /// for each identifier.
    pub fn seller_identifiers(self, values: impl IntoIterator<Item = super::OtherId>) -> Self {
        values.into_iter().fold(self, Self::seller_identifier)
    }

    /// Append several buyer identifiers, preserving iteration order.
    ///
    /// Appends; equivalent to calling [`buyer_identifier`](Self::buyer_identifier)
    /// for each identifier. `build` fails if no buyer is set and any identifier
    /// was supplied.
    pub fn buyer_identifiers(self, values: impl IntoIterator<Item = super::OtherId>) -> Self {
        values.into_iter().fold(self, Self::buyer_identifier)
    }

    /// Preserve a supplied VAT breakdown after checking the applicable arithmetic.
    ///
    /// Appends; each call adds another supplied breakdown.
    pub fn vat_breakdown(mut self, value: super::VatBreakdown) -> Self {
        self.supplied_breakdowns.push(value);
        self
    }

    /// Append a document-level allowance (`cac:AllowanceCharge` with
    /// `cbc:ChargeIndicator` `false`); validation occurs in `build`.
    ///
    /// This is the way to add document-level allowances. Each entry carries its own
    /// amount or percentage calculation, reason, reason code and VAT category and rate.
    ///
    /// Appends; allowances and charges keep their relative call order.
    ///
    /// # Examples
    /// ```rust
    /// use fatoora_core::Decimal;
    /// use fatoora_core::invoice::{
    ///     Adjustment, DocumentAdjustment, InvoiceBuilder, InvoiceSubType, InvoiceType,
    ///     VatCategory,
    /// };
    ///
    /// let rate = Decimal::parse("15").unwrap();
    /// let builder = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
    ///     .allowance(
    ///         DocumentAdjustment::new(
    ///             Adjustment::amount(Decimal::parse("10").unwrap()),
    ///             VatCategory::Standard,
    ///             rate,
    ///         )
    ///         .reason("Loyalty discount"),
    ///     )
    ///     .charge(
    ///         DocumentAdjustment::new(
    ///             Adjustment::percentage(
    ///                 Decimal::parse("100").unwrap(),
    ///                 Decimal::parse("5").unwrap(),
    ///             ),
    ///             VatCategory::Standard,
    ///             rate,
    ///         )
    ///         .reason_code("FC"),
    ///     );
    /// # let _ = builder;
    /// ```
    pub fn allowance(mut self, adjustment: super::DocumentAdjustment) -> Self {
        self.adjustments
            .push((super::AdjustmentKind::Allowance, adjustment));
        self
    }
    /// Append a document-level charge (`cac:AllowanceCharge` with
    /// `cbc:ChargeIndicator` `true`); validation occurs in `build`.
    ///
    /// This is the way to add document-level charges; see [`allowance`](Self::allowance).
    ///
    /// Appends; allowances and charges keep their relative call order.
    pub fn charge(mut self, adjustment: super::DocumentAdjustment) -> Self {
        self.adjustments
            .push((super::AdjustmentKind::Charge, adjustment));
        self
    }
    /// Append payment means, or fill the placeholder created by `payment_instruction`.
    /// The existing instruction precedes any notes supplied on the payment means.
    ///
    /// Appends, except that the first call after `payment_instruction` on an
    /// empty builder replaces the placeholder it created.
    pub fn payment_means(mut self, mut value: super::PaymentMeans) -> Self {
        if self.payment_instruction_placeholder {
            let mut notes = std::mem::take(&mut self.payment_means[0].instruction_notes);
            notes.append(&mut value.instruction_notes);
            value.instruction_notes = notes;
            self.payment_means[0] = value;
            self.payment_instruction_placeholder = false;
        } else {
            self.payment_means.push(value);
        }
        self
    }
    /// Set the instruction note on the first payment means, creating a placeholder
    /// payment means if none exists.
    ///
    /// Replaces the first payment means' existing instruction notes.
    pub fn payment_instruction(mut self, instruction: impl Into<String>) -> Self {
        if self.payment_means.is_empty() {
            self.payment_means.push(super::PaymentMeans::new(""));
            self.payment_instruction_placeholder = true;
        }
        self.payment_means[0].instruction_notes = vec![instruction.into()];
        self
    }
    /// Set the prepaid amount.
    ///
    /// Replaces any previously set value.
    pub fn prepaid_amount(mut self, amount: Decimal) -> Self {
        self.prepaid_amount = amount;
        self
    }
    /// Set the payable rounding amount.
    ///
    /// Replaces any previously set value.
    pub fn payable_rounding_amount(mut self, amount: Decimal) -> Self {
        self.payable_rounding_amount = amount;
        self
    }

    /// Set the tax currency and its tax amount.
    ///
    /// Replaces any previously set tax currency and amount.
    pub fn tax_currency(mut self, currency: super::CurrencyCode, amount: Decimal) -> Self {
        self.tax_currency = Some(currency);
        self.tax_currency_amount = Some(amount);
        self
    }

    /// Add an individual document adjustment. Do not combine with legacy aggregate setters.
    ///
    /// Appends; earlier adjustments are kept in call order. Forwards to
    /// [`allowance`](Self::allowance) or [`charge`](Self::charge) according to
    /// `adjustment.kind`, converting with `DocumentAdjustment::from`.
    #[deprecated(
        since = "0.6.0",
        note = "use `allowance(DocumentAdjustment)` or `charge(DocumentAdjustment)` instead; \
                `DocumentAdjustment::from(AllowanceCharge)` converts an existing entry"
    )]
    pub fn adjustment(self, adjustment: super::AllowanceCharge) -> Self {
        let kind = adjustment.kind;
        let adjustment = super::DocumentAdjustment::from(adjustment);
        match kind {
            super::AdjustmentKind::Allowance => self.allowance(adjustment),
            super::AdjustmentKind::Charge => self.charge(adjustment),
        }
    }

    /// Append a VAT exemption reason for a category and rate.
    ///
    /// Appends; conflicting exemptions for the same category and rate fail in `build`.
    pub fn tax_exemption(
        mut self,
        category: VatCategory,
        rate: Decimal,
        exemption: super::TaxExemption,
    ) -> Self {
        self.exemptions.push((category, rate, exemption));
        self
    }
    /// Set the delivery information.
    ///
    /// Replaces any previously set value.
    pub fn delivery(mut self, delivery: super::Delivery) -> Self {
        self.delivery = Some(delivery);
        self
    }

    /// Create a builder with the invoice type.
    ///
    /// All optional fields start unset and all collections start empty.
    pub fn new(invoice_type: InvoiceType) -> Self {
        Self {
            invoice_type,
            id: None,
            uuid: None,
            issue_datetime: None,
            currency: None,
            tax_currency: None,
            tax_currency_amount: None,
            previous_invoice_hash: None,
            invoice_counter: None,
            billing_references: Vec::new(),
            notes: Vec::new(),
            purchase_order_id: None,
            contract_id: None,
            seller: None,
            buyer: None,
            delivery: None,
            line_items: Vec::new(),
            payment_means: Vec::new(),
            payment_instruction_placeholder: false,
            prepaid_amount: Decimal::ZERO,
            payable_rounding_amount: Decimal::ZERO,
            vat_category: None,
            flags: InvoiceFlags::empty(),
            invoice_level_charge: Decimal::ZERO,
            invoice_level_discount: Decimal::ZERO,
            adjustment_vat_rate: None,
            allowance_reason: None,
            adjustments: Vec::new(),
            exemptions: Vec::new(),
            supplied_breakdowns: Vec::new(),
            party_identifiers: Vec::new(),
        }
    }

    /// Set the invoice ID.
    ///
    /// Replaces any previously set value.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the invoice UUID (`cbc:UUID`).
    ///
    /// Parse a string with [`InvoiceUuid::parse`]; with the `uuid` feature a
    /// `uuid::Uuid` converts via `From`. Replaces any previously set value.
    pub fn uuid(mut self, uuid: InvoiceUuid) -> Self {
        self.uuid = Some(uuid);
        self
    }

    /// Set the issue timestamp; `cbc:IssueDate` and `cbc:IssueTime` are derived from it.
    ///
    /// Parse a `YYYY-MM-DDTHH:MM:SSZ` string with [`InvoiceTimestamp::parse`]; with the
    /// `chrono` feature a `chrono::DateTime<Utc>` converts via `From`. Replaces any
    /// previously set value.
    pub fn issue_datetime(mut self, issue_datetime: InvoiceTimestamp) -> Self {
        self.issue_datetime = Some(issue_datetime);
        self
    }

    /// Set the document currency (`cbc:DocumentCurrencyCode`).
    ///
    /// Parse an ISO 4217 code with [`CurrencyCode::parse`]. Replaces any previously set value.
    pub fn currency(mut self, currency: CurrencyCode) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set the previous invoice hash (PIH).
    ///
    /// Parse a string with [`InvoiceHash::parse`]. Replaces any previously set value.
    pub fn previous_invoice_hash(mut self, hash: InvoiceHash) -> Self {
        self.previous_invoice_hash = Some(hash);
        self
    }

    /// Set the invoice counter value.
    ///
    /// Replaces any previously set value.
    pub fn invoice_counter(mut self, counter: u64) -> Self {
        self.invoice_counter = Some(counter);
        self
    }

    /// Set the seller.
    ///
    /// Replaces any previously set value.
    pub fn seller(mut self, seller: Seller) -> Self {
        self.seller = Some(seller);
        self
    }

    /// Set the legacy aggregate document-level charge.
    ///
    /// Replaces any previously set value. The charge is emitted with the document
    /// VAT category and a rate taken from the first line in that category, and its
    /// reason is the [`allowance_reason`](Self::allowance_reason) (or `"charge"`).
    /// It cannot be combined with [`allowance`](Self::allowance) or
    /// [`charge`](Self::charge).
    #[deprecated(
        since = "0.6.0",
        note = "use `charge(DocumentAdjustment::new(Adjustment::amount(..), category, rate))` \
                instead; it appends an entry with an explicit VAT category and rate \
                instead of replacing one aggregate inferred from the document"
    )]
    pub fn invoice_level_charge(mut self, charge: Decimal) -> Self {
        self.invoice_level_charge = charge;
        self
    }

    /// Set the legacy aggregate document-level discount.
    ///
    /// Replaces any previously set value. The allowance is emitted with the document
    /// VAT category and a rate taken from the first line in that category, and its
    /// reason is the [`allowance_reason`](Self::allowance_reason) (or `"discount"`).
    /// It cannot be combined with [`allowance`](Self::allowance) or
    /// [`charge`](Self::charge).
    #[deprecated(
        since = "0.6.0",
        note = "use `allowance(DocumentAdjustment::new(Adjustment::amount(..), category, rate))` \
                instead; it appends an entry with an explicit VAT category and rate \
                instead of replacing one aggregate inferred from the document"
    )]
    pub fn invoice_level_discount(mut self, discount: Decimal) -> Self {
        self.invoice_level_discount = discount;
        self
    }

    /// Set the legacy aggregate allowance reason.
    ///
    /// Replaces any previously set value. The reason applies to both the aggregate
    /// discount and the aggregate charge, and on its own emits a zero-amount
    /// allowance. It cannot be combined with [`allowance`](Self::allowance) or
    /// [`charge`](Self::charge).
    #[deprecated(
        since = "0.6.0",
        note = "set the reason on each entry with `DocumentAdjustment::reason` and add it \
                with `allowance` or `charge` instead"
    )]
    pub fn allowance_reason(mut self, reason: impl Into<String>) -> Self {
        self.allowance_reason = Some(reason.into());
        self
    }

    /// Append a billing reference to an original invoice.
    ///
    /// Appends; earlier references are kept in call order.
    pub fn billing_reference(mut self, reference: super::OriginalInvoiceRef) -> Self {
        self.billing_references.push(reference);
        self
    }

    /// Append several billing references, preserving iteration order.
    ///
    /// Appends; equivalent to calling [`billing_reference`](Self::billing_reference)
    /// for each reference.
    pub fn billing_references(
        self,
        references: impl IntoIterator<Item = super::OriginalInvoiceRef>,
    ) -> Self {
        references.into_iter().fold(self, Self::billing_reference)
    }
    /// Set the purchase order ID.
    ///
    /// Replaces any previously set value.
    pub fn purchase_order_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_id = Some(value.into());
        self
    }
    /// Set the contract ID.
    ///
    /// Replaces any previously set value.
    pub fn contract_id(mut self, value: impl Into<String>) -> Self {
        self.contract_id = Some(value.into());
        self
    }
    /// Append an invoice note.
    ///
    /// Appends; earlier notes are kept in call order.
    pub fn note(mut self, note: InvoiceNote) -> Self {
        self.notes.push(note);
        self
    }

    /// Append several invoice notes, preserving iteration order.
    ///
    /// Appends; equivalent to calling [`note`](Self::note) for each note.
    pub fn notes(self, notes: impl IntoIterator<Item = InvoiceNote>) -> Self {
        notes.into_iter().fold(self, Self::note)
    }

    /// Set the buyer.
    ///
    /// Replaces any previously set value.
    pub fn buyer(mut self, buyer: Buyer) -> Self {
        self.buyer = Some(buyer);
        self
    }

    /// Set the payment means type code of the first payment means, creating
    /// one if none exists.
    ///
    /// Replaces the first payment means' code; later payment means are unchanged.
    #[deprecated(
        since = "0.6.0",
        note = "use `payment_means(PaymentMeans::new(code))` instead; it appends a \
                payment means rather than changing the code of the first one"
    )]
    pub fn payment_means_code(mut self, code: impl Into<String>) -> Self {
        self.payment_instruction_placeholder = false;
        if self.payment_means.is_empty() {
            self.payment_means.push(super::PaymentMeans::new(code));
        } else {
            self.payment_means[0].payment_means_type_code = code.into();
        }
        self
    }

    /// Set the document VAT category.
    ///
    /// Replaces any previously set value.
    pub fn vat_category(mut self, vat_category: VatCategory) -> Self {
        self.vat_category = Some(vat_category);
        self
    }

    /// Append a line item, preserving insertion order.
    ///
    /// Appends; earlier line items are kept.
    pub fn line_item(mut self, line_item: super::LineItem) -> Self {
        self.line_items.push(line_item);
        self
    }

    /// Append several line items, preserving iteration order.
    ///
    /// Appends; equivalent to calling [`line_item`](Self::line_item) for each item.
    pub fn line_items(self, line_items: impl IntoIterator<Item = super::LineItem>) -> Self {
        line_items.into_iter().fold(self, Self::line_item)
    }

    /// Set the invoice flags.
    ///
    /// Replaces any previously set flags rather than merging them.
    pub fn flags(mut self, flags: InvoiceFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Run the implemented field checks and compute totals.
    ///
    /// Checks include required fields, supported formats, numeric ranges, and
    /// selected amount consistency checks. This does not run XSD validation,
    /// full business rules, or signature verification.
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] when required fields are missing or invalid,
    /// or [`InvoiceError::Decimal`] if an amount calculation overflows.
    ///
    /// # Panics
    /// Does not panic in practice. Internal `expect` calls unwrap required fields only after
    /// every missing or invalid required field has been reported as an error, so they are
    /// unreachable.
    ///
    /// Consumes the builder.
    pub fn build(mut self) -> Result<FinalizedInvoice, InvoiceError> {
        // Conversion errors are reported with the other adjustment checks below,
        // after the field checks.
        let entries: Vec<_> = std::mem::take(&mut self.adjustments)
            .into_iter()
            .map(|(kind, adjustment)| adjustment.into_entry(kind))
            .collect();

        for (index, line) in self.line_items.iter_mut().enumerate() {
            if line.id.is_none() {
                line.id = Some((index + 1).to_string());
            }
        }

        let mut issues = Vec::new();
        let mut push_issue = |field: InvoiceField, kind: ValidationKind, line_item_index| {
            issues.push(ValidationIssue::new(field, kind, line_item_index));
        };

        for reference in &self.billing_references {
            if reference.id().trim().is_empty() {
                push_issue(InvoiceField::OriginalInvoiceId, ValidationKind::Empty, None);
            }
        }
        match self.id.as_deref() {
            None => push_issue(InvoiceField::Id, ValidationKind::Missing, None),
            Some(value) if value.trim().is_empty() => {
                push_issue(InvoiceField::Id, ValidationKind::Empty, None)
            }
            _ => {}
        }
        if self.uuid.is_none() {
            push_issue(InvoiceField::Uuid, ValidationKind::Missing, None);
        }
        if self.issue_datetime.is_none() {
            push_issue(InvoiceField::IssueDateTime, ValidationKind::Missing, None);
        }
        if self.currency.is_none() {
            push_issue(InvoiceField::Currency, ValidationKind::Missing, None);
        }
        if self.previous_invoice_hash.is_none() {
            push_issue(
                InvoiceField::PreviousInvoiceHash,
                ValidationKind::Missing,
                None,
            );
        }
        if self.invoice_counter.is_none() {
            push_issue(InvoiceField::InvoiceCounter, ValidationKind::Missing, None);
        }
        if self.seller.is_none() {
            push_issue(InvoiceField::Seller, ValidationKind::Missing, None);
        }
        for payment in &self.payment_means {
            if payment.payment_means_type_code.trim().is_empty() {
                push_issue(InvoiceField::PaymentMeansCode, ValidationKind::Empty, None);
            }
        }
        if self.vat_category.is_none() {
            push_issue(InvoiceField::VatCategory, ValidationKind::Missing, None);
        }
        if self.line_items.is_empty() {
            push_issue(InvoiceField::LineItems, ValidationKind::Missing, None);
        } else {
            for (idx, item) in self.line_items.iter().enumerate() {
                if item.id().is_some_and(|id| id.trim().is_empty()) {
                    push_issue(InvoiceField::LineItemId, ValidationKind::Empty, Some(idx));
                }
                for reference in item.document_references() {
                    if reference.id().trim().is_empty() {
                        push_issue(
                            InvoiceField::LineItemDocumentReferenceId,
                            ValidationKind::Empty,
                            Some(idx),
                        );
                    }
                }
                if item.description().trim().is_empty() {
                    push_issue(
                        InvoiceField::LineItemDescription,
                        ValidationKind::Empty,
                        Some(idx),
                    );
                }
                if item.quantity() < Decimal::ZERO {
                    push_issue(
                        InvoiceField::LineItemQuantity,
                        ValidationKind::OutOfRange,
                        Some(idx),
                    );
                }
                if item.unit_price() < Decimal::ZERO {
                    push_issue(
                        InvoiceField::LineItemUnitPrice,
                        ValidationKind::OutOfRange,
                        Some(idx),
                    );
                }
                if item.total_amount() < Decimal::ZERO {
                    push_issue(
                        InvoiceField::LineItemTotalAmount,
                        ValidationKind::OutOfRange,
                        Some(idx),
                    );
                }
                if item.vat_rate() < Decimal::ZERO
                    || item.vat_rate() > Decimal::from(100)
                    || item.vat_rate().scale() > 2
                {
                    push_issue(
                        InvoiceField::LineItemVatRate,
                        ValidationKind::OutOfRange,
                        Some(idx),
                    );
                }
                if item.vat_amount() < Decimal::ZERO {
                    push_issue(
                        InvoiceField::LineItemVatAmount,
                        ValidationKind::OutOfRange,
                        Some(idx),
                    );
                }
                if let Some(field) = item.tax_presence_error() {
                    push_issue(field, ValidationKind::OutOfRange, Some(idx));
                }
                if item.is_prepayment_adjustment()
                    && (item.quantity() != Decimal::ZERO
                        || item.unit_price() != Decimal::ZERO
                        || item.total_amount() != Decimal::ZERO
                        || item.vat_amount() != Decimal::ZERO
                        || item
                            .adjustments()
                            .iter()
                            .any(|a| a.adjustment.value().is_ok_and(|v| v != Decimal::ZERO)))
                {
                    push_issue(
                        InvoiceField::LineItemTotalAmount,
                        ValidationKind::Mismatch,
                        Some(idx),
                    );
                }
                let expected = item.expected_net()?;
                if super::line_builder::check_supplied(
                    InvoiceField::LineItemTotalAmount,
                    item.total_amount(),
                    expected,
                    "0.01".parse()?,
                )
                .is_err()
                {
                    push_issue(
                        InvoiceField::LineItemTotalAmount,
                        ValidationKind::Mismatch,
                        Some(idx),
                    );
                }
                if item.vat_amount().scale() > 2 {
                    push_issue(
                        InvoiceField::LineItemVatAmount,
                        ValidationKind::OutOfRange,
                        Some(idx),
                    );
                }
                if item.vat_category() != VatCategory::Standard && item.vat_rate() != Decimal::ZERO
                {
                    push_issue(
                        InvoiceField::LineItemVatRate,
                        ValidationKind::Mismatch,
                        Some(idx),
                    );
                }
                item.total_amount().add(item.vat_amount())?;
            }
        }
        for (field, amount) in [
            (
                InvoiceField::InvoiceLevelDiscount,
                self.invoice_level_discount,
            ),
            (InvoiceField::InvoiceLevelCharge, self.invoice_level_charge),
        ] {
            if amount < Decimal::ZERO || amount.scale() > 2 {
                push_issue(field, ValidationKind::OutOfRange, None);
            }
        }
        if !issues.is_empty() {
            return Err(InvoiceError::Validation(ValidationError::new(issues)));
        }

        let mut adjustments = Vec::with_capacity(entries.len());
        if !entries.is_empty() {
            if self.invoice_level_discount != Decimal::ZERO
                || self.invoice_level_charge != Decimal::ZERO
                || self.allowance_reason.is_some()
            {
                return Err(ValidationError::new(vec![ValidationIssue::new(
                    InvoiceField::InvoiceLevelDiscount,
                    ValidationKind::Mismatch,
                    None,
                )])
                .into());
            }
            for entry in entries {
                let adjustment = entry?;
                super::Adjustment {
                    amount: Some(adjustment.amount),
                    base_amount: adjustment.base_amount,
                    percentage: adjustment.percentage,
                    reason: adjustment.reason.clone(),
                    reason_code: adjustment.reason_code.clone(),
                }
                .value()?;
                if (adjustment.omitted_rate && adjustment.category != VatCategory::OutOfScope)
                    || adjustment.amount < Decimal::ZERO
                    || adjustment.amount.scale() > 2
                    || adjustment.rate < Decimal::ZERO
                    || adjustment.rate > Decimal::from(100)
                    || adjustment.rate.scale() > 2
                    || (adjustment.category != VatCategory::Standard
                        && adjustment.rate != Decimal::ZERO)
                {
                    return Err(ValidationError::new(vec![ValidationIssue::new(
                        InvoiceField::InvoiceLevelDiscount,
                        ValidationKind::OutOfRange,
                        None,
                    )])
                    .into());
                }
                if adjustment.kind.is_charge() {
                    self.invoice_level_charge = self.invoice_level_charge.add(adjustment.amount)?;
                } else {
                    self.invoice_level_discount =
                        self.invoice_level_discount.add(adjustment.amount)?;
                }
                adjustments.push(adjustment);
            }
        }
        for (seller, value) in self.party_identifiers {
            if seller {
                self.seller = self.seller.map(|p| p.identifier(value));
            } else {
                let buyer = self.buyer.take().ok_or_else(|| {
                    ValidationError::new(vec![ValidationIssue::new(
                        InvoiceField::Buyer,
                        ValidationKind::Missing,
                        None,
                    )])
                })?;
                self.buyer = Some(buyer.identifier(value));
            }
        }
        let mut invoice = InvoiceData {
            issue_date_override: None,
            issue_time_override: None,
            document_tax_first: false,
            invoice_type: self.invoice_type,
            id: self.id.expect("validated id"),
            uuid: self.uuid.expect("validated uuid"),
            issue_datetime: self.issue_datetime.expect("validated issue datetime"),
            currency: self.currency.expect("validated currency"),
            tax_currency: self.tax_currency,
            tax_currency_amount: self.tax_currency_amount,
            previous_invoice_hash: self
                .previous_invoice_hash
                .expect("validated previous invoice hash"),
            invoice_counter: self.invoice_counter.expect("validated invoice counter"),
            billing_references: self.billing_references,
            notes: self.notes,
            purchase_order_id: self.purchase_order_id,
            contract_id: self.contract_id,
            seller: self.seller.expect("validated seller"),
            buyer: self.buyer,
            delivery: self.delivery,
            line_items: self.line_items,
            payment_means: self.payment_means,
            vat_category: self.vat_category.expect("validated vat category"),
            flags: self.flags,
            invoice_level_charge: self.invoice_level_charge,
            invoice_level_discount: self.invoice_level_discount,
            adjustment_vat_rate: self.adjustment_vat_rate,
            allowance_reason: self.allowance_reason,
            adjustments,
        };

        if self.prepaid_amount < Decimal::ZERO
            || self.prepaid_amount.scale() > 2
            || self.payable_rounding_amount.scale() > 2
            || invoice
                .tax_currency_amount
                .is_some_and(|v| v < Decimal::ZERO || v.scale() > 2)
            || invoice.adjustments.iter().any(|a| {
                !(1..=super::AllowanceCharge::MAX_TAX_CATEGORY_COUNT)
                    .contains(&a.tax_category_count)
            })
        {
            return Err(ValidationError::new(vec![ValidationIssue::new(
                InvoiceField::InvoiceLevelCharge,
                ValidationKind::OutOfRange,
                None,
            )])
            .into());
        }
        let mut totals = InvoiceTotals::from_data(&invoice)?;
        let mut seen_groups = Vec::new();
        for supplied in self.supplied_breakdowns {
            let key = (supplied.category, supplied.rate);
            if seen_groups.contains(&key) {
                return Err(ValidationError::new(vec![ValidationIssue::new(
                    InvoiceField::VatCategory,
                    ValidationKind::Mismatch,
                    None,
                )])
                .into());
            }
            seen_groups.push(key);
            let expected = totals
                .groups
                .iter_mut()
                .find(|g| (g.category, g.rate) == key)
                .ok_or_else(|| {
                    ValidationError::new(vec![ValidationIssue::new(
                        InvoiceField::VatCategory,
                        ValidationKind::Mismatch,
                        None,
                    )])
                })?;
            supplied.check_against(expected, invoice.line_items.len())?;
            *expected = supplied;
        }
        totals.tax_amount = totals
            .groups
            .iter()
            .try_fold(Decimal::ZERO, |sum, g| sum.add(g.tax_amount))?;
        totals.tax_inclusive_amount = totals.taxable_amount.add(totals.tax_amount)?;
        if invoice
            .tax_currency
            .as_ref()
            .is_some_and(|c| c == &invoice.currency)
            && invoice
                .tax_currency_amount
                .is_some_and(|amount| amount != totals.tax_amount)
        {
            return Err(ValidationError::new(vec![ValidationIssue::new(
                InvoiceField::Currency,
                ValidationKind::Mismatch,
                None,
            )])
            .into());
        }
        if invoice.tax_currency.is_none() {
            invoice.tax_currency = Some(invoice.currency.clone());
        }
        if invoice.tax_currency_amount.is_none() {
            invoice.tax_currency_amount = Some(totals.tax_amount);
        }

        if invoice
            .line_items
            .iter()
            .any(|l| l.is_prepayment_adjustment())
        {
            let expected = invoice
                .line_items
                .iter()
                .filter(|l| l.is_prepayment_adjustment())
                .flat_map(|l| l.tax_subtotals())
                .try_fold(Decimal::ZERO, |sum, g| {
                    sum.add(g.taxable_amount)?.add(g.tax_amount)
                })?;
            super::line_builder::check_supplied(
                InvoiceField::InvoiceLevelDiscount,
                self.prepaid_amount,
                expected,
                Decimal::ZERO,
            )?;
        }
        totals.prepaid_amount = self.prepaid_amount;
        totals.payable_rounding_amount = self.payable_rounding_amount;
        totals.payable_amount = totals
            .tax_inclusive_amount
            .sub(self.prepaid_amount)?
            .add(self.payable_rounding_amount)?;
        let mut assigned = Vec::new();
        for (category, rate, exemption) in self.exemptions {
            if let Some((_, _, previous)) = assigned
                .iter()
                .find(|(c, r, _)| *c == category && *r == rate)
            {
                if previous != &exemption {
                    return Err(ValidationError::new(vec![ValidationIssue::new(
                        InvoiceField::VatCategory,
                        ValidationKind::Mismatch,
                        None,
                    )])
                    .into());
                }
            } else {
                assigned.push((category, rate, exemption.clone()));
            }
            let group = totals
                .groups
                .iter_mut()
                .find(|g| g.category == category && g.rate == rate)
                .ok_or_else(|| {
                    ValidationError::new(vec![ValidationIssue::new(
                        InvoiceField::VatCategory,
                        ValidationKind::Mismatch,
                        None,
                    )])
                })?;
            if seen_groups.contains(&(category, rate)) && group.exemption != exemption {
                return Err(ValidationError::new(vec![ValidationIssue::new(
                    InvoiceField::VatCategory,
                    ValidationKind::Mismatch,
                    None,
                )])
                .into());
            }
            group.exemption = exemption;
        }
        Ok(FinalizedInvoice {
            totals,
            data: invoice,
        })
    }
}

impl FinalizedInvoice {
    pub(super) fn preserve_import_tax_layout(
        &mut self,
        currency: Option<super::CurrencyCode>,
        accounting_total: bool,
        document_first: bool,
    ) {
        self.data.tax_currency = currency;
        if !accounting_total {
            self.data.tax_currency_amount = None;
        }
        self.data.document_tax_first = document_first;
    }
    pub(super) fn preserve_import_time(
        &mut self,
        date: super::InvoiceDate,
        time: super::InvoiceTime,
    ) {
        if !time.as_str().ends_with('Z') || time.as_str() == "24:00:00Z" {
            self.data.issue_date_override = Some(date);
            self.data.issue_time_override = Some(time);
        }
    }
    pub(super) fn refresh_accounting_tax(&mut self) {
        if self.data.tax_currency.as_ref() == Some(&self.data.currency) {
            self.data.tax_currency_amount = Some(self.totals.tax_amount);
        }
    }

    /// Access the underlying invoice data.
    pub fn data(&self) -> &InvoiceData {
        &self.data
    }

    /// Totals computed by [`InvoiceBuilder::build`].
    pub fn totals(&self) -> &InvoiceTotals {
        &self.totals
    }

    /// Hash the XML serialization of this finalized model.
    ///
    /// # Errors
    /// Returns [`SigningError`] if XML serialization, parsing, or hashing fails.
    pub fn hash_base64(&self) -> Result<String, SigningError> {
        let xml = self
            .to_xml()
            .map_err(|e| SigningError::SigningError(e.to_string()))?;
        invoice_hash_base64_from_xml(&xml)
    }

    /// Sign the invoice with the provided signer.
    ///
    /// Produces signature material without running XSD or business-rule validation.
    ///
    /// # Errors
    /// Returns [`SigningError`] if signing or XML generation fails.
    pub fn sign(self, signer: &InvoiceSigner) -> Result<SignedInvoice, SigningError> {
        signer.sign(self)
    }

    pub(crate) fn sign_with_bundle(
        self,
        signed_properties: SignedProperties,
        signed_xml: String,
    ) -> QrResult<SignedInvoice> {
        let qr_code = super::validation::integrity::signed_qr(&signed_xml)
            .map_err(QrCodeError::InvalidField)?;
        Ok(SignedInvoice {
            finalized: self,
            signed_properties,
            qr_code,
            signed_xml,
        })
    }
}

impl SignedInvoice {
    /// Access the underlying invoice data.
    pub fn data(&self) -> &InvoiceData {
        self.finalized.data()
    }

    /// Totals of the underlying finalized invoice.
    pub fn totals(&self) -> &InvoiceTotals {
        self.finalized.totals()
    }

    /// Signature metadata (hashes, certificate details and signing time).
    pub fn signed_properties(&self) -> &SignedProperties {
        &self.signed_properties
    }

    /// Borrow the exact Base64 payload embedded in the retained XML.
    pub fn qr_code(&self) -> &str {
        &self.qr_code
    }

    /// Borrow the exact stored signed XML without reformatting it.
    pub fn xml(&self) -> &str {
        &self.signed_xml
    }

    /// Consume the invoice and return its stored XML without copying it.
    pub fn into_xml(self) -> String {
        self.signed_xml
    }

    /// Invoice UUID.
    pub fn uuid(&self) -> &InvoiceUuid {
        self.finalized.data().uuid()
    }

    /// Return the embedded digest without recomputing or verifying it.
    ///
    /// Equality with `hash_base64()` alone does not verify signature authenticity.
    pub fn invoice_hash(&self) -> &InvoiceHash {
        self.signed_properties.invoice_hash()
    }

    /// Recompute the invoice digest from the exact stored XML using ZATCA exclusions.
    ///
    /// # Errors
    /// Returns [`SigningError`] if XML parsing or hashing fails.
    pub fn hash_base64(&self) -> Result<String, SigningError> {
        invoice_hash_base64_from_xml(self.xml())
    }

    /// Base64 DER ECDSA signature over the invoice hash.
    pub fn signature(&self) -> &str {
        self.signed_properties.signature()
    }

    /// Base64 DER public key of the signing certificate.
    pub fn public_key(&self) -> &str {
        self.signed_properties.public_key()
    }

    /// Base64 signature value of the signing certificate, if available.
    pub fn zatca_key_signature(&self) -> Option<&str> {
        self.signed_properties.zatca_key_signature()
    }

    /// Base64-encode the stored signed XML, as submitted to the ZATCA API.
    pub fn to_xml_base64(&self) -> String {
        use base64ct::{Base64, Encoding};
        Base64::encode_string(self.signed_xml.as_bytes())
    }
}

pub(super) trait InvoiceView {
    /// Invoice data.
    fn data(&self) -> &InvoiceData;
    /// Computed totals.
    fn totals(&self) -> &InvoiceTotals;
    /// Optional QR payload.
    fn qr_code(&self) -> Option<&str>;
}

impl InvoiceView for FinalizedInvoice {
    fn data(&self) -> &InvoiceData {
        self.data()
    }

    fn totals(&self) -> &InvoiceTotals {
        self.totals()
    }

    fn qr_code(&self) -> Option<&str> {
        None
    }
}

impl InvoiceView for SignedInvoice {
    fn data(&self) -> &InvoiceData {
        self.data()
    }

    fn totals(&self) -> &InvoiceTotals {
        self.totals()
    }

    fn qr_code(&self) -> Option<&str> {
        Some(self.qr_code())
    }
}
