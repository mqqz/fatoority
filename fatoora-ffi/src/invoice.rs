//! Invoice construction and owned snapshots shared by all generated bindings.
use crate::common::ffi::BindingError;
use crate::common::{local_error, text};
use fatoora_core::invoice as inv_core;

fn vat(value: u8) -> Result<inv_core::VatCategory, Box<BindingError>> {
    match value {
        0 => Ok(inv_core::VatCategory::Exempt),
        1 => Ok(inv_core::VatCategory::Standard),
        2 => Ok(inv_core::VatCategory::Zero),
        3 => Ok(inv_core::VatCategory::OutOfScope),
        _ => Err(local_error(1, "invalid VAT category")),
    }
}
fn vat_code(value: inv_core::VatCategory) -> u8 {
    match value {
        inv_core::VatCategory::Exempt => 0,
        inv_core::VatCategory::Standard => 1,
        inv_core::VatCategory::Zero => 2,
        inv_core::VatCategory::OutOfScope => 3,
    }
}
fn decimal(value: &[u8], label: &str) -> Result<fatoora_core::Decimal, Box<BindingError>> {
    fatoora_core::Decimal::parse(text(value)?).map_err(|e| crate::common::context_error(e, label))
}
struct PartyData {
    name: String,
    address: Option<inv_core::Address>,
    vat_id: Option<inv_core::VatId>,
    identifiers: Vec<inv_core::OtherId>,
}
fn party<R: inv_core::PartyRole>(p: &inv_core::Party<R>) -> ffi::Party {
    ffi::Party(PartyData {
        name: p.name().into(),
        address: p.address().cloned(),
        vat_id: p.vat_id().cloned(),
        identifiers: p.identifiers().to_vec(),
    })
}

#[diplomat::bridge]
#[diplomat::abi_rename = "fatoora_{0}"]
#[diplomat::attr(cpp, namespace = "fatoora")]
pub mod ffi {
    use super::{PartyData, decimal, inv_core, party, vat, vat_code};
    use crate::common::ffi::{BindingError, Text};
    use crate::common::{boundary, core_error, local_error, optional_text, text, write};
    use crate::crypto::ffi::Config;
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};

    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct Address(pub(crate) inv_core::Address);
    impl Address {
        pub fn with_province(
            &self,
            value: &DiplomatStr,
        ) -> Result<Box<Address>, Box<BindingError>> {
            boundary(|| {
                let mut a = self.0.clone();
                a.province = Some(text(value)?.into());
                Ok(Box::new(Address(a)))
            })
        }
        pub fn province(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.province().map(|v| Box::new(Text(v.into())))))
        }

        #[allow(clippy::too_many_arguments)]
        pub fn new(
            country_code: &DiplomatStr,
            city: &DiplomatStr,
            street: &DiplomatStr,
            building_number: &DiplomatStr,
            postal_code: &DiplomatStr,
            additional_street: Option<&DiplomatStr>,
            additional_number: Option<&DiplomatStr>,
            district: Option<&DiplomatStr>,
        ) -> Result<Box<Address>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Address(inv_core::Address {
                    province: None,
                    country_code: inv_core::CountryCode::parse(text(country_code)?)
                        .map_err(core_error)?,
                    city: text(city)?.into(),
                    street: text(street)?.into(),
                    building_number: text(building_number)?.into(),
                    postal_code: text(postal_code)?.into(),
                    additional_street: optional_text(additional_street)?,
                    additional_number: optional_text(additional_number)?,
                    district: optional_text(district)?,
                })))
            })
        }
        pub fn city(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.city(), out))
        }
        pub fn street(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.street(), out))
        }
        pub fn building_number(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.building_number(), out))
        }
        pub fn postal_code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.postal_code(), out))
        }
        pub fn country_code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.country_code().as_str(), out))
        }
        pub fn additional_street(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok((self.0.additional_street()).map(|v| Box::new(Text(v.to_string())))))
        }
        pub fn additional_number(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok((self.0.additional_number()).map(|v| Box::new(Text(v.to_string())))))
        }
        pub fn district(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok((self.0.district()).map(|v| Box::new(Text(v.to_string())))))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct VatId(pub(crate) inv_core::VatId);
    impl VatId {
        pub fn value(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.as_str(), out))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct OtherId(pub(crate) inv_core::OtherId);
    impl OtherId {
        pub fn value(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.as_str(), out))
        }
        pub fn scheme(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok((self.0.scheme_id()).map(|v| Box::new(Text(v.to_string())))))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct InvoiceNote(pub(crate) inv_core::InvoiceNote);
    impl InvoiceNote {
        pub fn language_id(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.language_id().map(|s| Box::new(Text(s.into())))))
        }

        pub fn language(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.language(), out))
        }
        pub fn text(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.text(), out))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct OriginalInvoiceRef(pub(crate) inv_core::OriginalInvoiceRef);
    impl OriginalInvoiceRef {
        pub fn new(
            id: &DiplomatStr,
            uuid: Option<&DiplomatStr>,
            date: Option<&DiplomatStr>,
            time: Option<&DiplomatStr>,
            type_code: Option<&DiplomatStr>,
        ) -> Result<Box<OriginalInvoiceRef>, Box<BindingError>> {
            boundary(|| {
                let mut r = inv_core::OriginalInvoiceRef::new(text(id)?);
                if let Some(v) = optional_text(uuid)? {
                    r = r.with_uuid(v);
                }
                if let Some(v) = optional_text(date)? {
                    r = r.with_issue_date(inv_core::InvoiceDate::parse(v).map_err(core_error)?);
                }
                if let Some(v) = optional_text(time)? {
                    r = r.with_issue_time(inv_core::InvoiceTime::parse(v).map_err(core_error)?);
                }
                if let Some(v) = optional_text(type_code)? {
                    r = r.with_document_type_code(v);
                }
                Ok(Box::new(Self(r)))
            })
        }
        pub fn issue_time(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .issue_time()
                    .map(|v| Box::new(Text(v.as_str().into()))))
            })
        }
        pub fn document_type_code(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .document_type_code()
                    .map(|v| Box::new(Text(v.into()))))
            })
        }

        pub fn id(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.id(), out))
        }
        pub fn uuid(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok((self.0.uuid()).map(|v| Box::new(Text(v.to_string())))))
        }
        pub fn issue_date(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(
                    (self.0.issue_date().map(|v| v.as_str()))
                        .map(|v| Box::new(Text(v.to_string()))),
                )
            })
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct Party(pub(super) PartyData);
    impl Party {
        pub fn identifiers_len(&self) -> usize {
            self.0.identifiers.len()
        }
        pub fn identifier(&self, index: usize) -> Result<Box<OtherId>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .identifiers
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(OtherId(v)))
                    .ok_or_else(|| local_error(1, "identifier index out of range"))
            })
        }

        pub fn address(&self) -> Result<Option<Box<Address>>, Box<BindingError>> {
            boundary(|| Ok(self.0.address.clone().map(|a| Box::new(Address(a)))))
        }
        pub fn vat_id(&self) -> Result<Option<Box<VatId>>, Box<BindingError>> {
            boundary(|| Ok(self.0.vat_id.clone().map(|v| Box::new(VatId(v)))))
        }
        pub fn other_id(&self) -> Result<Option<Box<OtherId>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .identifiers
                    .first()
                    .cloned()
                    .map(|v| Box::new(OtherId(v))))
            })
        }
        pub fn name(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.name.as_str(), out))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct TaxExemption(pub(crate) inv_core::TaxExemption);
    impl TaxExemption {
        pub fn new(
            code: Option<&DiplomatStr>,
            reason: Option<&DiplomatStr>,
        ) -> Result<Box<TaxExemption>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(inv_core::TaxExemption {
                    reason_code: optional_text(code)?,
                    reason: optional_text(reason)?,
                })))
            })
        }
        pub fn reason_code(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.reason_code.clone().map(|s| Box::new(Text(s)))))
        }
        pub fn reason(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.reason.clone().map(|s| Box::new(Text(s)))))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct AllowanceCharge(pub(crate) inv_core::AllowanceCharge);
    impl AllowanceCharge {
        pub fn has_vat_rate(&self) -> bool {
            !self.0.omitted_rate
        }

        pub fn without_vat_rate(&self) -> Result<Box<AllowanceCharge>, Box<BindingError>> {
            boundary(|| {
                if self.0.category != inv_core::VatCategory::OutOfScope
                    || self.0.rate != fatoora_core::Decimal::ZERO
                {
                    return Err(local_error(
                        1,
                        "only out-of-scope adjustments can omit a zero VAT rate",
                    ));
                }
                let mut value = self.0.clone();
                value.omitted_rate = true;
                Ok(Box::new(Self(value)))
            })
        }

        /// Retain this adjustment's metadata and check its percentage calculation.
        pub fn with_percentage(
            &self,
            base: &DiplomatStr,
            percent: &DiplomatStr,
        ) -> Result<Box<AllowanceCharge>, Box<BindingError>> {
            boundary(|| {
                let base = decimal(base, "base")?;
                let percent = decimal(percent, "percent")?;
                inv_core::Adjustment::percentage(base, percent)
                    .supplied_amount(self.0.amount)
                    .value()
                    .map_err(core_error)?;
                let mut adjustment = self.0.clone();
                adjustment.base_amount = Some(base);
                adjustment.percentage = Some(percent);
                Ok(Box::new(Self(adjustment)))
            })
        }

        pub fn base_amount(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.base_amount.map(|v| Box::new(Text(v.to_string())))))
        }
        pub fn percentage(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.percentage.map(|v| Box::new(Text(v.to_string())))))
        }

        #[allow(clippy::too_many_arguments)]
        pub fn new(
            charge: bool,
            amount: &DiplomatStr,
            reason: Option<&DiplomatStr>,
            reason_code: Option<&DiplomatStr>,
            category: u8,
            rate: &DiplomatStr,
            exemption: &TaxExemption,
        ) -> Result<Box<AllowanceCharge>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(inv_core::AllowanceCharge {
                    omitted_rate: false,
                    base_amount: None,
                    percentage: None,
                    tax_category_count: 1,
                    charge,
                    amount: decimal(amount, "amount")?,
                    reason: optional_text(reason)?,
                    reason_code: optional_text(reason_code)?,
                    category: vat(category)?,
                    rate: decimal(rate, "rate")?,
                    exemption: exemption.0.clone(),
                })))
            })
        }
        pub fn is_charge(&self) -> bool {
            self.0.charge
        }
        pub fn amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.amount.to_string(), out))
        }
        pub fn rate(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.rate.to_string(), out))
        }
        pub fn category(&self) -> u8 {
            vat_code(self.0.category)
        }
        /// Return an independent adjustment with the given number of identical tax classifications.
        pub fn with_tax_category_count(
            &self,
            count: usize,
        ) -> Result<Box<AllowanceCharge>, Box<BindingError>> {
            boundary(|| {
                if !(1..=inv_core::AllowanceCharge::MAX_TAX_CATEGORY_COUNT).contains(&count) {
                    return Err(local_error(
                        1,
                        "tax category count must be between 1 and 1024",
                    ));
                }
                let mut adjustment = self.0.clone();
                adjustment.tax_category_count = count;
                Ok(Box::new(Self(adjustment)))
            })
        }

        pub fn tax_category_count(&self) -> usize {
            self.0.tax_category_count
        }
        pub fn reason(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.reason.clone().map(|s| Box::new(Text(s)))))
        }
        pub fn reason_code(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.reason_code.clone().map(|s| Box::new(Text(s)))))
        }
        pub fn exemption(&self) -> Result<Box<TaxExemption>, Box<BindingError>> {
            boundary(|| Ok(Box::new(TaxExemption(self.0.exemption.clone()))))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct VatBreakdown(pub(crate) inv_core::VatBreakdown);
    impl VatBreakdown {
        pub fn has_vat_rate(&self) -> bool {
            self.0.has_vat_rate()
        }
        pub fn without_vat_rate(&self) -> Result<Box<VatBreakdown>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    self.0.clone().without_vat_rate().map_err(core_error)?,
                )))
            })
        }
        pub fn new(
            category: u8,
            rate: &DiplomatStr,
            taxable_amount: &DiplomatStr,
            tax_amount: &DiplomatStr,
            exemption: &TaxExemption,
        ) -> Result<Box<VatBreakdown>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    inv_core::VatBreakdown::new(
                        vat(category)?,
                        decimal(rate, "rate")?,
                        decimal(taxable_amount, "taxable_amount")?,
                        decimal(tax_amount, "tax_amount")?,
                    )
                    .map_err(core_error)?
                    .with_exemption(exemption.0.clone()),
                )))
            })
        }

        pub fn category(&self) -> u8 {
            vat_code(self.0.category())
        }
        pub fn rate(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.rate().to_string(), out))
        }
        pub fn taxable_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.taxable_amount().to_string(), out))
        }
        pub fn tax_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.tax_amount().to_string(), out))
        }
        pub fn exemption(&self) -> Result<Box<TaxExemption>, Box<BindingError>> {
            boundary(|| Ok(Box::new(TaxExemption(self.0.exemption().clone()))))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct Adjustment(pub(crate) inv_core::Adjustment);
    impl Adjustment {
        pub fn amount(value: &DiplomatStr) -> Result<Box<Adjustment>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(inv_core::Adjustment::amount(decimal(
                    value, "amount",
                )?))))
            })
        }
        pub fn percentage(
            base: &DiplomatStr,
            percent: &DiplomatStr,
        ) -> Result<Box<Adjustment>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(inv_core::Adjustment::percentage(
                    decimal(base, "base")?,
                    decimal(percent, "percent")?,
                ))))
            })
        }
        pub fn with_supplied_amount(
            &self,
            value: &DiplomatStr,
        ) -> Result<Box<Adjustment>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    self.0.clone().supplied_amount(decimal(value, "amount")?),
                )))
            })
        }
        pub fn with_reason(
            &self,
            reason: Option<&DiplomatStr>,
            code: Option<&DiplomatStr>,
        ) -> Result<Box<Adjustment>, Box<BindingError>> {
            boundary(|| {
                let mut a = self.0.clone();
                if let Some(v) = reason {
                    a = a.reason(text(v)?);
                }
                if let Some(v) = code {
                    a = a.reason_code(text(v)?);
                }
                Ok(Box::new(Self(a)))
            })
        }
        pub fn value(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.value().map_err(core_error)?.to_string(), out))
        }
        pub fn base_amount(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .calculation_base_amount()
                    .map(|v| Box::new(Text(v.to_string()))))
            })
        }
        pub fn percent(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .calculation_percentage()
                    .map(|v| Box::new(Text(v.to_string()))))
            })
        }
        pub fn reason(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .allowance_charge_reason()
                    .map(|v| Box::new(Text(v.into()))))
            })
        }
        pub fn reason_code(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .allowance_charge_reason_code()
                    .map(|v| Box::new(Text(v.into()))))
            })
        }
    }
    #[diplomat::opaque_mut]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct LineItemBuilder(pub(crate) Option<inv_core::LineItemBuilder>);
    impl LineItemBuilder {
        pub fn set_tax_total_present(&mut self, present: bool) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.tax_total_present(present)))
        }
        pub fn set_gross_amount_present(&mut self, present: bool) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.gross_amount_present(present)))
        }
        pub fn new() -> Box<LineItemBuilder> {
            Box::new(Self(Some(inv_core::LineItem::builder())))
        }
        pub fn build(&mut self) -> Result<Box<InvoiceLineItem>, Box<BindingError>> {
            boundary(|| {
                let b = self
                    .0
                    .take()
                    .ok_or_else(|| local_error(1, "line builder consumed"))?;
                Ok(Box::new(InvoiceLineItem(b.build().map_err(core_error)?)))
            })
        }
        pub fn set_item_name(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.item_name(value))
            })
        }
        pub fn set_invoiced_quantity_unit_code(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.invoiced_quantity_unit_code(value))
            })
        }
        pub fn set_item_price_base_quantity_unit_code(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.item_price_base_quantity_unit_code(value))
            })
        }
        pub fn set_id(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.id(value))
            })
        }
        pub fn set_invoiced_quantity(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "invoiced_quantity")?;
                self.update(|b| b.invoiced_quantity(value))
            })
        }
        pub fn set_item_net_price(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "item_net_price")?;
                self.update(|b| b.item_net_price(value))
            })
        }
        pub fn set_item_gross_price(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "item_gross_price")?;
                self.update(|b| b.item_gross_price(value))
            })
        }
        pub fn set_item_price_discount(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "item_price_discount")?;
                self.update(|b| b.item_price_discount(value))
            })
        }
        pub fn set_item_price_base_quantity(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "item_price_base_quantity")?;
                self.update(|b| b.item_price_base_quantity(value))
            })
        }
        pub fn set_line_net_amount(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "line_net_amount")?;
                self.update(|b| b.line_net_amount(value))
            })
        }
        pub fn set_line_vat_amount(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "line_vat_amount")?;
                self.update(|b| b.line_vat_amount(value))
            })
        }
        pub fn set_line_amount_with_vat(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "line_amount_with_vat")?;
                self.update(|b| b.line_amount_with_vat(value))
            })
        }
        pub fn set_vat_rate(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "vat_rate")?;
                self.update(|b| b.vat_rate(value))
            })
        }
        pub fn set_vat_category(&mut self, value: u8) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = vat(value)?;
                self.update(|b| b.vat_category(value))
            })
        }
        pub fn set_exemption(&mut self, value: &TaxExemption) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.exemption(value.0.clone())))
        }
        pub fn add_allowance(&mut self, value: &Adjustment) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.allowance(value.0.clone())))
        }
        pub fn add_charge(&mut self, value: &Adjustment) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.charge(value.0.clone())))
        }
        pub fn add_document_reference(
            &mut self,
            value: &OriginalInvoiceRef,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.document_reference(value.0.clone())))
        }
        pub fn add_tax_subtotal(&mut self, value: &VatBreakdown) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.tax_subtotal(value.0.clone())))
        }
        pub fn set_item_seller_identifier(
            &mut self,
            value: &DiplomatStr,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = InvoiceBuilder::other(Some(value), scheme)?
                    .ok_or_else(|| local_error(1, "identifier required"))?;
                self.update(|b| b.item_seller_identifier(value))
            })
        }
        pub fn set_item_buyer_identifier(
            &mut self,
            value: &DiplomatStr,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = InvoiceBuilder::other(Some(value), scheme)?
                    .ok_or_else(|| local_error(1, "identifier required"))?;
                self.update(|b| b.item_buyer_identifier(value))
            })
        }
        pub fn set_item_standard_identifier(
            &mut self,
            value: &DiplomatStr,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = InvoiceBuilder::other(Some(value), scheme)?
                    .ok_or_else(|| local_error(1, "identifier required"))?;
                self.update(|b| b.item_standard_identifier(value))
            })
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct PaymentMeans(pub(crate) inv_core::PaymentMeans);
    impl PaymentMeans {
        pub fn with_instruction_note(
            &self,
            value: &DiplomatStr,
        ) -> Result<Box<PaymentMeans>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    self.0.clone().instruction_note(text(value)?),
                )))
            })
        }
        pub fn instruction_notes_len(&self) -> usize {
            self.0.instruction_notes.len()
        }
        pub fn instruction_note_at(&self, index: usize) -> Result<Box<Text>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .instruction_notes
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(Text(v)))
                    .ok_or_else(|| local_error(1, "instruction index out of range"))
            })
        }

        pub fn new(
            code: &DiplomatStr,
            instruction: Option<&DiplomatStr>,
        ) -> Result<Box<PaymentMeans>, Box<BindingError>> {
            boundary(|| {
                let mut p = inv_core::PaymentMeans::new(text(code)?);
                p.instruction_notes = optional_text(instruction)?.into_iter().collect();
                Ok(Box::new(Self(p)))
            })
        }
        pub fn with_account(
            &self,
            id: Option<&DiplomatStr>,
            terms: Option<&DiplomatStr>,
        ) -> Result<Box<PaymentMeans>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(self.0.clone().payee_financial_account(
                    inv_core::PaymentAccount {
                        payment_account_identifier: optional_text(id)?,
                        payment_terms: optional_text(terms)?,
                    },
                ))))
            })
        }
        pub fn code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(&self.0.payment_means_type_code, out))
        }
        pub fn instruction_note(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .instruction_notes
                    .first()
                    .map(|v| Box::new(Text(v.clone()))))
            })
        }
        pub fn has_account(&self) -> bool {
            self.0.payee_financial_account.is_some()
        }
        pub fn account_identifier(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .payee_financial_account
                    .as_ref()
                    .and_then(|v| v.payment_account_identifier.as_ref())
                    .map(|v| Box::new(Text(v.clone()))))
            })
        }
        pub fn payment_terms(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .payee_financial_account
                    .as_ref()
                    .and_then(|v| v.payment_terms.as_ref())
                    .map(|v| Box::new(Text(v.clone()))))
            })
        }
    }
    #[diplomat::opaque_mut]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct InvoiceBuilder(pub(crate) Option<inv_core::InvoiceBuilder>);
    impl InvoiceBuilder {
        pub fn add_seller_identifier(
            &mut self,
            value: &DiplomatStr,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = Self::other(Some(value), scheme)?.unwrap();
                self.update(|b| b.seller_identifier(value))
            })
        }
        pub fn add_buyer_identifier(
            &mut self,
            value: &DiplomatStr,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = Self::other(Some(value), scheme)?.unwrap();
                self.update(|b| b.buyer_identifier(value))
            })
        }

        pub fn add_vat_breakdown(&mut self, value: &VatBreakdown) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.vat_breakdown(value.0.clone())))
        }

        pub fn add_payment_means(&mut self, value: &PaymentMeans) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.payment_means(value.0.clone())))
        }
        pub fn add_note(
            &mut self,
            language: Option<&DiplomatStr>,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                let note = match language {
                    Some(v) => inv_core::InvoiceNote::new(text(v)?, value),
                    None => inv_core::InvoiceNote::without_language(value),
                };
                self.update(|b| b.note(note))
            })
        }
        pub fn add_document_adjustment(
            &mut self,
            charge: bool,
            value: &Adjustment,
            category: u8,
            rate: &DiplomatStr,
            exemption: &TaxExemption,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = inv_core::AllowanceCharge {
                    omitted_rate: false,
                    charge,
                    tax_category_count: 1,
                    amount: value.0.value().map_err(core_error)?,
                    base_amount: value.0.calculation_base_amount(),
                    percentage: value.0.calculation_percentage(),
                    reason: value.0.allowance_charge_reason().map(str::to_owned),
                    reason_code: value.0.allowance_charge_reason_code().map(str::to_owned),
                    category: vat(category)?,
                    rate: decimal(rate, "rate")?,
                    exemption: exemption.0.clone(),
                };
                self.update(|b| b.adjustment(value))
            })
        }
        pub fn set_buyer_without_address(
            &mut self,
            name: &DiplomatStr,
            vat_id: Option<&DiplomatStr>,
            other_id: Option<&DiplomatStr>,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let buyer = inv_core::Buyer::new(
                    text(name)?.into(),
                    None,
                    optional_text(vat_id)?,
                    Self::other(other_id, scheme)?,
                )
                .map_err(core_error)?;
                self.update(|b| b.buyer(buyer))
            })
        }

        /// Construct a credit or debit note with a complete billing reference.
        pub fn new_with_reference(
            kind: u8,
            subtype: u8,
            reference: &OriginalInvoiceRef,
            reason: &DiplomatStr,
        ) -> Result<Box<InvoiceBuilder>, Box<BindingError>> {
            boundary(|| {
                let sub = match subtype {
                    0 => inv_core::InvoiceSubType::Standard,
                    1 => inv_core::InvoiceSubType::Simplified,
                    _ => return Err(local_error(1, "invalid invoice subtype")),
                };
                let reason = text(reason)?.to_owned();
                let invoice_type = match kind {
                    2 => inv_core::InvoiceType::CreditNote(sub),
                    3 => inv_core::InvoiceType::DebitNote(sub),
                    _ => {
                        return Err(local_error(
                            1,
                            "billing references require a credit or debit note",
                        ));
                    }
                };
                Ok(Box::new(Self(Some(
                    inv_core::InvoiceBuilder::new(invoice_type)
                        .billing_reference(reference.0.clone())
                        .payment_instruction(reason),
                ))))
            })
        }

        pub fn set_note_without_language(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let note = inv_core::InvoiceNote::without_language(text(value)?);
                self.update(|b| b.note(note))
            })
        }

        pub fn set_payment_instruction(
            &mut self,
            instruction: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let instruction = text(instruction)?;
                self.update(|b| b.payment_instruction(instruction))
            })
        }
        pub fn set_prepaid_amount(
            &mut self,
            amount: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let amount = decimal(amount, "prepaid_amount")?;
                self.update(|b| b.prepaid_amount(amount))
            })
        }
        pub fn set_payable_rounding_amount(
            &mut self,
            amount: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let amount = decimal(amount, "payable_rounding_amount")?;
                self.update(|b| b.payable_rounding_amount(amount))
            })
        }

        pub fn set_delivery(
            &mut self,
            actual: Option<&DiplomatStr>,
            latest: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let actual_date = optional_text(actual)?
                    .map(inv_core::InvoiceDate::parse)
                    .transpose()
                    .map_err(core_error)?;
                let latest_date = optional_text(latest)?
                    .map(inv_core::InvoiceDate::parse)
                    .transpose()
                    .map_err(core_error)?;
                self.update(|b| {
                    b.delivery(inv_core::Delivery {
                        actual_date,
                        latest_date,
                    })
                })
            })
        }
        pub fn set_tax_currency(
            &mut self,
            currency: &DiplomatStr,
            amount: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let currency =
                    inv_core::CurrencyCode::parse(text(currency)?).map_err(core_error)?;
                let amount = decimal(amount, "amount")?;
                self.update(|b| b.tax_currency(currency, amount))
            })
        }
        pub fn set_tax_exemption(
            &mut self,
            category: u8,
            rate: &DiplomatStr,
            exemption: &TaxExemption,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let category = vat(category)?;
                let rate = decimal(rate, "rate")?;
                self.update(|b| b.tax_exemption(category, rate, exemption.0.clone()))
            })
        }
        pub fn add_adjustment(
            &mut self,
            adjustment: &AllowanceCharge,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.adjustment(adjustment.0.clone())))
        }
        pub fn add_item(&mut self, item: &InvoiceLineItem) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.line_item(item.0.clone())))
        }

        pub fn new(
            kind: u8,
            subtype: u8,
            original_id: Option<&DiplomatStr>,
            original_uuid: Option<&DiplomatStr>,
            original_date: Option<&DiplomatStr>,
            reason: Option<&DiplomatStr>,
        ) -> Result<Box<InvoiceBuilder>, Box<BindingError>> {
            boundary(|| {
                let sub = match subtype {
                    0 => inv_core::InvoiceSubType::Standard,
                    1 => inv_core::InvoiceSubType::Simplified,
                    _ => return Err(local_error(1, "invalid invoice subtype")),
                };
                let mut billing = None;
                let mut instruction = None;
                let kind = match kind {
                    0 => inv_core::InvoiceType::Tax(sub),
                    1 => inv_core::InvoiceType::Prepayment(sub),
                    2 | 3 => {
                        if let Some(id) = original_id {
                            let mut original = inv_core::OriginalInvoiceRef::new(text(id)?);
                            if let Some(v) = original_uuid {
                                original = original.with_uuid(text(v)?);
                            }
                            if let Some(v) = original_date {
                                original =
                                    original.with_issue_date_str(text(v)?).map_err(core_error)?;
                            }
                            billing = Some(original);
                        } else if original_uuid.is_some() || original_date.is_some() {
                            return Err(local_error(
                                1,
                                "reference metadata requires an original invoice id",
                            ));
                        }
                        instruction = optional_text(reason)?;
                        if kind == 2 {
                            inv_core::InvoiceType::CreditNote(sub)
                        } else {
                            inv_core::InvoiceType::DebitNote(sub)
                        }
                    }
                    _ => return Err(local_error(1, "invalid invoice type")),
                };
                let mut builder = inv_core::InvoiceBuilder::new(kind);
                if let Some(reference) = billing {
                    builder = builder.billing_reference(reference);
                }
                if let Some(reason) = instruction {
                    builder = builder.payment_instruction(reason);
                }
                Ok(Box::new(InvoiceBuilder(Some(builder))))
            })
        }
        pub fn add_billing_reference(
            &mut self,
            value: &OriginalInvoiceRef,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.billing_reference(value.0.clone())))
        }
        pub fn set_purchase_order_id(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.purchase_order_id(value))
            })
        }
        pub fn set_contract_id(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.contract_id(value))
            })
        }
        pub fn set_id(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.id(value))
            })
        }
        pub fn set_uuid(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.uuid(value))
            })
        }
        pub fn set_issue_datetime(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                inv_core::InvoiceTimestamp::parse(value).map_err(core_error)?;
                self.update(|b| b.issue_datetime(value))
            })
        }
        pub fn set_currency(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                inv_core::CurrencyCode::parse(value).map_err(core_error)?;
                self.update(|b| b.currency(value))
            })
        }
        pub fn set_previous_invoice_hash(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.previous_invoice_hash(value))
            })
        }
        pub fn set_payment_means_code(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.payment_means_code(value))
            })
        }
        pub fn allowance_reason(&mut self, value: &DiplomatStr) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = text(value)?;
                self.update(|b| b.allowance_reason(value))
            })
        }
        pub fn invoice_level_charge(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "invoice_level_charge")?;
                self.update(|b| b.invoice_level_charge(value))
            })
        }
        pub fn invoice_level_discount(
            &mut self,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = decimal(value, "invoice_level_discount")?;
                self.update(|b| b.invoice_level_discount(value))
            })
        }
        pub fn set_invoice_counter(&mut self, value: u64) -> Result<(), Box<BindingError>> {
            boundary(|| self.update(|b| b.invoice_counter(value)))
        }
        pub fn set_vat_category(&mut self, value: u8) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let v = vat(value)?;
                self.update(|b| b.vat_category(v))
            })
        }
        pub fn flags(&mut self, value: u8) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let v = inv_core::InvoiceFlags::from_bits(value)
                    .ok_or_else(|| local_error(1, "invalid invoice flags"))?;
                self.update(|b| b.flags(v))
            })
        }
        pub fn set_note(
            &mut self,
            language: &DiplomatStr,
            value: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let note = inv_core::InvoiceNote::new(text(language)?, text(value)?);
                self.update(|b| b.note(note))
            })
        }
        pub fn set_allowance(
            &mut self,
            reason: &DiplomatStr,
            amount: &DiplomatStr,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let reason = text(reason)?;
                let amount = decimal(amount, "allowance")?;
                self.update(|b| b.invoice_level_discount(amount).allowance_reason(reason))
            })
        }
        pub fn set_seller(
            &mut self,
            name: &DiplomatStr,
            address: &Address,
            vat_id: &DiplomatStr,
            other_id: Option<&DiplomatStr>,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let other = Self::other(other_id, scheme)?;
                let seller = inv_core::Party::<inv_core::SellerRole>::new(
                    text(name)?.into(),
                    address.0.clone(),
                    text(vat_id)?,
                    other,
                )
                .map_err(core_error)?;
                self.update(|b| b.seller(seller))
            })
        }
        pub fn set_buyer(
            &mut self,
            name: &DiplomatStr,
            address: &Address,
            vat_id: Option<&DiplomatStr>,
            other_id: Option<&DiplomatStr>,
            scheme: Option<&DiplomatStr>,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let other = Self::other(other_id, scheme)?;
                let buyer = inv_core::Party::<inv_core::BuyerRole>::new(
                    text(name)?.into(),
                    address.0.clone(),
                    optional_text(vat_id)?,
                    other,
                )
                .map_err(core_error)?;
                self.update(|b| b.buyer(buyer))
            })
        }
        pub fn add_line_item(
            &mut self,
            description: &DiplomatStr,
            quantity: &DiplomatStr,
            unit_code: &DiplomatStr,
            unit_price: &DiplomatStr,
            vat_rate: &DiplomatStr,
            category: u8,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let line = inv_core::LineItem::new(
                    text(description)?,
                    decimal(quantity, "quantity")?,
                    text(unit_code)?,
                    decimal(unit_price, "unit_price")?,
                    decimal(vat_rate, "vat_rate")?,
                    vat(category)?,
                )
                .map_err(core_error)?;
                self.update(|b| b.line_item(line))
            })
        }
        pub fn build(&mut self) -> Result<Box<FinalizedInvoice>, Box<BindingError>> {
            boundary(|| {
                let b = self
                    .0
                    .take()
                    .ok_or_else(|| local_error(1, "builder has been consumed"))?;
                Ok(Box::new(FinalizedInvoice(Some(
                    b.build().map_err(core_error)?,
                ))))
            })
        }
    }
    #[diplomat::opaque_mut]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct FinalizedInvoice(pub(crate) Option<inv_core::FinalizedInvoice>);
    impl FinalizedInvoice {
        pub fn from_xml(value: &DiplomatStr) -> Result<Box<FinalizedInvoice>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(FinalizedInvoice(Some(
                    inv_core::xml::parse::parse_finalized_invoice_xml(text(value)?)
                        .map_err(core_error)?,
                ))))
            })
        }
        #[diplomat::attr(js, disable)]
        pub fn from_file(value: &DiplomatStr) -> Result<Box<FinalizedInvoice>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(FinalizedInvoice(Some(
                    inv_core::xml::parse::parse_finalized_invoice_xml_file(text(value)?)
                        .map_err(core_error)?,
                ))))
            })
        }
        pub fn data(&self) -> Result<Box<InvoiceData>, Box<BindingError>> {
            boundary(|| Ok(Box::new(InvoiceData(self.get()?.data().clone()))))
        }
        pub fn totals(&self) -> Result<Box<InvoiceTotals>, Box<BindingError>> {
            boundary(|| Ok(Box::new(InvoiceTotals(self.get()?.totals().clone()))))
        }
        pub fn hash_base64(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.hash_base64().map_err(core_error)?, out))
        }
        pub fn xml(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.to_xml().map_err(core_error)?, out))
        }
    }
    #[diplomat::opaque_mut]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct SignedInvoice(pub(crate) Option<inv_core::SignedInvoice>);
    impl SignedInvoice {
        pub fn from_xml(value: &DiplomatStr) -> Result<Box<SignedInvoice>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(SignedInvoice(Some(
                    inv_core::xml::parse::parse_signed_invoice_xml(text(value)?)
                        .map_err(core_error)?,
                ))))
            })
        }
        #[diplomat::attr(js, disable)]
        pub fn from_file(value: &DiplomatStr) -> Result<Box<SignedInvoice>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(SignedInvoice(Some(
                    inv_core::xml::parse::parse_signed_invoice_xml_file(text(value)?)
                        .map_err(core_error)?,
                ))))
            })
        }
        pub fn data(&self) -> Result<Box<InvoiceData>, Box<BindingError>> {
            boundary(|| Ok(Box::new(InvoiceData(self.get()?.data().clone()))))
        }
        pub fn totals(&self) -> Result<Box<InvoiceTotals>, Box<BindingError>> {
            boundary(|| Ok(Box::new(InvoiceTotals(self.get()?.totals().clone()))))
        }
        pub fn hash_base64(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.hash_base64().map_err(core_error)?, out))
        }
        pub fn xml(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.xml(), out))
        }
        pub fn qr_code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.qr_code(), out))
        }
        pub fn signature(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.signature(), out))
        }
        pub fn public_key(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.public_key(), out))
        }
        pub fn invoice_hash(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.invoice_hash(), out))
        }
        pub fn to_xml_base64(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.to_xml_base64(), out))
        }
        pub fn issuer(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.signed_properties().issuer(), out))
        }
        pub fn serial(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.signed_properties().serial(), out))
        }
        pub fn cert_hash(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.signed_properties().cert_hash(), out))
        }
        pub fn signed_props_hash(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.signed_properties().signed_props_hash(), out))
        }
        pub fn signing_time(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.get()?.signed_properties().signing_time(), out))
        }
        pub fn zatca_key_signature(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok((self.get()?.zatca_key_signature()).map(|v| Box::new(Text(v.to_string()))))
            })
        }
        pub fn into_xml(&mut self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let invoice = self
                    .0
                    .take()
                    .ok_or_else(|| local_error(1, "invoice has been consumed"))?;
                write(invoice.into_xml(), out)
            })
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct InvoiceData(pub(crate) inv_core::InvoiceData);
    impl InvoiceData {
        pub fn issue_time(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.issue_time(), out))
        }
        pub fn issue_date(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.issue_date(), out))
        }

        pub fn payment_means_len(&self) -> usize {
            self.0.payment_means().len()
        }
        pub fn payment_means(&self, index: usize) -> Result<Box<PaymentMeans>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .payment_means()
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(PaymentMeans(v)))
                    .ok_or_else(|| local_error(1, "payment index out of range"))
            })
        }

        pub fn purchase_order_id(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.purchase_order_id().map(|v| Box::new(Text(v.into())))))
        }

        pub fn contract_id(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.contract_id().map(|v| Box::new(Text(v.into())))))
        }

        pub fn payment_instruction(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .payment_instruction()
                    .map(|s| Box::new(Text(s.into()))))
            })
        }

        pub fn actual_delivery_date(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .delivery()
                    .and_then(|d| d.actual_date.as_ref())
                    .map(|d| Box::new(Text(d.as_str().into()))))
            })
        }
        pub fn latest_delivery_date(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .delivery()
                    .and_then(|d| d.latest_date.as_ref())
                    .map(|d| Box::new(Text(d.as_str().into()))))
            })
        }
        pub fn tax_currency(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .tax_currency()
                    .map(|d| Box::new(Text(d.as_str().into()))))
            })
        }
        pub fn tax_currency_amount(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .tax_currency_amount()
                    .map(|d| Box::new(Text(d.to_string()))))
            })
        }
        pub fn adjustments_len(&self) -> usize {
            self.0.adjustments().len()
        }
        pub fn adjustment(&self, index: usize) -> Result<Box<AllowanceCharge>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .adjustments()
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(AllowanceCharge(v)))
                    .ok_or_else(|| local_error(1, "adjustment index out of range"))
            })
        }

        pub fn id(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.id(), out))
        }
        pub fn uuid(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.uuid(), out))
        }
        pub fn previous_invoice_hash(
            &self,
            out: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.previous_invoice_hash(), out))
        }
        pub fn payment_means_code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.payment_means_code(), out))
        }
        pub fn currency(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.currency().as_str(), out))
        }
        pub fn issue_datetime(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.issue_datetime().as_str(), out))
        }
        pub fn invoice_level_charge(
            &self,
            out: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.invoice_level_charge().to_string(), out))
        }
        pub fn invoice_level_discount(
            &self,
            out: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.invoice_level_discount().to_string(), out))
        }
        pub fn allowance_reason(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok((self.0.allowance_reason()).map(|v| Box::new(Text(v.to_string())))))
        }
        pub fn invoice_counter(&self) -> u64 {
            self.0.invoice_counter()
        }
        pub fn vat_category(&self) -> u8 {
            vat_code(self.0.vat_category())
        }
        pub fn flags_raw(&self) -> u8 {
            self.0.flags().bits()
        }
        pub fn invoice_type_kind(&self) -> u8 {
            match self.0.invoice_type() {
                inv_core::InvoiceType::Tax(_) => 0,
                inv_core::InvoiceType::Prepayment(_) => 1,
                inv_core::InvoiceType::CreditNote(..) => 2,
                inv_core::InvoiceType::DebitNote(..) => 3,
            }
        }
        pub fn invoice_sub_type(&self) -> u8 {
            if self.0.invoice_type().is_simplified() {
                1
            } else {
                0
            }
        }
        pub fn seller(&self) -> Result<Box<Party>, Box<BindingError>> {
            boundary(|| Ok(Box::new(party(self.0.seller()))))
        }
        pub fn buyer(&self) -> Result<Option<Box<Party>>, Box<BindingError>> {
            boundary(|| Ok(self.0.buyer().map(|p| Box::new(party(p)))))
        }
        pub fn note(&self) -> Result<Option<Box<InvoiceNote>>, Box<BindingError>> {
            boundary(|| Ok(self.0.note().cloned().map(|v| Box::new(InvoiceNote(v)))))
        }
        pub fn line_items_len(&self) -> usize {
            self.0.line_items().len()
        }
        pub fn line_item(&self, index: usize) -> Result<Box<InvoiceLineItem>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .line_items()
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(InvoiceLineItem(v)))
                    .ok_or_else(|| local_error(1, "line item index out of bounds"))
            })
        }
        pub fn original_invoice_ref(
            &self,
        ) -> Result<Option<Box<OriginalInvoiceRef>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .billing_references()
                    .first()
                    .cloned()
                    .map(|v| Box::new(OriginalInvoiceRef(v))))
            })
        }
        pub fn original_invoice_reason(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .payment_instruction()
                    .map(|v| Box::new(Text(v.into()))))
            })
        }
        pub fn notes_len(&self) -> usize {
            self.0.notes().len()
        }
        pub fn note_at(&self, index: usize) -> Result<Box<InvoiceNote>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .notes()
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(InvoiceNote(v)))
                    .ok_or_else(|| local_error(1, "note index out of range"))
            })
        }
        pub fn billing_references_len(&self) -> usize {
            self.0.billing_references().len()
        }
        pub fn billing_reference(
            &self,
            index: usize,
        ) -> Result<Box<OriginalInvoiceRef>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .billing_references()
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(OriginalInvoiceRef(v)))
                    .ok_or_else(|| local_error(1, "reference index out of range"))
            })
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct InvoiceLineItem(pub(crate) inv_core::LineItem);
    impl InvoiceLineItem {
        pub fn item_gross_price(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_gross_price()
                    .map(|v| Box::new(Text(v.to_string()))))
            })
        }
        pub fn item_price_discount(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_price_discount()
                    .map(|v| Box::new(Text(v.to_string()))))
            })
        }
        pub fn item_price_base_quantity(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_price_base_quantity()
                    .map(|v| Box::new(Text(v.to_string()))))
            })
        }
        pub fn item_price_base_quantity_unit_code(
            &self,
        ) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_price_base_quantity_unit_code()
                    .map(|v| Box::new(Text(v.into()))))
            })
        }
        pub fn adjustments_len(&self) -> usize {
            self.0.adjustments().len()
        }
        pub fn adjustment(&self, index: usize) -> Result<Box<Adjustment>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .adjustments()
                    .get(index)
                    .map(|v| Box::new(Adjustment(v.adjustment.clone())))
                    .ok_or_else(|| local_error(1, "adjustment index out of range"))
            })
        }
        pub fn adjustment_is_charge(&self, index: usize) -> Result<bool, Box<BindingError>> {
            boundary(|| {
                self.0
                    .adjustments()
                    .get(index)
                    .map(|v| v.charge)
                    .ok_or_else(|| local_error(1, "adjustment index out of range"))
            })
        }
        pub fn item_seller_identifier(&self) -> Result<Option<Box<OtherId>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_seller_identifier()
                    .cloned()
                    .map(|v| Box::new(OtherId(v))))
            })
        }
        pub fn item_buyer_identifier(&self) -> Result<Option<Box<OtherId>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_buyer_identifier()
                    .cloned()
                    .map(|v| Box::new(OtherId(v))))
            })
        }
        pub fn item_standard_identifier(&self) -> Result<Option<Box<OtherId>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .item_standard_identifier()
                    .cloned()
                    .map(|v| Box::new(OtherId(v))))
            })
        }

        pub fn with_document_reference(
            &self,
            reference: &OriginalInvoiceRef,
        ) -> Result<Box<InvoiceLineItem>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    self.0.clone().document_reference(reference.0.clone()),
                )))
            })
        }
        pub fn with_tax_subtotal(
            &self,
            subtotal: &VatBreakdown,
        ) -> Result<Box<InvoiceLineItem>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    self.0.clone().tax_subtotal(subtotal.0.clone()),
                )))
            })
        }
        pub fn document_references_len(&self) -> usize {
            self.0.document_references().len()
        }
        pub fn document_reference(
            &self,
            index: usize,
        ) -> Result<Box<OriginalInvoiceRef>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .document_references()
                    .get(index)
                    .cloned()
                    .map(|r| Box::new(OriginalInvoiceRef(r)))
                    .ok_or_else(|| local_error(1, "reference index out of range"))
            })
        }
        pub fn tax_subtotals_len(&self) -> usize {
            self.0.tax_subtotals().len()
        }
        pub fn tax_subtotal(&self, index: usize) -> Result<Box<VatBreakdown>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .tax_subtotals()
                    .get(index)
                    .cloned()
                    .map(|r| Box::new(VatBreakdown(r)))
                    .ok_or_else(|| local_error(1, "subtotal index out of range"))
            })
        }

        pub fn new(
            description: &DiplomatStr,
            quantity: &DiplomatStr,
            unit_code: &DiplomatStr,
            unit_price: &DiplomatStr,
            rate: &DiplomatStr,
            category: u8,
        ) -> Result<Box<InvoiceLineItem>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(
                    inv_core::LineItem::new(
                        text(description)?,
                        decimal(quantity, "quantity")?,
                        text(unit_code)?,
                        decimal(unit_price, "unit_price")?,
                        decimal(rate, "rate")?,
                        vat(category)?,
                    )
                    .map_err(core_error)?,
                )))
            })
        }
        pub fn with_metadata(
            &self,
            id: Option<&DiplomatStr>,
            exemption: &TaxExemption,
        ) -> Result<Box<InvoiceLineItem>, Box<BindingError>> {
            boundary(|| {
                let mut item = self.0.clone().with_exemption(exemption.0.clone());
                if let Some(id) = optional_text(id)? {
                    item = item.with_id(id);
                }
                Ok(Box::new(Self(item)))
            })
        }
        pub fn id(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| Ok(self.0.id().map(|s| Box::new(Text(s.into())))))
        }
        pub fn exemption(&self) -> Result<Box<TaxExemption>, Box<BindingError>> {
            boundary(|| Ok(Box::new(TaxExemption(self.0.exemption().clone()))))
        }

        pub fn description(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.description(), out))
        }
        pub fn unit_code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.unit_code(), out))
        }
        pub fn quantity(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.quantity().to_string(), out))
        }
        pub fn unit_price(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.unit_price().to_string(), out))
        }
        pub fn total_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.total_amount().to_string(), out))
        }
        pub fn vat_rate(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.vat_rate().to_string(), out))
        }
        pub fn vat_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.vat_amount().to_string(), out))
        }
        pub fn has_vat_rate(&self) -> bool {
            self.0.has_vat_rate()
        }
        pub fn has_tax_total(&self) -> bool {
            self.0.has_tax_total()
        }
        pub fn has_gross_amount(&self) -> bool {
            self.0.has_gross_amount()
        }
        pub fn vat_category(&self) -> u8 {
            vat_code(self.0.vat_category())
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct InvoiceTotals(pub(crate) inv_core::InvoiceTotalsData);
    impl InvoiceTotals {
        pub fn vat_breakdown_len(&self) -> usize {
            self.0.vat_breakdown().len()
        }
        pub fn vat_breakdown(&self, index: usize) -> Result<Box<VatBreakdown>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .vat_breakdown()
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(VatBreakdown(v)))
                    .ok_or_else(|| local_error(1, "VAT breakdown index out of range"))
            })
        }

        pub fn tax_inclusive(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.tax_inclusive_amount().to_string(), out))
        }
        pub fn tax_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.tax_amount().to_string(), out))
        }
        pub fn line_extension(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.line_extension().to_string(), out))
        }
        pub fn allowance_total(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.allowance_total().to_string(), out))
        }
        pub fn charge_total(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.charge_total().to_string(), out))
        }
        pub fn taxable_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.taxable_amount().to_string(), out))
        }
        pub fn prepaid_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.prepaid_amount().to_string(), out))
        }
        pub fn payable_rounding_amount(
            &self,
            out: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.payable_rounding_amount().to_string(), out))
        }
        pub fn payable_amount(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.payable_amount().to_string(), out))
        }
    }
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct Xml;
    impl Xml {
        /// Produce an owned local ZATCA report as JSON. Inspect is_valid before acceptance.
        /// Options default when absent; JSON inputs are limited to 4 KiB.
        pub fn validate_zatca(
            config: &Config,
            xml: &DiplomatStr,
            options_json: Option<&DiplomatStr>,
            out: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let options = match options_json {
                    None => inv_core::validation::ZatcaValidationOptions::default(),
                    Some(value) => {
                        if value.len() > 4096 {
                            return Err(local_error(1, "options_json exceeds 4 KiB"));
                        }
                        serde_json::from_str(text(value)?).map_err(|error| {
                            local_error(1, &format!("Invalid validation options: {error}"))
                        })?
                    }
                };
                let report = inv_core::validation::validate_zatca_invoice_from_str(
                    text(xml)?,
                    &config.0,
                    &options,
                )
                .map_err(core_error)?;
                let json = serde_json::to_string(&report).map_err(|error| {
                    local_error(
                        9,
                        &format!("Could not serialize validation report: {error}"),
                    )
                })?;
                write(json, out)
            })
        }

        pub fn validate(config: &Config, xml: &DiplomatStr) -> Result<bool, Box<BindingError>> {
            boundary(|| {
                inv_core::validation::validate_xml_invoice_from_str(text(xml)?, &config.0)
                    .map(|()| true)
                    .map_err(core_error)
            })
        }
        pub fn hash(xml: &DiplomatStr, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| {
                write(
                    &inv_core::sign::invoice_hash_base64_from_xml_str(text(xml)?)
                        .map_err(core_error)?,
                    out,
                )
            })
        }
    }
}
impl ffi::InvoiceBuilder {
    fn update(
        &mut self,
        f: impl FnOnce(inv_core::InvoiceBuilder) -> inv_core::InvoiceBuilder,
    ) -> Result<(), Box<BindingError>> {
        let b = self
            .0
            .take()
            .ok_or_else(|| local_error(1, "builder has been consumed"))?;
        self.0 = Some(f(b));
        Ok(())
    }
    fn other(
        value: Option<&[u8]>,
        scheme: Option<&[u8]>,
    ) -> Result<Option<inv_core::OtherId>, Box<BindingError>> {
        match value {
            None => Ok(None),
            Some(v) => Ok(Some(match scheme {
                None => inv_core::OtherId::new(text(v)?),
                Some(s) => inv_core::OtherId::with_scheme(text(v)?, text(s)?),
            })),
        }
    }
}
impl ffi::FinalizedInvoice {
    pub(crate) fn get(&self) -> Result<&inv_core::FinalizedInvoice, Box<BindingError>> {
        self.0
            .as_ref()
            .ok_or_else(|| local_error(1, "invoice has been consumed"))
    }
}
impl ffi::SignedInvoice {
    pub(crate) fn get(&self) -> Result<&inv_core::SignedInvoice, Box<BindingError>> {
        self.0
            .as_ref()
            .ok_or_else(|| local_error(1, "invoice has been consumed"))
    }
}

impl ffi::LineItemBuilder {
    fn update(
        &mut self,
        f: impl FnOnce(inv_core::LineItemBuilder) -> inv_core::LineItemBuilder,
    ) -> Result<(), Box<BindingError>> {
        let b = self
            .0
            .take()
            .ok_or_else(|| local_error(1, "line builder consumed"))?;
        self.0 = Some(f(b));
        Ok(())
    }
}
