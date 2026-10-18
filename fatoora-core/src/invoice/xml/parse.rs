//! XML parsing for invoices.
use super::{
    lexical,
    nav::{self, Element},
};
use crate::Decimal;
use crate::invoice::sign::SignedProperties;
use crate::invoice::xml::constants::{DS_NS, XADES_NS};
use crate::invoice::{
    Address, CountryCode, CurrencyCode, FinalizedInvoice, InvoiceBuilder, InvoiceDate,
    InvoiceFlags, InvoiceSubType, InvoiceTimestamp, InvoiceType, LineItem, OriginalInvoiceRef,
    OtherId, Party, SellerRole, SignedInvoice, VatCategory,
};
use base64ct::{Base64, Encoding};
use chrono::{NaiveDateTime, NaiveTime};
use roxmltree::Document;
use std::path::Path;
use thiserror::Error;

/// Errors emitted while parsing XML invoices.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseError {
    #[error("{0}")]
    Qr(#[from] crate::invoice::QrCodeError),
    #[error("{0}")]
    Invoice(#[from] crate::invoice::InvoiceError),
    #[error("{0}")]
    Decimal(#[from] crate::DecimalError),
    #[error("failed to read {path}: {source}")]
    Io {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("XML parse error: {0}")]
    XmlParse(String),
    #[error("XPath error: {0}")]
    XPath(String),
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: &'static str, value: String },
}

impl ParseError {
    /// Shared classification used by bindings.
    pub fn kind(&self) -> crate::ErrorKind {
        match self {
            Self::Qr(error) => error.kind(),
            Self::Invoice(error) => error.kind(),
            Self::Decimal(error) => error.kind(),
            Self::Io { .. } => crate::ErrorKind::Io,
            Self::XmlParse(_) => crate::ErrorKind::Xml,
            Self::XPath(_) => crate::ErrorKind::Parse,
            Self::MissingField(_) | Self::InvalidValue { .. } => crate::ErrorKind::InvalidInput,
        }
    }
}

/// Parse supported invoice fields and check the modeled supplied amounts.
///
/// Runs wrapper constructors and builder field checks. It does not validate the
/// XSD, run full business rules, or verify signatures.
///
/// # Examples
/// ```rust,no_run
/// use fatoora_core::invoice::xml::parse::parse_finalized_invoice_xml;
///
/// let xml = std::fs::read_to_string("invoice.xml")?;
/// let invoice = parse_finalized_invoice_xml(&xml)?;
/// # let _ = invoice;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
/// Returns [`ParseError`] if the XML is invalid or required fields are missing.
pub fn parse_finalized_invoice_xml(xml: &str) -> Result<FinalizedInvoice, ParseError> {
    let doc = lexical::document(xml).map_err(|e| ParseError::XmlParse(format!("{e:?}")))?;
    parse_finalized_invoice_doc(&doc)
}

/// File variant of [`parse_finalized_invoice_xml`], with the same guarantees.
///
/// # Errors
/// Returns [`ParseError`] if the file cannot be read or the XML is invalid.
pub fn parse_finalized_invoice_xml_file(
    path: impl AsRef<Path>,
) -> Result<FinalizedInvoice, ParseError> {
    let path = path.as_ref();
    let xml = std::fs::read_to_string(path).map_err(|e| ParseError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    parse_finalized_invoice_xml(&xml)
}

/// Parse supported invoice fields and extract signature material.
///
/// Applies the same field and amount checks as [`parse_finalized_invoice_xml`].
/// Does not verify the signature, certificate trust, XSD, or full business rules.
/// A successful parse must not be treated as proof of authenticity.
///
/// # Errors
/// Returns [`ParseError`] if the XML is invalid or required fields are missing.
pub fn parse_signed_invoice_xml(xml: &str) -> Result<SignedInvoice, ParseError> {
    let doc = lexical::document(xml).map_err(|e| ParseError::XmlParse(format!("{e:?}")))?;
    let finalized = parse_finalized_invoice_doc(&doc)?;
    let signing = parse_signed_properties(&doc)?;
    let signed = finalized
        .sign_with_bundle(signing, xml.to_string())
        .map_err(ParseError::from)?;
    Ok(signed)
}

/// File variant of [`parse_signed_invoice_xml`]; does not verify authenticity.
///
/// # Errors
/// Returns [`ParseError`] if the file cannot be read or the XML is invalid.
pub fn parse_signed_invoice_xml_file(path: impl AsRef<Path>) -> Result<SignedInvoice, ParseError> {
    let path = path.as_ref();
    let xml = std::fs::read_to_string(path).map_err(|e| ParseError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    parse_signed_invoice_xml(&xml)
}

fn parse_finalized_invoice_doc(doc: &Document<'_>) -> Result<FinalizedInvoice, ParseError> {
    let ctx = Element::invoice(doc);
    check_model_structure(ctx)?;

    let id = text_required(ctx.basic("ID").content(), "ID")?;
    let uuid = text_required(ctx.basic("UUID").content(), "UUID")?;
    let issue_date = text_required(ctx.basic("IssueDate").content(), "IssueDate")?;
    let issue_time = text_required(ctx.basic("IssueTime").content(), "IssueTime")?;
    let invoice_type_code =
        text_required(ctx.basic("InvoiceTypeCode").content(), "InvoiceTypeCode")?;
    let invoice_type_name = text_required(
        ctx.basic("InvoiceTypeCode").attribute("name"),
        "InvoiceTypeCode@name",
    )?;
    let currency_code = text_required(
        ctx.basic("DocumentCurrencyCode").content(),
        "DocumentCurrencyCode",
    )?;
    let payment_means_code = text_required(
        ctx.aggregate("PaymentMeans")
            .basic("PaymentMeansCode")
            .content(),
        "PaymentMeansCode",
    )?;

    let issue_datetime = parse_datetime(&issue_date, &issue_time)?;
    let currency_code =
        CurrencyCode::parse(&currency_code).map_err(|_| ParseError::InvalidValue {
            field: "DocumentCurrencyCode",
            value: currency_code.clone(),
        })?;
    let original_ref = if invoice_type_code == "381" || invoice_type_code == "383" {
        Some(parse_original_ref(&ctx)?)
    } else {
        None
    };
    let reason = content_optional(
        ctx.aggregate("PaymentMeans")
            .basic("InstructionNote")
            .content(),
    )?
    .or_else(|| content_optional(ctx.basic("Note").content()).ok().flatten());
    let invoice_type = parse_invoice_type(
        &invoice_type_name,
        &invoice_type_code,
        original_ref,
        reason.unwrap_or_default(),
    )?;

    let seller = parse_seller(&ctx)?;
    let buyer = parse_buyer(ctx)?;

    let previous_invoice_hash = text_required(
        nav::document_reference(ctx, "PIH")
            .aggregate("Attachment")
            .basic("EmbeddedDocumentBinaryObject")
            .content(),
        "PIH",
    )?;

    let vat_category = parse_vat_category(&ctx)?;
    let line_items = parse_line_items(&ctx)?;
    let invoice_counter_str = text_required(
        nav::document_reference(ctx, "ICV").basic("UUID").content(),
        "ICV",
    )?;
    let invoice_counter =
        invoice_counter_str
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidValue {
                field: "ICV",
                value: invoice_counter_str,
            })?;

    let mut builder = InvoiceBuilder::new(invoice_type);
    builder = builder
        .id(id)
        .uuid(uuid)
        .issue_datetime(issue_datetime)
        .currency(currency_code.as_str())
        .previous_invoice_hash(previous_invoice_hash)
        .invoice_counter(invoice_counter)
        .seller(seller)
        .payment_means_code(payment_means_code)
        .vat_category(vat_category)
        .flags(InvoiceFlags::from_bits_truncate(
            invoice_type_name.as_bytes()[2..]
                .iter()
                .enumerate()
                .fold(0, |bits, (index, digit)| bits | ((digit - b'0') << index)),
        ));
    if let Some(buyer) = buyer {
        builder = builder.buyer(buyer);
    }
    let delivery = ctx.aggregate("Delivery");
    if delivery.0.is_some() {
        let date = |field| -> Result<Option<InvoiceDate>, ParseError> {
            delivery
                .basic(field)
                .content()
                .map(|value| InvoiceDate::parse(value).map_err(ParseError::from))
                .transpose()
        };
        builder = builder.delivery(crate::invoice::Delivery {
            actual_date: date("ActualDeliveryDate")?,
            latest_date: date("LatestDeliveryDate")?,
        });
    }
    for item in line_items {
        builder = builder.line_item(item);
    }
    if let Some(note) = content_optional(ctx.basic("Note").content())? {
        let note = match ctx.basic("Note").attribute("languageID") {
            Some(language) => crate::invoice::InvoiceNote::new(language, note),
            None => crate::invoice::InvoiceNote::without_language(note),
        };
        builder = builder.note(note);
    }
    if let Some(instruction) = ctx
        .aggregate("PaymentMeans")
        .basic("InstructionNote")
        .content()
    {
        builder = builder.payment_instruction(instruction);
    }
    let mut discount = Decimal::ZERO;
    let mut charge = Decimal::ZERO;

    for adjustment in nav::allowance_charges(ctx) {
        let amount = decimal_required(adjustment.basic("Amount").content(), "AdjustmentAmount")?;
        let category = text_required(
            adjustment.aggregate("TaxCategory").basic("ID").content(),
            "AdjustmentVatCategory",
        )?;
        let rate = decimal_required(
            adjustment
                .aggregate("TaxCategory")
                .basic("Percent")
                .content(),
            "AdjustmentVatRate",
        )?;
        let category = match category.as_str() {
            "S" => VatCategory::Standard,
            "Z" => VatCategory::Zero,
            "E" => VatCategory::Exempt,
            "O" => VatCategory::OutOfScope,
            _ => {
                return Err(ParseError::InvalidValue {
                    field: "AdjustmentVatCategory",
                    value: category,
                });
            }
        };
        let categories: Vec<_> = adjustment
            .children(super::constants::CAC_NS, "TaxCategory")
            .collect();
        if let Some(first) = categories.first() {
            for other in categories.iter().skip(1) {
                let same_id = text_required(first.basic("ID").content(), "AdjustmentVatCategory")?
                    == text_required(other.basic("ID").content(), "AdjustmentVatCategory")?;
                let other_rate =
                    decimal_required(other.basic("Percent").content(), "AdjustmentVatRate")?;
                if !same_id
                    || rate != other_rate
                    || parse_exemption(*first) != parse_exemption(*other)
                {
                    return Err(ParseError::InvalidValue {
                        field: "AllowanceCharge",
                        value: "distinct VAT classifications on one adjustment are not supported"
                            .into(),
                    });
                }
            }
        }
        let is_charge = text_required(
            adjustment.basic("ChargeIndicator").content(),
            "ChargeIndicator",
        )?;
        match is_charge.as_str() {
            "true" | "1" => charge = charge.add(amount).map_err(ParseError::from)?,
            "false" | "0" => discount = discount.add(amount).map_err(ParseError::from)?,
            _ => {
                return Err(ParseError::InvalidValue {
                    field: "ChargeIndicator",
                    value: is_charge,
                });
            }
        }
        if amount < Decimal::ZERO || amount.scale() > 2 {
            return Err(ParseError::InvalidValue {
                field: "AdjustmentAmount",
                value: amount.to_string(),
            });
        }
        builder = builder.adjustment(crate::invoice::AllowanceCharge {
            tax_category_count: adjustment
                .children(super::constants::CAC_NS, "TaxCategory")
                .count(),
            charge: matches!(is_charge.as_str(), "true" | "1"),
            amount,
            reason: adjustment.basic("AllowanceChargeReason").content(),
            reason_code: adjustment.basic("AllowanceChargeReasonCode").content(),
            category,
            rate,
            exemption: parse_exemption(adjustment.aggregate("TaxCategory")),
        });
    }
    if let Some(code) = ctx.basic("TaxCurrencyCode").content() {
        let tax_currency = CurrencyCode::parse(code)?;
        if tax_currency != currency_code {
            let totals: Vec<_> = nav::tax_totals(ctx)
                .filter(|t| {
                    t.basic("TaxAmount")
                        .attribute("currencyID")
                        .and_then(|code| CurrencyCode::parse(code).ok())
                        .as_ref()
                        == Some(&tax_currency)
                })
                .collect();
            if totals.len() != 1 || totals[0].aggregate("TaxSubtotal").0.is_some() {
                return Err(ParseError::InvalidValue {
                    field: "TaxCurrencyCode",
                    value: "expected one accounting-currency tax total without subtotals".into(),
                });
            }
            let amount =
                decimal_required(totals[0].basic("TaxAmount").content(), "TaxCurrencyAmount")?;
            if amount.scale() > 2 || amount < Decimal::ZERO {
                return Err(ParseError::InvalidValue {
                    field: "TaxCurrencyAmount",
                    value: amount.to_string(),
                });
            }
            builder = builder.tax_currency(tax_currency, amount);
        }
    }
    let mut invoice = builder.build().map_err(ParseError::from)?;
    // Validate the amounts we model before returning a finalized imported invoice.
    for (field, expected) in [
        ("LineExtensionAmount", invoice.totals().line_extension()),
        ("TaxExclusiveAmount", invoice.totals().taxable_amount()),
        (
            "TaxInclusiveAmount",
            invoice.totals().tax_inclusive_amount(),
        ),
        ("AllowanceTotalAmount", discount),
        ("ChargeTotalAmount", charge),
    ] {
        if let Some(value) =
            content_optional(ctx.aggregate("LegalMonetaryTotal").basic(field).content())?
        {
            check_amount(field, &value, expected)?;
        }
    }
    let mut seen = Vec::new();
    for tax_total in nav::tax_totals(ctx).filter(|total| {
        total
            .basic("TaxAmount")
            .attribute("currencyID")
            .and_then(|code| CurrencyCode::parse(code).ok())
            .as_ref()
            == Some(&currency_code)
    }) {
        let value = text_required(tax_total.basic("TaxAmount").content(), "TaxAmount")?;
        check_amount("TaxAmount", &value, invoice.totals().tax_amount())?;
        for subtotal in tax_total.children(super::constants::CAC_NS, "TaxSubtotal") {
            let category = text_required(
                subtotal.aggregate("TaxCategory").basic("ID").content(),
                "VatCategory",
            )?;
            let rate = decimal_required(
                subtotal.aggregate("TaxCategory").basic("Percent").content(),
                "VatRate",
            )?;
            let group = invoice
                .totals
                .groups
                .iter_mut()
                .find(|g| {
                    let code = match g.category() {
                        VatCategory::Standard => "S",
                        VatCategory::Zero => "Z",
                        VatCategory::Exempt => "E",
                        VatCategory::OutOfScope => "O",
                    };
                    category == code && rate == g.rate()
                })
                .ok_or_else(|| ParseError::InvalidValue {
                    field: "TaxSubtotal",
                    value: format!("unknown category/rate {category}/{rate}"),
                })?;
            if seen.contains(&(category.clone(), rate)) {
                return Err(ParseError::InvalidValue {
                    field: "TaxSubtotal",
                    value: "duplicate category/rate".into(),
                });
            }
            group.exemption = parse_exemption(subtotal.aggregate("TaxCategory"));
            seen.push((category, rate));
            for (field, expected) in [
                ("TaxableAmount", group.taxable_amount()),
                ("TaxAmount", group.tax_amount()),
            ] {
                let value = text_required(subtotal.basic(field).content(), field)?;
                check_amount(field, &value, expected)?;
            }
        }
    }
    let mut prepaid = Decimal::ZERO;
    let mut rounding = Decimal::ZERO;
    for (field, dest) in [
        ("PrepaidAmount", &mut prepaid),
        ("PayableRoundingAmount", &mut rounding),
    ] {
        if let Some(value) =
            content_optional(ctx.aggregate("LegalMonetaryTotal").basic(field).content())?
        {
            *dest = value.parse().map_err(ParseError::from)?;
            if dest.scale() > 2 {
                return Err(ParseError::InvalidValue { field, value });
            }
        }
    }
    if prepaid < Decimal::ZERO {
        return Err(ParseError::InvalidValue {
            field: "PrepaidAmount",
            value: prepaid.to_string(),
        });
    }
    let payable = invoice
        .totals()
        .tax_inclusive_amount()
        .sub(prepaid)
        .and_then(|v| v.add(rounding))
        .map_err(ParseError::from)?;
    if let Some(value) = content_optional(
        ctx.aggregate("LegalMonetaryTotal")
            .basic("PayableAmount")
            .content(),
    )? {
        check_amount("PayableAmount", &value, payable)?;
    }
    invoice.totals.prepaid_amount = prepaid;
    invoice.totals.payable_rounding_amount = rounding;
    invoice.totals.payable_amount = payable;
    Ok(invoice)
}

// Reject ambiguous occurrences of modeled singleton fields before first-child navigation.
// Signature extensions and unrelated UBL content remain outside the typed model.
// Both public import paths call lexical::document first; its 128-level XML depth
// limit bounds this visitor before it can recurse into recognized aggregates.
fn check_model_structure(root: Element<'_, '_>) -> Result<(), ParseError> {
    use super::constants::{CAC_NS, CBC_NS};
    fn visit(element: Element<'_, '_>) -> Result<(), ParseError> {
        let Some(node) = element.0 else {
            return Ok(());
        };
        if nav::is(node, CAC_NS, "TaxScheme")
            && element
                .basic("ID")
                .content()
                .is_some_and(|id| id.trim() != "VAT")
        {
            return Err(ParseError::InvalidValue {
                field: "TaxScheme",
                value: "only VAT is supported".into(),
            });
        }
        let mut seen = std::collections::HashSet::new();
        for child in node.children().filter(|n| n.is_element()) {
            let name = child.tag_name();
            let basic = name.namespace() == Some(CBC_NS);
            let aggregate = name.namespace() == Some(CAC_NS);
            if !basic && !aggregate {
                continue;
            }
            let supported = if basic {
                matches!(
                    name.name(),
                    "ID" | "UUID"
                        | "IssueDate"
                        | "IssueTime"
                        | "InvoiceTypeCode"
                        | "DocumentTypeCode"
                        | "ProfileID"
                        | "Note"
                        | "DocumentCurrencyCode"
                        | "TaxCurrencyCode"
                        | "PaymentMeansCode"
                        | "InstructionNote"
                        | "ActualDeliveryDate"
                        | "LatestDeliveryDate"
                        | "CompanyID"
                        | "RegistrationName"
                        | "StreetName"
                        | "AdditionalStreetName"
                        | "BuildingNumber"
                        | "PlotIdentification"
                        | "CitySubdivisionName"
                        | "CityName"
                        | "PostalZone"
                        | "IdentificationCode"
                        | "Name"
                        | "InvoicedQuantity"
                        | "PriceAmount"
                        | "Percent"
                        | "TaxExemptionReasonCode"
                        | "TaxExemptionReason"
                        | "ChargeIndicator"
                        | "AllowanceChargeReason"
                        | "AllowanceChargeReasonCode"
                        | "Amount"
                        | "TaxAmount"
                        | "TaxableAmount"
                        | "RoundingAmount"
                        | "LineExtensionAmount"
                        | "TaxExclusiveAmount"
                        | "TaxInclusiveAmount"
                        | "AllowanceTotalAmount"
                        | "ChargeTotalAmount"
                        | "PrepaidAmount"
                        | "PayableRoundingAmount"
                        | "PayableAmount"
                        | "EmbeddedDocumentBinaryObject"
                )
            } else {
                matches!(
                    name.name(),
                    "AccountingSupplierParty"
                        | "AccountingCustomerParty"
                        | "Party"
                        | "PartyIdentification"
                        | "PartyTaxScheme"
                        | "PartyLegalEntity"
                        | "PostalAddress"
                        | "Country"
                        | "Delivery"
                        | "PaymentMeans"
                        | "BillingReference"
                        | "InvoiceDocumentReference"
                        | "DocumentReference"
                        | "AdditionalDocumentReference"
                        | "Attachment"
                        | "AllowanceCharge"
                        | "TaxCategory"
                        | "ClassifiedTaxCategory"
                        | "TaxScheme"
                        | "TaxTotal"
                        | "TaxSubtotal"
                        | "LegalMonetaryTotal"
                        | "InvoiceLine"
                        | "Item"
                        | "Price"
                )
            };
            if !supported {
                continue;
            }
            let repeated = aggregate
                && matches!(
                    name.name(),
                    "InvoiceLine"
                        | "TaxSubtotal"
                        | "AllowanceCharge"
                        | "AdditionalDocumentReference"
                        | "DocumentReference"
                )
                || (aggregate && name.name() == "TaxTotal" && node.tag_name().name() == "Invoice")
                || (aggregate
                    && name.name() == "TaxCategory"
                    && node.tag_name().name() == "AllowanceCharge");
            if !repeated && !seen.insert((name.namespace(), name.name())) {
                return Err(ParseError::InvalidValue {
                    field: "Cardinality",
                    value: format!(
                        "multiple {} in {} are not supported",
                        name.name(),
                        node.tag_name().name()
                    ),
                });
            }
            if basic && child.children().any(|n| n.is_element()) {
                return Err(ParseError::InvalidValue {
                    field: "Content",
                    value: format!("{} must contain text", name.name()),
                });
            }
            if aggregate {
                visit(Element(Some(child)))?;
            }
        }
        Ok(())
    }
    visit(root)?;
    if let Some(profile) = root.basic("ProfileID").content()
        && profile != "reporting:1.0"
    {
        return Err(ParseError::InvalidValue {
            field: "ProfileID",
            value: profile,
        });
    }
    if let Some(node) = root.0
        && let Some(document_currency) = root
            .basic("DocumentCurrencyCode")
            .content()
            .and_then(|code| CurrencyCode::parse(code).ok())
    {
        let tax_currency = root
            .basic("TaxCurrencyCode")
            .content()
            .map(CurrencyCode::parse)
            .transpose()?
            .unwrap_or_else(|| document_currency.clone());
        for n in node.descendants().filter(|n| n.is_element()) {
            if n.tag_name().namespace() != Some(CBC_NS) {
                continue;
            }
            let field = n.tag_name().name();
            if !matches!(
                field,
                "TaxAmount"
                    | "TaxableAmount"
                    | "LineExtensionAmount"
                    | "TaxExclusiveAmount"
                    | "TaxInclusiveAmount"
                    | "AllowanceTotalAmount"
                    | "ChargeTotalAmount"
                    | "PrepaidAmount"
                    | "PayableRoundingAmount"
                    | "PayableAmount"
                    | "PriceAmount"
                    | "RoundingAmount"
                    | "Amount"
            ) {
                continue;
            }
            // Scope this check to modeled amounts, excluding extension and prepayment-reference content.
            let parent = n.parent_element().unwrap();
            let direct_total =
                nav::is(parent, CAC_NS, "TaxTotal") && parent.parent_element() == root.0;
            let modeled = direct_total
                || matches!(
                    parent.tag_name().name(),
                    "LegalMonetaryTotal"
                        | "InvoiceLine"
                        | "Price"
                        | "AllowanceCharge"
                        | "TaxSubtotal"
                        | "TaxTotal"
                );
            let amount_currency = n
                .attribute("currencyID")
                .and_then(|code| CurrencyCode::parse(code).ok());
            if modeled
                && amount_currency.as_ref() != Some(&document_currency)
                && !(direct_total && amount_currency.as_ref() == Some(&tax_currency))
            {
                return Err(ParseError::InvalidValue {
                    field: "currencyID",
                    value: format!("inconsistent or missing currency on {field}"),
                });
            }
        }
    }
    for id in ["ICV", "PIH", "QR"] {
        if root
            .children(CAC_NS, "AdditionalDocumentReference")
            .filter(|r| r.basic("ID").content().as_deref() == Some(id))
            .count()
            > 1
        {
            return Err(ParseError::InvalidValue {
                field: "AdditionalDocumentReference",
                value: format!("duplicate {id}"),
            });
        }
    }
    // A unit price with a basis other than one or line adjustments needs a different
    // line calculation model. Reject it instead of dropping its financial meaning.
    for line in nav::invoice_lines(root) {
        if line.aggregate("AllowanceCharge").0.is_some()
            || line
                .aggregate("Price")
                .aggregate("AllowanceCharge")
                .0
                .is_some()
        {
            return Err(ParseError::InvalidValue {
                field: "InvoiceLine",
                value: "line/price allowances are not supported".into(),
            });
        }
        if let Some(value) = line.aggregate("Price").basic("BaseQuantity").content() {
            let quantity: Decimal = value.parse()?;
            if quantity != Decimal::from(1) {
                return Err(ParseError::InvalidValue {
                    field: "BaseQuantity",
                    value,
                });
            }
        }
    }
    Ok(())
}

fn parse_signed_properties(doc: &Document<'_>) -> Result<SignedProperties, ParseError> {
    let ctx = Element::invoice(doc);

    let qr = text_required(
        nav::document_reference(ctx, "QR")
            .aggregate("Attachment")
            .basic("EmbeddedDocumentBinaryObject")
            .content(),
        "QR",
    )?;
    let qr_fields = decode_qr_tlv(&qr)?;

    let invoice_hash = qr_fields
        .get(&6)
        .and_then(bytes_to_string)
        .ok_or(ParseError::MissingField("QR tag 6 (invoice hash)"))?;
    let signature = qr_fields
        .get(&7)
        .and_then(bytes_to_string)
        .ok_or(ParseError::MissingField("QR tag 7 (signature)"))?;
    let public_key = qr_fields
        .get(&8)
        .map(|v| Base64::encode_string(v))
        .ok_or(ParseError::MissingField("QR tag 8 (public key)"))?;
    let zatca_key_signature = qr_fields.get(&9).map(|v| Base64::encode_string(v));

    let signing_time = text_required(
        nav::signed_properties(ctx)
            .child(XADES_NS, "SignedSignatureProperties")
            .child(XADES_NS, "SigningTime")
            .content(),
        "SigningTime",
    )?;
    let signing_time = chrono::NaiveDateTime::parse_from_str(&signing_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| ParseError::InvalidValue {
            field: "SigningTime",
            value: format!("{signing_time} ({e:?})"),
        })?;
    let signing_time = signing_time.format("%Y-%m-%dT%H:%M:%S").to_string();

    let cert_hash = text_required(
        nav::signed_properties(ctx)
            .child(XADES_NS, "SignedSignatureProperties")
            .child(XADES_NS, "SigningCertificate")
            .child(XADES_NS, "Cert")
            .child(XADES_NS, "CertDigest")
            .child(DS_NS, "DigestValue")
            .content(),
        "CertDigest",
    )?;
    let issuer = text_required(
        nav::signed_properties(ctx)
            .child(XADES_NS, "SignedSignatureProperties")
            .child(XADES_NS, "SigningCertificate")
            .child(XADES_NS, "Cert")
            .child(XADES_NS, "IssuerSerial")
            .child(DS_NS, "X509IssuerName")
            .content(),
        "IssuerName",
    )?;
    let serial = text_required(
        nav::signed_properties(ctx)
            .child(XADES_NS, "SignedSignatureProperties")
            .child(XADES_NS, "SigningCertificate")
            .child(XADES_NS, "Cert")
            .child(XADES_NS, "IssuerSerial")
            .child(DS_NS, "X509SerialNumber")
            .content(),
        "SerialNumber",
    )?;
    let signed_props_hash = text_required(
        nav::signature(ctx)
            .child(DS_NS, "SignedInfo")
            .children(DS_NS, "Reference")
            .find(|reference| {
                reference.attribute("URI").as_deref() == Some("#xadesSignedProperties")
            })
            .unwrap_or(Element(None))
            .child(DS_NS, "DigestValue")
            .content(),
        "SignedPropertiesDigest",
    )?;

    Ok(SignedProperties {
        invoice_hash,
        signature,
        public_key,
        issuer,
        serial,
        cert_hash,
        signed_props_hash,
        signing_time,
        zatca_key_signature,
    })
}

fn parse_invoice_type(
    name_code: &str,
    body_code: &str,
    original_ref: Option<OriginalInvoiceRef>,
    reason: String,
) -> Result<InvoiceType, ParseError> {
    if name_code.len() != 7
        || !name_code.as_bytes()[2..]
            .iter()
            .all(|digit| matches!(digit, b'0' | b'1'))
    {
        return Err(ParseError::InvalidValue {
            field: "InvoiceTypeCode@name",
            value: name_code.to_string(),
        });
    }

    let subtype = if name_code.starts_with("02") {
        InvoiceSubType::Simplified
    } else if name_code.starts_with("01") {
        InvoiceSubType::Standard
    } else {
        return Err(ParseError::InvalidValue {
            field: "InvoiceTypeCode@name",
            value: name_code.to_string(),
        });
    };

    match body_code {
        "388" => Ok(InvoiceType::Tax(subtype)),
        "386" => Ok(InvoiceType::Prepayment(subtype)),
        "381" => Ok(InvoiceType::CreditNote(
            subtype,
            original_ref.ok_or(ParseError::MissingField("BillingReference"))?,
            reason,
        )),
        "383" => Ok(InvoiceType::DebitNote(
            subtype,
            original_ref.ok_or(ParseError::MissingField("BillingReference"))?,
            reason,
        )),
        _ => Err(ParseError::InvalidValue {
            field: "InvoiceTypeCode",
            value: body_code.to_string(),
        }),
    }
}

fn parse_original_ref(ctx: &Element<'_, '_>) -> Result<OriginalInvoiceRef, ParseError> {
    parse_reference(
        ctx.aggregate("BillingReference")
            .aggregate("InvoiceDocumentReference"),
    )
}
fn parse_reference(node: Element<'_, '_>) -> Result<OriginalInvoiceRef, ParseError> {
    let mut reference = OriginalInvoiceRef::new(text_required(
        node.basic("ID").content(),
        "BillingReferenceID",
    )?);
    if let Some(uuid) = node.basic("UUID").content() {
        reference = reference.with_uuid(uuid);
    }
    if let Some(date) = node.basic("IssueDate").content() {
        reference = reference.with_issue_date(InvoiceDate::parse(&date).map_err(|_| {
            ParseError::InvalidValue {
                field: "BillingReferenceIssueDate",
                value: date,
            }
        })?);
    }
    if let Some(time) = node.basic("IssueTime").content() {
        reference = reference.with_issue_time(crate::invoice::InvoiceTime::parse(time)?);
    }
    if let Some(code) = node.basic("DocumentTypeCode").content() {
        reference = reference.with_document_type_code(code);
    }
    Ok(reference)
}

fn parse_seller(ctx: &Element<'_, '_>) -> Result<Party<SellerRole>, ParseError> {
    let seller = nav::seller_party(*ctx);
    let name = content_required(
        seller
            .aggregate("PartyLegalEntity")
            .basic("RegistrationName")
            .content(),
        "SellerName",
    )?;
    let vat = text_required(
        seller
            .aggregate("PartyTaxScheme")
            .basic("CompanyID")
            .content(),
        "SellerVAT",
    )?;
    let identification = seller.aggregate("PartyIdentification");
    let crn = if identification.0.is_some() {
        Some(text_required(
            identification.basic("ID").content(),
            "SellerID",
        )?)
    } else {
        None
    };
    let crn_scheme = text_optional(
        seller
            .aggregate("PartyIdentification")
            .basic("ID")
            .attribute("schemeID"),
    )?;

    let address = parse_address(seller, false)?;
    let other_id = match (crn, crn_scheme) {
        (Some(value), Some(scheme)) => Some(OtherId::with_scheme(value, scheme)),
        (Some(value), None) => Some(OtherId::new(value)),
        _ => None,
    };

    Party::<SellerRole>::new(name, address, vat, other_id).map_err(ParseError::from)
}

fn parse_buyer(ctx: Element<'_, '_>) -> Result<Option<crate::invoice::Buyer>, ParseError> {
    let customer = ctx.aggregate("AccountingCustomerParty");
    let party = customer.aggregate("Party");
    if party.0.is_none()
        && customer.0.is_some_and(|n| {
            n.children().any(|n| n.is_element()) || n.text().is_some_and(|t| !t.trim().is_empty())
        })
    {
        return Err(ParseError::MissingField("BuyerParty"));
    }
    if party.0.is_none_or(|node| {
        !node.children().any(|n| n.is_element()) && node.text().is_none_or(|t| t.trim().is_empty())
    }) {
        return Ok(None);
    }
    let name = content_required(
        party
            .aggregate("PartyLegalEntity")
            .basic("RegistrationName")
            .content(),
        "BuyerName",
    )?;
    let id = party.aggregate("PartyIdentification");
    let other_id = if id.0.is_some() {
        let value = text_required(id.basic("ID").content(), "BuyerID")?;
        Some(match id.basic("ID").attribute("schemeID") {
            Some(scheme) => OtherId::with_scheme(value, scheme),
            None => OtherId::new(value),
        })
    } else {
        None
    };
    let tax = party.aggregate("PartyTaxScheme");
    let vat = if tax.0.is_some() {
        Some(text_required(tax.basic("CompanyID").content(), "BuyerVAT")?)
    } else {
        None
    };
    crate::invoice::Buyer::new(name, parse_address(party, true)?, vat, other_id)
        .map(Some)
        .map_err(ParseError::from)
}

fn parse_exemption(category: Element<'_, '_>) -> crate::invoice::TaxExemption {
    crate::invoice::TaxExemption {
        reason_code: category.basic("TaxExemptionReasonCode").content(),
        reason: category.basic("TaxExemptionReason").content(),
    }
}

fn parse_address(seller: Element<'_, '_>, buyer: bool) -> Result<Address, ParseError> {
    let required = |value: Option<String>, label| {
        if buyer {
            Ok(value.unwrap_or_default())
        } else {
            content_required(value, label)
        }
    };
    let street = required(
        seller
            .aggregate("PostalAddress")
            .basic("StreetName")
            .content(),
        "SellerStreet",
    )?;
    let building_number = required(
        seller
            .aggregate("PostalAddress")
            .basic("BuildingNumber")
            .content(),
        "SellerBuildingNumber",
    )?;
    let district = content_optional(
        seller
            .aggregate("PostalAddress")
            .basic("CitySubdivisionName")
            .content(),
    )?;
    let city = required(
        seller
            .aggregate("PostalAddress")
            .basic("CityName")
            .content(),
        "SellerCity",
    )?;
    let postal_code = required(
        seller
            .aggregate("PostalAddress")
            .basic("PostalZone")
            .content(),
        "SellerPostalCode",
    )?;
    let country_field = if buyer {
        "BuyerCountry"
    } else {
        "SellerCountry"
    };
    let country = text_required(
        seller
            .aggregate("PostalAddress")
            .aggregate("Country")
            .basic("IdentificationCode")
            .content(),
        country_field,
    )?;
    let country_code = CountryCode::parse(&country).map_err(|_| ParseError::InvalidValue {
        field: country_field,
        value: country,
    })?;

    Ok(Address {
        country_code,
        city,
        street,
        additional_street: content_optional(
            seller
                .aggregate("PostalAddress")
                .basic("AdditionalStreetName")
                .content(),
        )?,
        building_number,
        additional_number: text_optional(
            seller
                .aggregate("PostalAddress")
                .basic("PlotIdentification")
                .content(),
        )?,
        postal_code,
        district,
    })
}

fn parse_vat_category(ctx: &Element<'_, '_>) -> Result<VatCategory, ParseError> {
    let category = text_required(
        nav::invoice_lines(*ctx)
            .next()
            .unwrap_or(Element(None))
            .aggregate("Item")
            .aggregate("ClassifiedTaxCategory")
            .basic("ID")
            .content(),
        "VatCategory",
    )?;
    match category.as_str() {
        "S" => Ok(VatCategory::Standard),
        "E" => Ok(VatCategory::Exempt),
        "Z" => Ok(VatCategory::Zero),
        "O" => Ok(VatCategory::OutOfScope),
        _ => Err(ParseError::InvalidValue {
            field: "VatCategory",
            value: category,
        }),
    }
}

fn parse_line_items(ctx: &Element<'_, '_>) -> Result<Vec<LineItem>, ParseError> {
    let nodes: Vec<_> = nav::invoice_lines(*ctx).collect();
    if nodes.is_empty() {
        return Err(ParseError::MissingField("InvoiceLine"));
    }
    let mut items = Vec::with_capacity(nodes.len());
    for line in nodes {
        let id = text_required(line.basic("ID").content(), "LineID")?;
        let name = content_required(line.aggregate("Item").basic("Name").content(), "LineName")?;
        let quantity = text_required(line.basic("InvoicedQuantity").content(), "InvoicedQuantity")?;
        let unit_code = text_required(
            line.basic("InvoicedQuantity").attribute("unitCode"),
            "InvoicedQuantity@unitCode",
        )?;
        let line_extension = text_required(
            line.basic("LineExtensionAmount").content(),
            "LineExtensionAmount",
        )?;
        let price = text_required(
            line.aggregate("Price").basic("PriceAmount").content(),
            "PriceAmount",
        )?;
        let vat_rate = text_required(
            line.aggregate("Item")
                .aggregate("ClassifiedTaxCategory")
                .basic("Percent")
                .content(),
            "LineVatPercent",
        )?;
        let vat_category = text_required(
            line.aggregate("Item")
                .aggregate("ClassifiedTaxCategory")
                .basic("ID")
                .content(),
            "LineVatCategory",
        )?;
        let vat_amount = text_required(
            line.aggregate("TaxTotal").basic("TaxAmount").content(),
            "LineTaxAmount",
        )?;

        let vat_category = match vat_category.as_str() {
            "S" => VatCategory::Standard,
            "E" => VatCategory::Exempt,
            "Z" => VatCategory::Zero,
            "O" => VatCategory::OutOfScope,
            _ => {
                return Err(ParseError::InvalidValue {
                    field: "LineVatCategory",
                    value: vat_category,
                });
            }
        };

        let quantity = quantity
            .parse::<Decimal>()
            .map_err(|_| ParseError::InvalidValue {
                field: "InvoicedQuantity",
                value: quantity,
            })?;
        let unit_price = price
            .parse::<Decimal>()
            .map_err(|_| ParseError::InvalidValue {
                field: "PriceAmount",
                value: price,
            })?;
        let total_amount =
            line_extension
                .parse::<Decimal>()
                .map_err(|_| ParseError::InvalidValue {
                    field: "LineExtensionAmount",
                    value: line_extension,
                })?;
        let vat_rate = vat_rate
            .parse::<Decimal>()
            .map_err(|_| ParseError::InvalidValue {
                field: "LineVatPercent",
                value: vat_rate,
            })?;
        let vat_amount = vat_amount
            .parse::<Decimal>()
            .map_err(|_| ParseError::InvalidValue {
                field: "LineTaxAmount",
                value: vat_amount,
            })?;

        // BR-KSA-50 was removed in the May 2023 standard. Imported line VAT
        // is supplied data; validate its scale and gross equation, not a guessed formula.
        if total_amount.scale() > 2 || vat_amount.scale() > 2 {
            return Err(ParseError::InvalidValue {
                field: "LineItem",
                value: "amounts require at most two decimals".into(),
            });
        }
        let expected = quantity
            .product_rounded(unit_price, false)
            .map_err(ParseError::from)?;
        check_amount("LineExtensionAmount", &total_amount.to_string(), expected)?;
        if let Some(gross) =
            content_optional(line.aggregate("TaxTotal").basic("RoundingAmount").content())?
        {
            check_amount(
                "LineGrossAmount",
                &gross,
                total_amount.add(vat_amount).map_err(ParseError::from)?,
            )?;
        }
        let mut document_references = Vec::new();
        for reference in line.children(super::constants::CAC_NS, "DocumentReference") {
            document_references.push(parse_reference(reference)?);
        }
        let mut tax_subtotals = Vec::new();
        for subtotal in line
            .aggregate("TaxTotal")
            .children(super::constants::CAC_NS, "TaxSubtotal")
        {
            let category = subtotal.aggregate("TaxCategory");
            let code = text_required(category.basic("ID").content(), "VatCategory")?;
            let vat = match code.as_str() {
                "S" => VatCategory::Standard,
                "E" => VatCategory::Exempt,
                "Z" => VatCategory::Zero,
                "O" => VatCategory::OutOfScope,
                _ => {
                    return Err(ParseError::InvalidValue {
                        field: "VatCategory",
                        value: code,
                    });
                }
            };
            tax_subtotals.push(
                crate::invoice::VatBreakdown::new(
                    vat,
                    decimal_required(category.basic("Percent").content(), "VatRate")?,
                    decimal_required(subtotal.basic("TaxableAmount").content(), "TaxableAmount")?,
                    decimal_required(subtotal.basic("TaxAmount").content(), "TaxAmount")?,
                )?
                .with_exemption(parse_exemption(category)),
            );
        }
        let line_item = LineItem {
            document_references,
            tax_subtotals,
            id: Some(id),
            exemption: parse_exemption(line.aggregate("Item").aggregate("ClassifiedTaxCategory")),
            description: name,
            quantity,
            unit_code,
            unit_price,
            total_amount,
            vat_rate,
            vat_amount,
            vat_category,
        };

        items.push(line_item);
    }

    Ok(items)
}

fn parse_datetime(date: &str, time: &str) -> Result<String, ParseError> {
    let time =
        NaiveTime::parse_from_str(time, "%H:%M:%S").map_err(|e| ParseError::InvalidValue {
            field: "IssueTime",
            value: format!("{time} ({e:?})"),
        })?;
    let combined = format!("{date}T{}Z", time.format("%H:%M:%S"));
    let naive = NaiveDateTime::parse_from_str(&combined, "%Y-%m-%dT%H:%M:%SZ").map_err(|e| {
        ParseError::InvalidValue {
            field: "IssueDateTime",
            value: format!("{combined} ({e:?})"),
        }
    })?;
    let timestamp = naive.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    InvoiceTimestamp::parse(&timestamp).map_err(|e| ParseError::InvalidValue {
        field: "IssueDateTime",
        value: e.to_string(),
    })?;
    Ok(timestamp)
}

fn text_required(value: Option<String>, label: &'static str) -> Result<String, ParseError> {
    text_optional(value)?.ok_or(ParseError::MissingField(label))
}
fn text_optional(value: Option<String>) -> Result<Option<String>, ParseError> {
    Ok(value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty()))
}
fn content_required(value: Option<String>, label: &'static str) -> Result<String, ParseError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or(ParseError::MissingField(label))
}
fn content_optional(value: Option<String>) -> Result<Option<String>, ParseError> {
    Ok(value)
}

fn decode_qr_tlv(qr_b64: &str) -> Result<std::collections::HashMap<u8, Vec<u8>>, ParseError> {
    let raw = Base64::decode_vec(qr_b64).map_err(|e| ParseError::InvalidValue {
        field: "QR",
        value: format!("{e:?}"),
    })?;
    let mut entries = std::collections::HashMap::new();
    let mut idx = 0;
    while idx < raw.len() {
        if raw.len() - idx < 2 {
            return Err(ParseError::InvalidValue {
                field: "QR",
                value: "truncated TLV header".to_string(),
            });
        }
        let tag = raw[idx];
        let len = raw[idx + 1] as usize;
        let start = idx + 2;
        let end = start + len;
        if end > raw.len() {
            return Err(ParseError::InvalidValue {
                field: "QR",
                value: "truncated TLV".to_string(),
            });
        }
        if entries.insert(tag, raw[start..end].to_vec()).is_some() {
            return Err(ParseError::InvalidValue {
                field: "QR",
                value: format!("duplicate TLV tag {tag}"),
            });
        }
        idx = end;
    }
    Ok(entries)
}

#[allow(clippy::ptr_arg)]
fn bytes_to_string(bytes: &Vec<u8>) -> Option<String> {
    String::from_utf8(bytes.clone()).ok()
}

fn decimal_required(value: Option<String>, field: &'static str) -> Result<Decimal, ParseError> {
    let value = text_required(value, field)?;
    value
        .parse()
        .map_err(|_| ParseError::InvalidValue { field, value })
}
fn check_amount(field: &'static str, value: &str, expected: Decimal) -> Result<(), ParseError> {
    let supplied: Decimal = value.parse().map_err(|_| ParseError::InvalidValue {
        field,
        value: value.into(),
    })?;
    if supplied.scale() > 2 || supplied != expected {
        return Err(ParseError::InvalidValue {
            field,
            value: format!("supplied {supplied}, expected {expected}"),
        });
    }
    Ok(())
}
