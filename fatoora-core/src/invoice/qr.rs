//! QR payload generation and encoding.
use super::xml::{
    lexical,
    nav::{self, Element},
};
use base64ct::{Base64, Encoding};
use thiserror::Error;

/// Errors emitted when building or encoding QR payloads.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum QrCodeError {
    #[error("seller legal name is missing")]
    MissingSellerName,
    #[error("seller VAT ID is missing")]
    MissingSellerVat,
    #[error("TLV field {tag} exceeds 255 bytes (len={len})")]
    ValueTooLong { tag: u8, len: usize },
    #[error("QR code payload exceeds 700 characters once base64 encoded (len={len})")]
    EncodedTooLong { len: usize },
    #[error("invalid QR field: {0}")]
    InvalidField(String),
    #[error("QR XML error: {0}")]
    Xml(String),
}

impl QrCodeError {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        match self {
            Self::InvalidField(_)
            | Self::MissingSellerName
            | Self::MissingSellerVat
            | Self::ValueTooLong { .. }
            | Self::EncodedTooLong { .. } => crate::ErrorKind::InvalidInput,
            Self::Xml(_) => crate::ErrorKind::Xml,
        }
    }
}

/// QR result alias.
pub type QrResult<T> = std::result::Result<T, QrCodeError>;

/// QR payload used internally by signing. Read a completed payload with
/// [`super::SignedInvoice::qr_code`]. Typed signing serializes before constructing
/// QR fields; importing signed XML extracts its existing payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QrPayload {
    applicability: QrApplicability,
    seller_name: String,
    seller_vat: String,
    timestamp: String,
    total_with_vat: String,
    total_vat: String,
    invoice_hash: Option<String>,
    signature: Option<String>,
    public_key: Option<String>,
    zatca_key_signature: Option<String>,
}

/// Phase-two stamp applicability. A local standard stamp is not clearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum QrApplicability {
    Standard,
    Simplified,
}

impl QrApplicability {
    pub(crate) fn parse(code: &str) -> QrResult<Self> {
        if code.len() != 7
            || !code.as_bytes()[2..]
                .iter()
                .all(|b| matches!(b, b'0' | b'1'))
        {
            return Err(QrCodeError::InvalidField("InvoiceTypeCode@name".into()));
        }
        match &code[..2] {
            "01" => Ok(Self::Standard),
            "02" => Ok(Self::Simplified),
            _ => Err(QrCodeError::InvalidField("InvoiceTypeCode@name".into())),
        }
    }

    pub(crate) fn from_xml(xml: &str) -> QrResult<Self> {
        let doc = lexical::document(xml).map_err(|e| QrCodeError::Xml(e.to_string()))?;
        Self::parse(
            &Element::invoice(&doc)
                .basic("InvoiceTypeCode")
                .attribute("name")
                .unwrap_or_default(),
        )
    }

    pub(crate) fn last_tag(self) -> u8 {
        match self {
            Self::Standard => 8,
            Self::Simplified => 9,
        }
    }

    pub(crate) fn check_tags(self, tags: &std::collections::BTreeMap<u8, Vec<u8>>) -> QrResult<()> {
        if tags.len() != self.last_tag() as usize
            || !(1..=self.last_tag()).all(|t| tags.contains_key(&t))
        {
            return Err(QrCodeError::InvalidField(format!(
                "expected tags 1 through {}",
                self.last_tag()
            )));
        }
        Ok(())
    }
}

/// Decode bounded, ordered, nonempty TLV fields without inferring a subtype.
pub(crate) fn decode_tlv(encoded: &str) -> QrResult<std::collections::BTreeMap<u8, Vec<u8>>> {
    if encoded.len() > 700 {
        return Err(QrCodeError::EncodedTooLong { len: encoded.len() });
    }
    let raw =
        Base64::decode_vec(encoded).map_err(|_| QrCodeError::InvalidField("QR Base64".into()))?;
    let mut tags = std::collections::BTreeMap::new();
    let mut at = 0;
    let mut previous = 0;
    while at < raw.len() {
        if raw.len() - at < 2 {
            return Err(QrCodeError::InvalidField("truncated TLV header".into()));
        }
        let tag = raw[at];
        let len = raw[at + 1] as usize;
        at += 2;
        if tag <= previous || tag > 9 || len == 0 || len > raw.len() - at {
            return Err(QrCodeError::InvalidField(
                "TLV order, number or length".into(),
            ));
        }
        tags.insert(tag, raw[at..at + len].to_vec());
        at += len;
        previous = tag;
    }
    Ok(tags)
}

fn binary_field(tag: u8, value: &str) -> QrResult<Vec<u8>> {
    // Bound before Base64 allocation, including deliberately oversized inputs.
    if value.len() > 344 {
        return Err(QrCodeError::ValueTooLong {
            tag,
            len: value.len() / 4 * 3,
        });
    }
    let bytes = Base64::decode_vec(value)
        .map_err(|_| QrCodeError::InvalidField(format!("tag {tag} Base64")))?;
    if bytes.is_empty() {
        return Err(QrCodeError::InvalidField(format!("empty tag {tag}")));
    }
    Ok(bytes)
}

#[derive(Clone, Copy)]
pub(crate) enum QrField {
    SellerName,
    SellerVat,
    IssueDate,
    IssueTime,
    TotalWithVat,
    TotalVat,
}

impl QrField {
    pub(crate) fn normalize(self, value: &str) -> &str {
        match self {
            Self::SellerName => value,
            Self::SellerVat
            | Self::IssueDate
            | Self::IssueTime
            | Self::TotalWithVat
            | Self::TotalVat => value.trim_matches(|c| matches!(c, ' ' | '\t' | '\r' | '\n')),
        }
    }
}

impl QrPayload {
    pub(crate) fn from_xml(xml: &str) -> QrResult<Self> {
        let doc = lexical::document(xml).map_err(|e| QrCodeError::Xml(e.to_string()))?;
        let invoice = Element::invoice(&doc);
        let seller = nav::seller_party(invoice);
        let seller_name = required_value(
            seller
                .aggregate("PartyLegalEntity")
                .basic("RegistrationName")
                .content(),
            "seller name",
            QrField::SellerName,
        )?;
        let seller_vat = required_value(
            seller
                .aggregate("PartyTaxScheme")
                .basic("CompanyID")
                .content(),
            "seller VAT",
            QrField::SellerVat,
        )?;
        let issue_date = required_value(
            invoice.basic("IssueDate").content(),
            "issue date",
            QrField::IssueDate,
        )?;
        let issue_time = required_value(
            invoice.basic("IssueTime").content(),
            "issue time",
            QrField::IssueTime,
        )?;
        let total_with_vat = required_value(
            invoice
                .aggregate("LegalMonetaryTotal")
                .basic("PayableAmount")
                .content(),
            "total with VAT",
            QrField::TotalWithVat,
        )?;
        let total_vat = required_value(
            nav::qr_tax_amount(invoice).content(),
            "total VAT",
            QrField::TotalVat,
        )?;

        for value in [&total_with_vat, &total_vat] {
            super::validation::check_qr_decimal(value).map_err(QrCodeError::InvalidField)?;
        }
        Ok(Self {
            applicability: QrApplicability::parse(
                &invoice
                    .basic("InvoiceTypeCode")
                    .attribute("name")
                    .unwrap_or_default(),
            )?,
            seller_name,
            seller_vat,
            timestamp: format!("{issue_date}T{issue_time}"),
            total_with_vat,
            total_vat,
            invoice_hash: None,
            signature: None,
            public_key: None,
            zatca_key_signature: None,
        })
    }

    pub(crate) fn with_signing_parts(
        mut self,
        invoice_hash: Option<&str>,
        signature: Option<&str>,
        public_key: Option<&str>,
        zatca_key_signature: Option<&str>,
    ) -> Self {
        self.invoice_hash = invoice_hash.map(|value| value.to_string());
        self.signature = signature.map(|value| value.to_string());
        self.public_key = public_key.map(|value| value.to_string());
        self.zatca_key_signature = zatca_key_signature.map(|value| value.to_string());
        self
    }

    pub(crate) fn encode(&self) -> QrResult<String> {
        let stamped = self.invoice_hash.is_some()
            || self.signature.is_some()
            || self.public_key.is_some()
            || self.zatca_key_signature.is_some();
        if stamped
            && (self.invoice_hash.is_none()
                || self.signature.is_none()
                || self.public_key.is_none()
                || (self.applicability == QrApplicability::Simplified
                    && self.zatca_key_signature.is_none()))
        {
            return Err(QrCodeError::InvalidField(
                "incomplete stamp material".into(),
            ));
        }
        if self.applicability == QrApplicability::Standard && self.zatca_key_signature.is_some() {
            return Err(QrCodeError::InvalidField(
                "standard invoice prohibits tag 9".into(),
            ));
        }
        let mut tlv = TlvBuilder::new();
        tlv.push_str(1, &self.seller_name)?;
        tlv.push_str(2, &self.seller_vat)?;
        tlv.push_str(3, &self.timestamp)?;
        tlv.push_str(4, &self.total_with_vat)?;
        tlv.push_str(5, &self.total_vat)?;

        if let Some(hash) = self.invoice_hash.as_deref() {
            tlv.push_bytes(6, hash.as_bytes())?;
        }
        if let Some(sig) = self.signature.as_deref() {
            tlv.push_bytes(7, sig.as_bytes())?;
        }
        if let Some(pk) = self.public_key.as_deref() {
            tlv.push_bytes(8, &binary_field(8, pk)?)?;
        }
        if let Some(stamp_sig) = self.zatca_key_signature.as_deref() {
            tlv.push_bytes(9, &binary_field(9, stamp_sig)?)?;
        }

        tlv.finish()
    }
}

struct TlvBuilder {
    bytes: Vec<u8>,
}

impl TlvBuilder {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    fn push_str(&mut self, tag: u8, value: &str) -> QrResult<()> {
        self.push_bytes(tag, value.as_bytes())
    }

    fn push_bytes(&mut self, tag: u8, value: &[u8]) -> QrResult<()> {
        if value.len() > u8::MAX as usize {
            return Err(QrCodeError::ValueTooLong {
                tag,
                len: value.len(),
            });
        }
        self.bytes.push(tag);
        self.bytes.push(value.len() as u8);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    fn finish(self) -> QrResult<String> {
        let encoded = Base64::encode_string(&self.bytes);
        if encoded.len() > 700 {
            return Err(QrCodeError::EncodedTooLong { len: encoded.len() });
        }
        Ok(encoded)
    }
}

fn required_value(value: Option<String>, label: &str, field: QrField) -> QrResult<String> {
    let value = value.ok_or_else(|| QrCodeError::Xml(format!("Missing {label} in invoice XML")))?;
    let value = field.normalize(&value);
    if value.trim().is_empty() {
        return Err(QrCodeError::Xml(format!("Empty {label} in invoice XML")));
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::invoice::sign::SignedProperties;
    use crate::invoice::{
        Address, CountryCode, FinalizedInvoice, InvoiceBuilder, InvoiceSubType, InvoiceType,
        LineItem, Party, SellerRole, VatCategory,
    };
    use base64ct::{Base64, Encoding};

    fn sample_invoice() -> FinalizedInvoice {
        sample_invoice_with_items(vec![
            LineItem::new(
                "Item",
                crate::Decimal::from(1),
                "PCE",
                crate::Decimal::from(100),
                crate::Decimal::from(15),
                VatCategory::Standard,
            )
            .unwrap(),
        ])
    }

    fn sample_invoice_with_items(items: Vec<LineItem>) -> FinalizedInvoice {
        let seller = Party::<SellerRole>::new(
            "Acme Inc".into(),
            Address {
                province: None,
                country_code: CountryCode::parse("SAU").expect("country code"),
                city: "Riyadh".into(),
                street: "King Fahd".into(),
                additional_street: None,
                building_number: "1234".into(),
                additional_number: Some("5678".into()),
                postal_code: "12222".into(),
                district: None,
            },
            "301121971500003",
            None,
        )
        .expect("valid seller");

        let mut builder = InvoiceBuilder::new(InvoiceType::Tax(InvoiceSubType::Simplified));
        builder = builder
            .id("INV-1")
            .uuid("uuid-123")
            .issue_datetime("2024-01-01T12:30:00Z")
            .currency("SAR")
            .previous_invoice_hash("hash")
            .invoice_counter(0)
            .seller(seller)
            .payment_means_code("10")
            .vat_category(VatCategory::Standard);
        for item in items {
            builder = builder.line_item(item);
        }
        builder.build().expect("build sample invoice")
    }

    fn decode_test_tlv(bytes: &[u8]) -> Vec<(u8, Vec<u8>)> {
        let mut entries = Vec::new();
        let mut idx = 0;
        while idx < bytes.len() {
            let tag = bytes[idx];
            let len = bytes[idx + 1] as usize;
            let start = idx + 2;
            let end = start + len;
            entries.push((tag, bytes[start..end].to_vec()));
            idx = end;
        }
        entries
    }

    #[test]
    fn qr_uses_document_vat_instead_of_rounded_line_vat() {
        let invoice = sample_invoice_with_items(vec![
            LineItem::new(
                "tiny",
                crate::Decimal::from(1),
                "PCE",
                crate::Decimal::parse("0.03").unwrap(),
                crate::Decimal::from(15),
                VatCategory::Standard
            )
            .unwrap();
            3
        ]);
        let payload = QrPayload::from_xml(&invoice.to_xml().unwrap()).unwrap();
        assert_eq!(payload.total_vat, "0.01");
        assert_eq!(payload.total_with_vat, "0.10");
    }

    #[test]
    fn qr_code_contains_all_required_tags() {
        let invoice = sample_invoice();
        let public_key_bytes = b"public-key";
        let public_key_b64 = Base64::encode_string(public_key_bytes);
        let stamp_bytes = b"stamp";
        let stamp_b64 = Base64::encode_string(stamp_bytes);
        let signing = SignedProperties::from_qr_parts(
            "hash==",
            "signature==",
            &public_key_b64,
            Some(&stamp_b64),
        );

        let signed_xml = invoice.to_xml().expect("serialize invoice");
        let qr = QrPayload::from_xml(&signed_xml)
            .unwrap()
            .with_signing_parts(
                Some(signing.invoice_hash()),
                Some(signing.signature()),
                Some(signing.public_key()),
                signing.zatca_key_signature(),
            )
            .encode()
            .unwrap();
        assert!(qr.len() < 700);

        let raw = Base64::decode_vec(&qr).expect("base64 decode");
        let entries = decode_test_tlv(&raw);
        let expected = vec![
            (1, b"Acme Inc".to_vec()),
            (2, b"301121971500003".to_vec()),
            (3, b"2024-01-01T12:30:00Z".to_vec()),
            (4, b"115.00".to_vec()),
            (5, b"15.00".to_vec()),
            (6, b"hash==".to_vec()),
            (7, b"signature==".to_vec()),
            (8, public_key_bytes.to_vec()),
            (9, stamp_bytes.to_vec()),
        ];
        assert_eq!(entries, expected);
    }

    #[test]
    fn qr_code_errors_on_large_field() {
        let invoice = sample_invoice();
        let oversized = "a".repeat(300);
        let pk_b64 = Base64::encode_string(b"pk");
        let signing = SignedProperties::from_qr_parts(&oversized, "sig", &pk_b64, Some(&pk_b64));

        let signed_xml = invoice.to_xml().expect("serialize invoice");
        match QrPayload::from_xml(&signed_xml)
            .unwrap()
            .with_signing_parts(
                Some(signing.invoice_hash()),
                Some(signing.signature()),
                Some(signing.public_key()),
                signing.zatca_key_signature(),
            )
            .encode()
        {
            Err(QrCodeError::ValueTooLong { tag, .. }) => assert_eq!(tag, 6),
            other => panic!("expected ValueTooLong error, got {:?}", other),
        }
    }

    #[test]
    fn qr_code_errors_when_payload_too_long() {
        let invoice = sample_invoice();
        let long_value = "a".repeat(200);
        let long_key_bytes = vec![b'k'; 200];
        let long_key_b64 = Base64::encode_string(&long_key_bytes);
        let signing = SignedProperties::from_qr_parts(
            &long_value,
            &long_value,
            &long_key_b64,
            Some(&long_key_b64),
        );

        let signed_xml = invoice.to_xml().expect("serialize invoice");
        match QrPayload::from_xml(&signed_xml)
            .unwrap()
            .with_signing_parts(
                Some(signing.invoice_hash()),
                Some(signing.signature()),
                Some(signing.public_key()),
                signing.zatca_key_signature(),
            )
            .encode()
        {
            Err(QrCodeError::EncodedTooLong { .. }) => {}
            other => panic!("expected EncodedTooLong error, got {:?}", other),
        }
    }
    #[test]
    fn malformed_signing_material_is_rejected_without_panics() {
        let xml = sample_invoice().to_xml().unwrap();
        let payload = QrPayload::from_xml(&xml).unwrap();
        let key = Base64::encode_string(b"key");
        for (pk, stamp) in [
            ("%%%", Some(key.as_str())),
            (key.as_str(), Some("%%%")),
            (key.as_str(), None),
            ("", Some(key.as_str())),
        ] {
            assert!(
                payload
                    .clone()
                    .with_signing_parts(Some("hash"), Some("sig"), Some(pk), stamp)
                    .encode()
                    .is_err()
            );
        }
        let large = Base64::encode_string(&[0; 256]);
        assert!(matches!(
            payload
                .with_signing_parts(Some("hash"), Some("sig"), Some(&key), Some(&large))
                .encode(),
            Err(QrCodeError::ValueTooLong { tag: 9, .. })
        ));
    }

    #[test]
    fn subtype_and_tlv_boundaries_are_explicit() {
        for code in [
            "", "03", "0300000", "0200002", "02000000", "é00000", "0é0000",
        ] {
            assert!(QrApplicability::parse(code).is_err(), "{code}");
        }
        for raw in [
            vec![],
            vec![1],
            vec![1, 0],
            vec![1, 2, 65],
            vec![1, 1, 65, 1, 1, 66],
            vec![2, 1, 65, 1, 1, 66],
            vec![10, 1, 65],
        ] {
            let result = decode_tlv(&Base64::encode_string(&raw));
            assert!(
                result.is_err()
                    || QrApplicability::Simplified
                        .check_tags(&result.unwrap())
                        .is_err()
            );
        }
        assert!(decode_tlv(&"A".repeat(704)).is_err());
        let tags = (1..=8).map(|t| (t, vec![65])).collect();
        QrApplicability::Standard.check_tags(&tags).unwrap();
        assert!(QrApplicability::Simplified.check_tags(&tags).is_err());
        let mut tags = tags;
        tags.insert(9, vec![65]);
        assert!(QrApplicability::Standard.check_tags(&tags).is_err());
    }

    #[test]
    fn xml_qr_preserves_decimal_text_and_validates_syntax_and_limits() {
        let input = sample_invoice().to_xml().unwrap();
        for value in [
            "300",
            "300.0",
            "300.00",
            "300.01",
            "0.00",
            "45.0",
            "45.00",
            "123456789012345678901234.56",
        ] {
            let xml = input
                .replace(
                    ">115.00</cbc:PayableAmount>",
                    &format!("> \t{value}\n</cbc:PayableAmount>"),
                )
                .replace(
                    ">15.00</cbc:TaxAmount>",
                    &format!(">{value}</cbc:TaxAmount>"),
                );
            let payload = QrPayload::from_xml(&xml).unwrap();
            assert_eq!(payload.total_with_vat, value);
            assert_eq!(payload.total_vat, value);
        }
        for value in ["NaN", "1e2", "1.2.3", "", "１２"] {
            let xml = input.replace(
                ">115.00</cbc:PayableAmount>",
                &format!(">{value}</cbc:PayableAmount>"),
            );
            assert!(QrPayload::from_xml(&xml).is_err(), "{value}");
        }
        let xml = input.replace(
            ">115.00</cbc:PayableAmount>",
            &format!(">{}</cbc:PayableAmount>", "1".repeat(4097)),
        );
        assert!(QrPayload::from_xml(&xml).is_err());
    }
    #[test]
    fn qr_uses_document_currency_vat_even_when_accounting_total_is_first() {
        let input = sample_invoice().to_xml().unwrap();
        let doc = lexical::document(&input).unwrap();
        let first = nav::tax_totals(Element::invoice(&doc))
            .next()
            .unwrap()
            .0
            .unwrap()
            .range();
        let mut xml = input.clone();
        xml.insert_str(
            first.start,
            "<cac:TaxTotal><cbc:TaxAmount currencyID=\"USD\">999.00</cbc:TaxAmount></cac:TaxTotal>",
        );
        assert_eq!(QrPayload::from_xml(&xml).unwrap().total_vat, "15.00");
    }
}
