use fatoora_core::config::Environment;
use fatoora_core::csr::{CsrProperties, SigningKey};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let properties = CsrProperties::from_file("csr.properties")?;
    let key = SigningKey::generate();
    let csr = properties.build(&key, Environment::NonProduction)?;
    std::fs::write("csr.pem", csr.to_pem()?)?;
    std::fs::write("key.pem", key.to_pem()?)?;
    println!("Saved csr.pem and key.pem");
    Ok(())
}
