use fatoora_core::invoice::xml::parse::{parse_finalized_invoice_xml, parse_signed_invoice_xml};
use std::path::{Path, PathBuf};

fn collect_xml_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "xml") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn read_xml(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {:?}: {e:?}", path))
}

#[test]
fn parse_standard_and_simplified_invoices() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invoices");
    let standard = root.join("Standard/Invoice");
    let simplified = root.join("Simplified/Invoice");

    let files = collect_xml_files(&standard)
        .into_iter()
        .chain(collect_xml_files(&simplified))
        .collect::<Vec<_>>();

    assert!(!files.is_empty(), "no invoice fixtures found");

    for file in files {
        let xml = read_xml(&file);
        let invoice = parse_finalized_invoice_xml(&xml)
            .unwrap_or_else(|e| panic!("failed to parse {:?}: {e:?}", file));
        assert_roundtrip(&xml, &invoice);
        parse_signed_invoice_xml(&xml)
            .unwrap_or_else(|e| panic!("failed to parse signed {:?}: {e:?}", file));
    }
}

#[test]
fn parse_credit_debit_invoices() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invoices");
    let standard_credit = root.join("Standard/Credit");
    let standard_debit = root.join("Standard/Debit");
    let simplified_credit = root.join("Simplified/Credit");
    let simplified_debit = root.join("Simplified/Debit");

    let files = collect_xml_files(&standard_credit)
        .into_iter()
        .chain(collect_xml_files(&standard_debit))
        .chain(collect_xml_files(&simplified_credit))
        .chain(collect_xml_files(&simplified_debit))
        .collect::<Vec<_>>();

    assert!(!files.is_empty(), "no credit/debit fixtures found");

    for file in files {
        let xml = read_xml(&file);
        let invoice = parse_finalized_invoice_xml(&xml)
            .unwrap_or_else(|e| panic!("failed to parse {:?}: {e:?}", file));
        assert_roundtrip(&xml, &invoice);
        match invoice.data().invoice_type() {
            fatoora_core::invoice::InvoiceType::CreditNote(_, original, _)
            | fatoora_core::invoice::InvoiceType::DebitNote(_, original, _) => {
                assert_eq!(original.id(), "SME00002");
            }
            _ => panic!("expected credit/debit invoice type for {:?}", file),
        }
        parse_signed_invoice_xml(&xml)
            .unwrap_or_else(|e| panic!("failed to parse signed {:?}: {e:?}", file));
    }
}

fn assert_roundtrip(xml: &str, invoice: &fatoora_core::invoice::FinalizedInvoice) {
    let generated = invoice.to_xml().unwrap();
    let parsed = parse_finalized_invoice_xml(&generated).unwrap();
    assert_eq!(&parsed, invoice);
    fatoora_core::invoice::validation::validate_xml_invoice_from_str(
        &generated,
        &fatoora_core::config::Config::default(),
    )
    .unwrap();
    let signed = parse_signed_invoice_xml(xml).unwrap();
    assert_eq!(signed.xml(), xml);
    assert_ne!(signed.xml(), generated);
    let hash = fatoora_core::invoice::sign::invoice_hash_base64_from_xml_str(xml).unwrap();
    assert_eq!(
        hash,
        fatoora_core::invoice::sign::invoice_hash_base64_from_xml_str(signed.xml()).unwrap()
    );
}

#[test]
fn regenerated_compliant_fixtures_retain_business_rule_compliance() {
    use fatoora_core::invoice::validation::{
        Severity, ZatcaStage, ZatcaValidationOptions, validate_zatca_invoice_from_str,
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invoices");
    let mut compliant = 0;
    for family in ["Standard", "Simplified"] {
        for kind in ["Invoice", "Credit", "Debit"] {
            for file in collect_xml_files(&root.join(family).join(kind)) {
                let source = read_xml(&file);
                let invoice = parse_finalized_invoice_xml(&source).unwrap();
                let options = ZatcaValidationOptions {
                    evaluated_at: Some("2026-10-18T12:00:00Z".parse().unwrap()),
                    previous_invoice_hash: Some(invoice.data().previous_invoice_hash().into()),
                };
                let original = validate_zatca_invoice_from_str(
                    &source,
                    &fatoora_core::config::Config::default(),
                    &options,
                )
                .unwrap();
                let errors = |report: &fatoora_core::invoice::validation::ZatcaValidationReport| -> Vec<String> {
                    report.stages.iter().filter(|s| matches!(s.stage, ZatcaStage::Xsd | ZatcaStage::Cen | ZatcaStage::Ksa)).flat_map(|s| &s.findings).filter(|f| f.finding.severity == Severity::Error).map(|f| f.finding.code.clone()).collect()
                };
                if errors(&original).is_empty() {
                    compliant += 1;
                    let regenerated = validate_zatca_invoice_from_str(
                        &invoice.to_xml().unwrap(),
                        &fatoora_core::config::Config::default(),
                        &options,
                    )
                    .unwrap();
                    assert!(
                        errors(&regenerated).is_empty(),
                        "{file:?}: {:?}",
                        errors(&regenerated)
                    );
                }
            }
        }
    }
    assert!(compliant > 0);
}
