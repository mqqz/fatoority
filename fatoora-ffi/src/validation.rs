//! Owned local ZATCA validation reports mirroring core `ZatcaValidationReport`.
use crate::common::local_error;
use fatoora_core::invoice::validation as zv;

impl From<zv::ZatcaStage> for ffi::ZatcaStage {
    fn from(value: zv::ZatcaStage) -> Self {
        match value {
            zv::ZatcaStage::Xsd => Self::Xsd,
            zv::ZatcaStage::Cen => Self::Cen,
            zv::ZatcaStage::Ksa => Self::Ksa,
            zv::ZatcaStage::Signature => Self::Signature,
            zv::ZatcaStage::Qr => Self::Qr,
            zv::ZatcaStage::PreviousInvoiceHash => Self::PreviousInvoiceHash,
            _ => Self::Unknown,
        }
    }
}

impl From<zv::ZatcaStageStatus> for ffi::ZatcaStageStatus {
    fn from(value: zv::ZatcaStageStatus) -> Self {
        match value {
            zv::ZatcaStageStatus::NotRun => Self::NotRun,
            zv::ZatcaStageStatus::Completed => Self::Completed,
            zv::ZatcaStageStatus::NotApplicable => Self::NotApplicable,
            zv::ZatcaStageStatus::ContextRequired => Self::ContextRequired,
            zv::ZatcaStageStatus::EvaluationFailed => Self::EvaluationFailed,
            _ => Self::Unknown,
        }
    }
}

impl From<zv::ValidationLayer> for ffi::ValidationLayer {
    fn from(value: zv::ValidationLayer) -> Self {
        match value {
            zv::ValidationLayer::FieldChecks => Self::FieldChecks,
            zv::ValidationLayer::Xsd => Self::Xsd,
            zv::ValidationLayer::BusinessRules => Self::BusinessRules,
            zv::ValidationLayer::Signature => Self::Signature,
            zv::ValidationLayer::Qr => Self::Qr,
            zv::ValidationLayer::PreviousInvoiceHash => Self::PreviousInvoiceHash,
            _ => Self::Unknown,
        }
    }
}

fn index_error(what: &str) -> Box<crate::common::ffi::BindingError> {
    local_error(1, &format!("{what} index out of range"))
}

fn json_error(error: serde_json::Error) -> Box<crate::common::ffi::BindingError> {
    local_error(
        9,
        &format!("Could not serialize validation report: {error}"),
    )
}

#[diplomat::bridge]
#[diplomat::abi_rename = "fatoora_{0}"]
#[diplomat::attr(cpp, namespace = "fatoora")]
pub mod ffi {
    use super::{index_error, json_error, zv};
    use crate::common::ffi::{BindingError, Text};
    use crate::common::{boundary, write, write_optional};
    use diplomat_runtime::DiplomatWrite;

    /// A stage of the local ZATCA validation profile.
    ///
    /// `Unknown` marks a stage this binding version does not recognize.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ZatcaStage {
        Unknown = 0,
        /// UBL XML schema (XSD) validation.
        Xsd = 1,
        /// CEN EN 16931 business rules.
        Cen = 2,
        /// ZATCA KSA-specific business rules.
        Ksa = 3,
        /// Cryptographic signature, digest and certificate checks.
        Signature = 4,
        /// QR payload consistency checks.
        Qr = 5,
        /// Comparison of the embedded previous invoice hash with the supplied one.
        PreviousInvoiceHash = 6,
    }

    /// Whether a stage ran and produced a complete result.
    ///
    /// `Unknown` marks a status this binding version does not recognize.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ZatcaStageStatus {
        Unknown = 0,
        /// The stage did not start, typically because an earlier stage failed.
        NotRun = 1,
        /// All checks finished; findings can still contain validation errors.
        Completed = 2,
        /// The profile does not require this stage for the document.
        NotApplicable = 3,
        /// Required caller context was unavailable.
        ContextRequired = 4,
        /// Execution stopped before the stage completed; findings may be partial.
        EvaluationFailed = 5,
    }

    /// The validation layer that produced a finding.
    ///
    /// `Unknown` marks a layer this binding version does not recognize.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ValidationLayer {
        Unknown = 0,
        /// Builder/model field checks.
        FieldChecks = 1,
        /// UBL XML schema (XSD) validation.
        Xsd = 2,
        /// CEN EN 16931 and KSA business rules.
        BusinessRules = 3,
        /// Cryptographic signature, digest and certificate checks.
        Signature = 4,
        /// QR payload consistency.
        Qr = 5,
        /// Previous invoice hash comparison.
        PreviousInvoiceHash = 6,
    }

    /// Severity of a validation finding.
    #[diplomat::enum_convert(zv::Severity)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum Severity {
        /// Reported but does not make the document invalid.
        Warning = 0,
        /// Makes the document invalid.
        Error = 1,
    }

    /// Which kind of location a finding carries.
    ///
    /// `Unknown` marks a location kind this binding version does not recognize.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ValidationLocationKind {
        Unknown = 0,
        /// A model field path such as `line_items[0].quantity`; see `location_path`.
        Field = 1,
        /// A namespace-independent XPath; see `location_path`.
        XPath = 2,
        /// One-based XML source coordinates; see `location_line`/`location_column`.
        Xml = 3,
    }

    /// Owned results of local ZATCA validation with explicit stage coverage.
    ///
    /// Completion and validity describe only the local profile; they do not
    /// establish acceptance by ZATCA or certificate issuer trust.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ZatcaValidationReport(pub(crate) zv::ZatcaValidationReport);

    /// Coverage and findings from one validation stage.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ZatcaStageReport(pub(crate) zv::ZatcaStageReport);

    /// Pinned upstream rule source of a business-rule stage.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ZatcaRuleSource(pub(crate) zv::ZatcaRuleSource);

    /// One validation finding with its source assertion, when applicable.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ZatcaFinding(pub(crate) zv::ZatcaFinding);

    impl ZatcaValidationReport {
        /// Version of the report's serialized format.
        pub fn schema_version(&self) -> u32 {
            self.0.schema_version
        }
        /// Identifier of the validation profile (the ZATCA SDK release it mirrors).
        pub fn profile(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(&self.0.profile, out))
        }
        /// RFC 3339 instant used for time-dependent rules.
        pub fn evaluated_at(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| {
                let value = serde_json::to_value(self.0.evaluated_at).map_err(json_error)?;
                write(value.as_str().unwrap_or_default(), out)
            })
        }
        /// Whether every required stage finished exactly once.
        pub fn is_complete(&self) -> bool {
            self.0.is_complete()
        }
        /// Whether any stage, including an incomplete stage, reported an error.
        pub fn has_errors(&self) -> bool {
            self.0.has_errors()
        }
        /// Whether validation is complete and contains no error findings.
        pub fn is_valid(&self) -> bool {
            self.0.is_valid()
        }
        pub fn stages_len(&self) -> usize {
            self.0.stages.len()
        }
        /// The stage report at `index`, in stage order.
        pub fn stage(&self, index: usize) -> Result<Box<ZatcaStageReport>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .stages
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(ZatcaStageReport(v)))
                    .ok_or_else(|| index_error("stage"))
            })
        }
        /// The report serialized as JSON, including the derived `is_complete`,
        /// `has_errors` and `is_valid` fields.
        pub fn to_json(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(serde_json::to_string(&self.0).map_err(json_error)?, out))
        }
    }

    impl ZatcaStageReport {
        pub fn stage(&self) -> ZatcaStage {
            self.0.stage.into()
        }
        pub fn status(&self) -> ZatcaStageStatus {
            self.0.status.into()
        }
        /// Pinned rule source, for the CEN and KSA business-rule stages.
        pub fn provenance(&self) -> Result<Option<Box<ZatcaRuleSource>>, Box<BindingError>> {
            boundary(|| {
                Ok(self
                    .0
                    .provenance
                    .clone()
                    .map(|v| Box::new(ZatcaRuleSource(v))))
            })
        }
        pub fn evaluated_assertions_len(&self) -> usize {
            self.0.evaluated_assertions.len()
        }
        /// Assertion site at `index` whose selected contexts were all evaluated.
        pub fn evaluated_assertion(&self, index: usize) -> Result<Box<Text>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .evaluated_assertions
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(Text(v)))
                    .ok_or_else(|| index_error("assertion"))
            })
        }
        pub fn findings_len(&self) -> usize {
            self.0.findings.len()
        }
        pub fn finding(&self, index: usize) -> Result<Box<ZatcaFinding>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .findings
                    .get(index)
                    .cloned()
                    .map(|v| Box::new(ZatcaFinding(v)))
                    .ok_or_else(|| index_error("finding"))
            })
        }
    }

    impl ZatcaRuleSource {
        /// File name of the upstream rule stylesheet.
        pub fn stylesheet(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(&self.0.stylesheet, out))
        }
        /// Hex SHA-256 digest of the pinned stylesheet.
        pub fn sha256(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(&self.0.sha256, out))
        }
    }

    impl ZatcaFinding {
        /// Rule assertion that produced the finding, for business-rule findings.
        pub fn assertion_site(&self, out: &mut DiplomatWrite) -> Option<()> {
            write_optional(self.0.assertion_site.as_deref(), out)
        }
        pub fn layer(&self) -> ValidationLayer {
            self.0.finding.layer.into()
        }
        /// Library-owned rule identifier, independent of message wording.
        pub fn code(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(&self.0.finding.code, out))
        }
        pub fn severity(&self) -> Severity {
            self.0.finding.severity.into()
        }
        /// Human-readable text; the wording is not a stable API.
        pub fn message(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(&self.0.finding.message, out))
        }
        /// The kind of location, or null when the source cannot locate the finding.
        pub fn location_kind(&self) -> Option<ValidationLocationKind> {
            self.0
                .finding
                .location
                .as_ref()
                .map(|location| match location {
                    zv::ValidationLocation::Field(_) => ValidationLocationKind::Field,
                    zv::ValidationLocation::XPath(_) => ValidationLocationKind::XPath,
                    zv::ValidationLocation::Xml { .. } => ValidationLocationKind::Xml,
                    _ => ValidationLocationKind::Unknown,
                })
        }
        /// The field path or XPath, or null for other location kinds.
        pub fn location_path(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| {
                Ok(match &self.0.finding.location {
                    Some(zv::ValidationLocation::Field(path))
                    | Some(zv::ValidationLocation::XPath(path)) => {
                        Some(Box::new(Text(path.clone())))
                    }
                    _ => None,
                })
            })
        }
        /// One-based XML line, or null when the location is not XML coordinates.
        pub fn location_line(&self) -> Option<u32> {
            match &self.0.finding.location {
                Some(zv::ValidationLocation::Xml { line, .. }) => Some(*line),
                _ => None,
            }
        }
        /// One-based XML column, or null when unknown.
        pub fn location_column(&self) -> Option<u32> {
            match &self.0.finding.location {
                Some(zv::ValidationLocation::Xml { column, .. }) => *column,
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ffi::*;
    use super::zv;

    fn finding(location: Option<zv::ValidationLocation>) -> zv::ZatcaFinding {
        serde_json::from_value(serde_json::json!({
            "assertion_site": "BR-01#0",
            "layer": "business_rules",
            "code": "BR-01",
            "severity": "error",
            "message": "رسالة",
            "location": location,
        }))
        .unwrap()
    }

    fn report() -> ZatcaValidationReport {
        let mut stages: Vec<serde_json::Value> = [
            "xsd",
            "cen",
            "ksa",
            "signature",
            "qr",
            "previous_invoice_hash",
        ]
        .into_iter()
        .map(|stage| {
            serde_json::json!({
                "stage": stage, "status": "completed", "provenance": null,
                "evaluated_assertions": [], "findings": []
            })
        })
        .collect();
        stages[1]["provenance"] = serde_json::json!({"stylesheet": "CEN.xsl", "sha256": "ab"});
        stages[1]["evaluated_assertions"] = serde_json::json!(["BR-01#0"]);
        stages[1]["findings"] = serde_json::json!([finding(Some(zv::ValidationLocation::XPath(
            "/Invoice".into()
        )))]);
        ZatcaValidationReport(
            serde_json::from_value(serde_json::json!({
                "schema_version": 1,
                "profile": "zatca-sdk",
                "evaluated_at": "2026-01-02T03:04:05+03:00",
                "stages": stages,
            }))
            .unwrap(),
        )
    }

    fn written(f: impl FnOnce(&mut diplomat_runtime::DiplomatWrite)) -> String {
        struct Writer(*mut diplomat_runtime::DiplomatWrite);
        impl Drop for Writer {
            fn drop(&mut self) {
                // SAFETY: the buffer was created by `diplomat_buffer_write_create`.
                unsafe { diplomat_runtime::diplomat_buffer_write_destroy(self.0) }
            }
        }
        let out = Writer(diplomat_runtime::diplomat_buffer_write_create(0));
        // SAFETY: the buffer stays alive until `out` is dropped.
        unsafe {
            f(&mut *out.0);
            String::from_utf8((*out.0).as_bytes().to_vec()).unwrap()
        }
    }

    #[test]
    fn report_exposes_outcomes_stages_and_json() {
        let report = report();
        assert_eq!(report.schema_version(), 1);
        assert!(report.is_complete());
        assert!(report.has_errors());
        assert!(!report.is_valid());
        assert_eq!(written(|o| report.profile(o).ok().unwrap()), "zatca-sdk");
        assert_eq!(
            written(|o| report.evaluated_at(o).ok().unwrap()),
            "2026-01-02T03:04:05+03:00"
        );
        let json: serde_json::Value =
            serde_json::from_str(&written(|o| report.to_json(o).ok().unwrap())).unwrap();
        assert_eq!(json["is_valid"], false);
        assert_eq!(json["stages"].as_array().unwrap().len(), 6);
        assert_eq!(report.stages_len(), 6);
        assert!(report.stage(6).is_err());
        let expected = [
            ZatcaStage::Xsd,
            ZatcaStage::Cen,
            ZatcaStage::Ksa,
            ZatcaStage::Signature,
            ZatcaStage::Qr,
            ZatcaStage::PreviousInvoiceHash,
        ];
        for (index, stage) in expected.into_iter().enumerate() {
            let value = report.stage(index).ok().unwrap();
            assert_eq!(value.stage(), stage);
            assert_eq!(value.status(), ZatcaStageStatus::Completed);
        }
        let cen = report.stage(1).ok().unwrap();
        drop(report);
        let source = cen.provenance().ok().unwrap().unwrap();
        assert_eq!(written(|o| source.stylesheet(o).ok().unwrap()), "CEN.xsl");
        assert_eq!(written(|o| source.sha256(o).ok().unwrap()), "ab");
        assert_eq!(cen.evaluated_assertions_len(), 1);
        assert_eq!(cen.evaluated_assertion(0).ok().unwrap().0, "BR-01#0");
        assert!(cen.evaluated_assertion(1).is_err());
        assert_eq!(cen.findings_len(), 1);
        assert!(cen.finding(1).is_err());
        let finding = cen.finding(0).ok().unwrap();
        let mut present = None;
        let site = written(|o| present = finding.assertion_site(o));
        assert_eq!((present, site.as_str()), (Some(()), "BR-01#0"));
        assert_eq!(finding.layer(), ValidationLayer::BusinessRules);
        assert_eq!(finding.severity(), Severity::Error);
        assert_eq!(written(|o| finding.code(o).ok().unwrap()), "BR-01");
        assert_eq!(written(|o| finding.message(o).ok().unwrap()), "رسالة");
        assert_eq!(finding.location_kind(), Some(ValidationLocationKind::XPath));
        assert_eq!(finding.location_path().ok().unwrap().unwrap().0, "/Invoice");
        assert_eq!(finding.location_line(), None);
        assert_eq!(finding.location_column(), None);
        let first = report_stage_without_provenance();
        assert!(first.provenance().ok().unwrap().is_none());
    }

    fn report_stage_without_provenance() -> Box<ZatcaStageReport> {
        report().stage(0).ok().unwrap()
    }

    #[test]
    fn finding_locations_map_every_kind() {
        let field = ZatcaFinding(finding(Some(zv::ValidationLocation::Field(
            "line_items[0].quantity".into(),
        ))));
        assert_eq!(field.location_kind(), Some(ValidationLocationKind::Field));
        assert_eq!(
            field.location_path().ok().unwrap().unwrap().0,
            "line_items[0].quantity"
        );
        let xml = ZatcaFinding(finding(Some(zv::ValidationLocation::Xml {
            line: 3,
            column: Some(7),
        })));
        assert_eq!(xml.location_kind(), Some(ValidationLocationKind::Xml));
        assert!(xml.location_path().ok().unwrap().is_none());
        assert_eq!(xml.location_line(), Some(3));
        assert_eq!(xml.location_column(), Some(7));
        let none = ZatcaFinding(finding(None));
        assert_eq!(none.location_kind(), None);
        assert_eq!(none.location_line(), None);
        let mut warning = finding(None);
        warning.assertion_site = None;
        warning.finding.severity = zv::Severity::Warning;
        let warning = ZatcaFinding(warning);
        assert_eq!(warning.severity(), Severity::Warning);
        let mut present = Some(());
        let site = written(|o| present = warning.assertion_site(o));
        assert_eq!((present, site.as_str()), (None, ""));
    }

    #[test]
    fn stage_status_and_layer_mappings_cover_core_variants() {
        for (core_value, ffi) in [
            (zv::ZatcaStageStatus::NotRun, ZatcaStageStatus::NotRun),
            (zv::ZatcaStageStatus::Completed, ZatcaStageStatus::Completed),
            (
                zv::ZatcaStageStatus::NotApplicable,
                ZatcaStageStatus::NotApplicable,
            ),
            (
                zv::ZatcaStageStatus::ContextRequired,
                ZatcaStageStatus::ContextRequired,
            ),
            (
                zv::ZatcaStageStatus::EvaluationFailed,
                ZatcaStageStatus::EvaluationFailed,
            ),
        ] {
            assert_eq!(ZatcaStageStatus::from(core_value), ffi);
        }
        for (core_value, ffi) in [
            (
                zv::ValidationLayer::FieldChecks,
                ValidationLayer::FieldChecks,
            ),
            (zv::ValidationLayer::Xsd, ValidationLayer::Xsd),
            (
                zv::ValidationLayer::BusinessRules,
                ValidationLayer::BusinessRules,
            ),
            (zv::ValidationLayer::Signature, ValidationLayer::Signature),
            (zv::ValidationLayer::Qr, ValidationLayer::Qr),
            (
                zv::ValidationLayer::PreviousInvoiceHash,
                ValidationLayer::PreviousInvoiceHash,
            ),
        ] {
            assert_eq!(ValidationLayer::from(core_value), ffi);
        }
    }
}
