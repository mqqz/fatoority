mod support;
use fatoora_ffi::common::ffi::BindingError;
use fatoora_ffi::invoice::ffi::{Address, FinalizedInvoice, InvoiceBuilder};
use support::written;

fn ok<T>(result: Result<T, Box<BindingError>>) -> T {
    result.unwrap_or_else(|e| {
        panic!(
            "binding error {}: {}",
            e.code(),
            written(|out| e.message(out))
        )
    })
}

#[test]
fn exact_decimals_and_consumption() {
    let mut b = builder();
    ok(b.add_line_item(b"Item", b"3", b"PCE", b"0.3333", b"15", 1));
    let invoice = ok(b.build());
    let xml = written(|out| ok(invoice.xml(out)));
    assert!(xml.contains(">0.3333</cbc:PriceAmount>"));
    assert!(xml.contains(">1.15</cbc:TaxInclusiveAmount>"));
    let parsed = ok(FinalizedInvoice::from_xml(xml.as_bytes()));
    assert_eq!(
        written(|out| ok(ok(parsed.totals()).tax_inclusive(out))),
        "1.15"
    );
    let line = ok(ok(parsed.data()).line_item(0));
    assert_eq!(written(|out| ok(line.unit_price(out))), "0.3333");
    assert_eq!(written(|out| ok(line.quantity(out))), "3");
    assert_eq!(b.build().err().unwrap().code(), 1);
    assert!(b.set_id(b"reuse").is_err());
}

#[test]
fn rejected_inputs_preserve_builder_and_structured_error() {
    let mut b = builder();
    let error = b
        .add_line_item(b"Item", b"1", b"PCE", b"invalid", b"15", 1)
        .err()
        .unwrap();
    assert_eq!(error.code(), 1);
    let details: serde_json::Value =
        serde_json::from_str(&written(|out| error.details_json(out))).unwrap();
    assert!(details["type"].is_string());
    for id in [&b"bad\0id"[..], &[0xff][..]] {
        assert_eq!(b.set_id(id).err().unwrap().code(), 1);
    }
    assert!(b.set_vat_category(255).is_err());
    assert!(b.flags(0x80).is_err());
    assert!(
        b.add_line_item(b"Item", b"1", b"PCE", b"1", b"15", 255)
            .is_err()
    );
    ok(b.add_line_item(b"Item", b"1", b"PCE", b"1.005", b"15", 1));
    let invoice = ok(b.build());
    let data = ok(invoice.data());
    assert_eq!(written(|out| ok(data.id(out))), "INV-1");
    assert_eq!(data.line_items_len(), 1);
    assert!(data.line_item(1).is_err());
    assert!(data.line_item(usize::MAX).is_err());
}

#[test]
fn failed_build_also_consumes_builder() {
    let mut b = builder();
    assert!(b.build().is_err());
    assert_eq!(b.build().err().unwrap().code(), 1);
    assert!(b.set_id(b"again").is_err());
}

#[test]
fn enum_values_and_credit_note_inputs_are_checked() {
    assert!(InvoiceBuilder::new(255, 1, None, None, None, None).is_err());
    assert!(InvoiceBuilder::new(0, 255, None, None, None, None).is_err());
    assert!(InvoiceBuilder::new(2, 1, None, None, None, None).is_err());
    assert!(InvoiceBuilder::new(3, 1, Some(b"INV-0"), None, None, None).is_err());
    assert!(
        InvoiceBuilder::new(
            2,
            1,
            Some(b"INV-0"),
            None,
            Some(b"not-a-date"),
            Some(b"Correction")
        )
        .is_err()
    );
    assert!(
        InvoiceBuilder::new(
            2,
            1,
            Some(b"INV-0"),
            None,
            Some(b"2023-11-13"),
            Some(b"Correction")
        )
        .is_ok()
    );
}

#[test]
fn child_snapshots_survive_parent_drop() {
    let mut b = builder();
    ok(b.add_line_item(b"Item", b"1", b"PCE", b"100", b"15", 1));
    let invoice = ok(b.build());
    let data = ok(invoice.data());
    let line = ok(data.line_item(0));
    let seller = ok(data.seller());
    let address = ok(seller.address());
    drop(invoice);
    drop(data);
    drop(seller);
    assert_eq!(written(|out| ok(line.description(out))), "Item");
    assert_eq!(written(|out| ok(address.city(out))), "Riyadh");
}

fn builder() -> Box<InvoiceBuilder> {
    configured(ok(InvoiceBuilder::new(0, 1, None, None, None, None)))
}

fn configured(mut b: Box<InvoiceBuilder>) -> Box<InvoiceBuilder> {
    ok(b.set_id(b"INV-1"));
    ok(b.set_uuid(b"8e6000cf-1a98-4174-b3e7-b5d5954bc10d"));
    ok(b.set_issue_datetime(b"2024-01-01T12:30:00Z"));
    ok(b.set_previous_invoice_hash(b"hash"));
    ok(b.set_invoice_counter(1));
    ok(b.set_currency(b"SAR"));
    ok(b.set_payment_means_code(b"10"));
    ok(b.set_vat_category(1));
    let address = ok(Address::new(
        b"SAU",
        b"Riyadh",
        b"King Fahd",
        b"1234",
        b"12222",
        None,
        None,
        Some(b"Olaya"),
    ));
    ok(b.set_seller(
        b"Seller",
        &address,
        b"399999999900003",
        Some(b"7003339333"),
        Some(b"CRN"),
    ));
    b
}

#[test]
fn rejected_setters_leave_previous_values_and_totals_intact() {
    let mut b = builder();
    ok(b.set_note(b"ar", "خصم & خدمة".as_bytes()));
    ok(b.set_allowance(b"Discount", b"10"));
    ok(b.invoice_level_charge(b"5"));
    assert!(b.set_issue_datetime(b"2024-02-30T12:00:00Z").is_err());
    assert!(b.set_currency(b"NOT-CURRENCY").is_err());
    assert!(b.set_note(b"ar", b"bad\0note").is_err());
    assert!(b.set_allowance(b"overwrite", b"NaN").is_err());
    assert!(b.invoice_level_charge(b"Infinity").is_err());
    assert!(b.invoice_level_discount(b"invalid").is_err());
    ok(b.add_line_item(b"Item", b"2", b"PCE", b"50", b"15", 1));
    let invoice = ok(b.build());
    let data = ok(invoice.data());
    assert_eq!(
        written(|out| ok(data.issue_datetime(out))),
        "2024-01-01T12:30:00Z"
    );
    assert_eq!(written(|out| ok(data.currency(out))), "SAR");
    assert_eq!(
        written(|out| ok(ok(data.allowance_reason()).unwrap().value(out))),
        "Discount"
    );
    let note = ok(data.note()).unwrap();
    assert_eq!(written(|out| ok(note.language(out))), "ar");
    assert_eq!(written(|out| ok(note.text(out))), "خصم & خدمة");
    let totals = ok(invoice.totals());
    // Independent arithmetic: (2 * 50 - 10 + 5) * 1.15 = 109.25.
    assert_eq!(written(|out| ok(totals.line_extension(out))), "100");
    assert_eq!(written(|out| ok(totals.allowance_total(out))), "10");
    assert_eq!(written(|out| ok(totals.charge_total(out))), "5");
    assert_eq!(written(|out| ok(totals.taxable_amount(out))), "95");
    assert_eq!(written(|out| ok(totals.tax_amount(out))), "14.25");
    assert_eq!(written(|out| ok(totals.payable_amount(out))), "109.25");
    let xml = written(|out| ok(invoice.xml(out)));
    assert!(xml.contains("خصم &amp; خدمة"));
    let parsed = ok(FinalizedInvoice::from_xml(xml.as_bytes()));
    assert_eq!(
        written(|out| ok(ok(ok(parsed.data()).note()).unwrap().text(out))),
        "خصم & خدمة"
    );
}

#[test]
fn buyer_optional_fields_and_owned_snapshots_preserve_absent_vs_empty() {
    for vat in [None, Some(&b"399999999900003"[..])] {
        let mut b = builder();
        let address = ok(Address::new(
            b"SAU",
            b"Riyadh",
            b"Street",
            b"1234",
            b"12222",
            Some(b""),
            None,
            Some("حي".as_bytes()),
        ));
        ok(b.set_buyer("مشتري".as_bytes(), &address, vat, Some(b"buyer-id"), None));
        // A failed replacement must leave the existing buyer intact.
        assert!(
            b.set_buyer(b"replacement", &address, Some(b"invalid\0vat"), None, None)
                .is_err()
        );
        drop(address);
        ok(b.add_line_item(b"Item", b"1", b"PCE", b"100", b"15", 1));
        let invoice = ok(b.build());
        let data = ok(invoice.data());
        let buyer = ok(data.buyer()).unwrap();
        let address = ok(buyer.address());
        let id = ok(buyer.other_id()).unwrap();
        let vat_id = ok(buyer.vat_id());
        assert_eq!(written(|out| ok(buyer.name(out))), "مشتري");
        drop(invoice);
        drop(data);
        drop(buyer);
        assert_eq!(vat_id.is_some(), vat.is_some());
        if let Some(vat_id) = vat_id {
            assert_eq!(written(|out| ok(vat_id.value(out))), "399999999900003");
        }
        assert_eq!(written(|out| ok(id.value(out))), "buyer-id");
        assert!(ok(id.scheme()).is_none());
        assert_eq!(
            written(|out| ok(ok(address.additional_street()).unwrap().value(out))),
            ""
        );
        assert!(ok(address.additional_number()).is_none());
        assert_eq!(
            written(|out| ok(ok(address.district()).unwrap().value(out))),
            "حي"
        );
    }
}

#[test]
fn vat_category_mapping_preserves_zero_tax_categories() {
    for (category, xml_code) in [(0, "E"), (1, "S"), (2, "Z"), (3, "O")] {
        let mut b = builder();
        ok(b.set_vat_category(category));
        ok(b.add_line_item(
            b"Item",
            b"1",
            b"PCE",
            b"100",
            if category == 1 { b"15" } else { b"0" },
            category,
        ));
        let invoice = ok(b.build());
        let data = ok(invoice.data());
        assert_eq!(data.vat_category(), category);
        assert_eq!(ok(data.line_item(0)).vat_category(), category);
        assert_eq!(
            written(|out| ok(ok(invoice.totals()).tax_amount(out))),
            if category == 1 { "15" } else { "0" }
        );
        let xml = written(|out| ok(invoice.xml(out)));
        assert!(xml.contains(&format!(">{xml_code}</cbc:ID>")));
        assert!(ok(data.buyer()).is_none());
        assert!(ok(data.note()).is_none());
        assert!(ok(data.original_invoice_ref()).is_none());
        assert!(ok(data.original_invoice_reason()).is_none());
    }
}

#[test]
fn credit_and_debit_notes_keep_original_reference_after_parent_drop() {
    for kind in [2, 3] {
        for optional in [false, true] {
            let mut b = configured(ok(InvoiceBuilder::new(
                kind,
                1,
                Some(b"INV-0"),
                optional.then_some(b"old-uuid"),
                optional.then_some(b"2023-11-13"),
                Some("تصحيح".as_bytes()),
            )));
            ok(b.add_line_item(b"Item", b"1", b"PCE", b"100", b"15", 1));
            let invoice = ok(b.build());
            let data = ok(invoice.data());
            assert_eq!(data.invoice_type_kind(), kind);
            assert_eq!(data.invoice_sub_type(), 1);
            let reference = ok(data.original_invoice_ref()).unwrap();
            let reason = ok(data.original_invoice_reason()).unwrap();
            drop(invoice);
            drop(data);
            assert_eq!(written(|out| ok(reference.id(out))), "INV-0");
            assert_eq!(written(|out| ok(reason.value(out))), "تصحيح");
            let uuid = ok(reference.uuid());
            let date = ok(reference.issue_date());
            assert_eq!(uuid.is_some(), optional);
            assert_eq!(date.is_some(), optional);
            if optional {
                assert_eq!(written(|out| ok(uuid.unwrap().value(out))), "old-uuid");
                assert_eq!(written(|out| ok(date.unwrap().value(out))), "2023-11-13");
            }
        }
    }
}

#[test]
fn prepayment_handoff_preserves_transaction_identity_and_amounts() {
    for subtype in [0, 1] {
        let mut b = configured(ok(InvoiceBuilder::new(1, subtype, None, None, None, None)));
        ok(b.flags(0b00101));
        ok(b.set_invoice_counter(u64::MAX));
        ok(b.invoice_level_discount(b"10"));
        ok(b.allowance_reason(b"Loyalty"));
        ok(b.invoice_level_charge(b"5"));
        ok(b.add_line_item(b"Service", b"2", b"HUR", b"50", b"15", 1));
        let invoice = ok(b.build());
        let xml = written(|out| ok(invoice.xml(out)));
        let parsed = ok(FinalizedInvoice::from_xml(xml.as_bytes()));
        let data = ok(parsed.data());
        let line = ok(data.line_item(0));
        let address = ok(ok(data.seller()).address());
        let expected_hash = written(|out| ok(invoice.hash_base64(out)));
        assert_eq!(
            written(|out| ok(fatoora_ffi::invoice::ffi::Xml::hash(xml.as_bytes(), out))),
            expected_hash
        );
        drop(invoice);
        drop(parsed);
        assert_eq!(data.invoice_type_kind(), 1);
        assert_eq!(data.invoice_sub_type(), subtype);
        assert_eq!(data.flags_raw(), 0b00101);
        assert_eq!(data.invoice_counter(), u64::MAX);
        assert_eq!(
            written(|out| ok(data.uuid(out))),
            "8e6000cf-1a98-4174-b3e7-b5d5954bc10d"
        );
        assert_eq!(written(|out| ok(data.previous_invoice_hash(out))), "hash");
        assert_eq!(written(|out| ok(data.payment_means_code(out))), "10");
        assert_eq!(written(|out| ok(data.invoice_level_discount(out))), "10");
        assert_eq!(written(|out| ok(data.invoice_level_charge(out))), "5");
        assert_eq!(written(|out| ok(line.unit_code(out))), "HUR");
        assert_eq!(written(|out| ok(line.total_amount(out))), "100");
        assert_eq!(written(|out| ok(line.vat_rate(out))), "15");
        assert_eq!(written(|out| ok(line.vat_amount(out))), "15");
        assert_eq!(written(|out| ok(address.street(out))), "King Fahd");
        assert_eq!(written(|out| ok(address.building_number(out))), "1234");
        assert_eq!(written(|out| ok(address.postal_code(out))), "12222");
        assert_eq!(written(|out| ok(address.country_code(out))), "SAU");
    }
}

fn optional_string(
    value: Result<Option<Box<fatoora_ffi::common::ffi::Text>>, Box<BindingError>>,
) -> Option<String> {
    ok(value).map(|text| written(|out| ok(text.value(out))))
}

#[test]
fn roundtrip_metadata_crosses_binding_boundary_and_snapshots_own_their_data() {
    use fatoora_ffi::invoice::ffi::{
        AllowanceCharge, InvoiceLineItem, OriginalInvoiceRef, TaxExemption, VatBreakdown,
    };
    let exemption = ok(TaxExemption::new(
        Some(b"VATEX-SA-32"),
        Some("تصدير & exempt".as_bytes()),
    ));
    let reference = ok(OriginalInvoiceRef::new(
        b"PRE-1",
        Some(b"original-uuid"),
        Some(b"2023-12-01"),
        Some(b"10:20:30"),
        Some(b"386"),
    ));
    let subtotal = ok(VatBreakdown::new(2, b"0", b"10", b"0", &exemption));
    let item = ok(InvoiceLineItem::new(
        b"Export", b"1", b"PCE", b"100", b"0", 2,
    ));
    let item = ok(item.with_metadata(Some(b"line-export"), &exemption));
    let item = ok(item.with_document_reference(&reference));
    let item = ok(item.with_tax_subtotal(&subtotal));
    let adjustment = ok(AllowanceCharge::new(
        false,
        b"5",
        Some(b"Discount & rebate"),
        Some(b"95"),
        2,
        b"0",
        &exemption,
    ));
    assert!(!adjustment.is_charge());
    let mut b = builder();
    ok(b.set_currency(b"USD"));
    ok(b.set_tax_currency(b"SAR", b"0"));
    ok(b.set_delivery(Some(b"2024-01-01"), Some(b"2024-01-02")));
    ok(b.set_payment_instruction(b"Transfer & receipt"));
    ok(b.set_prepaid_amount(b"10"));
    ok(b.set_payable_rounding_amount(b"0.01"));
    ok(b.set_note(b"ar", "فاتورة".as_bytes()));
    ok(b.set_tax_exemption(2, b"0", &exemption));
    ok(b.add_item(&item));
    ok(b.add_adjustment(&adjustment));
    let invoice = ok(b.build());
    let xml = written(|out| ok(invoice.xml(out)));
    let parsed = ok(FinalizedInvoice::from_xml(xml.as_bytes()));
    let data = ok(parsed.data());
    let totals = ok(parsed.totals());
    drop((
        b, invoice, parsed, item, reference, subtotal, adjustment, exemption,
    ));
    assert_eq!(
        optional_string(data.payment_instruction()).as_deref(),
        Some("Transfer & receipt")
    );
    assert_eq!(
        optional_string(data.actual_delivery_date()).as_deref(),
        Some("2024-01-01")
    );
    assert_eq!(
        optional_string(data.latest_delivery_date()).as_deref(),
        Some("2024-01-02")
    );
    assert_eq!(optional_string(data.tax_currency()).as_deref(), Some("SAR"));
    assert_eq!(
        optional_string(data.tax_currency_amount()).as_deref(),
        Some("0")
    );
    assert_eq!(
        optional_string(ok(data.note()).unwrap().language_id()).as_deref(),
        Some("ar")
    );
    assert_eq!(written(|out| ok(totals.prepaid_amount(out))), "10");
    assert_eq!(
        written(|out| ok(totals.payable_rounding_amount(out))),
        "0.01"
    );
    assert_eq!(written(|out| ok(totals.payable_amount(out))), "85.01");
    assert_eq!(data.adjustments_len(), 1);
    let adjustment = ok(data.adjustment(0));
    assert!(data.adjustment(1).is_err());
    assert!(!adjustment.is_charge());
    assert_eq!(adjustment.category(), 2);
    assert_eq!(adjustment.tax_category_count(), 1);
    assert_eq!(written(|out| ok(adjustment.amount(out))), "5");
    assert_eq!(written(|out| ok(adjustment.rate(out))), "0");
    assert_eq!(
        optional_string(adjustment.reason()).as_deref(),
        Some("Discount & rebate")
    );
    assert_eq!(
        optional_string(adjustment.reason_code()).as_deref(),
        Some("95")
    );
    let line = ok(data.line_item(0));
    assert_eq!(optional_string(line.id()).as_deref(), Some("line-export"));
    assert_eq!(line.document_references_len(), 1);
    assert_eq!(line.tax_subtotals_len(), 1);
    assert!(line.document_reference(1).is_err());
    assert!(line.tax_subtotal(1).is_err());
    let reference = ok(line.document_reference(0));
    assert_eq!(written(|out| ok(reference.id(out))), "PRE-1");
    assert_eq!(
        optional_string(reference.uuid()).as_deref(),
        Some("original-uuid")
    );
    assert_eq!(
        optional_string(reference.issue_date()).as_deref(),
        Some("2023-12-01")
    );
    assert_eq!(
        optional_string(reference.issue_time()).as_deref(),
        Some("10:20:30")
    );
    assert_eq!(
        optional_string(reference.document_type_code()).as_deref(),
        Some("386")
    );
    assert_eq!(totals.vat_breakdown_len(), 1);
    assert!(totals.vat_breakdown(1).is_err());
    let group = ok(totals.vat_breakdown(0));
    assert_eq!(group.category(), 2);
    assert_eq!(written(|out| ok(group.rate(out))), "0");
    assert_eq!(written(|out| ok(group.taxable_amount(out))), "95");
    assert_eq!(written(|out| ok(group.tax_amount(out))), "0");
    let subtotal = ok(line.tax_subtotal(0));
    assert_eq!(written(|out| ok(subtotal.taxable_amount(out))), "10");
    for exemption in [
        ok(line.exemption()),
        ok(adjustment.exemption()),
        ok(group.exemption()),
        ok(subtotal.exemption()),
    ] {
        assert_eq!(
            optional_string(exemption.reason_code()).as_deref(),
            Some("VATEX-SA-32")
        );
        assert_eq!(
            optional_string(exemption.reason()).as_deref(),
            Some("تصدير & exempt")
        );
    }
}

#[test]
fn optional_metadata_and_invalid_inputs_are_distinct_in_bindings() {
    use fatoora_ffi::invoice::ffi::{
        AllowanceCharge, InvoiceLineItem, OriginalInvoiceRef, TaxExemption, VatBreakdown,
    };
    let empty = ok(TaxExemption::new(None, None));
    assert_eq!(optional_string(empty.reason()), None);
    assert_eq!(optional_string(empty.reason_code()), None);
    let reference = ok(OriginalInvoiceRef::new(b"PRE", None, None, None, None));
    assert_eq!(optional_string(reference.issue_time()), None);
    assert_eq!(optional_string(reference.document_type_code()), None);
    assert!(OriginalInvoiceRef::new(b"PRE", None, Some(b"invalid"), None, None).is_err());
    assert!(OriginalInvoiceRef::new(b"PRE", None, None, Some(b"25:00:00"), None).is_err());
    assert!(TaxExemption::new(Some(&[0xff]), None).is_err());
    assert!(AllowanceCharge::new(true, b"invalid", None, None, 1, b"15", &empty).is_err());
    assert!(VatBreakdown::new(1, b"15", b"-1", b"0", &empty).is_err());
    assert!(InvoiceLineItem::new(b"x", b"invalid", b"PCE", b"1", b"15", 1).is_err());
    let item = ok(InvoiceLineItem::new(b"x", b"1", b"PCE", b"1", b"15", 1));
    let item = ok(item.with_metadata(None, &empty));
    assert_eq!(optional_string(item.id()), None);
    let mut b = builder();
    ok(b.set_delivery(None, None));
    assert!(b.set_delivery(Some(b"2024-02-30"), None).is_err());
    assert!(b.set_delivery(None, Some(b"2024-13-01")).is_err());
    assert!(b.set_tax_currency(b"INVALID", b"0").is_err());
    assert!(b.set_tax_currency(b"SAR", b"invalid").is_err());
    assert!(b.set_tax_exemption(255, b"0", &empty).is_err());
    assert!(b.set_prepaid_amount(b"bad").is_err());
    assert!(b.set_payable_rounding_amount(b"bad").is_err());
    assert!(b.set_payment_instruction(&[0xff]).is_err());
    ok(b.add_item(&item));
    let invoice = ok(b.build());
    let data = ok(invoice.data());
    assert_eq!(optional_string(data.actual_delivery_date()), None);
    assert_eq!(optional_string(data.latest_delivery_date()), None);
    assert_eq!(optional_string(data.payment_instruction()), None);
    assert_eq!(optional_string(data.tax_currency()).as_deref(), Some("SAR"));
    assert_eq!(
        optional_string(data.tax_currency_amount()).as_deref(),
        Some("0.15")
    );
    assert!(b.add_item(&item).is_err());
    let xml = written(|out| ok(invoice.xml(out))).replace(
        "<cbc:DocumentCurrencyCode>",
        "<cbc:Note>no language</cbc:Note><cbc:DocumentCurrencyCode>",
    );
    let parsed = ok(FinalizedInvoice::from_xml(xml.as_bytes()));
    assert_eq!(
        optional_string(ok(ok(parsed.data()).note()).unwrap().language_id()),
        None
    );
}

#[test]
fn supplied_blank_line_ids_return_indexed_binding_validation_errors() {
    use fatoora_ffi::invoice::ffi::{InvoiceLineItem, TaxExemption};
    let exemption = ok(TaxExemption::new(None, None));
    for id in ["", " ", "\t\r\n", "\u{2003}"] {
        let item = ok(InvoiceLineItem::new(
            b"Item", b"1", b"PCE", b"100", b"15", 1,
        ));
        let item = ok(item.with_metadata(Some(id.as_bytes()), &exemption));
        let mut b = builder();
        ok(b.add_line_item(b"First", b"1", b"PCE", b"10", b"15", 1));
        ok(b.add_item(&item));
        let error = b.build().err().expect("blank line ID must fail");
        let details: serde_json::Value =
            serde_json::from_str(&written(|out| error.details_json(out))).unwrap();
        assert_eq!(details["type"], "invoice_validation");
        assert_eq!(details["issues"][0]["field"], "line_item_id");
        assert_eq!(details["issues"][0]["kind"], "empty");
        assert_eq!(details["issues"][0]["line_item_index"], 1);
    }
}

#[test]
fn mixed_category_adjustment_preserves_invoice_and_adjustment_categories() {
    use fatoora_ffi::invoice::ffi::{AllowanceCharge, TaxExemption};
    let exemption = ok(TaxExemption::new(
        Some(b"VATEX-SA-29"),
        Some(b"Exempt charge"),
    ));
    let charge = ok(AllowanceCharge::new(
        true,
        b"5",
        Some(b"Fee"),
        None,
        0,
        b"0",
        &exemption,
    ));
    let mut b = builder();
    ok(b.add_line_item(b"Standard item", b"1", b"PCE", b"100", b"15", 1));
    ok(b.add_adjustment(&charge));
    let invoice = ok(b.build());
    let xml = written(|out| ok(invoice.xml(out)));
    let imported = ok(FinalizedInvoice::from_xml(xml.as_bytes()));
    let data = ok(imported.data());
    assert_eq!(data.vat_category(), 1);
    assert_eq!(ok(data.adjustment(0)).category(), 0);
    assert_eq!(
        written(|out| ok(ok(imported.totals()).payable_amount(out))),
        "120"
    );
    assert_eq!(written(|out| ok(imported.xml(out))), xml);
}
