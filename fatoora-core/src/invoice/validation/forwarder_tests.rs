//! The `Validator` entry points must match the deprecated `*_from_str` forwarders.
#![allow(deprecated)]
use super::*;
use std::path::{Path, PathBuf};

fn collect_xml(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read fixture dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            collect_xml(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "xml") {
            out.push(path);
        }
    }
}

fn inputs() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut files = Vec::new();
    for dir in ["invoices", "malformed", "sdk-parity"] {
        collect_xml(&root.join(dir), &mut files);
    }
    files.sort();
    let mut inputs: Vec<_> = files
        .into_iter()
        .map(|path| {
            let xml = std::fs::read_to_string(&path).expect("read fixture");
            (path.display().to_string(), xml)
        })
        .collect();
    let standard =
        include_str!("../../../tests/fixtures/sdk-parity/cases/standard-invoice/input.xml");
    for (name, xml) in [
        ("wrong root", "<wrong/>".to_owned()),
        ("truncated", "<Invoice".to_owned()),
        ("empty", String::new()),
        (
            "business rejection",
            standard.replace(
                "<cbc:UUID>b2a43c49-3aab-4e3b-9d67-0da45a5e33cc</cbc:UUID>",
                "",
            ),
        ),
    ] {
        inputs.push((name.to_owned(), xml));
    }
    inputs
}

fn options() -> ZatcaValidationOptions {
    ZatcaValidationOptions {
        evaluated_at: Some("2026-09-23T12:00:00+03:00".parse().unwrap()),
        previous_invoice_hash: Some(
            "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ=="
                .parse()
                .unwrap(),
        ),
    }
}

#[test]
fn validator_matches_deprecated_free_functions() {
    let validator = Validator::new();
    let config = Config::default();
    let options = options();
    let (mut accepted, mut rejected, mut zatca_valid) = (0, 0, 0);
    for (name, xml) in inputs() {
        let new = validator.validate_xml(&xml);
        let old = validate_xml_invoice_from_str(&xml, &config);
        assert_eq!(
            format!("{new:?}"),
            format!("{old:?}"),
            "validate_xml: {name}"
        );
        if new.is_ok() {
            accepted += 1;
        } else {
            rejected += 1;
        }

        let new = validator.validate_xml_report(&xml);
        let old = validate_xml_invoice_report_from_str(&xml, &config);
        assert_eq!(
            format!("{new:?}"),
            format!("{old:?}"),
            "validate_xml_report: {name}"
        );

        match (
            validator.validate_zatca(&xml, &options),
            validate_zatca_invoice_from_str(&xml, &config, &options),
        ) {
            (Ok(new), Ok(old)) => {
                zatca_valid += usize::from(new.is_valid());
                assert_eq!(new, old, "validate_zatca: {name}");
            }
            (Err(new), Err(old)) => {
                assert_eq!(
                    format!("{new:?}"),
                    format!("{old:?}"),
                    "validate_zatca: {name}"
                )
            }
            (new, old) => panic!("validate_zatca diverged for {name}: {new:?} vs {old:?}"),
        }
    }
    assert!(
        accepted > 0 && rejected > 0 && zatca_valid > 0,
        "{accepted} accepted, {rejected} rejected, {zatca_valid} ZATCA-valid"
    );
}
