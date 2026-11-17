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
    VatId,
    InvoiceUuid,
    InvoiceHash
);

/// Invoice-related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InvoiceError {
    /// A monetary or quantity value could not be parsed or represented as a [`Decimal`].
    #[error(transparent)]
    Decimal(#[from] crate::DecimalError),
    /// One or more field-level validation issues; see [`ValidationError::issues`].
    #[error(transparent)]
    Validation(#[from] ValidationError),
    /// The input (upper-cased) is not an ISO 3166-1 alpha-2 or alpha-3 country code.
    #[error("Invalid country code: {0}")]
    InvalidCountryCode(String),
    /// The input (upper-cased) is not an ISO 4217 currency code.
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),
    /// The input is not a valid `YYYY-MM-DDTHH:MM:SSZ` timestamp or `HH:MM:SS[Z]` time.
    #[error("Invalid invoice timestamp: {0}")]
    InvalidTimestamp(String),
    /// The input is not a valid `YYYY-MM-DD` calendar date.
    #[error("Invalid invoice date: {0}")]
    InvalidIssueDate(String),
    /// The seller party has no VAT registration number.
    #[error("Missing VAT ID for seller")]
    MissingVatForSeller,
    /// The buyer party has no identifier.
    #[error("Missing Buyer ID for buyer")]
    MissingBuyerId,
    /// A VAT ID was empty after trimming whitespace.
    #[error("Invalid VAT ID format")]
    InvalidVatFormat,
}

impl InvoiceError {
    /// Returns the coarse [`ErrorKind`](crate::ErrorKind) shared with [`crate::Error`] and the bindings.
    ///
    /// [`Validation`](Self::Validation) maps to `Validation`; every other variant maps to `InvalidInput`.
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
    /// Creates an error from a list of issues, preserving their order.
    pub fn new(issues: Vec<ValidationIssue>) -> Self {
        Self { issues }
    }

    /// Returns the individual issues, in the order they were found.
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
    /// For [`ValidationKind::Mismatch`] issues on computed amounts, the value the caller supplied.
    ///
    /// `None` for issues created with [`ValidationIssue::new`].
    pub fn supplied(&self) -> Option<Decimal> {
        self.supplied
    }
    /// For [`ValidationKind::Mismatch`] issues on computed amounts, the value the SDK calculated.
    ///
    /// `None` for issues created with [`ValidationIssue::new`].
    pub fn expected(&self) -> Option<Decimal> {
        self.expected
    }

    /// Creates an issue for `field`, optionally tied to a zero-based line item index.
    ///
    /// The supplied/expected amounts are left unset.
    pub fn new(field: InvoiceField, kind: ValidationKind, line_item_index: Option<usize>) -> Self {
        Self {
            field,
            kind,
            line_item_index,
            supplied: None,
            expected: None,
        }
    }

    /// Returns the invoice field the issue refers to.
    pub fn field(&self) -> InvoiceField {
        self.field
    }

    /// Returns what is wrong with the field.
    pub fn kind(&self) -> ValidationKind {
        self.kind
    }

    /// Returns the zero-based index of the offending line item, if the issue is line-specific.
    pub fn line_item_index(&self) -> Option<usize> {
        self.line_item_index
    }
}

#[non_exhaustive]
/// Field associated with a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InvoiceField {
    /// Invoice number (`cbc:ID`).
    Id,
    /// Invoice UUID (`cbc:UUID`).
    Uuid,
    /// Issue date and time.
    IssueDateTime,
    /// Invoice or tax currency.
    Currency,
    /// Previous invoice hash (PIH).
    PreviousInvoiceHash,
    /// Invoice counter value (ICV).
    InvoiceCounter,
    /// Seller party.
    Seller,
    /// Buyer party.
    Buyer,
    /// The invoice's line items as a whole (for example, none were supplied).
    LineItems,
    /// Payment means code.
    PaymentMeansCode,
    /// Payment instruction note.
    PaymentInstruction,
    /// ID of the original invoice referenced by a credit or debit note.
    OriginalInvoiceId,
    /// VAT category of a line, allowance/charge or VAT breakdown.
    VatCategory,
    /// Line item ID.
    LineItemId,
    /// ID of a document reference on a line item.
    LineItemDocumentReferenceId,
    /// Line item description (item name).
    LineItemDescription,
    /// Line item unit of measure code.
    LineItemUnitCode,
    /// Line item quantity.
    LineItemQuantity,
    /// Line item unit price.
    LineItemUnitPrice,
    /// Line item net total amount.
    LineItemTotalAmount,
    /// Line item VAT rate.
    LineItemVatRate,
    /// Line item VAT amount.
    LineItemVatAmount,
    /// Document-level allowance (discount).
    InvoiceLevelDiscount,
    /// Document-level charge.
    InvoiceLevelCharge,
}

#[non_exhaustive]
/// Classification of validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationKind {
    /// A required value was not supplied.
    Missing,
    /// A value was supplied but is empty or blank.
    Empty,
    /// A value does not have the expected format.
    InvalidFormat,
    /// A numeric value is outside its allowed range (for example, negative).
    OutOfRange,
    /// A value disagrees with the value calculated from other fields.
    Mismatch,
}

/// Country code wrapper with ISO validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct CountryCode(String);

impl CountryCode {
    /// Parses an ISO 3166-1 alpha-2 or alpha-3 code (case-insensitive, trimmed) and
    /// normalizes it to upper-case alpha-3 (for example `"sa"` becomes `"SAU"`).
    ///
    /// # Errors
    /// Returns [`InvoiceError::InvalidCountryCode`] if the input is not a known
    /// two- or three-letter ISO country code.
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

    /// Returns the normalized upper-case ISO alpha-3 code.
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

impl std::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
    /// Parses an ISO 4217 currency code (case-insensitive, trimmed) and stores it upper-cased.
    ///
    /// # Errors
    /// Returns [`InvoiceError::InvalidCurrencyCode`] if the input is not a known ISO 4217 code.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_uppercase();
        IsoCurrency::from_code(&value)
            .ok_or_else(|| InvoiceError::InvalidCurrencyCode(value.clone()))?;
        Ok(Self(value))
    }

    /// Returns the upper-case ISO 4217 code, for example `"SAR"`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
    /// Parses a trimmed `YYYY-MM-DDTHH:MM:SSZ` UTC timestamp and stores it in canonical form.
    ///
    /// # Errors
    /// Returns [`InvoiceError::InvalidTimestamp`] if the input does not match that
    /// format or is not a real date and time.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_string();
        let parsed = NaiveDateTime::parse_from_str(&value, TIMESTAMP_FORMAT)
            .map_err(|_| InvoiceError::InvalidTimestamp(value.clone()))?;
        Ok(Self(parsed.format(TIMESTAMP_FORMAT).to_string()))
    }

    /// Returns the timestamp as `YYYY-MM-DDTHH:MM:SSZ`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn date_str(&self) -> &str {
        &self.0[..10]
    }

    /// Returns the `HH:MM:SSZ` clock part.
    pub(crate) fn time_str(&self) -> &str {
        &self.0[11..]
    }

    /// Returns the timestamp without the trailing `Z` (`YYYY-MM-DDTHH:MM:SS`),
    /// the form used by the XAdES `SigningTime` element.
    pub(crate) fn xades_str(&self) -> &str {
        &self.0[..19]
    }

    /// Parses a XAdES `SigningTime` value (`YYYY-MM-DDTHH:MM:SS`, no zone suffix).
    pub(crate) fn parse_xades(value: &str) -> Result<Self> {
        let value = value.trim();
        let parsed = NaiveDateTime::parse_from_str(value, XADES_FORMAT)
            .map_err(|_| InvoiceError::InvalidTimestamp(value.to_owned()))?;
        Ok(Self::from_naive_utc(parsed))
    }

    /// Wraps a UTC wall-clock value, truncating sub-second precision.
    pub(crate) fn from_naive_utc(value: NaiveDateTime) -> Self {
        Self(value.format(TIMESTAMP_FORMAT).to_string())
    }
}

impl AsRef<str> for InvoiceTimestamp {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
    /// Parses a trimmed `YYYY-MM-DD` calendar date and stores it in canonical form.
    ///
    /// # Errors
    /// Returns [`InvoiceError::InvalidIssueDate`] if the input does not match that
    /// format or is not a real calendar date.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_string();
        let parsed = NaiveDate::parse_from_str(&value, DATE_FORMAT)
            .map_err(|_| InvoiceError::InvalidIssueDate(value.clone()))?;
        Ok(Self(parsed.format(DATE_FORMAT).to_string()))
    }

    /// Returns the date as `YYYY-MM-DD`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for InvoiceDate {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
    /// Parses a trimmed `HH:MM:SS` time with an optional trailing `Z` (UTC), which is preserved.
    ///
    /// Exactly eight clock characters are required; `24:00:00` is also accepted.
    ///
    /// # Errors
    /// Returns [`InvoiceError::InvalidTimestamp`] if the clock part is not a valid
    /// two-digit `HH:MM:SS` time.
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
    /// Returns the time as `HH:MM:SS` or `HH:MM:SSZ`, matching the parsed input.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for InvoiceTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for InvoiceTime {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for InvoiceTime {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        InvoiceTime::parse(s)
    }
}

impl TryFrom<String> for InvoiceTime {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        InvoiceTime::parse(value)
    }
}

impl TryFrom<&str> for InvoiceTime {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        InvoiceTime::parse(value)
    }
}

/// Format used by [`InvoiceTimestamp::parse`] and its chrono conversion.
const TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";
/// Format of the XAdES `SigningTime` element (no zone suffix).
const XADES_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
/// Format used by [`InvoiceDate::parse`] and its chrono conversion.
const DATE_FORMAT: &str = "%Y-%m-%d";

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for InvoiceTimestamp {
    /// Format as `YYYY-MM-DDTHH:MM:SSZ`, truncating sub-second precision.
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::from_naive_utc(value.naive_utc())
    }
}

#[cfg(feature = "chrono")]
impl From<NaiveDate> for InvoiceDate {
    /// Format as `YYYY-MM-DD`.
    fn from(value: NaiveDate) -> Self {
        Self(value.format(DATE_FORMAT).to_string())
    }
}

/// Builds the validation error reported for a blank identifier.
fn empty_field_error(field: InvoiceField) -> InvoiceError {
    InvoiceError::Validation(ValidationError::new(vec![ValidationIssue::new(
        field,
        ValidationKind::Empty,
        None,
    )]))
}

/// Invoice UUID (`cbc:UUID`).
///
/// Accepts any input that is non-empty after trimming surrounding whitespace;
/// the stored value is trimmed. Pass it to [`InvoiceBuilder::uuid`] and
/// [`OriginalInvoiceRef::with_uuid`].
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::InvoiceUuid;
///
/// let uuid = InvoiceUuid::parse("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")?;
/// assert_eq!(uuid.as_str(), "8e6000cf-1a98-4174-b3e7-b5d5954bc10d");
/// assert!(InvoiceUuid::parse("  ").is_err());
/// # Ok::<(), fatoora_core::invoice::InvoiceError>(())
/// ```
///
/// # Errors
/// [`InvoiceUuid::parse`] returns [`InvoiceError::Validation`] with an
/// [`InvoiceField::Uuid`] / [`ValidationKind::Empty`] issue for blank input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct InvoiceUuid(String);

impl InvoiceUuid {
    /// Trims the input and wraps it; the UUID syntax itself is not checked.
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] with an [`InvoiceField::Uuid`] /
    /// [`ValidationKind::Empty`] issue if the input is blank.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_string();
        if value.is_empty() {
            return Err(empty_field_error(InvoiceField::Uuid));
        }
        Ok(Self(value))
    }

    /// Returns the trimmed UUID string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for InvoiceUuid {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for InvoiceUuid {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        InvoiceUuid::parse(s)
    }
}

impl TryFrom<String> for InvoiceUuid {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        InvoiceUuid::parse(value)
    }
}

impl TryFrom<&str> for InvoiceUuid {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        InvoiceUuid::parse(value)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for InvoiceUuid {
    /// Format as the lowercase hyphenated form, e.g. `8e6000cf-1a98-4174-b3e7-b5d5954bc10d`.
    fn from(value: uuid::Uuid) -> Self {
        Self(value.hyphenated().to_string())
    }
}

/// Previous invoice hash (the `PIH` additional document reference).
///
/// Accepts any input that is non-empty after trimming surrounding whitespace;
/// the stored value is trimmed. Pass it to [`InvoiceBuilder::previous_invoice_hash`].
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::InvoiceHash;
///
/// let hash = InvoiceHash::parse("NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==")?;
/// assert!(hash.as_str().ends_with("=="));
/// assert!(InvoiceHash::parse("").is_err());
/// # Ok::<(), fatoora_core::invoice::InvoiceError>(())
/// ```
///
/// # Errors
/// [`InvoiceHash::parse`] returns [`InvoiceError::Validation`] with an
/// [`InvoiceField::PreviousInvoiceHash`] / [`ValidationKind::Empty`] issue for
/// blank input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct InvoiceHash(String);

impl InvoiceHash {
    /// Trims the input and wraps it; the Base64 encoding is not checked.
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] with an [`InvoiceField::PreviousInvoiceHash`] /
    /// [`ValidationKind::Empty`] issue if the input is blank.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let value = s.into().trim().to_string();
        if value.is_empty() {
            return Err(empty_field_error(InvoiceField::PreviousInvoiceHash));
        }
        Ok(Self(value))
    }

    /// Returns the trimmed hash string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for InvoiceHash {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for InvoiceHash {
    type Err = InvoiceError;
    fn from_str(s: &str) -> Result<Self> {
        InvoiceHash::parse(s)
    }
}

impl TryFrom<String> for InvoiceHash {
    type Error = InvoiceError;
    fn try_from(value: String) -> Result<Self> {
        InvoiceHash::parse(value)
    }
}

impl TryFrom<&str> for InvoiceHash {
    type Error = InvoiceError;
    fn try_from(value: &str) -> Result<Self> {
        InvoiceHash::parse(value)
    }
}

/// Compute the ZATCA invoice hash of an XML invoice.
///
/// The hash is the base64-encoded SHA-256 digest of the invoice after ZATCA
/// canonicalisation (the `UBLExtensions`, `cac:Signature` and QR document
/// reference removed, then canonicalised). It is the value signed into the invoice and the value the
/// next invoice in the chain carries as its previous invoice hash (`PIH`).
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::hash_xml;
///
/// let xml = include_str!("../tests/fixtures/invoices/sample-simplified-invoice.xml");
/// let hash = hash_xml(xml)?;
/// assert_eq!(hash.as_str().len(), 44);
/// # Ok::<(), fatoora_core::invoice::sign::SigningError>(())
/// ```
///
/// # Errors
/// Returns [`sign::SigningError::Xml`] if the XML cannot be parsed or
/// canonicalised.
pub fn hash_xml(xml: &str) -> std::result::Result<InvoiceHash, sign::SigningError> {
    sign::invoice_hash_base64_from_xml(xml).map(InvoiceHash)
}

/// Postal address for parties.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    /// Province or region, serialized as UBL `cbc:CountrySubentity`.
    #[serde(default)]
    pub province: Option<String>,
    /// Country; serialized as its ISO alpha-2 code in `cac:Country/cbc:IdentificationCode`.
    pub country_code: CountryCode,
    /// City name (`cbc:CityName`); omitted from XML when empty.
    pub city: String,
    /// Street name (`cbc:StreetName`); omitted from XML when empty.
    pub street: String,
    /// Additional street name (`cbc:AdditionalStreetName`).
    pub additional_street: Option<String>,
    /// Building number (`cbc:BuildingNumber`); omitted from XML when empty.
    pub building_number: String,
    /// Additional (plot) number, serialized as `cbc:PlotIdentification`.
    pub additional_number: Option<String>,
    /// Postal code (`cbc:PostalZone`); omitted from XML when empty.
    pub postal_code: String, //fix 5 digits if country is KSA
    /// City district, serialized as UBL `cbc:CitySubdivisionName`.
    pub district: Option<String>,
}

impl Address {
    /// Returns the province or region, if set.
    pub fn province(&self) -> Option<&str> {
        self.province.as_deref()
    }
    /// Returns the country code.
    pub fn country_code(&self) -> &CountryCode {
        &self.country_code
    }

    /// Returns the city name.
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns the street name.
    pub fn street(&self) -> &str {
        &self.street
    }

    /// Returns the additional street name, if set.
    pub fn additional_street(&self) -> Option<&str> {
        self.additional_street.as_deref()
    }

    /// Returns the building number.
    pub fn building_number(&self) -> &str {
        &self.building_number
    }

    /// Returns the additional (plot) number, if set.
    pub fn additional_number(&self) -> Option<&str> {
        self.additional_number.as_deref()
    }

    /// Returns the postal code.
    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    /// Returns the city district, if set.
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
    /// Trims the input and wraps it. Only emptiness is checked; the KSA 15-digit
    /// format is enforced later by XML business-rule validation.
    ///
    /// # Errors
    /// Returns [`InvoiceError::InvalidVatFormat`] if the input is blank.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let s = s.into().trim().to_string();
        if s.is_empty() {
            return Err(InvoiceError::InvalidVatFormat);
        }
        // TODO: tighten validation (e.g., KSA = 15 digits)
        Ok(VatId(s))
    }
    /// Returns the trimmed VAT registration number.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for VatId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
    /// Creates an identifier without a scheme ID.
    pub fn new<S: Into<String>>(value: S) -> Self {
        OtherId {
            value: value.into(),
            scheme_id: None,
        }
    }

    /// Creates an identifier with a scheme ID such as `"CRN"` or `"NAT"` (`schemeID` attribute).
    pub fn with_scheme<V: Into<String>, S: Into<String>>(value: V, scheme_id: S) -> Self {
        OtherId {
            value: value.into(),
            scheme_id: Some(scheme_id.into()),
        }
    }

    /// Returns the identifier value (without the scheme).
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the scheme ID, if one was given.
    pub fn scheme_id(&self) -> Option<&str> {
        self.scheme_id.as_deref()
    }
}
impl AsRef<str> for OtherId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Formats only the identifier value; the scheme ID is omitted (see [`OtherId::scheme_id`]).
impl std::fmt::Display for OtherId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
    /// Creates a note with no `languageID` attribute.
    ///
    /// [`language`](Self::language) still reports `"en"`, but [`language_id`](Self::language_id)
    /// returns `None` and no attribute is emitted in XML.
    pub fn without_language(text: impl Into<String>) -> Self {
        Self {
            language_omitted: true,
            language: "en".into(),
            text: text.into(),
        }
    }
    /// Returns the language emitted as the `languageID` attribute, or `None` for notes
    /// created with [`without_language`](Self::without_language).
    pub fn language_id(&self) -> Option<&str> {
        (!self.language_omitted).then_some(self.language.as_str())
    }

    /// Creates a note with an explicit language code (for example `"en"` or `"ar"`).
    pub fn new(language: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            language_omitted: false,
            language: language.into(),
            text: text.into(),
        }
    }

    /// Returns the note language, defaulting to `"en"` when none was given.
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Returns the note text.
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
/// use fatoora_core::invoice::{Party, SellerRole, Address, OtherId, VatId};
/// use fatoora_core::invoice::CountryCode;
///
/// let seller = Party::<SellerRole>::new(
///     "Acme Inc",
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
///     VatId::parse("399999999900003")?,
///     Some(OtherId::with_scheme("7003339333", "CRN")),
/// );
/// assert_eq!(seller.vat_id().map(VatId::as_str), Some("399999999900003"));
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

/// A seller (supplier) party; its constructor requires an address and a VAT ID.
pub type Seller = Party<SellerRole>;
/// A buyer (customer) party; its constructor accepts an optional address and VAT ID.
pub type Buyer = Party<BuyerRole>;

impl Party<SellerRole> {
    /// Create a seller party; the address and VAT ID are required.
    ///
    /// Parse the VAT ID with [`VatId::parse`] (or `str::parse`) first.
    pub fn new(
        name: impl Into<String>,
        address: Address,
        vat_id: VatId,
        other_id: Option<OtherId>,
    ) -> Self {
        Party {
            _marker: PhantomData,
            name: name.into(),
            address: Some(address),
            vat_id: Some(vat_id),
            identifiers: other_id.into_iter().collect(),
        }
    }
}

impl Party<BuyerRole> {
    /// Create a buyer party; the address, VAT ID and other identifier are optional.
    ///
    /// Identification requirements depend on the invoice type and are checked by
    /// XML business-rule validation, not here.
    pub fn new(
        name: impl Into<String>,
        address: impl Into<Option<Address>>,
        vat_id: Option<VatId>,
        other_id: Option<OtherId>,
    ) -> Self {
        Party {
            _marker: PhantomData,
            name: name.into(),
            address: address.into(),
            vat_id,
            identifiers: other_id.into_iter().collect(),
        }
    }
}

impl<R: PartyRole> Party<R> {
    /// Returns the registration name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the postal address, if any.
    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    /// Returns the VAT registration number, if any.
    pub fn vat_id(&self) -> Option<&VatId> {
        self.vat_id.as_ref()
    }

    /// Append an identification, preserving its scheme and order.
    pub fn identifier(mut self, value: OtherId) -> Self {
        self.identifiers.push(value);
        self
    }
    /// Returns all additional identifications, in insertion order.
    pub fn identifiers(&self) -> &[OtherId] {
        &self.identifiers
    }
    /// Returns the first additional identification, if any.
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
    /// Simplified (B2C) invoice, reported to ZATCA after issue.
    Simplified,
    /// Standard (B2B) invoice, cleared by ZATCA before issue.
    Standard,
}

/// Reference to an original invoice for credit/debit notes.
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::{InvoiceDate, InvoiceUuid, OriginalInvoiceRef};
///
/// let original = OriginalInvoiceRef::new("INV-ORIG")
///     .with_uuid(InvoiceUuid::parse("uuid-orig")?)
///     .with_issue_date("2024-01-01".parse()?);
/// assert_eq!(original.id(), "INV-ORIG");
/// assert_eq!(original.uuid().map(InvoiceUuid::as_str), Some("uuid-orig"));
/// assert_eq!(original.issue_date().map(InvoiceDate::as_str), Some("2024-01-01"));
/// # Ok::<(), fatoora_core::invoice::InvoiceError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OriginalInvoiceRef {
    id: String,
    uuid: Option<InvoiceUuid>,
    issue_date: Option<InvoiceDate>,
    #[serde(default)]
    issue_time: Option<InvoiceTime>,
    #[serde(default)]
    document_type_code: Option<String>,
}

impl OriginalInvoiceRef {
    /// Sets the referenced document's issue time (`cbc:IssueTime`).
    pub fn with_issue_time(mut self, time: InvoiceTime) -> Self {
        self.issue_time = Some(time);
        self
    }
    /// Sets the referenced document's type code (`cbc:DocumentTypeCode`).
    pub fn with_document_type_code(mut self, code: impl Into<String>) -> Self {
        self.document_type_code = Some(code.into());
        self
    }
    /// Returns the referenced document's issue time, if set.
    pub fn issue_time(&self) -> Option<&InvoiceTime> {
        self.issue_time.as_ref()
    }
    /// Returns the referenced document's type code, if set.
    pub fn document_type_code(&self) -> Option<&str> {
        self.document_type_code.as_deref()
    }

    /// Creates a reference to the document with the given ID (`cbc:ID`); all other fields are unset.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            uuid: None,
            issue_date: None,
            issue_time: None,
            document_type_code: None,
        }
    }

    /// Sets the referenced document's UUID (`cbc:UUID`).
    pub fn with_uuid(mut self, uuid: InvoiceUuid) -> Self {
        self.uuid = Some(uuid);
        self
    }

    /// Sets the referenced document's issue date (`cbc:IssueDate`).
    ///
    /// Parse a `YYYY-MM-DD` string with [`InvoiceDate::parse`], `str::parse` or
    /// `InvoiceDate::try_from`.
    pub fn with_issue_date(mut self, issue_date: InvoiceDate) -> Self {
        self.issue_date = Some(issue_date);
        self
    }

    /// Returns the referenced document's ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the referenced document's UUID, if set.
    pub fn uuid(&self) -> Option<&InvoiceUuid> {
        self.uuid.as_ref()
    }

    /// Returns the referenced document's issue date, if set.
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
    /// Tax invoice (UNTDID 1001 code `388`).
    Tax(InvoiceSubType),
    /// Prepayment invoice (code `386`).
    Prepayment(InvoiceSubType),
    /// Credit note (code `381`); supply the original-invoice reference and reason through the builder.
    CreditNote(InvoiceSubType),
    /// Debit note (code `383`); supply the original-invoice reference and reason through the builder.
    DebitNote(InvoiceSubType),
}

impl InvoiceType {
    /// Returns `true` if the subtype is [`InvoiceSubType::Simplified`].
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
    /// Exempt from VAT (UNCL 5305 code `E`).
    Exempt,
    /// Standard rate (code `S`).
    Standard,
    /// Zero-rated (code `Z`).
    Zero,
    /// Not subject to VAT / out of scope (code `O`).
    OutOfScope,
}
/// Independent optional exemption code and description for a tax category.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxExemption {
    /// Exemption reason code (`cbc:TaxExemptionReasonCode`), for example `VATEX-SA-29`.
    pub reason_code: Option<String>,
    /// Free-text exemption reason (`cbc:TaxExemptionReason`).
    pub reason: Option<String>,
}

/// A payment instruction. Absence of the collection is distinct from an invalid empty code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMeans {
    /// UNTDID 4461 payment means code (`cbc:PaymentMeansCode`), for example `10` for cash.
    pub payment_means_type_code: String,
    /// Instruction notes (`cbc:InstructionNote`), such as the reason for a credit or debit note.
    pub instruction_notes: Vec<String>,
    /// Payee account details (`cac:PayeeFinancialAccount`).
    pub payee_financial_account: Option<PaymentAccount>,
}
impl PaymentMeans {
    /// Creates payment means with the given code and no notes or account.
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            payment_means_type_code: code.into(),
            instruction_notes: Vec::new(),
            payee_financial_account: None,
        }
    }
    /// Appends an instruction note.
    pub fn instruction_note(mut self, value: impl Into<String>) -> Self {
        self.instruction_notes.push(value.into());
        self
    }
    /// Sets the payee financial account.
    pub fn payee_financial_account(mut self, value: PaymentAccount) -> Self {
        self.payee_financial_account = Some(value);
        self
    }
}
/// Payee financial account (`cac:PayeeFinancialAccount`) of a [`PaymentMeans`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentAccount {
    /// Account identifier such as an IBAN (`cbc:ID`).
    pub payment_account_identifier: Option<String>,
    /// Payment terms, serialized as `cbc:PaymentNote`.
    pub payment_terms: Option<String>,
}

/// Supply date or supply period. Business-rule validation checks date ordering.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delivery {
    /// Supply date, or start of the supply period (`cbc:ActualDeliveryDate`).
    pub actual_date: Option<InvoiceDate>,
    /// End of the supply period (`cbc:LatestDeliveryDate`).
    pub latest_date: Option<InvoiceDate>,
}

fn one_tax_category() -> usize {
    1
}

/// Whether an adjustment is an allowance (discount) or a charge (surcharge).
///
/// Maps to the UBL `cbc:ChargeIndicator` element: `false` for
/// [`Allowance`](Self::Allowance), `true` for [`Charge`](Self::Charge).
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::AdjustmentKind;
///
/// let kind = AdjustmentKind::from_charge_indicator(true);
/// assert_eq!(kind, AdjustmentKind::Charge);
/// assert!(kind.is_charge());
/// assert!(!AdjustmentKind::Allowance.charge_indicator());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjustmentKind {
    /// An allowance, which reduces the amount it applies to (`cbc:ChargeIndicator` `false`).
    Allowance,
    /// A charge, which increases the amount it applies to (`cbc:ChargeIndicator` `true`).
    Charge,
}

impl AdjustmentKind {
    /// Converts a UBL `cbc:ChargeIndicator` value: `true` is a charge, `false` an allowance.
    pub const fn from_charge_indicator(charge: bool) -> Self {
        if charge {
            Self::Charge
        } else {
            Self::Allowance
        }
    }

    /// Returns the UBL `cbc:ChargeIndicator` value: `true` for a charge, `false` for an allowance.
    pub const fn charge_indicator(self) -> bool {
        matches!(self, Self::Charge)
    }

    /// Returns `true` for [`AdjustmentKind::Charge`].
    pub const fn is_charge(self) -> bool {
        matches!(self, Self::Charge)
    }

    /// Returns `true` for [`AdjustmentKind::Allowance`].
    pub const fn is_allowance(self) -> bool {
        matches!(self, Self::Allowance)
    }
}

/// One document allowance or charge, retaining its own VAT category and reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowanceCharge {
    /// Preserve an absent VAT rate for an out-of-scope adjustment.
    #[serde(default)]
    pub omitted_rate: bool,
    /// Base amount the percentage applies to (`cbc:BaseAmount`).
    #[serde(default)]
    pub base_amount: Option<Decimal>,
    /// Percentage applied to the base amount (`cbc:MultiplierFactorNumeric`).
    #[serde(default)]
    pub percentage: Option<Decimal>,
    /// Identical repeated VAT classifications, as used by the bundled SDK fixtures.
    /// Must be in `1..=Self::MAX_TAX_CATEGORY_COUNT` (an implementation resource limit).
    #[serde(default = "one_tax_category")]
    pub tax_category_count: usize,
    /// Whether this is an allowance or a charge (`cbc:ChargeIndicator`).
    pub kind: AdjustmentKind,
    /// Allowance or charge amount, excluding VAT (`cbc:Amount`).
    pub amount: Decimal,
    /// Free-text reason (`cbc:AllowanceChargeReason`).
    pub reason: Option<String>,
    /// Reason code (`cbc:AllowanceChargeReasonCode`).
    pub reason_code: Option<String>,
    /// VAT category of the allowance or charge.
    pub category: VatCategory,
    /// VAT rate percentage for the category.
    pub rate: Decimal,
    /// Exemption code and reason for exempt, zero-rated or out-of-scope categories.
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
    /// Returns `false` if the line was built without an explicit VAT rate
    /// (out-of-scope lines), in which case no `cbc:Percent` is emitted.
    pub fn has_vat_rate(&self) -> bool {
        !self.omitted_vat_rate
    }
    /// Returns `false` if the line-level `cac:TaxTotal` is omitted from XML.
    pub fn has_tax_total(&self) -> bool {
        !self.omitted_tax_amount
    }
    /// Returns `false` if the VAT-inclusive line amount (`cbc:RoundingAmount`) is omitted from XML.
    pub fn has_gross_amount(&self) -> bool {
        !self.omitted_gross_amount
    }
    /// Appends a document reference, such as a prepayment invoice settled by this line.
    pub fn document_reference(mut self, reference: OriginalInvoiceRef) -> Self {
        self.document_references.push(reference);
        self
    }
    /// Appends a supplied line-level tax subtotal (used for prepayment adjustment lines).
    pub fn tax_subtotal(mut self, subtotal: VatBreakdown) -> Self {
        self.tax_subtotals.push(subtotal);
        self
    }
    /// Returns the line's document references, in insertion order.
    pub fn document_references(&self) -> &[OriginalInvoiceRef] {
        &self.document_references
    }
    /// A reference to an advance invoice identifies a prepayment adjustment line.
    pub fn is_prepayment_adjustment(&self) -> bool {
        self.document_references
            .iter()
            .any(|r| r.document_type_code().is_some_and(|v| v.trim() == "386"))
    }

    /// Returns the supplied line-level tax subtotals, in insertion order.
    pub fn tax_subtotals(&self) -> &[VatBreakdown] {
        &self.tax_subtotals
    }

    /// Sets the line ID (`cbc:ID`); lines without one are numbered sequentially in XML.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    /// Returns the explicit line ID, if set.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    /// Replaces the line's VAT exemption code and reason.
    pub fn with_exemption(mut self, exemption: TaxExemption) -> Self {
        self.exemption = exemption;
        self
    }
    /// Returns the line's VAT exemption code and reason.
    pub fn exemption(&self) -> &TaxExemption {
        &self.exemption
    }

    /// Creates a line, calculating the net amount (`quantity * unit_price`) and VAT amount.
    ///
    /// Shorthand for [`LineItem::builder`] with the corresponding setters.
    ///
    /// # Errors
    /// Returns the errors of [`LineItemBuilder::build`], for example
    /// [`InvoiceError::Validation`] for a blank description or unit code, a negative
    /// quantity or price, or a rate outside `0..=100` (or non-zero for a non-standard category).
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
    /// Creates a line with a supplied net total, which is checked against the calculated one.
    ///
    /// # Errors
    /// Returns the errors of [`LineItem::new`], plus [`InvoiceError::Validation`] with a
    /// [`ValidationKind::Mismatch`] issue on [`InvoiceField::LineItemTotalAmount`] if
    /// `total_amount` has more than two decimal places or differs from
    /// `quantity * unit_price` by more than `0.01`.
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
    /// Creates a line with supplied net total and VAT amount, both checked against calculated values.
    ///
    /// The VAT amount is used as given rather than recalculated.
    ///
    /// # Errors
    /// Returns the errors of [`LineItem::from_totals`], plus [`InvoiceError::Validation`] with a
    /// [`ValidationKind::OutOfRange`] issue on [`InvoiceField::LineItemVatAmount`] if
    /// `vat_amount` is negative or has more than two decimal places.
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
    /// Returns the item name (same as [`LineItem::item_name`]).
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Returns the invoiced quantity.
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
    /// Returns the quantity unit code; empty if none was set.
    pub fn unit_code(&self) -> &str {
        &self.unit_code
    }
    /// Returns the net unit price (same as [`LineItem::item_net_price`]).
    pub fn unit_price(&self) -> Decimal {
        self.unit_price
    }
    /// Returns the line net amount, excluding VAT.
    pub fn total_amount(&self) -> Decimal {
        self.total_amount
    }
    /// Returns the VAT rate percentage (zero when the rate was omitted).
    pub fn vat_rate(&self) -> Decimal {
        self.vat_rate
    }
    /// Returns the line VAT amount.
    pub fn vat_amount(&self) -> Decimal {
        self.vat_amount
    }
    /// Returns the line VAT category.
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
    uuid: InvoiceUuid,
    issue_datetime: InvoiceTimestamp,
    currency: CurrencyCode,
    #[serde(default)]
    tax_currency: Option<CurrencyCode>,
    #[serde(default)]
    tax_currency_amount: Option<Decimal>,
    previous_invoice_hash: InvoiceHash,
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
    /// Returns references to original invoices (`cac:BillingReference`), in order.
    pub fn billing_references(&self) -> &[OriginalInvoiceRef] {
        &self.billing_references
    }
    /// Returns all payment means, in order.
    pub fn payment_means(&self) -> &[PaymentMeans] {
        &self.payment_means
    }
    /// Returns all invoice notes, in order.
    pub fn notes(&self) -> &[InvoiceNote] {
        &self.notes
    }
    /// Returns the purchase order reference, if set.
    pub fn purchase_order_id(&self) -> Option<&str> {
        self.purchase_order_id.as_deref()
    }
    /// Returns the contract reference, if set.
    pub fn contract_id(&self) -> Option<&str> {
        self.contract_id.as_deref()
    }

    /// Returns the first instruction note of the first payment means, if any.
    pub fn payment_instruction(&self) -> Option<&str> {
        self.payment_means
            .first()
            .and_then(|p| p.instruction_notes.first().map(String::as_str))
    }
    /// Returns the tax currency, if it differs from the document currency and was set.
    pub fn tax_currency(&self) -> Option<&CurrencyCode> {
        self.tax_currency.as_ref()
    }
    /// Returns the total VAT amount expressed in the tax currency, if set.
    pub fn tax_currency_amount(&self) -> Option<Decimal> {
        self.tax_currency_amount
    }
    /// Returns the individual document-level allowances and charges, in order.
    pub fn adjustments(&self) -> &[AllowanceCharge] {
        &self.adjustments
    }
    /// Returns the supply date or period, if set.
    pub fn delivery(&self) -> Option<&Delivery> {
        self.delivery.as_ref()
    }
    /// Returns the invoice type and subtype.
    pub fn invoice_type(&self) -> &InvoiceType {
        &self.invoice_type
    }

    /// Returns the invoice number (`cbc:ID`).
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the invoice UUID (`cbc:UUID`).
    pub fn uuid(&self) -> &InvoiceUuid {
        &self.uuid
    }

    /// Returns the issue time (`cbc:IssueTime`) in its original clock representation;
    /// a missing suffix denotes AST, `Z` denotes UTC.
    ///
    /// Built invoices report the clock part of [`issue_datetime`](Self::issue_datetime);
    /// parsed invoices keep the value from the XML.
    pub fn issue_time(&self) -> InvoiceTime {
        self.issue_time_override
            .clone()
            .unwrap_or_else(|| InvoiceTime(self.issue_datetime.time_str().to_owned()))
    }
    /// Returns the issue date (`cbc:IssueDate`), preserving the original value for imported documents.
    pub fn issue_date(&self) -> InvoiceDate {
        self.issue_date_override
            .clone()
            .unwrap_or_else(|| InvoiceDate(self.issue_datetime.date_str().to_owned()))
    }

    /// Returns the issue timestamp in UTC `YYYY-MM-DDTHH:MM:SSZ` form.
    pub fn issue_datetime(&self) -> &InvoiceTimestamp {
        &self.issue_datetime
    }

    /// Returns the document currency.
    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

    /// Returns the previous invoice hash (PIH), Base64-encoded.
    pub fn previous_invoice_hash(&self) -> &InvoiceHash {
        &self.previous_invoice_hash
    }

    /// Returns the invoice counter value (ICV).
    pub fn invoice_counter(&self) -> u64 {
        self.invoice_counter
    }

    /// Returns the seller party.
    pub fn seller(&self) -> &Seller {
        &self.seller
    }

    /// Returns the buyer party, if any (it may be absent on simplified invoices).
    pub fn buyer(&self) -> Option<&Buyer> {
        self.buyer.as_ref()
    }

    /// Returns the line items, in order.
    pub fn line_items(&self) -> &[LineItem] {
        &self.line_items
    }

    /// Returns the code of the first payment means, or `""` if there are none.
    pub fn payment_means_code(&self) -> &str {
        self.payment_means
            .first()
            .map_or("", |p| p.payment_means_type_code.as_str())
    }

    /// Returns the document VAT category, used for the aggregate document-level
    /// discount and charge.
    pub fn vat_category(&self) -> VatCategory {
        self.vat_category
    }

    /// Returns the invoice transaction flags (third party, nominal, export, summary, self-billed).
    pub fn flags(&self) -> InvoiceFlags {
        self.flags
    }

    /// Returns `true` if [`InvoiceFlags::THIRD_PARTY`] is set.
    pub fn is_third_party(&self) -> bool {
        self.flags.contains(InvoiceFlags::THIRD_PARTY)
    }

    /// Returns `true` if [`InvoiceFlags::NOMINAL`] is set.
    pub fn is_nominal(&self) -> bool {
        self.flags.contains(InvoiceFlags::NOMINAL)
    }

    /// Returns `true` if [`InvoiceFlags::EXPORT`] is set.
    pub fn is_export(&self) -> bool {
        self.flags.contains(InvoiceFlags::EXPORT)
    }

    /// Returns `true` if [`InvoiceFlags::SUMMARY`] is set.
    pub fn is_summary(&self) -> bool {
        self.flags.contains(InvoiceFlags::SUMMARY)
    }

    /// Returns `true` if [`InvoiceFlags::SELF_BILLED`] is set.
    pub fn is_self_billed(&self) -> bool {
        self.flags.contains(InvoiceFlags::SELF_BILLED)
    }

    /// Returns the total of all document-level charges.
    pub fn invoice_level_charge(&self) -> Decimal {
        self.invoice_level_charge
    }

    /// Returns the total of all document-level allowances (discounts).
    pub fn invoice_level_discount(&self) -> Decimal {
        self.invoice_level_discount
    }

    /// Returns the document-level allowance reason: the explicitly set reason, or else the
    /// reason shared by every document-level adjustment (`None` if they differ).
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
}
/// Former name of [`InvoiceTotals`].
#[deprecated(since = "0.6.0", note = "use `InvoiceTotals` instead")]
pub type InvoiceTotalsData = InvoiceTotals;

/// Computed invoice totals.
///
/// # Examples
/// ```rust,ignore
/// use fatoora_core::invoice::InvoiceTotals;
///
/// let totals: InvoiceTotals = unimplemented!();
/// let _ = totals.tax_inclusive_amount();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceTotals {
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
    /// Returns `false` if the VAT rate (`cbc:Percent`) is omitted from XML.
    pub fn has_vat_rate(&self) -> bool {
        !self.omitted_rate
    }
    /// Marks the subtotal so its VAT rate is omitted from XML.
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] with an [`InvoiceField::VatCategory`] /
    /// [`ValidationKind::OutOfRange`] issue unless the category is
    /// [`VatCategory::OutOfScope`] and the rate is zero.
    pub fn without_vat_rate(mut self) -> Result<Self> {
        if self.category != VatCategory::OutOfScope || self.rate != Decimal::ZERO {
            return Err(ValidationError::new(vec![ValidationIssue::new(
                InvoiceField::VatCategory,
                ValidationKind::OutOfRange,
                None,
            )])
            .into());
        }
        self.omitted_rate = true;
        Ok(self)
    }
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
    ///
    /// # Errors
    /// Returns [`InvoiceError::Validation`] with an [`InvoiceField::VatCategory`] /
    /// [`ValidationKind::OutOfRange`] issue if the rate is outside `0..=100`, any value
    /// is negative, or any value has more than two decimal places.
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
    /// Replaces the subtotal's VAT exemption code and reason.
    pub fn with_exemption(mut self, exemption: TaxExemption) -> Self {
        self.exemption = exemption;
        self
    }

    /// Returns the VAT exemption code and reason.
    pub fn exemption(&self) -> &TaxExemption {
        &self.exemption
    }
    /// Returns the VAT category.
    pub fn category(&self) -> VatCategory {
        self.category
    }
    /// Returns the VAT rate percentage.
    pub fn rate(&self) -> Decimal {
        self.rate
    }
    /// Returns the taxable amount for this category and rate.
    pub fn taxable_amount(&self) -> Decimal {
        self.taxable_amount
    }
    /// Returns the VAT amount for this category and rate.
    pub fn tax_amount(&self) -> Decimal {
        self.tax_amount
    }
}
impl InvoiceTotals {
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
            group.taxable_amount = if adjustment.kind.is_charge() {
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
    /// Returns the sum of line net amounts (`cbc:LineExtensionAmount`), excluding prepayment adjustment lines.
    pub fn line_extension(&self) -> Decimal {
        self.line_extension
    }
    /// Returns the total VAT amount in the document currency.
    pub fn tax_amount(&self) -> Decimal {
        self.tax_amount
    }
    /// Returns the total of document-level allowances (`cbc:AllowanceTotalAmount`).
    pub fn allowance_total(&self) -> Decimal {
        self.allowance_total
    }
    /// Returns the total of document-level charges (`cbc:ChargeTotalAmount`).
    pub fn charge_total(&self) -> Decimal {
        self.charge_total
    }
    /// Returns the total excluding VAT (`cbc:TaxExclusiveAmount`): line total minus allowances plus charges.
    pub fn taxable_amount(&self) -> Decimal {
        self.taxable_amount
    }
    /// Returns the total including VAT (`cbc:TaxInclusiveAmount`).
    pub fn tax_inclusive_amount(&self) -> Decimal {
        self.tax_inclusive_amount
    }
    /// Returns the amount already paid (`cbc:PrepaidAmount`).
    pub fn prepaid_amount(&self) -> Decimal {
        self.prepaid_amount
    }
    /// Returns the rounding applied to the payable amount (`cbc:PayableRoundingAmount`).
    pub fn payable_rounding_amount(&self) -> Decimal {
        self.payable_rounding_amount
    }
    /// Returns the amount due for payment (`cbc:PayableAmount`).
    pub fn payable_amount(&self) -> Decimal {
        self.payable_amount
    }
    /// Returns the VAT breakdown, one entry per VAT category and rate.
    pub fn vat_breakdown(&self) -> &[VatBreakdown] {
        &self.groups
    }
}
