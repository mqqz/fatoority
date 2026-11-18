//! Names introduced in 0.6: CSR properties parsing, `Environment` and the CSID aliases.
use fatoora_core::api::{Compliance, ComplianceCsid, CsidCredentials, Production, ProductionCsid};
use fatoora_core::config::Environment;
use fatoora_core::csr::{CsrError, CsrProperties, SigningKey};
use std::str::FromStr;

const PROPERTIES: &str = include_str!("fixtures/csr-configs/csr-config-example-EN.properties");

#[test]
fn csr_properties_from_str_and_from_file_agree() {
    let parsed: CsrProperties = PROPERTIES.parse().unwrap();
    assert_eq!(CsrProperties::from_str(PROPERTIES).unwrap(), parsed);

    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("csr.properties");
    std::fs::write(&path, PROPERTIES).unwrap();
    assert_eq!(CsrProperties::from_file(&path).unwrap(), parsed);
    assert_eq!(CsrProperties::from_file(path.as_path()).unwrap(), parsed);

    // Both entry points build the same CSR subject.
    let key = SigningKey::generate();
    let a = parsed.build(&key, Environment::NonProduction).unwrap();
    let b = CsrProperties::from_file(&path)
        .unwrap()
        .build(&key, Environment::NonProduction)
        .unwrap();
    assert_eq!(a.subject_string(), b.subject_string());
}

#[test]
fn csr_properties_from_file_reports_the_file_path() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("missing.properties");
    assert!(matches!(
        CsrProperties::from_file(&missing),
        Err(CsrError::Io { path, .. }) if path == missing
    ));

    let incomplete = directory.path().join("incomplete.properties");
    std::fs::write(&incomplete, "csr.common.name=example\n").unwrap();
    assert!(matches!(
        CsrProperties::from_file(&incomplete),
        Err(CsrError::MissingProperty { path, .. }) if path == incomplete
    ));
    assert!(matches!(
        "csr.common.name=example\n".parse::<CsrProperties>(),
        Err(CsrError::MissingProperty { path, .. }) if path.as_os_str() == "<properties>"
    ));
}

#[test]
fn csid_aliases_name_the_typestates() {
    let compliance: ComplianceCsid =
        CsidCredentials::<Compliance>::new(Environment::NonProduction, None, "token", "secret");
    let production: ProductionCsid =
        CsidCredentials::<Production>::new(Environment::NonProduction, None, "token", "secret");
    assert_eq!(compliance.env(), production.env());
}
