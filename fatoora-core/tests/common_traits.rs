//! Common trait guarantees on public types (C-COMMON-TRAITS, C-SEND-SYNC).

use fatoora_core::api::{Compliance, CsidCredentials, Production, ZatcaClient};
use fatoora_core::config::Config;
use fatoora_core::csr::SigningKey;
use fatoora_core::invoice::sign::InvoiceSigner;
use fatoora_core::invoice::{
    CountryCode, CurrencyCode, FinalizedInvoice, InvoiceBuilder, InvoiceDate, InvoiceTime,
    InvoiceTimestamp, OtherId, SignedInvoice, VatId,
};

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ZatcaClient>();
    assert_send_sync::<InvoiceSigner>();
    assert_send_sync::<SigningKey>();
    assert_send_sync::<FinalizedInvoice>();
    assert_send_sync::<SignedInvoice>();
    assert_send_sync::<Config>();
    assert_send_sync::<CsidCredentials<Compliance>>();
    assert_send_sync::<CsidCredentials<Production>>();
    assert_send_sync::<InvoiceBuilder>();
};

#[test]
fn validated_newtypes_display_as_str() {
    let country = CountryCode::parse("sa").unwrap();
    assert_eq!(country.to_string(), country.as_str());
    assert_eq!(country.to_string(), "SAU");

    let currency = CurrencyCode::parse("sar").unwrap();
    assert_eq!(currency.to_string(), currency.as_str());

    let timestamp = InvoiceTimestamp::parse("2024-01-01T12:30:00Z").unwrap();
    assert_eq!(timestamp.to_string(), timestamp.as_str());

    let date = InvoiceDate::parse("2024-01-01").unwrap();
    assert_eq!(date.to_string(), date.as_str());

    for raw in ["12:30:00", "12:30:00Z"] {
        let time = InvoiceTime::parse(raw).unwrap();
        assert_eq!(time.to_string(), time.as_str());
        assert_eq!(time.to_string(), raw);
    }

    let vat = VatId::parse(" 399999999900003 ").unwrap();
    assert_eq!(vat.to_string(), vat.as_str());
    assert_eq!(vat.to_string(), "399999999900003");
}

#[test]
fn other_id_display_omits_scheme() {
    let id = OtherId::with_scheme("7003339333", "CRN");
    assert_eq!(id.to_string(), id.as_str());
    assert_eq!(id.to_string(), "7003339333");

    let plain = OtherId::new("abc");
    assert_eq!(plain.to_string(), plain.as_str());
}
