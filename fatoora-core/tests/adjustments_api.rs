//! Document allowances and charges, payment means and omitted VAT rates.
mod common;
use fatoora_core::Decimal;
use fatoora_core::invoice::{
    Adjustment, AdjustmentKind, AllowanceCharge, DocumentAdjustment, InvoiceBuilder, InvoiceError,
    InvoiceField, InvoiceSubType, InvoiceType, LineItem, PaymentMeans, TaxExemption,
    ValidationKind, VatBreakdown, VatCategory,
};

fn d(value: &str) -> Decimal {
    Decimal::parse(value).unwrap()
}

/// A complete builder for a document in `category`, without payment means or lines.
fn base(category: VatCategory) -> InvoiceBuilder {
    let source = common::dummy_finalized_invoice();
    InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .id("ADJ-1")
        .uuid(source.data().uuid().clone())
        .issue_datetime("2024-01-01T12:30:00Z".parse().unwrap())
        .currency("SAR".parse().unwrap())
        .previous_invoice_hash(source.data().previous_invoice_hash().clone())
        .invoice_counter(1)
        .seller(source.data().seller().clone())
        .vat_category(category)
}

fn standard() -> InvoiceBuilder {
    base(VatCategory::Standard)
        .payment_means(PaymentMeans::new("10"))
        .line_item(
            LineItem::new(
                "Item",
                d("1"),
                "PCE",
                d("100"),
                d("15"),
                VatCategory::Standard,
            )
            .unwrap(),
        )
}

fn out_of_scope_line() -> LineItem {
    LineItem::builder()
        .item_name("Service")
        .invoiced_quantity(d("1"))
        .invoiced_quantity_unit_code("PCE")
        .item_net_price(d("100"))
        .vat_category(VatCategory::OutOfScope)
        .build()
        .unwrap()
}

fn entry(amount: &str) -> DocumentAdjustment {
    DocumentAdjustment::new(
        Adjustment::amount(d(amount)),
        VatCategory::Standard,
        d("15"),
    )
}

fn validation_issue(error: InvoiceError) -> (InvoiceField, ValidationKind) {
    let InvoiceError::Validation(validation) = error else {
        panic!("expected a validation error, got {error}");
    };
    let issue = &validation.issues()[0];
    (issue.field(), issue.kind())
}

// ---- New API --------------------------------------------------------------

#[test]
fn document_adjustment_carries_every_entry_field() {
    let exemption = TaxExemption::default()
        .with_reason_code("VATEX-SA-OOS")
        .with_reason("Outside scope");
    let adjustment = DocumentAdjustment::new(
        Adjustment::percent_of(d("100"), d("10")),
        VatCategory::OutOfScope,
        d("0"),
    )
    .with_reason("Volume")
    .with_reason_code("95")
    .with_exemption(exemption.clone())
    .omit_vat_rate()
    .with_tax_category_count(2);
    assert!(!adjustment.has_vat_rate());
    assert_eq!(adjustment.tax_category_count(), 2);
    assert!(
        DocumentAdjustment::new(Adjustment::amount(d("1")), VatCategory::Standard, d("15"))
            .has_vat_rate()
    );

    let invoice = base(VatCategory::OutOfScope)
        .payment_means(PaymentMeans::new("10"))
        .line_item(out_of_scope_line())
        .allowance(adjustment)
        .build()
        .unwrap();
    let entry = &invoice.data().adjustments()[0];
    assert_eq!(entry.kind(), AdjustmentKind::Allowance);
    assert_eq!(entry.amount(), d("10.00"));
    assert_eq!(entry.base_amount(), Some(d("100")));
    assert_eq!(entry.percentage(), Some(d("10")));
    assert_eq!(entry.reason(), Some("Volume"));
    assert_eq!(entry.reason_code(), Some("95"));
    assert_eq!(entry.category(), VatCategory::OutOfScope);
    assert_eq!(entry.exemption(), &exemption);
    assert!(!entry.has_vat_rate());
    assert_eq!(entry.tax_category_count(), 2);

    let xml = invoice.to_xml().unwrap();
    let start = xml.find("<cac:AllowanceCharge>").unwrap();
    let end = xml[start..].find("</cac:AllowanceCharge>").unwrap() + start;
    let element = &xml[start..end];
    assert_eq!(element.matches("<cac:TaxCategory>").count(), 2);
    assert!(!element.contains("<cbc:Percent>"));
    assert!(element.contains("<cbc:MultiplierFactorNumeric>10</cbc:MultiplierFactorNumeric>"));
}

#[test]
fn omitted_rate_and_tax_category_count_are_checked_in_build() {
    let error = standard()
        .charge(entry("5").omit_vat_rate())
        .build()
        .unwrap_err();
    assert_eq!(
        validation_issue(error),
        (
            InvoiceField::InvoiceLevelDiscount,
            ValidationKind::OutOfRange
        )
    );
    for count in [0, AllowanceCharge::MAX_TAX_CATEGORY_COUNT + 1] {
        assert!(
            standard()
                .charge(entry("5").with_tax_category_count(count))
                .build()
                .is_err()
        );
    }
}

#[test]
fn document_adjustment_from_entry_round_trips_through_the_builder() {
    let original = AllowanceCharge::new(
        AdjustmentKind::Charge,
        d("5.00"),
        VatCategory::Standard,
        d("15"),
    )
    .with_base_amount(d("100"))
    .with_percentage(d("5"))
    .with_reason("Freight")
    .with_reason_code("FC")
    .with_tax_category_count(3);
    let converted = DocumentAdjustment::from(original.clone());
    assert_eq!(converted.tax_category_count(), 3);
    assert!(converted.has_vat_rate());
    assert_eq!(converted.adjustment().base_amount(), Some(d("100")));
    assert_eq!(converted.adjustment().percentage(), Some(d("5")));
    let invoice = standard().charge(converted).build().unwrap();
    assert_eq!(invoice.data().adjustments(), [original]);
}

#[test]
fn allowances_and_charges_keep_call_order_and_totals() {
    let invoice = standard()
        .charge(entry("5").with_reason("Freight"))
        .allowance(entry("10").with_reason("Loyalty"))
        .build()
        .unwrap();
    let kinds: Vec<_> = invoice
        .data()
        .adjustments()
        .iter()
        .map(|a| a.kind())
        .collect();
    assert_eq!(kinds, [AdjustmentKind::Charge, AdjustmentKind::Allowance]);
    assert_eq!(invoice.data().invoice_level_charge(), d("5"));
    assert_eq!(invoice.data().invoice_level_discount(), d("10"));
    assert_eq!(invoice.totals().taxable_amount(), d("95"));
}

#[test]
fn invalid_entries_are_reported_after_field_checks() {
    // An entry whose calculation fails is reported by the adjustment checks...
    let error = standard()
        .allowance(DocumentAdjustment::new(
            Adjustment::amount(d("1")).with_base_amount(d("100")),
            VatCategory::Standard,
            d("15"),
        ))
        .build()
        .unwrap_err();
    assert_eq!(
        validation_issue(error),
        (InvoiceField::InvoiceLevelDiscount, ValidationKind::Missing)
    );
    // ...after the field checks, so a missing field is reported first.
    let error = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified))
        .allowance(entry("-1"))
        .build()
        .unwrap_err();
    assert_eq!(validation_issue(error).0, InvoiceField::Id);
}

#[test]
fn allowance_charge_omit_vat_rate() {
    let entry = out_of_scope_entry();
    assert!(entry.has_vat_rate());
    let omitted = entry.omit_vat_rate();
    assert!(!omitted.has_vat_rate());
}

/// A plain out-of-scope allowance entry.
fn out_of_scope_entry() -> AllowanceCharge {
    AllowanceCharge::new(
        AdjustmentKind::Allowance,
        d("1"),
        VatCategory::OutOfScope,
        d("0"),
    )
}

#[test]
fn line_item_omit_vat_rate() {
    let explicit = LineItem::new(
        "Item",
        d("1"),
        "PCE",
        d("10"),
        d("0"),
        VatCategory::OutOfScope,
    )
    .unwrap();
    assert!(explicit.has_vat_rate());
    let omitted = explicit.omit_vat_rate().unwrap();
    assert!(!omitted.has_vat_rate());
    assert_eq!(omitted, out_of_scope_line_with_price("10"));

    let standard = LineItem::new(
        "Item",
        d("1"),
        "PCE",
        d("10"),
        d("15"),
        VatCategory::Standard,
    )
    .unwrap();
    let error = standard.omit_vat_rate().unwrap_err();
    assert_eq!(
        validation_issue(error),
        (InvoiceField::LineItemVatRate, ValidationKind::OutOfRange)
    );
}

fn out_of_scope_line_with_price(price: &str) -> LineItem {
    LineItem::builder()
        .item_name("Item")
        .invoiced_quantity(d("1"))
        .invoiced_quantity_unit_code("PCE")
        .item_net_price(d(price))
        .vat_category(VatCategory::OutOfScope)
        .build()
        .unwrap()
}

#[test]
fn vat_breakdown_omit_vat_rate() {
    let breakdown = VatBreakdown::new(VatCategory::OutOfScope, d("0"), d("10"), d("0")).unwrap();
    assert!(breakdown.has_vat_rate());
    assert!(!breakdown.omit_vat_rate().unwrap().has_vat_rate());
    let standard = VatBreakdown::new(VatCategory::Standard, d("15"), d("10"), d("1.5")).unwrap();
    assert_eq!(
        validation_issue(standard.omit_vat_rate().unwrap_err()),
        (InvoiceField::VatCategory, ValidationKind::OutOfRange)
    );
}

#[test]
fn payment_means_appends() {
    let invoice = base(VatCategory::Standard)
        .payment_means(PaymentMeans::new("10"))
        .payment_means(PaymentMeans::new("30"))
        .line_item(
            LineItem::new(
                "Item",
                d("1"),
                "PCE",
                d("100"),
                d("15"),
                VatCategory::Standard,
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let codes: Vec<_> = invoice
        .data()
        .payment_means()
        .iter()
        .map(|p| p.code())
        .collect();
    assert_eq!(codes, ["10", "30"]);
}

#[test]
fn document_adjustment_deserializes_without_the_new_fields() {
    let json = serde_json::to_value(entry("5").with_reason("Freight")).unwrap();
    let mut object = json.as_object().unwrap().clone();
    assert_eq!(object.remove("omitted_rate"), Some(false.into()));
    assert_eq!(object.remove("tax_category_count"), Some(1.into()));
    let restored: DocumentAdjustment =
        serde_json::from_value(serde_json::Value::Object(object)).unwrap();
    assert_eq!(restored, entry("5").with_reason("Freight"));
}
