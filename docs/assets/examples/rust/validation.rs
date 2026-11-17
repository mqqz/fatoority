use fatoora_core::invoice::validation::{Validator, ZatcaValidationOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = Validator::new();
    let xml = std::fs::read_to_string("invoice.xml")?;
    if std::env::args().any(|arg| arg == "--zatca") {
        let options = ZatcaValidationOptions {
            previous_invoice_hash: Some(std::fs::read_to_string("previous-hash.txt")?.parse()?),
            ..Default::default()
        };
        let report = validator.validate_zatca(&xml, &options)?;
        println!("{}", serde_json::to_string_pretty(&report)?);
        if !report.is_valid() { std::process::exit(1); }
    } else {
        validator.validate_xml(&xml)?;
        println!("XSD valid");
    }
    Ok(())
}
