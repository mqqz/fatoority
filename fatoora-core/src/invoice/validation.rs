//! XML schema validation and complete local ZATCA validation reports.
//!
//! XSD checks do not run builder field checks, business rules, or signature
//! verification. Signed imports separately check local signature and QR integrity.
mod decimal_lexical;
#[cfg(test)]
mod forwarder_tests;
pub(crate) mod integrity;
mod integrity_xml;
#[cfg(test)]
mod integrity_xml_tests;
mod report;
mod schemas;
mod zatca;
#[cfg(test)]
mod zatca_tests;
#[allow(deprecated)]
pub use zatca::validate_zatca_invoice_from_str;
mod zatca_report;
pub use zatca_report::{
    ZatcaFailureKind, ZatcaFinding, ZatcaRuleSource, ZatcaStage, ZatcaStageReport,
    ZatcaStageStatus, ZatcaValidationError, ZatcaValidationOptions, ZatcaValidationReport,
};
mod business_rules;
use crate::config::Config;
pub use report::{
    Severity, ValidationFinding, ValidationLayer, ValidationLocation, ValidationReport,
};
use thiserror::Error;

/// Result of XSD validation: `Ok(())` when the document conforms to the UBL schema.
pub type ValidationResult = Result<(), XmlValidationError>;

/// Errors emitted during XML schema validation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XmlValidationError {
    /// An XSD path could not be used. Schemas are embedded, so current validators do not produce
    /// this variant.
    #[error("invalid XSD path: {path}")]
    InvalidXsdPath {
        /// The rejected path.
        path: String,
    },
    /// The embedded UBL schemas failed to compile.
    #[error("schema parser error")]
    SchemaParse {
        /// Schema compiler diagnostics.
        errors: Vec<crate::Diagnostic>,
    },
    /// The input is not well-formed or supported XML.
    #[error("XML parse error: {message}")]
    XmlParse {
        /// Parser error description.
        message: String,
    },
    /// The document does not conform to the UBL invoice schema.
    #[error("schema validation error")]
    SchemaValidation {
        /// One diagnostic per schema violation.
        errors: Vec<crate::Diagnostic>,
    },
}

impl XmlValidationError {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        match self {
            Self::InvalidXsdPath { .. } => crate::ErrorKind::InvalidInput,
            Self::SchemaParse { .. } => crate::ErrorKind::Parse,
            Self::XmlParse { .. } => crate::ErrorKind::Xml,
            Self::SchemaValidation { .. } => crate::ErrorKind::Validation,
        }
    }
}

/// Entry point for local invoice validation.
///
/// Validation needs no runtime state: the UBL schemas and the pinned ZATCA
/// rule set are embedded in the crate, and the ZATCA environment does not
/// affect the result. Construct one with [`Validator::new`] (or
/// [`Validator::default`]) and reuse it freely; it is `Copy`.
///
/// | Method | Checks |
/// | --- | --- |
/// | [`validate_xml`](Self::validate_xml) | UBL XSD only, pass/fail |
/// | [`validate_xml_report`](Self::validate_xml_report) | UBL XSD only, as a [`ValidationReport`] |
/// | [`validate_zatca`](Self::validate_zatca) | full local ZATCA profile (XSD, CEN, KSA, signature, QR, PIH) |
///
/// # Examples
/// ```rust
/// use fatoora_core::invoice::validation::{Validator, ZatcaValidationOptions};
///
/// let xml = include_str!("../../tests/fixtures/sdk-parity/cases/standard-invoice/input.xml");
/// let validator = Validator::new();
/// validator.validate_xml(xml)?;
///
/// let options = ZatcaValidationOptions {
///     evaluated_at: Some("2026-09-23T12:00:00+03:00".parse()?),
///     previous_invoice_hash: Some("NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==".into()),
/// };
/// let report = validator.validate_zatca(xml, &options)?;
/// assert!(report.is_valid());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Validator {
    _private: (),
}

impl Validator {
    /// Create a validator using the bundled UBL XSD and ZATCA rule set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate an XML invoice string against the UBL schema only.
    ///
    /// Success establishes XSD conformance, not business-rule compliance or
    /// signature authenticity. See [`Validator::validate_xml_report`] for
    /// explicit coverage and [`Validator::validate_zatca`] for the full rule set.
    ///
    /// # Errors
    /// Returns [`XmlValidationError`] if the XML is invalid or validation fails.
    pub fn validate_xml(&self, xml: &str) -> ValidationResult {
        schemas::validate(xml)
    }

    /// Check the bundled UBL XSD and return findings with explicit layer coverage.
    ///
    /// Schema violations return `Ok(report)` with error findings. Only the XSD
    /// layer is marked checked, even when the input contains a signature. Source
    /// locations are included only when supplied by the backend. Fatal
    /// diagnostics are errors.
    ///
    /// # Errors
    /// Returns [`XmlValidationError`] when the schema cannot be loaded or parsed,
    /// or the input is not parseable XML. In those cases XSD validation did not
    /// complete.
    pub fn validate_xml_report(&self, xml: &str) -> Result<ValidationReport, XmlValidationError> {
        schema_validation_report(self.validate_xml(xml))
    }

    /// Validate raw XML against the pinned ZATCA SDK profile using local checks.
    ///
    /// Schema and business-rule rejections return a report. Warnings remain
    /// visible. Malformed input, resource exhaustion, or an interrupted
    /// evaluation return an error carrying the partial report.
    /// `report.is_valid()` requires every applicable stage to complete without
    /// errors; an empty finding list alone never establishes success.
    ///
    /// The default clock is UTC, captured once. Supply `evaluated_at` to
    /// reproduce a result or specify an implicit timezone. `previous_invoice_hash`
    /// is required to verify chain continuity; omitting it leaves that stage
    /// `context_required`. The SDK profile omits signature/QR checks for standard
    /// invoices. Local signature checks establish content/key integrity, not
    /// issuer trust, remote acceptance, certificate revocation, or invoice
    /// history. On WASM, `evaluated_at` is required.
    ///
    /// # Errors
    /// Returns [`ZatcaValidationError`] with completed and partial stage findings.
    pub fn validate_zatca(
        &self,
        xml: &str,
        options: &ZatcaValidationOptions,
    ) -> Result<ZatcaValidationReport, ZatcaValidationError> {
        zatca::validate_zatca(self, xml, options)
    }
}

/// Validate an XML invoice string against the UBL schema only.
///
/// # Errors
/// Returns [`XmlValidationError`] if the XML is invalid or validation fails.
#[deprecated(since = "0.6.0", note = "use `Validator::validate_xml` instead")]
pub fn validate_xml_invoice_from_str(xml: &str, config: &Config) -> ValidationResult {
    let _ = config;
    Validator::new().validate_xml(xml)
}

/// Check the bundled UBL XSD and return findings with explicit layer coverage.
///
/// # Errors
/// Returns [`XmlValidationError`] when the schema cannot be loaded or parsed, or
/// the input is not parseable XML.
#[deprecated(since = "0.6.0", note = "use `Validator::validate_xml_report` instead")]
pub fn validate_xml_invoice_report_from_str(
    xml: &str,
    config: &Config,
) -> Result<ValidationReport, XmlValidationError> {
    let _ = config;
    Validator::new().validate_xml_report(xml)
}

fn schema_validation_report(
    result: ValidationResult,
) -> Result<ValidationReport, XmlValidationError> {
    let issues = match result {
        Ok(()) => Vec::new(),
        Err(XmlValidationError::SchemaValidation { errors }) => {
            let mut findings: Vec<_> = errors
                .into_iter()
                .map(|diagnostic| ValidationFinding {
                    layer: ValidationLayer::Xsd,
                    code: "XSD_INVALID".to_owned(),
                    severity: match diagnostic.severity() {
                        Some(crate::DiagnosticSeverity::Warning) => Severity::Warning,
                        _ => Severity::Error,
                    },
                    message: diagnostic.message().to_owned(),
                    location: diagnostic.line().map(|line| ValidationLocation::Xml {
                        line,
                        column: diagnostic.column(),
                    }),
                })
                .collect();
            // A backend failure must remain a failure even if it supplies no
            // diagnostics, or only warning diagnostics.
            if !findings
                .iter()
                .any(|finding| finding.severity == Severity::Error)
            {
                findings.push(ValidationFinding {
                    layer: ValidationLayer::Xsd,
                    code: "XSD_INVALID".to_owned(),
                    severity: Severity::Error,
                    message: "XML failed schema validation".to_owned(),
                    location: None,
                });
            }
            findings
        }
        Err(error) => return Err(error),
    };
    Ok(ValidationReport {
        layers_checked: vec![ValidationLayer::Xsd],
        issues,
    })
}

/// QR money shares the bounded XML-decimal domain. Scale remains a separate rule.
pub(crate) fn check_qr_decimal(value: &str) -> Result<(), String> {
    business_rules::decimal_equal(value, value)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod schema_report_tests {
    use super::*;

    #[test]
    fn failure_without_diagnostics_still_reports_an_error() {
        let report = schema_validation_report(Err(XmlValidationError::SchemaValidation {
            errors: Vec::new(),
        }))
        .unwrap();
        assert!(report.has_errors());
        assert_eq!(report.layers_checked, [ValidationLayer::Xsd]);
        assert_eq!(report.issues.len(), 1);
        assert_eq!(report.issues[0].code, "XSD_INVALID");
        assert_eq!(report.issues[0].location, None);
    }

    #[test]
    fn schema_report_preserves_backend_source_coordinates() {
        for column in [0, 7] {
            let diagnostic =
                crate::Diagnostic::from_schema(xsd_schema::validation::ValidationError {
                    constraint: "cvc-type.3.1.3",
                    message: "invalid decimal".into(),
                    location: Some(xsd_schema::parser::location::SourceLocation {
                        base_uri: "invoice.xml".into(),
                        line: 12,
                        column,
                    }),
                    element_path: Some("/Invoice/Quantity".into()),
                });
            let message = diagnostic.message().to_owned();
            let report = schema_validation_report(Err(XmlValidationError::SchemaValidation {
                errors: vec![diagnostic],
            }))
            .unwrap();
            assert_eq!(report.issues.len(), 1);
            assert!(report.has_errors());
            assert_eq!(report.issues[0].message, message);
            assert_eq!(
                report.issues[0].location,
                Some(ValidationLocation::Xml {
                    line: 12,
                    column: (column > 0).then_some(column as u32),
                })
            );
        }
    }

    #[test]
    fn incomplete_validation_remains_an_error_instead_of_a_report() {
        for (error, kind) in [
            (
                XmlValidationError::InvalidXsdPath {
                    path: "missing.xsd".into(),
                },
                crate::ErrorKind::InvalidInput,
            ),
            (
                XmlValidationError::SchemaParse {
                    errors: vec![crate::Diagnostic::new("bad schema")],
                },
                crate::ErrorKind::Parse,
            ),
            (
                XmlValidationError::XmlParse {
                    message: "bad XML".into(),
                },
                crate::ErrorKind::Xml,
            ),
        ] {
            let original = error.to_string();
            let returned = schema_validation_report(Err(error)).unwrap_err();
            assert_eq!(returned.kind(), kind);
            assert_eq!(returned.to_string(), original);
        }
        assert_eq!(
            XmlValidationError::SchemaValidation { errors: vec![] }.kind(),
            crate::ErrorKind::Validation
        );
        assert!(matches!(
            Validator::new().validate_xml_report("<Invoice"),
            Err(XmlValidationError::XmlParse { .. })
        ));
    }
}
