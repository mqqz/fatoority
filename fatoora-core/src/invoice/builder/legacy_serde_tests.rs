//! Invoice data persisted by fatoora-core 0.5/0.6 renders to the same XML and
//! hash as it did under the version that wrote it.
use super::{FinalizedInvoice, InvoiceData, InvoiceTotals};

const V0_5_1: &str = include_str!("../../../tests/fixtures/serde/invoice-data-v0.5.1.json");
/// `hash()` of the v0.5.1 fixture under v0.5.1 (see `tests/legacy_serde.rs`).
const V0_5_1_HASH: &str = "zcgG7AnCQWejCWF1+FIkVctXxgiYy6DXPrHvQplBr84=";

/// `(case, JSON written by v0.6.0, XML v0.6.0 rendered from it, its hash)`.
const V0_6_0: [(&str, &str, &str, &str); 4] = [
    (
        "aggregates",
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-aggregates.json"),
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-aggregates.xml"),
        "uNC6NkYFp7LzESDmCuDwabxpfkdYUQbF6VMPHTfxrpE=",
    ),
    (
        "discount-only",
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-discount-only.json"),
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-discount-only.xml"),
        "Ht/+xPavTdw2Sn2DJNYSbFWi70cEoOfOD1yrHwX60XM=",
    ),
    (
        "reason-only",
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-reason-only.json"),
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-reason-only.xml"),
        "OZmSM2o7UtwQ2oiW7JLjfbQurgzN8Ek8B1vhEqVBlyU=",
    ),
    (
        "vat-rate",
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-vat-rate.json"),
        include_str!("../../../tests/fixtures/serde/invoice-data-v0.6.0-vat-rate.xml"),
        "+0XXYz9XRhbgtdAvESeThL8X4YWBirXRtT5CTec/VJg=",
    ),
];
const V0_6_0_BINCODE: &[u8] =
    include_bytes!("../../../tests/fixtures/serde/invoice-data-v0.6.0-aggregates.bincode");

/// Renders deserialized data the way the version that wrote it did: totals
/// recomputed from the data, XML serialized from the model.
fn render(data: InvoiceData) -> (String, String) {
    let totals = InvoiceTotals::from_data(&data).unwrap();
    let invoice = FinalizedInvoice { data, totals };
    let xml = invoice.to_xml().unwrap();
    let hash = invoice.hash().unwrap().as_str().to_owned();
    (xml, hash)
}

#[test]
fn v0_5_1_json_renders_unchanged() {
    let (_, hash) = render(serde_json::from_str(V0_5_1).unwrap());
    assert_eq!(hash, V0_5_1_HASH);
}

#[test]
fn v0_6_0_aggregates_render_unchanged() {
    for (case, json, xml, hash) in V0_6_0 {
        let data: InvoiceData = serde_json::from_str(json).unwrap();
        assert_eq!(render(data), (xml.to_owned(), hash.to_owned()), "{case}");
    }
}

#[test]
fn v0_6_0_bincode_renders_unchanged() {
    let data: InvoiceData = bincode::deserialize(V0_6_0_BINCODE).unwrap();
    let (_, json, xml, hash) = V0_6_0[0];
    assert_eq!(data, serde_json::from_str::<InvoiceData>(json).unwrap());
    assert_eq!(render(data), (xml.to_owned(), hash.to_owned()));
}
