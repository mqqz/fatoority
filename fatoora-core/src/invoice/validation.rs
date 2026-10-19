//! XML schema validation and complete local ZATCA validation reports.
//!
//! XSD checks do not run builder field checks, business rules, or signature
//! verification. Signed imports separately check local signature and QR integrity.
mod decimal_lexical;
pub(crate) mod integrity;
mod integrity_xml;
#[cfg(test)]
mod integrity_xml_tests;
mod report;
mod schemas;
mod zatca;
#[cfg(test)]
mod zatca_tests;
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

pub type ValidationResult = Result<(), XmlValidationError>;

/// Errors emitted during XML schema validation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XmlValidationError {
    #[error("invalid XSD path: {path}")]
    InvalidXsdPath { path: String },
    #[error("schema parser error")]
    SchemaParse { errors: Vec<crate::Diagnostic> },
    #[error("XML parse error: {message}")]
    XmlParse { message: String },
    #[error("schema validation error")]
    SchemaValidation { errors: Vec<crate::Diagnostic> },
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

/// Validate an XML invoice string against the UBL schema only.
///
/// Success establishes XSD conformance, not business-rule compliance or signature
/// authenticity. See [`validate_xml_invoice_report_from_str`] for explicit coverage.
///
/// # Errors
/// Returns [`XmlValidationError`] if the XML is invalid or validation fails.
pub fn validate_xml_invoice_from_str(xml: &str, config: &Config) -> ValidationResult {
    let _ = config;
    schemas::validate(xml)
}

/// Check the bundled UBL XSD and return findings with explicit layer coverage.
///
/// Schema violations return `Ok(report)` with error findings. Only the XSD layer
/// is marked checked, even when the input contains a signature. Source locations
/// are included only when supplied by the backend. Fatal diagnostics are errors.
///
/// # Errors
/// Returns [`XmlValidationError`] when the schema cannot be loaded or parsed, or
/// the input is not parseable XML. In those cases XSD validation did not complete.
pub fn validate_xml_invoice_report_from_str(
    xml: &str,
    config: &Config,
) -> Result<ValidationReport, XmlValidationError> {
    schema_validation_report(validate_xml_invoice_from_str(xml, config))
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
            validate_xml_invoice_report_from_str("<Invoice", &Config::default()),
            Err(XmlValidationError::XmlParse { .. })
        ));
    }
}
