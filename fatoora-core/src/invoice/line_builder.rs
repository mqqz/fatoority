//! Line construction and shared adjustment arithmetic.
use super::*;

fn invalid(field: InvoiceField, kind: ValidationKind) -> InvoiceError {
    ValidationError::new(vec![ValidationIssue::new(field, kind, None)]).into()
}

/// An allowance or charge calculation, independent of its placement and VAT treatment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Adjustment {
    pub(crate) amount: Option<Decimal>,
    pub(crate) base_amount: Option<Decimal>,
    pub(crate) percentage: Option<Decimal>,
    pub(crate) reason: Option<String>,
    pub(crate) reason_code: Option<String>,
}
impl Adjustment {
    /// Creates a fixed-amount adjustment with no calculation base or percentage.
    pub fn amount(amount: Decimal) -> Self {
        Self {
            amount: Some(amount),
            base_amount: None,
            percentage: None,
            reason: None,
            reason_code: None,
        }
    }
    /// Creates an adjustment calculated as `base_amount * percentage / 100`, rounded to two decimals.
    pub fn percent_of(base_amount: Decimal, percentage: Decimal) -> Self {
        Self {
            amount: None,
            base_amount: Some(base_amount),
            percentage: Some(percentage),
            reason: None,
            reason_code: None,
        }
    }
    /// Supplies an explicit amount; when a base and percentage are also set, [`value`](Self::value) checks it against the calculated amount.
    pub fn supplied_amount(mut self, amount: Decimal) -> Self {
        self.amount = Some(amount);
        self
    }
    /// Sets the calculation base amount (`cbc:BaseAmount`).
    pub fn with_base_amount(mut self, amount: Decimal) -> Self {
        self.base_amount = Some(amount);
        self
    }
    /// Sets the free-text allowance/charge reason (`cbc:AllowanceChargeReason`).
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
    /// Sets the allowance/charge reason code (`cbc:AllowanceChargeReasonCode`).
    pub fn with_reason_code(mut self, code: impl Into<String>) -> Self {
        self.reason_code = Some(code.into());
        self
    }
    /// Returns the adjustment amount: the supplied amount if set, otherwise `base * percentage / 100` rounded to two decimals.
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] (field [`InvoiceField::InvoiceLevelDiscount`]) when:
    /// - the base amount or percentage is negative or has more than two decimals, or the resulting amount does
    ///   ([`ValidationKind::OutOfRange`]);
    /// - only one of base amount and percentage is set, or neither an amount nor a base/percentage pair is set
    ///   ([`ValidationKind::Missing`]);
    /// - a supplied amount differs from the calculated amount ([`ValidationKind::Mismatch`]).
    ///
    /// Returns [`InvoiceError::Decimal`] if the calculation overflows the decimal range.
    pub fn value(&self) -> Result<Decimal> {
        let field = InvoiceField::InvoiceLevelDiscount;
        if self
            .base_amount
            .is_some_and(|v| v < Decimal::ZERO || v.scale() > 2)
            || self
                .percentage
                .is_some_and(|v| v < Decimal::ZERO || v.scale() > 2)
        {
            return Err(invalid(field, ValidationKind::OutOfRange));
        }
        let calculated = match (self.base_amount, self.percentage) {
            (Some(base), Some(percent)) => Some(base.product_rounded(percent, true)?),
            (None, None) => None,
            _ => return Err(invalid(field, ValidationKind::Missing)),
        };
        let amount = self
            .amount
            .or(calculated)
            .ok_or_else(|| invalid(field, ValidationKind::Missing))?;
        if amount < Decimal::ZERO || amount.scale() > 2 {
            return Err(invalid(field, ValidationKind::OutOfRange));
        }
        if let Some(expected) = calculated {
            check_supplied(field, amount, expected, Decimal::ZERO)?;
        }
        Ok(amount)
    }
    /// Returns the free-text reason, if set.
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
    /// Returns the reason code, if set.
    pub fn reason_code(&self) -> Option<&str> {
        self.reason_code.as_deref()
    }
    /// Returns the calculation base amount, if set.
    pub fn base_amount(&self) -> Option<Decimal> {
        self.base_amount
    }
    /// Returns the calculation percentage, if set.
    pub fn percentage(&self) -> Option<Decimal> {
        self.percentage
    }
    /// Returns the free-text reason, if set.
    #[deprecated(since = "0.6.0", note = "use `reason` instead")]
    pub fn allowance_charge_reason(&self) -> Option<&str> {
        self.reason()
    }
    /// Returns the reason code, if set.
    #[deprecated(since = "0.6.0", note = "use `reason_code` instead")]
    pub fn allowance_charge_reason_code(&self) -> Option<&str> {
        self.reason_code()
    }
    /// Returns the calculation base amount, if set.
    #[deprecated(since = "0.6.0", note = "use `base_amount` instead")]
    pub fn calculation_base_amount(&self) -> Option<Decimal> {
        self.base_amount()
    }
    /// Returns the calculation percentage, if set.
    #[deprecated(since = "0.6.0", note = "use `percentage` instead")]
    pub fn calculation_percentage(&self) -> Option<Decimal> {
        self.percentage()
    }
    pub(crate) fn finalize(mut self) -> Result<Self> {
        self.amount = Some(self.value()?);
        Ok(self)
    }
}

/// One finalized line allowance or charge, in insertion order.
///
/// Read from [`LineItem::adjustments`]; add one with
/// [`LineItemBuilder::allowance`] or [`LineItemBuilder::charge`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "LineAdjustmentRepr")]
pub struct LineAdjustment {
    /// A charge increases the line amount; an allowance decreases it (`cbc:ChargeIndicator`).
    kind: AdjustmentKind,
    /// The amount calculation and reason.
    adjustment: Adjustment,
}
/// Deserialization shape of [`LineAdjustment`]: accepts `kind`, or the 0.5 `charge` boolean.
#[derive(Deserialize)]
struct LineAdjustmentRepr {
    #[serde(default)]
    kind: Option<AdjustmentKind>,
    #[serde(default)]
    charge: Option<bool>,
    adjustment: Adjustment,
}
impl TryFrom<LineAdjustmentRepr> for LineAdjustment {
    type Error = &'static str;
    fn try_from(repr: LineAdjustmentRepr) -> std::result::Result<Self, Self::Error> {
        Ok(Self::new(
            AdjustmentKind::from_serialized(repr.kind, repr.charge)?,
            repr.adjustment,
        ))
    }
}
impl LineAdjustment {
    pub(crate) fn new(kind: AdjustmentKind, adjustment: Adjustment) -> Self {
        Self { kind, adjustment }
    }
    /// Returns whether this is an allowance (reduces the line amount) or a charge (increases it).
    pub fn kind(&self) -> AdjustmentKind {
        self.kind
    }
    /// Returns the amount calculation and reason.
    pub fn adjustment(&self) -> &Adjustment {
        &self.adjustment
    }
}

/// A document adjustment has its own VAT allocation.
///
/// # Examples
/// ```rust
/// use fatoora_core::Decimal;
/// use fatoora_core::invoice::{Adjustment, DocumentAdjustment, VatCategory};
///
/// let discount = DocumentAdjustment::new(
///     Adjustment::amount(Decimal::parse("5.00")?),
///     VatCategory::Standard,
///     Decimal::parse("15")?,
/// )
/// .with_reason("Loyalty discount")
/// .with_reason_code("95");
/// assert_eq!(discount.reason(), Some("Loyalty discount"));
/// assert_eq!(discount.category(), VatCategory::Standard);
/// # Ok::<(), fatoora_core::invoice::InvoiceError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentAdjustment {
    /// The amount calculation and reason.
    adjustment: Adjustment,
    /// VAT category applied to the adjustment.
    category: VatCategory,
    /// VAT rate (percent) applied to the adjustment.
    rate: Decimal,
    /// Exemption reason, for non-standard VAT categories.
    exemption: TaxExemption,
    /// Omit the VAT rate (`cbc:Percent`) from XML; only valid for an out-of-scope
    /// adjustment with a zero rate.
    #[serde(default)]
    omitted_rate: bool,
    /// Number of identical `cac:TaxCategory` elements to emit, as used by the bundled SDK
    /// fixtures. Must be in `1..=AllowanceCharge::MAX_TAX_CATEGORY_COUNT`; checked in `build`.
    #[serde(default = "one_tax_category")]
    tax_category_count: usize,
}
fn one_tax_category() -> usize {
    1
}
impl DocumentAdjustment {
    pub(crate) fn into_entry(self, kind: AdjustmentKind) -> Result<AllowanceCharge> {
        Ok(AllowanceCharge {
            omitted_rate: self.omitted_rate,
            tax_category_count: self.tax_category_count,
            kind,
            amount: self.adjustment.value()?,
            base_amount: self.adjustment.base_amount,
            percentage: self.adjustment.percentage,
            reason: self.adjustment.reason,
            reason_code: self.adjustment.reason_code,
            category: self.category,
            rate: self.rate,
            exemption: self.exemption,
        })
    }
    /// Creates a document adjustment with the given VAT category and rate and no exemption reason.
    pub fn new(adjustment: Adjustment, category: VatCategory, rate: Decimal) -> Self {
        Self {
            adjustment,
            category,
            rate,
            exemption: TaxExemption::default(),
            omitted_rate: false,
            tax_category_count: 1,
        }
    }
    /// Omits the VAT rate (`cbc:Percent`) from XML.
    ///
    /// Only an out-of-scope adjustment with a zero rate may omit its rate;
    /// [`InvoiceBuilder::build`](super::InvoiceBuilder::build) rejects any other combination.
    pub fn omit_vat_rate(mut self) -> Self {
        self.omitted_rate = true;
        self
    }
    /// Returns `false` if the VAT rate (`cbc:Percent`) is omitted from XML.
    pub fn has_vat_rate(&self) -> bool {
        !self.omitted_rate
    }
    /// Sets how many identical `cac:TaxCategory` elements are emitted (default `1`).
    ///
    /// [`InvoiceBuilder::build`](super::InvoiceBuilder::build) rejects a count outside
    /// `1..=`[`AllowanceCharge::MAX_TAX_CATEGORY_COUNT`].
    pub fn with_tax_category_count(mut self, count: usize) -> Self {
        self.tax_category_count = count;
        self
    }
    /// Sets the free-text allowance/charge reason (`cbc:AllowanceChargeReason`).
    pub fn with_reason(mut self, value: impl Into<String>) -> Self {
        self.adjustment = self.adjustment.with_reason(value);
        self
    }
    /// Sets the allowance/charge reason code (`cbc:AllowanceChargeReasonCode`).
    pub fn with_reason_code(mut self, value: impl Into<String>) -> Self {
        self.adjustment = self.adjustment.with_reason_code(value);
        self
    }
    /// Sets the VAT exemption code and reason.
    pub fn with_exemption(mut self, value: TaxExemption) -> Self {
        self.exemption = value;
        self
    }
    /// Returns the amount calculation and reason.
    pub fn adjustment(&self) -> &Adjustment {
        &self.adjustment
    }
    /// Returns the free-text reason, if set.
    pub fn reason(&self) -> Option<&str> {
        self.adjustment.reason()
    }
    /// Returns the reason code, if set.
    pub fn reason_code(&self) -> Option<&str> {
        self.adjustment.reason_code()
    }
    /// Returns the VAT category.
    pub fn category(&self) -> VatCategory {
        self.category
    }
    /// Returns the VAT rate (percent).
    pub fn rate(&self) -> Decimal {
        self.rate
    }
    /// Returns the VAT exemption code and reason.
    pub fn exemption(&self) -> &TaxExemption {
        &self.exemption
    }
    /// Returns how many identical `cac:TaxCategory` elements are emitted.
    pub fn tax_category_count(&self) -> usize {
        self.tax_category_count
    }
}

/// Converts a finalized entry back into a document adjustment.
///
/// The amount, base amount, percentage, reasons, VAT category and rate, exemption,
/// omitted rate and tax category count are kept. The [`AdjustmentKind`] is not: pass the
/// result to [`InvoiceBuilder::allowance`](super::InvoiceBuilder::allowance) or
/// [`InvoiceBuilder::charge`](super::InvoiceBuilder::charge) according to `entry.kind()`.
impl From<AllowanceCharge> for DocumentAdjustment {
    fn from(entry: AllowanceCharge) -> Self {
        Self {
            adjustment: Adjustment {
                amount: Some(entry.amount),
                base_amount: entry.base_amount,
                percentage: entry.percentage,
                reason: entry.reason,
                reason_code: entry.reason_code,
            },
            category: entry.category,
            rate: entry.rate,
            exemption: entry.exemption,
            omitted_rate: entry.omitted_rate,
            tax_category_count: entry.tax_category_count,
        }
    }
}

pub(super) fn check_supplied(
    field: InvoiceField,
    supplied: Decimal,
    expected: Decimal,
    tolerance: Decimal,
) -> Result<()> {
    let delta = supplied.sub(expected)?;
    if supplied.scale() > 2 || delta > tolerance || delta < Decimal::ZERO.sub(tolerance)? {
        return Err(ValidationError::new(vec![ValidationIssue::mismatch(
            field, supplied, expected,
        )])
        .into());
    }
    Ok(())
}

/// Named line construction. Build calculates absent amounts and checks supplied amounts.
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct LineItemBuilder {
    name: Option<String>,
    quantity: Option<Decimal>,
    unit: Option<String>,
    price: Option<Decimal>,
    gross_price: Option<Decimal>,
    discount: Option<Decimal>,
    basis: Option<Decimal>,
    basis_unit: Option<String>,
    net: Option<Decimal>,
    vat: Option<Decimal>,
    gross: Option<Decimal>,
    omit_tax_total: bool,
    omit_gross_amount: bool,
    category: Option<VatCategory>,
    rate: Option<Decimal>,
    id: Option<String>,
    exemption: TaxExemption,
    adjustments: Vec<LineAdjustment>,
    references: Vec<OriginalInvoiceRef>,
    subtotals: Vec<VatBreakdown>,
    seller_id: Option<OtherId>,
    buyer_id: Option<OtherId>,
    standard_id: Option<OtherId>,
}
macro_rules! decimal_setters {
    ($($(#[$meta:meta])* $name:ident => $field:ident),* $(,)?) => { $($(#[$meta])* pub fn $name(mut self, value: Decimal) -> Self { self.$field = Some(value); self })* };
}
impl LineItemBuilder {
    /// Control whether the line tax container is emitted; omitted tax must be zero.
    pub fn tax_total_present(mut self, present: bool) -> Self {
        self.omit_tax_total = !present;
        self
    }
    /// Control whether the line amount including VAT is emitted.
    pub fn gross_amount_present(mut self, present: bool) -> Self {
        self.omit_gross_amount = !present;
        self
    }
    /// Sets the item name (required).
    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets the unit-of-measure code for the invoiced quantity; must not be blank.
    pub fn invoiced_quantity_unit_code(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }
    /// Sets the unit-of-measure code for the price base quantity; requires [`item_price_base_quantity`](Self::item_price_base_quantity) and must not be blank.
    pub fn item_price_base_quantity_unit_code(mut self, value: impl Into<String>) -> Self {
        self.basis_unit = Some(value.into());
        self
    }
    /// Sets the line identifier; must not be blank.
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
    decimal_setters! {
        /// Sets the invoiced quantity (required, non-negative).
        invoiced_quantity => quantity,
        /// Sets the item net price per base quantity; if omitted it is derived from gross price minus discount.
        item_net_price => price,
        /// Sets the item gross price before discount; requires [`item_price_discount`](Self::item_price_discount).
        item_gross_price => gross_price,
        /// Sets the per-unit price discount (gross price minus net price).
        item_price_discount => discount,
        /// Sets the quantity the item price applies to (defaults to 1).
        item_price_base_quantity => basis,
        /// Supplies the line net amount; checked against the calculated amount within `0.01`.
        line_net_amount => net,
        /// Supplies the line VAT amount instead of calculating `net * rate / 100`.
        line_vat_amount => vat,
        /// Supplies the line amount including VAT; must equal net plus VAT exactly.
        line_amount_with_vat => gross,
        /// Sets the VAT rate in percent (0 to 100, at most two decimals); optional only for out-of-scope lines.
        vat_rate => rate,
    }
    /// Sets the VAT category (required).
    pub fn vat_category(mut self, value: VatCategory) -> Self {
        self.category = Some(value);
        self
    }
    /// Sets the VAT exemption reason.
    pub fn exemption(mut self, value: TaxExemption) -> Self {
        self.exemption = value;
        self
    }
    /// Appends a line-level allowance, which reduces the line net amount.
    pub fn allowance(mut self, adjustment: Adjustment) -> Self {
        self.adjustments
            .push(LineAdjustment::new(AdjustmentKind::Allowance, adjustment));
        self
    }
    /// Appends a line-level charge, which increases the line net amount.
    pub fn charge(mut self, adjustment: Adjustment) -> Self {
        self.adjustments
            .push(LineAdjustment::new(AdjustmentKind::Charge, adjustment));
        self
    }
    /// Appends a document reference for this line.
    pub fn document_reference(mut self, value: OriginalInvoiceRef) -> Self {
        self.references.push(value);
        self
    }
    /// Appends a line-level VAT subtotal.
    pub fn tax_subtotal(mut self, value: VatBreakdown) -> Self {
        self.subtotals.push(value);
        self
    }
    /// Sets the seller's identifier for the item.
    pub fn item_seller_identifier(mut self, value: OtherId) -> Self {
        self.seller_id = Some(value);
        self
    }
    /// Sets the buyer's identifier for the item.
    pub fn item_buyer_identifier(mut self, value: OtherId) -> Self {
        self.buyer_id = Some(value);
        self
    }
    /// Sets the standard (e.g. GTIN) identifier for the item.
    pub fn item_standard_identifier(mut self, value: OtherId) -> Self {
        self.standard_id = Some(value);
        self
    }
    /// Builds the line, calculating any amounts not supplied and checking supplied ones.
    ///
    /// The net amount defaults to `quantity * net_price / base_quantity` plus charges minus allowances,
    /// and the VAT amount to `net * rate / 100`, each rounded to two decimals. A missing net price is
    /// derived from gross price minus discount, and a supplied net amount may differ from the
    /// calculated one by at most `0.01`.
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] when:
    /// - the item name, quantity, VAT category, unit price (net, or gross plus discount) or VAT rate
    ///   (unless the category is out of scope) is missing ([`ValidationKind::Missing`]);
    /// - the item name, line ID or a unit code is blank ([`ValidationKind::Empty`]);
    /// - gross price minus discount does not equal the net price ([`ValidationKind::Mismatch`]);
    /// - a quantity, price, discount or rate is negative, the rate exceeds 100 or has more than two
    ///   decimals, a non-standard category has a non-zero rate, a base-quantity unit is set without a base
    ///   quantity, or the net or VAT amount is negative ([`ValidationKind::OutOfRange`]);
    /// - a supplied net, VAT-inclusive or adjustment amount disagrees with its calculated value;
    /// - an omitted tax total is combined with a non-zero VAT amount or tax subtotals.
    ///
    /// Returns [`InvoiceError::Decimal`] if an intermediate calculation overflows the decimal range.
    pub fn build(self) -> Result<LineItem> {
        let missing = |field| invalid(field, ValidationKind::Missing);
        let name = self
            .name
            .ok_or_else(|| missing(InvoiceField::LineItemDescription))?;
        let quantity = self
            .quantity
            .ok_or_else(|| missing(InvoiceField::LineItemQuantity))?;
        let category = self
            .category
            .ok_or_else(|| missing(InvoiceField::VatCategory))?;
        if self.unit.as_ref().is_some_and(|v| v.trim().is_empty())
            || self
                .basis_unit
                .as_ref()
                .is_some_and(|v| v.trim().is_empty())
        {
            return Err(invalid(
                InvoiceField::LineItemUnitCode,
                ValidationKind::Empty,
            ));
        }
        if self.gross_price.is_some() && self.discount.is_none() {
            return Err(missing(InvoiceField::LineItemUnitPrice));
        }
        let rate = match self.rate {
            Some(rate) => rate,
            None if category == VatCategory::OutOfScope => Decimal::ZERO,
            None => return Err(missing(InvoiceField::LineItemVatRate)),
        };
        let price = match (self.price, self.gross_price, self.discount) {
            (Some(p), _, _) => p,
            (None, Some(g), Some(d)) => g.sub(d)?,
            _ => return Err(missing(InvoiceField::LineItemUnitPrice)),
        };
        let gross_price = match (self.gross_price, self.discount) {
            (None, Some(d)) => Some(price.add(d)?),
            (g, _) => g,
        };
        if let (Some(g), Some(d)) = (gross_price, self.discount)
            && g.sub(d)? != price
        {
            return Err(invalid(
                InvoiceField::LineItemUnitPrice,
                ValidationKind::Mismatch,
            ));
        }
        if name.trim().is_empty() || self.id.as_ref().is_some_and(|v| v.trim().is_empty()) {
            return Err(invalid(
                InvoiceField::LineItemDescription,
                ValidationKind::Empty,
            ));
        }
        if quantity < Decimal::ZERO
            || price < Decimal::ZERO
            || rate < Decimal::ZERO
            || rate > Decimal::from(100)
            || rate.scale() > 2
            || self.discount.is_some_and(|v| v < Decimal::ZERO)
            || gross_price.is_some_and(|v| v < Decimal::ZERO)
            || (category != VatCategory::Standard && rate != Decimal::ZERO)
            || self.basis_unit.is_some() && self.basis.is_none()
        {
            return Err(invalid(
                InvoiceField::LineItemUnitPrice,
                ValidationKind::OutOfRange,
            ));
        }
        let adjustments = self
            .adjustments
            .into_iter()
            .map(|a| Ok(LineAdjustment::new(a.kind, a.adjustment.finalize()?)))
            .collect::<Result<Vec<_>>>()?;
        let mut line = LineItem {
            omitted_tax_amount: self.omit_tax_total,
            omitted_gross_amount: self.omit_tax_total || self.omit_gross_amount,
            omitted_vat_rate: self.rate.is_none(),
            description: name,
            quantity,
            unit_code: self.unit.unwrap_or_default(),
            unit_price: price,
            total_amount: Decimal::ZERO,
            vat_amount: Decimal::ZERO,
            vat_rate: rate,
            vat_category: category,
            id: self.id,
            exemption: self.exemption,
            document_references: self.references,
            tax_subtotals: self.subtotals,
            price_base_quantity: self.basis,
            price_base_unit: self.basis_unit,
            price_discount: self.discount,
            gross_price,
            adjustments,
            seller_item_id: self.seller_id,
            buyer_item_id: self.buyer_id,
            standard_item_id: self.standard_id,
        };
        let expected = line.expected_net()?;
        line.total_amount = self.net.unwrap_or(expected);
        check_supplied(
            InvoiceField::LineItemTotalAmount,
            line.total_amount,
            expected,
            "0.01".parse()?,
        )?;
        if line.total_amount < Decimal::ZERO {
            return Err(invalid(
                InvoiceField::LineItemTotalAmount,
                ValidationKind::OutOfRange,
            ));
        }
        line.vat_amount = self
            .vat
            .unwrap_or(line.total_amount.product_rounded(rate, true)?);
        if line.vat_amount < Decimal::ZERO || line.vat_amount.scale() > 2 {
            return Err(invalid(
                InvoiceField::LineItemVatAmount,
                ValidationKind::OutOfRange,
            ));
        }
        if let Some(gross) = self.gross {
            check_supplied(
                InvoiceField::LineItemTotalAmount,
                gross,
                line.total_amount.add(line.vat_amount)?,
                Decimal::ZERO,
            )?;
        }
        if let Some(field) = line.tax_presence_error() {
            return Err(invalid(field, ValidationKind::OutOfRange));
        }
        Ok(line)
    }
}
impl LineItem {
    pub(super) fn tax_presence_error(&self) -> Option<InvoiceField> {
        if self.omitted_vat_rate
            && (self.vat_category != VatCategory::OutOfScope || self.vat_rate != Decimal::ZERO)
        {
            return Some(InvoiceField::LineItemVatRate);
        }
        if self.omitted_tax_amount
            && (self.vat_amount != Decimal::ZERO
                || !self.tax_subtotals.is_empty()
                || !self.omitted_gross_amount)
        {
            return Some(InvoiceField::LineItemVatAmount);
        }
        None
    }
    /// Returns an empty [`LineItemBuilder`].
    pub fn builder() -> LineItemBuilder {
        LineItemBuilder::default()
    }
    /// Returns the item name.
    pub fn item_name(&self) -> &str {
        &self.description
    }
    /// Returns the item net price (per base quantity).
    pub fn item_net_price(&self) -> Decimal {
        self.unit_price
    }
    /// Returns the item gross price before the price discount, if set.
    pub fn item_gross_price(&self) -> Option<Decimal> {
        self.gross_price
    }
    /// Returns the per-unit price discount, if set.
    pub fn item_price_discount(&self) -> Option<Decimal> {
        self.price_discount
    }
    /// Returns the quantity the price applies to, if set (defaults to 1 in calculations).
    pub fn item_price_base_quantity(&self) -> Option<Decimal> {
        self.price_base_quantity
    }
    /// Returns the unit code of the price base quantity, if set.
    pub fn item_price_base_quantity_unit_code(&self) -> Option<&str> {
        self.price_base_unit.as_deref()
    }
    /// Returns the line allowances and charges in insertion order.
    pub fn adjustments(&self) -> &[LineAdjustment] {
        &self.adjustments
    }
    /// Returns the seller's identifier for the item, if set.
    pub fn item_seller_identifier(&self) -> Option<&OtherId> {
        self.seller_item_id.as_ref()
    }
    /// Returns the buyer's identifier for the item, if set.
    pub fn item_buyer_identifier(&self) -> Option<&OtherId> {
        self.buyer_item_id.as_ref()
    }
    /// Returns the standard identifier for the item, if set.
    pub fn item_standard_identifier(&self) -> Option<&OtherId> {
        self.standard_item_id.as_ref()
    }
    pub(super) fn expected_net(&self) -> Result<Decimal> {
        match (self.gross_price, self.price_discount) {
            (Some(gross), Some(discount)) => {
                if gross < Decimal::ZERO || discount < Decimal::ZERO {
                    return Err(invalid(
                        InvoiceField::LineItemUnitPrice,
                        ValidationKind::OutOfRange,
                    ));
                }
                if gross.sub(discount)? != self.unit_price {
                    return Err(invalid(
                        InvoiceField::LineItemUnitPrice,
                        ValidationKind::Mismatch,
                    ));
                }
            }
            (None, None) => {}
            _ => {
                return Err(invalid(
                    InvoiceField::LineItemUnitPrice,
                    ValidationKind::Missing,
                ));
            }
        }
        let mut amount = self.quantity.product_divided_rounded(
            self.unit_price,
            self.price_base_quantity.unwrap_or(Decimal::from(1)),
        )?;
        for a in &self.adjustments {
            amount = if a.kind.is_charge() {
                amount.add(a.adjustment.value()?)?
            } else {
                amount.sub(a.adjustment.value()?)?
            };
        }
        Ok(amount)
    }
}
