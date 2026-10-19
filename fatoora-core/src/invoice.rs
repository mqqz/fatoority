//! Invoice domain types and builders.
use crate::Decimal;
mod builder;
mod line_builder;
pub use line_builder::{Adjustment, DocumentAdjustment, LineAdjustment, LineItemBuilder};
mod flags;
pub use flags::{InvoiceFlagNames, InvoiceFlags, InvoiceFlagsIter};
mod qr;
pub mod sign;
pub mod validation;
pub mod xml;
use builder::InvoiceView;
pub use builder::{FinalizedInvoice, InvoiceBuilder, SignedInvoice};
pub use qr::{QrCodeError, QrPayload, QrResult};

use chrono::{NaiveDate, NaiveDateTime};
use iso_currency::Currency as IsoCurrency;
use isocountry::CountryCode as IsoCountryCode;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::str::FromStr;
use thiserror::Error;

type Result<T> = std::result::Result<T, InvoiceError>;

// Keep the same named newtype representation as derived Serialize, while routing
// the inner string through each wrapper's validating constructor.
macro_rules! deserialize_validated_string {
    ($($wrapper:ident),+ $(,)?) => {
        $(
            impl<'de> Deserialize<'de> for $wrapper {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct WrapperVisitor;

                    impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
                        type Value = $wrapper;

                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter.write_str(concat!("a valid ", stringify!($wrapper), " newtype"))
                        }

                        fn visit_newtype_struct<D>(
                            self,
                            deserializer: D,
                        ) -> std::result::Result<Self::Value, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            let value = String::deserialize(deserializer)?;
                            $wrapper::parse(value).map_err(serde::de::Error::custom)
                        }
                    }

                    deserializer.deserialize_newtype_struct(stringify!($wrapper), WrapperVisitor)
                }
            }
        )+
    };
}

deserialize_validated_string!(
    CountryCode,
    CurrencyCode,
    InvoiceTimestamp,
    InvoiceDate,
    InvoiceTime,
    VatId
);

/// Invoice-related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InvoiceError {
    #[error(transparent)]
    Decimal(#[from] crate::DecimalError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("Invalid country code: {0}")]
    InvalidCountryCode(String),
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),
    #[error("Invalid invoice timestamp: {0}")]
    InvalidTimestamp(String),
    #[error("Invalid invoice date: {0}")]
    InvalidIssueDate(String),
    #[error("Missing VAT ID for seller")]
    MissingVatForSeller,
    #[error("Missing Buyer ID for buyer")]
    MissingBuyerId,
    #[error("Invalid VAT ID format")]
    InvalidVatFormat,
}

impl InvoiceError {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        match self {
            Self::Decimal(_) => crate::ErrorKind::InvalidInput,
            Self::Validation(_) => crate::ErrorKind::Validation,
            Self::InvalidCountryCode(_)
            | Self::InvalidCurrencyCode(_)
            | Self::InvalidTimestamp(_)
            | Self::InvalidIssueDate(_)
            | Self::MissingVatForSeller
            | Self::MissingBuyerId
            | Self::InvalidVatFormat => crate::ErrorKind::InvalidInput,
        }
    }
}

/// Structured validation error with field-level issues.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidationError {
    issues: Vec<ValidationIssue>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invoice validation failed")?;
        for issue in &self.issues {
            write!(f, "; {:?}: {:?}", issue.field, issue.kind)?;
            if let Some(index) = issue.line_item_index {
                write!(f, " at line {}", index + 1)?;
            }
            if let (Some(supplied), Some(expected)) = (issue.supplied, issue.expected) {
                write!(f, ", supplied {supplied}, expected {expected}")?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ValidationError {}
impl ValidationError {
    pub fn new(issues: Vec<ValidationIssue>) -> Self {
        Self { issues }
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }
}

/// Single validation issue.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidationIssue {
    field: InvoiceField,
    kind: ValidationKind,
    line_item_index: Option<usize>,
    supplied: Option<Decimal>,
    expected: Option<Decimal>,
}

impl ValidationIssue {
    fn mismatch(field: InvoiceField, supplied: Decimal, expected: Decimal) -> Self {
        Self {
            field,
            kind: ValidationKind::Mismatch,
            line_item_index: None,
            supplied: Some(supplied),
            expected: Some(expected),
        }
    }
    pub fn supplied(&self) -> Option<Decimal> {
        self.supplied
    }
    pub fn expected(&self) -> Option<Decimal> {
        self.expected
    }

    pub fn new(field: InvoiceField, kind: ValidationKind, line_item_index: Option<usize>) -> Self {
        Self {
            field,
            kind,
            line_item_index,
            supplied: None,
            expected: None,
        }
    }

    pub fn field(&self) -> InvoiceField {
        self.field
    }

    pub fn kind(&self) -> ValidationKind {
        self.kind
    }

    pub fn line_item_index(&self) -> Option<usize> {
        self.line_item_index
    }
}

#[non_exhaustive]
/// Field associated with a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InvoiceField {
    Id,
    Uuid,
    IssueDateTime,
    Currency,
    PreviousInvoiceHash,
    InvoiceCounter,
    Seller,
    Buyer,
    LineItems,
    PaymentMeansCode,
    PaymentInstruction,
    OriginalInvoiceId,
    VatCategory,
    LineItemId,
    LineItemDocumentReferenceId,
    LineItemDescription,
    LineItemUnitCode,
    LineItemQuantity,
    LineItemUnitPrice,
    LineItemTotalAmount,
    LineItemVatRate,
    LineItemVatAmount,
    InvoiceLevelDiscount,
    InvoiceLevelCharge,
}

#[non_exhaustive]
/// Classification of validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationKind {
    Missing,
    Empty,
    InvalidFormat,
    OutOfRange,
    Mismatch,
}

/// Country code wrapper with ISO validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_uppercase();
        let normalized = match value.len() {
            2 => IsoCountryCode::for_alpha2(&value)
                .map_err(|_| InvoiceError::InvalidCountryCode(value.clone()))?
                .alpha3()
                .to_string(),
            3 => IsoCountryCode::for_alpha3(&value)
                .map_err(|_| InvoiceError::InvalidCountryCode(value.clone()))?
                .alpha3()
                .to_string(),
            _ => return Err(InvoiceError::InvalidCountryCode(value)),
        };
        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn alpha2(&self) -> String {
        IsoCountryCode::for_alpha3(self.as_str())
            .expect("validated country code")
            .alpha2()
            .to_string()
    }
}

impl AsRef<str> for CountryCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CountryCode {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        CountryCode::parse(s)
    }
}

impl TryFrom<String> for CountryCode {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        CountryCode::parse(value)
    }
}

impl TryFrom<&str> for CountryCode {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        CountryCode::parse(value)
    }
}

/// Currency code wrapper with ISO validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct CurrencyCode(String);

impl CurrencyCode {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_uppercase();
        IsoCurrency::from_code(&value)
            .ok_or_else(|| InvoiceError::InvalidCurrencyCode(value.clone()))?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CurrencyCode {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        CurrencyCode::parse(s)
    }
}

impl TryFrom<String> for CurrencyCode {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        CurrencyCode::parse(value)
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        CurrencyCode::parse(value)
    }
}

/// Invoice timestamp in ZATCA ISO format (UTC `YYYY-MM-DDTHH:MM:SSZ`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct InvoiceTimestamp(String);

impl InvoiceTimestamp {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_string();
        let parsed = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%SZ")
            .map_err(|_| InvoiceError::InvalidTimestamp(value.clone()))?;
        Ok(Self(parsed.format("%Y-%m-%dT%H:%M:%SZ").to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn date_str(&self) -> &str {
        &self.0[..10]
    }
}

impl AsRef<str> for InvoiceTimestamp {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for InvoiceTimestamp {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        InvoiceTimestamp::parse(s)
    }
}

impl TryFrom<String> for InvoiceTimestamp {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        InvoiceTimestamp::parse(value)
    }
}

impl TryFrom<&str> for InvoiceTimestamp {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        InvoiceTimestamp::parse(value)
    }
}

/// Invoice date in `YYYY-MM-DD` format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct InvoiceDate(String);

impl InvoiceDate {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_string();
        let parsed = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .map_err(|_| InvoiceError::InvalidIssueDate(value.clone()))?;
        Ok(Self(parsed.format("%Y-%m-%d").to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for InvoiceDate {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for InvoiceDate {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        InvoiceDate::parse(s)
    }
}

impl TryFrom<String> for InvoiceDate {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        InvoiceDate::parse(value)
    }
}

impl TryFrom<&str> for InvoiceDate {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        InvoiceDate::parse(value)
    }
}

/// Invoice/reference time in `HH:MM:SS` (AST) or `HH:MM:SSZ` (UTC) format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct InvoiceTime(String);
impl InvoiceTime {
    pub fn parse(value: impl Into<String>) -> Result<Self> {
        let value = value.into().trim().to_owned();
        let clock = value.strip_suffix('Z').unwrap_or(&value);
        if clock == "24:00:00" {
            return Ok(Self(value));
        }
        let time = chrono::NaiveTime::parse_from_str(clock, "%H:%M:%S")
            .map_err(|_| InvoiceError::InvalidTimestamp(value.clone()))?;
        if clock.len() != 8 {
            return Err(InvoiceError::InvalidTimestamp(value));
        }
        let suffix = if value.ends_with('Z') { "Z" } else { "" };
        Ok(Self(format!("{}{suffix}", time.format("%H:%M:%S"))))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Postal address for parties.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    #[serde(default)]
    pub province: Option<String>,
    pub country_code: CountryCode,
    pub city: String,
    pub street: String,
    pub additional_street: Option<String>,
    pub building_number: String,
    pub additional_number: Option<String>,
    pub postal_code: String, //fix 5 digits if country is KSA
    /// City district, serialized as UBL `cbc:CitySubdivisionName`.
    pub district: Option<String>,
}

impl Address {
    pub fn province(&self) -> Option<&str> {
        self.province.as_deref()
    }
    pub fn country_code(&self) -> &CountryCode {
        &self.country_code
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn additional_street(&self) -> Option<&str> {
        self.additional_street.as_deref()
    }

    pub fn building_number(&self) -> &str {
        &self.building_number
    }

    pub fn additional_number(&self) -> Option<&str> {
        self.additional_number.as_deref()
    }

    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    pub fn district(&self) -> Option<&str> {
        self.district.as_deref()
    }
}

/// VAT identifier wrapper with validation helpers.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{VatId, InvoiceError};
///
/// let vat = VatId::parse("399999999900003")?;
/// assert_eq!(vat.as_str(), "399999999900003");
/// # Ok::<(), InvoiceError>(())
/// ```
///
/// # Errors
/// Returns [`InvoiceError::InvalidVatFormat`] if the input is empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct VatId(String);
impl VatId {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let s = s.into().trim().to_string();
        if s.is_empty() {
            return Err(InvoiceError::InvalidVatFormat);
        }
        // TODO: tighten validation (e.g., KSA = 15 digits)
        Ok(VatId(s))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for VatId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl FromStr for VatId {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        VatId::parse(s)
    }
}
impl TryFrom<String> for VatId {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        VatId::parse(value)
    }
}
impl TryFrom<&str> for VatId {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        VatId::parse(value)
    }
}

/// Additional party identifier.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::OtherId;
///
/// let id = OtherId::with_scheme("7003339333", "CRN");
/// assert_eq!(id.as_str(), "7003339333");
/// assert_eq!(id.scheme_id(), Some("CRN"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OtherId {
    value: String,
    scheme_id: Option<String>,
}
impl OtherId {
    pub fn new<S: Into<String>>(value: S) -> Self {
        OtherId {
            value: value.into(),
            scheme_id: None,
        }
    }

    pub fn with_scheme<V: Into<String>, S: Into<String>>(value: V, scheme_id: S) -> Self {
        OtherId {
            value: value.into(),
            scheme_id: Some(scheme_id.into()),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn scheme_id(&self) -> Option<&str> {
        self.scheme_id.as_deref()
    }
}
impl AsRef<str> for OtherId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Invoice note with language metadata.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::InvoiceNote;
///
/// let note = InvoiceNote::new("en", "Thank you");
/// assert_eq!(note.language(), "en");
/// assert_eq!(note.text(), "Thank you");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InvoiceNote {
    #[serde(default)]
    language_omitted: bool,
    language: String,
    text: String,
}

impl InvoiceNote {
    pub fn without_language(text: impl Into<String>) -> Self {
        Self {
            language_omitted: true,
            language: "en".into(),
            text: text.into(),
        }
    }
    pub fn language_id(&self) -> Option<&str> {
        (!self.language_omitted).then_some(self.language.as_str())
    }

    pub fn new(language: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            language_omitted: false,
            language: language.into(),
            text: text.into(),
        }
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

// Marker roles
/// Sealed marker trait for party role types.
pub trait PartyRole: party_role::Sealed {}

mod party_role {
    pub trait Sealed {}
    impl Sealed for super::SellerRole {}
    impl Sealed for super::BuyerRole {}
}

/// Seller role marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SellerRole;
impl PartyRole for SellerRole {}
/// Buyer role marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct BuyerRole;
impl PartyRole for BuyerRole {}

/// Party wrapper with role-specific typing.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{Party, SellerRole, Address, OtherId};
/// use fatoora_core::invoice::CountryCode;
///
/// let seller = Party::<SellerRole>::new(
///     "Acme Inc".into(),
///     Address {
///         province: None,
///         country_code: CountryCode::parse("SAU")?,
///         city: "Riyadh".into(),
///         street: "King Fahd".into(),
///         additional_street: None,
///         building_number: "1234".into(),
///         additional_number: Some("5678".into()),
///         postal_code: "12222".into(),
///         district: None,
///     },
///     "399999999900003",
///     Some(OtherId::with_scheme("7003339333", "CRN")),
/// )?;
/// # let _ = seller;
/// use fatoora_core::invoice::InvoiceError;
/// # Ok::<(), InvoiceError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Party<R: PartyRole> {
    _marker: PhantomData<R>,
    name: String,
    address: Option<Address>,
    vat_id: Option<VatId>,
    #[serde(default)]
    identifiers: Vec<OtherId>,
}

pub type Seller = Party<SellerRole>;
pub type Buyer = Party<BuyerRole>;

impl Party<SellerRole> {
    /// Create a seller party from validated inputs.
    ///
    /// # Errors
    /// Returns an error if the VAT ID is invalid.
    pub fn new(
        name: String,
        address: Address,
        vat_id: impl Into<String>, // required
        other_id: Option<OtherId>, // optional
    ) -> Result<Self> {
        let vat = VatId::parse(vat_id.into())?;
        Ok(Party {
            _marker: PhantomData,
            name,
            address: Some(address),
            vat_id: Some(vat),
            identifiers: other_id.into_iter().collect(),
        })
    }
}

impl Party<BuyerRole> {
    /// Create a buyer party from validated inputs.
    ///
    /// # Errors
    /// Returns an error if a supplied VAT ID is empty. Identification requirements
    /// depend on invoice type and are checked by XML business-rule validation.
    pub fn new(
        name: String,
        address: impl Into<Option<Address>>,
        vat_id: Option<String>,    // optional
        other_id: Option<OtherId>, // required if vat_id is None
    ) -> Result<Self> {
        let vat = match vat_id {
            Some(v) => Some(VatId::parse(v)?),
            None => None,
        };
        Ok(Party {
            _marker: PhantomData,
            name,
            address: address.into(),
            vat_id: vat,
            identifiers: other_id.into_iter().collect(),
        })
    }
}

impl<R: PartyRole> Party<R> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    pub fn vat_id(&self) -> Option<&VatId> {
        self.vat_id.as_ref()
    }

    /// Append an identification, preserving its scheme and order.
    pub fn identifier(mut self, value: OtherId) -> Self {
        self.identifiers.push(value);
        self
    }
    pub fn identifiers(&self) -> &[OtherId] {
        &self.identifiers
    }
    pub fn other_id(&self) -> Option<&OtherId> {
        self.identifiers.first()
    }
}

/// Invoice subtype used for tax invoices and notes.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{InvoiceSubType, InvoiceType};
///
/// let invoice_type = InvoiceType::Tax(InvoiceSubType::Simplified);
/// assert!(invoice_type.is_simplified());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InvoiceSubType {
    Simplified,
    Standard,
}

/// Reference to an original invoice for credit/debit notes.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::OriginalInvoiceRef;
///
/// let original = OriginalInvoiceRef::new("INV-ORIG")
///     .with_uuid("uuid-orig");
/// assert_eq!(original.id(), "INV-ORIG");
/// assert_eq!(original.uuid(), Some("uuid-orig"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OriginalInvoiceRef {
    id: String,
    uuid: Option<String>,
    issue_date: Option<InvoiceDate>,
    #[serde(default)]
    issue_time: Option<InvoiceTime>,
    #[serde(default)]
    document_type_code: Option<String>,
}

impl OriginalInvoiceRef {
    pub fn with_issue_time(mut self, time: InvoiceTime) -> Self {
        self.issue_time = Some(time);
        self
    }
    pub fn with_document_type_code(mut self, code: impl Into<String>) -> Self {
        self.document_type_code = Some(code.into());
        self
    }
    pub fn issue_time(&self) -> Option<&InvoiceTime> {
        self.issue_time.as_ref()
    }
    pub fn document_type_code(&self) -> Option<&str> {
        self.document_type_code.as_deref()
    }

    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            uuid: None,
            issue_date: None,
            issue_time: None,
            document_type_code: None,
        }
    }

    pub fn with_uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    pub fn with_issue_date(mut self, issue_date: InvoiceDate) -> Self {
        self.issue_date = Some(issue_date);
        self
    }

    pub fn with_issue_date_str(mut self, issue_date: impl Into<String>) -> Result<Self> {
        self.issue_date = Some(InvoiceDate::parse(issue_date)?);
        Ok(self)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    pub fn issue_date(&self) -> Option<&InvoiceDate> {
        self.issue_date.as_ref()
    }
}

/// Invoice type and required metadata.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{InvoiceSubType, InvoiceType};
///
/// let invoice_type = InvoiceType::Prepayment(InvoiceSubType::Standard);
/// assert!(!invoice_type.is_simplified());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InvoiceType {
    Tax(InvoiceSubType),
    Prepayment(InvoiceSubType),
    CreditNote(InvoiceSubType),
    DebitNote(InvoiceSubType),
}

impl InvoiceType {
    pub fn is_simplified(&self) -> bool {
        matches!(
            self,
            InvoiceType::Tax(InvoiceSubType::Simplified)
                | InvoiceType::Prepayment(InvoiceSubType::Simplified)
                | InvoiceType::CreditNote(InvoiceSubType::Simplified, ..)
                | InvoiceType::DebitNote(InvoiceSubType::Simplified, ..)
        )
    }
}

/// VAT category for line items.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::VatCategory;
///
/// let cat = VatCategory::Standard;
/// assert!(matches!(cat, VatCategory::Standard));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VatCategory {
    Exempt,
    Standard,
    Zero,
    OutOfScope,
}
/// Independent optional exemption code and description for a tax category.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxExemption {
    pub reason_code: Option<String>,
    pub reason: Option<String>,
}

/// A payment instruction. Absence of the collection is distinct from an invalid empty code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMeans {
    pub payment_means_type_code: String,
    pub instruction_notes: Vec<String>,
    pub payee_financial_account: Option<PaymentAccount>,
}
impl PaymentMeans {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            payment_means_type_code: code.into(),
            instruction_notes: Vec::new(),
            payee_financial_account: None,
        }
    }
    pub fn instruction_note(mut self, value: impl Into<String>) -> Self {
        self.instruction_notes.push(value.into());
        self
    }
    pub fn payee_financial_account(mut self, value: PaymentAccount) -> Self {
        self.payee_financial_account = Some(value);
        self
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentAccount {
    pub payment_account_identifier: Option<String>,
    pub payment_terms: Option<String>,
}

/// Supply date or supply period. Business-rule validation checks date ordering.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delivery {
    pub actual_date: Option<InvoiceDate>,
    pub latest_date: Option<InvoiceDate>,
}

fn one_tax_category() -> usize {
    1
}

/// One document allowance or charge, retaining its own VAT category and reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowanceCharge {
    /// Preserve an absent VAT rate for an out-of-scope adjustment.
    #[serde(default)]
    pub omitted_rate: bool,
    #[serde(default)]
    pub base_amount: Option<Decimal>,
    #[serde(default)]
    pub percentage: Option<Decimal>,
    /// Identical repeated VAT classifications, as used by the bundled SDK fixtures.
    /// Must be in `1..=Self::MAX_TAX_CATEGORY_COUNT` (an implementation resource limit).
    #[serde(default = "one_tax_category")]
    pub tax_category_count: usize,
    pub charge: bool,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub reason_code: Option<String>,
    pub category: VatCategory,
    pub rate: Decimal,
    #[serde(default)]
    pub exemption: TaxExemption,
}

impl AllowanceCharge {
    /// Resource limit for repeated identical classifications, not a UBL or ZATCA rule.
    pub const MAX_TAX_CATEGORY_COUNT: usize = 1024;
}

/// Single invoice line item.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{LineItem, VatCategory};
///
/// let item = LineItem::new("Item", fatoora_core::Decimal::parse("2.0").unwrap(), "PCE", fatoora_core::Decimal::parse("50.0").unwrap(), fatoora_core::Decimal::parse("15.0").unwrap(), VatCategory::Standard).unwrap();
/// assert_eq!(item.total_amount(), fatoora_core::Decimal::parse("100.0").unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    #[serde(default)]
    omitted_tax_amount: bool,
    #[serde(default)]
    omitted_gross_amount: bool,
    #[serde(default)]
    omitted_vat_rate: bool,
    #[serde(default)]
    price_base_quantity: Option<Decimal>,
    #[serde(default)]
    price_base_unit: Option<String>,
    #[serde(default)]
    price_discount: Option<Decimal>,
    #[serde(default)]
    gross_price: Option<Decimal>,
    #[serde(default)]
    adjustments: Vec<LineAdjustment>,
    #[serde(default)]
    seller_item_id: Option<OtherId>,
    #[serde(default)]
    buyer_item_id: Option<OtherId>,
    #[serde(default)]
    standard_item_id: Option<OtherId>,

    #[serde(default)]
    document_references: Vec<OriginalInvoiceRef>,
    #[serde(default)]
    tax_subtotals: Vec<VatBreakdown>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    exemption: TaxExemption,
    description: String,
    quantity: Decimal,
    unit_code: String,
    unit_price: Decimal,
    total_amount: Decimal,
    vat_rate: Decimal,
    vat_amount: Decimal,
    vat_category: VatCategory,
}

impl LineItem {
    pub fn document_reference(mut self, reference: OriginalInvoiceRef) -> Self {
        self.document_references.push(reference);
        self
    }
    pub fn tax_subtotal(mut self, subtotal: VatBreakdown) -> Self {
        self.tax_subtotals.push(subtotal);
        self
    }
    pub fn document_references(&self) -> &[OriginalInvoiceRef] {
        &self.document_references
    }
    /// A reference to an advance invoice identifies a prepayment adjustment line.
    pub fn is_prepayment_adjustment(&self) -> bool {
        self.document_references
            .iter()
            .any(|r| r.document_type_code().is_some_and(|v| v.trim() == "386"))
    }

    pub fn tax_subtotals(&self) -> &[VatBreakdown] {
        &self.tax_subtotals
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn with_exemption(mut self, exemption: TaxExemption) -> Self {
        self.exemption = exemption;
        self
    }
    pub fn exemption(&self) -> &TaxExemption {
        &self.exemption
    }

    pub fn new(
        description: impl Into<String>,
        quantity: Decimal,
        unit_code: impl Into<String>,
        unit_price: Decimal,
        vat_rate: Decimal,
        vat_category: VatCategory,
    ) -> Result<Self> {
        Self::builder()
            .item_name(description)
            .invoiced_quantity(quantity)
            .invoiced_quantity_unit_code(unit_code)
            .item_net_price(unit_price)
            .vat_rate(vat_rate)
            .vat_category(vat_category)
            .build()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn from_totals(
        description: impl Into<String>,
        quantity: Decimal,
        unit_code: impl Into<String>,
        unit_price: Decimal,
        total_amount: Decimal,
        vat_rate: Decimal,
        vat_category: VatCategory,
    ) -> Result<Self> {
        Self::builder()
            .item_name(description)
            .invoiced_quantity(quantity)
            .invoiced_quantity_unit_code(unit_code)
            .item_net_price(unit_price)
            .vat_rate(vat_rate)
            .vat_category(vat_category)
            .line_net_amount(total_amount)
            .build()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn try_from_parts(
        description: impl Into<String>,
        quantity: Decimal,
        unit_code: impl Into<String>,
        unit_price: Decimal,
        total_amount: Decimal,
        vat_rate: Decimal,
        vat_amount: Decimal,
        vat_category: VatCategory,
    ) -> Result<Self> {
        Self::builder()
            .item_name(description)
            .invoiced_quantity(quantity)
            .invoiced_quantity_unit_code(unit_code)
            .item_net_price(unit_price)
            .vat_rate(vat_rate)
            .vat_category(vat_category)
            .line_net_amount(total_amount)
            .line_vat_amount(vat_amount)
            .build()
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
    pub fn unit_code(&self) -> &str {
        &self.unit_code
    }
    pub fn unit_price(&self) -> Decimal {
        self.unit_price
    }
    pub fn total_amount(&self) -> Decimal {
        self.total_amount
    }
    pub fn vat_rate(&self) -> Decimal {
        self.vat_rate
    }
    pub fn vat_amount(&self) -> Decimal {
        self.vat_amount
    }
    pub fn vat_category(&self) -> VatCategory {
        self.vat_category
    }
}

/// Collection of line items.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{LineItem, LineItems, VatCategory};
///
/// let items: LineItems = vec![LineItem::new(
///     "Item",
///     fatoora_core::Decimal::parse("1.0").unwrap(),
///     "PCE",
///     fatoora_core::Decimal::parse("100.0").unwrap(),
///     fatoora_core::Decimal::parse("15.0").unwrap(),
///     VatCategory::Standard,
/// ).unwrap()];
/// assert_eq!(items.len(), 1);
/// ```
pub type LineItems = Vec<LineItem>;

/// Core invoice data model.
///
/// Instances are produced by the builder and exposed via views.
///
/// # Examples
/// ```rust,ignore
/// use fatoora_core::invoice::InvoiceData;
///
/// let data: InvoiceData = unimplemented!();
/// # let _ = data;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceData {
    #[serde(default)]
    issue_date_override: Option<InvoiceDate>,
    #[serde(default)]
    issue_time_override: Option<InvoiceTime>,
    #[serde(default)]
    document_tax_first: bool,
    invoice_type: InvoiceType,
    id: String,
    uuid: String,
    issue_datetime: InvoiceTimestamp,
    currency: CurrencyCode,
    #[serde(default)]
    tax_currency: Option<CurrencyCode>,
    #[serde(default)]
    tax_currency_amount: Option<Decimal>,
    previous_invoice_hash: String,
    invoice_counter: u64,
    notes: Vec<InvoiceNote>,
    billing_references: Vec<OriginalInvoiceRef>,
    #[serde(default)]
    purchase_order_id: Option<String>,
    #[serde(default)]
    contract_id: Option<String>,
    seller: Seller,
    buyer: Option<Buyer>,
    #[serde(default)]
    delivery: Option<Delivery>,
    line_items: LineItems,
    payment_means: Vec<PaymentMeans>,
    vat_category: VatCategory,

    flags: InvoiceFlags,

    invoice_level_charge: Decimal,
    invoice_level_discount: Decimal,
    #[serde(default)]
    adjustment_vat_rate: Option<Decimal>,
    allowance_reason: Option<String>,
    #[serde(default)]
    adjustments: Vec<AllowanceCharge>,
}

impl InvoiceData {
    pub fn billing_references(&self) -> &[OriginalInvoiceRef] {
        &self.billing_references
    }
    pub fn payment_means(&self) -> &[PaymentMeans] {
        &self.payment_means
    }
    pub fn notes(&self) -> &[InvoiceNote] {
        &self.notes
    }
    pub fn purchase_order_id(&self) -> Option<&str> {
        self.purchase_order_id.as_deref()
    }
    pub fn contract_id(&self) -> Option<&str> {
        self.contract_id.as_deref()
    }

    pub fn payment_instruction(&self) -> Option<&str> {
        self.payment_means
            .first()
            .and_then(|p| p.instruction_notes.first().map(String::as_str))
    }
    pub fn tax_currency(&self) -> Option<&CurrencyCode> {
        self.tax_currency.as_ref()
    }
    pub fn tax_currency_amount(&self) -> Option<Decimal> {
        self.tax_currency_amount
    }
    pub fn adjustments(&self) -> &[AllowanceCharge] {
        &self.adjustments
    }
    pub fn delivery(&self) -> Option<&Delivery> {
        self.delivery.as_ref()
    }
    pub fn invoice_type(&self) -> &InvoiceType {
        &self.invoice_type
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    /// Original clock representation; a missing suffix denotes AST, `Z` denotes UTC.
    pub fn issue_time(&self) -> &str {
        self.issue_time_override
            .as_ref()
            .map_or(&self.issue_datetime.as_str()[11..], InvoiceTime::as_str)
    }
    pub fn issue_date(&self) -> &str {
        self.issue_date_override
            .as_ref()
            .map_or(self.issue_datetime.date_str(), InvoiceDate::as_str)
    }

    pub fn issue_datetime(&self) -> &InvoiceTimestamp {
        &self.issue_datetime
    }

    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

    pub fn previous_invoice_hash(&self) -> &str {
        &self.previous_invoice_hash
    }

    pub fn invoice_counter(&self) -> u64 {
        self.invoice_counter
    }

    pub fn note(&self) -> Option<&InvoiceNote> {
        self.notes.first()
    }

    pub fn seller(&self) -> &Seller {
        &self.seller
    }

    pub fn buyer(&self) -> Option<&Buyer> {
        self.buyer.as_ref()
    }

    pub fn line_items(&self) -> &[LineItem] {
        &self.line_items
    }

    pub fn payment_means_code(&self) -> &str {
        self.payment_means
            .first()
            .map_or("", |p| p.payment_means_type_code.as_str())
    }

    pub fn vat_category(&self) -> VatCategory {
        self.vat_category
    }

    pub fn flags(&self) -> InvoiceFlags {
        self.flags
    }

    pub fn is_third_party(&self) -> bool {
        self.flags.contains(InvoiceFlags::THIRD_PARTY)
    }

    pub fn is_nominal(&self) -> bool {
        self.flags.contains(InvoiceFlags::NOMINAL)
    }

    pub fn is_export(&self) -> bool {
        self.flags.contains(InvoiceFlags::EXPORT)
    }

    pub fn is_summary(&self) -> bool {
        self.flags.contains(InvoiceFlags::SUMMARY)
    }

    pub fn is_self_billed(&self) -> bool {
        self.flags.contains(InvoiceFlags::SELF_BILLED)
    }

    pub fn invoice_level_charge(&self) -> Decimal {
        self.invoice_level_charge
    }

    pub fn invoice_level_discount(&self) -> Decimal {
        self.invoice_level_discount
    }

    pub fn allowance_reason(&self) -> Option<&str> {
        self.allowance_reason.as_deref().or_else(|| {
            let first = self.adjustments.first()?.reason.as_deref();
            self.adjustments
                .iter()
                .all(|a| a.reason.as_deref() == first)
                .then_some(first)
                .flatten()
        })
    }

    pub(crate) fn seller_name(&self) -> QrResult<&str> {
        let name = self.seller.name.as_str();
        if name.trim().is_empty() {
            return Err(QrCodeError::MissingSellerName);
        }
        Ok(name)
    }

    pub(crate) fn seller_vat(&self) -> QrResult<&str> {
        let vat = self
            .seller
            .vat_id
            .as_ref()
            .ok_or(QrCodeError::MissingSellerVat)?
            .as_str()
            .trim();
        if vat.is_empty() {
            return Err(QrCodeError::MissingSellerVat);
        }
        Ok(vat)
    }

    pub(crate) fn issue_date_string(&self) -> String {
        self.issue_date().to_string()
    }

    pub(crate) fn issue_time_string(&self) -> String {
        self.issue_time().to_string()
    }

    pub(crate) fn format_amount(amount: Decimal) -> String {
        amount.fixed(2)
    }
}
/// Computed invoice totals.
///
/// # Examples
/// ```rust,ignore
/// use fatoora_core::invoice::InvoiceTotalsData;
///
/// let totals: InvoiceTotalsData = unimplemented!();
/// let _ = totals.tax_inclusive_amount();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceTotalsData {
    #[serde(default)]
    pub(crate) omitted_optional_amounts: [bool; 4],
    line_extension: Decimal,
    tax_amount: Decimal,
    allowance_total: Decimal,
    charge_total: Decimal,
    taxable_amount: Decimal,
    tax_inclusive_amount: Decimal,
    pub(crate) prepaid_amount: Decimal,
    pub(crate) payable_rounding_amount: Decimal,
    pub(crate) payable_amount: Decimal,
    groups: Vec<VatBreakdown>,
}
/// Finalized document-level VAT calculation for one category and rate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VatBreakdown {
    #[serde(default)]
    pub(crate) exemption: TaxExemption,
    #[serde(default)]
    pub(crate) omitted_rate: bool,
    pub(crate) category: VatCategory,
    pub(crate) rate: Decimal,
    pub(crate) taxable_amount: Decimal,
    pub(crate) tax_amount: Decimal,
}
impl VatBreakdown {
    pub(crate) fn check_against(&self, expected: &Self, line_count: usize) -> Result<()> {
        line_builder::check_supplied(
            InvoiceField::VatCategory,
            self.taxable_amount,
            expected.taxable_amount,
            Decimal::ZERO,
        )?;
        let tolerance = if self.category == VatCategory::Standard {
            Decimal::from(line_count as i64).product_rounded("0.01".parse()?, false)?
        } else {
            Decimal::ZERO
        };
        if self.tax_amount < Decimal::ZERO {
            return Err(ValidationError::new(vec![ValidationIssue::new(
                InvoiceField::VatCategory,
                ValidationKind::OutOfRange,
                None,
            )])
            .into());
        }
        line_builder::check_supplied(
            InvoiceField::VatCategory,
            self.tax_amount,
            self.taxable_amount.vat_tolerance_center(self.rate)?,
            tolerance,
        )
    }

    /// Supplied line-level tax subtotal (for example a prepayment adjustment).
    pub fn new(
        category: VatCategory,
        rate: Decimal,
        taxable_amount: Decimal,
        tax_amount: Decimal,
    ) -> Result<Self> {
        if rate < Decimal::ZERO
            || rate > Decimal::from(100)
            || rate.scale() > 2
            || taxable_amount < Decimal::ZERO
            || taxable_amount.scale() > 2
            || tax_amount < Decimal::ZERO
            || tax_amount.scale() > 2
        {
            return Err(ValidationError::new(vec![ValidationIssue::new(
                InvoiceField::VatCategory,
                ValidationKind::OutOfRange,
                None,
            )])
            .into());
        }
        Ok(Self {
            omitted_rate: false,
            category,
            rate,
            taxable_amount,
            tax_amount,
            exemption: TaxExemption::default(),
        })
    }
    pub fn with_exemption(mut self, exemption: TaxExemption) -> Self {
        self.exemption = exemption;
        self
    }

    pub fn exemption(&self) -> &TaxExemption {
        &self.exemption
    }
    pub fn category(&self) -> VatCategory {
        self.category
    }
    pub fn rate(&self) -> Decimal {
        self.rate
    }
    pub fn taxable_amount(&self) -> Decimal {
        self.taxable_amount
    }
    pub fn tax_amount(&self) -> Decimal {
        self.tax_amount
    }
}
impl InvoiceTotalsData {
    pub(crate) fn from_data(data: &InvoiceData) -> Result<Self> {
        let mut line_extension = Decimal::ZERO;
        let mut groups: Vec<VatBreakdown> = Vec::new();
        for line in data.line_items.iter() {
            if line.is_prepayment_adjustment() {
                continue;
            }
            line_extension = line_extension.add(line.total_amount)?;
            if let Some(group) = groups
                .iter_mut()
                .find(|g| g.category == line.vat_category && g.rate == line.vat_rate)
            {
                group.taxable_amount = group.taxable_amount.add(line.total_amount)?;
            } else {
                groups.push(VatBreakdown {
                    omitted_rate: false,
                    exemption: TaxExemption::default(),
                    category: line.vat_category,
                    rate: line.vat_rate,
                    taxable_amount: line.total_amount,
                    tax_amount: Decimal::ZERO,
                });
            }
        }
        if data.adjustments.is_empty()
            && (data.invoice_level_discount != Decimal::ZERO
                || data.invoice_level_charge != Decimal::ZERO)
        {
            let matches: Vec<_> = groups
                .iter_mut()
                .filter(|g| {
                    g.category == data.vat_category
                        && data.adjustment_vat_rate.is_none_or(|rate| g.rate == rate)
                })
                .collect();
            if matches.len() != 1 {
                return Err(ValidationError::new(vec![ValidationIssue::new(
                    InvoiceField::VatCategory,
                    ValidationKind::Mismatch,
                    None,
                )])
                .into());
            }
            let group = matches.into_iter().next().unwrap();
            group.taxable_amount = group
                .taxable_amount
                .sub(data.invoice_level_discount)?
                .add(data.invoice_level_charge)?;
        }
        for adjustment in &data.adjustments {
            let index = groups
                .iter()
                .position(|g| g.category == adjustment.category && g.rate == adjustment.rate);
            let index = index.unwrap_or_else(|| {
                groups.push(VatBreakdown {
                    omitted_rate: false,
                    category: adjustment.category,
                    rate: adjustment.rate,
                    taxable_amount: Decimal::ZERO,
                    tax_amount: Decimal::ZERO,
                    exemption: TaxExemption::default(),
                });
                groups.len() - 1
            });
            let group = &mut groups[index];
            group.taxable_amount = if adjustment.charge {
                group.taxable_amount.add(adjustment.amount)?
            } else {
                group.taxable_amount.sub(adjustment.amount)?
            };
        }
        let mut tax_amount = Decimal::ZERO;
        // Validate final group balances: an allowance may precede a charge
        // that restores the same group's taxable amount.
        for group in &mut groups {
            if group.taxable_amount < Decimal::ZERO {
                return Err(ValidationError::new(vec![ValidationIssue::new(
                    InvoiceField::InvoiceLevelDiscount,
                    ValidationKind::OutOfRange,
                    None,
                )])
                .into());
            }
            group.tax_amount = group.taxable_amount.product_rounded(group.rate, true)?;
            tax_amount = tax_amount.add(group.tax_amount)?;
        }
        let taxable_amount = line_extension
            .sub(data.invoice_level_discount)?
            .add(data.invoice_level_charge)?;
        let tax_inclusive_amount = taxable_amount.add(tax_amount)?;
        Ok(Self {
            omitted_optional_amounts: [false; 4],
            line_extension,
            tax_amount,
            allowance_total: data.invoice_level_discount,
            charge_total: data.invoice_level_charge,
            taxable_amount,
            tax_inclusive_amount,
            prepaid_amount: Decimal::ZERO,
            payable_rounding_amount: Decimal::ZERO,
            payable_amount: tax_inclusive_amount,
            groups,
        })
    }
    pub fn line_extension(&self) -> Decimal {
        self.line_extension
    }
    pub fn tax_amount(&self) -> Decimal {
        self.tax_amount
    }
    pub fn allowance_total(&self) -> Decimal {
        self.allowance_total
    }
    pub fn charge_total(&self) -> Decimal {
        self.charge_total
    }
    pub fn taxable_amount(&self) -> Decimal {
        self.taxable_amount
    }
    pub fn tax_inclusive_amount(&self) -> Decimal {
        self.tax_inclusive_amount
    }
    pub fn prepaid_amount(&self) -> Decimal {
        self.prepaid_amount
    }
    pub fn payable_rounding_amount(&self) -> Decimal {
        self.payable_rounding_amount
    }
    pub fn payable_amount(&self) -> Decimal {
        self.payable_amount
    }
    pub fn vat_breakdown(&self) -> &[VatBreakdown] {
        &self.groups
    }
}
