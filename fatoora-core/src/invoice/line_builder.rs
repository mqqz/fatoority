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
    pub fn amount(amount: Decimal) -> Self {
        Self {
            amount: Some(amount),
            base_amount: None,
            percentage: None,
            reason: None,
            reason_code: None,
        }
    }
    pub fn percentage(base_amount: Decimal, percentage: Decimal) -> Self {
        Self {
            amount: None,
            base_amount: Some(base_amount),
            percentage: Some(percentage),
            reason: None,
            reason_code: None,
        }
    }
    pub fn supplied_amount(mut self, amount: Decimal) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn base_amount(mut self, amount: Decimal) -> Self {
        self.base_amount = Some(amount);
        self
    }
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
    pub fn reason_code(mut self, code: impl Into<String>) -> Self {
        self.reason_code = Some(code.into());
        self
    }
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
    pub fn allowance_charge_reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
    pub fn allowance_charge_reason_code(&self) -> Option<&str> {
        self.reason_code.as_deref()
    }
    pub fn calculation_base_amount(&self) -> Option<Decimal> {
        self.base_amount
    }
    pub fn calculation_percentage(&self) -> Option<Decimal> {
        self.percentage
    }
    pub(crate) fn finalize(mut self) -> Result<Self> {
        self.amount = Some(self.value()?);
        Ok(self)
    }
}

/// One finalized line allowance or charge, in insertion order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineAdjustment {
    pub charge: bool,
    pub adjustment: Adjustment,
}

/// A document adjustment has its own VAT allocation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentAdjustment {
    pub adjustment: Adjustment,
    pub category: VatCategory,
    pub rate: Decimal,
    pub exemption: TaxExemption,
}
impl DocumentAdjustment {
    pub(crate) fn into_entry(self, charge: bool) -> Result<AllowanceCharge> {
        Ok(AllowanceCharge {
            omitted_rate: false,
            tax_category_count: 1,
            charge,
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
    pub fn new(adjustment: Adjustment, category: VatCategory, rate: Decimal) -> Self {
        Self {
            adjustment,
            category,
            rate,
            exemption: TaxExemption::default(),
        }
    }
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.adjustment = self.adjustment.reason(value);
        self
    }
    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.adjustment = self.adjustment.reason_code(value);
        self
    }
    pub fn exemption(mut self, value: TaxExemption) -> Self {
        self.exemption = value;
        self
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
    ($($name:ident => $field:ident),* $(,)?) => { $(pub fn $name(mut self, value: Decimal) -> Self { self.$field = Some(value); self })* };
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
    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    pub fn invoiced_quantity_unit_code(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }
    pub fn item_price_base_quantity_unit_code(mut self, value: impl Into<String>) -> Self {
        self.basis_unit = Some(value.into());
        self
    }
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
    decimal_setters! { invoiced_quantity => quantity, item_net_price => price, item_gross_price => gross_price,
    item_price_discount => discount, item_price_base_quantity => basis, line_net_amount => net,
    line_vat_amount => vat, line_amount_with_vat => gross, vat_rate => rate }
    pub fn vat_category(mut self, value: VatCategory) -> Self {
        self.category = Some(value);
        self
    }
    pub fn exemption(mut self, value: TaxExemption) -> Self {
        self.exemption = value;
        self
    }
    pub fn allowance(mut self, adjustment: Adjustment) -> Self {
        self.adjustments.push(LineAdjustment {
            charge: false,
            adjustment,
        });
        self
    }
    pub fn charge(mut self, adjustment: Adjustment) -> Self {
        self.adjustments.push(LineAdjustment {
            charge: true,
            adjustment,
        });
        self
    }
    pub fn document_reference(mut self, value: OriginalInvoiceRef) -> Self {
        self.references.push(value);
        self
    }
    pub fn tax_subtotal(mut self, value: VatBreakdown) -> Self {
        self.subtotals.push(value);
        self
    }
    pub fn item_seller_identifier(mut self, value: OtherId) -> Self {
        self.seller_id = Some(value);
        self
    }
    pub fn item_buyer_identifier(mut self, value: OtherId) -> Self {
        self.buyer_id = Some(value);
        self
    }
    pub fn item_standard_identifier(mut self, value: OtherId) -> Self {
        self.standard_id = Some(value);
        self
    }
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
            .map(|a| {
                Ok(LineAdjustment {
                    charge: a.charge,
                    adjustment: a.adjustment.finalize()?,
                })
            })
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
    pub fn builder() -> LineItemBuilder {
        LineItemBuilder::default()
    }
    pub fn item_name(&self) -> &str {
        &self.description
    }
    pub fn item_net_price(&self) -> Decimal {
        self.unit_price
    }
    pub fn item_gross_price(&self) -> Option<Decimal> {
        self.gross_price
    }
    pub fn item_price_discount(&self) -> Option<Decimal> {
        self.price_discount
    }
    pub fn item_price_base_quantity(&self) -> Option<Decimal> {
        self.price_base_quantity
    }
    pub fn item_price_base_quantity_unit_code(&self) -> Option<&str> {
        self.price_base_unit.as_deref()
    }
    pub fn adjustments(&self) -> &[LineAdjustment] {
        &self.adjustments
    }
    pub fn item_seller_identifier(&self) -> Option<&OtherId> {
        self.seller_item_id.as_ref()
    }
    pub fn item_buyer_identifier(&self) -> Option<&OtherId> {
        self.buyer_item_id.as_ref()
    }
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
            amount = if a.charge {
                amount.add(a.adjustment.value()?)?
            } else {
                amount.sub(a.adjustment.value()?)?
            };
        }
        Ok(amount)
    }
}
