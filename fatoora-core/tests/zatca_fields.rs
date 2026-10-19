mod common;
use fatoora_core::{
    Decimal,
    config::Config,
    invoice::{
        validation::validate_xml_invoice_from_str,
        xml::parse::parse_finalized_invoice_xml as parse, *,
    },
};
fn d(v: &str) -> Decimal {
    v.parse().unwrap()
}
fn builder() -> InvoiceBuilder {
    let source = common::dummy_finalized_invoice();
    InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("fields")
        .uuid(source.data().uuid())
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash(source.data().previous_invoice_hash())
        .invoice_counter(1)
        .seller(source.data().seller().clone())
        .vat_category(VatCategory::Standard)
}
fn line() -> LineItemBuilder {
    LineItem::builder()
        .item_name("سلعة & item")
        .invoiced_quantity(d("30"))
        .invoiced_quantity_unit_code("PCE")
        .item_net_price(d("100"))
        .vat_category(VatCategory::Standard)
        .vat_rate(d("15"))
}
type XmlLeaf = (String, Vec<(String, String)>, String);
fn leaves(xml: &str) -> Vec<XmlLeaf> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    doc.descendants()
        .filter(|n| n.is_element() && !n.children().any(|c| c.is_element()))
        .map(|n| {
            let path = n
                .ancestors()
                .filter(|n| n.is_element())
                .map(|n| {
                    format!(
                        "{{{}}}{}",
                        n.tag_name().namespace().unwrap_or(""),
                        n.tag_name().name()
                    )
                })
                .collect::<Vec<_>>()
                .join("/");
            let attrs = n
                .attributes()
                .map(|a| (a.name().into(), a.value().into()))
                .collect();
            (path, attrs, n.text().unwrap_or("").into())
        })
        .collect()
}
#[test]
fn complete_line_and_metadata_preserve_source_fields_and_schema() {
    let item = line()
        .item_price_base_quantity(d("12"))
        .item_price_base_quantity_unit_code("PCE")
        .item_price_discount(d("20"))
        .item_gross_price(d("120"))
        .item_seller_identifier(OtherId::new("SKU-1"))
        .item_buyer_identifier(OtherId::new("BUY-1"))
        .item_standard_identifier(OtherId::with_scheme("1234567890123", "0160"))
        .allowance(
            Adjustment::percentage(d("250"), d("10"))
                .reason("rebate")
                .reason_code("95"),
        )
        .charge(Adjustment::amount(d("5")).reason("delivery"))
        .build()
        .unwrap();
    assert_eq!(item.total_amount(), d("230"));
    let invoice = builder()
        .line_item(item)
        .note(InvoiceNote::new("ar", "الأولى"))
        .note(InvoiceNote::without_language("second & note"))
        .purchase_order_id("PO-1")
        .contract_id("CON-1")
        .delivery(Delivery {
            actual_date: Some("2024-01-01".parse().unwrap()),
            latest_date: Some("2024-01-02".parse().unwrap()),
        })
        .payment_means(
            PaymentMeans::new("30").payee_financial_account(PaymentAccount {
                payment_account_identifier: Some("ACCOUNT".into()),
                payment_terms: Some("30 days".into()),
            }),
        )
        .payment_means(PaymentMeans::new("10"))
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    validate_xml_invoice_from_str(&xml, &Config::default()).unwrap();
    let imported = parse(&xml).unwrap();
    let regenerated = imported.to_xml().unwrap();
    validate_xml_invoice_from_str(&regenerated, &Config::default()).unwrap();
    assert_eq!(leaves(&xml), leaves(&regenerated));
    assert_eq!(invoice, imported);
}
#[test]
fn accepted_line_and_group_tax_values_drive_serialized_totals() {
    let item = line()
        .item_price_base_quantity(d("12"))
        .line_net_amount(d("250.01"))
        .build()
        .unwrap();
    let invoice = builder().line_item(item).build().unwrap();
    assert_eq!(invoice.totals().line_extension(), d("250.01"));
    let xml = invoice.to_xml().unwrap();
    // Supply a group tax one cent above the computed tax and propagate it through totals.
    let modified = xml
        .replace(">37.50</cbc:TaxAmount>", ">37.51</cbc:TaxAmount>")
        .replace(
            ">287.51</cbc:TaxInclusiveAmount>",
            ">287.52</cbc:TaxInclusiveAmount>",
        )
        .replace(">287.51</cbc:PayableAmount>", ">287.52</cbc:PayableAmount>")
        .replace(
            ">287.51</cbc:RoundingAmount>",
            ">287.52</cbc:RoundingAmount>",
        );
    let imported = parse(&modified).unwrap();
    assert_eq!(imported.totals().tax_amount(), d("37.51"));
    assert_eq!(imported.totals().payable_amount(), d("287.52"));
    assert_eq!(leaves(&modified), leaves(&imported.to_xml().unwrap()));
    assert!(
        line()
            .item_price_base_quantity(d("12"))
            .line_net_amount(d("250.02"))
            .build()
            .is_err()
    );
    assert!(line().item_price_base_quantity(d("0")).build().is_err());
}
#[test]
fn optional_containers_are_not_missing_required_children() {
    let invoice = builder()
        .line_item(line().build().unwrap())
        .buyer(Buyer::new("Buyer".into(), None, None, None).unwrap())
        .delivery(Delivery::default())
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    validate_xml_invoice_from_str(&xml, &Config::default()).unwrap();
    assert_eq!(
        leaves(&xml),
        leaves(&parse(&xml).unwrap().to_xml().unwrap())
    );
    let invalid = xml.replace("<cac:Delivery/>", "<cac:PaymentMeans/>");
    assert_ne!(invalid, xml);
    assert!(parse(&invalid).is_err());
}
#[test]
fn duplicate_group_metadata_errors_and_line_metadata_stays_independent() {
    let invoice = builder()
        .line_item(line().build().unwrap())
        .tax_exemption(
            VatCategory::Standard,
            d("15"),
            TaxExemption {
                reason_code: None,
                reason: Some("one".into()),
            },
        )
        .tax_exemption(
            VatCategory::Standard,
            d("15"),
            TaxExemption {
                reason_code: None,
                reason: Some("two".into()),
            },
        );
    assert!(invoice.build().is_err());
}

#[path = "support/xml.rs"]
#[allow(dead_code)]
mod source_xml;

#[test]
fn dictionary_source_to_output_coverage() {
    use serde_json::Value;
    let manifest: Value =
        serde_json::from_str(include_str!("fixtures/model-coverage.json")).unwrap();
    let fields = manifest["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 150);
    let mut covered = std::collections::HashSet::new();
    let mut sources = Vec::new();
    fn collect(path: &std::path::Path, sources: &mut Vec<String>) {
        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect(&path, sources);
            } else if path.extension().is_some_and(|e| e == "xml") {
                sources.push(std::fs::read_to_string(path).unwrap());
            }
        }
    }
    collect(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invoices"),
        &mut sources,
    );
    let mut address = common::dummy_finalized_invoice()
        .data()
        .seller()
        .address()
        .unwrap()
        .clone();
    address.province = Some("Riyadh Province".into());
    address.additional_street = Some("Second street".into());
    let seller = Seller::new(
        "Seller".into(),
        address.clone(),
        "399999999900003",
        Some(OtherId::with_scheme("123", "CRN")),
    )
    .unwrap();
    let item = line()
        .item_price_base_quantity(d("12"))
        .item_price_base_quantity_unit_code("PCE")
        .item_gross_price(d("120"))
        .item_price_discount(d("20"))
        .item_seller_identifier(OtherId::new("seller-sku"))
        .item_buyer_identifier(OtherId::new("buyer-sku"))
        .item_standard_identifier(OtherId::with_scheme("123", "0160"))
        .allowance(
            Adjustment::percentage(d("250"), d("10"))
                .reason("line allowance")
                .reason_code("95"),
        )
        .charge(
            Adjustment::percentage(d("250"), d("2"))
                .reason("line charge")
                .reason_code("AA"),
        )
        .build()
        .unwrap();
    let rich = builder()
        .seller(seller)
        .buyer(
            Buyer::new(
                "Buyer".into(),
                address,
                Some("399999999900003".into()),
                Some(OtherId::with_scheme("456", "CRN")),
            )
            .unwrap(),
        )
        .line_item(item)
        .purchase_order_id("PO")
        .contract_id("CON")
        .note(InvoiceNote::new("ar", "note one"))
        .note(InvoiceNote::without_language("note two"))
        .payment_means(
            PaymentMeans::new("30")
                .instruction_note("terms")
                .payee_financial_account(PaymentAccount {
                    payment_account_identifier: Some("bank".into()),
                    payment_terms: Some("net 30".into()),
                }),
        )
        .allowance(DocumentAdjustment::new(
            Adjustment::percentage(d("230"), d("10"))
                .reason("allowance")
                .reason_code("95"),
            VatCategory::Standard,
            d("15"),
        ))
        .charge(DocumentAdjustment::new(
            Adjustment::percentage(d("230"), d("2"))
                .reason("charge")
                .reason_code("AA"),
            VatCategory::Standard,
            d("15"),
        ))
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    sources.push(rich);
    fn normalized(value: &str, path: &str) -> String {
        let name = path.rsplit('/').next().unwrap();
        if (name.contains("Amount")
            || name.contains("Price")
            || name.contains("Percent")
            || name.contains("Quantity")
            || name.contains("FactorNumeric"))
            && let Ok(number) = value.trim().parse::<Decimal>()
        {
            return number.to_string();
        }
        if name == "cbc:IdentificationCode"
            && let Ok(code) = CountryCode::parse(value.trim())
        {
            return code.as_str().to_string();
        }
        value.trim().to_string()
    }
    for source in sources {
        let invoice = parse(&source).unwrap();
        let output = invoice.to_xml().unwrap();
        let input_doc = roxmltree::Document::parse(&source).unwrap();
        let output_doc = roxmltree::Document::parse(&output).unwrap();
        for field in fields {
            let row = field["row"].as_u64().unwrap();
            if matches!(row, 19 | 20) {
                // Signatures and QR belong to the retained source, never reconstructed XML.
                continue;
            }
            let path = field["xml_path"].as_str().unwrap();
            let (element, attribute) = path
                .rsplit_once("/@")
                .map_or((path, None), |(p, a)| (p, Some(a)));
            let values = |doc: &roxmltree::Document<'_>| {
                source_xml::select(doc, element)
                    .into_iter()
                    .filter_map(|n| {
                        let v = match attribute {
                            Some(a) => n.attribute(a)?.to_owned(),
                            None => source_xml::content(n),
                        };
                        let attrs = field["attributes"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|a| {
                                let name = a.as_str().unwrap();
                                (name.to_owned(), n.attribute(name).map(str::to_owned))
                            })
                            .collect::<Vec<_>>();
                        Some((normalized(&v, path), attrs))
                    })
                    .collect::<Vec<_>>()
            };
            let supplied = values(&input_doc);
            if !supplied.is_empty() {
                covered.insert(row);
                assert_eq!(supplied, values(&output_doc), "row {row} {path}");
            }
        }
    }
    let signed = include_str!("fixtures/invoices/sample-simplified-invoice.xml");
    assert_eq!(
        fatoora_core::invoice::xml::parse::parse_signed_invoice_xml(signed)
            .unwrap()
            .xml(),
        signed
    );
    covered.extend([19, 20]);
    let missing: Vec<_> = fields
        .iter()
        .filter(|f| !covered.contains(&f["row"].as_u64().unwrap()))
        .map(|f| (&f["row"], &f["term_id"], &f["xml_path"]))
        .collect();
    assert!(
        missing.is_empty(),
        "untested dictionary fields: {missing:?}"
    );
}

#[test]
fn supported_container_shapes_have_distinct_validation_outcomes() {
    let source = builder()
        .line_item(line().build().unwrap())
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    for container in ["<cac:PaymentMeans/>", "<cac:AllowanceCharge/>"] {
        let malformed = source.replacen("<cac:TaxTotal>", &format!("{container}<cac:TaxTotal>"), 1);
        assert!(validate_xml_invoice_from_str(&malformed, &Config::default()).is_err());
        assert!(parse(&malformed).is_err());
    }
    let empty_totals = source_xml::select(
        &roxmltree::Document::parse(&source).unwrap(),
        "/ubl:Invoice/cac:LegalMonetaryTotal",
    )[0]
    .range();
    let mut invalid = source.clone();
    invalid.replace_range(empty_totals, "<cac:LegalMonetaryTotal/>");
    assert!(parse(&invalid).is_err());
    // A partial financial account is structurally permitted; it remains present.
    let invoice = builder()
        .line_item(line().build().unwrap())
        .payment_means(
            PaymentMeans::new("30").payee_financial_account(PaymentAccount {
                payment_account_identifier: None,
                payment_terms: Some("terms only".into()),
            }),
        )
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    validate_xml_invoice_from_str(&xml, &Config::default()).unwrap();
    assert_eq!(
        leaves(&xml),
        leaves(&parse(&xml).unwrap().to_xml().unwrap())
    );
}

#[test]
fn supplied_breakdown_amounts_and_tolerance_boundaries() {
    for supplied in ["37.49", "37.50", "37.51"] {
        let invoice = builder()
            .line_item(line().item_price_base_quantity(d("12")).build().unwrap())
            .vat_breakdown(
                VatBreakdown::new(VatCategory::Standard, d("15"), d("250"), d(supplied)).unwrap(),
            )
            .build()
            .unwrap();
        assert_eq!(invoice.totals().tax_amount(), d(supplied));
        let xml = invoice.to_xml().unwrap();
        assert_eq!(
            leaves(&xml),
            leaves(&parse(&xml).unwrap().to_xml().unwrap())
        );
    }
    for (taxable, tax) in [("250", "37.48"), ("250", "37.52"), ("250.01", "37.50")] {
        assert!(
            builder()
                .line_item(line().item_price_base_quantity(d("12")).build().unwrap())
                .vat_breakdown(
                    VatBreakdown::new(VatCategory::Standard, d("15"), d(taxable), d(tax)).unwrap()
                )
                .build()
                .is_err()
        );
    }
    // Rational division is rounded only after multiplying by quantity.
    assert_eq!(
        line()
            .item_net_price(d("1"))
            .item_price_base_quantity(d("3"))
            .invoiced_quantity(d("3"))
            .build()
            .unwrap()
            .total_amount(),
        d("1")
    );
}

#[test]
fn prepayment_adjustments_are_not_sales_lines() {
    let advance = LineItem::builder()
        .id("advance-original")
        .item_name("advance adjustment")
        .invoiced_quantity(d("0"))
        .item_net_price(d("0"))
        .vat_category(VatCategory::Standard)
        .vat_rate(d("15"))
        .document_reference(OriginalInvoiceRef::new("advance").with_document_type_code("386"))
        .tax_subtotal(VatBreakdown::new(VatCategory::Standard, d("15"), d("100"), d("15")).unwrap())
        .build()
        .unwrap();
    assert!(advance.is_prepayment_adjustment());
    let invoice = builder()
        .line_item(line().build().unwrap())
        .line_item(advance.clone())
        .prepaid_amount(d("115"))
        .build()
        .unwrap();
    assert_eq!(invoice.totals().line_extension(), d("3000"));
    assert_eq!(invoice.totals().payable_amount(), d("3335"));
    assert!(
        builder()
            .line_item(line().build().unwrap())
            .line_item(advance)
            .prepaid_amount(d("114.99"))
            .build()
            .is_err()
    );
    let xml = invoice.to_xml().unwrap();
    assert_eq!(
        leaves(&xml),
        leaves(&parse(&xml).unwrap().to_xml().unwrap())
    );
    let invalid_sale = line()
        .document_reference(OriginalInvoiceRef::new("advance").with_document_type_code("386"))
        .build()
        .unwrap();
    assert!(builder().line_item(invalid_sale).build().is_err());
}

#[test]
fn repeated_identifiers_and_payment_instructions_keep_order() {
    let buyer = Buyer::new(
        "Buyer".into(),
        None,
        None,
        Some(OtherId::with_scheme("one", "NAT")),
    )
    .unwrap()
    .identifier(OtherId::with_scheme("two", "PAS"));
    let invoice = builder()
        .line_item(line().build().unwrap())
        .buyer(buyer)
        .seller_identifier(OtherId::with_scheme("second", "OTH"))
        .payment_means(
            PaymentMeans::new("10")
                .instruction_note("first")
                .instruction_note("second"),
        )
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    validate_xml_invoice_from_str(&xml, &Config::default()).unwrap();
    let parsed = parse(&xml).unwrap();
    assert_eq!(parsed.data().buyer().unwrap().identifiers().len(), 2);
    assert_eq!(
        parsed.data().payment_means()[0].instruction_notes,
        ["first", "second"]
    );
    assert_eq!(leaves(&xml), leaves(&parsed.to_xml().unwrap()));
}

#[test]
fn absent_line_tax_container_remains_absent_and_empty_is_rejected() {
    let xml = builder()
        .line_item(line().build().unwrap())
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let range = source_xml::select(&doc, "/ubl:Invoice/cac:InvoiceLine/cac:TaxTotal")[0].range();
    let mut absent = xml.clone();
    absent.replace_range(range.clone(), "");
    validate_xml_invoice_from_str(&absent, &Config::default()).unwrap();
    let regenerated = parse(&absent).unwrap().to_xml().unwrap();
    validate_xml_invoice_from_str(&regenerated, &Config::default()).unwrap();
    assert_eq!(leaves(&absent), leaves(&regenerated));
    let mut malformed = xml;
    malformed.replace_range(range, "<cac:TaxTotal/>");
    assert!(validate_xml_invoice_from_str(&malformed, &Config::default()).is_err());
    assert!(parse(&malformed).is_err());
}

#[test]
fn document_tax_order_and_optional_accounting_total_are_preserved() {
    let xml = builder()
        .line_item(line().build().unwrap())
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let totals = source_xml::select(&doc, "/ubl:Invoice/cac:TaxTotal");
    assert_eq!(totals.len(), 2);
    let first = totals[0].range();
    let second = totals[1].range();
    let mut reordered = xml.clone();
    reordered.replace_range(
        first.start..second.end,
        &format!("{}{}", &xml[second.clone()], &xml[first.clone()]),
    );
    let output = parse(&reordered).unwrap().to_xml().unwrap();
    assert_eq!(leaves(&reordered), leaves(&output));
    let mut without_accounting = xml.clone();
    without_accounting.replace_range(first, "");
    validate_xml_invoice_from_str(&without_accounting, &Config::default()).unwrap();
    let output = parse(&without_accounting).unwrap().to_xml().unwrap();
    assert_eq!(leaves(&without_accounting), leaves(&output));
    validate_xml_invoice_from_str(&output, &Config::default()).unwrap();
}

#[test]
fn utc_and_ast_clock_values_survive_import_including_midnight() {
    let xml = builder()
        .issue_datetime("2024-01-01T01:30:00Z")
        .line_item(line().build().unwrap())
        .billing_reference(
            OriginalInvoiceRef::new("advance")
                .with_issue_time(InvoiceTime::parse("23:59:59Z").unwrap()),
        )
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    assert!(xml.contains("<cbc:IssueTime>01:30:00Z</cbc:IssueTime>"));
    let local = xml.replacen(">01:30:00Z</cbc:IssueTime>", ">01:30:00</cbc:IssueTime>", 1);
    for source in [&xml, &local] {
        let parsed = parse(source).unwrap();
        validate_xml_invoice_from_str(source, &Config::default()).unwrap();
        assert_eq!(leaves(source), leaves(&parsed.to_xml().unwrap()));
        assert_eq!(
            parsed.data().billing_references()[0]
                .issue_time()
                .unwrap()
                .as_str(),
            "23:59:59Z"
        );
    }
    let parsed = parse(&local).unwrap();
    assert_eq!(
        parsed.data().issue_datetime().as_str(),
        "2023-12-31T22:30:00Z"
    );
    assert_eq!(parsed.data().issue_date(), "2024-01-01");
    assert_eq!(parsed.data().issue_time(), "01:30:00");
    assert_eq!(
        InvoiceTime::parse("24:00:00Z").unwrap().as_str(),
        "24:00:00Z"
    );
    assert!(InvoiceTime::parse("24:01:00Z").is_err());
    for (date, next_date) in [
        ("2024-01-31", "2024-02-01"),
        ("2024-02-28", "2024-02-29"),
        ("2024-02-29", "2024-03-01"),
        ("2024-12-31", "2025-01-01"),
    ] {
        for time in ["24:00:00", "24:00:00Z"] {
            let source = xml
                .replacen(
                    "2024-01-01</cbc:IssueDate>",
                    &format!("{date}</cbc:IssueDate>"),
                    1,
                )
                .replacen(
                    "01:30:00Z</cbc:IssueTime>",
                    &format!("{time}</cbc:IssueTime>"),
                    1,
                );
            let parsed = parse(&source).unwrap();
            let expected = if time.ends_with('Z') {
                format!("{next_date}T00:00:00Z")
            } else {
                format!("{date}T21:00:00Z")
            };
            assert_eq!(parsed.data().issue_datetime().as_str(), expected);
            assert_eq!(parsed.data().issue_date(), date);
            assert_eq!(parsed.data().issue_time(), time);
            let output = parsed.to_xml().unwrap();
            assert_eq!(leaves(&source), leaves(&output));
            validate_xml_invoice_from_str(&output, &Config::default()).unwrap();
        }
    }
    for time in ["24:00:01", "24:01:00Z", "25:00:00Z"] {
        let source = xml.replacen(
            "01:30:00Z</cbc:IssueTime>",
            &format!("{time}</cbc:IssueTime>"),
            1,
        );
        assert!(parse(&source).is_err());
    }
}

#[test]
fn vat_midpoint_tolerance_uses_the_pinned_rule_policy() {
    for (tax, accepted) in [
        ("2.64", false),
        ("2.65", true),
        ("2.66", true),
        ("2.67", true),
        ("2.68", false),
    ] {
        let result = builder()
            .line_item(
                line()
                    .invoiced_quantity(d("1"))
                    .item_net_price(d("53.30"))
                    .vat_rate(d("5"))
                    .build()
                    .unwrap(),
            )
            .vat_breakdown(
                VatBreakdown::new(VatCategory::Standard, d("5"), d("53.30"), d(tax)).unwrap(),
            )
            .build();
        assert_eq!(result.is_ok(), accepted, "tax {tax}");
    }
}

#[test]
fn payment_instruction_placeholder_retains_notes_and_payment_metadata() {
    let payment = PaymentMeans::new("30")
        .instruction_note("transfer")
        .payee_financial_account(PaymentAccount {
            payment_account_identifier: Some("bank".into()),
            payment_terms: Some("net 30".into()),
        });
    let invoice = builder()
        .line_item(line().build().unwrap())
        .payment_instruction("reason")
        .payment_means(payment.clone())
        .payment_means(PaymentMeans::new("10"))
        .build()
        .unwrap();
    let mut expected = payment;
    expected.instruction_notes.insert(0, "reason".into());
    assert_eq!(
        invoice.data().payment_means(),
        &[expected, PaymentMeans::new("10")]
    );
    let xml = invoice.to_xml().unwrap();
    assert_eq!(
        leaves(&xml),
        leaves(&parse(&xml).unwrap().to_xml().unwrap())
    );
    for b in [
        builder().payment_instruction("reason"),
        builder()
            .payment_means(PaymentMeans::new(""))
            .payment_means(PaymentMeans::new("30")),
        builder()
            .payment_instruction("reason")
            .payment_means_code("")
            .payment_means(PaymentMeans::new("30")),
    ] {
        assert!(b.line_item(line().build().unwrap()).build().is_err());
    }
}

#[test]
fn malformed_optional_prices_and_buyer_identity_are_rejected() {
    let xml = builder()
        .line_item(
            line()
                .item_gross_price(d("120"))
                .item_price_discount(d("20"))
                .allowance(Adjustment::amount(d("1")))
                .build()
                .unwrap(),
        )
        .buyer(Buyer::new("Buyer".into(), None, Some("301121971500003".into()), None).unwrap())
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    for (path, replacement) in [
        (
            "/ubl:Invoice/cac:InvoiceLine/cac:Price/cac:AllowanceCharge/cbc:Amount",
            "",
        ),
        (
            "/ubl:Invoice/cac:InvoiceLine/cac:Price/cac:AllowanceCharge/cbc:BaseAmount",
            "",
        ),
        (
            "/ubl:Invoice/cac:InvoiceLine/cac:AllowanceCharge/cbc:Amount",
            "",
        ),
        (
            "/ubl:Invoice/cac:AccountingCustomerParty/cac:Party/cac:PartyTaxScheme/cbc:CompanyID",
            "<cbc:CompanyID> </cbc:CompanyID>",
        ),
    ] {
        let nodes = source_xml::select(&doc, path);
        assert_eq!(nodes.len(), 1);
        let mut invalid = xml.clone();
        invalid.replace_range(nodes[0].range(), replacement);
        assert!(parse(&invalid).is_err(), "accepted malformed {path}");
    }
}

#[test]
fn imported_tax_totals_reject_missing_or_duplicate_owners() {
    let xml = builder()
        .line_item(line().build().unwrap())
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let totals = source_xml::select(&doc, "/ubl:Invoice/cac:TaxTotal");
    assert_eq!(totals.len(), 2);
    for total in totals {
        for replacement in [String::new(), xml[total.range()].repeat(2)] {
            // An absent accounting total is supported; the document breakdown is required.
            if replacement.is_empty() && !xml[total.range()].contains("TaxSubtotal") {
                continue;
            }
            let mut invalid = xml.clone();
            invalid.replace_range(total.range(), &replacement);
            let error = parse(&invalid).unwrap_err();
            assert!(error.to_string().contains("TaxTotal"), "{error}");
        }
    }
    for path in [
        "/ubl:Invoice/cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cbc:Percent",
        "/ubl:Invoice/cac:InvoiceLine/cac:Item/cac:ClassifiedTaxCategory/cbc:Percent",
    ] {
        let nodes = source_xml::select(&doc, path);
        assert_eq!(nodes.len(), 1);
        let mut invalid = xml.clone();
        invalid.replace_range(nodes[0].range(), "");
        assert!(parse(&invalid).is_err(), "accepted missing rate at {path}");
    }
}

#[test]
fn imported_price_discounts_obey_constructor_ranges() {
    let xml = builder()
        .line_item(
            line()
                .item_gross_price(d("120"))
                .item_price_discount(d("20"))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let invalid = xml
        .replace(
            "<cbc:Amount currencyID=\"SAR\">20</cbc:Amount>",
            "<cbc:Amount currencyID=\"SAR\">-1.00</cbc:Amount>",
        )
        .replace(
            "<cbc:BaseAmount currencyID=\"SAR\">120</cbc:BaseAmount>",
            "<cbc:BaseAmount currencyID=\"SAR\">99</cbc:BaseAmount>",
        );
    assert_ne!(xml, invalid);
    assert!(
        line()
            .item_gross_price(d("99"))
            .item_price_discount(d("-1"))
            .build()
            .is_err()
    );
    assert!(
        parse(&invalid).is_err(),
        "import accepted a negative price discount"
    );
    let inconsistent = xml.replace(
        "<cbc:BaseAmount currencyID=\"SAR\">120</cbc:BaseAmount>",
        "<cbc:BaseAmount currencyID=\"SAR\">121</cbc:BaseAmount>",
    );
    assert_ne!(xml, inconsistent);
    let error = parse(&inconsistent).unwrap_err();
    assert!(
        error.to_string().contains("gross minus discount"),
        "{error}"
    );
}

#[test]
fn out_of_scope_document_adjustments_preserve_absent_rates() {
    let xml = builder()
        .line_item(
            LineItem::new(
                "item",
                d("1"),
                "PCE",
                d("100"),
                d("0"),
                VatCategory::OutOfScope,
            )
            .unwrap(),
        )
        .charge(DocumentAdjustment::new(
            Adjustment::amount(d("10")),
            VatCategory::OutOfScope,
            d("0"),
        ))
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let source = xml.replace("<cbc:Percent>0</cbc:Percent>", "");
    assert_ne!(source, xml);
    validate_xml_invoice_from_str(&source, &Config::default()).unwrap();
    let imported = parse(&source).unwrap();
    assert!(imported.data().adjustments()[0].omitted_rate);
    assert_eq!(leaves(&source), leaves(&imported.to_xml().unwrap()));
}

#[test]
fn line_builder_requires_rates_except_out_of_scope() {
    for category in [
        VatCategory::Standard,
        VatCategory::Zero,
        VatCategory::Exempt,
        VatCategory::OutOfScope,
    ] {
        let result = LineItem::builder()
            .item_name("item")
            .invoiced_quantity(d("1"))
            .item_net_price(d("100"))
            .vat_category(category)
            .build();
        if category == VatCategory::OutOfScope {
            let xml = builder()
                .line_item(result.unwrap())
                .build()
                .unwrap()
                .to_xml()
                .unwrap();
            assert_eq!(
                leaves(&xml),
                leaves(&parse(&xml).unwrap().to_xml().unwrap())
            );
        } else {
            assert!(result.is_err(), "missing rate accepted for {category:?}");
        }
    }
}

#[test]
fn optional_line_inputs_reject_incomplete_and_inconsistent_prices() {
    for invalid in [
        LineItem::builder(),
        LineItem::builder().item_name("item"),
        LineItem::builder()
            .item_name("item")
            .invoiced_quantity(d("1")),
        LineItem::builder()
            .item_name("item")
            .invoiced_quantity(d("1"))
            .vat_category(VatCategory::OutOfScope),
        line().item_name(" "),
        line().id(" "),
        line().invoiced_quantity_unit_code(" "),
        line().item_price_base_quantity_unit_code(" "),
        line().item_price_base_quantity_unit_code("PCE"),
        line().item_price_base_quantity(d("0")),
        line().item_price_base_quantity(d("-1")),
        line().invoiced_quantity(d("-1")),
        line().item_net_price(d("-1")),
        line().vat_rate(d("100.01")),
        line().vat_rate(d("15.001")),
        line().vat_category(VatCategory::Exempt),
        line().item_gross_price(d("120")),
        line()
            .item_gross_price(d("120"))
            .item_price_discount(d("19")),
        line().item_price_discount(d("-1")),
        line().line_net_amount(d("3000.02")),
        line().line_vat_amount(d("-1")),
        line().line_vat_amount(d("1.001")),
        line().line_amount_with_vat(d("3450.01")),
        line().item_net_price(d("0")).line_net_amount(d("-0.01")),
    ] {
        assert!(invalid.build().is_err());
    }
    let gross_only = LineItem::builder()
        .item_name("item")
        .invoiced_quantity(d("1"))
        .vat_category(VatCategory::Standard)
        .vat_rate(d("15"))
        .item_gross_price(d("120"))
        .item_price_discount(d("20"))
        .build()
        .unwrap();
    assert_eq!(gross_only.unit_price(), d("100"));
    assert_eq!(
        line()
            .item_price_discount(d("20"))
            .build()
            .unwrap()
            .item_gross_price(),
        Some(d("120"))
    );
    for invalid in [
        Adjustment::amount(d("-1")),
        Adjustment::amount(d("1.001")),
        Adjustment::amount(d("1")).base_amount(d("100")),
        Adjustment::percentage(d("-1"), d("10")),
        Adjustment::percentage(d("100"), d("-1")),
        Adjustment::percentage(d("100"), d("10")).supplied_amount(d("9.99")),
    ] {
        assert!(line().allowance(invalid).build().is_err());
    }
}

#[test]
fn supplied_breakdowns_reject_ambiguous_ownership_and_invalid_amounts() {
    let group = VatBreakdown::new(VatCategory::Standard, d("15"), d("3000"), d("450")).unwrap();
    assert!(
        builder()
            .line_item(line().build().unwrap())
            .vat_breakdown(group.clone())
            .vat_breakdown(group.clone())
            .build()
            .is_err()
    );
    assert!(
        builder()
            .line_item(line().build().unwrap())
            .vat_breakdown(
                VatBreakdown::new(VatCategory::Standard, d("5"), d("3000"), d("150")).unwrap()
            )
            .build()
            .is_err()
    );
    let reason = TaxExemption {
        reason_code: None,
        reason: Some("supplied".into()),
    };
    assert!(
        builder()
            .line_item(line().build().unwrap())
            .vat_breakdown(group)
            .tax_exemption(VatCategory::Standard, d("15"), reason.clone())
            .build()
            .is_err()
    );
    assert!(
        builder()
            .line_item(line().build().unwrap())
            .tax_exemption(VatCategory::Standard, d("15"), reason.clone())
            .tax_exemption(VatCategory::Standard, d("15"), reason.clone())
            .build()
            .is_ok()
    );
    let mut invalid_group = serde_json::to_value(
        VatBreakdown::new(VatCategory::Standard, d("15"), d("3000"), d("450")).unwrap(),
    )
    .unwrap();
    invalid_group["tax_amount"] = serde_json::json!("-0.01");
    assert!(
        builder()
            .line_item(line().build().unwrap())
            .vat_breakdown(serde_json::from_value(invalid_group).unwrap())
            .build()
            .is_err()
    );
    let error = builder()
        .line_item(line().build().unwrap())
        .buyer_identifier(OtherId::new("id"))
        .build()
        .unwrap_err();
    if let InvoiceError::Validation(ref error) = error {
        let report = fatoora_core::invoice::validation::ValidationReport::from(error);
        assert!(format!("{report:?}").contains("buyer"));
    } else {
        panic!("expected buyer validation error");
    }
    let error: fatoora_core::Error = error.into();
    assert!(
        serde_json::to_string(&error.details_json())
            .unwrap()
            .contains("buyer")
    );
    let adjustment =
        DocumentAdjustment::new(Adjustment::amount(d("1")), VatCategory::Standard, d("15"))
            .reason("rebate")
            .reason_code("95")
            .exemption(reason);
    let invoice = builder()
        .line_item(line().build().unwrap())
        .allowance(adjustment)
        .build()
        .unwrap();
    assert_eq!(
        invoice.data().adjustments()[0].reason.as_deref(),
        Some("rebate")
    );
    assert_eq!(
        invoice.data().adjustments()[0].reason_code.as_deref(),
        Some("95")
    );
    assert_eq!(invoice.data().line_items()[0].item_name(), "سلعة & item");
    assert_eq!(invoice.data().line_items()[0].item_net_price(), d("100"));
    assert!(InvoiceTime::parse("1:02:03").is_err());
}

#[test]
fn deserialized_price_metadata_is_checked_before_invoice_construction() {
    let item = line()
        .item_gross_price(d("120"))
        .item_price_discount(d("20"))
        .build()
        .unwrap();
    for gross in [serde_json::json!("119"), serde_json::Value::Null] {
        let mut value = serde_json::to_value(&item).unwrap();
        value["gross_price"] = gross;
        let invalid: LineItem = serde_json::from_value(value).unwrap();
        assert!(builder().line_item(invalid).build().is_err());
    }
}

#[test]
fn omitting_line_tax_must_not_hide_nonzero_tax_or_subtotals() {
    assert!(line().tax_total_present(false).build().is_err());
    let subtotal = VatBreakdown::new(VatCategory::OutOfScope, d("0"), d("100"), d("0")).unwrap();
    assert!(
        line()
            .vat_category(VatCategory::OutOfScope)
            .vat_rate(d("0"))
            .tax_total_present(false)
            .tax_subtotal(subtotal)
            .build()
            .is_err()
    );
}

#[test]
fn deserialized_line_tax_presence_is_revalidated_with_line_indices() {
    let standard = line().build().unwrap();
    let outside = LineItem::builder()
        .item_name("outside")
        .invoiced_quantity(d("30"))
        .item_net_price(d("100"))
        .vat_category(VatCategory::OutOfScope)
        .tax_total_present(false)
        .build()
        .unwrap();
    let with_subtotal = outside.clone().tax_subtotal(
        VatBreakdown::new(VatCategory::OutOfScope, d("0"), d("3000"), d("0")).unwrap(),
    );
    for (source, field, value, expected) in [
        (
            standard.clone(),
            "omitted_vat_rate",
            serde_json::json!(true),
            "line_item_vat_rate",
        ),
        (
            standard,
            "omitted_tax_amount",
            serde_json::json!(true),
            "line_item_vat_amount",
        ),
        (
            with_subtotal,
            "omitted_tax_amount",
            serde_json::json!(true),
            "line_item_vat_amount",
        ),
        (
            outside.clone(),
            "omitted_gross_amount",
            serde_json::json!(false),
            "line_item_vat_amount",
        ),
        (
            outside.clone(),
            "vat_rate",
            serde_json::json!("15"),
            "line_item_vat_rate",
        ),
    ] {
        let mut encoded = serde_json::to_value(source).unwrap();
        encoded[field] = value;
        let decoded: LineItem = serde_json::from_value(encoded).unwrap();
        let error: fatoora_core::Error = builder().line_item(decoded).build().unwrap_err().into();
        let details: serde_json::Value = serde_json::from_str(&error.details_json()).unwrap();
        let issues = details["issues"].as_array().unwrap();
        assert!(
            issues
                .iter()
                .any(|issue| issue["field"] == expected && issue["line_item_index"] == 0),
            "{details}"
        );
    }
    let decoded: LineItem = serde_json::from_value(serde_json::to_value(outside).unwrap()).unwrap();
    let xml = builder()
        .line_item(decoded)
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    assert_eq!(
        leaves(&xml),
        leaves(&parse(&xml).unwrap().to_xml().unwrap())
    );
}
