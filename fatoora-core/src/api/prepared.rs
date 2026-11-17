//! Owned protocol requests. No transport or input object is retained.
use super::*;
use base64ct::{Base64, Encoding};

/// An owned HTTP request. Headers include credentials; do not log them.
#[derive(Clone)]
pub struct HttpRequest {
    method: &'static str,
    url: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    error: Option<String>,
}

impl std::fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequest")
            .field("method", &self.method)
            .finish_non_exhaustive()
    }
}

impl HttpRequest {
    /// Returns the HTTP method (e.g. `POST`, `PATCH`).
    pub fn method(&self) -> &str {
        self.method
    }
    /// Returns the absolute request URL.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Returns the request headers as `(name, value)` pairs, including the
    /// `Authorization` header when credentials are used.
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }
    /// Returns the request body (JSON) bytes.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub(super) fn new(method: &'static str, url: String) -> Self {
        Self {
            method,
            url,
            headers: vec![],
            body: vec![],
            error: None,
        }
    }
    pub(super) fn header(mut self, name: &str, value: &str) -> Self {
        if let Err(error) = http::HeaderValue::from_str(value) {
            self.error = Some(error.to_string());
        }
        self.headers.push((name.to_owned(), value.to_owned()));
        self
    }
    pub(super) fn basic_auth(self, token: String, secret: Option<String>) -> Self {
        let encoded =
            Base64::encode_string(format!("{token}:{}", secret.unwrap_or_default()).as_bytes());
        self.header("Authorization", &format!("Basic {encoded}"))
    }
    pub(super) fn json(mut self, payload: &serde_json::Value) -> Self {
        match serde_json::to_vec(payload) {
            Ok(body) => self.body = body,
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }
    pub(super) fn finish(self) -> Result<Self, ZatcaError> {
        if let Some(error) = &self.error {
            return Err(ZatcaError::Http(error.clone()));
        }
        Ok(self)
    }
}

/// A reporting, clearance, or compliance request with its decoding context.
#[derive(Debug, Clone)]
pub struct InvoiceRequest {
    request: HttpRequest,
    operation: InvoiceOperation,
}
impl InvoiceRequest {
    pub(super) fn new(request: HttpRequest, operation: InvoiceOperation) -> Self {
        Self { request, operation }
    }
    /// Returns the HTTP request to send.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }
    /// Decodes the endpoint's response into a [`ValidationResponse`] tagged with
    /// this operation, so [`ValidationResponse::outcome`] can classify it.
    ///
    /// # Errors
    /// - [`ZatcaError::InvalidResponse`] if `status` is not a valid HTTP status code.
    /// - [`ZatcaError::Response`] for a non-2xx status.
    /// - [`ZatcaError::ResponseDecode`] if a 2xx body is not a valid validation response.
    pub fn decode(&self, status: u16, body: &[u8]) -> Result<ValidationResponse, ZatcaError> {
        check_status(status)?;
        super::response::decode_validation_response(status, body, self.operation)
    }
}

/// A compliance credential request retaining the issuing environment.
#[derive(Debug, Clone)]
pub struct ComplianceRequest {
    request: HttpRequest,
    env: Environment,
}
impl ComplianceRequest {
    pub(super) fn new(request: HttpRequest, env: Environment) -> Self {
        Self { request, env }
    }
    /// Returns the HTTP request to send.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }
    /// Decodes the response into compliance credentials for the request's environment.
    ///
    /// # Errors
    /// Returns [`ZatcaError::InvalidResponse`] if `status` is invalid or not 2xx,
    /// or the body is not a valid CSID response.
    pub fn decode(
        &self,
        status: u16,
        body: &[u8],
    ) -> Result<CsidCredentials<Compliance>, ZatcaError> {
        let payload = decode_csid(status, body, false)?;
        Ok(CsidCredentials::new(
            self.env,
            payload.request_id,
            payload.binary_security_token,
            payload.secret,
        ))
    }
}

/// A production credential request retaining its environment and renewal semantics.
#[derive(Debug, Clone)]
pub struct ProductionRequest {
    request: HttpRequest,
    env: Environment,
    renewal: bool,
}
impl ProductionRequest {
    pub(super) fn new(request: HttpRequest, env: Environment, renewal: bool) -> Self {
        Self {
            request,
            env,
            renewal,
        }
    }
    /// Returns the HTTP request to send.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }
    /// Decodes the response into production credentials for the request's environment.
    ///
    /// # Errors
    /// - [`ZatcaError::Unauthorized`] for HTTP 401 (renewal only).
    /// - [`ZatcaError::ServerError`] for HTTP 5xx (renewal only).
    /// - [`ZatcaError::InvalidResponse`] if `status` is invalid or otherwise
    ///   unexpected, or the body is not a valid CSID response. Renewal also
    ///   accepts an HTTP 428 body carrying credentials.
    pub fn decode(
        &self,
        status: u16,
        body: &[u8],
    ) -> Result<CsidCredentials<Production>, ZatcaError> {
        let payload = decode_csid(status, body, self.renewal)?;
        Ok(CsidCredentials::new(
            self.env,
            payload.request_id,
            payload.binary_security_token,
            payload.secret,
        ))
    }
}

fn check_status(status: u16) -> Result<http::StatusCode, ZatcaError> {
    http::StatusCode::from_u16(status).map_err(|e| ZatcaError::InvalidResponse(e.to_string()))
}

fn decode_csid(status: u16, bytes: &[u8], renewal: bool) -> Result<CsidResponseBody, ZatcaError> {
    let status = check_status(status)?;
    let body = String::from_utf8_lossy(bytes).into_owned();
    if !renewal {
        if !status.is_success() {
            return Err(ZatcaError::InvalidResponse(format!(
                "status {status}: {body}"
            )));
        }
        return serde_json::from_slice(bytes)
            .map_err(|e| ZatcaError::InvalidResponse(e.to_string()));
    }
    if status.is_success() || status.as_u16() == 428 {
        let parsed: RenewalResponseBody = serde_json::from_str(&body)
            .map_err(|e| ZatcaError::InvalidResponse(format!("Invalid response: {e:?}")))?;
        return Ok(match parsed {
            RenewalResponseBody::Direct(value) | RenewalResponseBody::Wrapped { value } => value,
        });
    }
    if status.as_u16() == 401 {
        let parsed = serde_json::from_str::<UnauthorizedResponse>(&body).unwrap_or_else(|_| {
            UnauthorizedResponse {
                timestamp: None,
                status: Some(401),
                error: Some("Unauthorized".into()),
                message: Some(body.clone()),
            }
        });
        return Err(ZatcaError::Unauthorized(parsed));
    }
    if status.is_server_error() {
        let parsed = serde_json::from_str::<ServerErrorResponse>(&body).unwrap_or_else(|_| {
            ServerErrorResponse {
                category: None,
                code: Some("ServerError".into()),
                message: Some(body.clone()),
            }
        });
        return Err(ZatcaError::ServerError(parsed));
    }
    Err(ZatcaError::InvalidResponse(format!(
        "status {status}: {body}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::invoice::{InvoiceSubType, InvoiceType};

    #[test]
    fn prepared_invoice_owns_inputs_and_keeps_operation_context() {
        let client = ZatcaClient::new(Config::default()).unwrap();
        let invoice =
            super::super::tests::build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
        let credentials =
            CsidCredentials::<Production>::new(Environment::NonProduction, None, "token", "secret");
        let prepared = client
            .prepare_report_simplified_invoice(&invoice, &credentials, true, Some("ar"))
            .unwrap();
        let expected = serde_json::json!({"invoiceHash": invoice.invoice_hash(), "uuid": invoice.uuid(), "invoice": invoice.to_xml_base64()});
        drop(invoice);
        drop(credentials);
        drop(client);
        assert_eq!(prepared.request().method(), "POST");
        assert!(
            prepared
                .request()
                .url()
                .ends_with("/invoices/reporting/single")
        );
        let headers = prepared.request().headers();
        for (name, value) in [
            ("Accept-Version", "V2"),
            ("Clearance-Status", "1"),
            ("accept-language", "ar"),
            ("Authorization", "Basic dG9rZW46c2VjcmV0"),
        ] {
            assert!(headers.contains(&(name.into(), value.into())));
        }
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(prepared.request().body()).unwrap(),
            expected
        );
        let body = br#"{"validationResults":{"status":"PASS"},"reportingStatus":"NOT_REPORTED","clearanceStatus":"CLEARED"}"#;
        let response = prepared.decode(200, body).unwrap();
        assert_eq!(response.outcome(), InvoiceOutcome::Rejected);
        assert!(response.ensure_accepted().is_err());
        assert_eq!(
            prepared.decode(409, body).unwrap_err().http_status(),
            Some(409)
        );
        assert!(matches!(
            prepared.decode(200, b"bad"),
            Err(ZatcaError::ResponseDecode { .. })
        ));
        assert!(prepared.decode(0, body).is_err());
        assert!(!format!("{prepared:?}").contains("dG9r"));
    }

    #[test]
    fn prepared_request_debug_omits_credentials_and_authorization_header() {
        const TOKEN: &str = "TUlJQ1BUQ0NBZU9nQXdJQkFnSTOKEN";
        const SECRET: &str = "Dehvg1fc8GF6Jwt5bOxXwC6enSECRET";
        let client = ZatcaClient::new(Config::default()).unwrap();
        let invoice =
            super::super::tests::build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Simplified));
        let credentials =
            CsidCredentials::<Production>::new(Environment::NonProduction, None, TOKEN, SECRET);
        let invoice_request = client
            .prepare_report_simplified_invoice(&invoice, &credentials, false, None)
            .unwrap();
        let (_, authorization) = invoice_request
            .request()
            .headers()
            .iter()
            .find(|(name, _)| name == "Authorization")
            .expect("authorization header")
            .clone();
        let encoded = authorization.trim_start_matches("Basic ").to_owned();
        let http = invoice_request.request().clone();
        let compliance = ComplianceRequest::new(http.clone(), Environment::Simulation);
        let production = ProductionRequest::new(http.clone(), Environment::Simulation, true);
        let rendered = [
            format!("{http:?}"),
            format!("{http:#?}"),
            format!("{invoice_request:?}"),
            format!("{invoice_request:#?}"),
            format!("{compliance:?}"),
            format!("{compliance:#?}"),
            format!("{production:?}"),
            format!("{production:#?}"),
        ];
        for rendered in rendered {
            for needle in [TOKEN, SECRET, authorization.as_str(), encoded.as_str()] {
                assert!(!rendered.contains(needle), "leaked {needle}: {rendered}");
            }
            assert!(!rendered.contains("Authorization"), "{rendered}");
        }
    }

    #[test]
    fn preparation_rejects_mismatched_scope_and_invalid_headers() {
        let client = ZatcaClient::new(Config::default()).unwrap();
        let invoice =
            super::super::tests::build_signed_invoice(InvoiceType::Tax(InvoiceSubType::Standard));
        let credentials =
            CsidCredentials::<Production>::new(Environment::NonProduction, None, "token", "secret");
        assert!(
            client
                .prepare_report_simplified_invoice(&invoice, &credentials, false, None)
                .is_err()
        );
        let credentials =
            CsidCredentials::<Production>::new(Environment::Production, None, "token", "secret");
        assert!(
            client
                .prepare_clear_standard_invoice(&invoice, &credentials, true, None)
                .is_err()
        );
        assert!(
            HttpRequest::new("POST", "https://example.test/".into())
                .header("OTP", "x\r\nInjected: yes")
                .finish()
                .is_err()
        );
    }

    #[test]
    fn credential_decoding_retains_environment_and_renewal_contract() {
        let request = HttpRequest::new("PATCH", "https://example.test/".into());
        let renewal = ProductionRequest::new(request.clone(), Environment::Simulation, true);
        let body =
            br#"{"value":{"requestID":123,"binarySecurityToken":"token","secret":"secret"}}"#;
        let credentials = renewal.decode(428, body).unwrap();
        assert_eq!(credentials.env(), Environment::Simulation);
        assert_eq!(credentials.request_id(), Some("123"));
        assert!(matches!(
            renewal.decode(401, b"bad"),
            Err(ZatcaError::Unauthorized(_))
        ));
        assert!(matches!(
            renewal.decode(500, b"bad"),
            Err(ZatcaError::ServerError(_))
        ));
        let issuance = ProductionRequest::new(request, Environment::Simulation, false);
        assert!(issuance.decode(428, body).is_err());
    }
}
