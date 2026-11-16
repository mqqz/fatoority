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
/// use fatoora_core::config::EnvironmentType;
///
/// let env = EnvironmentType::from_str("simulation")?;
/// assert_eq!(env, EnvironmentType::Simulation);
/// use fatoora_core::config::EnvironmentParseError;
/// # Ok::<(), EnvironmentParseError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EnvironmentType {
    /// ZATCA "Integration Sandbox" (developer portal); parsed from `"non_production"`.
    NonProduction,
    /// ZATCA "Simulation Test Environment"; parsed from `"simulation"`.
    Simulation,
    /// Live production environment; parsed from `"production"`.
    Production,
}

/// Error returned when parsing an [`EnvironmentType`] from a string.
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

impl FromStr for EnvironmentType {
    type Err = EnvironmentParseError;
    fn from_str(env: &str) -> Result<EnvironmentType, EnvironmentParseError> {
        match env.to_ascii_lowercase().as_str() {
            "non_production" => Ok(EnvironmentType::NonProduction),
            "simulation" => Ok(EnvironmentType::Simulation),
            "production" => Ok(EnvironmentType::Production),
            _ => Err(EnvironmentParseError::Invalid {
                input: env.to_string(),
            }),
        }
    }
}

impl EnvironmentType {
    /// Returns the canonical lowercase name (`non_production`, `simulation` or
    /// `production`), accepted by [`EnvironmentType::from_str`].
    pub fn as_str(&self) -> &'static str {
        match self {
            EnvironmentType::NonProduction => "non_production",
            EnvironmentType::Simulation => "simulation",
            EnvironmentType::Production => "production",
        }
    }

    /// Returns the base URL of this environment's ZATCA e-invoicing API, with a
    /// trailing slash.
    pub fn endpoint_url(&self) -> &'static str {
        match self {
            EnvironmentType::NonProduction => {
                "https://gw-fatoora.zatca.gov.sa/e-invoicing/developer-portal/"
            }
            EnvironmentType::Simulation => {
                "https://gw-fatoora.zatca.gov.sa/e-invoicing/simulation/"
            }
            EnvironmentType::Production => "https://gw-fatoora.zatca.gov.sa/e-invoicing/core/",
        }
    }
}

/// Configuration for validation and API clients.
///
/// # Examples
/// ```rust
/// use fatoora_core::config::{Config, EnvironmentType};
///
/// let config = Config::new(EnvironmentType::NonProduction);
/// # let _ = config;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Config {
    env: EnvironmentType,
}

impl Config {
    /// Create a config using the bundled UBL XSD.
    pub fn new(env: EnvironmentType) -> Self {
        Self { env }
    }

    /// Returns the configured ZATCA environment.
    pub fn env(&self) -> EnvironmentType {
        self.env
    }
}

// static function to get default config
impl Default for Config {
    fn default() -> Self {
        Config {
            env: EnvironmentType::NonProduction,
        }
    }
}
