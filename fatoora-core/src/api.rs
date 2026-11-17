//! ZATCA HTTP API client and response types.
use crate::{
    config::{Config, Environment},
    csr::Csr,
    invoice::SignedInvoice,
};
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;
mod prepared;
pub use prepared::{ComplianceRequest, HttpRequest, InvoiceRequest, ProductionRequest};
mod response;
use response::InvoiceOperation;
#[cfg(test)]
use response::read_validation_response;
pub use response::{
    ClearanceStatus, HttpResponseError, InvoiceOutcome, MessageType, ReportingStatus,
    ValidationStatus,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// Errors returned by the ZATCA API client.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ZatcaError {
    /// A non-success HTTP response. A duplicate submission can refer to an earlier acceptance.
    #[error("ZATCA returned HTTP {0}")]
    Response(Box<HttpResponseError>),
    /// A successful HTTP response whose body could not be decoded.
    #[error("Invalid ZATCA response (HTTP {response}): {message}")]
    ResponseDecode {
        /// The HTTP response whose body failed to decode.
        response: Box<HttpResponseError>,
        /// Description of the decoding failure.
        message: String,
    },
    /// The response headers arrived but the body could not be read.
    #[error("Failed to read ZATCA response (HTTP {http_status}): {message}")]
    ResponseRead {
        /// HTTP status code of the response.
        http_status: u16,
        /// Description of the read failure.
        message: String,
    },
    /// The endpoint response does not establish acceptance of this operation.
    #[error("ZATCA response does not confirm acceptance")]
    NotAccepted(Box<ValidationResponse>),
    /// The returned cleared invoice could not be decoded as nonempty UTF-8 XML text.
    #[error("Invalid cleared invoice: {message}")]
    ClearedInvoice {
        /// HTTP status code of the response, when known.
        http_status: Option<u16>,
        /// Description of the decoding failure.
        message: String,
    },
    /// A network-level failure. Reserved for bindings and callers; the client
    /// itself reports transport failures as [`ZatcaError::Http`].
    #[error("Network error: {0}")]
    NetworkError(String),
    /// A CSID endpoint returned an invalid HTTP status, an unexpected status,
    /// or a body that could not be decoded.
    #[error("Invalid response from ZATCA: {0}")]
    InvalidResponse(String),
    /// A CSID endpoint returned HTTP 401; carries the parsed (or synthesized) body.
    #[error("Unauthorized: {0:?}")]
    Unauthorized(UnauthorizedResponse),
    /// A CSID endpoint returned an HTTP 5xx status; carries the parsed (or
    /// synthesized) body.
    #[error("Server error: {0:?}")]
    ServerError(ServerErrorResponse),
    /// The HTTP client could not be built, the request method was invalid, or
    /// the request could not be sent.
    #[error("HTTP client error: {0}")]
    Http(String),
    /// The request was inconsistent with the client or credentials, e.g. an
    /// environment mismatch or a missing compliance request ID.
    #[error("Client state error: {0}")]
    ClientState(String),
}

impl ZatcaError {
    /// Actual HTTP status when available; never inferred from a JSON body.
    pub fn http_status(&self) -> Option<u16> {
        match self {
            Self::Response(response) | Self::ResponseDecode { response, .. } => {
                Some(response.http_status())
            }
            Self::ResponseRead { http_status, .. } => Some(*http_status),
            Self::NotAccepted(response) => response.http_status(),
            Self::ClearedInvoice { http_status, .. } => *http_status,
            _ => None,
        }
    }

    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        match self {
            Self::Response(response) if response.http_status() == 401 => {
                crate::ErrorKind::Unauthorized
            }
            Self::Response(_) | Self::NotAccepted(_) => crate::ErrorKind::Api,
            Self::ResponseDecode { .. } | Self::ClearedInvoice { .. } => crate::ErrorKind::Parse,
            Self::ResponseRead { .. } => crate::ErrorKind::Network,
            Self::NetworkError(_) => crate::ErrorKind::Network,
            Self::InvalidResponse(_) => crate::ErrorKind::Parse,
            Self::Unauthorized(_) => crate::ErrorKind::Unauthorized,
            Self::ServerError(_) => crate::ErrorKind::Api,
            Self::Http(_) => crate::ErrorKind::Network,
            Self::ClientState(_) => crate::ErrorKind::Internal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Compliance (CCSID) token scope.
pub struct Compliance;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Production (PCSID) token scope.
pub struct Production;

/// Value of the `Clearance-Status` request header sent with reporting and
/// clearance submissions.
///
/// [`Enabled`](Self::Enabled) sends `Clearance-Status: 1` and
/// [`Disabled`](Self::Disabled) sends `Clearance-Status: 0`. ZATCA uses this
/// header to tell whether the taxpayer's clearance flow is switched on.
///
/// # Examples
/// ```rust
/// use fatoora_core::api::ClearanceMode;
///
/// assert_eq!(ClearanceMode::from(true), ClearanceMode::Enabled);
/// assert_eq!(ClearanceMode::from(false), ClearanceMode::Disabled);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearanceMode {
    /// Clearance is enabled; sends `Clearance-Status: 1`.
    Enabled,
    /// Clearance is disabled; sends `Clearance-Status: 0`.
    Disabled,
}

impl ClearanceMode {
    /// The `Clearance-Status` header value for this mode.
    pub(crate) const fn header_value(self) -> &'static str {
        match self {
            Self::Enabled => "1",
            Self::Disabled => "0",
        }
    }
}

impl From<bool> for ClearanceMode {
    /// Maps `true` to [`ClearanceMode::Enabled`] and `false` to [`ClearanceMode::Disabled`].
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

/// ZATCA API client.
///
/// # Examples
/// ```rust,no_run
/// use fatoora_core::api::ZatcaClient;
/// use fatoora_core::config::Config;
///
/// let client = ZatcaClient::new(Config::default())?;
/// # let _ = client;
/// use fatoora_core::api::ZatcaError;
/// # Ok::<(), ZatcaError>(())
/// ```
#[derive(Debug)]
pub struct ZatcaClient {
    config: Config,
    #[cfg(not(target_arch = "wasm32"))]
    _client: Client,
    base_url: String,
}

/// API validation response.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidationResponse {
    // HTTP metadata is supplied by the client, never trusted from gateway JSON.
    #[serde(skip)]
    http_status: Option<u16>,
    #[serde(skip)]
    operation: Option<InvoiceOperation>,
    #[serde(rename = "clearedInvoice")]
    cleared_invoice: Option<String>,
    #[serde(rename = "validationResults")]
    validation_results: ValidationResults,
    #[serde(rename = "reportingStatus")]
    reporting_status: Option<ReportingStatus>,
    #[serde(rename = "clearanceStatus")]
    clearance_status: Option<ClearanceStatus>,
    #[serde(rename = "qrSellertStatus")]
    qr_seller_status: Option<String>,
    #[serde(rename = "qrBuyertStatus")]
    qr_buyer_status: Option<String>,
}

impl ValidationResponse {
    /// Returns the info, warning and error messages reported by ZATCA.
    pub fn validation_results(&self) -> &ValidationResults {
        &self.validation_results
    }

    /// Returns the `reportingStatus` (e.g. [`ReportingStatus::Reported`]),
    /// present for reporting responses.
    pub fn reporting_status(&self) -> Option<&ReportingStatus> {
        self.reporting_status.as_ref()
    }

    /// Returns the `clearanceStatus` (e.g. [`ClearanceStatus::Cleared`]),
    /// present for clearance responses.
    pub fn clearance_status(&self) -> Option<&ClearanceStatus> {
        self.clearance_status.as_ref()
    }

    /// Returns the `qrSellertStatus` (sic) field from the response, if present.
    pub fn qr_seller_status(&self) -> Option<&str> {
        self.qr_seller_status.as_deref()
    }

    /// Returns the `qrBuyertStatus` (sic) field from the response, if present.
    pub fn qr_buyer_status(&self) -> Option<&str> {
        self.qr_buyer_status.as_deref()
    }
}

/// Validation results container.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidationResults {
    #[serde(rename = "infoMessages", default)]
    info_messages: MessageList,
    #[serde(rename = "warningMessages", default)]
    warning_messages: Vec<ValidationMessage>,
    #[serde(rename = "errorMessages", default)]
    error_messages: Vec<ValidationMessage>,
    #[serde(default)]
    status: Option<ValidationStatus>,
}

impl ValidationResults {
    /// Returns the informational messages (`infoMessages`), which ZATCA may
    /// send as a single object or a list.
    pub fn info_messages(&self) -> &MessageList {
        &self.info_messages
    }

    /// Returns the warning messages (`warningMessages`).
    pub fn warning_messages(&self) -> &[ValidationMessage] {
        &self.warning_messages
    }

    /// Returns the error messages (`errorMessages`).
    pub fn error_messages(&self) -> &[ValidationMessage] {
        &self.error_messages
    }

    /// Returns the overall validation status (`PASS`, `WARNING`, `ERROR`), if present.
    pub fn status(&self) -> Option<&ValidationStatus> {
        self.status.as_ref()
    }
}

/// Validation message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidationMessage {
    #[serde(rename = "type")]
    message_type: Option<MessageType>,
    code: Option<String>,
    category: Option<String>,
    message: Option<String>,
    #[serde(default)]
    status: Option<ValidationStatus>,
}

impl ValidationMessage {
    /// Returns the message type (`type`: `INFO`, `WARNING`, `ERROR`), if present.
    pub fn message_type(&self) -> Option<&MessageType> {
        self.message_type.as_ref()
    }

    /// Returns the ZATCA rule or message code, if present.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the message category (e.g. `XSD_SCHEMA_ERROR`, `KSA`), if present.
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    /// Returns the human-readable message text, if present.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the per-message status (`PASS`, `WARNING`, `ERROR`), if present.
    pub fn status(&self) -> Option<&ValidationStatus> {
        self.status.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Default)]
/// Message list returned by the API.
pub enum MessageList {
    /// A single message object.
    One(ValidationMessage),
    /// A list of messages.
    Many(Vec<ValidationMessage>),
    /// No messages (field absent or `null`).
    #[default]
    Empty,
}

/// Unauthorized response body.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnauthorizedResponse {
    timestamp: Option<i64>,
    status: Option<u16>,
    error: Option<String>,
    message: Option<String>,
}

impl UnauthorizedResponse {
    /// Returns the `timestamp` field (as sent by the gateway), if present.
    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    /// Returns the HTTP status reported in the body, if present.
    pub fn status(&self) -> Option<u16> {
        self.status
    }

    /// Returns the short error label (e.g. `Unauthorized`), if present.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Returns the error message, or the raw body when it was not valid JSON.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

/// Server error response body.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServerErrorResponse {
    category: Option<String>,
    code: Option<String>,
    message: Option<String>,
}

impl ServerErrorResponse {
    /// Returns the error category, if present.
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    /// Returns the error code (`ServerError` when the body was not valid JSON).
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the error message, or the raw body when it was not valid JSON.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

/// CSID credentials used for API calls.
/// This is usually obtained after requesting a CSID from ZATCA.
/// ie. through [request_production_csid][ZatcaClient::request_production_csid] or [request_compliance_csid][ZatcaClient::request_compliance_csid].
/// But can also be constructed manually if you have the necessary values.
///
/// # Examples
/// ```rust
/// use fatoora_core::api::{CsidCredentials, Compliance};
/// use fatoora_core::config::Environment;
///
/// let creds = CsidCredentials::<Compliance>::new(
///     Environment::NonProduction,
///     Some("1234567890123".to_string()), // requestID field
///     "TUlJQ1BUQ0NBZU9nQXdJQkFnS....",   // binarySecurityToken field
///     "Dehvg1fc8GF6Jwt5bOxXwC6en....",   // secret field
/// );
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CsidCredentials<T> {
    env: Environment,
    request_id: Option<String>,
    binary_security_token: String,
    secret: String,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> CsidCredentials<T> {
    /// Create credential bundle for ZATCA requests.
    pub fn new(
        env: Environment,
        request_id: Option<String>,
        binary_security_token: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        Self {
            env,
            request_id,
            binary_security_token: binary_security_token.into(),
            secret: secret.into(),
            _marker: PhantomData,
        }
    }

    /// Returns the environment these credentials were issued for.
    pub fn env(&self) -> Environment {
        self.env
    }

    /// Returns the ZATCA `requestID`, required to exchange a compliance CSID
    /// for a production CSID.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Returns the `binarySecurityToken` (base64-encoded certificate), used as
    /// the basic-auth username.
    pub fn binary_security_token(&self) -> &str {
        &self.binary_security_token
    }

    /// Returns the API secret, used as the basic-auth password.
    pub fn secret(&self) -> &str {
        &self.secret
    }
}

/// Compliance CSID (CCSID) credentials, as returned by
/// [`ZatcaClient::request_compliance_csid`].
///
/// Used to run compliance checks and to request a production CSID.
pub type ComplianceCsid = CsidCredentials<Compliance>;

/// Production CSID (PCSID) credentials, as returned by
/// [`ZatcaClient::request_production_csid`] and [`ZatcaClient::renew_csid`].
///
/// Used to report and clear invoices.
pub type ProductionCsid = CsidCredentials<Production>;

/// Placeholder printed in place of credential material in `Debug` output.
const REDACTED: &str = "<redacted>";

// Manual impl: a derived `Debug` would print the binary security token and secret.
impl<T> std::fmt::Debug for CsidCredentials<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsidCredentials")
            .field("env", &self.env)
            .field("request_id", &self.request_id)
            .field("binary_security_token", &format_args!("{REDACTED}"))
            .field("secret", &format_args!("{REDACTED}"))
            .finish()
    }
}

#[derive(Deserialize)]
struct CsidResponseBody {
    #[serde(rename = "requestID")]
    #[serde(deserialize_with = "deserialize_request_id")]
    request_id: Option<String>,
    #[serde(rename = "binarySecurityToken")]
    binary_security_token: String,
    secret: String,
    #[allow(dead_code)]
    #[serde(rename = "tokenType")]
    token_type: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "dispositionMessage")]
    disposition_message: Option<String>,
}

// Manual impl: a derived `Debug` would print the binary security token and secret.
impl std::fmt::Debug for CsidResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsidResponseBody")
            .field("request_id", &self.request_id)
            .field("binary_security_token", &format_args!("{REDACTED}"))
            .field("secret", &format_args!("{REDACTED}"))
            .finish_non_exhaustive()
    }
}

fn deserialize_request_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::String(value)) => Ok(Some(value)),
        Some(serde_json::Value::Number(value)) => Ok(Some(value.to_string())),
        Some(other) => Err(serde::de::Error::custom(format!(
            "invalid requestID type: {other}"
        ))),
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RenewalResponseBody {
    Direct(CsidResponseBody),
    Wrapped { value: CsidResponseBody },
}

// Public API
impl ZatcaClient {
    /// Create a new API client using the provided configuration.
    ///
    /// # Errors
    /// Returns [`ZatcaError::Http`] if the HTTP client cannot be built.
    pub fn new(config: Config) -> Result<Self, ZatcaError> {
        #[cfg(not(target_arch = "wasm32"))]
        let builder = Client::builder().redirect(reqwest::redirect::Policy::none());
        #[cfg(target_os = "android")]
        let builder = {
            let roots = rustls::RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
            };
            let tls = rustls::ClientConfig::builder_with_provider(std::sync::Arc::new(
                rustls::crypto::ring::default_provider(),
            ))
            .with_safe_default_protocol_versions()
            .map_err(|e| ZatcaError::Http(e.to_string()))?
            .with_root_certificates(roots)
            .with_no_client_auth();
            builder.tls_backend_preconfigured(tls)
        };
        #[cfg(not(target_arch = "wasm32"))]
        let client = builder
            .build()
            .map_err(|e| ZatcaError::Http(e.to_string()))?;
        let base_url = std::env::var("FATOORA_ZATCA_BASE_URL")
            .ok()
            .map(|value| {
                if value.ends_with('/') {
                    value
                } else {
                    format!("{value}/")
                }
            })
            .unwrap_or_else(|| config.env().endpoint_url().to_string());

        Ok(Self {
            config,
            #[cfg(not(target_arch = "wasm32"))]
            _client: client,
            base_url,
        })
    }

    /// Report a simplified invoice to ZATCA's gateway.
    /// See [ZATCA documentation](https://sandbox.zatca.gov.sa/IntegrationSandbox/reporting-api) for more details.
    ///
    /// Prepare an owned request without sending it; decode through the returned request.
    ///
    /// # Errors
    /// Returns [`ZatcaError`] for invalid credentials, invoice scope, or request data.
    pub fn prepare_report_simplified_invoice(
        &self,
        invoice: &SignedInvoice,
        credentials: &CsidCredentials<Production>,
        clearance_mode: ClearanceMode,
        accept_language: Option<&str>,
    ) -> Result<InvoiceRequest, ZatcaError> {
        self.ensure_env(credentials)?;
        if !invoice.data().invoice_type().is_simplified() {
            return Err(ZatcaError::ClientState(
                "Reporting only supports simplified invoices".into(),
            ));
        }

        let payload = serde_json::json!({
            "invoiceHash": invoice.invoice_hash(),
            "uuid": invoice.uuid(),
            "invoice": invoice.to_xml_base64()
        });
        let url = self.build_endpoint("invoices/reporting/single");
        let mut request = HttpRequest::new("POST", url)
            .header("Accept", "application/json")
            .header("Accept-Version", "V2")
            .header("Content-Type", "application/json")
            .header("Clearance-Status", clearance_mode.header_value())
            .basic_auth(
                credentials.binary_security_token().to_string(),
                Some(credentials.secret().to_string()),
            )
            .json(&payload);

        match accept_language {
            Some("ar") => request = request.header("accept-language", "ar"),
            _ => request = request.header("accept-language", "en"),
        }

        Ok(InvoiceRequest::new(
            request.finish()?,
            InvoiceOperation::Reporting,
        ))
    }

    /// Clear a standard invoice through ZATCA's gateway.
    /// See [ZATCA documentation](https://sandbox.zatca.gov.sa/Integration/clearance-api) for more details.
    ///
    /// Prepare an owned request without sending it; decode through the returned request.
    ///
    /// # Errors
    /// Returns [`ZatcaError`] for invalid credentials, invoice scope, or request data.
    pub fn prepare_clear_standard_invoice(
        &self,
        invoice: &SignedInvoice,
        credentials: &CsidCredentials<Production>,
        clearance_mode: ClearanceMode,
        accept_language: Option<&str>,
    ) -> Result<InvoiceRequest, ZatcaError> {
        self.ensure_env(credentials)?;
        if invoice.data().invoice_type().is_simplified() {
            return Err(ZatcaError::ClientState(
                "Clearance only supports standard invoices".into(),
            ));
        }

        let payload = serde_json::json!({
            "invoiceHash": invoice.invoice_hash(),
            "uuid": invoice.uuid(),
            "invoice": invoice.to_xml_base64()
        });
        let url = self.build_endpoint("invoices/clearance/single");
        let mut request = HttpRequest::new("POST", url)
            .header("Accept", "application/json")
            .header("Accept-Version", "V2")
            .header("Content-Type", "application/json")
            .header("Clearance-Status", clearance_mode.header_value())
            .basic_auth(
                credentials.binary_security_token().to_string(),
                Some(credentials.secret().to_string()),
            )
            .json(&payload);

        match accept_language {
            Some("ar") => request = request.header("accept-language", "ar"),
            _ => request = request.header("accept-language", "en"),
        }

        Ok(InvoiceRequest::new(
            request.finish()?,
            InvoiceOperation::Clearance,
        ))
    }

    /// Check invoice compliance through ZATCA's gateway.
    /// See [ZATCA documentation](https://sandbox.zatca.gov.sa/IntegrationSandbox/preInvoice-api) for more details.
    ///
    /// Prepare an owned request without sending it; decode through the returned request.
    ///
    /// # Errors
    /// Returns [`ZatcaError`] for invalid credentials, invoice scope, or request data.
    pub fn prepare_check_invoice_compliance(
        &self,
        invoice: &SignedInvoice,
        credentials: &CsidCredentials<Compliance>,
    ) -> Result<InvoiceRequest, ZatcaError> {
        self.ensure_env(credentials)?;
        let payload = serde_json::json!({
            "invoiceHash": invoice.invoice_hash(),
            "uuid": invoice.uuid(),
            "invoice": invoice.to_xml_base64()
        });

        let url = self.build_endpoint("compliance/invoices");

        let request = HttpRequest::new("POST", url)
            .header("Accept", "application/json")
            .header("Accept-Language", "en")
            .header("Accept-Version", "V2")
            .header("Content-Type", "application/json")
            .basic_auth(
                credentials.binary_security_token().to_string(),
                Some(credentials.secret().to_string()),
            )
            .json(&payload)
            .finish()?;
        Ok(InvoiceRequest::new(request, InvoiceOperation::Compliance))
    }
    /// Request a compliance CSID from ZATCA by submitting a CSR.
    /// See [ZATCA
    /// documentation](https://sandbox.zatca.gov.sa/IntegrationSandbox/complianceCert-api) for more details.
    ///
    /// Prepare an owned request without sending it; decode through the returned request.
    ///
    /// # Errors
    /// Returns [`ZatcaError`] if request data cannot be prepared.
    pub fn prepare_request_compliance_csid(
        &self,
        csr: &Csr,
        otp: &str,
    ) -> Result<ComplianceRequest, ZatcaError> {
        let encoded_csr = csr
            .to_pem_base64()
            .map_err(|e| ZatcaError::InvalidResponse(e.to_string()))?;
        let csr_payload = serde_json::json!({ "csr": encoded_csr });
        let url = self.build_endpoint("compliance");
        let request = HttpRequest::new("POST", url)
            .header("Accept", "application/json")
            .header("OTP", otp)
            .header("Accept-Version", "V2")
            .header("Content-Type", "application/json")
            .json(&csr_payload)
            .finish()?;
        Ok(ComplianceRequest::new(request, self.config.env()))
    }

    /// Requests a production CSID from ZATCA using a compliance CSID previously obtained e.g. from [request_compliance_csid][ZatcaClient::request_compliance_csid].
    /// See [ZATCA documentation](https://sandbox.zatca.gov.sa/Integration/request-api) for more details.
    ///
    /// Prepare an owned request without sending it; decode through the returned request.
    ///
    /// # Errors
    /// Returns [`ZatcaError`] if the request fails or the compliance CSID is missing data.
    pub fn prepare_request_production_csid(
        &self,
        ccsid: &CsidCredentials<Compliance>,
    ) -> Result<ProductionRequest, ZatcaError> {
        self.ensure_env(ccsid)?;
        let request_id = ccsid
            .request_id()
            .ok_or_else(|| ZatcaError::ClientState("Missing compliance request_id".into()))?;
        let payload = serde_json::json!({
            "compliance_request_id": request_id,
        });

        let url = self.build_endpoint("production/csids");
        let request = HttpRequest::new("POST", url)
            .header("Accept", "application/json")
            .header("Accept-Version", "V2")
            .header("Content-Type", "application/json")
            .basic_auth(
                ccsid.binary_security_token().to_string(),
                Some(ccsid.secret().to_string()),
            )
            .json(&payload)
            .finish()?;
        Ok(ProductionRequest::new(request, self.config.env(), false))
    }

    /// Former name of [`prepare_request_compliance_csid`](Self::prepare_request_compliance_csid).
    ///
    /// # Errors
    /// Same as [`prepare_request_compliance_csid`](Self::prepare_request_compliance_csid).
    #[deprecated(
        since = "0.6.0",
        note = "use `prepare_request_compliance_csid` instead"
    )]
    pub fn prepare_post_csr_for_ccsid(
        &self,
        csr: &Csr,
        otp: &str,
    ) -> Result<ComplianceRequest, ZatcaError> {
        self.prepare_request_compliance_csid(csr, otp)
    }

    /// Former name of [`prepare_request_production_csid`](Self::prepare_request_production_csid).
    ///
    /// # Errors
    /// Same as [`prepare_request_production_csid`](Self::prepare_request_production_csid).
    #[deprecated(
        since = "0.6.0",
        note = "use `prepare_request_production_csid` instead"
    )]
    pub fn prepare_post_ccsid_for_pcsid(
        &self,
        ccsid: &CsidCredentials<Compliance>,
    ) -> Result<ProductionRequest, ZatcaError> {
        self.prepare_request_production_csid(ccsid)
    }

    /// Renew a production CSID by submitting a new CSR.
    /// See [ZATCA documentation](https://sandbox.zatca.gov.sa/Integration/renewal-api) for more details.
    ///
    /// Prepare an owned request without sending it; decode through the returned request.
    ///
    /// # Errors
    /// Returns [`ZatcaError`] if request data cannot be prepared.
    pub fn prepare_renew_csid(
        &self,
        pcsid: &CsidCredentials<Production>,
        csr: &Csr,
        otp: &str,
        accept_language: Option<&str>,
    ) -> Result<ProductionRequest, ZatcaError> {
        self.ensure_env(pcsid)?;
        let encoded_csr = csr
            .to_pem_base64()
            .map_err(|e| ZatcaError::InvalidResponse(e.to_string()))?;
        let csr_payload = serde_json::json!({ "csr": encoded_csr });
        let url = self.build_endpoint("production/csids");
        let mut request = HttpRequest::new("PATCH", url)
            .header("Accept", "application/json")
            .header("OTP", otp)
            .header("Accept-Version", "V2")
            .header("Content-Type", "application/json")
            .basic_auth(
                pcsid.binary_security_token().to_string(),
                Some(pcsid.secret().to_string()),
            )
            .json(&csr_payload);

        match accept_language {
            Some("ar") => request = request.header("accept-language", "ar"),
            _ => request = request.header("accept-language", "en"),
        }

        Ok(ProductionRequest::new(
            request.finish()?,
            self.config.env(),
            true,
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
enum BodyReadPolicy {
    Invoice,
    Issuance,
    Renewal,
}

#[cfg(not(target_arch = "wasm32"))]
impl ZatcaClient {
    async fn send(
        &self,
        request: &HttpRequest,
        policy: BodyReadPolicy,
    ) -> Result<(u16, Vec<u8>), ZatcaError> {
        let method = reqwest::Method::from_bytes(request.method().as_bytes())
            .map_err(|e| ZatcaError::Http(e.to_string()))?;
        let mut builder = self._client.request(method, request.url());
        for (name, value) in request.headers() {
            builder = builder.header(name, value);
        }
        let response = builder
            .body(request.body().to_vec())
            .send()
            .await
            .map_err(|e| ZatcaError::Http(e.to_string()))?;
        let status = response.status().as_u16();
        let body = match response.bytes().await {
            Ok(body) => body.to_vec(),
            Err(error) => match policy {
                // Preserve the existing native CSID endpoint error contracts.
                BodyReadPolicy::Renewal => Vec::new(),
                BodyReadPolicy::Issuance if !(200..300).contains(&status) => Vec::new(),
                BodyReadPolicy::Issuance => {
                    return Err(ZatcaError::InvalidResponse(error.to_string()));
                }
                BodyReadPolicy::Invoice => {
                    return Err(ZatcaError::ResponseRead {
                        http_status: status,
                        message: error.to_string(),
                    });
                }
            },
        };
        Ok((status, body))
    }
}

impl ZatcaClient {
    #[cfg(not(target_arch = "wasm32"))]
    /// Prepare, send through the native HTTP client, and decode the response.
    ///
    /// A decoded 2xx response is returned even if ZATCA did not accept the
    /// invoice; check [`ValidationResponse::outcome`] or call
    /// [`ValidationResponse::ensure_accepted`].
    ///
    /// # Errors
    /// - [`ZatcaError::ClientState`] if the credentials' environment differs from
    ///   the client's, or (reporting/clearance only) the invoice type does not
    ///   match the endpoint.
    /// - [`ZatcaError::Http`] if the request cannot be built or sent.
    /// - [`ZatcaError::ResponseRead`] if the response body cannot be read.
    /// - [`ZatcaError::Response`] for a non-2xx HTTP status.
    /// - [`ZatcaError::ResponseDecode`] if a 2xx body is not a valid validation response.
    pub async fn report_simplified_invoice(
        &self,
        invoice: &SignedInvoice,
        credentials: &CsidCredentials<Production>,
        clearance_mode: ClearanceMode,
        accept_language: Option<&str>,
    ) -> Result<ValidationResponse, ZatcaError> {
        let prepared = self.prepare_report_simplified_invoice(
            invoice,
            credentials,
            clearance_mode,
            accept_language,
        )?;
        let (status, body) = self
            .send(prepared.request(), BodyReadPolicy::Invoice)
            .await?;
        prepared.decode(status, &body)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prepare, send through the native HTTP client, and decode the response.
    ///
    /// A decoded 2xx response is returned even if ZATCA did not accept the
    /// invoice; check [`ValidationResponse::outcome`] or call
    /// [`ValidationResponse::ensure_accepted`].
    ///
    /// # Errors
    /// - [`ZatcaError::ClientState`] if the credentials' environment differs from
    ///   the client's, or (reporting/clearance only) the invoice type does not
    ///   match the endpoint.
    /// - [`ZatcaError::Http`] if the request cannot be built or sent.
    /// - [`ZatcaError::ResponseRead`] if the response body cannot be read.
    /// - [`ZatcaError::Response`] for a non-2xx HTTP status.
    /// - [`ZatcaError::ResponseDecode`] if a 2xx body is not a valid validation response.
    pub async fn clear_standard_invoice(
        &self,
        invoice: &SignedInvoice,
        credentials: &CsidCredentials<Production>,
        clearance_mode: ClearanceMode,
        accept_language: Option<&str>,
    ) -> Result<ValidationResponse, ZatcaError> {
        let prepared = self.prepare_clear_standard_invoice(
            invoice,
            credentials,
            clearance_mode,
            accept_language,
        )?;
        let (status, body) = self
            .send(prepared.request(), BodyReadPolicy::Invoice)
            .await?;
        prepared.decode(status, &body)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prepare, send through the native HTTP client, and decode the response.
    ///
    /// A decoded 2xx response is returned even if ZATCA did not accept the
    /// invoice; check [`ValidationResponse::outcome`] or call
    /// [`ValidationResponse::ensure_accepted`].
    ///
    /// # Errors
    /// - [`ZatcaError::ClientState`] if the credentials' environment differs from
    ///   the client's, or (reporting/clearance only) the invoice type does not
    ///   match the endpoint.
    /// - [`ZatcaError::Http`] if the request cannot be built or sent.
    /// - [`ZatcaError::ResponseRead`] if the response body cannot be read.
    /// - [`ZatcaError::Response`] for a non-2xx HTTP status.
    /// - [`ZatcaError::ResponseDecode`] if a 2xx body is not a valid validation response.
    pub async fn check_invoice_compliance(
        &self,
        invoice: &SignedInvoice,
        credentials: &CsidCredentials<Compliance>,
    ) -> Result<ValidationResponse, ZatcaError> {
        let prepared = self.prepare_check_invoice_compliance(invoice, credentials)?;
        let (status, body) = self
            .send(prepared.request(), BodyReadPolicy::Invoice)
            .await?;
        prepared.decode(status, &body)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prepare, send through the native HTTP client, and decode the response.
    ///
    /// # Errors
    /// - [`ZatcaError::InvalidResponse`] if the CSR cannot be PEM-encoded.
    /// - [`ZatcaError::Http`] if the request cannot be built or sent.
    /// - [`ZatcaError::Unauthorized`] for HTTP 401 and [`ZatcaError::ServerError`]
    ///   for HTTP 5xx.
    /// - [`ZatcaError::InvalidResponse`] for any other unexpected status or an
    ///   undecodable body.
    pub async fn request_compliance_csid(
        &self,
        csr: &Csr,
        otp: &str,
    ) -> Result<CsidCredentials<Compliance>, ZatcaError> {
        let prepared = self.prepare_request_compliance_csid(csr, otp)?;
        let (status, body) = self
            .send(prepared.request(), BodyReadPolicy::Issuance)
            .await?;
        prepared.decode(status, &body)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prepare, send through the native HTTP client, and decode the response.
    ///
    /// # Errors
    /// - [`ZatcaError::ClientState`] if the credentials' environment differs from
    ///   the client's or they have no compliance request ID.
    /// - [`ZatcaError::Http`] if the request cannot be built or sent.
    /// - [`ZatcaError::Unauthorized`] for HTTP 401 and [`ZatcaError::ServerError`]
    ///   for HTTP 5xx.
    /// - [`ZatcaError::InvalidResponse`] for any other unexpected status or an
    ///   undecodable body.
    pub async fn request_production_csid(
        &self,
        ccsid: &CsidCredentials<Compliance>,
    ) -> Result<CsidCredentials<Production>, ZatcaError> {
        let prepared = self.prepare_request_production_csid(ccsid)?;
        let (status, body) = self
            .send(prepared.request(), BodyReadPolicy::Issuance)
            .await?;
        prepared.decode(status, &body)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Former name of [`request_compliance_csid`](Self::request_compliance_csid).
    ///
    /// # Errors
    /// Same as [`request_compliance_csid`](Self::request_compliance_csid).
    #[deprecated(since = "0.6.0", note = "use `request_compliance_csid` instead")]
    pub async fn post_csr_for_ccsid(
        &self,
        csr: &Csr,
        otp: &str,
    ) -> Result<CsidCredentials<Compliance>, ZatcaError> {
        self.request_compliance_csid(csr, otp).await
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Former name of [`request_production_csid`](Self::request_production_csid).
    ///
    /// # Errors
    /// Same as [`request_production_csid`](Self::request_production_csid).
    #[deprecated(since = "0.6.0", note = "use `request_production_csid` instead")]
    pub async fn post_ccsid_for_pcsid(
        &self,
        ccsid: &CsidCredentials<Compliance>,
    ) -> Result<CsidCredentials<Production>, ZatcaError> {
        self.request_production_csid(ccsid).await
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prepare, send through the native HTTP client, and decode the response.
    ///
    /// # Errors
    /// - [`ZatcaError::ClientState`] if the credentials' environment differs from
    ///   the client's.
    /// - [`ZatcaError::InvalidResponse`] if the CSR cannot be PEM-encoded.
    /// - [`ZatcaError::Http`] if the request cannot be built or sent.
    /// - [`ZatcaError::Unauthorized`] for HTTP 401 and [`ZatcaError::ServerError`]
    ///   for HTTP 5xx.
    /// - [`ZatcaError::InvalidResponse`] for any other unexpected status or an
    ///   undecodable body. An HTTP 428 body carrying credentials is treated as
    ///   success.
    pub async fn renew_csid(
        &self,
        pcsid: &CsidCredentials<Production>,
        csr: &Csr,
        otp: &str,
        accept_language: Option<&str>,
    ) -> Result<CsidCredentials<Production>, ZatcaError> {
        let prepared = self.prepare_renew_csid(pcsid, csr, otp, accept_language)?;
        let (status, body) = self
            .send(prepared.request(), BodyReadPolicy::Renewal)
            .await?;
        prepared.decode(status, &body)
    }
}

// Private API
impl ZatcaClient {
    fn build_endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path.trim_start_matches('/'))
    }

    fn ensure_env<T>(&self, creds: &CsidCredentials<T>) -> Result<(), ZatcaError> {
        if creds.env() != self.config.env() {
            return Err(ZatcaError::ClientState("CSID environment mismatch".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        csr::CsrProperties,
        invoice::{
            Address, CountryCode, InvoiceBuilder, InvoiceSubType, InvoiceType, LineItem, Party,
            SellerRole, VatCategory,
        },
    };
    use httpmock::{Method::PATCH, Method::POST, MockServer};
    use std::path::Path;

    const TEST_TOKEN: &str = "TUlJQ1BUQ0NBZU9nQXdJQkFnSTOKEN";
    const TEST_SECRET: &str = "Dehvg1fc8GF6Jwt5bOxXwC6enSECRET";

    fn assert_redacted(rendered: &str) {
        assert!(!rendered.contains(TEST_TOKEN), "token leaked: {rendered}");
        assert!(!rendered.contains(TEST_SECRET), "secret leaked: {rendered}");
        assert!(
            rendered.contains("<redacted>"),
            "missing marker: {rendered}"
        );
    }

    #[test]
    fn csid_credentials_debug_redacts_token_and_secret() {
        let compliance = CsidCredentials::<Compliance>::new(
            Environment::Simulation,
            Some("1234567890123".into()),
            TEST_TOKEN,
            TEST_SECRET,
        );
        let production = CsidCredentials::<Production>::new(
            Environment::Production,
            None,
            TEST_TOKEN,
            TEST_SECRET,
        );
        for rendered in [
            format!("{compliance:?}"),
            format!("{compliance:#?}"),
            format!("{production:?}"),
            format!("{production:#?}"),
        ] {
            assert_redacted(&rendered);
            assert!(rendered.contains("CsidCredentials"));
        }
        let rendered = format!("{compliance:?}");
        assert!(rendered.contains("Simulation"));
        assert!(rendered.contains("1234567890123"));
        // Accessors still expose the real values.
        assert_eq!(compliance.binary_security_token(), TEST_TOKEN);
        assert_eq!(compliance.secret(), TEST_SECRET);
    }

    #[test]
    fn csid_response_body_debug_redacts_token_and_secret() {
        let body = format!(
            r#"{{"requestID":42,"binarySecurityToken":"{TEST_TOKEN}","secret":"{TEST_SECRET}"}}"#
        );
        let direct: CsidResponseBody = serde_json::from_str(&body).unwrap();
        let wrapped: RenewalResponseBody =
            serde_json::from_str(&format!(r#"{{"value":{body}}}"#)).unwrap();
        for rendered in [
            format!("{direct:?}"),
            format!("{direct:#?}"),
            format!("{wrapped:?}"),
            format!("{wrapped:#?}"),
        ] {
            assert_redacted(&rendered);
        }
    }

    #[test]
    fn zatca_client_debug_contains_no_credentials() {
        let client = ZatcaClient::new(Config::default()).unwrap();
        for rendered in [format!("{client:?}"), format!("{client:#?}")] {
            assert!(!rendered.contains("Authorization"));
            assert!(!rendered.contains("secret"));
        }
    }

    #[test]
    fn deserialize_validation_response_with_info_object() {
        let payload = r#"{
          "validationResults": {
            "infoMessages": {
              "type": "INFO",
              "code": "XSD_ZATCA_VALID",
              "category": "XSD validation",
              "message": "Complied with UBL 2.1 standards in line with ZATCA specifications",
              "status": "PASS"
            }
          },
          "warningMessages": [],
          "errorMessages": [],
          "status": "PASS",
          "reportingStatus": "REPORTED",
          "clearanceStatus": null,
          "qrSellertStatus": null,
          "qrBuyertStatus": null
        }"#;

        let parsed: ValidationResponse = serde_json::from_str(payload).expect("deserialize");
        match parsed.validation_results.info_messages {
            MessageList::One(msg) => assert_eq!(msg.code.as_deref(), Some("XSD_ZATCA_VALID")),
            other => panic!("expected single info message, got {:?}", other),
        }
    }

    #[test]
    fn deserialize_validation_response_with_info_array() {
        let payload = r#"{
          "validationResults": {
            "infoMessages": [
              {
                "type": "INFO",
                "code": "XSD_ZATCA_VALID",
                "category": "XSD validation",
                "message": "Complied with UBL 2.1 standards in line with ZATCA specifications",
                "status": "PASS"
              }
            ],
            "warningMessages": [],
            "errorMessages": [
              {
                "type": "ERROR",
                "code": "BR-KSA-37",
                "category": "KSA",
                "message": "The seller address building number must contain 4 digits.",
                "status": "ERROR"
              }
            ],
            "status": "ERROR"
          },
          "reportingStatus": "NOT_REPORTED",
          "clearanceStatus": null,
          "qrSellertStatus": null,
          "qrBuyertStatus": null
        }"#;

        let parsed: ValidationResponse = serde_json::from_str(payload).expect("deserialize");
        match parsed.validation_results.info_messages {
            MessageList::Many(list) => assert_eq!(list.len(), 1),
            other => panic!("expected info list, got {:?}", other),
        }
        assert_eq!(parsed.validation_results.error_messages.len(), 1);
    }

    #[test]
    fn deserialize_renewal_response_wrapped() {
        let payload = r#"{
          "value": {
            "requestID": 1234567890,
            "tokenType": null,
            "dispositionMessage": "NOT_COMPLIANT",
            "binarySecurityToken": "token",
            "secret": "secret",
            "errors": null
          }
        }"#;

        let parsed: RenewalResponseBody = serde_json::from_str(payload).expect("deserialize");
        let value = match parsed {
            RenewalResponseBody::Wrapped { value } => value,
            RenewalResponseBody::Direct(value) => value,
        };
        assert_eq!(value.request_id.as_deref(), Some("1234567890"));
        assert_eq!(value.binary_security_token, "token");
        assert_eq!(value.secret, "secret");
    }

    #[test]
    fn deserialize_validation_response_defaults_info_messages() {
        let payload = r#"{
          "validationResults": {
            "warningMessages": [],
            "errorMessages": [],
            "status": "PASS"
          },
          "reportingStatus": "REPORTED",
          "clearanceStatus": null,
          "qrSellertStatus": null,
          "qrBuyertStatus": null
        }"#;

        let parsed: ValidationResponse = serde_json::from_str(payload).expect("deserialize");
        assert!(matches!(
            parsed.validation_results.info_messages,
            MessageList::Empty
        ));
    }

    #[test]
    fn csid_credentials_new_stores_env() {
        let creds = CsidCredentials::<Compliance>::new(
            Environment::Simulation,
            Some("10".into()),
            "token",
            "secret",
        );
        assert_eq!(creds.env(), Environment::Simulation);
        assert_eq!(creds.request_id(), Some("10"));
    }

    #[test]
    fn response_getters_expose_fields() {
        let payload = r#"{
          "validationResults": {
            "infoMessages": {
              "type": "INFO",
              "code": "INFO_CODE",
              "category": "Info",
              "message": "ok",
              "status": "PASS"
            },
            "warningMessages": [
              {
                "type": "WARN",
                "code": "WARN_CODE",
                "category": "Warn",
                "message": "warn",
                "status": "WARN"
              }
            ],
            "errorMessages": [],
            "status": "PASS"
          },
          "reportingStatus": "REPORTED",
          "clearanceStatus": "CLEARED",
          "qrSellertStatus": "OK",
          "qrBuyertStatus": "OK"
        }"#;

        let parsed: ValidationResponse = serde_json::from_str(payload).expect("deserialize");
        assert_eq!(parsed.reporting_status(), Some(&ReportingStatus::Reported));
        assert_eq!(parsed.clearance_status(), Some(&ClearanceStatus::Cleared));
        assert_eq!(parsed.qr_seller_status(), Some("OK"));
        assert_eq!(parsed.qr_buyer_status(), Some("OK"));

        let results = parsed.validation_results();
        assert_eq!(results.status(), Some(&ValidationStatus::Pass));
        assert_eq!(results.warning_messages().len(), 1);
        let warning = &results.warning_messages()[0];
        assert_eq!(
            warning.message_type(),
            Some(&MessageType::Other("WARN".into()))
        );
        assert_eq!(warning.status().map(ValidationStatus::as_str), Some("WARN"));
        assert_eq!(results.error_messages().len(), 0);

        match results.info_messages() {
            MessageList::One(message) => {
                assert_eq!(message.message_type(), Some(&MessageType::Info));
                assert_eq!(message.code(), Some("INFO_CODE"));
                assert_eq!(message.category(), Some("Info"));
                assert_eq!(message.message(), Some("ok"));
                assert_eq!(message.status(), Some(&ValidationStatus::Pass));
            }
            _ => panic!("expected info message"),
        }
    }

    #[test]
    fn error_response_getters_expose_fields() {
        let unauthorized = UnauthorizedResponse {
            timestamp: Some(1),
            status: Some(401),
            error: Some("Unauthorized".into()),
            message: Some("nope".into()),
        };
        assert_eq!(unauthorized.timestamp(), Some(1));
        assert_eq!(unauthorized.status(), Some(401));
        assert_eq!(unauthorized.error(), Some("Unauthorized"));
        assert_eq!(unauthorized.message(), Some("nope"));

        let server_error = ServerErrorResponse {
            category: Some("Server".into()),
            code: Some("ERR".into()),
            message: Some("boom".into()),
        };
        assert_eq!(server_error.category(), Some("Server"));
        assert_eq!(server_error.code(), Some("ERR"));
        assert_eq!(server_error.message(), Some("boom"));
    }

    #[test]
    fn build_endpoint_trims_leading_slash() {
        let client = ZatcaClient::new(Config::default()).expect("client");
        let with_slash = client.build_endpoint("/invoices/reporting/single");
        let without_slash = client.build_endpoint("invoices/reporting/single");
        assert_eq!(with_slash, without_slash);
    }

    #[test]
    fn ensure_env_rejects_mismatch() {
        let client = ZatcaClient::new(Config::default()).expect("client");
        let creds =
            CsidCredentials::<Compliance>::new(Environment::Production, None, "token", "secret");
        let err = client.ensure_env(&creds).expect_err("env mismatch");
        assert!(matches!(err, ZatcaError::ClientState(_)));
    }

    #[tokio::test]
    async fn report_rejects_standard_invoice() {
        let signed_invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Standard));

        let creds = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
        let client = ZatcaClient::new(Config::default()).expect("client");

        let result = client
            .report_simplified_invoice(&signed_invoice, &creds, ClearanceMode::Disabled, None)
            .await;
        assert!(matches!(result, Err(ZatcaError::ClientState(_))));
    }

    #[tokio::test]
    async fn clearance_rejects_simplified_invoice() {
        let signed_invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));

        let creds = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
        let client = ZatcaClient::new(Config::default()).expect("client");

        let result = client
            .clear_standard_invoice(&signed_invoice, &creds, ClearanceMode::Enabled, None)
            .await;
        assert!(matches!(result, Err(ZatcaError::ClientState(_))));
    }

    #[tokio::test]
    async fn compliance_rejects_env_mismatch() {
        let signed_invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
        let client = ZatcaClient::new(Config::default()).expect("client");
        let creds = CsidCredentials::new(Environment::Production, None, "token", "secret");

        let result = client
            .check_invoice_compliance(&signed_invoice, &creds)
            .await;
        assert!(matches!(result, Err(ZatcaError::ClientState(_))));
    }

    #[tokio::test]
    async fn post_ccsid_requires_request_id() {
        let client = ZatcaClient::new(Config::default()).expect("client");
        let creds = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

        let result = client.request_production_csid(&creds).await;
        assert!(matches!(result, Err(ZatcaError::ClientState(_))));
    }

    use std::sync::{Mutex, OnceLock};

    fn base_url_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct BaseUrlGuard {
        _lock: std::sync::MutexGuard<'static, ()>,
        previous: Option<String>,
    }

    impl BaseUrlGuard {
        fn new(url: &str) -> Self {
            let lock = base_url_lock().lock().expect("base url lock");
            let previous = std::env::var("FATOORA_ZATCA_BASE_URL").ok();
            unsafe {
                std::env::set_var("FATOORA_ZATCA_BASE_URL", url);
            }
            Self {
                _lock: lock,
                previous,
            }
        }
    }

    impl Drop for BaseUrlGuard {
        fn drop(&mut self) {
            match self.previous.as_ref() {
                Some(value) => unsafe {
                    std::env::set_var("FATOORA_ZATCA_BASE_URL", value);
                },
                None => unsafe {
                    std::env::remove_var("FATOORA_ZATCA_BASE_URL");
                },
            }
        }
    }

    fn try_start_server() -> Option<MockServer> {
        std::panic::catch_unwind(MockServer::start).ok()
    }

    #[test]
    fn zatca_invoice_endpoints_use_base_url() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));
        let body = r#"{
          "validationResults": {
            "infoMessages": [],
            "warningMessages": [],
            "errorMessages": [],
            "status": "PASS"
          },
          "reportingStatus": "REPORTED",
          "clearanceStatus": null,
          "qrSellertStatus": null,
          "qrBuyertStatus": null
        }"#;

        let report_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/invoices/reporting/single")
                .header("accept-language", "ar");
            then.status(200)
                .header("content-type", "application/json")
                .body(body);
        });

        let clear_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/invoices/clearance/single")
                .header("accept-language", "en");
            then.status(200)
                .header("content-type", "application/json")
                .body(body);
        });

        let compliance_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/compliance/invoices")
                .header("accept-language", "en");
            then.status(200)
                .header("content-type", "application/json")
                .body(body);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let simplified = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let standard = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Standard));
            let pcsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
            let ccsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let report = client
                .report_simplified_invoice(&simplified, &pcsid, ClearanceMode::Disabled, Some("ar"))
                .await;
            assert!(report.is_ok());
            let clear = client
                .clear_standard_invoice(&standard, &pcsid, ClearanceMode::Enabled, None)
                .await;
            assert!(clear.is_ok());
            let compliance = client.check_invoice_compliance(&simplified, &ccsid).await;
            assert!(compliance.is_ok());

            report_mock.assert();
            clear_mock.assert();
            compliance_mock.assert();
        });
    }

    #[test]
    fn zatca_csid_endpoints_use_base_url() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));
        let ccsid_body = r#"{
          "requestID": 42,
          "binarySecurityToken": "token",
          "secret": "secret"
        }"#;
        let pcsid_body = r#"{
          "requestID": 77,
          "binarySecurityToken": "ptoken",
          "secret": "psecret"
        }"#;
        let renew_body = r#"{
          "value": {
            "requestID": 88,
            "binarySecurityToken": "rtoken",
            "secret": "rsecret"
          }
        }"#;

        let csr_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/compliance")
                .header("OTP", "123456");
            then.status(200)
                .header("content-type", "application/json")
                .body(ccsid_body);
        });
        let pcsid_mock = server.mock(|when, then| {
            when.method(POST).path("/production/csids");
            then.status(200)
                .header("content-type", "application/json")
                .body(pcsid_body);
        });
        let renew_mock = server.mock(|when, then| {
            when.method(PATCH)
                .path("/production/csids")
                .header("accept-language", "ar");
            then.status(428)
                .header("content-type", "application/json")
                .body(renew_body);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let csr = build_csr();
            let ccsid = client
                .request_compliance_csid(&csr, "123456")
                .await
                .expect("ccsid");
            assert_eq!(ccsid.request_id(), Some("42"));

            let pcsid = client.request_production_csid(&ccsid).await.expect("pcsid");
            assert_eq!(pcsid.request_id(), Some("77"));

            let renewed = client
                .renew_csid(&pcsid, &csr, "123456", Some("ar"))
                .await
                .expect("renew");
            assert_eq!(renewed.request_id(), Some("88"));

            csr_mock.assert();
            pcsid_mock.assert();
            renew_mock.assert();
        });
    }

    #[test]
    fn report_handles_unauthorized_and_not_acceptable() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));
        let unauthorized = r#"{"status":401,"error":"Unauthorized","message":"nope"}"#;

        let mut unauthorized_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/reporting/single");
            then.status(401)
                .header("content-type", "application/json")
                .body(unauthorized);
        });

        let not_acceptable_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/reporting/single");
            then.status(406).body("nope");
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let creds = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let result = client
                .report_simplified_invoice(&invoice, &creds, ClearanceMode::Disabled, None)
                .await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 401));

            unauthorized_mock.delete();

            let result = client
                .report_simplified_invoice(&invoice, &creds, ClearanceMode::Disabled, None)
                .await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 406));

            not_acceptable_mock.assert();
        });
    }

    #[test]
    fn report_and_clear_handle_server_error_and_unauthorized() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let report_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/reporting/single");
            then.status(500)
                .header("content-type", "application/json")
                .body(r#"{"category":"Server","code":"ERR","message":"boom"}"#);
        });

        let clear_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/clearance/single");
            then.status(401)
                .header("content-type", "application/json")
                .body(r#"{"status":401,"error":"Unauthorized","message":"nope"}"#);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let simplified = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let standard = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Standard));
            let pcsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let result = client
                .report_simplified_invoice(&simplified, &pcsid, ClearanceMode::Disabled, None)
                .await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 500));

            let result = client
                .clear_standard_invoice(&standard, &pcsid, ClearanceMode::Enabled, None)
                .await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 401));

            report_mock.assert();
            clear_mock.assert();
        });
    }

    #[test]
    fn clear_and_compliance_error_paths() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let clear_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/clearance/single");
            then.status(200).body("not json");
        });

        let compliance_mock = server.mock(|when, then| {
            when.method(POST).path("/compliance/invoices");
            then.status(500)
                .header("content-type", "application/json")
                .body(r#"{"category":"Server","code":"ERR","message":"boom"}"#);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let standard = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Standard));
            let simplified = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let pcsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
            let ccsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let result = client
                .clear_standard_invoice(&standard, &pcsid, ClearanceMode::Enabled, None)
                .await;
            assert!(matches!(result, Err(ZatcaError::ResponseDecode { .. })));

            let result = client.check_invoice_compliance(&simplified, &ccsid).await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 500));

            clear_mock.assert();
            compliance_mock.assert();
        });
    }

    #[test]
    fn report_handles_conflict_response() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));
        let body = r#"{
          "validationResults": {
            "infoMessages": [],
            "warningMessages": [],
            "errorMessages": [],
            "status": "PASS"
          },
          "reportingStatus": "REPORTED",
          "clearanceStatus": null,
          "qrSellertStatus": null,
          "qrBuyertStatus": null
        }"#;

        let report_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/reporting/single");
            then.status(409)
                .header("content-type", "application/json")
                .body(body);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let pcsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let result = client
                .report_simplified_invoice(&invoice, &pcsid, ClearanceMode::Disabled, None)
                .await;
            assert_eq!(result.unwrap_err().http_status(), Some(409));

            report_mock.assert();
        });
    }

    #[test]
    fn clear_handles_server_error() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let clear_mock = server.mock(|when, then| {
            when.method(POST).path("/invoices/clearance/single");
            then.status(500)
                .header("content-type", "application/json")
                .body(r#"{"category":"Server","code":"ERR","message":"boom"}"#);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Standard));
            let pcsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let result = client
                .clear_standard_invoice(&invoice, &pcsid, ClearanceMode::Enabled, None)
                .await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 500));

            clear_mock.assert();
        });
    }

    #[test]
    fn compliance_handles_unauthorized() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let compliance_mock = server.mock(|when, then| {
            when.method(POST).path("/compliance/invoices");
            then.status(401)
                .header("content-type", "application/json")
                .body(r#"{"status":401,"error":"Unauthorized","message":"nope"}"#);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let ccsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");

            let result = client.check_invoice_compliance(&invoice, &ccsid).await;
            assert!(matches!(result, Err(ZatcaError::Response(ref e)) if e.http_status() == 401));

            compliance_mock.assert();
        });
    }

    #[test]
    fn csid_requests_handle_invalid_and_unauthorized() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let csr_mock = server.mock(|when, then| {
            when.method(POST).path("/compliance");
            then.status(400).body("bad");
        });

        let pcsid_mock = server.mock(|when, then| {
            when.method(POST).path("/production/csids");
            then.status(400).body("bad");
        });

        let renew_mock = server.mock(|when, then| {
            when.method(PATCH).path("/production/csids");
            then.status(401)
                .header("content-type", "application/json")
                .body(r#"{"status":401,"error":"Unauthorized","message":"nope"}"#);
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let csr = build_csr();

            let result = client.request_compliance_csid(&csr, "123456").await;
            assert!(matches!(result, Err(ZatcaError::InvalidResponse(_))));

            let ccsid = CsidCredentials::new(
                Environment::NonProduction,
                Some("10".into()),
                "token",
                "secret",
            );
            let result = client.request_production_csid(&ccsid).await;
            assert!(matches!(result, Err(ZatcaError::InvalidResponse(_))));

            let pcsid = CsidCredentials::new(
                Environment::NonProduction,
                Some("11".into()),
                "token",
                "secret",
            );
            let result = client.renew_csid(&pcsid, &csr, "123456", None).await;
            assert!(matches!(result, Err(ZatcaError::Unauthorized(_))));

            csr_mock.assert();
            pcsid_mock.assert();
            renew_mock.assert();
        });
    }

    #[test]
    fn compliance_and_renew_invalid_response_paths() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let compliance_mock = server.mock(|when, then| {
            when.method(POST).path("/compliance/invoices");
            then.status(200).body("not json");
        });

        let renew_mock = server.mock(|when, then| {
            when.method(PATCH).path("/production/csids");
            then.status(418).body("nope");
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let invoice = build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
            let ccsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
            let pcsid = CsidCredentials::new(
                Environment::NonProduction,
                Some("1".into()),
                "token",
                "secret",
            );
            let csr = build_csr();

            let result = client.check_invoice_compliance(&invoice, &ccsid).await;
            assert!(matches!(result, Err(ZatcaError::ResponseDecode { .. })));

            let result = client.renew_csid(&pcsid, &csr, "123456", None).await;
            assert!(matches!(result, Err(ZatcaError::InvalidResponse(_))));

            compliance_mock.assert();
            renew_mock.assert();
        });
    }

    #[test]
    fn csid_requests_handle_invalid_json() {
        let server = match try_start_server() {
            Some(server) => server,
            None => return,
        };
        let _guard = BaseUrlGuard::new(&format!("{}/", server.base_url()));

        let csr_mock = server.mock(|when, then| {
            when.method(POST).path("/compliance");
            then.status(200).body("not json");
        });

        let pcsid_mock = server.mock(|when, then| {
            when.method(POST).path("/production/csids");
            then.status(200).body("not json");
        });

        let rt = tokio::runtime::Runtime::new().expect("runtime");
        rt.block_on(async {
            let client = ZatcaClient::new(Config::default()).expect("client");
            let csr = build_csr();

            let result = client.request_compliance_csid(&csr, "123456").await;
            assert!(matches!(result, Err(ZatcaError::InvalidResponse(_))));

            let ccsid = CsidCredentials::new(
                Environment::NonProduction,
                Some("10".into()),
                "token",
                "secret",
            );
            let result = client.request_production_csid(&ccsid).await;
            assert!(matches!(result, Err(ZatcaError::InvalidResponse(_))));

            csr_mock.assert();
            pcsid_mock.assert();
        });
    }

    fn build_csr() -> Csr {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/csr-configs/csr-config-example-EN.properties");
        let csr_props = std::fs::read_to_string(&config_path).expect("read csr config");
        let csr_config = csr_props.parse::<CsrProperties>().expect("csr config");
        let key = crate::csr::SigningKey::generate();
        csr_config
            .build(&key, Environment::NonProduction)
            .expect("csr build")
    }

    pub(super) fn build_signed_invoice(invoice_type: InvoiceType) -> SignedInvoice {
        let seller = Party::<SellerRole>::new(
            "Acme Inc",
            Address {
                province: None,
                country_code: CountryCode::parse("SAU").expect("country code"),
                city: "Riyadh".into(),
                street: "King Fahd".into(),
                additional_street: None,
                building_number: "1234".into(),
                additional_number: Some("5678".into()),
                postal_code: "12222".into(),
                district: None,
            },
            "301121971500003".parse().unwrap(),
            None,
        );

        let line_item = LineItem::new(
            "Item",
            crate::Decimal::from(1),
            "PCE",
            crate::Decimal::from(100),
            crate::Decimal::from(15),
            VatCategory::Standard,
        )
        .unwrap();

        let mut builder = InvoiceBuilder::new(invoice_type);
        builder = builder
            .id("INV-TEST-1")
            .uuid("uuid-test-1".parse().unwrap())
            .issue_datetime("2024-01-01T12:30:00Z".parse().unwrap())
            .currency("SAR".parse().unwrap())
            .previous_invoice_hash("hash".parse().unwrap())
            .invoice_counter(0)
            .seller(seller)
            .payment_means_code("10")
            .vat_category(VatCategory::Standard)
            .line_item(line_item);
        let invoice = builder.build().expect("build invoice");

        let signer = crate::invoice::sign::InvoiceSigner::from_der(
            include_bytes!("../tests/fixtures/sdk-parity/credentials/certificate.der"),
            include_bytes!("../tests/fixtures/sdk-parity/credentials/private-key.der"),
        )
        .unwrap();
        invoice.sign(&signer).expect("sign invoice")
    }
}

#[cfg(test)]
#[path = "api/response_tests.rs"]
mod response_tests;
