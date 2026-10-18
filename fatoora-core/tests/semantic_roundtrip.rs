mod common;
use fatoora_core::{
    Decimal,
    config::Config,
    invoice::{
        validation::validate_xml_invoice_from_str,
        xml::parse::parse_finalized_invoice_xml as parse, *,
    },
};
fn d(s: &str) -> Decimal {
    s.parse().unwrap()
}
fn builder() -> InvoiceBuilder {
    let invoice = common::dummy_finalized_invoice();
    InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("roundtrip")
        .uuid(invoice.data().uuid())
        .issue_datetime("2024-01-01T12:30:00Z")
        .currency("SAR")
        .previous_invoice_hash(invoice.data().previous_invoice_hash())
        .invoice_counter(1)
        .seller(invoice.data().seller().clone())
        .payment_means_code("10")
        .vat_category(VatCategory::Standard)
}
fn line(category: VatCategory) -> LineItem {
    LineItem::new(
        "سلعة & <test>",
        d("2"),
        "PCE",
        d("10.1234"),
        if category == VatCategory::Standard {
            d("15")
        } else {
            d("0")
        },
        category,
    )
    .unwrap()
}
fn roundtrip(xml: &str) -> FinalizedInvoice {
    let first = parse(xml).unwrap();
    let regenerated = first.to_xml().unwrap();
    validate_xml_invoice_from_str(&regenerated, &Config::default()).unwrap();
    let second = parse(&regenerated).unwrap();
    assert_eq!(first, second);
    second
}
#[test]
fn buyers_preserve_identifiers_address_and_optional_fields() {
    let standard = common::complex_standard_invoice();
    let result = roundtrip(&standard.to_xml().unwrap());
    assert_eq!(result.data().buyer(), standard.data().buyer());
    for vat in [None, Some("310122393500003".into())] {
        for other in [None, Some(OtherId::with_scheme("1234567890", "NAT"))] {
            let mut address = standard.data().buyer().unwrap().address().clone();
            address.building_number.clear();
            address.postal_code.clear();
            address.additional_street = Some("شارع & آخر".into());
            let buyer = Buyer::new("مشتري".into(), address, vat.clone(), other).unwrap();
            let invoice = builder()
                .buyer(buyer.clone())
                .line_item(line(VatCategory::Standard))
                .build()
                .unwrap();
            assert_eq!(
                roundtrip(&invoice.to_xml().unwrap()).data().buyer(),
                Some(&buyer)
            );
        }
    }
    assert!(
        roundtrip(&common::dummy_finalized_invoice().to_xml().unwrap())
            .data()
            .buyer()
            .is_none()
    );
    let xml = standard.to_xml().unwrap();
    assert!(
        parse(&xml.replace(
            "<cbc:IdentificationCode>SA</cbc:IdentificationCode>",
            "<cbc:IdentificationCode>INVALID</cbc:IdentificationCode>"
        ))
        .is_err()
    );
    assert!(
        parse(&xml.replace(
            "<cbc:RegistrationName>Blue Desert Retail LLC</cbc:RegistrationName>",
            "<cbc:RegistrationName/> "
        ))
        .is_err()
    );
}
#[test]
fn delivery_dates_are_independent_and_invalid_dates_fail() {
    for actual in [None, Some(InvoiceDate::parse("2024-01-01").unwrap())] {
        for latest in [None, Some(InvoiceDate::parse("2024-01-02").unwrap())] {
            let delivery = Delivery {
                actual_date: actual.clone(),
                latest_date: latest,
            };
            let invoice = builder()
                .delivery(delivery.clone())
                .line_item(line(VatCategory::Standard))
                .build()
                .unwrap();
            let xml = invoice.to_xml().unwrap();
            assert_eq!(roundtrip(&xml).data().delivery(), Some(&delivery));
            if actual.is_some() {
                assert!(
                    parse(&xml.replace(
                        "<cbc:ActualDeliveryDate>2024-01-01",
                        "<cbc:ActualDeliveryDate>2024-02-30"
                    ))
                    .is_err()
                );
                assert!(
                    parse(&xml.replace(
                        "<cbc:ActualDeliveryDate>2024-01-01</cbc:ActualDeliveryDate>",
                        "<cbc:ActualDeliveryDate/>"
                    ))
                    .is_err()
                );
            }
        }
    }
}
#[test]
fn mixed_exemptions_and_repeated_adjustments_preserve_independent_metadata() {
    let mut b = builder();
    for (i, category) in [
        VatCategory::Standard,
        VatCategory::Exempt,
        VatCategory::Zero,
        VatCategory::OutOfScope,
    ]
    .into_iter()
    .enumerate()
    {
        let exemption = TaxExemption {
            reason_code: (i != 0).then(|| format!("VATEX-SA-{}", i)),
            reason: (i > 1).then(|| "إعفاء & <ضريبة>".into()),
        };
        let item = line(category)
            .with_id(format!("line-{i}"))
            .with_exemption(exemption.clone());
        let rate = item.vat_rate();
        b = b
            .line_item(item)
            .tax_exemption(category, rate, exemption.clone());
        for charge in [false, true] {
            b = b.adjustment(AllowanceCharge {
                tax_category_count: 1,
                charge,
                amount: d("1"),
                reason: Some(format!("reason-{i}-{charge}")),
                reason_code: Some("95".into()),
                category,
                rate,
                exemption: exemption.clone(),
            });
        }
    }
    let invoice = b.build().unwrap();
    let parsed = roundtrip(&invoice.to_xml().unwrap());
    assert_eq!(parsed.data().line_items(), invoice.data().line_items());
    assert_eq!(parsed.data().adjustments(), invoice.data().adjustments());
    assert_eq!(parsed.totals(), invoice.totals());
}
#[test]
fn description_without_exemption_code_survives() {
    let exemption = TaxExemption {
        reason_code: None,
        reason: Some("إعفاء".into()),
    };
    let invoice = builder()
        .line_item(line(VatCategory::Exempt).with_exemption(exemption.clone()))
        .tax_exemption(VatCategory::Exempt, d("0"), exemption.clone())
        .build()
        .unwrap();
    assert_eq!(
        roundtrip(&invoice.to_xml().unwrap())
            .totals()
            .vat_breakdown()[0]
            .exemption(),
        &exemption
    );
}
#[test]
fn foreign_currency_tax_total_is_not_replaced_by_document_tax() {
    let invoice = builder()
        .currency("USD")
        .tax_currency(CurrencyCode::parse("SAR").unwrap(), d("11.40"))
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap();
    assert_eq!(
        roundtrip(&invoice.to_xml().unwrap())
            .data()
            .tax_currency_amount(),
        Some(d("11.40"))
    );
}
#[test]
fn duplicate_supported_singletons_are_rejected() {
    let xml = common::complex_standard_invoice().to_xml().unwrap();
    for tag in [
        "cbc:Note",
        "cac:PartyIdentification",
        "cac:Delivery",
        "cac:PaymentMeans",
    ] {
        let duplicate = if tag == "cac:Delivery" {
            "<cac:Delivery/><cac:Delivery/>".into()
        } else {
            let start = xml.find(&format!("<{tag}")).unwrap();
            let end = xml[start..].find(&format!("</{tag}>")).unwrap() + start + tag.len() + 3;
            format!("{}{}", &xml[start..end], &xml[start..end])
        };
        let modified = xml.replace("</Invoice>", &format!("{duplicate}</Invoice>"));
        assert!(parse(&modified).is_err(), "{tag}");
    }
}

#[test]
fn regenerated_compliant_standard_fixture_passes_business_rules() {
    use fatoora_core::invoice::validation::{
        Severity, ZatcaStage, ZatcaStageStatus, ZatcaValidationOptions,
        validate_zatca_invoice_from_str,
    };
    for name in [
        "Standard_Invoice.xml",
        "Exempt Tax Invoice.xml",
        "Export invoice.xml",
    ] {
        let original = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/invoices/Standard/Invoice")
                .join(name),
        )
        .unwrap();
        let invoice = parse(&original).unwrap();
        let xml = invoice.to_xml().unwrap();
        let options = ZatcaValidationOptions {
            evaluated_at: Some("2026-10-18T12:00:00Z".parse().unwrap()),
            previous_invoice_hash: Some(invoice.data().previous_invoice_hash().into()),
        };
        let report = validate_zatca_invoice_from_str(&xml, &Config::default(), &options).unwrap();
        for stage in report
            .stages
            .iter()
            .filter(|s| matches!(s.stage, ZatcaStage::Xsd | ZatcaStage::Cen | ZatcaStage::Ksa))
        {
            assert_eq!(stage.status, ZatcaStageStatus::Completed);
            let errors: Vec<_> = stage
                .findings
                .iter()
                .filter(|f| f.finding.severity == Severity::Error)
                .collect();
            assert!(errors.is_empty(), "{name}: {errors:#?}");
        }
    }
}

#[test]
fn optional_language_and_payment_instructions_are_preserved() {
    let invoice = builder()
        .note(InvoiceNote::without_language("ملاحظة"))
        .payment_instruction("نقداً & فقط")
        .prepaid_amount(d("1.00"))
        .payable_rounding_amount(d("-0.01"))
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    assert!(!xml.contains("languageID"));
    let parsed = roundtrip(&xml);
    assert_eq!(parsed.data().note(), invoice.data().note());
    assert_eq!(
        parsed.data().payment_instruction(),
        invoice.data().payment_instruction()
    );
    assert_eq!(parsed.totals(), invoice.totals());
}

#[test]
fn previous_serde_models_default_new_optional_fields() {
    let invoice = common::dummy_finalized_invoice();
    let mut data = serde_json::to_value(invoice.data()).unwrap();
    for field in [
        "delivery",
        "adjustments",
        "payment_instruction",
        "tax_currency",
        "tax_currency_amount",
    ] {
        data.as_object_mut().unwrap().remove(field);
    }
    for line in data["line_items"].as_array_mut().unwrap() {
        for field in ["id", "exemption"] {
            line.as_object_mut().unwrap().remove(field);
        }
    }
    let parsed: InvoiceData = serde_json::from_value(data).unwrap();
    assert!(parsed.delivery().is_none());
    assert!(parsed.adjustments().is_empty());
    assert_eq!(parsed.line_items()[0].exemption(), &TaxExemption::default());
}

#[test]
fn delivery_order_is_reported_by_business_validation_without_import_repair() {
    use fatoora_core::invoice::validation::{
        ZatcaValidationOptions, validate_zatca_invoice_from_str,
    };
    let invoice = builder()
        .delivery(Delivery {
            actual_date: Some(InvoiceDate::parse("2024-02-02").unwrap()),
            latest_date: Some(InvoiceDate::parse("2024-02-01").unwrap()),
        })
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap();
    let parsed = roundtrip(&invoice.to_xml().unwrap());
    assert_eq!(parsed.data().delivery(), invoice.data().delivery());
    let report = validate_zatca_invoice_from_str(
        &parsed.to_xml().unwrap(),
        &Config::default(),
        &ZatcaValidationOptions::default(),
    )
    .unwrap();
    assert!(
        report
            .stages
            .iter()
            .flat_map(|s| &s.findings)
            .any(|f| f.finding.code == "BR-KSA-36")
    );
}

#[test]
fn prepayment_reference_and_subtotals_are_read_from_source() {
    let source = include_str!("fixtures/invoices/Standard/Invoice/Advance Payment adjustments.xml");
    let invoice = roundtrip(source);
    let line = &invoice.data().line_items()[1];
    let reference = &line.document_references()[0];
    assert_eq!(reference.id(), "46531");
    assert_eq!(
        reference.uuid(),
        Some("a79760f7-2f48-4da9-85a5-40459a147c80")
    );
    assert_eq!(reference.issue_date().unwrap().as_str(), "2022-08-15");
    assert_eq!(reference.issue_time().unwrap().as_str(), "12:28:17");
    assert_eq!(reference.document_type_code(), Some("386"));
    assert_eq!(line.tax_subtotals()[0].taxable_amount(), d("1000"));
    assert_eq!(line.tax_subtotals()[0].tax_amount(), d("150"));
    let xml = invoice.to_xml().unwrap();
    assert!(parse(&xml.replace("12:28:17", "25:28:17")).is_err());
}

#[test]
fn malformed_supplied_money_and_inconsistent_currency_are_not_dropped() {
    let xml = common::dummy_finalized_invoice().to_xml().unwrap();
    assert!(
        parse(&xml.replace(
            "<cbc:PrepaidAmount currencyID=\"SAR\">0.00</cbc:PrepaidAmount>",
            "<cbc:PrepaidAmount currencyID=\"SAR\"/> "
        ))
        .is_err()
    );
    assert!(
        parse(&xml.replace(
            "<cbc:PriceAmount currencyID=\"SAR\">",
            "<cbc:PriceAmount currencyID=\"USD\">"
        ))
        .is_err()
    );
}

#[test]
fn unsupported_profiles_schemes_and_duplicate_line_totals_fail_explicitly() {
    let xml = common::dummy_finalized_invoice().to_xml().unwrap();
    assert!(parse(&xml.replace("reporting:1.0", "other:1.0")).is_err());
    assert!(parse(&xml.replace(">VAT</cbc:ID>", ">GST</cbc:ID>")).is_err());
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let line_node = doc
        .descendants()
        .find(|n| n.tag_name().name() == "InvoiceLine")
        .unwrap();
    let total = line_node
        .children()
        .find(|n| n.tag_name().name() == "TaxTotal")
        .unwrap();
    let mut duplicate = xml.clone();
    duplicate.insert_str(total.range().end, &xml[total.range()]);
    assert!(parse(&duplicate).is_err());
    assert!(
        builder()
            .line_item(line(VatCategory::Standard))
            .tax_exemption(
                VatCategory::Exempt,
                d("0"),
                TaxExemption {
                    reason_code: Some("VATEX-SA-29".into()),
                    reason: None
                }
            )
            .build()
            .is_err()
    );
}

fn balance_adjustment(charge: bool, amount: &str, category: VatCategory) -> AllowanceCharge {
    AllowanceCharge {
        tax_category_count: 1,
        charge,
        amount: d(amount),
        reason: Some(if charge { "charge" } else { "allowance" }.into()),
        reason_code: None,
        category,
        rate: if category == VatCategory::Standard {
            d("15")
        } else {
            d("0")
        },
        exemption: TaxExemption::default(),
    }
}

#[test]
fn adjustment_balances_are_validated_after_all_entries() {
    let allowance = balance_adjustment(false, "25", VatCategory::Standard);
    let charge = balance_adjustment(true, "30", VatCategory::Standard);
    for adjustments in [[allowance.clone(), charge.clone()], [charge, allowance]] {
        let invoice = builder()
            .line_item(line(VatCategory::Standard))
            .adjustment(adjustments[0].clone())
            .adjustment(adjustments[1].clone())
            .build()
            .unwrap();
        assert_eq!(invoice.totals().taxable_amount(), d("25.25"));
        assert_eq!(invoice.totals().tax_amount(), d("3.79"));
        let parsed = roundtrip(&invoice.to_xml().unwrap());
        assert_eq!(parsed.data().adjustments(), adjustments);
        assert_eq!(parsed.totals(), invoice.totals());
    }
}

#[test]
fn adjustment_balances_in_import_do_not_depend_on_xml_order() {
    let invoice = builder()
        .line_item(line(VatCategory::Standard))
        .adjustment(balance_adjustment(true, "30", VatCategory::Standard))
        .adjustment(balance_adjustment(false, "25", VatCategory::Standard))
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let ranges: Vec<_> = doc
        .root_element()
        .children()
        .filter(|node| node.tag_name().name() == "AllowanceCharge")
        .map(|node| node.range())
        .collect();
    assert_eq!(ranges.len(), 2);
    let mut allowance_first = xml.clone();
    allowance_first.replace_range(ranges[1].clone(), &xml[ranges[0].clone()]);
    allowance_first.replace_range(ranges[0].clone(), &xml[ranges[1].clone()]);
    let parsed = roundtrip(&allowance_first);
    assert!(!parsed.data().adjustments()[0].charge);
    assert_eq!(parsed.totals(), invoice.totals());
}

#[test]
fn adjustment_balances_still_reject_a_negative_final_vat_group() {
    // A charge in a different group cannot offset this group's negative balance,
    // even when the document's combined taxable balance is positive.
    for charge in [
        balance_adjustment(true, "4", VatCategory::Standard),
        balance_adjustment(true, "30", VatCategory::Zero),
    ] {
        let allowance = balance_adjustment(false, "25", VatCategory::Standard);
        for adjustments in [[allowance.clone(), charge.clone()], [charge, allowance]] {
            let error = builder()
                .line_item(line(VatCategory::Standard))
                .adjustment(adjustments[0].clone())
                .adjustment(adjustments[1].clone())
                .build()
                .unwrap_err();
            let InvoiceError::Validation(error) = error else {
                panic!("unexpected error: {error}");
            };
            assert!(
                error
                    .issues()
                    .iter()
                    .any(|issue| issue.field() == InvoiceField::InvoiceLevelDiscount
                        && issue.kind() == ValidationKind::OutOfRange)
            );
        }
    }
}

#[test]
fn builder_rejects_invalid_adjustments_and_inconsistent_accounting_metadata() {
    let adjustment = AllowanceCharge {
        tax_category_count: 1,
        charge: true,
        amount: d("1"),
        reason: None,
        reason_code: None,
        category: VatCategory::Standard,
        rate: d("15"),
        exemption: TaxExemption::default(),
    };
    for legacy in [
        builder().invoice_level_charge(d("1")),
        builder().invoice_level_discount(d("1")),
        builder().allowance_reason("legacy"),
    ] {
        assert!(
            legacy
                .line_item(line(VatCategory::Standard))
                .adjustment(adjustment.clone())
                .build()
                .is_err()
        );
    }
    for (amount, rate, category, count) in [
        ("-1", "15", VatCategory::Standard, 1),
        ("0.001", "15", VatCategory::Standard, 1),
        ("1", "-1", VatCategory::Standard, 1),
        ("1", "101", VatCategory::Standard, 1),
        ("1", "15.001", VatCategory::Standard, 1),
        ("1", "15", VatCategory::Exempt, 1),
        ("1", "15", VatCategory::Standard, 0),
    ] {
        let invalid = AllowanceCharge {
            amount: d(amount),
            rate: d(rate),
            category,
            tax_category_count: count,
            ..adjustment.clone()
        };
        assert!(
            builder()
                .line_item(line(VatCategory::Standard))
                .adjustment(invalid)
                .build()
                .is_err(),
            "{amount}/{rate}/{category:?}/{count}"
        );
    }
    for invalid in [
        builder().prepaid_amount(d("-1")),
        builder().prepaid_amount(d("0.001")),
        builder().payable_rounding_amount(d("0.001")),
        builder().tax_currency(CurrencyCode::parse("USD").unwrap(), d("-1")),
        builder().tax_currency(CurrencyCode::parse("USD").unwrap(), d("0.001")),
        builder().tax_currency(CurrencyCode::parse("SAR").unwrap(), d("999")),
        builder().tax_exemption(VatCategory::Exempt, d("0"), TaxExemption::default()),
    ] {
        assert!(
            invalid
                .line_item(line(VatCategory::Standard))
                .build()
                .is_err()
        );
    }
}

#[test]
fn line_prepayment_subtotals_preserve_every_vat_category_and_reject_invalid_values() {
    for category in [
        VatCategory::Standard,
        VatCategory::Exempt,
        VatCategory::Zero,
        VatCategory::OutOfScope,
    ] {
        let subtotal = VatBreakdown::new(category, d("0"), d("5"), d("0")).unwrap();
        let item = line(category)
            .tax_subtotal(subtotal.clone())
            .document_reference(OriginalInvoiceRef::new("PRE-1"));
        let invoice = builder().line_item(item).build().unwrap();
        let xml = invoice.to_xml().unwrap();
        let restored = roundtrip(&xml);
        assert_eq!(restored.data().line_items()[0].tax_subtotals(), &[subtotal]);
        let start = xml.find("<cac:InvoiceLine>").unwrap();
        let (head, tail) = xml.split_at(start);
        let doc = roxmltree::Document::parse(&xml).unwrap();
        let code = doc
            .descendants()
            .find(|n| n.is_element() && n.tag_name().name() == "InvoiceLine")
            .unwrap()
            .descendants()
            .find(|n| n.is_element() && n.tag_name().name() == "TaxCategory")
            .unwrap()
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "ID")
            .unwrap();
        let mut bad_code = xml.clone();
        bad_code.replace_range(code.range(), "<cbc:ID>INVALID</cbc:ID>");
        assert!(
            parse(&bad_code)
                .unwrap_err()
                .to_string()
                .contains("VatCategory")
        );
        let bad_amount =
            tail.replacen(">5.00</cbc:TaxableAmount>", ">-5.00</cbc:TaxableAmount>", 1);
        assert_ne!(bad_amount, tail);
        assert!(parse(&format!("{head}{bad_amount}")).is_err());
    }
}

#[test]
fn serialized_adjustments_default_the_legacy_tax_category_count() {
    let adjustment = AllowanceCharge {
        tax_category_count: 2,
        charge: true,
        amount: d("1"),
        reason: None,
        reason_code: None,
        category: VatCategory::Standard,
        rate: d("15"),
        exemption: TaxExemption::default(),
    };
    let mut serialized = serde_json::to_value(&adjustment).unwrap();
    serialized
        .as_object_mut()
        .unwrap()
        .remove("tax_category_count");
    let decoded: AllowanceCharge = serde_json::from_value(serialized).unwrap();
    assert_eq!(decoded.tax_category_count, 1);
}

#[test]
fn buyer_without_identification_scheme_and_empty_optional_address_roundtrips() {
    let original = common::complex_standard_invoice();
    let mut address = original.data().buyer().unwrap().address().clone();
    address.street.clear();
    address.city.clear();
    let buyer = Buyer::new(
        "Buyer".into(),
        address,
        None,
        Some(OtherId::new("customer-id")),
    )
    .unwrap();
    let invoice = builder()
        .buyer(buyer.clone())
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap();
    assert_eq!(
        roundtrip(&invoice.to_xml().unwrap()).data().buyer(),
        Some(&buyer)
    );
}

#[test]
fn foreign_tax_totals_require_one_valid_accounting_amount() {
    let invoice = builder()
        .currency("USD")
        .tax_currency(CurrencyCode::parse("SAR").unwrap(), d("11.40"))
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let accounting = doc
        .root_element()
        .children()
        .find(|node| {
            node.is_element()
                && node.tag_name().name() == "TaxTotal"
                && node
                    .children()
                    .any(|n| n.attribute("currencyID") == Some("SAR"))
        })
        .unwrap();
    let original = &xml[accounting.range()];
    for replacement in [
        String::new(),
        format!("{original}{original}"),
        original.replace("</cac:TaxTotal>", "<cac:TaxSubtotal/></cac:TaxTotal>"),
    ] {
        let modified = xml.replace(original, &replacement);
        assert!(
            parse(&modified)
                .unwrap_err()
                .to_string()
                .contains("TaxCurrencyCode")
        );
    }
    for amount in ["-1.00", "0.001"] {
        let modified = xml.replace(
            ">11.40</cbc:TaxAmount>",
            &format!(">{amount}</cbc:TaxAmount>"),
        );
        assert!(
            parse(&modified)
                .unwrap_err()
                .to_string()
                .contains("TaxCurrencyAmount")
        );
    }
    let domestic = builder()
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    let without_tax_currency =
        domestic.replace("<cbc:TaxCurrencyCode>SAR</cbc:TaxCurrencyCode>", "");
    assert_ne!(without_tax_currency, domestic);
    assert_eq!(
        parse(&without_tax_currency).unwrap(),
        parse(&domestic).unwrap()
    );
}

#[test]
fn supplied_blank_line_ids_are_rejected_without_replacing_them() {
    for id in ["", " ", "\t\r\n", "\u{2003}"] {
        let result = builder()
            .line_item(line(VatCategory::Standard))
            .line_item(line(VatCategory::Standard).with_id(id))
            .build();
        let InvoiceError::Validation(error) = result.expect_err("blank line ID") else {
            panic!("expected field validation error");
        };
        assert_eq!(error.issues().len(), 1);
        let issue = &error.issues()[0];
        assert_eq!(issue.field(), InvoiceField::LineItemId);
        assert_eq!(issue.kind(), ValidationKind::Empty);
        assert_eq!(issue.line_item_index(), Some(1));
        let report = validation::ValidationReport::from(&error);
        assert_eq!(report.issues[0].code, "FIELD_EMPTY");
        assert_eq!(
            report.issues[0].location,
            Some(validation::ValidationLocation::Field(
                "line_items[1].id".into()
            ))
        );
        let public_error: fatoora_core::Error = InvoiceError::Validation(error).into();
        let details: serde_json::Value =
            serde_json::from_str(&public_error.details_json()).unwrap();
        assert_eq!(details["issues"][0]["field"], "line_item_id");
        assert_eq!(details["issues"][0]["line_item_index"], 1);
    }
}

#[test]
fn absent_line_ids_are_generated_and_supplied_ids_are_preserved() {
    let invoice = builder()
        .line_item(line(VatCategory::Standard))
        .line_item(line(VatCategory::Standard).with_id("CUSTOM-42"))
        .line_item(line(VatCategory::Standard))
        .build()
        .unwrap();
    let restored = roundtrip(&invoice.to_xml().unwrap());
    let ids: Vec<_> = restored
        .data()
        .line_items()
        .iter()
        .map(LineItem::id)
        .collect();
    assert_eq!(ids, [Some("1"), Some("CUSTOM-42"), Some("3")]);
}

#[test]
fn document_adjustments_do_not_replace_the_line_derived_vat_category() {
    for (line_category, adjustment_category) in [
        (VatCategory::Standard, VatCategory::Exempt),
        (VatCategory::Exempt, VatCategory::Standard),
        (VatCategory::Zero, VatCategory::OutOfScope),
    ] {
        for charge in [false, true] {
            let mut b = builder()
                .vat_category(line_category)
                .line_item(line(line_category));
            // An allowance needs a taxable balance in its own category. A charge
            // may introduce a category with no corresponding invoice line.
            if !charge {
                b = b.line_item(line(adjustment_category));
            }
            let invoice = b
                .adjustment(AllowanceCharge {
                    tax_category_count: 1,
                    charge,
                    amount: d("1"),
                    reason: Some("Adjustment".into()),
                    reason_code: None,
                    category: adjustment_category,
                    rate: if adjustment_category == VatCategory::Standard {
                        d("15")
                    } else {
                        d("0")
                    },
                    exemption: TaxExemption::default(),
                })
                .build()
                .unwrap();
            let restored = roundtrip(&invoice.to_xml().unwrap());
            assert_eq!(restored.data().vat_category(), line_category);
            assert_eq!(
                restored.data().adjustments()[0].category,
                adjustment_category
            );
            assert_eq!(restored, invoice);
        }
    }
}

#[test]
fn supplied_blank_line_reference_ids_are_rejected() {
    for id in ["", " ", "\t\r\n", "\u{2003}"] {
        let result = builder()
            .line_item(line(VatCategory::Standard))
            .line_item(
                line(VatCategory::Standard)
                    .document_reference(OriginalInvoiceRef::new("PRE-1"))
                    .document_reference(OriginalInvoiceRef::new(id)),
            )
            .build();
        let InvoiceError::Validation(error) = result.expect_err("blank reference ID") else {
            panic!("expected field validation error");
        };
        assert_eq!(error.issues().len(), 1);
        let issue = &error.issues()[0];
        assert_eq!(issue.field(), InvoiceField::LineItemDocumentReferenceId);
        assert_eq!(issue.kind(), ValidationKind::Empty);
        assert_eq!(issue.line_item_index(), Some(1));
        let report = validation::ValidationReport::from(&error);
        assert_eq!(
            report.issues[0].location,
            Some(validation::ValidationLocation::Field(
                "line_items[1].document_references.id".into()
            ))
        );
        let public_error: fatoora_core::Error = InvoiceError::Validation(error).into();
        let details: serde_json::Value =
            serde_json::from_str(&public_error.details_json()).unwrap();
        assert_eq!(
            details["issues"][0]["field"],
            "line_item_document_reference_id"
        );
    }
}

#[test]
fn reference_issue_times_accept_surrounding_whitespace_but_reject_invalid_values() {
    let billing = common::credit_note_standard_invoice()
        .to_xml()
        .unwrap()
        .replace(
            "</cac:InvoiceDocumentReference>",
            "<cbc:IssueTime>10:20:30</cbc:IssueTime></cac:InvoiceDocumentReference>",
        );
    let line_xml = builder()
        .line_item(
            line(VatCategory::Standard).document_reference(
                OriginalInvoiceRef::new("PRE-1")
                    .with_issue_time(InvoiceTime::parse("10:20:30").unwrap()),
            ),
        )
        .build()
        .unwrap()
        .to_xml()
        .unwrap();
    for xml in [billing, line_xml] {
        let expected = parse(&xml).unwrap();
        let padded = xml.replace(
            "<cbc:IssueTime>10:20:30</cbc:IssueTime>",
            "<cbc:IssueTime> \t10:20:30\r\n </cbc:IssueTime>",
        );
        assert_ne!(padded, xml);
        validate_xml_invoice_from_str(&padded, &Config::default()).unwrap();
        assert_eq!(roundtrip(&padded), expected);
        for invalid in ["", " \t\r\n ", "25:00:00", "not-a-time"] {
            let modified = xml.replace(
                "<cbc:IssueTime>10:20:30</cbc:IssueTime>",
                &format!("<cbc:IssueTime>{invalid}</cbc:IssueTime>"),
            );
            assert!(parse(&modified).is_err(), "{invalid:?}");
        }
    }
}

#[test]
fn repeated_adjustment_classifications_compare_normalized_scalars() {
    let adjustment = AllowanceCharge {
        tax_category_count: 2,
        ..balance_adjustment(true, "1", VatCategory::Standard)
    };
    let invoice = builder()
        .line_item(line(VatCategory::Standard))
        .adjustment(adjustment)
        .build()
        .unwrap();
    let xml = invoice.to_xml().unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let category = doc
        .descendants()
        .find(|n| {
            n.has_tag_name((
                "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
                "AllowanceCharge",
            ))
        })
        .unwrap()
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "TaxCategory")
        .nth(1)
        .unwrap();
    let original = &xml[category.range()];
    for rate in [" 15 ", "15.00", "\t15.0\n"] {
        let replacement = original.replace(">S</cbc:ID>", "> S </cbc:ID>").replace(
            "<cbc:Percent>15</cbc:Percent>",
            &format!("<cbc:Percent>{rate}</cbc:Percent>"),
        );
        assert_ne!(replacement, original);
        let mut modified = xml.clone();
        modified.replace_range(category.range(), &replacement);
        validate_xml_invoice_from_str(&modified, &Config::default()).unwrap();
        assert_eq!(roundtrip(&modified), invoice);
    }
    for (field, value) in [
        ("ID", "E"),
        ("Percent", "14"),
        ("Percent", "invalid"),
        ("Percent", ""),
        ("ID", ""),
    ] {
        let node = category
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == field)
            .unwrap();
        let mut modified = xml.clone();
        modified.replace_range(node.range(), &format!("<cbc:{field}>{value}</cbc:{field}>"));
        assert!(parse(&modified).is_err(), "{field}: {value}");
    }
    for field in ["TaxExemptionReasonCode", "TaxExemptionReason"] {
        let mut modified = xml.clone();
        modified.replace_range(
            category.range(),
            &original.replace(
                "<cac:TaxScheme>",
                &format!("<cbc:{field}> reason </cbc:{field}><cac:TaxScheme>"),
            ),
        );
        assert!(parse(&modified).is_err(), "{field}");
    }
}

#[test]
fn currency_normalization_is_consistent_across_tax_totals() {
    for currency in ["SAR", "USD"] {
        let mut source = builder()
            .currency(currency)
            .line_item(line(VatCategory::Standard));
        if currency == "USD" {
            source = source.tax_currency(CurrencyCode::parse("SAR").unwrap(), d("11.40"));
        }
        let invoice = source.build().unwrap();
        let xml = invoice.to_xml().unwrap();
        for modified in [
            xml.replace("SAR", "sar").replace("USD", "usd"),
            xml.replace("SAR", " SaR ").replace("USD", " uSd "),
            xml.replace("currencyID=\"SAR\"", "currencyID=\"sar\"")
                .replace("currencyID=\"USD\"", "currencyID=\"usd\""),
        ] {
            assert_eq!(roundtrip(&modified), invoice);
            let wrong_tax = modified.replacen(">3.04</cbc:TaxAmount>", ">3.05</cbc:TaxAmount>", 1);
            assert_ne!(wrong_tax, modified);
            assert!(parse(&wrong_tax).is_err());
            let missing_currency = modified.replacen("currencyID=", "missingCurrency=", 1);
            assert!(parse(&missing_currency).is_err());
        }
    }
}

#[test]
fn credit_debit_reasons_cannot_conflict_with_payment_instructions() {
    for credit in [true, false] {
        for reason in ["return & <تصحيح>", "", " \t "] {
            let make = |id: &str| {
                let original = OriginalInvoiceRef::new(id);
                let kind = if credit {
                    InvoiceType::CreditNote(InvoiceSubType::Simplified, original, reason.into())
                } else {
                    InvoiceType::DebitNote(InvoiceSubType::Simplified, original, reason.into())
                };
                let source = common::dummy_finalized_invoice();
                InvoiceBuilder::new(kind)
                    .id("note")
                    .uuid(source.data().uuid())
                    .issue_datetime("2024-01-01T12:30:00Z")
                    .currency("SAR")
                    .previous_invoice_hash(source.data().previous_invoice_hash())
                    .invoice_counter(1)
                    .seller(source.data().seller().clone())
                    .payment_means_code("10")
                    .vat_category(VatCategory::Standard)
                    .line_item(line(VatCategory::Standard))
                    .note(InvoiceNote::without_language("unrelated note"))
            };
            for explicit in [false, true] {
                let b = make("ORIGINAL");
                let invoice = if explicit {
                    b.payment_instruction(reason)
                } else {
                    b
                }
                .build()
                .unwrap();
                let parsed = roundtrip(&invoice.to_xml().unwrap());
                assert_eq!(parsed.data().invoice_type(), invoice.data().invoice_type());
                assert_eq!(parsed.data().payment_instruction(), Some(reason));
            }
            for id in ["", " ", "\t\n", "\u{2003}"] {
                let InvoiceError::Validation(error) = make(id).build().unwrap_err() else {
                    panic!("unexpected error")
                };
                assert_eq!(error.issues()[0].field(), InvoiceField::OriginalInvoiceId);
                let report = validation::ValidationReport::from(&error);
                assert_eq!(
                    report.issues[0].location,
                    Some(validation::ValidationLocation::Field(
                        "original_invoice_id".into()
                    ))
                );
                let public: fatoora_core::Error = InvoiceError::Validation(error).into();
                assert!(public.details_json().contains("original_invoice_id"));
            }
            let error = make("ORIGINAL")
                .payment_instruction("different")
                .build()
                .unwrap_err();
            let InvoiceError::Validation(error) = error else {
                panic!("unexpected error")
            };
            assert_eq!(error.issues()[0].field(), InvoiceField::PaymentInstruction);
            assert_eq!(error.issues()[0].kind(), ValidationKind::Mismatch);
            let report = validation::ValidationReport::from(&error);
            assert_eq!(
                report.issues[0].location,
                Some(validation::ValidationLocation::Field(
                    "payment_instruction".into()
                ))
            );
            let public: fatoora_core::Error = InvoiceError::Validation(error).into();
            assert!(public.details_json().contains("payment_instruction"));
        }
    }
}
