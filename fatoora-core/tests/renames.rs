//! New names introduced in 0.6 and the deprecated aliases kept for one release.
use fatoora_core::api::{
    Compliance, ComplianceCsid, CsidCredentials, Production, ProductionCsid, ZatcaClient,
};
use fatoora_core::config::{Config, Environment};
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
#[allow(deprecated)]
fn deprecated_csr_properties_constructors_forward() {
    let expected: CsrProperties = PROPERTIES.parse().unwrap();
    assert_eq!(
        CsrProperties::from_properties_str(PROPERTIES).unwrap(),
        expected
    );
    assert_eq!(
        CsrProperties::parse_csr_config(PROPERTIES).unwrap(),
        expected
    );

    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("csr.properties");
    std::fs::write(&path, PROPERTIES).unwrap();
    assert_eq!(
        CsrProperties::parse_csr_config_file(&path).unwrap(),
        expected
    );
}

#[test]
#[allow(deprecated)]
fn deprecated_environment_type_alias_is_environment() {
    use fatoora_core::config::EnvironmentType;
    let env: EnvironmentType = Environment::Simulation;
    assert_eq!(EnvironmentType::from_str("simulation").unwrap(), env);
    assert_eq!(
        Config::new(EnvironmentType::Production).env(),
        Environment::Production
    );
}

#[test]
fn csid_aliases_name_the_typestates() {
    let compliance: ComplianceCsid =
        CsidCredentials::<Compliance>::new(Environment::NonProduction, None, "token", "secret");
    let production: ProductionCsid =
        CsidCredentials::<Production>::new(Environment::NonProduction, None, "token", "secret");
    assert_eq!(compliance.env(), production.env());
}

#[test]
#[allow(deprecated)]
fn deprecated_invoice_totals_alias_is_invoice_totals() {
    fn same(
        _: Option<&fatoora_core::invoice::InvoiceTotalsData>,
    ) -> Option<&fatoora_core::invoice::InvoiceTotals> {
        None
    }
    assert!(same(None).is_none());
}

#[test]
#[allow(deprecated)]
fn deprecated_csid_request_preparers_forward() {
    let client = ZatcaClient::new(Config::new(Environment::NonProduction)).unwrap();
    let csr = PROPERTIES
        .parse::<CsrProperties>()
        .unwrap()
        .build(&SigningKey::generate(), Environment::NonProduction)
        .unwrap();

    let new = client
        .prepare_request_compliance_csid(&csr, "123456")
        .unwrap();
    let old = client.prepare_post_csr_for_ccsid(&csr, "123456").unwrap();
    assert_eq!(old.request().url(), new.request().url());
    assert_eq!(old.request().headers(), new.request().headers());
    assert_eq!(old.request().body(), new.request().body());

    let ccsid: ComplianceCsid = CsidCredentials::new(
        Environment::NonProduction,
        Some("1234567890123".into()),
        "token",
        "secret",
    );
    let new = client.prepare_request_production_csid(&ccsid).unwrap();
    let old = client.prepare_post_ccsid_for_pcsid(&ccsid).unwrap();
    assert_eq!(old.request().url(), new.request().url());
    assert_eq!(old.request().headers(), new.request().headers());
    assert_eq!(old.request().body(), new.request().body());
}
