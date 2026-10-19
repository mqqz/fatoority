//! Replay deterministic fields from a live pinned-SDK comparison.
#[path = "support/sdk_checks.rs"]
#[allow(dead_code)]
mod independent;
use base64ct::{Base64, Encoding};
use fatoora_core::invoice::sign::InvoiceSigner;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

#[test]
fn generated_fields_match_recorded_sdk_and_explicit_currency_policy() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let report: Value =
        serde_json::from_slice(&fs::read(root.join("qr-correctness/report.json")).unwrap())
            .unwrap();
    assert_eq!(report["sdk_version"], "238-R3.4.8");
    assert_eq!(
        report["jar_sha256"],
        "48abeb828d453ef6fafba792fddbbb2701da5c7018c24bde918853e80ff5d530"
    );
    assert_eq!(report["installed_sdk_unchanged"], true);
    assert_eq!(report["results"].as_object().unwrap().len(), 16);
    let signer = InvoiceSigner::from_der(
        &fs::read(root.join("sdk-parity/credentials/certificate.der")).unwrap(),
        &fs::read(root.join("sdk-parity/credentials/private-key.der")).unwrap(),
    )
    .unwrap();
    let amounts = regex::Regex::new(r"(?P<open><cbc:(?:PayableAmount|TaxAmount)\b[^>]*>)[^<]*(?P<close></cbc:(?:PayableAmount|TaxAmount)>)").unwrap();
    for (name, case) in report["results"].as_object().unwrap() {
        if name == "qr-total-scale" {
            continue;
        }
        let input = if let Some(amount) = name.strip_prefix("decimal-") {
            let base =
                fs::read_to_string(root.join("sdk-parity/cases/simplified-invoice/input.xml"))
                    .unwrap();
            amounts
                .replace_all(&base, |c: &regex::Captures<'_>| {
                    format!("{}{amount}{}", &c["open"], &c["close"])
                })
                .into_owned()
        } else if name == "foreign-currency-vat" {
            fs::read_to_string(root.join("invoices/Standard/Invoice/Advance Payment adjustments with foreign currency invoice.xml")).unwrap()
        } else {
            fs::read_to_string(root.join(format!("sdk-parity/cases/{name}/input.xml"))).unwrap()
        };
        assert_eq!(
            format!("{:x}", Sha256::digest(input.as_bytes())),
            case["input_sha256"],
            "{name}"
        );
        let xml = signer.sign_xml(&input).unwrap();
        if !name.starts_with("decimal-") {
            let imported =
                fatoora_core::invoice::xml::parse::parse_signed_invoice_xml(&xml).unwrap();
            assert_eq!(imported.xml(), xml);
            assert_eq!(imported.qr_code(), independent::text(&independent::document(&xml).unwrap(), "//cac:AdditionalDocumentReference[cbc:ID='QR']/cac:Attachment/cbc:EmbeddedDocumentBinaryObject").unwrap());
        }
        let doc = independent::document(&xml).unwrap();
        let tags = independent::qr(independent::context(&doc)).unwrap();
        assert_eq!(
            tags.keys().copied().collect::<Vec<_>>(),
            serde_json::from_value::<Vec<u8>>(case["fatoority"]["tags"].clone()).unwrap()
        );
        for (tag, key) in [(4, "tag4"), (5, "tag5")] {
            assert_eq!(
                std::str::from_utf8(&tags[&tag]).unwrap(),
                case["fatoority"][key].as_str().unwrap(),
                "{name} tag {tag}"
            );
            if name != "foreign-currency-vat" {
                assert_eq!(case["sdk"][key], case["fatoority"][key], "{name}");
            }
        }
        assert_eq!(
            tags.get(&9).map(|v| Base64::encode_string(v)),
            case["sdk"]["tag9_base64"].as_str().map(str::to_owned)
        );
    }
    assert_eq!(
        report["results"]["foreign-currency-vat"]["sdk"]["tag5"],
        "1068.75"
    );
    assert_eq!(
        report["results"]["foreign-currency-vat"]["fatoority"]["tag5"],
        "285"
    );
    assert_eq!(
        report["results"]["qr-total-scale"]["sdk_validation"]["layers"]["qr"],
        "failed"
    );
    assert_eq!(
        report["results"]["qr-total-scale"]["fatoority_sdk_qr"]["findings"][0]["code"],
        "QR_VALUE"
    );
}

#[test]
fn development_capture_keeps_acceptance_and_negative_controls_distinct() {
    let report: Value =
        serde_json::from_str(include_str!("fixtures/qr-correctness/development-api.json")).unwrap();
    assert_eq!(report["environment"], "developer-portal");
    assert_eq!(report["cases"].as_object().unwrap().len(), 15);
    for name in [
        "tag4-equivalent-scale",
        "tag4-equivalent-integer",
        "tag4-wrong-value",
    ] {
        assert_eq!(report["cases"][name]["http_status"], 400);
        assert_eq!(
            report["cases"][name]["response"]["validationResults"]["errorMessages"][0]["code"],
            "invoiceTotal_QRCODE_INVALID"
        );
    }
    // These accepted negative controls are observations, never native policy.
    assert_eq!(
        report["cases"]["tag5-wrong-value"]["response"]["validationResults"]["status"],
        "PASS"
    );
    for name in ["foreign-currency-bt110", "foreign-currency-bt111"] {
        assert_eq!(
            report["cases"][name]["response"]["validationResults"]["status"],
            "PASS"
        );
    }
    assert_eq!(
        report["cases"]["simplified-missing-tag9"]["http_status"],
        400
    );
    assert_eq!(
        report["cases"]["standard-unexpected-tag9"]["response"]["clearanceStatus"],
        "CLEARED"
    );
}
