use fatoora_core::invoice::validation::{Validator, ZatcaValidationOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = include_str!("../tests/fixtures/sdk-parity/cases/standard-invoice/input.xml");
    // Use the actual predecessor digest for every invoice after the first.
    let options = ZatcaValidationOptions {
        previous_invoice_hash: Some("NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==".parse()?),
        evaluated_at: Some("2026-09-23T12:00:00+03:00".parse()?),
    };
    let validator = Validator::new();
    let report = validator.validate_zatca(xml, &options)?;
    assert!(report.is_valid());
    println!("{}", serde_json::to_string_pretty(&report)?);
    let rejected = validator.validate_zatca("<wrong/>", &options)?;
    assert!(rejected.has_errors());
    assert!(!rejected.is_valid());
    Ok(())
}
