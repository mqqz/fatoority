//! Crate-owned errors and backend diagnostics.
use crate::{api, config, csr, invoice};

mod details;

/// Backend diagnostic text and optional source location.
/// Messages are for people; their wording is not a stable API.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct Diagnostic {
    message: String,
    file: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
    severity: Option<DiagnosticSeverity>,
}

impl Diagnostic {
    /// Create a diagnostic with only a message and no location or severity.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file: None,
            line: None,
            column: None,
            severity: None,
        }
    }
    /// Human-readable diagnostic text.
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Source file or document URI, when provided by the backend.
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }
    /// One-based line number, when provided by the backend.
    pub fn line(&self) -> Option<u32> {
        self.line
    }
    /// One-based column number, when provided by the backend.
    pub fn column(&self) -> Option<u32> {
        self.column
    }

    /// Severity, when provided by the backend.
    pub fn severity(&self) -> Option<DiagnosticSeverity> {
        self.severity
    }

    pub(crate) fn from_schema(error: xsd_schema::validation::ValidationError) -> Self {
        let mut diagnostic = Self::new(error.to_string());
        diagnostic.severity = Some(DiagnosticSeverity::Error);
        if let Some(location) = error.location {
            diagnostic.file = (!location.base_uri.is_empty()).then_some(location.base_uri);
            diagnostic.line = u32::try_from(location.line).ok().filter(|line| *line > 0);
            diagnostic.column = u32::try_from(location.column)
                .ok()
                .filter(|column| *column > 0);
        }
        diagnostic
    }

    pub(crate) fn from_properties(error: java_properties::PropertiesError) -> Self {
        Self {
            line: error
                .line_number()
                .and_then(|line| u32::try_from(line).ok())
                .filter(|line| *line > 0),
            ..Self::new(error.to_string())
        }
    }
}

/// Severity reported by a diagnostic backend.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// A non-blocking issue.
    Warning,
    /// An error that fails the operation.
    Error,
    /// An unrecoverable error after which the backend stopped processing.
    Fatal,
}

/// Stable error kinds used across the core and FFI layers.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A caller-supplied value is malformed or out of range.
    InvalidInput = 1,
    /// Input is well-formed but fails invoice, CSR, or schema validation rules.
    Validation = 2,
    /// Structured input could not be parsed, such as a properties file, an XSD
    /// schema, or a ZATCA API response body.
    Parse = 3,
    /// XML could not be serialized, parsed, or processed.
    Xml = 4,
    /// Key handling, signing, hashing, or certificate processing failed.
    Crypto = 5,
    /// A file or other I/O operation failed.
    Io = 6,
    /// The HTTP request or response transport failed.
    Network = 7,
    /// The ZATCA API rejected the supplied credentials.
    Unauthorized = 8,
    /// An internal invariant or client state error.
    Internal = 9,
    /// The ZATCA API returned an error or non-accepted response.
    Api = 10,
}

/// An error retaining the structured failure from its originating module.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid environment name; see [`config::EnvironmentParseError`].
    #[error("{0}")]
    Environment(#[from] config::EnvironmentParseError),
    /// Decimal parsing or arithmetic failed; see [`crate::DecimalError`].
    #[error("{0}")]
    Decimal(#[from] crate::DecimalError),
    /// CSR or signing-key handling failed; see [`csr::CsrError`].
    #[error("{0}")]
    Csr(#[from] csr::CsrError),
    /// Invoice construction or validation failed; see [`invoice::InvoiceError`].
    #[error("{0}")]
    Invoice(#[from] invoice::InvoiceError),
    /// Invoice signing failed; see [`invoice::sign::SigningError`].
    #[error("{0}")]
    Signing(#[from] invoice::sign::SigningError),
    /// QR payload generation failed; see [`invoice::QrCodeError`].
    #[error("{0}")]
    Qr(#[from] invoice::QrCodeError),
    /// Invoice XML serialization failed; see [`invoice::xml::InvoiceXmlError`].
    #[error("{0}")]
    Xml(#[from] invoice::xml::InvoiceXmlError),
    /// Parsing invoice XML into the model failed; see [`invoice::xml::parse::ParseError`].
    #[error("{0}")]
    Parse(#[from] invoice::xml::parse::ParseError),
    /// XML schema/structure validation could not run or failed; see
    /// [`invoice::validation::XmlValidationError`].
    #[error("{0}")]
    XmlValidation(#[from] invoice::validation::XmlValidationError),
    /// ZATCA validation could not run; see [`invoice::validation::ZatcaValidationError`].
    #[error("{0}")]
    ZatcaValidation(#[from] invoice::validation::ZatcaValidationError),
    /// A ZATCA API call failed; see [`api::ZatcaError`].
    #[error("{0}")]
    Api(#[from] api::ZatcaError),
}

impl Error {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Environment(err) => err.kind(),
            Self::Decimal(err) => err.kind(),
            Self::Csr(err) => err.kind(),
            Self::Invoice(err) => err.kind(),
            Self::Signing(err) => err.kind(),
            Self::Qr(err) => err.kind(),
            Self::Xml(err) => err.kind(),
            Self::Parse(err) => err.kind(),
            Self::XmlValidation(err) => err.kind(),
            Self::ZatcaValidation(err) => err.kind(),
            Self::Api(err) => err.kind(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn schema_diagnostics_preserve_source_and_severity() {
        let error = xsd_schema::validation::ValidationError {
            constraint: "cvc-type.3.1.3",
            message: "invalid decimal".into(),
            location: Some(xsd_schema::parser::location::SourceLocation {
                base_uri: "invoice.xml".into(),
                line: 12,
                column: 7,
            }),
            element_path: Some("/Invoice/PayableAmount".into()),
        };
        let message = error.to_string();
        let diagnostic = super::Diagnostic::from_schema(error);
        assert_eq!(diagnostic.message(), message);
        assert_eq!(diagnostic.file(), Some("invoice.xml"));
        assert_eq!(diagnostic.line(), Some(12));
        assert_eq!(diagnostic.column(), Some(7));
        assert_eq!(
            diagnostic.severity(),
            Some(super::DiagnosticSeverity::Error)
        );
    }

    #[test]
    fn schema_diagnostics_omit_unavailable_or_unrepresentable_locations() {
        use xsd_schema::parser::location::SourceLocation;
        let mut locations = vec![
            None,
            Some(SourceLocation {
                base_uri: String::new(),
                line: 0,
                column: 0,
            }),
        ];
        if let Ok(overflow) = usize::try_from(u64::from(u32::MAX) + 1) {
            locations.push(Some(SourceLocation {
                base_uri: String::new(),
                line: overflow,
                column: overflow,
            }));
        }
        for location in locations {
            let diagnostic =
                super::Diagnostic::from_schema(xsd_schema::validation::ValidationError {
                    constraint: "cvc-type",
                    message: "invalid value".into(),
                    location,
                    element_path: None,
                });
            assert_eq!(diagnostic.file(), None);
            assert_eq!(diagnostic.line(), None);
            assert_eq!(diagnostic.column(), None);
            assert_eq!(
                diagnostic.severity(),
                Some(super::DiagnosticSeverity::Error)
            );
            assert!(diagnostic.message().contains("invalid value"));
        }
    }

    use super::{Error, ErrorKind};
    use crate::invoice::validation::{
        ZatcaFailureKind, ZatcaValidationError, ZatcaValidationReport,
    };
    use std::error::Error as StdError;

    #[test]
    fn zatca_execution_errors_keep_their_category_and_typed_source() {
        for (kind, expected) in [
            (ZatcaFailureKind::InvalidXml, ErrorKind::Xml),
            (ZatcaFailureKind::UnsupportedXml, ErrorKind::InvalidInput),
            (ZatcaFailureKind::CapacityExceeded, ErrorKind::InvalidInput),
            (ZatcaFailureKind::InvalidContext, ErrorKind::InvalidInput),
            (ZatcaFailureKind::RuleEvaluation, ErrorKind::Validation),
            (ZatcaFailureKind::Schema, ErrorKind::Parse),
            (ZatcaFailureKind::Integrity, ErrorKind::Crypto),
        ] {
            let failure = ZatcaValidationError {
                kind,
                stage: None,
                assertion_site: None,
                location: None,
                message: "validation execution stopped".to_owned(),
                report: Box::new(ZatcaValidationReport {
                    schema_version: 1,
                    profile: "zatca-sdk-238-R3.4.8".to_owned(),
                    evaluated_at: chrono::DateTime::parse_from_rfc3339("2026-09-23T12:00:00+03:00")
                        .unwrap(),
                    stages: Vec::new(),
                }),
            };
            let error = Error::from(failure);
            assert_eq!(error.kind(), expected);
            assert_eq!(error.to_string(), "validation execution stopped");
            let source = error
                .source()
                .unwrap()
                .downcast_ref::<ZatcaValidationError>()
                .expect("root error must preserve the typed validation failure");
            assert_eq!(source.kind, kind);
            assert!(!source.report.is_complete());
            assert!(matches!(error, Error::ZatcaValidation(_)));
        }
    }
}
