use fatoora_core::config::{Config, Environment};
use fatoora_core::{Error, ErrorKind};

#[test]
fn environment_selection_never_defaults_to_production() {
    assert_eq!(Config::default().env(), Environment::NonProduction);
    for (input, environment, path) in [
        (
            "NON_PRODUCTION",
            Environment::NonProduction,
            "developer-portal",
        ),
        ("Simulation", Environment::Simulation, "simulation"),
        ("production", Environment::Production, "core"),
    ] {
        let parsed: Environment = input.parse().unwrap();
        assert_eq!(parsed, environment);
        assert_eq!(parsed.as_str(), input.to_ascii_lowercase());
        assert_eq!(
            Config::new(parsed).env().endpoint_url(),
            format!("https://gw-fatoora.zatca.gov.sa/e-invoicing/{path}/")
        );
    }
    for input in ["", "prod", " production", "production\0", "simulatiоn"] {
        let error: Error = input.parse::<Environment>().unwrap_err().into();
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        let details: serde_json::Value = serde_json::from_str(&error.details_json()).unwrap();
        assert_eq!(details["type"], "invalid_environment");
        assert_eq!(details["value"], input);
    }
}
