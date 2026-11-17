use super::{ValidationResponse, ZatcaError};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::Infallible, fmt, str::FromStr};

/// Defines a non-exhaustive wire-status enum with an `Other(String)` fallback.
macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident => $wire:literal, )+
        }
    ) => {
        $(#[$meta])*
        ///
        /// Parsing is infallible and exact (case-sensitive): an unrecognized value
        /// becomes `Other` holding the raw text, so [`as_str`](Self::as_str),
        /// [`Display`](std::fmt::Display) and serde serialization reproduce the
        /// value exactly as ZATCA sent it. Parsing never puts a value that has a
        /// named variant into `Other`.
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(
                $(#[$vmeta])*
                ///
                #[doc = concat!("Wire value `", $wire, "`.")]
                $variant,
            )+
            /// A value not recognized by this version of the crate, preserved verbatim.
            Other(String),
        }

        impl $name {
            /// Returns the value as it appears on the wire.
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $wire,)+
                    Self::Other(value) => value,
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $($wire => Self::$variant,)+
                    other => Self::Other(other.to_owned()),
                }
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $($wire => Self::$variant,)+
                    _ => Self::Other(value),
                }
            }
        }

        impl FromStr for $name {
            type Err = Infallible;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self::from(value))
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                String::deserialize(deserializer).map(Self::from)
            }
        }
    };
}

wire_enum! {
    /// `reportingStatus` of a reporting response.
    ReportingStatus {
        /// The simplified invoice was reported.
        Reported => "REPORTED",
        /// The simplified invoice was rejected and not reported.
        NotReported => "NOT_REPORTED",
    }
}

wire_enum! {
    /// `clearanceStatus` of a clearance response.
    ClearanceStatus {
        /// The standard invoice was cleared.
        Cleared => "CLEARED",
        /// The standard invoice was rejected and not cleared.
        NotCleared => "NOT_CLEARED",
    }
}

wire_enum! {
    /// Overall `validationResults.status`, or the `status` of one validation message.
    ValidationStatus {
        /// Validation passed.
        Pass => "PASS",
        /// Validation passed with warnings.
        Warning => "WARNING",
        /// Validation failed.
        Error => "ERROR",
    }
}

wire_enum! {
    /// The `type` of a validation message.
    MessageType {
        /// An informational message.
        Info => "INFO",
        /// A warning message.
        Warning => "WARNING",
        /// An error message.
        Error => "ERROR",
    }
}

/// Outcome of the invoked operation. Compliance acceptance only means the check passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvoiceOutcome {
    /// The endpoint confirmed acceptance (`REPORTED`, `CLEARED`, or a
    /// compliance `PASS`/`WARNING`) with no error messages.
    Accepted,
    /// The endpoint explicitly rejected the invoice (`NOT_REPORTED`,
    /// `NOT_CLEARED`, or a compliance `ERROR`).
    Rejected,
    /// Missing, unrecognized, contradictory, or detached response context.
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum InvoiceOperation {
    Reporting,
    Clearance,
    Compliance,
}

/// An HTTP response retained on failure, including unrecognized gateway fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponseError {
    http_status: u16,
    body: String,
    validation_response: Option<ValidationResponse>,
}

impl HttpResponseError {
    /// Returns the HTTP status code of the response.
    pub fn http_status(&self) -> u16 {
        self.http_status
    }
    /// Response text, including fields not represented by the typed validation response.
    pub fn body(&self) -> &str {
        &self.body
    }
    /// Returns the body parsed as a [`ValidationResponse`], if it was one.
    pub fn validation_response(&self) -> Option<&ValidationResponse> {
        self.validation_response.as_ref()
    }
}

impl fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.http_status.fmt(f)
    }
}

impl ValidationResponse {
    /// Actual HTTP status, or `None` for a standalone deserialized JSON body.
    pub fn http_status(&self) -> Option<u16> {
        self.http_status
    }

    /// Classify only the invoked endpoint; unfamiliar or contradictory evidence is unknown.
    /// Warnings alone do not prevent acceptance. A compliance pass is not production acceptance.
    pub fn outcome(&self) -> InvoiceOutcome {
        let verdict = match self.operation {
            Some(InvoiceOperation::Reporting) => match self.reporting_status() {
                Some(ReportingStatus::Reported) => Some(true),
                Some(ReportingStatus::NotReported) => Some(false),
                _ => None,
            },
            Some(InvoiceOperation::Clearance) => match self.clearance_status() {
                Some(ClearanceStatus::Cleared) => Some(true),
                Some(ClearanceStatus::NotCleared) => Some(false),
                _ => None,
            },
            Some(InvoiceOperation::Compliance) => match self.validation_results().status() {
                Some(ValidationStatus::Pass | ValidationStatus::Warning) => Some(true),
                Some(ValidationStatus::Error) => Some(false),
                _ => None,
            },
            None => None,
        };
        if verdict == Some(false) {
            return InvoiceOutcome::Rejected;
        }
        if verdict == Some(true) {
            if self.validation_results().status() == Some(&ValidationStatus::Error)
                || !self.validation_results().error_messages().is_empty()
            {
                return InvoiceOutcome::Unknown;
            }
            return InvoiceOutcome::Accepted;
        }
        InvoiceOutcome::Unknown
    }

    /// Fail for rejected or unknown outcomes, retaining the response for inspection.
    ///
    /// # Errors
    /// Returns [`ZatcaError::NotAccepted`] unless [`ValidationResponse::outcome`]
    /// is [`InvoiceOutcome::Accepted`].
    pub fn ensure_accepted(&self) -> Result<(), ZatcaError> {
        if self.outcome() == InvoiceOutcome::Accepted {
            Ok(())
        } else {
            Err(ZatcaError::NotAccepted(Box::new(self.clone())))
        }
    }

    /// Exact base64 field returned by the gateway; presence does not establish authenticity.
    pub fn cleared_invoice_base64(&self) -> Option<&str> {
        self.cleared_invoice.as_deref()
    }

    /// Decode the returned invoice without XML parsing, normalization, or signature verification.
    /// Missing content returns `None`; empty, invalid base64, NUL, or invalid UTF-8 content is an error.
    ///
    /// # Errors
    /// Returns [`ZatcaError::ClearedInvoice`] if the field is not valid base64 or
    /// decodes to empty content, content containing NUL, or invalid UTF-8.
    pub fn cleared_invoice_xml(&self) -> Result<Option<String>, ZatcaError> {
        let Some(encoded) = self.cleared_invoice_base64() else {
            return Ok(None);
        };
        let fail = |message: String| ZatcaError::ClearedInvoice {
            http_status: self.http_status(),
            message,
        };
        let bytes = Base64::decode_vec(encoded).map_err(|e| fail(e.to_string()))?;
        if bytes.is_empty() {
            return Err(fail("empty cleared invoice".into()));
        }
        if bytes.contains(&0) {
            return Err(fail(
                "cleared invoice contains NUL, which is not valid XML text".into(),
            ));
        }
        String::from_utf8(bytes)
            .map(Some)
            .map_err(|e| fail(e.to_string()))
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
pub(super) async fn read_validation_response(
    response: reqwest::Response,
    operation: InvoiceOperation,
) -> Result<ValidationResponse, ZatcaError> {
    let status = response.status().as_u16();
    let body = response
        .bytes()
        .await
        .map_err(|e| ZatcaError::ResponseRead {
            http_status: status,
            message: e.to_string(),
        })?;
    decode_validation_response(status, &body, operation)
}

pub(super) fn decode_validation_response(
    status: u16,
    bytes: &[u8],
    operation: InvoiceOperation,
) -> Result<ValidationResponse, ZatcaError> {
    let body = String::from_utf8_lossy(bytes).into_owned();
    let parsed = serde_json::from_str::<ValidationResponse>(&body).map(|mut parsed| {
        parsed.http_status = Some(status);
        parsed.operation = Some(operation);
        parsed
    });
    if !(200..300).contains(&status) {
        return Err(ZatcaError::Response(Box::new(HttpResponseError {
            http_status: status,
            body,
            validation_response: parsed.ok(),
        })));
    }
    parsed.map_err(|e| ZatcaError::ResponseDecode {
        response: Box::new(HttpResponseError {
            http_status: status,
            body,
            validation_response: None,
        }),
        message: e.to_string(),
    })
}
