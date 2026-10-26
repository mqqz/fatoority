use fatoora_core::invoice::{
    Address, CountryCode, InvoiceBuilder, InvoiceSubType, InvoiceType, LineItem, OtherId, Party,
    SellerRole, VatCategory,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seller = Party::<SellerRole>::new(
        "Example Store".into(),
        Address {
            country_code: CountryCode::parse("SA")?,
            city: "Riyadh".into(),
            street: "King Fahd Road".into(),
            building_number: "1234".into(),
            postal_code: "12222".into(),
            district: Some("Olaya".into()),
            additional_number: Some("5678".into()),
            additional_street: None,
            province: None,
        },
        "399999999900003",
        Some(OtherId::with_scheme("7003339333", "CRN")),
    )?;
    let item = LineItem::new(
        "Notebook",
        "2".parse()?,
        "PCE",
        "50".parse()?,
        "15".parse()?,
        VatCategory::Standard,
    )?;
    let invoice = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("INV-1001")
        .uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
        .issue_datetime("2026-10-26T09:00:00Z")
        .currency("SAR")
        // Demonstration predecessor; use your own invoice history in production.
        .previous_invoice_hash("NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==")
        .invoice_counter(1)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .line_item(item)
        .build()?;

    std::fs::write("invoice.xml", invoice.to_xml()?)?;
    println!("VAT: {} SAR", invoice.totals().tax_amount());
    println!("Payable: {} SAR", invoice.totals().payable_amount());
    println!("Saved invoice.xml");
    Ok(())
}
