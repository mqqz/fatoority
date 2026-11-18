//! Serde compatibility with JSON persisted by fatoora-core 0.5, which spelled
//! an adjustment's kind as `"charge": bool` instead of `"kind"`.
mod common;
use fatoora_core::Decimal;
use fatoora_core::invoice::{
    Adjustment, AdjustmentKind, AllowanceCharge, DocumentAdjustment, InvoiceBuilder, InvoiceData,
    InvoiceSubType, InvoiceType, LineAdjustment, LineItem, PaymentMeans, VatCategory,
};
use serde_json::{Value, json};

/// `serde_json::to_string_pretty(invoice.data())` from fatoora-core v0.5.1 for
/// the invoice [`current_invoice`] builds.
const V0_5_1_JSON: &str = include_str!("fixtures/serde/invoice-data-v0.5.1.json");
/// `hash_base64()` of the same invoice under v0.5.1.
const V0_5_1_HASH: &str = "zcgG7AnCQWejCWF1+FIkVctXxgiYy6DXPrHvQplBr84=";

/// The 0.5/0.6 `InvoiceData` fields replaced by `adjustments` in 0.7.
const LEGACY_AGGREGATE_FIELDS: [&str; 4] = [
    "invoice_level_charge",
    "invoice_level_discount",
    "adjustment_vat_rate",
    "allowance_reason",
];

fn d(value: &str) -> Decimal {
    Decimal::parse(value).unwrap()
}

/// One document allowance, one document charge, one line allowance and one line charge.
fn current_invoice() -> InvoiceData {
    let source = common::dummy_finalized_invoice();
    let line = LineItem::builder()
        .item_name("Widget")
        .invoiced_quantity(d("2"))
        .invoiced_quantity_unit_code("PCE")
        .item_net_price(d("100"))
        .vat_category(VatCategory::Standard)
        .vat_rate(d("15"))
        .allowance(
            Adjustment::amount(d("5.00"))
                .with_reason("Line discount")
                .with_reason_code("95"),
        )
        .charge(
            Adjustment::percent_of(d("100.00"), d("10"))
                .with_reason("Line handling")
                .with_reason_code("ABL"),
        )
        .build()
        .unwrap();
    let invoice = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("INV-LEGACY-1")
        .uuid(source.data().uuid().clone())
        .issue_datetime("2024-01-01T12:30:00Z".parse().unwrap())
        .currency("SAR".parse().unwrap())
        .previous_invoice_hash(source.data().previous_invoice_hash().clone())
        .invoice_counter(0)
        .seller(source.data().seller().clone())
        .payment_means(PaymentMeans::new("10"))
        .vat_category(VatCategory::Standard)
        .allowance(
            DocumentAdjustment::new(
                Adjustment::amount(d("10.00")),
                VatCategory::Standard,
                d("15"),
            )
            .with_reason("Loyalty discount")
            .with_reason_code("95"),
        )
        .charge(
            DocumentAdjustment::new(
                Adjustment::amount(d("4.00")),
                VatCategory::Standard,
                d("15"),
            )
            .with_reason("Freight")
            .with_reason_code("FC"),
        )
        .line_item(line)
        .build()
        .unwrap();
    assert_eq!(invoice.hash().unwrap().as_str(), V0_5_1_HASH);
    invoice.data().clone()
}

/// Every adjustment object in serialized invoice data.
fn adjustments(data: &mut Value) -> Vec<&mut Value> {
    let mut out = Vec::new();
    for (key, value) in data.as_object_mut().unwrap() {
        match key.as_str() {
            "line_items" => {
                for line in value.as_array_mut().unwrap() {
                    out.extend(line["adjustments"].as_array_mut().unwrap());
                }
            }
            "adjustments" => out.extend(value.as_array_mut().unwrap()),
            _ => {}
        }
    }
    out
}

#[test]
fn v0_5_1_json_deserializes_to_the_current_model() {
    let current = current_invoice();
    let legacy: InvoiceData = serde_json::from_str(V0_5_1_JSON).unwrap();
    assert_eq!(legacy, current);

    let kinds: Vec<AdjustmentKind> = legacy
        .line_items()
        .iter()
        .flat_map(|line| line.adjustments().iter().map(LineAdjustment::kind))
        .chain(legacy.adjustments().iter().map(AllowanceCharge::kind))
        .collect();
    assert_eq!(
        kinds,
        [
            AdjustmentKind::Allowance,
            AdjustmentKind::Charge,
            AdjustmentKind::Allowance,
            AdjustmentKind::Charge,
        ]
    );

    // Re-serializing emits `kind` only and drops the aggregate fields, which
    // the entries already carry; the rest of the document is unchanged.
    let mut expected: Value = serde_json::from_str(V0_5_1_JSON).unwrap();
    for field in LEGACY_AGGREGATE_FIELDS {
        expected.as_object_mut().unwrap().remove(field).unwrap();
    }
    for adjustment in adjustments(&mut expected) {
        let object = adjustment.as_object_mut().unwrap();
        let charge = object.remove("charge").unwrap().as_bool().unwrap();
        object.insert(
            "kind".into(),
            json!(if charge { "charge" } else { "allowance" }),
        );
    }
    assert_eq!(serde_json::to_value(&legacy).unwrap(), expected);
}

#[test]
fn current_json_round_trips_with_kind_only() {
    let current = current_invoice();
    let mut value = serde_json::to_value(&current).unwrap();
    for adjustment in adjustments(&mut value) {
        assert!(adjustment.get("kind").is_some());
        assert!(adjustment.get("charge").is_none());
    }
    let decoded: InvoiceData = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(decoded, current);
    assert_eq!(serde_json::to_value(&decoded).unwrap(), value);
}

#[test]
fn agreeing_kind_and_charge_are_accepted() {
    let current = current_invoice();
    let mut value = serde_json::to_value(&current).unwrap();
    for adjustment in adjustments(&mut value) {
        let charge = adjustment["kind"] == "charge";
        adjustment["charge"] = json!(charge);
    }
    let decoded: InvoiceData = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, current);
}

#[test]
fn conflicting_kind_and_charge_are_rejected() {
    let current = current_invoice();
    let base = serde_json::to_value(&current).unwrap();
    let count = adjustments(&mut base.clone()).len();
    for index in 0..count {
        let mut value = base.clone();
        let adjustment = adjustments(&mut value).swap_remove(index);
        let charge = adjustment["kind"] == "charge";
        adjustment["charge"] = json!(!charge);
        let error = serde_json::from_value::<InvoiceData>(value)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("conflicting adjustment `kind` and legacy `charge` fields"),
            "{error}"
        );
    }
}

#[test]
fn missing_kind_and_charge_are_rejected() {
    let current = current_invoice();
    let base = serde_json::to_value(&current).unwrap();
    let count = adjustments(&mut base.clone()).len();
    for index in 0..count {
        let mut value = base.clone();
        let adjustment = adjustments(&mut value).swap_remove(index);
        adjustment.as_object_mut().unwrap().remove("kind");
        let error = serde_json::from_value::<InvoiceData>(value)
            .unwrap_err()
            .to_string();
        assert!(error.contains("missing field `kind`"), "{error}");
    }
}

#[test]
fn standalone_adjustments_accept_either_spelling() {
    let line: LineAdjustment = serde_json::from_value(json!({
        "charge": true,
        "adjustment": {"amount": "1", "base_amount": null, "percentage": null,
                       "reason": null, "reason_code": null}
    }))
    .unwrap();
    assert_eq!(line.kind(), AdjustmentKind::Charge);

    let entry: AllowanceCharge = serde_json::from_value(json!({
        "charge": false,
        "amount": "1",
        "reason": null,
        "reason_code": null,
        "category": "Standard",
        "rate": "15"
    }))
    .unwrap();
    assert_eq!(entry.kind(), AdjustmentKind::Allowance);
    assert_eq!(entry.tax_category_count(), 1);
    assert!(entry.has_vat_rate());
}

#[test]
fn option_aware_formats_round_trip_adjustments() {
    // RON spells `Option` explicitly; the bare `kind` written by `Serialize`
    // must still be read back.
    let data = current_invoice();
    let ron_text = ron::to_string(&data).unwrap();
    assert!(!ron_text.contains("Some(allowance)"), "{ron_text}");
    assert_eq!(ron::from_str::<InvoiceData>(&ron_text).unwrap(), data);
    // With struct names on, the shadow types must read back under the public names.
    let named =
        ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default().struct_names(true))
            .unwrap();
    assert!(named.contains("AllowanceCharge("), "{named}");
    assert!(named.contains("LineAdjustment("), "{named}");
    assert_eq!(ron::from_str::<InvoiceData>(&named).unwrap(), data);

    for entry in data.adjustments() {
        let text = ron::to_string(entry).unwrap();
        assert_eq!(&ron::from_str::<AllowanceCharge>(&text).unwrap(), entry);
    }
    for line in data.line_items() {
        for adjustment in line.adjustments() {
            let text = ron::to_string(adjustment).unwrap();
            assert_eq!(&ron::from_str::<LineAdjustment>(&text).unwrap(), adjustment);
        }
    }
}

#[test]
fn compact_formats_round_trip_adjustments() {
    // bincode reads fields by position, so it must see exactly the serialized
    // shape with no legacy `charge` slot.
    let data = current_invoice();
    let mut checked = 0;
    for entry in data.adjustments() {
        let bytes = bincode::serialize(entry).unwrap();
        assert_eq!(
            &bincode::deserialize::<AllowanceCharge>(&bytes).unwrap(),
            entry
        );
        checked += 1;
    }
    for line in data.line_items() {
        for adjustment in line.adjustments() {
            let bytes = bincode::serialize(adjustment).unwrap();
            assert_eq!(
                &bincode::deserialize::<LineAdjustment>(&bytes).unwrap(),
                adjustment
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 4);
}

/// `serde_json::to_string_pretty(invoice.data())` from fatoora-core v0.6.0 for
/// invoices built with the deprecated aggregate setters (two lines: 2 x 100.00
/// at standard 15% and 50.00 zero-rated):
/// `invoice_level_discount(10.00)`, `invoice_level_charge(4.00)`,
/// `allowance_reason("Loyalty")`.
const V0_6_0_AGGREGATES: &str = include_str!("fixtures/serde/invoice-data-v0.6.0-aggregates.json");
/// The same invoice serialized with `bincode::serialize`.
const V0_6_0_AGGREGATES_BINCODE: &[u8] =
    include_bytes!("fixtures/serde/invoice-data-v0.6.0-aggregates.bincode");
/// `bincode::serialize` under v0.6.0 of the v0.5.1 JSON fixture, which has entries.
const V0_6_0_ENTRIES_BINCODE: &[u8] =
    include_bytes!("fixtures/serde/invoice-data-v0.6.0-entries.bincode");
/// `invoice_level_discount(7.50)` only.
const V0_6_0_DISCOUNT_ONLY: &str =
    include_str!("fixtures/serde/invoice-data-v0.6.0-discount-only.json");
/// `allowance_reason("Reason only")` only, which emitted a zero-amount allowance.
const V0_6_0_REASON_ONLY: &str =
    include_str!("fixtures/serde/invoice-data-v0.6.0-reason-only.json");
/// The aggregates invoice plus a standard 5% line, so two standard VAT groups
/// match the document category; v0.6.0 rejected its totals.
const V0_6_0_AMBIGUOUS: &str = include_str!("fixtures/serde/invoice-data-v0.6.0-ambiguous.json");
/// The ambiguous invoice with `adjustment_vat_rate` 5 selecting one group.
const V0_6_0_VAT_RATE: &str = include_str!("fixtures/serde/invoice-data-v0.6.0-vat-rate.json");

fn entry(kind: AdjustmentKind, amount: &str, rate: &str, reason: &str) -> AllowanceCharge {
    AllowanceCharge::new(kind, d(amount), VatCategory::Standard, d(rate)).with_reason(reason)
}

#[test]
fn legacy_aggregates_become_entries_with_the_shared_reason() {
    let data: InvoiceData = serde_json::from_str(V0_6_0_AGGREGATES).unwrap();
    assert_eq!(
        data.adjustments(),
        [
            entry(AdjustmentKind::Allowance, "10", "15", "Loyalty"),
            entry(AdjustmentKind::Charge, "4", "15", "Loyalty"),
        ]
    );
    assert_eq!(data.invoice_level_discount(), d("10"));
    assert_eq!(data.invoice_level_charge(), d("4"));
    assert_eq!(data.allowance_reason(), Some("Loyalty"));
}

#[test]
fn legacy_aggregates_without_a_reason_use_the_legacy_defaults() {
    let data: InvoiceData = serde_json::from_str(V0_6_0_DISCOUNT_ONLY).unwrap();
    assert_eq!(
        data.adjustments(),
        [entry(AdjustmentKind::Allowance, "7.5", "15", "discount")]
    );
    assert_eq!(data.invoice_level_charge(), Decimal::ZERO);

    // A charge alone defaults to "charge"; with both, the reasons differ.
    let mut value: Value = serde_json::from_str(V0_6_0_AGGREGATES).unwrap();
    value["allowance_reason"] = Value::Null;
    value["invoice_level_discount"] = json!("0");
    let data: InvoiceData = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(
        data.adjustments(),
        [entry(AdjustmentKind::Charge, "4", "15", "charge")]
    );
    assert_eq!(data.allowance_reason(), Some("charge"));
    value["invoice_level_discount"] = json!("10");
    let data: InvoiceData = serde_json::from_value(value).unwrap();
    assert_eq!(data.adjustments().len(), 2);
    assert_eq!(data.allowance_reason(), None);
}

#[test]
fn a_legacy_reason_alone_becomes_a_zero_allowance() {
    let data: InvoiceData = serde_json::from_str(V0_6_0_REASON_ONLY).unwrap();
    assert_eq!(
        data.adjustments(),
        [entry(AdjustmentKind::Allowance, "0", "15", "Reason only")]
    );
    assert_eq!(data.invoice_level_discount(), Decimal::ZERO);
}

#[test]
fn legacy_adjustment_vat_rate_selects_the_vat_group() {
    let data: InvoiceData = serde_json::from_str(V0_6_0_VAT_RATE).unwrap();
    assert_eq!(
        data.adjustments(),
        [
            entry(AdjustmentKind::Allowance, "10", "5", "Loyalty"),
            entry(AdjustmentKind::Charge, "4", "5", "Loyalty"),
        ]
    );
}

#[test]
fn ambiguous_legacy_aggregates_are_rejected() {
    let error = serde_json::from_str::<InvoiceData>(V0_6_0_AMBIGUOUS)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains(
            "cannot convert the legacy invoice_level_discount/invoice_level_charge: \
             VAT category Standard matches 2 VAT groups of the line items, expected exactly one"
        ),
        "{error}"
    );

    // A rate no line has matches no group.
    let mut value: Value = serde_json::from_str(V0_6_0_AMBIGUOUS).unwrap();
    value["adjustment_vat_rate"] = json!("7");
    let error = serde_json::from_value::<InvoiceData>(value)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("VAT category Standard and rate 7 matches 0 VAT groups"),
        "{error}"
    );

    // Neither does a category no line has.
    let mut value: Value = serde_json::from_str(V0_6_0_AGGREGATES).unwrap();
    value["vat_category"] = json!("Exempt");
    let error = serde_json::from_value::<InvoiceData>(value)
        .unwrap_err()
        .to_string();
    assert!(error.contains("VAT category Exempt matches 0"), "{error}");
}

#[test]
fn aggregate_fields_beside_entries_are_ignored() {
    let current = current_invoice();
    let mut value = serde_json::to_value(&current).unwrap();
    value["invoice_level_discount"] = json!("999");
    value["invoice_level_charge"] = json!("999");
    value["adjustment_vat_rate"] = json!("5");
    value["allowance_reason"] = json!("Ignored");
    let decoded: InvoiceData = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, current);
    assert_eq!(decoded.invoice_level_discount(), d("10"));
    assert_eq!(decoded.invoice_level_charge(), d("4"));
    // The entries' reasons differ.
    assert_eq!(decoded.allowance_reason(), None);
}

#[test]
fn json_no_longer_writes_the_aggregate_fields() {
    for data in [
        current_invoice(),
        serde_json::from_str(V0_6_0_AGGREGATES).unwrap(),
        common::dummy_finalized_invoice().data().clone(),
    ] {
        let value = serde_json::to_value(&data).unwrap();
        for field in LEGACY_AGGREGATE_FIELDS {
            assert!(value.get(field).is_none(), "{field}");
        }
        assert_eq!(serde_json::from_value::<InvoiceData>(value).unwrap(), data);
        let text = ron::to_string(&data).unwrap();
        assert_eq!(ron::from_str::<InvoiceData>(&text).unwrap(), data);
    }
}

#[test]
fn bincode_keeps_the_0_6_layout() {
    // v0.6.0 bincode converts like the JSON written alongside it.
    let legacy: InvoiceData = bincode::deserialize(V0_6_0_AGGREGATES_BINCODE).unwrap();
    let json: InvoiceData = serde_json::from_str(V0_6_0_AGGREGATES).unwrap();
    assert_eq!(legacy, json);

    for data in [
        current_invoice(),
        legacy,
        common::dummy_finalized_invoice().data().clone(),
    ] {
        let bytes = bincode::serialize(&data).unwrap();
        assert_eq!(bincode::deserialize::<InvoiceData>(&bytes).unwrap(), data);
    }

    // The aggregate slots hold the entry totals, as 0.6 wrote them, so data
    // with entries encodes to the same bytes as under 0.6, in both directions.
    let current = current_invoice();
    assert_eq!(
        bincode::serialize(&current).unwrap(),
        V0_6_0_ENTRIES_BINCODE
    );
    assert_eq!(
        bincode::deserialize::<InvoiceData>(V0_6_0_ENTRIES_BINCODE).unwrap(),
        current
    );
}

#[test]
fn getters_are_computed_from_the_entries() {
    let source = common::dummy_finalized_invoice();
    let adjustment = |amount: &str, reason: &str| {
        DocumentAdjustment::new(
            Adjustment::amount(d(amount)),
            VatCategory::Standard,
            d("15"),
        )
        .with_reason(reason)
    };
    let mut builder = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("INV-GETTERS")
        .uuid(source.data().uuid().clone())
        .issue_datetime("2024-01-01T12:30:00Z".parse().unwrap())
        .currency("SAR".parse().unwrap())
        .previous_invoice_hash(source.data().previous_invoice_hash().clone())
        .invoice_counter(0)
        .seller(source.data().seller().clone())
        .payment_means(PaymentMeans::new("10"))
        .vat_category(VatCategory::Standard)
        .allowance(adjustment("3.00", "Same"))
        .allowance(adjustment("2.50", "Same"))
        .charge(adjustment("1.25", "Same"));
    for line in source.data().line_items() {
        builder = builder.line_item(line.clone());
    }
    let invoice = builder.clone().build().unwrap();
    assert_eq!(invoice.data().invoice_level_discount(), d("5.50"));
    assert_eq!(invoice.data().invoice_level_charge(), d("1.25"));
    assert_eq!(invoice.data().allowance_reason(), Some("Same"));
    assert_eq!(invoice.totals().allowance_total(), d("5.50"));
    assert_eq!(invoice.totals().charge_total(), d("1.25"));

    let invoice = builder.charge(adjustment("1.00", "Other")).build().unwrap();
    assert_eq!(invoice.data().invoice_level_charge(), d("2.25"));
    assert_eq!(invoice.data().allowance_reason(), None);

    let plain = source.data();
    assert!(plain.adjustments().is_empty());
    assert_eq!(plain.invoice_level_discount(), Decimal::ZERO);
    assert_eq!(plain.invoice_level_charge(), Decimal::ZERO);
    assert_eq!(plain.allowance_reason(), None);
}

#[test]
fn out_of_range_entry_totals_are_rejected() {
    let mut value = serde_json::to_value(current_invoice()).unwrap();
    let huge = "79228162514264337593543950335";
    for adjustment in value["adjustments"].as_array_mut().unwrap() {
        adjustment["amount"] = json!(huge);
        adjustment["kind"] = json!("charge");
    }
    let error = serde_json::from_value::<InvoiceData>(value)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("document-level allowance or charge total is out of range"),
        "{error}"
    );
}
