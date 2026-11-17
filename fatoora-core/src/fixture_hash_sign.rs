use std::path::{Path, PathBuf};

use crate::invoice::hash_xml;

const DS_NS: &str = "http://www.w3.org/2000/09/xmldsig#";

#[test]
fn fixture_invoices_match_hash_digest() {
    let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invoices");
    let files = collect_xml_files(&fixtures_root);
    assert!(!files.is_empty(), "no fixture invoices found");

    for file in files {
        let xml = std::fs::read_to_string(&file).expect("read fixture");
        let doc = roxmltree::Document::parse(&xml).expect("parse fixture");
        let reference = doc
            .descendants()
            .find(|node| {
                node.tag_name().namespace() == Some(DS_NS)
                    && node.tag_name().name() == "Reference"
                    && node.attribute("Id") == Some("invoiceSignedData")
            })
            .expect("invoice digest reference");
        let expected_invoice_digest = reference
            .children()
            .find(|node| {
                node.tag_name().namespace() == Some(DS_NS)
                    && node.tag_name().name() == "DigestValue"
            })
            .and_then(|node| node.text())
            .expect("invoice digest")
            .trim();
        let actual_invoice_digest = hash_xml(&xml).expect("invoice hash");
        assert_eq!(
            expected_invoice_digest,
            actual_invoice_digest.as_str(),
            "invoice hash mismatch for {}",
            file.display()
        );
        #[allow(deprecated)]
        let legacy = crate::invoice::sign::invoice_hash_base64_from_xml_str(&xml);
        assert_eq!(
            legacy.expect("legacy invoice hash"),
            actual_invoice_digest.as_str()
        );
    }
}

fn collect_xml_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).expect("read dir");
        for entry in entries {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("xml") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}
