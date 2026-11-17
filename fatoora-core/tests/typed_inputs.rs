//! Typed builder inputs, typed `InvoiceData` / signed getters and typed party constructors.
mod common;

use fatoora_core::invoice::{
    Address, Buyer, CountryCode, CurrencyCode, InvoiceBuilder, InvoiceDate, InvoiceError,
    InvoiceField, InvoiceHash, InvoiceSubType, InvoiceTime, InvoiceTimestamp, InvoiceType,
    InvoiceUuid, LineItem, OriginalInvoiceRef, OtherId, Seller, ValidationKind, VatCategory, VatId,
};

const UUID: &str = "8e6000cf-1a98-4174-b3e7-b5d5954bc10d";
const HASH: &str =
    "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==";

fn address() -> Address {
    Address::new(
        CountryCode::parse("SAU").unwrap(),
        "Riyadh",
        "King Fahd",
        "1234",
        "12222",
    )
    .with_additional_number("5678")
    .with_district("Olaya")
}

fn seller() -> Seller {
    Seller::new(
        String::from("Acme Inc"),
        address(),
        VatId::parse("399999999900003").unwrap(),
        Some(OtherId::with_scheme("7003339333", "CRN")),
    )
}

fn line() -> LineItem {
    LineItem::new(
        "Item",
        "1".parse().unwrap(),
        "PCE",
        "100".parse().unwrap(),
        "15".parse().unwrap(),
        VatCategory::Standard,
    )
    .unwrap()
}

fn typed_builder() -> InvoiceBuilder {
    InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("INV-1")
        .uuid(InvoiceUuid::parse(UUID).unwrap())
        .issue_datetime(InvoiceTimestamp::parse("2024-01-01T12:30:00Z").unwrap())
        .currency(CurrencyCode::parse("sar").unwrap())
        .previous_invoice_hash(InvoiceHash::parse(HASH).unwrap())
        .invoice_counter(1)
        .seller(seller())
        .payment_means(fatoora_core::invoice::PaymentMeans::new("10"))
        .vat_category(VatCategory::Standard)
        .line_item(line())
}

#[test]
fn typed_builder_inputs_flow_to_typed_getters() {
    let invoice = typed_builder().build().unwrap();
    let data = invoice.data();
    assert_eq!(data.uuid(), &InvoiceUuid::parse(UUID).unwrap());
    assert_eq!(
        data.issue_datetime(),
        &InvoiceTimestamp::parse("2024-01-01T12:30:00Z").unwrap()
    );
    assert_eq!(data.currency().as_str(), "SAR");
    assert_eq!(
        data.previous_invoice_hash(),
        &InvoiceHash::parse(HASH).unwrap()
    );
    let date: InvoiceDate = data.issue_date();
    let time: InvoiceTime = data.issue_time();
    assert_eq!(date, InvoiceDate::parse("2024-01-01").unwrap());
    assert_eq!(time, InvoiceTime::parse("12:30:00Z").unwrap());

    let xml = invoice.to_xml().unwrap();
    assert!(xml.contains(&format!("<cbc:UUID>{UUID}</cbc:UUID>")));
    assert!(xml.contains("<cbc:IssueDate>2024-01-01</cbc:IssueDate>"));
    assert!(xml.contains("<cbc:IssueTime>12:30:00Z</cbc:IssueTime>"));
    assert!(xml.contains(HASH));
}

#[test]
fn imported_ast_issue_time_keeps_its_representation() {
    let xml = common::dummy_finalized_invoice()
        .to_xml()
        .unwrap()
        .replace("12:30:00Z", "15:30:00");
    let parsed = fatoora_core::invoice::xml::parse::parse_finalized_invoice_xml(&xml).unwrap();
    assert_eq!(parsed.data().issue_time().as_str(), "15:30:00");
    assert_eq!(parsed.data().issue_date().as_str(), "2024-01-01");
    assert_eq!(
        parsed.data().issue_datetime().as_str(),
        "2024-01-01T12:30:00Z"
    );
}

#[test]
fn builder_reports_unset_typed_fields_as_missing() {
    let err = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("INV-1")
        .invoice_counter(1)
        .seller(seller())
        .vat_category(VatCategory::Standard)
        .line_item(line())
        .build()
        .unwrap_err();
    let InvoiceError::Validation(validation) = err else {
        panic!("expected validation error");
    };
    for field in [
        InvoiceField::Uuid,
        InvoiceField::IssueDateTime,
        InvoiceField::Currency,
        InvoiceField::PreviousInvoiceHash,
    ] {
        assert!(
            validation
                .issues()
                .iter()
                .any(|issue| issue.field() == field && issue.kind() == ValidationKind::Missing),
            "{field:?}"
        );
    }
}

#[test]
fn invoice_time_parses_through_every_conversion() {
    assert_eq!(
        InvoiceTime::parse(" 09:05:00 ").unwrap().as_str(),
        "09:05:00"
    );
    assert_eq!(
        "09:05:00Z".parse::<InvoiceTime>().unwrap().as_str(),
        "09:05:00Z"
    );
    assert_eq!(
        InvoiceTime::try_from("24:00:00").unwrap().as_str(),
        "24:00:00"
    );
    let owned = InvoiceTime::try_from(String::from("23:59:59")).unwrap();
    assert_eq!(AsRef::<str>::as_ref(&owned), "23:59:59");
    assert_eq!(owned.to_string(), "23:59:59");
    for invalid in ["", "9:05:00", "25:00:00", "12:30", "12:30:00+03:00", "noon"] {
        assert!(
            matches!(
                InvoiceTime::try_from(invalid),
                Err(InvoiceError::InvalidTimestamp(_))
            ),
            "{invalid:?}"
        );
    }
}

#[test]
fn typed_newtypes_reject_invalid_input() {
    assert!(matches!(
        InvoiceTimestamp::try_from("2024-01-01T12:30:00"),
        Err(InvoiceError::InvalidTimestamp(_))
    ));
    assert!(matches!(
        "2024-13-01T00:00:00Z".parse::<InvoiceTimestamp>(),
        Err(InvoiceError::InvalidTimestamp(_))
    ));
    assert!(matches!(
        CurrencyCode::try_from("RIYAL"),
        Err(InvoiceError::InvalidCurrencyCode(_))
    ));
    assert!(matches!(
        InvoiceDate::try_from("2024-02-30"),
        Err(InvoiceError::InvalidIssueDate(_))
    ));
    assert!(matches!(
        VatId::try_from(" "),
        Err(InvoiceError::InvalidVatFormat)
    ));
    assert!(InvoiceUuid::try_from("").is_err());
    assert!(InvoiceHash::try_from(" ").is_err());
}

#[test]
fn original_invoice_ref_takes_typed_uuid_and_date() {
    let reference = OriginalInvoiceRef::new("INV-ORIG")
        .with_uuid(InvoiceUuid::parse(UUID).unwrap())
        .with_issue_date(InvoiceDate::try_from("2024-01-15").unwrap());
    assert_eq!(reference.uuid().map(InvoiceUuid::as_str), Some(UUID));
    assert_eq!(
        reference.issue_date().map(InvoiceDate::as_str),
        Some("2024-01-15")
    );
    assert!(OriginalInvoiceRef::new("INV-ORIG").uuid().is_none());
}

#[test]
fn party_constructors_take_typed_vat_ids() {
    let seller = seller();
    assert_eq!(seller.name(), "Acme Inc");
    assert_eq!(seller.vat_id().map(VatId::as_str), Some("399999999900003"));

    let buyer = Buyer::new(
        "Blue Desert",
        None,
        Some(VatId::parse("310122393500003").unwrap()),
        None,
    );
    assert_eq!(buyer.name(), "Blue Desert");
    assert!(buyer.address().is_none());
    assert_eq!(buyer.vat_id().map(VatId::as_str), Some("310122393500003"));

    let anonymous = Buyer::new(
        String::from("Walk-in"),
        address(),
        None,
        Some(OtherId::with_scheme("1234567890", "NAT")),
    );
    assert!(anonymous.vat_id().is_none());
    assert_eq!(anonymous.identifiers().len(), 1);
}

#[test]
fn signed_getters_return_typed_values() {
    let signed = common::dummy_finalized_invoice()
        .sign(&common::fixture_signer())
        .unwrap();
    assert_eq!(signed.uuid(), &InvoiceUuid::parse(UUID).unwrap());
    let hash: &InvoiceHash = signed.invoice_hash();
    assert_eq!(hash.as_str(), signed.hash_base64().unwrap());
    assert_eq!(signed.signed_properties().invoice_hash(), hash);

    let signing_time: &InvoiceTimestamp = signed.signed_properties().signing_time();
    assert_eq!(signing_time.as_str(), "2024-01-01T12:30:00Z");
    // The XAdES element keeps its zone-less form.
    assert!(
        signed
            .xml()
            .contains("<xades:SigningTime>2024-01-01T12:30:00</xades:SigningTime>")
    );

    let parsed = fatoora_core::invoice::xml::parse::parse_signed_invoice_xml(signed.xml()).unwrap();
    assert_eq!(parsed.signed_properties().signing_time(), signing_time);
    assert_eq!(parsed.invoice_hash(), hash);
}
