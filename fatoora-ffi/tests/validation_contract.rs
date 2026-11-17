mod support;
use fatoora_ffi::validation::ffi::{
    Severity, ValidationLayer, ZatcaStage, ZatcaStageStatus, ZatcaValidationReport,
};
use fatoora_ffi::{common::ffi::BindingError, invoice::ffi::Xml};
use support::written;
fn ok<T>(value: Result<T, Box<BindingError>>) -> T {
    value.unwrap_or_else(|e| panic!("{}: {}", e.code(), written(|out| e.message(out))))
}

fn json(report: &ZatcaValidationReport) -> serde_json::Value {
    serde_json::from_str(&written(|out| ok(report.to_json(out)))).unwrap()
}

#[test]
fn options_limit_is_in_bytes_and_invalid_options_do_not_run_validation() {
    let mut options = br#"{"evaluated_at":"2026-09-23T12:00:00+03:00"}"#.to_vec();
    options.resize(4096, b' ');
    let report = ok(Xml::validate_zatca(b"<wrong/>", Some(&options)));
    assert_eq!(
        written(|out| ok(report.evaluated_at(out))),
        "2026-09-23T12:00:00+03:00"
    );
    assert!(!report.is_valid());
    let value = json(&report);
    assert_eq!(value["evaluated_at"], "2026-09-23T12:00:00+03:00");
    assert_eq!(value["is_valid"], false);
    let xsd = ok(report.stage(0));
    assert_eq!(xsd.stage(), ZatcaStage::Xsd);
    let finding = ok(xsd.finding(0));
    assert_eq!(written(|out| ok(finding.code(out))), "XSD_INVALID");
    assert_eq!(finding.layer(), ValidationLayer::Xsd);
    assert_eq!(finding.severity(), Severity::Error);
    assert_eq!(value["stages"][0]["findings"][0]["code"], "XSD_INVALID");
    options.push(b' ');
    for invalid in [
        options.as_slice(),
        b"{\"unknown\":true}",
        b"null",
        b"{}\0",
        b"\xff",
        br#"{"evaluated_at":"bad"}"#,
        br#"{"previous_invoice_hash":" "}"#,
    ] {
        let error = Xml::validate_zatca(b"<wrong/>", Some(invalid))
            .err()
            .unwrap();
        assert_eq!(error.code(), 1);
    }
}

#[test]
fn malformed_xml_error_keeps_partial_report_and_does_not_poison_next_call() {
    let error = Xml::validate_zatca(b"<Invoice", None).err().unwrap();
    let details: serde_json::Value =
        serde_json::from_str(&written(|out| error.details_json(out))).unwrap();
    assert_eq!(details["report"]["is_valid"], false);
    assert_eq!(details["report"]["is_complete"], false);
    assert_eq!(details["report"]["stages"].as_array().unwrap().len(), 6);
    let report = ok(Xml::validate_zatca(b"<wrong/>", None));
    assert!(report.has_errors());
    assert!(!report.is_complete());
    assert_eq!(report.schema_version(), json(&report)["schema_version"]);
    assert!(!written(|out| ok(report.profile(out))).is_empty());
    assert_eq!(report.stages_len(), 6);
    assert_eq!(ok(report.stage(0)).status(), ZatcaStageStatus::Completed);
    assert!((1..6).all(|index| ok(report.stage(index)).status() == ZatcaStageStatus::NotRun));
    let value = json(&report);
    assert_eq!(value["has_errors"], true);
    assert_eq!(value["stages"][0]["status"], "completed");
}
