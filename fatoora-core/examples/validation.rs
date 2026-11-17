// --8<-- [start:example]
pub fn main() {
    use fatoora_core::invoice::validation::Validator;

    // Doc example: validate XML against the bundled schema.
    let invoice_xml_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/sample-simplified-invoice.xml");
    let xml = std::fs::read_to_string(&invoice_xml_path).expect("read invoice xml");
    Validator::new()
        .validate_xml(&xml)
        .expect("validate invoice");
}
// --8<-- [end:example]
