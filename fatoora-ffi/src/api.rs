//! Owned ZATCA credentials and blocking HTTP operations for generated bindings.
use crate::common::ffi::{BindingError, Text};
use crate::common::{boundary, core_error, local_error, text, write};
use diplomat_runtime::DiplomatStr;
use fatoora_core::api as core_api;
#[cfg(not(target_arch = "wasm32"))]
use fatoora_core::api as core;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;

#[cfg(not(target_arch = "wasm32"))]
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

// Enter Tokio on a scoped worker so callers already inside a runtime remain supported.
// The worker cannot outlive borrowed invoice, credentials, or client objects.
#[cfg(not(target_arch = "wasm32"))]
fn run<T: Send>(
    future: impl std::future::Future<Output = Result<T, core::ZatcaError>> + Send,
) -> Result<T, Box<BindingError>> {
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .name("fatoora-http".into())
            .spawn_scoped(scope, move || {
                boundary(|| {
                    if RUNTIME.get().is_none() {
                        let runtime = tokio::runtime::Runtime::new().map_err(|err| {
                            local_error(9, &format!("runtime init failed: {err}"))
                        })?;
                        let _ = RUNTIME.set(runtime);
                    }
                    RUNTIME
                        .get()
                        .expect("runtime initialized")
                        .block_on(future)
                        .map_err(core_error)
                })
            })
            .map_err(|err| local_error(9, &format!("HTTP worker failed: {err}")))?
            .join()
            .map_err(|payload| {
                if let Err(nested) =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(payload)))
                {
                    std::mem::forget(nested);
                }
                local_error(9, "HTTP worker panicked")
            })?
    })
}

fn optional(value: Option<&str>) -> Result<Option<Box<Text>>, Box<BindingError>> {
    value
        .map(|value| {
            if value.contains('\0') {
                return Err(local_error(1, "output contains NUL"));
            }
            Ok(Box::new(Text(value.to_owned())))
        })
        .transpose()
}

fn optional_input(value: Option<&DiplomatStr>) -> Result<Option<&str>, Box<BindingError>> {
    value.map(text).transpose()
}

impl From<&core_api::ReportingStatus> for ffi::ReportingStatus {
    fn from(value: &core_api::ReportingStatus) -> Self {
        match value {
            core_api::ReportingStatus::Reported => Self::Reported,
            core_api::ReportingStatus::NotReported => Self::NotReported,
            _ => Self::Unknown,
        }
    }
}

impl From<&core_api::ClearanceStatus> for ffi::ClearanceStatus {
    fn from(value: &core_api::ClearanceStatus) -> Self {
        match value {
            core_api::ClearanceStatus::Cleared => Self::Cleared,
            core_api::ClearanceStatus::NotCleared => Self::NotCleared,
            _ => Self::Unknown,
        }
    }
}

impl From<&core_api::ValidationStatus> for ffi::ValidationStatus {
    fn from(value: &core_api::ValidationStatus) -> Self {
        match value {
            core_api::ValidationStatus::Pass => Self::Pass,
            core_api::ValidationStatus::Warning => Self::Warning,
            core_api::ValidationStatus::Error => Self::Error,
            _ => Self::Unknown,
        }
    }
}

impl From<&core_api::MessageType> for ffi::MessageType {
    fn from(value: &core_api::MessageType) -> Self {
        match value {
            core_api::MessageType::Info => Self::Info,
            core_api::MessageType::Warning => Self::Warning,
            core_api::MessageType::Error => Self::Error,
            _ => Self::Unknown,
        }
    }
}

#[diplomat::bridge]
#[diplomat::abi_rename = "fatoora_{0}"]
#[diplomat::attr(cpp, namespace = "fatoora")]
pub mod ffi {
    #[cfg(not(target_arch = "wasm32"))]
    use super::run;
    use super::{boundary, core_error, local_error, optional, optional_input, text, write};
    use crate::common::environment_to_ffi;
    use crate::common::ffi::{BindingError, Environment, Text};
    use crate::crypto::ffi::{Bytes, Config, Csr};
    use crate::invoice::ffi::SignedInvoice;
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};

    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::attr(
        nanobind,
        custom_extra_code(file = "client_bindings.cpp", location = "init_block")
    )]
    pub struct ZatcaClient(pub(crate) fatoora_core::api::ZatcaClient);
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ComplianceCsid(
        pub(crate) fatoora_core::api::CsidCredentials<fatoora_core::api::Compliance>,
    );
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ProductionCsid(
        pub(crate) fatoora_core::api::CsidCredentials<fatoora_core::api::Production>,
    );
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ValidationResponse(pub(crate) fatoora_core::api::ValidationResponse);
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ValidationResults(pub(crate) fatoora_core::api::ValidationResults);
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ValidationMessage(pub(crate) fatoora_core::api::ValidationMessage);

    pub enum InvoiceOutcome {
        Unknown = 0,
        Accepted = 1,
        Rejected = 2,
    }

    /// Value of the `Clearance-Status` header sent with reporting and clearance requests.
    #[diplomat::enum_convert(fatoora_core::api::ClearanceMode)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum ClearanceMode {
        /// Header value `0` (clearance is switched off for the taxpayer).
        Disabled = 0,
        /// Header value `1` (clearance is switched on for the taxpayer).
        Enabled = 1,
    }

    /// `reportingStatus` of a reporting response.
    ///
    /// `Unknown` marks a value this binding version does not recognize; read the
    /// verbatim wire value with the sibling `*_raw` getter.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ReportingStatus {
        Unknown = 0,
        /// Wire value `REPORTED`.
        Reported = 1,
        /// Wire value `NOT_REPORTED`.
        NotReported = 2,
    }

    /// `clearanceStatus` of a clearance response.
    ///
    /// `Unknown` marks a value this binding version does not recognize; read the
    /// verbatim wire value with the sibling `*_raw` getter.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ClearanceStatus {
        Unknown = 0,
        /// Wire value `CLEARED`.
        Cleared = 1,
        /// Wire value `NOT_CLEARED`.
        NotCleared = 2,
    }

    /// Overall `validationResults.status`, or the `status` of one validation message.
    ///
    /// `Unknown` marks a value this binding version does not recognize; read the
    /// verbatim wire value with the sibling `*_raw` getter.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ValidationStatus {
        Unknown = 0,
        /// Wire value `PASS`.
        Pass = 1,
        /// Wire value `WARNING`.
        Warning = 2,
        /// Wire value `ERROR`.
        Error = 3,
    }

    /// The `type` of a validation message.
    ///
    /// `Unknown` marks a value this binding version does not recognize; read the
    /// verbatim wire value with the sibling `*_raw` getter.
    #[derive(Debug, PartialEq, Eq)]
    pub enum MessageType {
        Unknown = 0,
        /// Wire value `INFO`.
        Info = 1,
        /// Wire value `WARNING`.
        Warning = 2,
        /// Wire value `ERROR`.
        Error = 3,
    }

    // Diplomat expands this macro for Rust and every generated language.
    #[diplomat::macro_rules]
    macro_rules! request_accessors {
        () => {
            pub fn method(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
                boundary(|| write(self.0.request().method(), out))
            }
            pub fn url(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
                boundary(|| write(self.0.request().url(), out))
            }
            pub fn headers_json(&self, out: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
                boundary(|| {
                    let json = serde_json::to_string(self.0.request().headers())
                        .map_err(|e| local_error(9, &e.to_string()))?;
                    write(json, out)
                })
            }
            pub fn body(&self) -> Result<Box<Bytes>, Box<BindingError>> {
                boundary(|| Ok(Box::new(Bytes(self.0.request().body().to_vec()))))
            }
        };
    }

    /// Owned request and decoding context; contains credentials.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct InvoiceRequest(pub(crate) fatoora_core::api::InvoiceRequest);
    impl InvoiceRequest {
        request_accessors!();
        pub fn decode(
            &self,
            status: u16,
            body: &[u8],
        ) -> Result<Box<ValidationResponse>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .decode(status, body)
                    .map(|v| Box::new(ValidationResponse(v)))
                    .map_err(core_error)
            })
        }
    }
    /// Owned request and decoding context; contains credentials.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ComplianceRequest(pub(crate) fatoora_core::api::ComplianceRequest);
    impl ComplianceRequest {
        request_accessors!();
        pub fn decode(
            &self,
            status: u16,
            body: &[u8],
        ) -> Result<Box<ComplianceCsid>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .decode(status, body)
                    .map(|v| Box::new(ComplianceCsid(v)))
                    .map_err(core_error)
            })
        }
    }
    /// Owned request and decoding context; contains credentials.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct ProductionRequest(pub(crate) fatoora_core::api::ProductionRequest);
    impl ProductionRequest {
        request_accessors!();
        pub fn decode(
            &self,
            status: u16,
            body: &[u8],
        ) -> Result<Box<ProductionCsid>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .decode(status, body)
                    .map(|v| Box::new(ProductionCsid(v)))
                    .map_err(core_error)
            })
        }
    }
    impl ComplianceCsid {
        pub fn create(
            environment: Environment,
            request_id: Option<&DiplomatStr>,
            token: &DiplomatStr,
            secret: &DiplomatStr,
        ) -> Result<Box<Self>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(fatoora_core::api::CsidCredentials::<
                    fatoora_core::api::Compliance,
                >::new(
                    environment.into(),
                    optional_input(request_id)?.map(str::to_owned),
                    text(token)?,
                    text(secret)?,
                ))))
            })
        }
        pub fn env(&self) -> Result<Environment, Box<BindingError>> {
            boundary(|| environment_to_ffi(self.0.env()))
        }
        pub fn request_id(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.request_id()))
        }
        pub fn binary_security_token(
            &self,
            output: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.binary_security_token(), output))
        }
        pub fn secret(&self, output: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.secret(), output))
        }
    }

    impl ProductionCsid {
        pub fn create(
            environment: Environment,
            request_id: Option<&DiplomatStr>,
            token: &DiplomatStr,
            secret: &DiplomatStr,
        ) -> Result<Box<Self>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(Self(fatoora_core::api::CsidCredentials::<
                    fatoora_core::api::Production,
                >::new(
                    environment.into(),
                    optional_input(request_id)?.map(str::to_owned),
                    text(token)?,
                    text(secret)?,
                ))))
            })
        }
        pub fn env(&self) -> Result<Environment, Box<BindingError>> {
            boundary(|| environment_to_ffi(self.0.env()))
        }
        pub fn request_id(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.request_id()))
        }
        pub fn binary_security_token(
            &self,
            output: &mut DiplomatWrite,
        ) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.binary_security_token(), output))
        }
        pub fn secret(&self, output: &mut DiplomatWrite) -> Result<(), Box<BindingError>> {
            boundary(|| write(self.0.secret(), output))
        }
    }

    impl ZatcaClient {
        pub fn create(config: &Config) -> Result<Box<Self>, Box<BindingError>> {
            boundary(|| {
                fatoora_core::api::ZatcaClient::new(config.0.clone())
                    .map(|value| Box::new(Self(value)))
                    .map_err(core_error)
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[diplomat::attr(js, disable)]
        pub fn request_compliance_csid(
            &self,
            csr: &Csr,
            otp: &DiplomatStr,
        ) -> Result<Box<ComplianceCsid>, Box<BindingError>> {
            boundary(|| {
                run(self.0.request_compliance_csid(&csr.0, text(otp)?))
                    .map(|value| Box::new(ComplianceCsid(value)))
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[diplomat::attr(js, disable)]
        pub fn request_production_csid(
            &self,
            credentials: &ComplianceCsid,
        ) -> Result<Box<ProductionCsid>, Box<BindingError>> {
            boundary(|| {
                run(self.0.request_production_csid(&credentials.0))
                    .map(|value| Box::new(ProductionCsid(value)))
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[diplomat::attr(js, disable)]
        pub fn renew_csid(
            &self,
            credentials: &ProductionCsid,
            csr: &Csr,
            otp: &DiplomatStr,
            accept_language: Option<&DiplomatStr>,
        ) -> Result<Box<ProductionCsid>, Box<BindingError>> {
            boundary(|| {
                run(self.0.renew_csid(
                    &credentials.0,
                    &csr.0,
                    text(otp)?,
                    optional_input(accept_language)?,
                ))
                .map(|value| Box::new(ProductionCsid(value)))
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[diplomat::attr(js, disable)]
        pub fn check_invoice_compliance(
            &self,
            invoice: &SignedInvoice,
            credentials: &ComplianceCsid,
        ) -> Result<Box<ValidationResponse>, Box<BindingError>> {
            boundary(|| {
                let invoice = invoice
                    .0
                    .as_ref()
                    .ok_or_else(|| local_error(1, "signed invoice has been consumed"))?;
                run(self.0.check_invoice_compliance(invoice, &credentials.0))
                    .map(|value| Box::new(ValidationResponse(value)))
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[diplomat::attr(js, disable)]
        pub fn report_simplified_invoice(
            &self,
            invoice: &SignedInvoice,
            credentials: &ProductionCsid,
            clearance_mode: ClearanceMode,
            accept_language: Option<&DiplomatStr>,
        ) -> Result<Box<ValidationResponse>, Box<BindingError>> {
            boundary(|| {
                let invoice = invoice
                    .0
                    .as_ref()
                    .ok_or_else(|| local_error(1, "signed invoice has been consumed"))?;
                run(self.0.report_simplified_invoice(
                    invoice,
                    &credentials.0,
                    clearance_mode.into(),
                    optional_input(accept_language)?,
                ))
                .map(|value| Box::new(ValidationResponse(value)))
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[diplomat::attr(js, disable)]
        pub fn clear_standard_invoice(
            &self,
            invoice: &SignedInvoice,
            credentials: &ProductionCsid,
            clearance_mode: ClearanceMode,
            accept_language: Option<&DiplomatStr>,
        ) -> Result<Box<ValidationResponse>, Box<BindingError>> {
            boundary(|| {
                let invoice = invoice
                    .0
                    .as_ref()
                    .ok_or_else(|| local_error(1, "signed invoice has been consumed"))?;
                run(self.0.clear_standard_invoice(
                    invoice,
                    &credentials.0,
                    clearance_mode.into(),
                    optional_input(accept_language)?,
                ))
                .map(|value| Box::new(ValidationResponse(value)))
            })
        }
    }
    impl ZatcaClient {
        pub fn prepare_request_compliance_csid(
            &self,
            csr: &Csr,
            otp: &DiplomatStr,
        ) -> Result<Box<ComplianceRequest>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .prepare_request_compliance_csid(&csr.0, text(otp)?)
                    .map(|value| Box::new(ComplianceRequest(value)))
                    .map_err(core_error)
            })
        }
        pub fn prepare_request_production_csid(
            &self,
            credentials: &ComplianceCsid,
        ) -> Result<Box<ProductionRequest>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .prepare_request_production_csid(&credentials.0)
                    .map(|value| Box::new(ProductionRequest(value)))
                    .map_err(core_error)
            })
        }
        pub fn prepare_renew_csid(
            &self,
            credentials: &ProductionCsid,
            csr: &Csr,
            otp: &DiplomatStr,
            accept_language: Option<&DiplomatStr>,
        ) -> Result<Box<ProductionRequest>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .prepare_renew_csid(
                        &credentials.0,
                        &csr.0,
                        text(otp)?,
                        optional_input(accept_language)?,
                    )
                    .map(|value| Box::new(ProductionRequest(value)))
                    .map_err(core_error)
            })
        }
        pub fn prepare_check_invoice_compliance(
            &self,
            invoice: &SignedInvoice,
            credentials: &ComplianceCsid,
        ) -> Result<Box<InvoiceRequest>, Box<BindingError>> {
            boundary(|| {
                let invoice = invoice
                    .0
                    .as_ref()
                    .ok_or_else(|| local_error(1, "signed invoice has been consumed"))?;
                self.0
                    .prepare_check_invoice_compliance(invoice, &credentials.0)
                    .map(|value| Box::new(InvoiceRequest(value)))
                    .map_err(core_error)
            })
        }
        pub fn prepare_report_simplified_invoice(
            &self,
            invoice: &SignedInvoice,
            credentials: &ProductionCsid,
            clearance_mode: ClearanceMode,
            accept_language: Option<&DiplomatStr>,
        ) -> Result<Box<InvoiceRequest>, Box<BindingError>> {
            boundary(|| {
                let invoice = invoice
                    .0
                    .as_ref()
                    .ok_or_else(|| local_error(1, "signed invoice has been consumed"))?;
                self.0
                    .prepare_report_simplified_invoice(
                        invoice,
                        &credentials.0,
                        clearance_mode.into(),
                        optional_input(accept_language)?,
                    )
                    .map(|value| Box::new(InvoiceRequest(value)))
                    .map_err(core_error)
            })
        }
        pub fn prepare_clear_standard_invoice(
            &self,
            invoice: &SignedInvoice,
            credentials: &ProductionCsid,
            clearance_mode: ClearanceMode,
            accept_language: Option<&DiplomatStr>,
        ) -> Result<Box<InvoiceRequest>, Box<BindingError>> {
            boundary(|| {
                let invoice = invoice
                    .0
                    .as_ref()
                    .ok_or_else(|| local_error(1, "signed invoice has been consumed"))?;
                self.0
                    .prepare_clear_standard_invoice(
                        invoice,
                        &credentials.0,
                        clearance_mode.into(),
                        optional_input(accept_language)?,
                    )
                    .map(|value| Box::new(InvoiceRequest(value)))
                    .map_err(core_error)
            })
        }
    }
    impl ValidationResponse {
        pub fn http_status(&self) -> Option<u16> {
            self.0.http_status()
        }
        pub fn outcome(&self) -> InvoiceOutcome {
            match self.0.outcome() {
                fatoora_core::api::InvoiceOutcome::Accepted => InvoiceOutcome::Accepted,
                fatoora_core::api::InvoiceOutcome::Rejected => InvoiceOutcome::Rejected,
                _ => InvoiceOutcome::Unknown,
            }
        }
        pub fn ensure_accepted(&self) -> Result<(), Box<BindingError>> {
            boundary(|| self.0.ensure_accepted().map_err(core_error))
        }
        pub fn cleared_invoice_xml(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.cleared_invoice_xml().map_err(core_error)?.as_deref()))
        }
        pub fn validation_results(&self) -> Result<Box<ValidationResults>, Box<BindingError>> {
            boundary(|| {
                Ok(Box::new(ValidationResults(
                    self.0.validation_results().clone(),
                )))
            })
        }
        pub fn cleared_invoice_base64(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.cleared_invoice_base64()))
        }
        /// The parsed `reportingStatus`, or null when the response has none.
        pub fn reporting_status(&self) -> Option<ReportingStatus> {
            self.0.reporting_status().map(Into::into)
        }
        /// The `reportingStatus` exactly as ZATCA sent it, or null when absent.
        pub fn reporting_status_raw(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.reporting_status().map(|v| v.as_str())))
        }
        /// The parsed `clearanceStatus`, or null when the response has none.
        pub fn clearance_status(&self) -> Option<ClearanceStatus> {
            self.0.clearance_status().map(Into::into)
        }
        /// The `clearanceStatus` exactly as ZATCA sent it, or null when absent.
        pub fn clearance_status_raw(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.clearance_status().map(|v| v.as_str())))
        }
        pub fn qr_seller_status(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.qr_seller_status()))
        }
        pub fn qr_buyer_status(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.qr_buyer_status()))
        }
    }
    impl ValidationResults {
        /// The parsed overall status, or null when the response has none.
        pub fn status(&self) -> Option<ValidationStatus> {
            self.0.status().map(Into::into)
        }
        /// The overall status exactly as ZATCA sent it, or null when absent.
        pub fn status_raw(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.status().map(|v| v.as_str())))
        }
        pub fn info_len(&self) -> usize {
            match self.0.info_messages() {
                fatoora_core::api::MessageList::One(_) => 1,
                fatoora_core::api::MessageList::Many(values) => values.len(),
                fatoora_core::api::MessageList::Empty => 0,
            }
        }
        pub fn info_message(
            &self,
            index: usize,
        ) -> Result<Box<ValidationMessage>, Box<BindingError>> {
            boundary(|| {
                let value = match self.0.info_messages() {
                    fatoora_core::api::MessageList::One(value) => (index == 0).then_some(value),
                    fatoora_core::api::MessageList::Many(values) => values.get(index),
                    fatoora_core::api::MessageList::Empty => None,
                };
                value
                    .cloned()
                    .map(|value| Box::new(ValidationMessage(value)))
                    .ok_or_else(|| local_error(1, "info message index out of range"))
            })
        }
        pub fn warning_len(&self) -> usize {
            self.0.warning_messages().len()
        }
        pub fn warning_message(
            &self,
            index: usize,
        ) -> Result<Box<ValidationMessage>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .warning_messages()
                    .get(index)
                    .cloned()
                    .map(|value| Box::new(ValidationMessage(value)))
                    .ok_or_else(|| local_error(1, "warning message index out of range"))
            })
        }
        pub fn error_len(&self) -> usize {
            self.0.error_messages().len()
        }
        pub fn error_message(
            &self,
            index: usize,
        ) -> Result<Box<ValidationMessage>, Box<BindingError>> {
            boundary(|| {
                self.0
                    .error_messages()
                    .get(index)
                    .cloned()
                    .map(|value| Box::new(ValidationMessage(value)))
                    .ok_or_else(|| local_error(1, "error message index out of range"))
            })
        }
    }
    impl ValidationMessage {
        /// The parsed message `type`, or null when the message has none.
        pub fn message_type(&self) -> Option<MessageType> {
            self.0.message_type().map(Into::into)
        }
        /// The message `type` exactly as ZATCA sent it, or null when absent.
        pub fn message_type_raw(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.message_type().map(|v| v.as_str())))
        }
        pub fn code(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.code()))
        }
        pub fn category(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.category()))
        }
        pub fn message(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.message()))
        }
        /// The parsed message `status`, or null when the message has none.
        pub fn status(&self) -> Option<ValidationStatus> {
            self.0.status().map(Into::into)
        }
        /// The message `status` exactly as ZATCA sent it, or null when absent.
        pub fn status_raw(&self) -> Result<Option<Box<Text>>, Box<BindingError>> {
            boundary(|| optional(self.0.status().map(|v| v.as_str())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ffi::*, run};
    use crate::common::ffi::Environment;

    #[test]
    fn blocking_adapter_supports_nested_runtime_and_contains_panics() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            assert_eq!(run(async { Ok(42) }).ok(), Some(42));
            let error = run::<()>(async { panic!("HTTP operation panic") })
                .err()
                .unwrap();
            assert_eq!(error.code(), 9);
            assert_eq!(run(async { Ok(43) }).ok(), Some(43));
        });
    }

    #[test]
    fn credentials_validate_environment_utf8_nul_and_optional_id() {
        assert!(
            ProductionCsid::create(Environment::NonProduction, None, b"\xff", b"secret").is_err()
        );
        assert!(
            ComplianceCsid::create(
                Environment::NonProduction,
                Some(b"id\0bad"),
                b"token",
                b"secret"
            )
            .is_err()
        );
        let absent = ComplianceCsid::create(Environment::Simulation, None, b"token", b"secret")
            .ok()
            .unwrap();
        let empty = ProductionCsid::create(Environment::Production, Some(b""), b"token", b"secret")
            .ok()
            .unwrap();
        assert_eq!(absent.env().ok(), Some(Environment::Simulation));
        assert!(absent.request_id().ok().unwrap().is_none());
        assert_eq!(empty.env().ok(), Some(Environment::Production));
        assert_eq!(empty.request_id().ok().unwrap().unwrap().0, "");
    }

    #[test]
    fn detached_response_cannot_claim_acceptance_and_children_are_owned() {
        let response = ValidationResponse(
            serde_json::from_value(serde_json::json!({
                "http_status": 200,
                "reportingStatus": "REPORTED",
                "validationResults": {
                    "status": "PASS",
                    "infoMessages": {"code":"INFO", "message":"retained"},
                    "warningMessages": [{"code":"WARN"}],
                    "errorMessages": []
                }
            }))
            .unwrap(),
        );
        assert_eq!(response.http_status(), None);
        assert!(matches!(response.outcome(), InvoiceOutcome::Unknown));
        assert_eq!(response.ensure_accepted().err().unwrap().code(), 10);
        let results = response.validation_results().ok().unwrap();
        drop(response);
        assert_eq!(results.info_len(), 1);
        assert_eq!(results.warning_len(), 1);
        assert_eq!(results.error_len(), 0);
        assert!(results.info_message(1).is_err());
        assert!(results.warning_message(usize::MAX).is_err());
        assert!(results.error_message(0).is_err());
        let message = results.info_message(0).ok().unwrap();
        drop(results);
        assert_eq!(message.message().ok().unwrap().unwrap().0, "retained");
    }

    #[test]
    fn response_rejects_nul_text_and_invalid_cleared_xml() {
        for encoded in ["", "!!!", "AA==", "/w=="] {
            let response = ValidationResponse(
                serde_json::from_value(serde_json::json!({
                    "validationResults": {}, "clearedInvoice": encoded
                }))
                .unwrap(),
            );
            assert_eq!(response.cleared_invoice_xml().err().unwrap().code(), 3);
        }
        let response = ValidationResponse(
            serde_json::from_value(serde_json::json!({
                "validationResults": {}, "reportingStatus": "REPORTED\u{0000}"
            }))
            .unwrap(),
        );
        assert_eq!(response.reporting_status_raw().err().unwrap().code(), 1);
        assert_eq!(response.reporting_status(), Some(ReportingStatus::Unknown));
        assert!(response.cleared_invoice_xml().ok().unwrap().is_none());
    }

    #[test]
    fn message_shapes_preserve_order_fields_and_owned_values() {
        let message = serde_json::json!({
            "type":"WARNING", "code":"BR-KSA-07", "category":"KSA",
            "message":"ملاحظة", "status":"WARNING"
        });
        for info in [
            serde_json::Value::Null,
            serde_json::json!([]),
            message.clone(),
            serde_json::json!([message, {"code":"SECOND"}]),
        ] {
            let expected_len = match &info {
                serde_json::Value::Null => 0,
                serde_json::Value::Array(v) => v.len(),
                _ => 1,
            };
            let response = ValidationResponse(serde_json::from_value(serde_json::json!({
                "validationResults":{"status":"ERROR", "infoMessages":info, "errorMessages":[message]}
            })).unwrap());
            let results = response.validation_results().ok().unwrap();
            drop(response);
            assert_eq!(results.status(), Some(ValidationStatus::Error));
            assert_eq!(results.status_raw().ok().unwrap().unwrap().0, "ERROR");
            assert_eq!(results.info_len(), expected_len);
            assert_eq!(results.error_len(), 1);
            assert_eq!(results.info_message(expected_len).err().unwrap().code(), 1);
            assert_eq!(results.info_message(usize::MAX).err().unwrap().code(), 1);
            if expected_len > 0 {
                assert_eq!(
                    results
                        .info_message(0)
                        .ok()
                        .unwrap()
                        .code()
                        .ok()
                        .unwrap()
                        .unwrap()
                        .0,
                    "BR-KSA-07"
                );
            }
            if expected_len == 2 {
                assert_eq!(
                    results
                        .info_message(1)
                        .ok()
                        .unwrap()
                        .code()
                        .ok()
                        .unwrap()
                        .unwrap()
                        .0,
                    "SECOND"
                );
            }
            let error = results.error_message(0).ok().unwrap();
            drop(results);
            assert_eq!(error.message_type(), Some(MessageType::Warning));
            assert_eq!(error.message_type_raw().ok().unwrap().unwrap().0, "WARNING");
            assert_eq!(error.code().ok().unwrap().unwrap().0, "BR-KSA-07");
            assert_eq!(error.category().ok().unwrap().unwrap().0, "KSA");
            assert_eq!(error.message().ok().unwrap().unwrap().0, "ملاحظة");
            assert_eq!(error.status(), Some(ValidationStatus::Warning));
            assert_eq!(error.status_raw().ok().unwrap().unwrap().0, "WARNING");
        }
    }

    #[test]
    fn clearance_payload_and_statuses_preserve_absent_and_empty_values() {
        use base64ct::{Base64, Encoding};
        let xml = "<?xml version=\"1.0\"?>\n<Invoice>  العربية  </Invoice>\n";
        let encoded = Base64::encode_string(xml.as_bytes());
        let response = ValidationResponse(
            serde_json::from_value(serde_json::json!({
                "validationResults":{}, "clearanceStatus":"CLEARED", "clearedInvoice":encoded,
                "qrSellertStatus":"", "qrBuyertStatus":"PASS"
            }))
            .unwrap(),
        );
        let decoded = response.cleared_invoice_xml().ok().unwrap().unwrap();
        assert_eq!(
            response.cleared_invoice_base64().ok().unwrap().unwrap().0,
            encoded
        );
        assert_eq!(response.clearance_status(), Some(ClearanceStatus::Cleared));
        assert_eq!(
            response.clearance_status_raw().ok().unwrap().unwrap().0,
            "CLEARED"
        );
        assert!(response.reporting_status().is_none());
        assert!(response.reporting_status_raw().ok().unwrap().is_none());
        assert_eq!(response.qr_seller_status().ok().unwrap().unwrap().0, "");
        assert_eq!(response.qr_buyer_status().ok().unwrap().unwrap().0, "PASS");
        drop(response);
        assert_eq!(decoded.0, xml);
    }

    #[test]
    fn unrecognized_statuses_round_trip_verbatim() {
        let response = ValidationResponse(
            serde_json::from_value(serde_json::json!({
                "reportingStatus": "reported ",
                "clearanceStatus": "",
                "validationResults": {
                    "status": "PARTIAL",
                    "errorMessages": [{"type": "Notice", "status": "WARN"}]
                }
            }))
            .unwrap(),
        );
        type Out =
            Result<Option<Box<crate::common::ffi::Text>>, Box<crate::common::ffi::BindingError>>;
        let text = |value: Out| value.ok().unwrap().unwrap().0;
        assert_eq!(text(response.reporting_status_raw()), "reported ");
        assert_eq!(response.reporting_status(), Some(ReportingStatus::Unknown));
        assert_eq!(text(response.clearance_status_raw()), "");
        assert_eq!(response.clearance_status(), Some(ClearanceStatus::Unknown));
        let results = response.validation_results().ok().unwrap();
        assert_eq!(text(results.status_raw()), "PARTIAL");
        assert_eq!(results.status(), Some(ValidationStatus::Unknown));
        let message = results.error_message(0).ok().unwrap();
        assert_eq!(text(message.message_type_raw()), "Notice");
        assert_eq!(message.message_type(), Some(MessageType::Unknown));
        assert_eq!(text(message.status_raw()), "WARN");
        assert_eq!(message.status(), Some(ValidationStatus::Unknown));
    }

    #[test]
    fn known_statuses_map_to_named_variants() {
        let response = ValidationResponse(
            serde_json::from_value(serde_json::json!({
                "reportingStatus": "NOT_REPORTED",
                "clearanceStatus": "NOT_CLEARED",
                "validationResults": {
                    "status": "PASS",
                    "errorMessages": [
                        {"type": "INFO", "status": "PASS"},
                        {"type": "ERROR", "status": "ERROR"},
                        {}
                    ]
                }
            }))
            .unwrap(),
        );
        assert_eq!(
            response.reporting_status(),
            Some(ReportingStatus::NotReported)
        );
        assert_eq!(
            response.clearance_status(),
            Some(ClearanceStatus::NotCleared)
        );
        let results = response.validation_results().ok().unwrap();
        assert_eq!(results.status(), Some(ValidationStatus::Pass));
        let info = results.error_message(0).ok().unwrap();
        assert_eq!(info.message_type(), Some(MessageType::Info));
        assert_eq!(info.status(), Some(ValidationStatus::Pass));
        let error = results.error_message(1).ok().unwrap();
        assert_eq!(error.message_type(), Some(MessageType::Error));
        assert_eq!(error.status(), Some(ValidationStatus::Error));
        let empty = results.error_message(2).ok().unwrap();
        assert!(empty.message_type().is_none());
        assert!(empty.status().is_none());
        assert!(empty.message_type_raw().ok().unwrap().is_none());
    }

    #[test]
    fn clearance_mode_maps_to_core() {
        use fatoora_core::api::ClearanceMode as Core;
        assert_eq!(Core::from(ClearanceMode::Enabled), Core::Enabled);
        assert_eq!(Core::from(ClearanceMode::Disabled), Core::Disabled);
        assert_eq!(ClearanceMode::from(Core::Enabled), ClearanceMode::Enabled);
    }
}
