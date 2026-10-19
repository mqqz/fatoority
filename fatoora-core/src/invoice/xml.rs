//! XML serialization for invoices.
use super::{
    Address, Buyer, FinalizedInvoice, InvoiceData, InvoiceNote, InvoiceType, InvoiceView, LineItem,
    OtherId, Party, PartyRole, Seller, VatCategory, VatId,
};
use crate::Decimal;

use helpers::{
    currency_amount, id_with_scheme, id_with_scheme_with_agency, quantity_with_unit,
    vat_category_code,
};
use quick_xml::se::Serializer as QuickXmlSerializer;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use thiserror::Error;

/// Wrapper for serializing invoices to XML.
#[derive(Debug, Clone, Copy)]
struct InvoiceXml<'a, T: InvoiceView + ?Sized>(&'a T);

/// XML serialization error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InvoiceXmlError {
    #[error("failed to serialize invoice to XML: {source}")]
    Serialize {
        #[source]
        source: crate::Diagnostic,
    },
}

impl InvoiceXmlError {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        crate::ErrorKind::Xml
    }
}

/// XML formatting options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum XmlFormat {
    #[default]
    Compact,
    Pretty {
        indent_char: char,
        indent_size: usize,
    },
}

mod helpers {
    use super::VatCategory;
    use crate::Decimal;
    use serde::ser::{Serialize, SerializeStruct, Serializer};
    use std::fmt::{self, Display, Formatter};

    pub(super) fn vat_category_code(category: &VatCategory) -> &'static str {
        match category {
            VatCategory::Exempt => "E",
            VatCategory::Standard => "S",
            VatCategory::Zero => "Z",
            VatCategory::OutOfScope => "O",
        }
    }

    pub(super) struct FixedPrecision {
        value: Decimal,
        precision: usize,
    }

    impl FixedPrecision {
        pub(super) fn new(value: Decimal, precision: usize) -> Self {
            Self { value, precision }
        }
    }

    impl Display for FixedPrecision {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(&self.value.fixed(self.precision))
        }
    }

    impl Serialize for FixedPrecision {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_str(self)
        }
    }

    struct CurrencyAmountSer<'a> {
        tag: &'static str,
        currency: &'a str,
        value: Decimal,
        precision: usize,
    }

    pub(super) fn currency_amount<'a>(
        tag: &'static str,
        currency: &'a str,
        value: Decimal,
    ) -> impl Serialize + 'a {
        CurrencyAmountSer {
            tag,
            currency,
            value,
            precision: 2,
        }
    }

    impl<'a> Serialize for CurrencyAmountSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct(self.tag, 2)?;
            st.serialize_field("@currencyID", self.currency)?;
            st.serialize_field("$text", &FixedPrecision::new(self.value, self.precision))?;
            st.end()
        }
    }

    struct IdWithSchemeSer<'a> {
        tag: &'static str,
        scheme_id: &'a str,
        scheme_agency_id: Option<&'a str>,
        value: &'a str,
    }

    pub(super) fn id_with_scheme<'a>(
        tag: &'static str,
        scheme_id: &'a str,
        value: &'a str,
    ) -> impl Serialize + 'a {
        IdWithSchemeSer {
            tag,
            scheme_id,
            scheme_agency_id: None,
            value,
        }
    }

    pub(super) fn id_with_scheme_with_agency<'a>(
        tag: &'static str,
        scheme_id: &'a str,
        scheme_agency_id: &'a str,
        value: &'a str,
    ) -> impl Serialize + 'a {
        IdWithSchemeSer {
            tag,
            scheme_id,
            scheme_agency_id: Some(scheme_agency_id),
            value,
        }
    }

    impl<'a> Serialize for IdWithSchemeSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct(self.tag, 3)?;
            st.serialize_field("@schemeID", self.scheme_id)?;
            if let Some(agency) = self.scheme_agency_id {
                st.serialize_field("@schemeAgencyID", agency)?;
            }
            st.serialize_field("$text", self.value)?;
            st.end()
        }
    }

    struct QuantityWithUnitSer<'a> {
        tag: &'static str,
        value: Decimal,
        unit_code: &'a str,
    }

    pub(super) fn quantity_with_unit<'a>(
        tag: &'static str,
        value: Decimal,
        unit_code: &'a str,
    ) -> impl Serialize + 'a {
        QuantityWithUnitSer {
            tag,
            value,
            unit_code,
        }
    }

    impl<'a> Serialize for QuantityWithUnitSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct(self.tag, 2)?;
            if !self.unit_code.is_empty() {
                st.serialize_field("@unitCode", self.unit_code)?;
            }
            // The pinned SDK BR-22 distinguishes the lexical "0" from the
            // decimal zero used by prepayment adjustment lines.
            if self.value == Decimal::ZERO {
                st.serialize_field("$text", "0.00")?;
            } else {
                st.serialize_field("$text", &self.value)?;
            }
            st.end()
        }
    }
}

pub(crate) mod canonicalize;
pub(crate) mod constants;
pub(crate) mod edit;
pub(crate) mod fragments;
pub(crate) mod lexical;
pub(crate) mod nav;
pub mod parse;

struct InvoiceTotals<'a> {
    currency: &'a str,
    vat_percent: Decimal,
    vat_category: &'a VatCategory,
    totals: &'a super::InvoiceTotalsData,
}

impl<'a> InvoiceTotals<'a> {
    fn new<T: InvoiceView + ?Sized>(inv: &'a T) -> Self {
        let data = inv.data();
        let vat_percent = data.adjustment_vat_rate.unwrap_or_else(|| {
            data.line_items
                .iter()
                .find(|li| li.vat_category == data.vat_category)
                .map(|li| li.vat_rate)
                .unwrap_or_default()
        });

        Self {
            currency: data.currency.as_str(),
            vat_percent,
            vat_category: &data.vat_category,
            totals: inv.totals(),
        }
    }

    fn currency(&self) -> &'a str {
        self.currency
    }

    fn tax_amount(&self) -> Decimal {
        self.totals.tax_amount()
    }

    fn allowance_total(&self) -> Decimal {
        self.totals.allowance_total()
    }

    fn charge_total(&self) -> Decimal {
        self.totals.charge_total()
    }

    fn vat_percent(&self) -> Decimal {
        self.vat_percent
    }

    fn vat_category(&self) -> &'a VatCategory {
        self.vat_category
    }
}

struct InvoiceTypeView<'a>(&'a InvoiceType, super::InvoiceFlags);

impl<'a> Serialize for InvoiceTypeView<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let body_code = match self.0 {
            InvoiceType::Tax(_) => "388",
            InvoiceType::Prepayment(_) => "386",
            InvoiceType::CreditNote(..) => "381",
            InvoiceType::DebitNote(..) => "383",
        };
        let mut attribute_code = if self.0.is_simplified() { "02" } else { "01" }.to_owned();
        for flag in [
            super::InvoiceFlags::THIRD_PARTY,
            super::InvoiceFlags::NOMINAL,
            super::InvoiceFlags::EXPORT,
            super::InvoiceFlags::SUMMARY,
            super::InvoiceFlags::SELF_BILLED,
        ] {
            attribute_code.push(if self.1.contains(flag) { '1' } else { '0' });
        }

        let mut st = s.serialize_struct("cbc:InvoiceTypeCode", 2)?;
        st.serialize_field("@name", &attribute_code)?;
        st.serialize_field("$text", &body_code)?;
        st.end()
    }
}

struct TaxSchemeXml;

impl Serialize for TaxSchemeXml {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:TaxScheme", 0)?;
        st.serialize_field(
            "cbc:ID",
            &id_with_scheme_with_agency("cbc:ID", "UN/ECE 5153", "6", "VAT"),
        )?;
        st.end()
    }
}

struct VatSchemeXml<'a>(&'a VatId);

impl<'a> Serialize for VatSchemeXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vat = self.0;
        let mut st = s.serialize_struct("cac:PartyTaxScheme", 0)?;
        st.serialize_field("cbc:CompanyID", vat.as_str())?;
        st.serialize_field("cac:TaxScheme", &TaxSchemeXml)?;
        st.end()
    }
}

struct PartyXml<'a, R: PartyRole>(&'a Party<R>);

struct PartyIdentificationXml<'a>(&'a OtherId);

impl<'a> Serialize for PartyIdentificationXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let other_id = self.0;
        let mut st = s.serialize_struct("cac:PartyIdentification", 0)?;
        if let Some(scheme_id) = other_id.scheme_id() {
            st.serialize_field(
                "cbc:ID",
                &id_with_scheme("cbc:ID", scheme_id, other_id.as_str()),
            )?;
        } else {
            st.serialize_field("cbc:ID", other_id.as_str())?;
        }
        st.end()
    }
}

struct PartyLegalEntityXml<'a>(&'a str);

impl<'a> Serialize for PartyLegalEntityXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:PartyLegalEntity", 0)?;
        st.serialize_field("cbc:RegistrationName", self.0)?;
        st.end()
    }
}

struct AccountingSupplierPartyXml<'a>(&'a Seller);

impl<'a> Serialize for AccountingSupplierPartyXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:AccountingSupplierParty", 0)?;
        st.serialize_field("cac:Party", &PartyXml(self.0))?;
        st.end()
    }
}

struct AccountingCustomerPartyXml<'a>(Option<&'a Buyer>);

impl<'a> Serialize for AccountingCustomerPartyXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:AccountingCustomerParty", 0)?;
        if let Some(party) = self.0 {
            st.serialize_field("cac:Party", &PartyXml(party))?;
        } else {
            st.serialize_field("cac:Party", &EmptyParty)?;
        }
        st.end()
    }
}

struct EmptyParty;

impl Serialize for EmptyParty {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let st = s.serialize_struct("cac:Party", 0)?;
        st.end()
    }
}

#[derive(serde::Serialize)]
struct DeliveryXml<'a> {
    #[serde(
        rename = "cbc:ActualDeliveryDate",
        skip_serializing_if = "Option::is_none"
    )]
    actual: Option<&'a str>,
    #[serde(
        rename = "cbc:LatestDeliveryDate",
        skip_serializing_if = "Option::is_none"
    )]
    latest: Option<&'a str>,
}

struct NoteXml<'a>(&'a InvoiceNote);

impl<'a> Serialize for NoteXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let note = self.0;
        let mut st = s.serialize_struct("cbc:Note", 2)?;
        if let Some(language) = note.language_id() {
            st.serialize_field("@languageID", language)?;
        }
        st.serialize_field("$text", &note.text)?;
        st.end()
    }
}

struct EmbeddedDocumentXml<'a> {
    mime_code: &'a str,
    data: &'a str,
}

impl<'a> Serialize for EmbeddedDocumentXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cbc:EmbeddedDocumentBinaryObject", 2)?;
        st.serialize_field("@mimeCode", self.mime_code)?;
        st.serialize_field("$text", self.data)?;
        st.end()
    }
}

enum AdditionalDocumentReferenceXml<'a> {
    InvoiceCounter(&'a str),
    PreviousInvoiceHash(&'a str),
    QrCode(&'a str),
}

struct BillingReferenceXml<'a>(&'a super::OriginalInvoiceRef);

struct InvoiceDocumentReferenceXml<'a>(&'a super::OriginalInvoiceRef);

impl<'a> Serialize for InvoiceDocumentReferenceXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:InvoiceDocumentReference", 0)?;
        st.serialize_field("cbc:ID", self.0.id())?;
        if let Some(uuid) = self.0.uuid() {
            st.serialize_field("cbc:UUID", uuid)?;
        }
        if let Some(issue_date) = self.0.issue_date() {
            st.serialize_field("cbc:IssueDate", issue_date.as_str())?;
        }
        if let Some(time) = self.0.issue_time() {
            st.serialize_field("cbc:IssueTime", time.as_str())?;
        }
        if let Some(code) = self.0.document_type_code() {
            st.serialize_field("cbc:DocumentTypeCode", code)?;
        }
        st.end()
    }
}

impl<'a> Serialize for BillingReferenceXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:BillingReference", 0)?;
        st.serialize_field(
            "cac:InvoiceDocumentReference",
            &InvoiceDocumentReferenceXml(self.0),
        )?;
        st.end()
    }
}

impl<'a> Serialize for AdditionalDocumentReferenceXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:AdditionalDocumentReference", 0)?;
        match self {
            AdditionalDocumentReferenceXml::InvoiceCounter(value) => {
                st.serialize_field("cbc:ID", "ICV")?;
                st.serialize_field("cbc:UUID", value)?;
            }
            AdditionalDocumentReferenceXml::PreviousInvoiceHash(value) => {
                st.serialize_field("cbc:ID", "PIH")?;
                st.serialize_field(
                    "cac:Attachment",
                    &AttachmentXml {
                        mime_code: "text/plain",
                        data: value,
                    },
                )?;
            }
            AdditionalDocumentReferenceXml::QrCode(value) => {
                st.serialize_field("cbc:ID", "QR")?;
                st.serialize_field(
                    "cac:Attachment",
                    &AttachmentXml {
                        mime_code: "text/plain",
                        data: value,
                    },
                )?;
            }
        }
        st.end()
    }
}

struct AttachmentXml<'a> {
    mime_code: &'a str,
    data: &'a str,
}

impl<'a> Serialize for AttachmentXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:Attachment", 0)?;
        st.serialize_field(
            "cbc:EmbeddedDocumentBinaryObject",
            &EmbeddedDocumentXml {
                mime_code: self.mime_code,
                data: self.data,
            },
        )?;
        st.end()
    }
}

struct TaxCategoryXml<'a> {
    omit_percent: bool,
    exemption: &'a super::TaxExemption,
    category: &'a VatCategory,
    percent: Decimal,
}

impl<'a> Serialize for TaxCategoryXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:TaxCategory", 0)?;
        st.serialize_field(
            "cbc:ID",
            &id_with_scheme_with_agency(
                "cbc:ID",
                "UN/ECE 5305",
                "6",
                vat_category_code(self.category),
            ),
        )?;
        if !self.omit_percent {
            st.serialize_field("cbc:Percent", &self.percent)?;
        }
        if let Some(code) = &self.exemption.reason_code {
            st.serialize_field("cbc:TaxExemptionReasonCode", code)?;
        }
        if let Some(reason) = &self.exemption.reason {
            st.serialize_field("cbc:TaxExemptionReason", reason)?;
        }
        st.serialize_field("cac:TaxScheme", &TaxSchemeXml)?;
        st.end()
    }
}

#[derive(Clone)]
struct TaxSubtotalData<'a> {
    omit_percent: bool,
    exemption: &'a super::TaxExemption,
    taxable_amount: Decimal,
    tax_amount: Decimal,
    currency: &'a str,
    category: &'a VatCategory,
    percent: Decimal,
}

fn allowance_charge<'a>(
    charge_indicator: bool,
    amount: Decimal,
    currency: &'a str,
    reason: &'a str,
    vat_category: &'a VatCategory,
    percent: Decimal,
) -> impl Serialize + 'a {
    struct AllowanceChargeSer<'a> {
        charge_indicator: bool,
        amount: Decimal,
        currency: &'a str,
        reason: &'a str,
        vat_category: &'a VatCategory,
        percent: Decimal,
    }
    impl<'a> Serialize for AllowanceChargeSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct("cac:AllowanceCharge", 0)?;
            st.serialize_field("cbc:ChargeIndicator", &self.charge_indicator)?;
            st.serialize_field("cbc:AllowanceChargeReason", self.reason)?;
            st.serialize_field(
                "cbc:Amount",
                &currency_amount("cbc:Amount", self.currency, self.amount),
            )?;
            st.serialize_field(
                "cac:TaxCategory",
                &TaxCategoryXml {
                    omit_percent: false,
                    category: self.vat_category,
                    percent: self.percent,
                    exemption: &super::TaxExemption::default(),
                },
            )?;
            st.end()
        }
    }
    AllowanceChargeSer {
        charge_indicator,
        amount,
        currency,
        reason,
        vat_category,
        percent,
    }
}

struct AdjustmentXml<'a>(&'a super::AllowanceCharge, &'a str);
impl Serialize for AdjustmentXml<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let a = self.0;
        if !(1..=super::AllowanceCharge::MAX_TAX_CATEGORY_COUNT).contains(&a.tax_category_count) {
            return Err(serde::ser::Error::custom(
                "tax category count must be between 1 and 1024",
            ));
        }
        let mut st = s.serialize_struct("cac:AllowanceCharge", 0)?;
        st.serialize_field("cbc:ChargeIndicator", &a.charge)?;
        if let Some(code) = &a.reason_code {
            st.serialize_field("cbc:AllowanceChargeReasonCode", code)?;
        }
        if let Some(reason) = &a.reason {
            st.serialize_field("cbc:AllowanceChargeReason", reason)?;
        }
        if let Some(value) = a.percentage {
            st.serialize_field("cbc:MultiplierFactorNumeric", &value)?;
        }
        st.serialize_field(
            "cbc:Amount",
            &currency_amount("cbc:Amount", self.1, a.amount),
        )?;
        if let Some(value) = a.base_amount {
            st.serialize_field(
                "cbc:BaseAmount",
                &currency_amount("cbc:BaseAmount", self.1, value),
            )?;
        }
        for _ in 0..a.tax_category_count {
            st.serialize_field(
                "cac:TaxCategory",
                &TaxCategoryXml {
                    omit_percent: a.omitted_rate,
                    category: &a.category,
                    percent: a.rate,
                    exemption: &a.exemption,
                },
            )?;
        }
        st.end()
    }
}

fn tax_total<'a>(
    amount: Decimal,
    currency: &'a str,
    subtotal: Vec<TaxSubtotalData<'a>>,
) -> impl Serialize + 'a {
    struct TaxTotalSer<'a> {
        amount: Decimal,
        currency: &'a str,
        subtotal: Vec<TaxSubtotalData<'a>>,
    }
    impl<'a> Serialize for TaxTotalSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct("cac:TaxTotal", 0)?;
            st.serialize_field(
                "cbc:TaxAmount",
                &currency_amount("cbc:TaxAmount", self.currency, self.amount),
            )?;
            for subtotal in &self.subtotal {
                st.serialize_field("cac:TaxSubtotal", &tax_subtotal(subtotal.clone()))?;
            }
            st.end()
        }
    }
    TaxTotalSer {
        amount,
        currency,
        subtotal,
    }
}

fn tax_subtotal<'a>(data: TaxSubtotalData<'a>) -> impl Serialize + 'a {
    struct TaxSubtotalSer<'a> {
        data: TaxSubtotalData<'a>,
    }
    impl<'a> Serialize for TaxSubtotalSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let data = &self.data;
            let mut st = s.serialize_struct("cac:TaxSubtotal", 0)?;
            st.serialize_field(
                "cbc:TaxableAmount",
                &currency_amount("cbc:TaxableAmount", data.currency, data.taxable_amount),
            )?;
            st.serialize_field(
                "cbc:TaxAmount",
                &currency_amount("cbc:TaxAmount", data.currency, data.tax_amount),
            )?;
            st.serialize_field(
                "cac:TaxCategory",
                &TaxCategoryXml {
                    omit_percent: data.omit_percent,
                    exemption: data.exemption,
                    category: data.category,
                    percent: data.percent,
                },
            )?;
            st.end()
        }
    }
    TaxSubtotalSer { data }
}

fn legal_monetary_total<'a>(
    currency: &'a str,
    totals: &'a super::InvoiceTotalsData,
) -> impl Serialize + 'a {
    struct LegalMonetaryTotalSer<'a> {
        currency: &'a str,
        omitted: [bool; 4],
        line_extension: Decimal,
        tax_exclusive: Decimal,
        tax_inclusive: Decimal,
        allowance_total: Decimal,
        charge_total: Decimal,
        prepaid: Decimal,
        payable_rounding: Decimal,
        payable: Decimal,
    }
    impl<'a> Serialize for LegalMonetaryTotalSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct("cac:LegalMonetaryTotal", 0)?;
            st.serialize_field(
                "cbc:LineExtensionAmount",
                &currency_amount(
                    "cbc:LineExtensionAmount",
                    self.currency,
                    self.line_extension,
                ),
            )?;
            st.serialize_field(
                "cbc:TaxExclusiveAmount",
                &currency_amount("cbc:TaxExclusiveAmount", self.currency, self.tax_exclusive),
            )?;
            st.serialize_field(
                "cbc:TaxInclusiveAmount",
                &currency_amount("cbc:TaxInclusiveAmount", self.currency, self.tax_inclusive),
            )?;
            if !self.omitted[0] {
                st.serialize_field(
                    "cbc:AllowanceTotalAmount",
                    &currency_amount(
                        "cbc:AllowanceTotalAmount",
                        self.currency,
                        self.allowance_total,
                    ),
                )?;
            }
            if !self.omitted[1] {
                st.serialize_field(
                    "cbc:ChargeTotalAmount",
                    &currency_amount("cbc:ChargeTotalAmount", self.currency, self.charge_total),
                )?;
            }
            if !self.omitted[2] {
                st.serialize_field(
                    "cbc:PrepaidAmount",
                    &currency_amount("cbc:PrepaidAmount", self.currency, self.prepaid),
                )?;
            }
            if !self.omitted[3] {
                st.serialize_field(
                    "cbc:PayableRoundingAmount",
                    &currency_amount(
                        "cbc:PayableRoundingAmount",
                        self.currency,
                        self.payable_rounding,
                    ),
                )?;
            }
            st.serialize_field(
                "cbc:PayableAmount",
                &currency_amount("cbc:PayableAmount", self.currency, self.payable),
            )?;
            st.end()
        }
    }
    LegalMonetaryTotalSer {
        omitted: totals.omitted_optional_amounts,
        currency,
        line_extension: totals.line_extension(),
        tax_exclusive: totals.taxable_amount(),
        tax_inclusive: totals.tax_inclusive_amount(),
        allowance_total: totals.allowance_total(),
        charge_total: totals.charge_total(),
        prepaid: totals.prepaid_amount(),
        payable_rounding: totals.payable_rounding_amount(),
        payable: totals.payable_amount(),
    }
}

fn invoice_line_tax_total<'a>(
    currency: &'a str,
    tax_amount: Decimal,
    rounding_amount: Decimal,
    subtotals: &'a [super::VatBreakdown],
    omit_tax: bool,
    omit_gross: bool,
) -> impl Serialize + 'a {
    struct InvoiceLineTaxTotalSer<'a> {
        currency: &'a str,
        tax_amount: Decimal,
        rounding_amount: Decimal,
        subtotals: &'a [super::VatBreakdown],
        omit_tax: bool,
        omit_gross: bool,
    }
    impl<'a> Serialize for InvoiceLineTaxTotalSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct("cac:TaxTotal", 0)?;
            if !self.omit_tax {
                st.serialize_field(
                    "cbc:TaxAmount",
                    &currency_amount("cbc:TaxAmount", self.currency, self.tax_amount),
                )?;
            }
            if !self.omit_gross {
                st.serialize_field(
                    "cbc:RoundingAmount",
                    &currency_amount("cbc:RoundingAmount", self.currency, self.rounding_amount),
                )?;
            }
            for g in self.subtotals {
                st.serialize_field(
                    "cac:TaxSubtotal",
                    &tax_subtotal(TaxSubtotalData {
                        omit_percent: g.omitted_rate,
                        exemption: &g.exemption,
                        taxable_amount: g.taxable_amount,
                        tax_amount: g.tax_amount,
                        currency: self.currency,
                        category: &g.category,
                        percent: g.rate,
                    }),
                )?;
            }
            st.end()
        }
    }
    InvoiceLineTaxTotalSer {
        currency,
        tax_amount,
        rounding_amount,
        subtotals,
        omit_tax,
        omit_gross,
    }
}

fn invoice_item<'a>(
    description: &'a str,
    vat_category: &'a VatCategory,
    vat_rate: Decimal,
    exemption: &'a super::TaxExemption,
    line: &'a LineItem,
) -> impl Serialize + 'a {
    struct InvoiceItemSer<'a> {
        description: &'a str,
        vat_category: &'a VatCategory,
        vat_rate: Decimal,
        exemption: &'a super::TaxExemption,
        line: &'a LineItem,
    }
    impl<'a> Serialize for InvoiceItemSer<'a> {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut st = s.serialize_struct("cac:Item", 0)?;
            st.serialize_field("cbc:Name", self.description)?;
            for (tag, id) in [
                ("cac:BuyersItemIdentification", &self.line.buyer_item_id),
                ("cac:SellersItemIdentification", &self.line.seller_item_id),
                (
                    "cac:StandardItemIdentification",
                    &self.line.standard_item_id,
                ),
            ] {
                if let Some(id) = id {
                    st.serialize_field(tag, &PartyIdentificationXml(id))?;
                }
            }
            st.serialize_field(
                "cac:ClassifiedTaxCategory",
                &TaxCategoryXml {
                    omit_percent: self.line.omitted_vat_rate,
                    category: self.vat_category,
                    percent: self.vat_rate,
                    exemption: self.exemption,
                },
            )?;
            st.end()
        }
    }
    InvoiceItemSer {
        description,
        vat_category,
        vat_rate,
        exemption,
        line,
    }
}

#[derive(serde::Serialize)]
struct ExactPrice<'a> {
    #[serde(rename = "@currencyID")]
    currency: &'a str,
    #[serde(rename = "$text")]
    value: Decimal,
}
#[derive(serde::Serialize)]
struct BasisQuantity<'a> {
    #[serde(rename = "@unitCode", skip_serializing_if = "Option::is_none")]
    unit: Option<&'a str>,
    #[serde(rename = "$text")]
    value: Decimal,
}
struct LineAdjustmentXml<'a>(&'a super::LineAdjustment, &'a str);
impl Serialize for LineAdjustmentXml<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let a = &self.0.adjustment;
        let mut st = s.serialize_struct("cac:AllowanceCharge", 0)?;
        st.serialize_field("cbc:ChargeIndicator", &self.0.charge)?;
        if let Some(code) = &a.reason_code {
            st.serialize_field("cbc:AllowanceChargeReasonCode", code)?;
        }
        if let Some(reason) = &a.reason {
            st.serialize_field("cbc:AllowanceChargeReason", reason)?;
        }
        if let Some(percent) = a.percentage {
            st.serialize_field("cbc:MultiplierFactorNumeric", &percent)?;
        }
        st.serialize_field(
            "cbc:Amount",
            &currency_amount(
                "cbc:Amount",
                self.1,
                a.value().map_err(serde::ser::Error::custom)?,
            ),
        )?;
        if let Some(base) = a.base_amount {
            st.serialize_field(
                "cbc:BaseAmount",
                &currency_amount("cbc:BaseAmount", self.1, base),
            )?;
        }
        st.end()
    }
}
fn invoice_line_price<'a>(currency: &'a str, line: &'a LineItem) -> impl Serialize + 'a {
    struct Price<'a>(&'a str, &'a LineItem);
    impl Serialize for Price<'_> {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut st = s.serialize_struct("cac:Price", 0)?;
            st.serialize_field(
                "cbc:PriceAmount",
                &ExactPrice {
                    currency: self.0,
                    value: self.1.unit_price,
                },
            )?;
            if let Some(value) = self.1.price_base_quantity {
                st.serialize_field(
                    "cbc:BaseQuantity",
                    &BasisQuantity {
                        value,
                        unit: self.1.price_base_unit.as_deref(),
                    },
                )?;
            }
            if let Some(discount) = self.1.price_discount {
                #[derive(serde::Serialize)]
                struct Discount<'a> {
                    #[serde(rename = "cbc:ChargeIndicator")]
                    charge: bool,
                    #[serde(rename = "cbc:Amount")]
                    amount: ExactPrice<'a>,
                    #[serde(rename = "cbc:BaseAmount", skip_serializing_if = "Option::is_none")]
                    base: Option<ExactPrice<'a>>,
                }
                st.serialize_field(
                    "cac:AllowanceCharge",
                    &Discount {
                        charge: false,
                        amount: ExactPrice {
                            currency: self.0,
                            value: discount,
                        },
                        base: self.1.gross_price.map(|value| ExactPrice {
                            currency: self.0,
                            value,
                        }),
                    },
                )?;
            }
            st.end()
        }
    }
    Price(currency, line)
}

struct PaymentXml<'a>(&'a super::PaymentMeans);
impl Serialize for PaymentXml<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut st = s.serialize_struct("cac:PaymentMeans", 0)?;
        st.serialize_field("cbc:PaymentMeansCode", &self.0.payment_means_type_code)?;
        for note in &self.0.instruction_notes {
            st.serialize_field("cbc:InstructionNote", note)?;
        }
        if let Some(account) = &self.0.payee_financial_account {
            #[derive(serde::Serialize)]
            struct Account<'a> {
                #[serde(rename = "cbc:ID", skip_serializing_if = "Option::is_none")]
                id: Option<&'a str>,
                #[serde(rename = "cbc:PaymentNote", skip_serializing_if = "Option::is_none")]
                note: Option<&'a str>,
            }
            st.serialize_field(
                "cac:PayeeFinancialAccount",
                &Account {
                    id: account.payment_account_identifier.as_deref(),
                    note: account.payment_terms.as_deref(),
                },
            )?;
        }
        st.end()
    }
}

impl<'a, R: PartyRole> Serialize for PartyXml<'a, R> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let p = self.0;

        let mut st = s.serialize_struct("cac:Party", 0)?;

        for other_id in &p.identifiers {
            st.serialize_field("cac:PartyIdentification", &PartyIdentificationXml(other_id))?;
        }

        // Identification
        // Address
        if let Some(address) = &p.address {
            st.serialize_field("cac:PostalAddress", &AddressXml(address))?;
        }

        if let Some(vat) = &p.vat_id {
            st.serialize_field("cac:PartyTaxScheme", &VatSchemeXml(vat))?;
        }

        // Legal entity
        st.serialize_field("cac:PartyLegalEntity", &PartyLegalEntityXml(&p.name))?;

        st.end()
    }
}

struct AddressXml<'a>(&'a Address);

impl<'a> Serialize for AddressXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let a = self.0;
        let mut st = s.serialize_struct("cac:PostalAddress", 0)?;

        if !a.street.is_empty() {
            st.serialize_field("cbc:StreetName", &a.street)?;
        }
        if let Some(additional) = &a.additional_street {
            st.serialize_field("cbc:AdditionalStreetName", additional)?;
        }
        if !a.building_number.is_empty() {
            st.serialize_field("cbc:BuildingNumber", &a.building_number)?;
        }
        if let Some(additional_number) = &a.additional_number {
            st.serialize_field("cbc:PlotIdentification", additional_number)?;
        }
        if let Some(district) = &a.district {
            st.serialize_field("cbc:CitySubdivisionName", district)?;
        }
        if !a.city.is_empty() {
            st.serialize_field("cbc:CityName", &a.city)?;
        }
        if !a.postal_code.is_empty() {
            st.serialize_field("cbc:PostalZone", &a.postal_code)?;
        }
        let alpha2 = a.country_code.alpha2();
        if let Some(province) = &a.province {
            st.serialize_field("cbc:CountrySubentity", province)?;
        }
        st.serialize_field("cac:Country", &CountryXml(&alpha2))?;

        st.end()
    }
}

struct CountryXml<'a>(&'a str);

impl<'a> Serialize for CountryXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = s.serialize_struct("cac:Country", 0)?;
        st.serialize_field("cbc:IdentificationCode", self.0)?;
        st.end()
    }
}

struct InvoiceLineXml<'a>(usize, &'a LineItem, &'a InvoiceData);

impl<'a> Serialize for InvoiceLineXml<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (idx, li, invoice) = (self.0, self.1, self.2);

        let mut st = s.serialize_struct("cac:InvoiceLine", 0)?;

        st.serialize_field("cbc:ID", li.id.as_deref().unwrap_or(&idx.to_string()))?;

        st.serialize_field(
            "cbc:InvoicedQuantity",
            &quantity_with_unit("cbc:InvoicedQuantity", li.quantity, &li.unit_code),
        )?;
        st.serialize_field(
            "cbc:LineExtensionAmount",
            &currency_amount(
                "cbc:LineExtensionAmount",
                invoice.currency.as_str(),
                li.total_amount,
            ),
        )?;
        for reference in &li.document_references {
            st.serialize_field(
                "cac:DocumentReference",
                &InvoiceDocumentReferenceXml(reference),
            )?;
        }
        for adjustment in &li.adjustments {
            st.serialize_field(
                "cac:AllowanceCharge",
                &LineAdjustmentXml(adjustment, invoice.currency.as_str()),
            )?;
        }
        if !li.omitted_tax_amount {
            st.serialize_field(
                "cac:TaxTotal",
                &invoice_line_tax_total(
                    invoice.currency.as_str(),
                    li.vat_amount,
                    li.total_amount
                        .add(li.vat_amount)
                        .map_err(serde::ser::Error::custom)?,
                    &li.tax_subtotals,
                    li.omitted_tax_amount,
                    li.omitted_gross_amount,
                ),
            )?;
        }
        st.serialize_field(
            "cac:Item",
            &invoice_item(
                &li.description,
                &li.vat_category,
                li.vat_rate,
                &li.exemption,
                li,
            ),
        )?;
        st.serialize_field(
            "cac:Price",
            &invoice_line_price(invoice.currency.as_str(), li),
        )?;

        st.end()
    }
}

impl FinalizedInvoice {
    /// Serialize to XML using two-space indentation.
    ///
    /// No serialization trait import is needed. Signed invoices instead expose
    /// their exact stored XML through `SignedInvoice::xml()` and `into_xml()`.
    ///
    /// # Errors
    /// Returns [`InvoiceXmlError`] if XML serialization fails.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use fatoora_core::invoice::FinalizedInvoice;
    /// fn export(invoice: &FinalizedInvoice) -> Result<String, fatoora_core::invoice::xml::InvoiceXmlError> {
    ///     invoice.to_xml()
    /// }
    /// ```
    pub fn to_xml(&self) -> Result<String, InvoiceXmlError> {
        self.to_xml_with_format(XmlFormat::Pretty {
            indent_char: ' ',
            indent_size: 2,
        })
    }

    /// Serialize unsigned invoice XML with the requested formatting.
    ///
    /// # Errors
    /// Returns [`InvoiceXmlError`] if XML serialization fails.
    pub fn to_xml_with_format(&self, format: XmlFormat) -> Result<String, InvoiceXmlError> {
        to_xml_with_format(self, format)
    }
}

fn to_xml_with_format<T: InvoiceView + ?Sized>(
    invoice: &T,
    format: XmlFormat,
) -> Result<String, InvoiceXmlError> {
    let mut buffer = String::with_capacity(4096);
    buffer.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    buffer.push('\n');

    {
        let mut serializer = QuickXmlSerializer::new(&mut buffer);
        if let XmlFormat::Pretty {
            indent_char,
            indent_size,
        } = format
        {
            serializer.indent(indent_char, indent_size);
        }
        InvoiceXml(invoice)
            .serialize(serializer)
            .map_err(|error| InvoiceXmlError::Serialize {
                source: crate::Diagnostic::new(error.to_string()),
            })?;
    }

    // XML normalizes literal CR and CRLF before parsing text. Character
    // references preserve the caller's value (including CR followed by LF).
    if buffer.contains('\r') {
        buffer = buffer.replace('\r', "&#13;");
    }
    if buffer.len() > constants::MAX_XML_DOCUMENT_BYTES {
        return Err(InvoiceXmlError::Serialize {
            source: crate::Diagnostic::new("XML output exceeds document byte limit"),
        });
    }
    Ok(buffer)
}

impl<'a, T: InvoiceView + ?Sized> Serialize for InvoiceXml<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let view = self.0;
        let data = view.data();
        let totals = InvoiceTotals::new(view);
        let currency_code = totals.currency();

        let mut root = serializer.serialize_struct("Invoice", 0)?;

        // ---- namespaces (attributes) ----
        root.serialize_field(
            "@xmlns",
            "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2",
        )?;
        root.serialize_field(
            "@xmlns:cac",
            "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
        )?;
        root.serialize_field(
            "@xmlns:cbc",
            "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2",
        )?;
        root.serialize_field(
            "@xmlns:ext",
            "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2",
        )?;

        // ---- identifiers & issue info ----
        root.serialize_field("cbc:ProfileID", "reporting:1.0")?;
        root.serialize_field("cbc:ID", &data.id)?;
        root.serialize_field("cbc:UUID", &data.uuid)?;
        root.serialize_field("cbc:IssueDate", data.issue_date())?;
        root.serialize_field("cbc:IssueTime", data.issue_time())?;

        // ---- invoice type ----
        root.serialize_field(
            "cbc:InvoiceTypeCode",
            &InvoiceTypeView(&data.invoice_type, data.flags),
        )?;
        for note in &data.notes {
            root.serialize_field("cbc:Note", &NoteXml(note))?;
        }
        root.serialize_field("cbc:DocumentCurrencyCode", currency_code)?;
        if let Some(currency) = &data.tax_currency {
            root.serialize_field("cbc:TaxCurrencyCode", currency.as_str())?;
        }

        #[derive(serde::Serialize)]
        struct IdReference<'a> {
            #[serde(rename = "cbc:ID")]
            id: &'a str,
        }
        if let Some(id) = &data.purchase_order_id {
            root.serialize_field("cac:OrderReference", &IdReference { id })?;
        }
        // ---- credit/debit references ----
        for reference in &data.billing_references {
            root.serialize_field("cac:BillingReference", &BillingReferenceXml(reference))?;
        }
        if let Some(id) = &data.contract_id {
            root.serialize_field("cac:ContractDocumentReference", &IdReference { id })?;
        }
        // ---- supporting references ----
        let counter = data.invoice_counter.to_string();
        root.serialize_field(
            "cac:AdditionalDocumentReference",
            &AdditionalDocumentReferenceXml::InvoiceCounter(&counter),
        )?;
        root.serialize_field(
            "cac:AdditionalDocumentReference",
            &AdditionalDocumentReferenceXml::PreviousInvoiceHash(&data.previous_invoice_hash),
        )?;
        if let Some(qr) = view.qr_code() {
            root.serialize_field(
                "cac:AdditionalDocumentReference",
                &AdditionalDocumentReferenceXml::QrCode(qr),
            )?;
        }

        // ---- parties ----
        root.serialize_field(
            "cac:AccountingSupplierParty",
            &AccountingSupplierPartyXml(&data.seller),
        )?;
        root.serialize_field(
            "cac:AccountingCustomerParty",
            &AccountingCustomerPartyXml(data.buyer.as_ref()),
        )?;

        if let Some(delivery) = &data.delivery {
            root.serialize_field(
                "cac:Delivery",
                &DeliveryXml {
                    actual: delivery.actual_date.as_ref().map(|d| d.as_str()),
                    latest: delivery.latest_date.as_ref().map(|d| d.as_str()),
                },
            )?;
        }
        for payment in &data.payment_means {
            root.serialize_field("cac:PaymentMeans", &PaymentXml(payment))?;
        }

        // ---- allowance / charges ----
        if data.adjustments.is_empty()
            && (totals.allowance_total() > Decimal::ZERO || data.allowance_reason.is_some())
        {
            root.serialize_field(
                "cac:AllowanceCharge",
                &allowance_charge(
                    false,
                    totals.allowance_total(),
                    currency_code,
                    data.allowance_reason.as_deref().unwrap_or("discount"),
                    totals.vat_category(),
                    totals.vat_percent(),
                ),
            )?;
        }
        if data.adjustments.is_empty() && totals.charge_total() > Decimal::ZERO {
            root.serialize_field(
                "cac:AllowanceCharge",
                &allowance_charge(
                    true,
                    totals.charge_total(),
                    currency_code,
                    data.allowance_reason.as_deref().unwrap_or("charge"),
                    totals.vat_category(),
                    totals.vat_percent(),
                ),
            )?;
        }

        for adjustment in &data.adjustments {
            root.serialize_field(
                "cac:AllowanceCharge",
                &AdjustmentXml(adjustment, currency_code),
            )?;
        }

        // ---- tax totals ----
        let tax_subtotals = view
            .totals()
            .vat_breakdown()
            .iter()
            .map(|g| TaxSubtotalData {
                omit_percent: g.omitted_rate,
                exemption: &g.exemption,
                taxable_amount: g.taxable_amount,
                tax_amount: g.tax_amount,
                currency: currency_code,
                category: &g.category,
                percent: g.rate,
            })
            .collect();
        let document_tax = tax_total(totals.tax_amount(), currency_code, tax_subtotals);
        if data.document_tax_first {
            root.serialize_field("cac:TaxTotal", &document_tax)?;
        }
        if let Some(amount) = data.tax_currency_amount {
            root.serialize_field(
                "cac:TaxTotal",
                &tax_total(
                    amount,
                    data.tax_currency
                        .as_ref()
                        .map_or(currency_code, |c| c.as_str()),
                    Vec::new(),
                ),
            )?;
        }
        if !data.document_tax_first {
            root.serialize_field("cac:TaxTotal", &document_tax)?;
        }

        // ---- legal monetary totals ----
        root.serialize_field(
            "cac:LegalMonetaryTotal",
            &legal_monetary_total(currency_code, view.totals()),
        )?;

        // ---- lines ----
        for (i, line) in data.line_items.iter().enumerate() {
            root.serialize_field("cac:InvoiceLine", &InvoiceLineXml(i + 1, line, data))?;
        }

        root.end()
    }
}

#[cfg(test)]
#[path = "../../tests/support/serializer.rs"]
mod test_serializer;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::invoice::{
        Address, CountryCode, InvoiceBuilder, InvoiceSubType, InvoiceType, LineItem, Party,
        SellerRole, VatCategory,
    };
    #[test]
    fn deserialized_adjustment_counts_fail_before_any_xml_is_written() {
        let adjustment = crate::invoice::AllowanceCharge {
            omitted_rate: false,
            base_amount: None,
            percentage: None,
            tax_category_count: 1,
            charge: true,
            amount: Decimal::from(1),
            reason: None,
            reason_code: None,
            category: VatCategory::Standard,
            rate: Decimal::from(15),
            exemption: crate::invoice::TaxExemption::default(),
        };
        for count in [
            0,
            crate::invoice::AllowanceCharge::MAX_TAX_CATEGORY_COUNT + 1,
            usize::MAX,
        ] {
            let mut value = serde_json::to_value(&adjustment).unwrap();
            value["tax_category_count"] = serde_json::json!(count);
            let decoded = serde_json::from_value(value).unwrap();
            let mut output = String::new();
            let error = AdjustmentXml(&decoded, "SAR")
                .serialize(QuickXmlSerializer::new(&mut output))
                .unwrap_err();
            assert!(error.to_string().contains("tax category count"));
            assert!(output.is_empty());
        }
    }

    #[test]
    fn test_invoice_xml_serialization() {
        let seller = Party::<SellerRole>::new(
            "Acme Inc".into(),
            Address {
                province: None,
                country_code: CountryCode::parse("SAU").expect("country code"),
                city: "Riyadh".into(),
                street: "King Fahd".into(),
                additional_street: None,
                building_number: "1234".into(),
                additional_number: Some("5678".into()),
                postal_code: "12222".into(),
                district: None,
            },
            "301121971500003",
            None,
        )
        .expect("valid seller");

        let line_item = LineItem::new(
            "Item",
            crate::Decimal::parse("1.0").unwrap(),
            "PCE",
            crate::Decimal::parse("100.0").unwrap(),
            crate::Decimal::parse("15.0").unwrap(),
            VatCategory::Standard,
        )
        .unwrap();

        let mut builder = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified));
        builder = builder
            .id("INV-1")
            .uuid("uuid-123")
            .issue_datetime("2024-01-01T12:30:00Z")
            .currency("SAR")
            .previous_invoice_hash("hash")
            .invoice_counter(0)
            .seller(seller)
            .payment_means_code("10")
            .vat_category(VatCategory::Standard)
            .line_item(line_item);
        let invoice = builder.build().expect("build invoice");
        let xml_result = invoice.to_xml().unwrap();
        println!("{xml_result:?}");

        let pretty = invoice
            .to_xml_with_format(XmlFormat::Pretty {
                indent_char: ' ',
                indent_size: 2,
            })
            .unwrap();
        assert_eq!(xml_result, pretty);
    }
    #[test]
    fn serializer_propagates_output_failures_in_nested_metadata() {
        use crate::invoice::{
            AllowanceCharge, Delivery, InvoiceDate, OriginalInvoiceRef, TaxExemption, VatBreakdown,
        };
        // Exercise the real XML serializer with a sink failing at every write boundary.
        // Each failure must propagate, including failures inside optional/repeated fields.
        struct FailingWriter {
            remaining: usize,
            calls: usize,
            output: String,
        }
        impl std::fmt::Write for FailingWriter {
            fn write_str(&mut self, text: &str) -> std::fmt::Result {
                if self.remaining == 0 {
                    return Err(std::fmt::Error);
                }
                self.remaining -= 1;
                self.calls += 1;
                self.output.push_str(text);
                Ok(())
            }
        }
        let xml = include_str!("../../tests/fixtures/invoices/sample-simplified-invoice.xml");
        let original = parse::parse_finalized_invoice_xml(xml).unwrap();
        let delivery = Delivery {
            actual_date: Some(InvoiceDate::parse("2024-01-01").unwrap()),
            latest_date: Some(InvoiceDate::parse("2024-01-02").unwrap()),
        };
        let adjustment = AllowanceCharge {
            omitted_rate: false,
            base_amount: None,
            percentage: None,
            tax_category_count: 2,
            charge: true,
            amount: Decimal::from(1),
            reason: Some("fee".into()),
            reason_code: Some("95".into()),
            category: VatCategory::Standard,
            rate: Decimal::from(15),
            exemption: TaxExemption::default(),
        };
        let line = original.data().line_items()[0]
            .clone()
            .document_reference(OriginalInvoiceRef::new("PRE-1"))
            .tax_subtotal(
                VatBreakdown::new(
                    VatCategory::Standard,
                    Decimal::from(15),
                    Decimal::from(10),
                    Decimal::parse("1.5").unwrap(),
                )
                .unwrap(),
            );
        let invoice = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
            .id("writer-test")
            .uuid("writer-test")
            .issue_datetime("2024-01-01T12:30:00Z")
            .currency("SAR")
            .previous_invoice_hash("hash")
            .invoice_counter(1)
            .seller(original.data().seller().clone())
            .payment_means_code("10")
            .vat_category(VatCategory::Standard)
            .line_item(line)
            .delivery(delivery)
            .adjustment(adjustment)
            .build()
            .unwrap();
        fn fails(value: impl Serialize, field: &'static str) {
            let error = value
                .serialize(test_serializer::FailField(field))
                .unwrap_err();
            assert_eq!(error.to_string(), format!("injected failure: {field}"));
        }
        let mut adjustment = invoice.data().adjustments()[0].clone();
        adjustment.base_amount = Some(Decimal::from(10));
        fails(AdjustmentXml(&adjustment, "SAR"), "cbc:BaseAmount");
        for field in [
            "cbc:AllowanceTotalAmount",
            "cbc:ChargeTotalAmount",
            "cbc:PrepaidAmount",
            "cbc:PayableRoundingAmount",
        ] {
            fails(legal_monetary_total("SAR", &invoice.totals), field);
        }
        for field in ["cbc:TaxAmount", "cbc:RoundingAmount"] {
            fails(
                invoice_line_tax_total(
                    "SAR",
                    Decimal::from(1),
                    Decimal::from(2),
                    &[],
                    false,
                    false,
                ),
                field,
            );
        }
        for (omit_tax, omit_gross) in [(true, false), (false, true), (true, true)] {
            let mut output = String::new();
            invoice_line_tax_total(
                "SAR",
                Decimal::from(1),
                Decimal::from(2),
                &[],
                omit_tax,
                omit_gross,
            )
            .serialize(QuickXmlSerializer::new(&mut output))
            .unwrap();
            assert_eq!(output.contains("cbc:TaxAmount"), !omit_tax);
            assert_eq!(output.contains("cbc:RoundingAmount"), !omit_gross);
        }
        let mut priced_line = invoice.data().line_items()[0].clone();
        priced_line.price_base_quantity = Some(Decimal::from(1));
        priced_line.price_discount = Some(Decimal::from(1));
        priced_line.gross_price = Some(priced_line.unit_price.add(Decimal::from(1)).unwrap());
        for field in ["cbc:BaseQuantity", "cac:AllowanceCharge"] {
            fails(invoice_line_price("SAR", &priced_line), field);
        }
        let line_adjustment = crate::invoice::LineAdjustment {
            charge: false,
            adjustment: crate::invoice::Adjustment::percentage(
                Decimal::from(10),
                Decimal::from(10),
            ),
        };
        for field in ["cbc:Amount", "cbc:BaseAmount"] {
            fails(LineAdjustmentXml(&line_adjustment, "SAR"), field);
        }
        priced_line.adjustments.push(line_adjustment);
        for field in ["cac:AllowanceCharge", "cac:TaxTotal"] {
            fails(InvoiceLineXml(1, &priced_line, invoice.data()), field);
        }
        let payment = crate::invoice::PaymentMeans::new("30").payee_financial_account(
            crate::invoice::PaymentAccount {
                payment_account_identifier: Some("bank".into()),
                payment_terms: None,
            },
        );
        fails(PaymentXml(&payment), "cac:PayeeFinancialAccount");
        fails(InvoiceXml(&invoice), "cac:TaxTotal");
        for field in ["cbc:TaxCurrencyCode", "cac:Delivery", "cac:AllowanceCharge"] {
            fails(InvoiceXml(&invoice), field);
        }
        for field in ["cbc:Amount", "cac:TaxCategory"] {
            fails(
                AdjustmentXml(&invoice.data().adjustments()[0], "SAR"),
                field,
            );
        }
        let line = &invoice.data().line_items()[0];
        fails(
            InvoiceLineXml(1, line, invoice.data()),
            "cac:DocumentReference",
        );
        fails(
            invoice_line_tax_total(
                "SAR",
                line.vat_amount(),
                line.total_amount(),
                line.tax_subtotals(),
                false,
                false,
            ),
            "cac:TaxSubtotal",
        );
        let mut good = FailingWriter {
            remaining: usize::MAX,
            calls: 0,
            output: String::new(),
        };
        InvoiceXml(&invoice)
            .serialize(QuickXmlSerializer::new(&mut good))
            .unwrap();
        assert!(good.output.contains("<cac:Delivery>"));
        assert!(good.output.contains("<cac:DocumentReference>"));
        for limit in 0..good.calls {
            let mut sink = FailingWriter {
                remaining: limit,
                calls: 0,
                output: String::new(),
            };
            assert!(
                InvoiceXml(&invoice)
                    .serialize(QuickXmlSerializer::new(&mut sink))
                    .is_err(),
                "write boundary {limit}"
            );
            assert!(good.output.starts_with(&sink.output));
        }
    }
}
