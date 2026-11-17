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

    // Re-serializing emits `kind` only; the rest of the document is unchanged.
    let mut expected: Value = serde_json::from_str(V0_5_1_JSON).unwrap();
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
