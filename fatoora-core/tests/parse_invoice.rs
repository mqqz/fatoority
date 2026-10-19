use fatoora_core::invoice::xml::parse::{
    ParseError, parse_finalized_invoice_xml, parse_finalized_invoice_xml_file,
    parse_signed_invoice_xml, parse_signed_invoice_xml_file,
};
use fatoora_core::invoice::{
    Address, CountryCode, InvoiceBuilder, InvoiceSubType, InvoiceType, LineItem,
    OriginalInvoiceRef, Party, SellerRole, VatCategory,
};
use std::path::Path;

#[test]
fn parse_sample_simplified_invoice() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/sample-simplified-invoice.xml");
    let xml = std::fs::read_to_string(&path).expect("read xml");
    let invoice = parse_finalized_invoice_xml(&xml).expect("parse invoice");
    let data = invoice.data();

    assert_eq!(data.id(), "SME00010");
    assert_eq!(data.uuid(), "8e6000cf-1a98-4174-b3e7-b5d5954bc10d");
    assert_eq!(data.issue_datetime().as_str(), "2022-08-17T14:41:08Z");
    assert_eq!(data.currency().as_str(), "SAR");
    assert_eq!(
        data.previous_invoice_hash(),
        "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ=="
    );
    assert_eq!(data.invoice_counter(), 10);

    assert_eq!(
        data.seller().name(),
        "شركة توريد التكنولوجيا بأقصى سرعة المحدودة | Maximum Speed Tech Supply LTD"
    );
    assert_eq!(data.seller().vat_id().unwrap().as_str(), "399999999900003");
    assert_eq!(data.seller().other_id().unwrap().as_str(), "1010010000");
    assert_eq!(data.seller().other_id().unwrap().scheme_id(), Some("CRN"));

    let address = data.seller().address().unwrap();
    assert!(address.street().contains("Prince Sultan"));
    assert_eq!(address.building_number(), "2322");
    assert!(address.city().contains("Riyadh"));
    assert_eq!(address.postal_code(), "23333");
    assert_eq!(address.country_code().as_str(), "SAU");

    let totals = invoice.totals();
    assert_eq!(
        totals.line_extension(),
        fatoora_core::Decimal::parse("201.0").unwrap()
    );
    assert_eq!(
        totals.tax_amount(),
        fatoora_core::Decimal::parse("30.15").unwrap()
    );
    assert_eq!(
        totals.tax_inclusive_amount(),
        fatoora_core::Decimal::parse("231.15").unwrap()
    );

    assert_eq!(data.line_items().len(), 2);
    assert_eq!(data.line_items()[0].description(), "كتاب");
    assert_eq!(
        data.line_items()[0].quantity(),
        fatoora_core::Decimal::parse("33.0").unwrap()
    );
    assert_eq!(data.line_items()[0].unit_code(), "PCE");
    assert_eq!(
        data.line_items()[0].total_amount(),
        fatoora_core::Decimal::parse("99.0").unwrap()
    );
    assert_eq!(
        data.line_items()[0].unit_price(),
        fatoora_core::Decimal::parse("3.0").unwrap()
    );
    assert_eq!(
        data.line_items()[0].vat_rate(),
        fatoora_core::Decimal::parse("15.0").unwrap()
    );
    assert_eq!(
        data.line_items()[0].vat_amount(),
        fatoora_core::Decimal::parse("14.85").unwrap()
    );
    assert!(matches!(
        data.line_items()[0].vat_category(),
        fatoora_core::invoice::VatCategory::Standard
    ));
    assert_eq!(data.line_items()[1].description(), "قلم");
    assert_eq!(
        data.line_items()[1].quantity(),
        fatoora_core::Decimal::parse("3.0").unwrap()
    );
    assert_eq!(data.line_items()[1].unit_code(), "PCE");
    assert_eq!(
        data.line_items()[1].total_amount(),
        fatoora_core::Decimal::parse("102.0").unwrap()
    );
    assert_eq!(
        data.line_items()[1].unit_price(),
        fatoora_core::Decimal::parse("34.0").unwrap()
    );
    assert_eq!(
        data.line_items()[1].vat_rate(),
        fatoora_core::Decimal::parse("15.0").unwrap()
    );
    assert_eq!(
        data.line_items()[1].vat_amount(),
        fatoora_core::Decimal::parse("15.30").unwrap()
    );
    assert!(matches!(
        data.line_items()[1].vat_category(),
        fatoora_core::invoice::VatCategory::Standard
    ));
}

#[test]
fn parse_sample_simplified_invoice_from_file() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/sample-simplified-invoice.xml");
    let invoice = parse_finalized_invoice_xml_file(&path).expect("parse invoice");
    assert_eq!(invoice.data().id(), "SME00010");
}

#[test]
fn parse_signed_invoice_from_fixture() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/sample-simplified-invoice.xml");
    let xml = std::fs::read_to_string(&path).expect("read xml");
    let signed = parse_signed_invoice_xml(&xml).expect("parse signed invoice");
    let data = signed.data();

    assert_eq!(data.id(), "SME00010");
    assert!(signed.xml().contains("<ds:Signature"));
    assert!(!signed.qr_code().trim().is_empty());
    assert_eq!(
        signed.invoice_hash(),
        "z5F9qsS6oWyDhehD8u8S0DaxV+2CUiUz9Y+UsR61JgQ="
    );
    assert!(signed.zatca_key_signature().is_some());

    let props = signed.signed_properties();
    assert_eq!(props.signing_time(), "2025-07-22T15:51:28");
    assert_eq!(
        props.cert_hash(),
        "ZDMwMmI0MTE1NzVjOTU2NTk4YzVlODhhYmI0ODU2NDUyNTU2YTVhYjhhMDFmN2FjYjk1YTA2OWQ0NjY2MjQ4NQ=="
    );
    assert_eq!(
        props.issuer(),
        "CN=PRZEINVOICESCA4-CA, DC=extgazt, DC=gov, DC=local"
    );
    assert_eq!(
        props.serial(),
        "379112742831380471835263969587287663520528387"
    );
    assert_eq!(
        props.signed_props_hash(),
        "ZmMwY2ZhNDljNzNjZDA5NmY4NDM4MmY1ZmY1YTA0NjY3MzY4NzMxOGJhYmZmNWU1OGYzZWJlODI3ZDgyZGVkZA=="
    );

    let serialized = signed.xml().to_owned();
    assert_eq!(serialized.trim(), xml.trim());
}

#[test]
fn parsing_rejects_modified_signature_material() {
    let xml = include_str!("fixtures/invoices/sample-simplified-invoice.xml");
    let original = parse_signed_invoice_xml(xml).unwrap();
    let signature = original.signature();
    assert!(!signature.is_empty());
    // Keep the encoding and length, but change the cryptographic signature.
    let first = if signature.starts_with('A') { "B" } else { "A" };
    let changed_signature = format!("{first}{}", &signature[1..]);
    let changed_xml = xml.replacen(signature, &changed_signature, 1);
    assert_ne!(changed_xml, xml);
    assert!(parse_signed_invoice_xml(&changed_xml).is_err());
}

#[test]
fn parse_signed_invoice_from_file() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/sample-simplified-invoice.xml");
    let signed = parse_signed_invoice_xml_file(&path).expect("parse signed invoice");
    assert_eq!(signed.data().id(), "SME00010");
}

#[test]
fn parse_rejects_missing_invoice_id() {
    let xml = load_sample_xml();
    let xml = xml.replace("<cbc:ID>SME00010</cbc:ID>", "");
    let err = parse_finalized_invoice_xml(&xml).expect_err("missing ID");
    assert!(matches!(err, ParseError::MissingField("ID")));
}

#[test]
fn parse_rejects_invalid_currency_code() {
    let xml = load_sample_xml();
    let xml = xml.replace(
        "<cbc:DocumentCurrencyCode>SAR</cbc:DocumentCurrencyCode>",
        "<cbc:DocumentCurrencyCode>ZZZ</cbc:DocumentCurrencyCode>",
    );
    let err = parse_finalized_invoice_xml(&xml).expect_err("invalid currency");
    assert!(matches!(
        err,
        ParseError::InvalidValue {
            field: "DocumentCurrencyCode",
            ..
        }
    ));
}

#[test]
fn parse_rejects_unknown_invoice_type_code() {
    let xml = load_sample_xml();
    let xml = xml.replace(
        "<cbc:InvoiceTypeCode name=\"0200000\">388</cbc:InvoiceTypeCode>",
        "<cbc:InvoiceTypeCode name=\"0200000\">999</cbc:InvoiceTypeCode>",
    );
    let err = parse_finalized_invoice_xml(&xml).expect_err("invalid invoice type");
    assert!(matches!(
        err,
        ParseError::InvalidValue {
            field: "InvoiceTypeCode",
            ..
        }
    ));
}

#[test]
fn parse_signed_rejects_missing_qr() {
    let xml = load_sample_xml();
    let xml = xml.replace("<cbc:ID>QR</cbc:ID>", "<cbc:ID>NOT_QR</cbc:ID>");
    let err = parse_signed_invoice_xml(&xml).expect_err("missing QR");
    assert!(matches!(err, ParseError::MissingField("QR")));
}

#[test]
fn parse_signed_rejects_invalid_signing_time() {
    let xml = load_sample_xml();
    let xml = xml.replace(
        "<xades:SigningTime>2025-07-22T15:51:28</xades:SigningTime>",
        "<xades:SigningTime>bad-time</xades:SigningTime>",
    );
    let err = parse_signed_invoice_xml(&xml).expect_err("invalid signing time");
    assert!(matches!(
        err,
        ParseError::InvalidValue {
            field: "SigningTime",
            ..
        }
    ));
}

#[test]
fn credit_note_serializes_billing_reference_and_reason() {
    let seller = build_seller();

    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("1.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("100.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG")
        .with_uuid("uuid-orig")
        .with_issue_date_str("2023-12-31")
        .expect("issue date");

    let mut builder = InvoiceBuilder::new(InvoiceType::CreditNote(InvoiceSubType::Standard))
        .billing_reference(original)
        .payment_instruction("pricing correction");
    builder = builder
        .id("CR-1")
        .uuid("uuid-cr-1")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .line_item(line_item);
    let invoice = builder.build().expect("build credit note");

    let xml = invoice.to_xml().expect("serialize credit note");
    assert!(xml.contains("<cac:BillingReference>"));
    assert!(xml.contains("<cbc:ID>INV-ORIG</cbc:ID>"));
    assert!(xml.contains("<cbc:UUID>uuid-orig</cbc:UUID>"));
    assert!(xml.contains("<cbc:IssueDate>2023-12-31</cbc:IssueDate>"));
    assert!(xml.contains("<cbc:InstructionNote>pricing correction</cbc:InstructionNote>"));
}

#[test]
fn debit_note_serializes_billing_reference_and_reason() {
    let seller = build_seller();
    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("1.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("100.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG")
        .with_uuid("uuid-orig")
        .with_issue_date_str("2023-12-31")
        .expect("issue date");

    let mut builder = InvoiceBuilder::new(InvoiceType::DebitNote(InvoiceSubType::Simplified))
        .billing_reference(original)
        .payment_instruction("pricing adjustment");
    builder = builder
        .id("DB-1")
        .uuid("uuid-db-1")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .line_item(line_item);
    let invoice = builder.build().expect("build debit note");

    let xml = invoice.to_xml().expect("serialize debit note");
    assert!(xml.contains("<cbc:InvoiceTypeCode name=\"0200000\">383</cbc:InvoiceTypeCode>"));
    assert!(xml.contains("<cac:BillingReference>"));
    assert!(xml.contains("<cbc:ID>INV-ORIG</cbc:ID>"));
    assert!(xml.contains("<cbc:UUID>uuid-orig</cbc:UUID>"));
    assert!(xml.contains("<cbc:IssueDate>2023-12-31</cbc:IssueDate>"));
    assert!(xml.contains("<cbc:InstructionNote>pricing adjustment</cbc:InstructionNote>"));
}

#[test]
fn parse_credit_note_requires_billing_reference_id() {
    let xml = load_credit_note_xml();
    let xml = xml.replace("<cbc:ID>SME00002</cbc:ID>", "");
    let err = parse_finalized_invoice_xml(&xml).expect_err("missing billing ref id");
    assert!(matches!(
        err,
        ParseError::MissingField("BillingReferenceID")
    ));
}

#[test]
fn parse_credit_note_missing_billing_reference_element() {
    let xml = load_credit_note_xml();
    let xml = remove_billing_reference(&xml);
    let invoice =
        parse_finalized_invoice_xml(&xml).expect("conditional reference checks are separate");
    assert!(invoice.data().billing_references().is_empty());
}

#[test]
fn parse_credit_note_rejects_invalid_original_issue_date() {
    let xml = load_credit_note_xml();
    let xml = xml.replace(
        "<cbc:ID>SME00002</cbc:ID>",
        "<cbc:ID>SME00002</cbc:ID><cbc:IssueDate>bad-date</cbc:IssueDate>",
    );
    let err = parse_finalized_invoice_xml(&xml).expect_err("invalid issue date");
    assert!(matches!(
        err,
        ParseError::InvalidValue {
            field: "BillingReferenceIssueDate",
            ..
        }
    ));
}

#[test]
fn parse_credit_note_does_not_invent_reason_when_missing() {
    let xml = load_credit_note_xml();
    let xml = xml.replace(
        "<cbc:InstructionNote>In case of goods or services refund | عند ترجيع السلع أو الخدمات</cbc:InstructionNote>",
        "",
    );
    let invoice = parse_finalized_invoice_xml(&xml).expect("parse credit note");
    match invoice.data().invoice_type() {
        InvoiceType::CreditNote(_) => {
            let original = &invoice.data().billing_references()[0];
            let reason = invoice.data().payment_instruction().unwrap_or("");
            assert_eq!(original.id(), "SME00002");
            assert_eq!(reason, "");
        }
        _ => panic!("expected credit note"),
    }
}

#[test]
fn parse_credit_note_does_not_use_general_note_as_instruction() {
    for (replacement, expected) in [
        ("", ""),
        ("<cbc:InstructionNote> \t </cbc:InstructionNote>", " \t "),
    ] {
        let xml = load_credit_note_xml();
        let xml = xml.replace(
            "<cbc:InstructionNote>In case of goods or services refund | عند ترجيع السلع أو الخدمات</cbc:InstructionNote>",
            replacement,
        );
        let xml = xml.replace(
            "<cbc:TaxCurrencyCode>SAR</cbc:TaxCurrencyCode>",
            "<cbc:TaxCurrencyCode>SAR</cbc:TaxCurrencyCode><cbc:Note>fallback note</cbc:Note>",
        );
        let invoice = parse_finalized_invoice_xml(&xml).expect("parse credit note");
        match invoice.data().invoice_type() {
            InvoiceType::CreditNote(_) => {
                let reason = invoice.data().payment_instruction().unwrap_or("");
                assert_eq!(reason, expected);
            }
            _ => panic!("expected credit note"),
        }
    }
}

#[test]
fn parse_credit_note_prefers_instruction_note_over_note() {
    let xml = load_credit_note_xml();
    let xml = xml.replace(
        "<cbc:TaxCurrencyCode>SAR</cbc:TaxCurrencyCode>",
        "<cbc:TaxCurrencyCode>SAR</cbc:TaxCurrencyCode><cbc:Note>fallback note</cbc:Note>",
    );
    let invoice = parse_finalized_invoice_xml(&xml).expect("parse credit note");
    match invoice.data().invoice_type() {
        InvoiceType::CreditNote(_) => {
            let reason = invoice.data().payment_instruction().unwrap_or("");
            assert!(reason.contains("In case of goods or services refund"));
        }
        _ => panic!("expected credit note"),
    }
}

#[test]
fn parse_debit_note_type_and_reason() {
    let xml = load_debit_note_xml();
    let invoice = parse_finalized_invoice_xml(&xml).expect("parse debit note");
    match invoice.data().invoice_type() {
        InvoiceType::DebitNote(subtype) => {
            let original = &invoice.data().billing_references()[0];
            let reason = invoice.data().payment_instruction().unwrap_or("");
            assert_eq!(*subtype, InvoiceSubType::Standard);
            assert_eq!(original.id(), "SME00002");
            assert!(reason.contains("Amendment"));
        }
        _ => panic!("expected debit note"),
    }
}

#[test]
fn credit_note_serialization_preserves_whitespace_reason() {
    let seller = build_seller();
    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("1.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("100.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG");
    let mut builder = InvoiceBuilder::new(InvoiceType::CreditNote(InvoiceSubType::Standard))
        .billing_reference(original)
        .payment_instruction("   ");
    builder = builder
        .id("CR-NO-REASON")
        .uuid("uuid-cr-no-reason")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .line_item(line_item);
    let invoice = builder.build().expect("build credit note");

    let xml = invoice.to_xml().expect("serialize credit note");
    assert!(xml.contains("<cbc:InstructionNote>   </cbc:InstructionNote>"));
}

#[test]
fn credit_note_allowance_discount_round_trip() {
    let seller = build_seller();
    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("2.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("50.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG");
    let mut builder = InvoiceBuilder::new(InvoiceType::CreditNote(InvoiceSubType::Standard))
        .billing_reference(original)
        .payment_instruction("pricing correction");
    builder = builder
        .id("CR-DISC-1")
        .uuid("uuid-cr-disc-1")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .invoice_level_discount(fatoora_core::Decimal::parse("10.0").unwrap())
        .allowance_reason("Discount")
        .line_item(line_item);
    let invoice = builder.build().expect("build credit note");

    let totals = invoice.totals();
    assert_eq!(
        totals.allowance_total(),
        fatoora_core::Decimal::parse("10.0").unwrap()
    );
    assert_eq!(
        totals.charge_total(),
        fatoora_core::Decimal::parse("0.0").unwrap()
    );
    assert_eq!(
        totals.taxable_amount(),
        fatoora_core::Decimal::parse("90.0").unwrap()
    );
    assert_eq!(
        totals.tax_inclusive_amount(),
        fatoora_core::Decimal::parse("103.50").unwrap()
    );

    let xml = invoice.to_xml().expect("serialize credit note");
    assert!(xml.contains("<cbc:AllowanceChargeReason>Discount</cbc:AllowanceChargeReason>"));
    assert!(xml.contains("<cbc:Amount currencyID=\"SAR\">10.00</cbc:Amount>"));

    let parsed = parse_finalized_invoice_xml(&xml).expect("parse credit note");
    assert_eq!(parsed.data().allowance_reason(), Some("Discount"));
    assert_eq!(
        parsed.totals().allowance_total(),
        fatoora_core::Decimal::parse("10.0").unwrap()
    );
}

#[test]
fn debit_note_charge_serializes_allowance_charge() {
    let seller = build_seller();
    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("1.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("100.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG");
    let mut builder = InvoiceBuilder::new(InvoiceType::DebitNote(InvoiceSubType::Standard))
        .billing_reference(original)
        .payment_instruction("pricing adjustment");
    builder = builder
        .id("DB-CHG-1")
        .uuid("uuid-db-chg-1")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .invoice_level_charge(fatoora_core::Decimal::parse("7.5").unwrap())
        .allowance_reason("Charge")
        .line_item(line_item);
    let invoice = builder.build().expect("build debit note");

    let totals = invoice.totals();
    assert_eq!(
        totals.charge_total(),
        fatoora_core::Decimal::parse("7.5").unwrap()
    );
    assert_eq!(
        totals.taxable_amount(),
        fatoora_core::Decimal::parse("107.5").unwrap()
    );

    let xml = invoice.to_xml().expect("serialize debit note");
    assert!(xml.contains("<cbc:ChargeIndicator>true</cbc:ChargeIndicator>"));
    assert!(xml.contains("<cbc:AllowanceChargeReason>Charge</cbc:AllowanceChargeReason>"));
    assert!(xml.contains("<cbc:Amount currencyID=\"SAR\">7.50</cbc:Amount>"));
}

#[test]
fn debit_note_allowance_round_trip() {
    let seller = build_seller();
    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("2.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("50.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG");
    let mut builder = InvoiceBuilder::new(InvoiceType::DebitNote(InvoiceSubType::Standard))
        .billing_reference(original)
        .payment_instruction("pricing adjustment");
    builder = builder
        .id("DB-DISC-1")
        .uuid("uuid-db-disc-1")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .invoice_level_discount(fatoora_core::Decimal::parse("8.0").unwrap())
        .allowance_reason("Adjustment")
        .line_item(line_item);
    let invoice = builder.build().expect("build debit note");

    let totals = invoice.totals();
    assert_eq!(
        totals.allowance_total(),
        fatoora_core::Decimal::parse("8.0").unwrap()
    );
    assert_eq!(
        totals.taxable_amount(),
        fatoora_core::Decimal::parse("92.0").unwrap()
    );
    assert_eq!(
        totals.tax_inclusive_amount(),
        fatoora_core::Decimal::parse("105.80").unwrap()
    );

    let xml = invoice.to_xml().expect("serialize debit note");
    assert!(xml.contains("<cbc:AllowanceChargeReason>Adjustment</cbc:AllowanceChargeReason>"));
    assert!(xml.contains("<cbc:Amount currencyID=\"SAR\">8.00</cbc:Amount>"));

    let parsed = parse_finalized_invoice_xml(&xml).expect("parse debit note");
    assert_eq!(parsed.data().allowance_reason(), Some("Adjustment"));
    assert_eq!(
        parsed.totals().allowance_total(),
        fatoora_core::Decimal::parse("8.0").unwrap()
    );
}

#[test]
fn credit_note_round_trip_preserves_original_ref_and_reason() {
    let seller = build_seller();
    let line_item = LineItem::new(
        "Item",
        fatoora_core::Decimal::parse("1.0").unwrap(),
        "PCE",
        fatoora_core::Decimal::parse("100.0").unwrap(),
        fatoora_core::Decimal::parse("15.0").unwrap(),
        VatCategory::Standard,
    )
    .unwrap();

    let original = OriginalInvoiceRef::new("INV-ORIG")
        .with_uuid("uuid-orig")
        .with_issue_date_str("2023-12-31")
        .expect("issue date");

    let mut builder = InvoiceBuilder::new(InvoiceType::CreditNote(InvoiceSubType::Standard))
        .billing_reference(original)
        .payment_instruction("pricing correction");
    builder = builder
        .id("CR-RT-1")
        .uuid("uuid-cr-rt-1")
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash("hash")
        .invoice_counter(0)
        .seller(seller)
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
        .line_item(line_item);
    let invoice = builder.build().expect("build credit note");

    let xml = invoice.to_xml().expect("serialize credit note");
    let parsed = parse_finalized_invoice_xml(&xml).expect("parse credit note");
    match parsed.data().invoice_type() {
        InvoiceType::CreditNote(subtype) => {
            let original = &parsed.data().billing_references()[0];
            let reason = parsed.data().payment_instruction().unwrap_or("");
            assert_eq!(*subtype, InvoiceSubType::Standard);
            assert_eq!(original.id(), "INV-ORIG");
            assert_eq!(original.uuid(), Some("uuid-orig"));
            assert_eq!(original.issue_date().unwrap().as_str(), "2023-12-31");
            assert_eq!(reason, "pricing correction");
        }
        _ => panic!("expected credit note"),
    }
}

fn build_seller() -> Party<SellerRole> {
    Party::<SellerRole>::new(
        "Acme Inc".into(),
        Address {
            province: None,
            country_code: CountryCode::parse("SAU").expect("country code"),
            city: "Riyadh".into(),
            street: "King Fahd".into(),
            additional_street: None,
            building_number: "1234".into(),
            additional_number: Some("5678".into()),
            postal_code: "12222".into(),
            district: None,
        },
        "301121971500003",
        None,
    )
    .expect("valid seller")
}

fn load_sample_xml() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/sample-simplified-invoice.xml");
    std::fs::read_to_string(&path).expect("read xml")
}

fn load_credit_note_xml() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/Standard/Credit/Standard_Credit_Note.xml");
    std::fs::read_to_string(&path).expect("read credit note xml")
}

fn load_debit_note_xml() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invoices/Standard/Debit/Standard_Debit_Note.xml");
    std::fs::read_to_string(&path).expect("read debit note xml")
}

fn remove_billing_reference(xml: &str) -> String {
    let start = xml.find("<cac:BillingReference>");
    let end = xml.find("</cac:BillingReference>");
    match (start, end) {
        (Some(start), Some(end)) => {
            let end = end + "</cac:BillingReference>".len();
            let mut out = String::with_capacity(xml.len());
            out.push_str(&xml[..start]);
            out.push_str(&xml[end..]);
            out
        }
        _ => xml.to_string(),
    }
}

#[test]
fn signed_certificate_accepts_only_xml_whitespace_without_changing_xml() {
    for kind in ["standard", "simplified"] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "tests/fixtures/sdk-parity/cases/{kind}-invoice/sdk-signed.xml"
        ));
        let xml = std::fs::read_to_string(path).unwrap();
        let original = parse_signed_invoice_xml(&xml).unwrap();
        let doc = roxmltree::Document::parse(&xml).unwrap();
        let certificate = doc
            .descendants()
            .find(|node| {
                node.has_tag_name(("http://www.w3.org/2000/09/xmldsig#", "X509Certificate"))
            })
            .unwrap()
            .text()
            .unwrap();
        let wrapped = certificate
            .as_bytes()
            .chunks(64)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<_>>()
            .join(" \t\r\n");
        let changed = xml.replacen(certificate, &wrapped, 1);
        let parsed = parse_signed_invoice_xml(&changed).unwrap();
        assert_eq!(parsed.xml(), changed);
        assert_eq!(parsed.qr_code(), original.qr_code());
        assert_eq!(parsed.zatca_key_signature(), original.zatca_key_signature());
        for invalid in [
            format!("{}!", certificate),
            format!("{}\u{a0}", certificate),
        ] {
            assert!(parse_signed_invoice_xml(&xml.replacen(certificate, &invalid, 1)).is_err());
        }
    }
}

#[test]
fn signed_import_rejects_malformed_certificate_encodings() {
    use base64ct::{Base64, Encoding};
    use x509_cert::der::Decode;
    let xml = include_str!("fixtures/sdk-parity/cases/simplified-invoice/sdk-signed.xml");
    let doc = roxmltree::Document::parse(xml).unwrap();
    let certificate = doc
        .descendants()
        .find(|node| node.has_tag_name(("http://www.w3.org/2000/09/xmldsig#", "X509Certificate")))
        .unwrap()
        .text()
        .unwrap();
    let mut der = Base64::decode_vec(certificate).unwrap();
    let cert = x509_cert::Certificate::from_der(&der).unwrap();
    let signature_len = cert.signature().as_bytes().unwrap().len();
    let unused_bits = der.len() - signature_len - 1;
    assert_eq!(der[unused_bits], 0);
    der[unused_bits] = 1;
    *der.last_mut().unwrap() &= 0xfe;
    for invalid in [
        Base64::encode_string(b"invalid DER"),
        Base64::encode_string(&der),
        Base64::encode_string(&vec![0; 16 * 1024 + 1]),
        "A".repeat(32 * 1024 + 1),
    ] {
        let err = parse_signed_invoice_xml(&xml.replacen(certificate, &invalid, 1)).unwrap_err();
        assert!(
            matches!(
                err,
                ParseError::InvalidValue {
                    field: "X509Certificate",
                    ..
                }
            ),
            "{err:?}"
        );
    }
}
