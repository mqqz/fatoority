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
