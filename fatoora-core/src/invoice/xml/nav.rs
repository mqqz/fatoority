//! Expanded-name navigation and UBL selections on an immutable source snapshot.
use super::constants::{CAC_NS, CBC_NS, DS_NS, EXT_NS, INVOICE_NS, SAC_NS, SIG_NS, XADES_NS};
use roxmltree::Node;

pub(crate) fn is(node: Node<'_, '_>, namespace: &str, local: &str) -> bool {
    node.is_element()
        && node.tag_name().namespace() == Some(namespace)
        && node.tag_name().name() == local
}
pub(crate) fn content(node: Node<'_, '_>) -> String {
    node.descendants()
        .filter(Node::is_text)
        .filter_map(|node| node.text())
        .collect()
}

#[derive(Clone, Copy)]
pub(crate) struct Element<'a, 'input>(pub Option<Node<'a, 'input>>);
impl<'a, 'input> Element<'a, 'input> {
    pub fn invoice(document: &'a roxmltree::Document<'input>) -> Self {
        let root = document.root_element();
        Self(is(root, INVOICE_NS, "Invoice").then_some(root))
    }
    pub fn child(self, namespace: &str, local: &str) -> Self {
        Self(
            self.0
                .and_then(|node| node.children().find(|child| is(*child, namespace, local))),
        )
    }
    pub fn children(
        self,
        namespace: &'static str,
        local: &'static str,
    ) -> impl Iterator<Item = Self> {
        self.0
            .into_iter()
            .flat_map(|node| node.children())
            .filter(move |node| is(*node, namespace, local))
            .map(|node| Self(Some(node)))
    }
    pub fn basic(self, local: &str) -> Self {
        self.child(CBC_NS, local)
    }
    pub fn aggregate(self, local: &str) -> Self {
        self.child(CAC_NS, local)
    }
    pub fn content(self) -> Option<String> {
        self.0.map(content)
    }
    pub fn attribute(self, name: &str) -> Option<String> {
        self.0
            .and_then(|node| node.attribute(name))
            .map(str::to_owned)
    }
}
pub(crate) fn document_reference<'a, 'input>(
    invoice: Element<'a, 'input>,
    id: &str,
) -> Element<'a, 'input> {
    invoice
        .children(CAC_NS, "AdditionalDocumentReference")
        .find(|reference| {
            reference
                .children(CBC_NS, "ID")
                .any(|node| node.content().as_deref() == Some(id))
        })
        .unwrap_or(Element(None))
}
pub(crate) fn seller_party<'a, 'input>(invoice: Element<'a, 'input>) -> Element<'a, 'input> {
    invoice
        .child(CAC_NS, "AccountingSupplierParty")
        .child(CAC_NS, "Party")
}
pub(crate) fn invoice_lines<'a, 'input>(
    invoice: Element<'a, 'input>,
) -> impl Iterator<Item = Element<'a, 'input>> {
    invoice.children(CAC_NS, "InvoiceLine")
}
pub(crate) fn allowance_charges<'a, 'input>(
    invoice: Element<'a, 'input>,
) -> impl Iterator<Item = Element<'a, 'input>> {
    invoice.children(CAC_NS, "AllowanceCharge")
}
pub(crate) fn tax_totals<'a, 'input>(
    invoice: Element<'a, 'input>,
) -> impl Iterator<Item = Element<'a, 'input>> {
    invoice.children(CAC_NS, "TaxTotal")
}
/// BT-110 is denominated in the document currency, not the accounting currency.
/// Preserve the first matching total in the SDK's two-total layout.
pub(crate) fn qr_tax_amount<'a, 'input>(invoice: Element<'a, 'input>) -> Element<'a, 'input> {
    let currency = invoice
        .basic("DocumentCurrencyCode")
        .content()
        .unwrap_or_default();
    tax_totals(invoice)
        .map(|total| total.basic("TaxAmount"))
        .find(|amount| {
            amount.attribute("currencyID").as_deref()
                == Some(currency.trim_matches([' ', '\t', '\r', '\n']))
        })
        .unwrap_or(Element(None))
}
pub(crate) fn signature_extension<'a, 'input>(invoice: Element<'a, 'input>) -> Element<'a, 'input> {
    invoice
        .child(EXT_NS, "UBLExtensions")
        .children(EXT_NS, "UBLExtension")
        .find(|extension| {
            extension.child(EXT_NS, "ExtensionURI").content().as_deref()
                == Some("urn:oasis:names:specification:ubl:dsig:enveloped:xades")
        })
        .unwrap_or(Element(None))
}
pub(crate) fn signature<'a, 'input>(invoice: Element<'a, 'input>) -> Element<'a, 'input> {
    signature_extension(invoice)
        .child(EXT_NS, "ExtensionContent")
        .child(SIG_NS, "UBLDocumentSignatures")
        .child(SAC_NS, "SignatureInformation")
        .child(DS_NS, "Signature")
}
pub(crate) fn signed_properties<'a, 'input>(invoice: Element<'a, 'input>) -> Element<'a, 'input> {
    signature(invoice)
        .child(DS_NS, "Object")
        .child(XADES_NS, "QualifyingProperties")
        .child(XADES_NS, "SignedProperties")
}

/// Preserve cardinality through each direct-child step for integrity checks.
#[derive(Clone)]
pub(crate) struct Nodes<'a, 'input>(pub Vec<Node<'a, 'input>>);
impl<'a, 'input> Nodes<'a, 'input> {
    pub fn children(&self, namespace: &str, local: &str) -> Self {
        Self(
            self.0
                .iter()
                .flat_map(Node::children)
                .filter(|node| is(*node, namespace, local))
                .collect(),
        )
    }
    pub fn filter(self, predicate: impl FnMut(&Node<'a, 'input>) -> bool) -> Self {
        Self(self.0.into_iter().filter(predicate).collect())
    }
    pub fn at(self, index: usize) -> Self {
        Self(self.0.into_iter().nth(index).into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::invoice::xml::lexical;

    #[test]
    fn original_element_ranges_preserve_every_lexical_byte() {
        for element in [
            "<b:ID code='&#x31;'>أ &amp; &#x32;</b:ID>",
            "<b:ID>Q<![CDATA[R]]></b:ID>",
            "<b:ID>first\r\nsecond</b:ID>",
            "<b:ID xmlns:b='urn:alias'><b:Child /></b:ID>",
            "<b:ID />",
        ] {
            let input = format!(
                "<Invoice xmlns='{INVOICE_NS}' xmlns:b='{CBC_NS}'><!--before-->{element}<?after keep?></Invoice>"
            );
            let doc = lexical::document(&input).unwrap();
            let node = doc
                .root_element()
                .children()
                .find(Node::is_element)
                .unwrap();
            assert_eq!(&input[node.range()], element);
        }
    }

    #[test]
    fn ubl_navigation_ignores_foreign_and_nested_lookalikes() {
        let input = format!(
            "<Invoice xmlns='{INVOICE_NS}' xmlns:b='{CBC_NS}' xmlns:a='{CAC_NS}' xmlns:f='urn:foreign'><f:InvoiceLine/><a:InvoiceLine/><a:AllowanceCharge/><a:TaxTotal/><a:AdditionalDocumentReference><b:ID>Q<![CDATA[R]]></b:ID></a:AdditionalDocumentReference><a:AdditionalDocumentReference><b:ID> QR </b:ID></a:AdditionalDocumentReference><f:wrapper><a:InvoiceLine/></f:wrapper></Invoice>"
        );
        let doc = lexical::document(&input).unwrap();
        let invoice = Element::invoice(&doc);
        assert_eq!(invoice_lines(invoice).count(), 1);
        assert_eq!(allowance_charges(invoice).count(), 1);
        assert_eq!(tax_totals(invoice).count(), 1);
        assert_eq!(
            document_reference(invoice, "QR")
                .basic("ID")
                .content()
                .as_deref(),
            Some("QR")
        );
        assert!(document_reference(invoice, "PIH").0.is_none());
        let refs = Nodes(vec![doc.root_element()]).children(CAC_NS, "AdditionalDocumentReference");
        assert_eq!(refs.0.len(), 2);
        assert!(refs.at(2).0.is_empty());
    }
}
