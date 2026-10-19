//! Mutate one field of an accepted fixture so rejection cannot be caused by an
//! unrelated missing field. Parsing checks supplied amounts; it is not validation.
use base64ct::{Base64, Encoding};
use fatoora_core::invoice::xml::parse::{parse_finalized_invoice_xml, parse_signed_invoice_xml};
use fatoora_core::{Error, ErrorKind};
#[path = "support/xml.rs"]
#[allow(dead_code)]
mod test_xml;
use serde_json::Value;

const XML: &str = include_str!("fixtures/invoices/sample-simplified-invoice.xml");
const ROOT: &str = "/ubl:Invoice";
const LINE: &str = "/ubl:Invoice/cac:InvoiceLine[1]";

fn change(xml: &str, path: &str, value: &str) -> String {
    test_xml::replace_text(xml, path, value)
}

fn invalid(xml: &str, field: &str) {
    let error: Error = parse_finalized_invoice_xml(xml).unwrap_err().into();
    assert_eq!(error.kind(), ErrorKind::InvalidInput, "{field}: {error}");
    let details: Value = serde_json::from_str(&error.details_json()).unwrap();
    assert_eq!(details["type"], "invalid_value", "{field}: {details}");
    assert_eq!(details["field"], field, "{details}");
}

#[test]
fn malformed_line_numbers_report_the_specific_field() {
    parse_finalized_invoice_xml(XML).unwrap();
    for (path, field) in [
        ("cbc:InvoicedQuantity", "InvoicedQuantity"),
        ("cac:Price/cbc:PriceAmount", "PriceAmount"),
        ("cbc:LineExtensionAmount", "LineExtensionAmount"),
        (
            "cac:Item/cac:ClassifiedTaxCategory/cbc:Percent",
            "LineVatPercent",
        ),
        ("cac:TaxTotal/cbc:TaxAmount", "LineTaxAmount"),
    ] {
        for value in ["NaN", "1,00", "79228162514264337593543950336"] {
            invalid(&change(XML, &format!("{LINE}/{path}"), value), field);
        }
    }
}

#[test]
fn line_precision_and_gross_checks_reject_one_cent_and_subcent_changes() {
    for (path, value, field) in [
        ("cbc:LineExtensionAmount", "99.001", "LineItem"),
        ("cac:TaxTotal/cbc:TaxAmount", "14.851", "LineItem"),
        ("cbc:LineExtensionAmount", "99.01", "LineGrossAmount"),
        (
            "cac:TaxTotal/cbc:RoundingAmount",
            "113.86",
            "LineGrossAmount",
        ),
    ] {
        invalid(&change(XML, &format!("{LINE}/{path}"), value), field);
    }
}

#[test]
fn adjustment_inputs_cannot_be_silently_coerced() {
    for (path, value, field) in [
        ("cbc:ChargeIndicator", "yes", "ChargeIndicator"),
        ("cbc:Amount", "-0.01", "AdjustmentAmount"),
        ("cbc:Amount", "0.001", "AdjustmentAmount"),
        ("cbc:Amount", "NaN", "AdjustmentAmount"),
        (
            "cac:TaxCategory[1]/cbc:ID",
            "UNKNOWN",
            "AdjustmentVatCategory",
        ),
        ("cac:TaxCategory[1]/cbc:Percent", "NaN", "AdjustmentVatRate"),
    ] {
        invalid(
            &change(XML, &format!("{ROOT}/cac:AllowanceCharge[1]/{path}"), value),
            field,
        );
    }
    // Both XML boolean spellings must have the same financial meaning.
    for value in ["false", "0", "true", "1"] {
        let parsed = parse_finalized_invoice_xml(&change(
            XML,
            &format!("{ROOT}/cac:AllowanceCharge[1]/cbc:ChargeIndicator"),
            value,
        ))
        .unwrap();
        assert_eq!(parsed.totals().payable_amount().to_string(), "231.15");
    }
}

#[test]
fn tax_subtotals_reject_unknown_and_duplicate_groups() {
    let path = format!("{ROOT}/cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cbc:Percent");
    invalid(&change(XML, &path, "14"), "TaxSubtotal");
    let start = XML.find("<cac:TaxSubtotal>").unwrap();
    let end = XML[start..].find("</cac:TaxSubtotal>").unwrap() + start + "</cac:TaxSubtotal>".len();
    let duplicated = format!("{}{}{}", &XML[..end], &XML[start..end], &XML[end..]);
    invalid(&duplicated, "TaxSubtotal");
}

#[test]
fn prepaid_and_rounding_values_preserve_the_payable_equation() {
    let with_amounts = XML.replace("</cac:LegalMonetaryTotal>", "<cbc:PayableRoundingAmount currencyID=\"SAR\">-0.01</cbc:PayableRoundingAmount></cac:LegalMonetaryTotal>");
    let with_amounts = change(
        &with_amounts,
        &format!("{ROOT}/cac:LegalMonetaryTotal/cbc:PrepaidAmount"),
        "31.15",
    );
    let xml = change(
        &with_amounts,
        &format!("{ROOT}/cac:LegalMonetaryTotal/cbc:PayableAmount"),
        "199.99",
    );
    let invoice = parse_finalized_invoice_xml(&xml).unwrap();
    assert_eq!(invoice.totals().prepaid_amount().to_string(), "31.15");
    assert_eq!(
        invoice.totals().payable_rounding_amount().to_string(),
        "-0.01"
    );
    assert_eq!(invoice.totals().payable_amount().to_string(), "199.99");
    for (field, value) in [
        ("PrepaidAmount", "-0.01"),
        ("PrepaidAmount", "31.151"),
        ("PayableRoundingAmount", "0.001"),
        ("PayableAmount", "200.00"),
    ] {
        invalid(
            &change(
                &xml,
                &format!("{ROOT}/cac:LegalMonetaryTotal/cbc:{field}"),
                value,
            ),
            field,
        );
    }
}

#[test]
fn malformed_dates_and_counters_are_not_normalized() {
    for (path, value, field) in [
        ("cbc:IssueDate", "2024-02-30", "IssueDateTime"),
        ("cbc:IssueTime", "24:01:00", "IssueTime"),
        (
            "cac:AdditionalDocumentReference[cbc:ID='ICV']/cbc:UUID",
            "-1",
            "ICV",
        ),
        (
            "cac:AdditionalDocumentReference[cbc:ID='ICV']/cbc:UUID",
            "18446744073709551616",
            "ICV",
        ),
    ] {
        invalid(&change(XML, &format!("{ROOT}/{path}"), value), field);
    }
}

#[test]
fn signed_parser_rejects_incomplete_or_ambiguous_qr_tlv() {
    let doc = roxmltree::Document::parse(XML).unwrap();
    let qr_path = "//*[local-name()='AdditionalDocumentReference'][*[local-name()='ID']='QR']//*[local-name()='EmbeddedDocumentBinaryObject']";
    let qr = test_xml::text(&doc, qr_path).unwrap();
    let raw = Base64::decode_vec(&qr).unwrap();
    let mut trailing_header = raw.clone();
    trailing_header.push(10);
    let mut truncated_value = raw.clone();
    truncated_value.extend([10, 2, 1]);
    let mut duplicate_hash = raw;
    duplicate_hash.extend([6, 1, b'x']);
    for value in [
        "%%%".to_owned(),
        Base64::encode_string(&trailing_header),
        Base64::encode_string(&truncated_value),
        Base64::encode_string(&duplicate_hash),
    ] {
        let xml = change(XML, qr_path, &value);
        let error: Error = parse_signed_invoice_xml(&xml)
            .expect_err("malformed QR must be rejected")
            .into();
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        let details: Value = serde_json::from_str(&error.details_json()).unwrap();
        assert_eq!(details["field"], "QR");
    }
}

#[test]
fn imported_transaction_flags_survive_serialization() {
    // The five digits follow the subtype in wire order: third party, nominal,
    // export, summary, self billed. Exercise each bit and their combinations.
    for bits in 0u8..32 {
        let digits: String = (0..5)
            .map(|bit| if bits & (1 << bit) == 0 { '0' } else { '1' })
            .collect();
        for subtype in ["01", "02"] {
            let code = format!("{subtype}{digits}");
            let xml = change(XML, &format!("{ROOT}/cbc:InvoiceTypeCode/@name"), &code);
            let invoice = parse_finalized_invoice_xml(&xml).unwrap();
            assert_eq!(invoice.data().flags().bits(), bits, "wire code {code}");
            assert_eq!(
                invoice.data().invoice_type().is_simplified(),
                subtype == "02"
            );
            assert!(
                invoice
                    .to_xml()
                    .unwrap()
                    .contains(&format!("name=\"{code}\""))
            );
        }
    }
    for code in ["02", "02000000", "02000x0", "0200020", "0300000", "02é000"] {
        invalid(
            &change(XML, &format!("{ROOT}/cbc:InvoiceTypeCode/@name"), code),
            "InvoiceTypeCode@name",
        );
    }
}

#[test]
fn signed_parser_requires_each_qr_signature_component() {
    use fatoora_core::invoice::xml::parse::ParseError;
    let doc = roxmltree::Document::parse(XML).unwrap();
    let path = "//*[local-name()='AdditionalDocumentReference'][*[local-name()='ID']='QR']//*[local-name()='EmbeddedDocumentBinaryObject']";
    let raw = Base64::decode_vec(&test_xml::text(&doc, path).unwrap()).unwrap();
    for (missing, label) in [
        (6, "QR tag 6 (invoice hash)"),
        (7, "QR tag 7 (signature)"),
        (8, "QR tag 8 (public key)"),
    ] {
        let mut without_tag = Vec::new();
        let mut offset = 0;
        while offset < raw.len() {
            let end = offset + 2 + usize::from(raw[offset + 1]);
            if raw[offset] != missing {
                without_tag.extend_from_slice(&raw[offset..end]);
            }
            offset = end;
        }
        let xml = change(XML, path, &Base64::encode_string(&without_tag));
        assert!(
            matches!(parse_signed_invoice_xml(&xml), Err(ParseError::MissingField(field)) if field == label)
        );
    }
    // Tags 6 and 7 carry UTF-8 text, unlike the binary public key in tag 8.
    for (tag, label) in [(6, "QR tag 6 (invoice hash)"), (7, "QR tag 7 (signature)")] {
        let mut corrupted = raw.clone();
        let mut offset = 0;
        while corrupted[offset] != tag {
            offset += 2 + usize::from(corrupted[offset + 1]);
        }
        corrupted[offset + 2] = 0xff;
        let xml = change(XML, path, &Base64::encode_string(&corrupted));
        assert!(
            matches!(parse_signed_invoice_xml(&xml), Err(ParseError::MissingField(field)) if field == label)
        );
    }
}

#[test]
fn imported_note_preserves_language_text_and_default_language() {
    let text = "خصم & تسوية <فاتورة>";
    let xml = XML.replace(
        ">ABC</cbc:Note>",
        ">خصم &amp; تسوية &lt;فاتورة&gt;</cbc:Note>",
    );
    for (xml, language) in [
        (xml.clone(), "ar"),
        (xml.replace(" languageID=\"ar\"", ""), "en"),
    ] {
        let invoice = parse_finalized_invoice_xml(&xml).unwrap();
        let note = invoice.data().note().unwrap();
        assert_eq!(note.text(), text);
        assert_eq!(note.language(), language);
        let serialized = invoice.to_xml().unwrap();
        let reparsed = parse_finalized_invoice_xml(&serialized).unwrap();
        assert_eq!(reparsed.data().note(), invoice.data().note());
    }
}

#[test]
fn adjustment_vat_groups_cannot_be_merged_or_reassigned() {
    // Even zero-value adjustments must refer to a group that actually exists.
    invalid(
        &change(
            XML,
            &format!("{ROOT}/cac:AllowanceCharge[1]/cac:TaxCategory[1]/cbc:Percent"),
            "5",
        ),
        "AllowanceCharge",
    );
    let start = XML.find("<cac:AllowanceCharge>").unwrap();
    let end = XML[start..].find("</cac:AllowanceCharge>").unwrap()
        + start
        + "</cac:AllowanceCharge>".len();
    let second = XML[start..end].replace(">15<", ">5<");
    assert_ne!(second, XML[start..end]);
    let xml = format!("{}{}{}", &XML[..end], second, &XML[end..]);
    // The additional category/rate needs its own supplied breakdown, even at zero.
    assert!(parse_finalized_invoice_xml(&xml).is_err());
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let group_total = test_xml::select(&doc, "/ubl:Invoice/cac:TaxTotal")
        .into_iter()
        .find(|n| n.children().any(|c| c.tag_name().name() == "TaxSubtotal"))
        .unwrap();
    let insert_at = group_total.range().end - "</cac:TaxTotal>".len();
    let mut xml = xml;
    xml.insert_str(insert_at, "<cac:TaxSubtotal><cbc:TaxableAmount currencyID=\"SAR\">0.00</cbc:TaxableAmount><cbc:TaxAmount currencyID=\"SAR\">0.00</cbc:TaxAmount><cac:TaxCategory><cbc:ID>S</cbc:ID><cbc:Percent>5</cbc:Percent><cac:TaxScheme><cbc:ID>VAT</cbc:ID></cac:TaxScheme></cac:TaxCategory></cac:TaxSubtotal>");

    let parsed = parse_finalized_invoice_xml(&xml).unwrap();
    assert_eq!(parsed.data().adjustments().len(), 2);
    assert_eq!(
        parsed.data().adjustments()[1].rate,
        fatoora_core::Decimal::from(5)
    );
}

#[test]
fn malformed_xml_and_missing_lines_are_distinct_import_failures() {
    use fatoora_core::invoice::xml::parse::ParseError;
    for parse in [
        parse_finalized_invoice_xml("").map(|_| ()),
        parse_signed_invoice_xml("").map(|_| ()),
    ] {
        let error = parse.unwrap_err();
        assert!(matches!(error, ParseError::XmlParse(_)));
        assert_eq!(error.kind(), ErrorKind::Xml);
    }
    let doc = roxmltree::Document::parse(XML).unwrap();
    let mut without_lines = XML.to_owned();
    let mut ranges: Vec<_> = test_xml::select(&doc, "//*[local-name()='InvoiceLine']")
        .into_iter()
        .map(|node| node.range())
        .collect();
    ranges.sort_by_key(|range| std::cmp::Reverse(range.start));
    for range in ranges {
        without_lines.replace_range(range, "");
    }
    assert!(matches!(
        parse_finalized_invoice_xml(&without_lines),
        Err(ParseError::MissingField("VatCategory"))
    ));
    invalid(
        &change(
            XML,
            &format!("{ROOT}/cac:InvoiceLine[2]/cac:Item/cac:ClassifiedTaxCategory/cbc:ID"),
            "UNKNOWN",
        ),
        "LineVatCategory",
    );
}

#[test]
fn complete_document_limit_is_shared_by_import_hashing_and_validation() {
    let limit = 8 * 1024 * 1024;
    let mut input = XML.to_owned();
    let comment = format!("<!--{}-->", "x".repeat(limit + 1 - input.len() - 7));
    input.insert_str(input.find("</Invoice>").unwrap(), &comment);
    assert_eq!(input.len(), limit + 1);
    let imported = parse_finalized_invoice_xml(&input).unwrap_err().to_string();
    let hashed = fatoora_core::invoice::sign::invoice_hash_base64_from_xml_str(&input)
        .unwrap_err()
        .to_string();
    let validated = fatoora_core::invoice::validation::validate_xml_invoice_from_str(
        &input,
        &fatoora_core::config::Config::default(),
    )
    .unwrap_err()
    .to_string();
    for error in [imported, hashed, validated] {
        assert!(error.contains("XML input bytes"), "{error}");
    }
}

#[test]
fn blank_required_scalars_report_their_own_field_and_leave_next_parse_usable() {
    use fatoora_core::invoice::xml::parse::ParseError;
    let seller = "cac:AccountingSupplierParty/cac:Party";
    let paths = [
        ("cbc:ID".to_owned(), "ID"),
        ("cbc:UUID".to_owned(), "UUID"),
        ("cbc:IssueDate".to_owned(), "IssueDate"),
        ("cbc:IssueTime".to_owned(), "IssueTime"),
        ("cbc:InvoiceTypeCode".to_owned(), "InvoiceTypeCode"),
        ("cbc:InvoiceTypeCode/@name".to_owned(), "InvoiceTypeCode@name"),
        ("cbc:DocumentCurrencyCode".to_owned(), "DocumentCurrencyCode"),
        ("cac:PaymentMeans/cbc:PaymentMeansCode".to_owned(), "PaymentMeansCode"),
        (format!("{seller}/cac:PartyLegalEntity/cbc:RegistrationName"), "SellerName"),
        (format!("{seller}/cac:PartyTaxScheme/cbc:CompanyID"), "SellerVAT"),
        (format!("{seller}/cac:PostalAddress/cbc:StreetName"), "SellerStreet"),
        (format!("{seller}/cac:PostalAddress/cbc:BuildingNumber"), "SellerBuildingNumber"),
        (format!("{seller}/cac:PostalAddress/cbc:CityName"), "SellerCity"),
        (format!("{seller}/cac:PostalAddress/cbc:PostalZone"), "SellerPostalCode"),
        (format!("{seller}/cac:PostalAddress/cac:Country/cbc:IdentificationCode"), "SellerCountry"),
        ("cac:AdditionalDocumentReference[cbc:ID='PIH']/cac:Attachment/cbc:EmbeddedDocumentBinaryObject".to_owned(), "PIH"),
        ("cac:AdditionalDocumentReference[cbc:ID='ICV']/cbc:UUID".to_owned(), "ICV"),
        ("cac:InvoiceLine[1]/cbc:ID".to_owned(), "LineID"),
        ("cac:InvoiceLine[1]/cac:Item/cbc:Name".to_owned(), "LineName"),
        ("cac:InvoiceLine[1]/cbc:InvoicedQuantity".to_owned(), "InvoicedQuantity"),
        ("cac:InvoiceLine[1]/cbc:InvoicedQuantity/@unitCode".to_owned(), "InvoicedQuantity@unitCode"),
        ("cac:InvoiceLine[1]/cbc:LineExtensionAmount".to_owned(), "LineExtensionAmount"),
        ("cac:InvoiceLine[1]/cac:Price/cbc:PriceAmount".to_owned(), "PriceAmount"),
    ];
    let expected = parse_finalized_invoice_xml(XML)
        .unwrap()
        .hash_base64()
        .unwrap();
    for (path, field) in paths {
        let xml = change(XML, &format!("{ROOT}/{path}"), " \t\n");
        let error = parse_finalized_invoice_xml(&xml).unwrap_err();
        assert!(
            matches!(error, ParseError::MissingField(actual) if actual == field),
            "{field}: {error}"
        );
        let error: Error = error.into();
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        let details: Value = serde_json::from_str(&error.details_json()).unwrap();
        assert_eq!(
            details,
            serde_json::json!({"type":"missing_field", "field":field})
        );
    }
    assert_eq!(
        parse_finalized_invoice_xml(XML)
            .unwrap()
            .hash_base64()
            .unwrap(),
        expected
    );
}

#[test]
fn invalid_country_vat_category_and_monetary_totals_keep_specific_diagnostics() {
    for (path, field) in [
        (
            "cac:AccountingSupplierParty/cac:Party/cac:PostalAddress/cac:Country/cbc:IdentificationCode",
            "SellerCountry",
        ),
        (
            "cac:InvoiceLine[1]/cac:Item/cac:ClassifiedTaxCategory/cbc:ID",
            "VatCategory",
        ),
        (
            "cac:LegalMonetaryTotal/cbc:LineExtensionAmount",
            "LineExtensionAmount",
        ),
        (
            "cac:LegalMonetaryTotal/cbc:TaxExclusiveAmount",
            "TaxExclusiveAmount",
        ),
        (
            "cac:LegalMonetaryTotal/cbc:TaxInclusiveAmount",
            "TaxInclusiveAmount",
        ),
        ("cac:LegalMonetaryTotal/cbc:PayableAmount", "PayableAmount"),
    ] {
        invalid(&change(XML, &format!("{ROOT}/{path}"), "INVALID"), field);
    }
}

#[test]
fn unsupported_structure_and_duplicate_document_identifiers_are_rejected() {
    invalid(
        &XML.replace("<cbc:UUID>", "<cbc:UUID><cbc:ID>nested</cbc:ID>"),
        "Content",
    );
    for id in ["ICV", "PIH", "QR"] {
        let duplicate = format!(
            "<cac:AdditionalDocumentReference><cbc:ID>{id}</cbc:ID></cac:AdditionalDocumentReference></Invoice>"
        );
        invalid(
            &XML.replace("</Invoice>", &duplicate),
            "AdditionalDocumentReference",
        );
    }
    for xml in [
        XML.replacen(
            "<cac:InvoiceLine>",
            "<cac:InvoiceLine><cac:AllowanceCharge/>",
            1,
        ),
        XML.replacen("<cac:Price>", "<cac:Price><cac:AllowanceCharge/>", 1),
    ] {
        assert!(parse_finalized_invoice_xml(&xml).is_err());
    }
    for quantity in ["0", "2", "0.5"] {
        let xml = XML.replacen(
            "<cac:Price>",
            &format!("<cac:Price><cbc:BaseQuantity>{quantity}</cbc:BaseQuantity>"),
            1,
        );
        assert!(parse_finalized_invoice_xml(&xml).is_err());
    }
    let unit_basis = XML.replacen(
        "<cac:Price>",
        "<cac:Price><cbc:BaseQuantity>1</cbc:BaseQuantity>",
        1,
    );
    let parsed = parse_finalized_invoice_xml(&unit_basis).unwrap();
    assert_eq!(
        parsed.data().line_items()[0].item_price_base_quantity(),
        Some("1".parse().unwrap())
    );
    assert_eq!(
        parsed.totals(),
        parse_finalized_invoice_xml(XML).unwrap().totals()
    );
    assert!(
        parse_finalized_invoice_xml(&XML.replacen(
            "<cac:Price>",
            "<cac:Price><cbc:BaseQuantity>bad</cbc:BaseQuantity>",
            1
        ))
        .is_err()
    );
    let doc = roxmltree::Document::parse(XML).unwrap();
    let customer = test_xml::select(&doc, "/ubl:Invoice/cac:AccountingCustomerParty")[0];
    let mut malformed = XML.to_owned();
    malformed.replace_range(
        customer.range(),
        "<cac:AccountingCustomerParty>not a party</cac:AccountingCustomerParty>",
    );
    assert!(
        parse_finalized_invoice_xml(&malformed)
            .unwrap_err()
            .to_string()
            .contains("BuyerParty")
    );
}

#[test]
fn supplied_invalid_party_identifiers_do_not_become_absent() {
    let seller =
        "/ubl:Invoice/cac:AccountingSupplierParty/cac:Party/cac:PartyIdentification/cbc:ID";
    assert!(
        parse_finalized_invoice_xml(&change(XML, seller, ""))
            .unwrap_err()
            .to_string()
            .contains("SellerID")
    );
    let customer = "<cac:AccountingCustomerParty><cac:Party><cac:PartyIdentification><cbc:ID>buyer-id</cbc:ID></cac:PartyIdentification><cac:PostalAddress><cac:Country><cbc:IdentificationCode>SA</cbc:IdentificationCode></cac:Country></cac:PostalAddress><cac:PartyTaxScheme><cbc:CompanyID> </cbc:CompanyID><cac:TaxScheme><cbc:ID>VAT</cbc:ID></cac:TaxScheme></cac:PartyTaxScheme><cac:PartyLegalEntity><cbc:RegistrationName>Buyer</cbc:RegistrationName></cac:PartyLegalEntity></cac:Party></cac:AccountingCustomerParty>";
    let doc = roxmltree::Document::parse(XML).unwrap();
    let old = test_xml::select(&doc, "/ubl:Invoice/cac:AccountingCustomerParty")[0];
    let mut modified = XML.to_owned();
    modified.replace_range(old.range(), customer);
    assert!(parse_finalized_invoice_xml(&modified).is_err());
    let invalid_customer = customer
        .replace(
            "<cbc:CompanyID> </cbc:CompanyID>",
            "<cbc:CompanyID>310122393500003</cbc:CompanyID>",
        )
        .replace(
            "<cbc:IdentificationCode>SA</cbc:IdentificationCode>",
            "<cbc:IdentificationCode>INVALID</cbc:IdentificationCode>",
        );
    let mut invalid_address = XML.to_owned();
    invalid_address.replace_range(old.range(), &invalid_customer);
    assert!(parse_finalized_invoice_xml(&invalid_address).is_err());
    let invalid_currency = change(XML, "/ubl:Invoice/cbc:DocumentCurrencyCode", "INVALID");
    assert!(parse_finalized_invoice_xml(&invalid_currency).is_err());
}

#[test]
fn recognized_aggregate_nesting_is_bounded_before_model_recursion() {
    use fatoora_core::invoice::xml::parse::{
        ParseError, parse_finalized_invoice_xml_file, parse_signed_invoice_xml_file,
    };
    // The Invoice root is level 1. These are recognized CAC nodes, so the model
    // visitor traverses the entire accepted chain rather than skipping it.
    let nested = |depth: usize| {
        XML.replace(
            "</Invoice>",
            &format!(
                "{}{} </Invoice>",
                "<cac:Party>".repeat(depth - 1),
                "</cac:Party>".repeat(depth - 1)
            ),
        )
    };
    let accepted = nested(128);
    assert_eq!(
        parse_finalized_invoice_xml(&accepted).unwrap(),
        parse_finalized_invoice_xml(XML).unwrap()
    );
    assert_eq!(parse_signed_invoice_xml(&accepted).unwrap().xml(), accepted);
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), &accepted).unwrap();
    assert_eq!(
        parse_finalized_invoice_xml_file(file.path()).unwrap(),
        parse_finalized_invoice_xml(XML).unwrap()
    );
    assert_eq!(
        parse_signed_invoice_xml_file(file.path()).unwrap().xml(),
        accepted
    );
    for depth in [129, 50_000] {
        let rejected = nested(depth);
        std::fs::write(file.path(), &rejected).unwrap();
        for error in [
            parse_finalized_invoice_xml(&rejected).unwrap_err(),
            parse_signed_invoice_xml(&rejected).unwrap_err(),
            parse_finalized_invoice_xml_file(file.path()).unwrap_err(),
            parse_signed_invoice_xml_file(file.path()).unwrap_err(),
        ] {
            assert!(matches!(error, ParseError::XmlParse(_)), "{error}");
            assert!(error.to_string().contains("XML depth"), "{error}");
        }
    }
    assert!(parse_finalized_invoice_xml(XML).is_ok());
}
