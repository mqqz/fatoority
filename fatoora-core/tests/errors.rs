use std::error::Error as StdError;

use fatoora_core::api::{ServerErrorResponse, ZatcaError};
use fatoora_core::config::Config;
use fatoora_core::csr::{Csr, CsrError, CsrProperties};
use fatoora_core::invoice::sign::SigningError;
use fatoora_core::invoice::validation::{XmlValidationError, validate_xml_invoice_from_str};
use fatoora_core::invoice::{InvoiceError, InvoiceField, LineItem, VatCategory};
use fatoora_core::{Decimal, Diagnostic, Error, ErrorKind};
use serde_json::{Value, json};

fn decimal(value: &str) -> Decimal {
    Decimal::parse(value).unwrap()
}

#[test]
fn wrapping_retains_validation_fields_and_exact_amounts() {
    let original = LineItem::try_from_parts(
        "item",
        decimal("1"),
        "PCE",
        decimal("100"),
        decimal("99"),
        decimal("15"),
        decimal("10"),
        VatCategory::Standard,
    )
    .unwrap_err();
    let error: Error = original.into();
    assert_eq!(error.kind(), ErrorKind::Validation);
    let Error::Invoice(InvoiceError::Validation(validation)) = &error else {
        panic!("lost module error: {error}");
    };
    let issue = &validation.issues()[0];
    assert_eq!(issue.field(), InvoiceField::LineItemTotalAmount);
    assert_eq!(issue.supplied(), Some(decimal("99")));
    assert_eq!(issue.expected(), Some(decimal("100")));
    assert!(error.source().unwrap().is::<InvoiceError>());
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    assert_eq!(
        details,
        json!({"type": "invoice_validation", "issues": [{
            "field": "line_item_total_amount", "kind": "mismatch", "line_item_index": null,
            "supplied": "99", "expected": "100"
        }]})
    );
}

#[test]
fn nested_signing_error_retains_validation_details() {
    let invoice_error =
        InvoiceError::Validation(fatoora_core::invoice::ValidationError::new(vec![
            fatoora_core::invoice::ValidationIssue::new(
                InvoiceField::LineItemDescription,
                fatoora_core::invoice::ValidationKind::Empty,
                Some(2),
            ),
        ]));
    let error: Error = SigningError::from(invoice_error).into();
    assert_eq!(error.kind(), ErrorKind::Validation);
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    assert_eq!(details["type"], "invoice_validation");
    assert_eq!(details["issues"][0]["line_item_index"], 2);
}

#[test]
fn schema_diagnostics_survive_conversion() {
    let error: Error = validate_xml_invoice_from_str("<Invoice/>\n", &Config::default())
        .unwrap_err()
        .into();
    assert_eq!(error.kind(), ErrorKind::Validation);
    let Error::XmlValidation(XmlValidationError::SchemaValidation { errors }) = &error else {
        panic!("expected schema diagnostics: {error}");
    };
    assert!(!errors.is_empty());
    assert!(!errors[0].message().is_empty());
    assert!(errors[0].source().is_none());
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    assert_eq!(details["type"], "schema_validation");
    assert_eq!(details["diagnostics"][0]["message"], errors[0].message());
    assert_eq!(details["diagnostics"][0]["line"], json!(errors[0].line()));
}

#[test]
fn csr_backend_sources_are_owned_diagnostics() {
    let error: Error = Csr::from_der(b"not DER").unwrap_err().into();
    let csr = error.source().unwrap();
    assert!(csr.is::<CsrError>());
    let diagnostic = csr.source().unwrap().downcast_ref::<Diagnostic>().unwrap();
    assert!(!diagnostic.message().is_empty());
    assert!(diagnostic.source().is_none());

    let error = CsrProperties::from_properties_str("name=\\uZZZZ").unwrap_err();
    let CsrError::PropertiesRead { source, .. } = error else {
        panic!("expected properties parse failure");
    };
    assert_eq!(source.line(), Some(1));
    assert!(source.source().is_none());
}

#[test]
fn api_response_fields_survive_and_json_escapes_diagnostics() {
    let response: ServerErrorResponse = serde_json::from_value(json!({
        "category": "validation", "code": "BR-01", "message": "خطأ\u{0}\n\"quoted\""
    }))
    .unwrap();
    let error: Error = ZatcaError::ServerError(response).into();
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    assert_eq!(error.kind(), ErrorKind::Api);
    assert_eq!(details["response"]["code"], "BR-01");
    assert_eq!(details["response"]["message"], "خطأ\u{0}\n\"quoted\"");
}

#[test]
fn unreadable_file_keeps_io_classification_and_path() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/nonexistent-issue-9/invoice.xml");
    let error: Error = fatoora_core::invoice::xml::parse::parse_finalized_invoice_xml_file(&path)
        .unwrap_err()
        .into();
    assert_eq!(error.kind(), ErrorKind::Io);
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    assert_eq!(details["type"], "io");
    assert_eq!(details["path"], path.to_string_lossy().as_ref());
}

#[test]
fn incomplete_builder_reports_every_missing_field_to_bindings() {
    use fatoora_core::invoice::validation::{
        ValidationLayer, ValidationLocation, ValidationReport,
    };
    use fatoora_core::invoice::{InvoiceBuilder, InvoiceSubType, InvoiceType};
    let original = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .build()
        .unwrap_err();
    let InvoiceError::Validation(validation) = &original else {
        panic!("expected field checks")
    };
    let report = ValidationReport::from(validation);
    assert_eq!(report.layers_checked, [ValidationLayer::FieldChecks]);
    assert!(report.has_errors());
    let expected = [
        "id",
        "uuid",
        "issue_datetime",
        "currency",
        "previous_invoice_hash",
        "invoice_counter",
        "seller",
        "vat_category",
        "line_items",
    ];
    let locations: Vec<_> = report
        .issues
        .iter()
        .map(|finding| {
            assert_eq!(finding.code, "FIELD_REQUIRED");
            let Some(ValidationLocation::Field(field)) = &finding.location else {
                panic!("missing field location")
            };
            field.as_str()
        })
        .collect();
    assert_eq!(locations, expected);
    let error: Error = original.into();
    assert_eq!(error.kind(), ErrorKind::Validation);
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    let issues = details["issues"].as_array().unwrap();
    assert_eq!(issues.len(), expected.len());
    for (issue, field) in issues.iter().zip(expected) {
        // The established error JSON uses issue_date_time; reports use the model path.
        assert_eq!(
            issue["field"],
            if field == "issue_datetime" {
                "issue_date_time"
            } else {
                field
            }
        );
        assert_eq!(issue["kind"], "missing");
        assert!(issue["line_item_index"].is_null());
    }
}

#[test]
fn invalid_imported_line_reports_zero_based_locations_and_numeric_ranges() {
    use fatoora_core::invoice::validation::{ValidationLocation, ValidationReport};
    use fatoora_core::invoice::{InvoiceBuilder, InvoiceSubType, InvoiceType};
    let good = LineItem::new(
        "good",
        decimal("1"),
        "PCE",
        decimal("100"),
        decimal("15"),
        VatCategory::Standard,
    )
    .unwrap();
    let bad: LineItem = serde_json::from_value(json!({
        "description":"", "quantity":"-1", "unit_code":"", "unit_price":"-1",
        "total_amount":"-1", "vat_rate":"101", "vat_amount":"-0.15", "vat_category":"Standard"
    }))
    .unwrap();
    let original = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .issue_datetime("not-a-date")
        .currency("invalid")
        .invoice_level_discount(decimal("-1"))
        .invoice_level_charge(decimal("0.001"))
        .line_item(good)
        .line_item(bad)
        .build()
        .unwrap_err();
    let InvoiceError::Validation(validation) = &original else {
        panic!("expected field checks")
    };
    let report = ValidationReport::from(validation);
    for (field, code) in [
        ("line_items[1].description", "FIELD_EMPTY"),
        ("line_items[1].quantity", "FIELD_OUT_OF_RANGE"),
        ("line_items[1].unit_price", "FIELD_OUT_OF_RANGE"),
        ("line_items[1].total_amount", "FIELD_OUT_OF_RANGE"),
        ("line_items[1].vat_rate", "FIELD_OUT_OF_RANGE"),
        ("line_items[1].vat_amount", "FIELD_OUT_OF_RANGE"),
        ("invoice_level_discount", "FIELD_OUT_OF_RANGE"),
        ("invoice_level_charge", "FIELD_OUT_OF_RANGE"),
        ("issue_datetime", "FIELD_INVALID_FORMAT"),
        ("currency", "FIELD_INVALID_FORMAT"),
    ] {
        assert!(
            report.issues.iter().any(|finding| finding.code == code
                && finding.location == Some(ValidationLocation::Field(field.into()))),
            "{field}"
        );
    }
    let error: Error = original.into();
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    let issues = details["issues"].as_array().unwrap();
    for field in [
        "line_item_description",
        "line_item_quantity",
        "line_item_unit_price",
        "line_item_total_amount",
        "line_item_vat_rate",
        "line_item_vat_amount",
    ] {
        assert!(
            issues
                .iter()
                .any(|issue| issue["field"] == field && issue["line_item_index"] == 1)
        );
    }
    assert!(issues.iter().all(|issue| issue["line_item_index"] != 0));
}

#[test]
fn binding_error_schema_preserves_nested_failures_and_stable_classifications() {
    use fatoora_core::DecimalError;
    use fatoora_core::invoice::QrCodeError;
    use fatoora_core::invoice::xml::{InvoiceXmlError, parse::ParseError};

    // These are public error values: downstream adapters may receive any variant.
    // Pin complete wire objects so changes cannot silently drop useful fields.
    let message = "خطأ\u{0}\n\"quoted\"";
    let diagnostic =
        json!({"message":message,"file":null,"line":null,"column":null,"severity":null});
    let cases: Vec<(Error, ErrorKind, Value)> = vec![
        (
            QrCodeError::ValueTooLong { tag: 1, len: 256 }.into(),
            ErrorKind::InvalidInput,
            json!({"type":"qr_value_too_long","tag":1,"length":256,"limit":255}),
        ),
        (
            QrCodeError::EncodedTooLong { len: 704 }.into(),
            ErrorKind::InvalidInput,
            json!({"type":"qr_encoded_too_long","length":704,"limit":700}),
        ),
        (
            QrCodeError::MissingSellerName.into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_seller_name"}),
        ),
        (
            QrCodeError::MissingSellerVat.into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_seller_vat"}),
        ),
        (
            QrCodeError::Xml(message.into()).into(),
            ErrorKind::Xml,
            json!({"type":"qr_xml","diagnostics":[{"message":message}]}),
        ),
        (
            InvoiceXmlError::Serialize {
                source: Diagnostic::new(message),
            }
            .into(),
            ErrorKind::Xml,
            json!({"type":"xml_serialize","diagnostics":[diagnostic.clone()]}),
        ),
        (
            SigningError::Serialize(InvoiceXmlError::Serialize {
                source: Diagnostic::new(message),
            })
            .into(),
            ErrorKind::Xml,
            json!({"type":"xml_serialize","diagnostics":[diagnostic.clone()]}),
        ),
        (
            SigningError::SigningError(message.into()).into(),
            ErrorKind::Crypto,
            json!({"type":"signing","diagnostics":[{"message":message}]}),
        ),
        (
            SigningError::Qr(QrCodeError::MissingSellerName).into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_seller_name"}),
        ),
        (
            ParseError::Qr(QrCodeError::MissingSellerVat).into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_seller_vat"}),
        ),
        (
            ParseError::Invoice(InvoiceError::MissingBuyerId).into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_buyer_id"}),
        ),
        (
            ParseError::Decimal(DecimalError::OutOfRange).into(),
            ErrorKind::InvalidInput,
            json!({"type":"decimal_out_of_range"}),
        ),
        (
            ParseError::XPath(message.into()).into(),
            ErrorKind::Parse,
            json!({"type":"xpath","diagnostics":[{"message":message}]}),
        ),
        (
            ParseError::XmlParse(message.into()).into(),
            ErrorKind::Xml,
            json!({"type":"xml_parse","diagnostics":[{"message":message}]}),
        ),
        (
            InvoiceError::Decimal(DecimalError::InvalidSyntax).into(),
            ErrorKind::InvalidInput,
            json!({"type":"invalid_decimal"}),
        ),
        (
            InvoiceError::InvalidCountryCode("ZZ".into()).into(),
            ErrorKind::InvalidInput,
            json!({"type":"invalid_country_code","value":"ZZ"}),
        ),
        (
            InvoiceError::MissingVatForSeller.into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_seller_vat"}),
        ),
        (
            InvoiceError::MissingBuyerId.into(),
            ErrorKind::InvalidInput,
            json!({"type":"missing_buyer_id"}),
        ),
        (
            InvoiceError::InvalidVatFormat.into(),
            ErrorKind::InvalidInput,
            json!({"type":"invalid_vat_format"}),
        ),
        (
            XmlValidationError::SchemaParse {
                errors: vec![Diagnostic::new(message)],
            }
            .into(),
            ErrorKind::Parse,
            json!({"type":"schema_parse","diagnostics":[diagnostic]}),
        ),
        (
            XmlValidationError::InvalidXsdPath {
                path: "missing.xsd".into(),
            }
            .into(),
            ErrorKind::InvalidInput,
            json!({"type":"invalid_xsd_path","path":"missing.xsd"}),
        ),
        (
            XmlValidationError::XmlParse {
                message: message.into(),
            }
            .into(),
            ErrorKind::Xml,
            json!({"type":"xml_parse","diagnostics":[{"message":message}]}),
        ),
        (
            ZatcaError::ResponseRead {
                http_status: 502,
                message: message.into(),
            }
            .into(),
            ErrorKind::Network,
            json!({"type":"api_response_read","http_status":502,"message":message}),
        ),
        (
            ZatcaError::NetworkError(message.into()).into(),
            ErrorKind::Network,
            json!({"type":"network","diagnostics":[{"message":message}]}),
        ),
        (
            ZatcaError::Http(message.into()).into(),
            ErrorKind::Network,
            json!({"type":"http","diagnostics":[{"message":message}]}),
        ),
    ];
    for (error, kind, expected) in cases {
        assert_eq!(error.kind(), kind, "{error}");
        let details: Value = serde_json::from_str(&error.details_json()).unwrap();
        assert_eq!(details, expected, "{error}");
    }
}

#[test]
fn csr_error_schema_preserves_failure_phase_and_extension_name() {
    let message = "invalid encoding";
    let cases = [
        (
            CsrError::InvalidSubject {
                message: message.into(),
            },
            ErrorKind::InvalidInput,
            json!({"type":"invalid_subject","diagnostics":[{"message":message}]}),
        ),
        (
            CsrError::InvalidSan {
                message: message.into(),
            },
            ErrorKind::InvalidInput,
            json!({"type":"invalid_san","diagnostics":[{"message":message}]}),
        ),
        (
            CsrError::RequestBuild {
                message: message.into(),
            },
            ErrorKind::Crypto,
            json!({"type":"csr_request","diagnostics":[{"message":message}]}),
        ),
        (
            CsrError::AddExtension {
                which: "subjectAltName",
                message: message.into(),
            },
            ErrorKind::Crypto,
            json!({"type":"csr_extension","extension":"subjectAltName","diagnostics":[{"message":message}]}),
        ),
        (
            CsrError::CsrBuild {
                message: message.into(),
            },
            ErrorKind::Crypto,
            json!({"type":"csr_build","diagnostics":[{"message":message}]}),
        ),
        (
            CsrError::KeyEncode {
                message: message.into(),
            },
            ErrorKind::Crypto,
            json!({"type":"key_encode","diagnostics":[{"message":message}]}),
        ),
    ];
    for (error, kind, expected) in cases {
        let error: Error = error.into();
        assert_eq!(error.kind(), kind);
        let details: Value = serde_json::from_str(&error.details_json()).unwrap();
        assert_eq!(details, expected);
    }
}
