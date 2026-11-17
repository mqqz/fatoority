//! Constructors, getters and `with_*` setters of the value types whose fields are
//! private, their deprecated forwarders, and their pinned serde wire format.
use fatoora_core::Decimal;
use fatoora_core::invoice::{
    Address, Adjustment, AdjustmentKind, AllowanceCharge, CountryCode, Delivery,
    DocumentAdjustment, InvoiceDate, LineItem, PaymentAccount, PaymentMeans, TaxExemption,
    VatCategory,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

fn d(value: &str) -> Decimal {
    value.parse().unwrap()
}

/// Asserts `value` serializes to exactly `json` and that `json` deserializes back to `value`.
fn assert_wire<T>(value: &T, json: &str)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    assert_eq!(serde_json::to_string(value).unwrap(), json);
    assert_eq!(&serde_json::from_str::<T>(json).unwrap(), value);
}

fn full_address() -> Address {
    Address::new(
        CountryCode::parse("SA").unwrap(),
        "Riyadh",
        "King",
        "1234",
        "12345",
    )
    .with_province("P")
    .with_additional_street("AS")
    .with_additional_number("5678")
    .with_district("D")
}

#[test]
fn address_constructor_getters_and_with_methods() {
    let minimal = Address::new(
        CountryCode::parse("SAU").unwrap(),
        "Riyadh",
        String::from("King Fahd"),
        "1234",
        "12222",
    );
    assert_eq!(minimal.country_code().as_str(), "SAU");
    assert_eq!(minimal.city(), "Riyadh");
    assert_eq!(minimal.street(), "King Fahd");
    assert_eq!(minimal.building_number(), "1234");
    assert_eq!(minimal.postal_code(), "12222");
    assert_eq!(minimal.province(), None);
    assert_eq!(minimal.additional_street(), None);
    assert_eq!(minimal.additional_number(), None);
    assert_eq!(minimal.district(), None);

    let full = full_address();
    assert_eq!(full.province(), Some("P"));
    assert_eq!(full.additional_street(), Some("AS"));
    assert_eq!(full.additional_number(), Some("5678"));
    assert_eq!(full.district(), Some("D"));
}

#[test]
fn address_wire_format_is_unchanged() {
    assert_wire(
        &full_address(),
        r#"{"province":"P","country_code":"SAU","city":"Riyadh","street":"King","additional_street":"AS","building_number":"1234","additional_number":"5678","postal_code":"12345","district":"D"}"#,
    );
    // `province` keeps its serde default.
    let without_province: Address = serde_json::from_str(
        r#"{"country_code":"SAU","city":"C","street":"S","additional_street":null,"building_number":"1","additional_number":null,"postal_code":"2","district":null}"#,
    )
    .unwrap();
    assert_eq!(
        without_province,
        Address::new(CountryCode::parse("SAU").unwrap(), "C", "S", "1", "2")
    );
}

#[test]
fn tax_exemption_with_methods_getters_and_wire_format() {
    let empty = TaxExemption::default();
    assert_eq!(empty.reason_code(), None);
    assert_eq!(empty.reason(), None);
    let exemption = TaxExemption::default()
        .with_reason_code("VATEX-SA-29")
        .with_reason(String::from("Financial"));
    assert_eq!(exemption.reason_code(), Some("VATEX-SA-29"));
    assert_eq!(exemption.reason(), Some("Financial"));
    assert_wire(
        &exemption,
        r#"{"reason_code":"VATEX-SA-29","reason":"Financial"}"#,
    );
    assert_wire(&empty, r#"{"reason_code":null,"reason":null}"#);
}

#[test]
fn payment_means_and_account_with_methods_getters_and_wire_format() {
    let account = PaymentAccount::default()
        .with_payment_account_identifier("SA03")
        .with_payment_terms("Net 30");
    assert_eq!(account.payment_account_identifier(), Some("SA03"));
    assert_eq!(account.payment_terms(), Some("Net 30"));
    let empty_account = PaymentAccount::default();
    assert_eq!(empty_account.payment_account_identifier(), None);
    assert_eq!(empty_account.payment_terms(), None);

    let bare = PaymentMeans::new("10");
    assert_eq!(bare.code(), "10");
    assert!(bare.instruction_notes().is_empty());
    assert_eq!(bare.payee_financial_account(), None);

    let means = PaymentMeans::new(String::from("30"))
        .with_instruction_note("n")
        .with_instruction_note("m")
        .with_payee_financial_account(account.clone());
    assert_eq!(means.code(), "30");
    assert_eq!(means.instruction_notes(), ["n", "m"]);
    assert_eq!(means.payee_financial_account(), Some(&account));

    assert_wire(
        &PaymentMeans::new("30")
            .with_instruction_note("n")
            .with_payee_financial_account(account),
        r#"{"payment_means_type_code":"30","instruction_notes":["n"],"payee_financial_account":{"payment_account_identifier":"SA03","payment_terms":"Net 30"}}"#,
    );
}

#[test]
#[allow(deprecated)] // covers the deprecated forwarder
fn deprecated_instruction_note_forwards_to_with_instruction_note() {
    assert_eq!(
        PaymentMeans::new("30").instruction_note("n"),
        PaymentMeans::new("30").with_instruction_note("n")
    );
}

#[test]
fn delivery_with_methods_getters_and_wire_format() {
    let empty = Delivery::default();
    assert_eq!(empty.actual_date(), None);
    assert_eq!(empty.latest_date(), None);
    let actual = InvoiceDate::parse("2024-01-01").unwrap();
    let latest = InvoiceDate::parse("2024-01-31").unwrap();
    let delivery = Delivery::default()
        .with_actual_date(actual.clone())
        .with_latest_date(latest.clone());
    assert_eq!(delivery.actual_date(), Some(&actual));
    assert_eq!(delivery.latest_date(), Some(&latest));
    assert_wire(
        &delivery,
        r#"{"actual_date":"2024-01-01","latest_date":"2024-01-31"}"#,
    );
}

#[test]
fn allowance_charge_constructor_getters_and_with_methods() {
    let bare = AllowanceCharge::new(AdjustmentKind::Allowance, d("5"), VatCategory::Zero, d("0"));
    assert_eq!(bare.kind(), AdjustmentKind::Allowance);
    assert_eq!(bare.amount(), d("5"));
    assert_eq!(bare.category(), VatCategory::Zero);
    assert_eq!(bare.rate(), d("0"));
    assert_eq!(bare.base_amount(), None);
    assert_eq!(bare.percentage(), None);
    assert_eq!(bare.reason(), None);
    assert_eq!(bare.reason_code(), None);
    assert_eq!(bare.exemption(), &TaxExemption::default());
    assert_eq!(bare.tax_category_count(), 1);
    assert!(bare.has_vat_rate());

    let exemption = TaxExemption::default().with_reason_code("VATEX-SA-32");
    let full = bare
        .with_base_amount(d("50"))
        .with_percentage(d("10"))
        .with_reason("r")
        .with_reason_code("95")
        .with_exemption(exemption.clone())
        .with_tax_category_count(3)
        .omit_vat_rate();
    assert_eq!(full.base_amount(), Some(d("50")));
    assert_eq!(full.percentage(), Some(d("10")));
    assert_eq!(full.reason(), Some("r"));
    assert_eq!(full.reason_code(), Some("95"));
    assert_eq!(full.exemption(), &exemption);
    assert_eq!(full.tax_category_count(), 3);
    assert!(!full.has_vat_rate());
}

#[test]
fn allowance_charge_wire_format_is_unchanged() {
    let entry = AllowanceCharge::new(
        AdjustmentKind::Charge,
        d("10"),
        VatCategory::Standard,
        d("15"),
    )
    .with_base_amount(d("100"))
    .with_percentage(d("10"))
    .with_tax_category_count(2)
    .with_reason("r")
    .with_reason_code("FC");
    assert_wire(
        &entry,
        r#"{"omitted_rate":false,"base_amount":"100","percentage":"10","tax_category_count":2,"kind":"charge","amount":"10","reason":"r","reason_code":"FC","category":"Standard","rate":"15","exemption":{"reason_code":null,"reason":null}}"#,
    );
}

#[test]
fn adjustment_renamed_constructor_setters_and_getters() {
    let percent = Adjustment::percent_of(d("200"), d("5"))
        .with_reason("Volume")
        .with_reason_code("95");
    assert_eq!(percent.base_amount(), Some(d("200")));
    assert_eq!(percent.percentage(), Some(d("5")));
    assert_eq!(percent.reason(), Some("Volume"));
    assert_eq!(percent.reason_code(), Some("95"));
    assert_eq!(percent.value().unwrap(), d("10.00"));

    let fixed = Adjustment::amount(d("3"));
    assert_eq!(fixed.base_amount(), None);
    assert_eq!(fixed.percentage(), None);
    assert_eq!(fixed.reason(), None);
    assert_eq!(fixed.reason_code(), None);
    // A base amount without a percentage is incomplete.
    assert!(fixed.with_base_amount(d("30")).value().is_err());
}

#[test]
#[allow(deprecated)] // covers the deprecated forwarders
fn adjustment_deprecated_getters_forward_to_plain_names() {
    let adjustment = Adjustment::percent_of(d("200"), d("5"))
        .with_reason("Volume")
        .with_reason_code("95");
    assert_eq!(adjustment.allowance_charge_reason(), adjustment.reason());
    assert_eq!(
        adjustment.allowance_charge_reason_code(),
        adjustment.reason_code()
    );
    assert_eq!(
        adjustment.calculation_base_amount(),
        adjustment.base_amount()
    );
    assert_eq!(adjustment.calculation_percentage(), adjustment.percentage());
    assert_eq!(adjustment.allowance_charge_reason(), Some("Volume"));
}

#[test]
fn document_adjustment_with_methods_and_getters() {
    let bare = DocumentAdjustment::new(Adjustment::amount(d("5")), VatCategory::Standard, d("15"));
    assert_eq!(bare.adjustment(), &Adjustment::amount(d("5")));
    assert_eq!(bare.category(), VatCategory::Standard);
    assert_eq!(bare.rate(), d("15"));
    assert_eq!(bare.reason(), None);
    assert_eq!(bare.reason_code(), None);
    assert_eq!(bare.exemption(), &TaxExemption::default());
    assert_eq!(bare.tax_category_count(), 1);
    assert!(bare.has_vat_rate());

    let exemption = TaxExemption::default().with_reason("Outside scope");
    let full = DocumentAdjustment::new(Adjustment::amount(d("5")), VatCategory::OutOfScope, d("0"))
        .with_reason("Loyalty")
        .with_reason_code("95")
        .with_exemption(exemption.clone())
        .with_tax_category_count(2)
        .omit_vat_rate();
    assert_eq!(full.reason(), Some("Loyalty"));
    assert_eq!(full.reason_code(), Some("95"));
    assert_eq!(full.adjustment().reason(), Some("Loyalty"));
    assert_eq!(full.exemption(), &exemption);
    assert_eq!(full.tax_category_count(), 2);
    assert!(!full.has_vat_rate());
}

#[test]
fn document_adjustment_wire_format_is_unchanged() {
    let adjustment = DocumentAdjustment::new(
        Adjustment::percent_of(d("100"), d("10")).with_reason("r"),
        VatCategory::Standard,
        d("15"),
    )
    .with_reason_code("95")
    .with_tax_category_count(2);
    assert_wire(
        &adjustment,
        r#"{"adjustment":{"amount":null,"base_amount":"100","percentage":"10","reason":"r","reason_code":"95"},"category":"Standard","rate":"15","exemption":{"reason_code":null,"reason":null},"omitted_rate":false,"tax_category_count":2}"#,
    );
}

#[test]
fn line_adjustment_getters_and_wire_format() {
    let line = LineItem::builder()
        .item_name("Item")
        .invoiced_quantity(d("1"))
        .item_net_price(d("100"))
        .vat_category(VatCategory::Standard)
        .vat_rate(d("15"))
        .allowance(Adjustment::amount(d("5")))
        .charge(Adjustment::percent_of(d("100"), d("2")).with_reason("Freight"))
        .build()
        .unwrap();
    let [allowance, charge] = line.adjustments() else {
        panic!("expected two adjustments");
    };
    assert_eq!(allowance.kind(), AdjustmentKind::Allowance);
    assert_eq!(allowance.adjustment().value().unwrap(), d("5"));
    assert_eq!(charge.kind(), AdjustmentKind::Charge);
    assert_eq!(charge.adjustment().reason(), Some("Freight"));
    assert_eq!(charge.adjustment().value().unwrap(), d("2.00"));
    assert_eq!(line.total_amount(), d("97.00"));

    let json = r#"{"kind":"allowance","adjustment":{"amount":"5","base_amount":null,"percentage":null,"reason":null,"reason_code":null}}"#;
    assert_eq!(serde_json::to_string(allowance).unwrap(), json);
    assert_eq!(
        &serde_json::from_str::<fatoora_core::invoice::LineAdjustment>(json).unwrap(),
        allowance
    );
}
