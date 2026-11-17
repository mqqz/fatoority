//! Configuration and environment selection.
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// ZATCA environment selection for API endpoints.
/// This determines which URL the API client will use and also is needed for the template format
/// used in CSR generation.
/// - NonProduction: This is what ZATCA refers to as the "Integration Sandbox".
/// - Simulation: This is the "Simulation Test Environment" provided by ZATCA which you need to
///   sign up for.
/// - Production: The live production environment.
/// # Examples
/// ```rust
/// use std::str::FromStr;
/// use fatoora_core::config::Environment;
///
/// let env = Environment::from_str("simulation")?;
/// assert_eq!(env, Environment::Simulation);
/// use fatoora_core::config::EnvironmentParseError;
/// # Ok::<(), EnvironmentParseError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Environment {
    /// ZATCA "Integration Sandbox" (developer portal); parsed from `"non_production"`.
    NonProduction,
    /// ZATCA "Simulation Test Environment"; parsed from `"simulation"`.
    Simulation,
    /// Live production environment; parsed from `"production"`.
    Production,
}

/// Former name of [`Environment`].
#[deprecated(since = "0.6.0", note = "use `Environment` instead")]
pub type EnvironmentType = Environment;

/// Error returned when parsing an [`Environment`] from a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[non_exhaustive]
pub enum EnvironmentParseError {
    /// The input did not match `non_production`, `simulation` or `production`
    /// (case-insensitive).
    #[error("invalid environment type: {input}")]
    Invalid {
        /// The rejected input, as given.
        input: String,
    },
}

impl EnvironmentParseError {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        crate::ErrorKind::InvalidInput
    }
}

impl FromStr for Environment {
    type Err = EnvironmentParseError;
    fn from_str(env: &str) -> Result<Environment, EnvironmentParseError> {
        match env.to_ascii_lowercase().as_str() {
            "non_production" => Ok(Environment::NonProduction),
            "simulation" => Ok(Environment::Simulation),
            "production" => Ok(Environment::Production),
            _ => Err(EnvironmentParseError::Invalid {
                input: env.to_string(),
            }),
        }
    }
}

impl Environment {
    /// Returns the canonical lowercase name (`non_production`, `simulation` or
    /// `production`), accepted by [`Environment::from_str`].
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::NonProduction => "non_production",
            Environment::Simulation => "simulation",
            Environment::Production => "production",
        }
    }

    /// Returns the base URL of this environment's ZATCA e-invoicing API, with a
    /// trailing slash.
    pub fn endpoint_url(&self) -> &'static str {
        match self {
            Environment::NonProduction => {
                "https://gw-fatoora.zatca.gov.sa/e-invoicing/developer-portal/"
            }
            Environment::Simulation => "https://gw-fatoora.zatca.gov.sa/e-invoicing/simulation/",
            Environment::Production => "https://gw-fatoora.zatca.gov.sa/e-invoicing/core/",
        }
    }
}

/// Configuration for validation and API clients.
///
/// # Examples
/// ```rust
/// use fatoora_core::config::{Config, Environment};
///
/// let config = Config::new(Environment::NonProduction);
/// # let _ = config;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Config {
    env: Environment,
}

impl Config {
    /// Create a config using the bundled UBL XSD.
    pub fn new(env: Environment) -> Self {
        Self { env }
    }

    /// Returns the configured ZATCA environment.
    pub fn env(&self) -> Environment {
        self.env
    }
}

// static function to get default config
impl Default for Config {
    fn default() -> Self {
        Config {
            env: Environment::NonProduction,
        }
    }
}
