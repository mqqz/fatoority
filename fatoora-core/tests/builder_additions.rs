//! Plural builder methods and the `InvoiceUuid` / `InvoiceHash` newtypes.
use fatoora_core::Decimal;
use fatoora_core::invoice::{
    Address, BuyerRole, CountryCode, InvoiceBuilder, InvoiceError, InvoiceField, InvoiceHash,
    InvoiceNote, InvoiceSubType, InvoiceType, InvoiceUuid, LineItem, OriginalInvoiceRef, OtherId,
    Party, SellerRole, ValidationKind, VatCategory,
};

fn address() -> Address {
    Address::new(
        CountryCode::parse("SAU").expect("country code"),
        "Riyadh",
        "King Fahd",
        "1234",
        "12222",
    )
    .with_additional_number("5678")
    .with_district("Olaya")
}

fn base() -> InvoiceBuilder {
    let seller = Party::<SellerRole>::new(
        "Acme Inc",
        address(),
        "399999999900003".parse().unwrap(),
        Some(OtherId::with_scheme("7003339333", "CRN")),
    );
    let buyer = Party::<BuyerRole>::new(
        "Blue Desert Retail LLC",
        address(),
        Some("310122393500003".parse().unwrap()),
        None,
    );
    InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Standard))
        .id("INV-1")
        .uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d".parse().unwrap())
        .issue_datetime("2024-01-01T12:30:00Z".parse().unwrap())
        .currency("SAR".parse().unwrap())
        .previous_invoice_hash("hash".parse().unwrap())
        .invoice_counter(1)
        .seller(seller)
        .buyer(buyer)
        .payment_means(fatoora_core::invoice::PaymentMeans::new("10"))
        .vat_category(VatCategory::Standard)
}

fn line(description: &str, price: &str) -> LineItem {
    LineItem::new(
        description,
        Decimal::parse("1").unwrap(),
        "PCE",
        Decimal::parse(price).unwrap(),
        Decimal::parse("15").unwrap(),
        VatCategory::Standard,
    )
    .unwrap()
}

fn lines() -> Vec<LineItem> {
    vec![line("A", "100"), line("B", "50"), line("C", "25")]
}

fn notes() -> Vec<InvoiceNote> {
    vec![InvoiceNote::new("en", "one"), InvoiceNote::new("ar", "two")]
}

fn references() -> Vec<OriginalInvoiceRef> {
    vec![
        OriginalInvoiceRef::new("ORIG-1").with_uuid("u1".parse().unwrap()),
        OriginalInvoiceRef::new("ORIG-2"),
    ]
}

fn ids() -> Vec<OtherId> {
    vec![
        OtherId::with_scheme("1010010000", "CRN"),
        OtherId::with_scheme("12345", "MOM"),
    ]
}

#[test]
fn line_items_equals_repeated_line_item() {
    let singular = lines()
        .into_iter()
        .fold(base(), InvoiceBuilder::line_item)
        .build()
        .unwrap();
    let plural = base().line_items(lines()).build().unwrap();
    assert_eq!(plural, singular);
    assert_eq!(plural.data().line_items().len(), 3);
}

#[test]
fn line_items_appends_to_existing_items() {
    let mixed = base()
        .line_item(line("A", "100"))
        .line_items([line("B", "50"), line("C", "25")])
        .build()
        .unwrap();
    let plural = base().line_items(lines()).build().unwrap();
    assert_eq!(mixed, plural);
    assert_eq!(
        base().line_items([]).line_items(lines()).build().unwrap(),
        plural
    );
}

#[test]
fn notes_equals_repeated_note() {
    let singular = notes()
        .into_iter()
        .fold(base().line_items(lines()), InvoiceBuilder::note)
        .build()
        .unwrap();
    let plural = base().line_items(lines()).notes(notes()).build().unwrap();
    assert_eq!(plural, singular);
    assert_eq!(plural.data().notes(), notes().as_slice());
    let appended = base()
        .line_items(lines())
        .note(notes()[0].clone())
        .notes([notes()[1].clone()])
        .build()
        .unwrap();
    assert_eq!(appended, plural);
}

#[test]
fn billing_references_equals_repeated_billing_reference() {
    let singular = references()
        .into_iter()
        .fold(
            base().line_items(lines()),
            InvoiceBuilder::billing_reference,
        )
        .build()
        .unwrap();
    let plural = base()
        .line_items(lines())
        .billing_references(references())
        .build()
        .unwrap();
    assert_eq!(plural, singular);
    assert_eq!(plural.data().billing_references(), references().as_slice());
}

#[test]
fn seller_identifiers_equals_repeated_seller_identifier() {
    let singular = ids()
        .into_iter()
        .fold(
            base().line_items(lines()),
            InvoiceBuilder::seller_identifier,
        )
        .build()
        .unwrap();
    let plural = base()
        .line_items(lines())
        .seller_identifiers(ids())
        .build()
        .unwrap();
    assert_eq!(plural, singular);
    assert!(
        plural
            .data()
            .seller()
            .identifiers()
            .ends_with(ids().as_slice())
    );
}

#[test]
fn buyer_identifiers_equals_repeated_buyer_identifier() {
    let singular = ids()
        .into_iter()
        .fold(base().line_items(lines()), InvoiceBuilder::buyer_identifier)
        .build()
        .unwrap();
    let plural = base()
        .line_items(lines())
        .buyer_identifiers(ids())
        .build()
        .unwrap();
    assert_eq!(plural, singular);
    assert!(
        plural
            .data()
            .buyer()
            .unwrap()
            .identifiers()
            .ends_with(ids().as_slice())
    );
}

fn assert_empty_issue(err: InvoiceError, field: InvoiceField) {
    let InvoiceError::Validation(validation) = err else {
        panic!("expected validation error");
    };
    let issues = validation.issues();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].field(), field);
    assert_eq!(issues[0].kind(), ValidationKind::Empty);
}

#[test]
fn invoice_uuid_parses_like_builder() {
    let uuid = InvoiceUuid::parse("8e6000cf-1a98-4174-b3e7-b5d5954bc10d").unwrap();
    assert_eq!(uuid.as_str(), "8e6000cf-1a98-4174-b3e7-b5d5954bc10d");
    assert_eq!(uuid.to_string(), uuid.as_str());
    assert_eq!(AsRef::<str>::as_ref(&uuid), uuid.as_str());
    // The builder accepts any non-blank UUID string, so the newtype does too.
    assert_eq!(InvoiceUuid::parse("uuid-1").unwrap().as_str(), "uuid-1");
    assert_eq!(InvoiceUuid::parse("  uuid-1 ").unwrap().as_str(), "uuid-1");
    assert_eq!("uuid-1".parse::<InvoiceUuid>().unwrap().as_str(), "uuid-1");
    assert_eq!(InvoiceUuid::try_from("uuid-1").unwrap().as_str(), "uuid-1");
    assert_eq!(
        InvoiceUuid::try_from(String::from("uuid-1"))
            .unwrap()
            .as_str(),
        "uuid-1"
    );
    for blank in ["", "   ", "\t\n"] {
        assert_empty_issue(InvoiceUuid::parse(blank).unwrap_err(), InvoiceField::Uuid);
    }
}

#[test]
fn invoice_hash_parses_like_builder() {
    let value =
        "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==";
    let hash = InvoiceHash::parse(value).unwrap();
    assert_eq!(hash.as_str(), value);
    assert_eq!(hash.to_string(), hash.as_str());
    assert_eq!(AsRef::<str>::as_ref(&hash), value);
    assert_eq!(InvoiceHash::parse("hash").unwrap().as_str(), "hash");
    assert_eq!(InvoiceHash::parse(" hash ").unwrap().as_str(), "hash");
    assert_eq!("hash".parse::<InvoiceHash>().unwrap().as_str(), "hash");
    assert_eq!(InvoiceHash::try_from("hash").unwrap().as_str(), "hash");
    assert_eq!(
        InvoiceHash::try_from(String::from("hash"))
            .unwrap()
            .as_str(),
        "hash"
    );
    for blank in ["", "   ", "\t\n"] {
        assert_empty_issue(
            InvoiceHash::parse(blank).unwrap_err(),
            InvoiceField::PreviousInvoiceHash,
        );
    }
}

#[test]
fn newtypes_serialize_and_validate_on_deserialize() {
    let uuid = InvoiceUuid::parse("uuid-1").unwrap();
    let json = serde_json::to_string(&uuid).unwrap();
    assert_eq!(json, "\"uuid-1\"");
    assert_eq!(serde_json::from_str::<InvoiceUuid>(&json).unwrap(), uuid);
    assert!(serde_json::from_str::<InvoiceUuid>("\"  \"").is_err());

    let hash = InvoiceHash::parse("hash").unwrap();
    let json = serde_json::to_string(&hash).unwrap();
    assert_eq!(serde_json::from_str::<InvoiceHash>(&json).unwrap(), hash);
    assert!(serde_json::from_str::<InvoiceHash>("\"\"").is_err());
}

#[cfg(feature = "uuid")]
#[test]
fn uuid_conversion_round_trips() {
    let source = uuid::Uuid::from_u128(0x8e6000cf_1a98_4174_b3e7_b5d5954bc10d);
    let converted = InvoiceUuid::from(source);
    assert_eq!(converted.as_str(), "8e6000cf-1a98-4174-b3e7-b5d5954bc10d");
    assert_eq!(InvoiceUuid::parse(converted.as_str()).unwrap(), converted);
    assert_eq!(
        uuid::Uuid::parse_str(converted.as_str()).unwrap(),
        source,
        "converted string parses back to the same UUID"
    );
    let nil = InvoiceUuid::from(uuid::Uuid::nil());
    assert_eq!(InvoiceUuid::parse(nil.as_str()).unwrap(), nil);
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_timestamp_conversion_round_trips() {
    use chrono::{NaiveDate, TimeZone, Utc};
    use fatoora_core::invoice::InvoiceTimestamp;
    let source = Utc.with_ymd_and_hms(2024, 1, 1, 12, 30, 5).unwrap();
    let converted = InvoiceTimestamp::from(source);
    assert_eq!(converted.as_str(), "2024-01-01T12:30:05Z");
    assert_eq!(
        InvoiceTimestamp::parse(converted.as_str()).unwrap(),
        converted
    );

    // Sub-second precision is truncated, matching the accepted format.
    let fractional = source + chrono::Duration::milliseconds(999);
    assert_eq!(InvoiceTimestamp::from(fractional), converted);

    for (y, m, d) in [
        (1, 1, 1),
        (9999, 12, 31),
        (2024, 2, 29),
        (10000, 1, 1),
        (-1, 1, 1),
    ] {
        let source = NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc();
        let converted = InvoiceTimestamp::from(source);
        assert_eq!(
            InvoiceTimestamp::parse(converted.as_str()).unwrap(),
            converted
        );
    }
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_date_conversion_round_trips() {
    use chrono::NaiveDate;
    use fatoora_core::invoice::InvoiceDate;
    for (y, m, d) in [
        (2024, 1, 15),
        (1, 1, 1),
        (9999, 12, 31),
        (2024, 2, 29),
        (10000, 1, 1),
        (-1, 1, 1),
    ] {
        let source = NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let converted = InvoiceDate::from(source);
        assert_eq!(InvoiceDate::parse(converted.as_str()).unwrap(), converted);
    }
    assert_eq!(
        InvoiceDate::from(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()).as_str(),
        "2024-01-15"
    );
}
