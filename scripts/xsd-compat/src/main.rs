//! Differential feasibility probe; never used by the production validator.
use libxml::{
    parser::{Parser, ParserOptions},
    schemas::{SchemaParserContext, SchemaValidationContext},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{path::Path, process::Command};
use xsd_schema::{
    SchemaSet, SchemaSetBuilder,
    validation::{
        ErrorOnlySink, SchemaValidator, SchemaValidity, ValidationFlags, drive_quick_xml,
    },
};

fn candidate(schema: &SchemaSet, xml: &str) -> (bool, Vec<String>) {
    let validator = SchemaValidator::new(
        schema,
        ValidationFlags::default() | ValidationFlags::PROCESS_IDENTITY_CONSTRAINTS,
    );
    let mut errors = Vec::new();
    let outcome = {
        let mut runtime = validator.start_run(ErrorOnlySink {
            errors: &mut errors,
        });
        // Unlike drive_quick_xml_with, this helper finalizes the runtime.
        // Both deferred sink diagnostics and completion errors count below.
        drive_quick_xml(xml.as_bytes(), &mut runtime, schema)
    };
    let accepted = matches!(&outcome, Ok(outcome) if outcome.root_validity == Some(SchemaValidity::Valid))
        && errors.is_empty();
    let mut diagnostics: Vec<_> = errors.iter().map(ToString::to_string).collect();
    if let Err(error) = outcome {
        diagnostics.push(error.to_string());
    }
    (accepted, diagnostics)
}

fn compare(
    schema: &SchemaSet,
    baseline: &mut SchemaValidationContext,
    name: &str,
    xml: &str,
    expected: Option<bool>,
    bundled: bool,
) -> Value {
    let parsed = Parser::default().parse_string_with_options(
        xml,
        ParserOptions {
            recover: false,
            no_net: true,
            ..Default::default()
        },
    );
    let (old, old_errors) = match parsed {
        Ok(document) => match baseline.validate_document(&document) {
            Ok(()) => (true, Vec::new()),
            Err(errors) => (false, errors.iter().map(|e| format!("{e:?}")).collect()),
        },
        Err(error) => (false, vec![format!("{error:?}")]),
    };
    let (raw, errors) = candidate(schema, xml);
    // Issue #4 assigns document well-formedness to roxmltree. Keep raw results
    // visible so parser gaps are not mistaken for schema correctness.
    let well_formed = roxmltree::Document::parse(xml).is_ok();
    let production =
        bundled.then(|| fatoora_core::invoice::validation::Validator::new().validate_xml(xml));
    let new = production
        .as_ref()
        .map_or(well_formed && raw, Result::is_ok);
    json!({"case": name, "libxml": old, "xsd_schema": new, "raw_xsd_schema": raw,
        "well_formed": well_formed, "expected": expected, "matches": old == new,
        "expected_matches": expected.is_none_or(|e| new == e),
        "sha256": format!("{:x}", Sha256::digest(xml.as_bytes())),
        "production_error": production.and_then(Result::err).map(|e| format!("{e:?}")), "xsd_errors": errors, "libxml_errors": old_errors})
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let xsd = root.join("fatoora-core/assets/schemas/UBL2.1/xsd/maindoc/UBL-Invoice-2.1.xsd");
    let schema = SchemaSetBuilder::new()
        .add("", xsd.to_str().unwrap())
        .unwrap()
        .compile()
        .expect("unchanged UBL/XAdES schema must compile")
        .into_schema_set();
    let _ = Parser::default();
    let mut parser = SchemaParserContext::from_file(xsd.to_str().unwrap());
    let mut baseline = SchemaValidationContext::from_parser(&mut parser).unwrap();
    let files = Command::new("git")
        .args(["ls-files", "-z", "fatoora-core/tests/fixtures"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(files.status.success());
    let verified_artifacts = verify_artifacts(&root, &files.stdout);
    let mut results = Vec::new();
    for file in files
        .stdout
        .split(|b| *b == 0)
        .filter(|p| p.ends_with(b".xml"))
    {
        let name = std::str::from_utf8(file).unwrap();
        let xml = std::fs::read_to_string(root.join(name)).unwrap();
        let expected_path = root.join(name).with_file_name("expected.json");
        let expected = if name.ends_with("/input.xml") && expected_path.exists() {
            let value: Value =
                serde_json::from_slice(&std::fs::read(expected_path).unwrap()).unwrap();
            match value["layers"]["xsd"].as_str() {
                Some("passed") => Some(true),
                Some("failed") => Some(false),
                _ => None,
            }
        } else {
            None
        };
        results.push(compare(&schema, &mut baseline, name, &xml, expected, true));
    }
    let sample = std::fs::read_to_string(
        root.join("fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml"),
    )
    .unwrap();
    let doc = roxmltree::Document::parse(&sample).unwrap();
    let cbc = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";
    let node = |name| {
        doc.root_element()
            .children()
            .find(|n| n.has_tag_name((cbc, name)))
            .unwrap()
    };
    let date = node("IssueDate");
    let id = node("ID");
    let mut mutations = vec![("valid-control", sample.clone(), true)];
    for (name, range, replacement) in [
        ("missing-id", id.range(), String::new()),
        ("duplicate-id", id.range(), sample[id.range()].repeat(2)),
        (
            "invalid-date",
            date.range(),
            "<cbc:IssueDate>2026-02-30</cbc:IssueDate>".into(),
        ),
        (
            "date-child",
            date.range(),
            "<cbc:IssueDate><cbc:ID>2026-01-01</cbc:ID></cbc:IssueDate>".into(),
        ),
        (
            "date-attribute",
            date.range(),
            "<cbc:IssueDate unexpected=\"x\">2026-01-01</cbc:IssueDate>".into(),
        ),
        (
            "wrong-namespace",
            date.range(),
            "<IssueDate xmlns=\"urn:wrong\">2026-01-01</IssueDate>".into(),
        ),
    ] {
        let mut xml = sample.clone();
        xml.replace_range(range, &replacement);
        mutations.push((name, xml, false));
    }
    let mut reordered = sample.clone();
    assert!(date.range().start > id.range().end);
    reordered.replace_range(date.range(), "");
    reordered.insert_str(id.range().start, &sample[date.range()]);
    mutations.push(("wrong-order", reordered, false));
    for (name, xml) in [
        ("empty", "".into()),
        ("truncated", "<Invoice".into()),
        ("unknown-root", "<unknown/>".into()),
        ("multiple-roots", format!("{sample}<extra/>")),
    ] {
        mutations.push((name, xml, false));
    }
    for (name, xml, expected) in mutations {
        results.push(compare(
            &schema,
            &mut baseline,
            name,
            &xml,
            Some(expected),
            true,
        ));
    }
    let mismatches = results
        .iter()
        .filter(|r| r["matches"] == false || r["expected_matches"] == false)
        .count();
    let minimal = minimal_cases();
    let minimal_failures = minimal
        .iter()
        .filter(|r| r["expected_matches"] == false)
        .count();
    let sdk: Vec<_> = results
        .iter()
        .filter(|r| r["case"].as_str().unwrap().ends_with("/input.xml") && !r["expected"].is_null())
        .collect();
    let report = json!({"upstream_version": "0.2.2", "candidate": "fatoora-core decimal adapter", "verified_artifacts": verified_artifacts,
        "cases": results.len(), "libxml_accepted": results.iter().filter(|r| r["libxml"] == true).count(),
        "candidate_accepted": results.iter().filter(|r| r["xsd_schema"] == true).count(), "mismatches": mismatches,
        "sdk_cases": sdk.len(), "sdk_mismatches": sdk.iter().filter(|r| r["expected_matches"] == false).count(),
        "minimal_failures": minimal_failures, "minimal": minimal, "results": results});
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    if mismatches != 0 || minimal_failures != 0 {
        std::process::exit(1);
    }
}

fn verify_artifacts(root: &Path, files: &[u8]) -> usize {
    let mut count = 0;
    for name in files
        .split(|b| *b == 0)
        .filter(|p| p.ends_with(b"/manifest.json"))
    {
        let path = root.join(std::str::from_utf8(name).unwrap());
        let manifest: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        if let Some(artifacts) = manifest["artifacts"].as_object() {
            for (relative, expected) in artifacts {
                let artifact = path.parent().unwrap().join(relative);
                let actual = format!("{:x}", Sha256::digest(std::fs::read(&artifact).unwrap()));
                assert_eq!(
                    Some(actual.as_str()),
                    expected.as_str(),
                    "artifact changed: {}",
                    artifact.display()
                );
                count += 1;
            }
        }
    }
    count
}

fn minimal_cases() -> Vec<Value> {
    let mut results = Vec::new();
    for (kind, value, expected) in [
        ("time", "09:15:00", true),
        ("time", " 09:15:00 ", true),
        ("time", "09:15:00+14:00", true),
        ("time", "09:15:00-14:00", true),
        ("time", "09:15:00+14:01", false),
        ("time", "09:15:00-14:59", false),
        ("time", "09:15:00+15:00", false),
        ("time", "09:15:00+01:60", false),
        ("time", "09:15:00+1:00", false),
        ("date", "2026-01-01+15:00", false),
        ("dateTime", "2026-01-01T09:15:00+15:00", false),
        ("decimal", "1.25", true),
        ("decimal", "10000000000000000000000000000000000000000", true),
        ("decimal", "1e2", false),
    ] {
        let source = format!(
            r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"><xs:element name="v" type="xs:{kind}"/></xs:schema>"#
        );
        let schema = SchemaSetBuilder::new()
            .add_source(&source, "minimal.xsd")
            .unwrap()
            .compile()
            .unwrap()
            .into_schema_set();
        let mut parser = SchemaParserContext::from_buffer(source.as_bytes());
        let mut baseline = SchemaValidationContext::from_parser(&mut parser).unwrap();
        let xml = format!("<v>{value}</v>");
        let mut result = compare(
            &schema,
            &mut baseline,
            &format!("minimal: {kind}: {value}"),
            &xml,
            Some(expected),
            false,
        );
        result["schema"] = source.into();
        result["xml"] = xml.into();
        results.push(result);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_candidate_finalizes_deferred_references() {
        let source = r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:element name="root"><xs:complexType><xs:sequence>
                <xs:element name="item" minOccurs="0" maxOccurs="unbounded">
                    <xs:complexType>
                        <xs:attribute name="id" type="xs:ID"/>
                        <xs:attribute name="ref" type="xs:IDREF"/>
                    </xs:complexType>
                </xs:element>
            </xs:sequence></xs:complexType></xs:element>
        </xs:schema>"#;
        let schema = SchemaSetBuilder::new()
            .add_source(source, "deferred-idrefs.xsd")
            .unwrap()
            .compile()
            .unwrap()
            .into_schema_set();
        let (accepted, errors) = candidate(&schema, r#"<root><item ref="missing"/></root>"#);
        assert!(!accepted);
        assert!(
            errors.iter().any(|error| error.contains("cvc-id.1")),
            "{errors:?}"
        );
        // Forward references resolve only once the whole document is known.
        let (accepted, errors) = candidate(
            &schema,
            r#"<root><item ref="later"/><item id="later"/></root>"#,
        );
        assert!(accepted, "{errors:?}");
        assert!(errors.is_empty());
        // Empty and truncated documents cannot be accepted as valid roots.
        assert!(!candidate(&schema, "").0);
        let (accepted, errors) = candidate(&schema, "<root>");
        assert!(!accepted);
        assert!(!errors.is_empty());
        assert_eq!(candidate(&schema, "<root/>"), (true, Vec::new()));
    }
}
