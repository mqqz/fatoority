//! Shared findings and explicit validation coverage.
use super::super::{InvoiceField, ValidationError, ValidationIssue, ValidationKind};
use serde::{Deserialize, Serialize};

/// Results of completed checks, including checks that found errors.
///
/// An absent layer makes no claim. An empty issue list only means that the listed
/// layers found no issues; it does not establish complete invoice compliance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Layers whose checks completed; only these are covered by `issues`.
    pub layers_checked: Vec<ValidationLayer>,
    /// Findings from the checked layers, including warnings.
    pub issues: Vec<ValidationFinding>,
}

impl ValidationReport {
    /// Whether any completed check reported an error. Warnings do not count.
    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == Severity::Error)
    }
}

/// Independent validation layers. Each API reports only checks it completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ValidationLayer {
    /// Builder/model field checks (required values, formats and arithmetic).
    FieldChecks,
    /// UBL XML schema (XSD) validation.
    Xsd,
    /// CEN EN 16931 and KSA business rules.
    BusinessRules,
    /// Cryptographic signature, digest and certificate checks.
    Signature,
    /// QR payload consistency with the invoice and signature.
    Qr,
    /// Comparison of the embedded previous invoice hash with the expected predecessor.
    PreviousInvoiceHash,
}

/// A stable code and location for a human-readable finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Layer that produced the finding.
    pub layer: ValidationLayer,
    /// Library-owned rule identifier, independent of backend message wording.
    pub code: String,
    /// Whether the finding is a warning or an error.
    pub severity: Severity,
    /// Human-readable text; wording is not a stable API.
    pub message: String,
    /// None when the source cannot provide a reliable location.
    pub location: Option<ValidationLocation>,
}

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Reported but does not make the document invalid.
    Warning,
    /// Makes the document invalid.
    Error,
}

/// Where a validation finding applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ValidationLocation {
    /// Model field path, with zero-based indices (e.g. `line_items[0].quantity`).
    Field(String),
    /// Namespace-independent XPath selecting the original XML context.
    XPath(String),
    /// One-based XML source coordinates, when supplied by the backend.
    Xml {
        /// One-based line number.
        line: u32,
        /// One-based column number, if known.
        column: Option<u32>,
    },
}

impl From<&ValidationError> for ValidationReport {
    /// Convert existing field-check errors without running additional validation.
    fn from(error: &ValidationError) -> Self {
        Self {
            layers_checked: vec![ValidationLayer::FieldChecks],
            issues: error.issues().iter().map(ValidationFinding::from).collect(),
        }
    }
}

impl From<&ValidationIssue> for ValidationFinding {
    fn from(issue: &ValidationIssue) -> Self {
        let (code, description) = match issue.kind() {
            ValidationKind::Missing => ("FIELD_REQUIRED", "Required field is missing"),
            ValidationKind::Empty => ("FIELD_EMPTY", "Field must not be empty"),
            ValidationKind::InvalidFormat => {
                ("FIELD_INVALID_FORMAT", "Field has an invalid format")
            }
            ValidationKind::OutOfRange => ("FIELD_OUT_OF_RANGE", "Field is out of range"),
            ValidationKind::Mismatch => (
                "FIELD_MISMATCH",
                "Supplied value differs from the computed value",
            ),
        };
        let field = match issue.field() {
            InvoiceField::Id => "id",
            InvoiceField::Uuid => "uuid",
            InvoiceField::IssueDateTime => "issue_datetime",
            InvoiceField::Currency => "currency",
            InvoiceField::PreviousInvoiceHash => "previous_invoice_hash",
            InvoiceField::InvoiceCounter => "invoice_counter",
            InvoiceField::Seller => "seller",
            InvoiceField::Buyer => "buyer",
            InvoiceField::LineItems => "line_items",
            InvoiceField::PaymentMeansCode => "payment_means_code",
            InvoiceField::PaymentInstruction => "payment_instruction",
            InvoiceField::OriginalInvoiceId => "original_invoice_id",
            InvoiceField::VatCategory => "vat_category",
            InvoiceField::LineItemDocumentReferenceId => "line_items.document_references.id",
            InvoiceField::LineItemId => "line_items.id",
            InvoiceField::LineItemDescription => "line_items.description",
            InvoiceField::LineItemUnitCode => "line_items.unit_code",
            InvoiceField::LineItemQuantity => "line_items.quantity",
            InvoiceField::LineItemUnitPrice => "line_items.unit_price",
            InvoiceField::LineItemTotalAmount => "line_items.total_amount",
            InvoiceField::LineItemVatRate => "line_items.vat_rate",
            InvoiceField::LineItemVatAmount => "line_items.vat_amount",
            InvoiceField::InvoiceLevelDiscount => "invoice_level_discount",
            InvoiceField::InvoiceLevelCharge => "invoice_level_charge",
        };
        let location = match (issue.line_item_index(), field.strip_prefix("line_items.")) {
            (Some(index), Some(name)) => format!("line_items[{index}].{name}"),
            _ => field.to_owned(),
        };
        let mut message = description.to_owned();
        if let (Some(supplied), Some(expected)) = (issue.supplied(), issue.expected()) {
            message.push_str(&format!("; supplied {supplied}, expected {expected}"));
        }
        Self {
            layer: ValidationLayer::FieldChecks,
            code: code.to_owned(),
            severity: Severity::Error,
            message,
            location: Some(ValidationLocation::Field(location)),
        }
    }
}
