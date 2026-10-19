//! Bounded lexical policy only; semantic selection belongs to `nav`.
use quick_xml::{events::Event, name::ResolveResult, reader::NsReader};
use std::collections::HashSet;

/// Decode XML base64Binary with bounded input and decoded sizes.
pub(crate) fn decode_base64(value: &str, maximum: usize) -> Result<Vec<u8>, &'static str> {
    use base64ct::{Base64, Encoding};
    if value.len() > maximum.saturating_mul(2) {
        return Err("Encoded value exceeds the profile limit");
    }
    let compact: String = value
        .chars()
        .filter(|c| !matches!(c, ' ' | '\t' | '\r' | '\n'))
        .collect();
    let bytes = Base64::decode_vec(&compact).map_err(|_| "Invalid base64 value")?;
    if bytes.len() > maximum {
        return Err("Decoded value exceeds the profile limit");
    }
    Ok(bytes)
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum XmlError {
    #[error("invalid XML: {0}")]
    InvalidXml(String),
    #[error("unsupported XML: {0}")]
    UnsupportedXml(String),
    #[error("XML capacity exceeded: {0}")]
    Limit(&'static str),
}

pub(crate) struct XmlLimits {
    pub xml_bytes: usize,
    pub elements: usize,
    pub dom_nodes: u32,
    pub depth: usize,
}
impl Default for XmlLimits {
    fn default() -> Self {
        Self {
            xml_bytes: super::constants::MAX_XML_DOCUMENT_BYTES,
            elements: 100_000,
            dom_nodes: 400_000,
            depth: 128,
        }
    }
}

pub(crate) fn document(input: &str) -> Result<roxmltree::Document<'_>, XmlError> {
    document_with_limits(input, &XmlLimits::default())
}

pub(crate) fn document_with_limits<'a>(
    input: &'a str,
    limits: &XmlLimits,
) -> Result<roxmltree::Document<'a>, XmlError> {
    preflight(input, limits)?;
    roxmltree::Document::parse_with_options(
        input,
        roxmltree::ParsingOptions {
            nodes_limit: limits.dom_nodes,
            ..Default::default()
        },
    )
    .map_err(|e| match e {
        roxmltree::Error::NodesLimitReached => XmlError::Limit("XML DOM nodes"),
        _ => XmlError::InvalidXml(e.to_string()),
    })
}

pub(crate) fn preflight(input: &str, limits: &XmlLimits) -> Result<(), XmlError> {
    if input.len() > limits.xml_bytes {
        return Err(XmlError::Limit("XML input bytes"));
    }
    let mut reader = NsReader::from_str(input);
    reader.config_mut().check_comments = true;
    let mut depth = 0usize;
    let mut nodes = 0usize;
    loop {
        let (namespace, event) = reader
            .read_resolved_event()
            .map_err(|e| XmlError::InvalidXml(e.to_string()))?;
        if matches!(namespace, ResolveResult::Unknown(_)) {
            return Err(XmlError::InvalidXml("undeclared element prefix".into()));
        }
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::DocType(_) => {
                return Err(XmlError::UnsupportedXml("DTD declarations".into()));
            }
            Event::Start(element) | Event::Empty(element) => {
                nodes += 1;
                if nodes > limits.elements {
                    return Err(XmlError::Limit("XML nodes"));
                }
                if depth + 1 > limits.depth.min(128) {
                    return Err(XmlError::Limit("XML depth"));
                }
                let mut names = HashSet::new();
                for attr in element.attributes() {
                    let attr = attr.map_err(|e| XmlError::InvalidXml(e.to_string()))?;
                    if attr.key.as_ref() == b"xmlns" || attr.key.as_ref().starts_with(b"xmlns:") {
                        continue;
                    }
                    let (namespace, name) = reader.resolver().resolve_attribute(attr.key);
                    let namespace = match namespace {
                        ResolveResult::Bound(ns) => ns.as_ref().to_vec(),
                        ResolveResult::Unbound => Vec::new(),
                        ResolveResult::Unknown(_) => {
                            return Err(XmlError::InvalidXml("undeclared attribute prefix".into()));
                        }
                    };
                    if !names.insert((namespace, name.as_ref().to_vec())) {
                        return Err(XmlError::InvalidXml(
                            "duplicate expanded attribute name".into(),
                        ));
                    }
                }
                if !empty {
                    depth += 1;
                }
            }
            Event::End(_) => {
                depth = depth
                    .checked_sub(1)
                    .ok_or_else(|| XmlError::InvalidXml("unexpected closing tag".into()))?;
            }
            Event::Decl(decl) => {
                // roxmltree deliberately leaves declaration values partly unchecked.
                let declaration =
                    std::str::from_utf8(&decl).map_err(|e| XmlError::InvalidXml(e.to_string()))?;
                let start = quick_xml::events::BytesStart::from_content(declaration, 3);
                let mut last = 0;
                for attr in start.attributes() {
                    let attr = attr.map_err(|e| XmlError::InvalidXml(e.to_string()))?;
                    let rank = match attr.key.as_ref() {
                        b"version" => 1,
                        b"encoding" => 2,
                        b"standalone" if matches!(attr.value.as_ref(), b"yes" | b"no") => 3,
                        _ => {
                            return Err(XmlError::InvalidXml(
                                "invalid XML declaration attribute".into(),
                            ));
                        }
                    };
                    if rank <= last {
                        return Err(XmlError::InvalidXml(
                            "XML declaration attributes are out of order".into(),
                        ));
                    }
                    last = rank;
                }

                let version = decl
                    .version()
                    .map_err(|e| XmlError::InvalidXml(e.to_string()))?;
                if version.as_ref() != b"1.0" {
                    return Err(XmlError::UnsupportedXml(
                        "XML version other than 1.0".into(),
                    ));
                }
                if let Some(encoding) = decl.encoding() {
                    let encoding = encoding.map_err(|e| XmlError::InvalidXml(e.to_string()))?;
                    if !encoding.eq_ignore_ascii_case(b"UTF-8") {
                        return Err(XmlError::UnsupportedXml("input must declare UTF-8".into()));
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if depth != 0 {
        return Err(XmlError::InvalidXml("unclosed element".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_limits_and_xml_whitespace() {
        assert_eq!(decode_base64(" Y Q==\t\r\n", 8).unwrap(), b"a");
        assert!(decode_base64("YQ==\u{a0}", 8).is_err());
        assert_eq!(decode_base64("YWFhYQ==", 4).unwrap(), b"aaaa");
        assert_eq!(
            decode_base64("YWFhYWE=", 4),
            Err("Decoded value exceeds the profile limit")
        );
        assert_eq!(
            decode_base64("YQ==     ", 4),
            Err("Encoded value exceeds the profile limit")
        );
    }

    #[test]
    fn declaration_policy_rejects_values_the_tree_parser_leaves_unchecked() {
        for decl in [
            "<?xml version='1.1'?>",
            "<?xml version='1.0' encoding='ISO-8859-1'?>",
            "<?xml version='1.0' standalone='maybe'?>",
            "<?xml version='1.0' unknown='yes'?>",
            "<?xml version='1.0' encoding='UTF-8' encoding='UTF-8'?>",
            "<?xml version='1.0' standalone='yes' encoding='UTF-8'?>",
        ] {
            assert!(document(&format!("{decl}<r/> ")).is_err(), "{decl}");
        }
        for decl in [
            "",
            "<?xml version='1.0'?>",
            "<?xml version='1.0' encoding='utf-8' standalone='yes'?>",
        ] {
            document(&format!("{decl}<r/> ")).unwrap();
        }
    }

    #[test]
    fn element_limit_accepts_scalar_text_and_trivia_at_the_boundary() {
        let mut input = "<r>".to_owned();
        input.push_str(&"<a>x</a>".repeat(99_999));
        input.push_str("<!--keep--><?keep yes?></r>");
        let doc = document(&input).unwrap();
        assert_eq!(
            doc.descendants()
                .filter(roxmltree::Node::is_element)
                .count(),
            100_000
        );
        input.insert_str(3, "<a/>");
        assert!(matches!(
            document(&input),
            Err(XmlError::Limit("XML nodes"))
        ));
    }

    #[test]
    fn document_size_limit_accepts_exact_boundary_and_rejects_one_more_byte() {
        let limit = super::super::constants::MAX_XML_DOCUMENT_BYTES;
        let mut input = format!("<r>{}</r>", "x".repeat(limit - 7));
        assert_eq!(input.len(), limit);
        document(&input).unwrap();
        input.insert(3, 'x');
        assert!(matches!(
            document(&input),
            Err(XmlError::Limit("XML input bytes"))
        ));
    }

    #[test]
    fn dom_budget_counts_text_and_trivia_separately_from_elements() {
        let input = "<r><a>x</a><!--keep--><?keep yes?></r>";
        let mut limits = XmlLimits {
            elements: 2,
            dom_nodes: 6,
            ..XmlLimits::default()
        };
        assert_eq!(
            document_with_limits(input, &limits)
                .unwrap()
                .descendants()
                .count(),
            6
        );
        limits.dom_nodes = 5;
        assert!(matches!(
            document_with_limits(input, &limits),
            Err(XmlError::Limit("XML DOM nodes"))
        ));
    }

    #[test]
    fn lexical_policy_bounds_input_and_rejects_dtds() {
        assert!(matches!(
            document("<!DOCTYPE r [<!ENTITY e 'x'>]><r>&e;</r>"),
            Err(XmlError::UnsupportedXml(_))
        ));
        for (input, limits, expected) in [
            (
                "<r/>",
                XmlLimits {
                    xml_bytes: 3,
                    elements: 10,
                    dom_nodes: 40,
                    depth: 2,
                },
                "XML input bytes",
            ),
            (
                "<r><a/></r>",
                XmlLimits {
                    xml_bytes: 100,
                    elements: 1,
                    dom_nodes: 40,
                    depth: 2,
                },
                "XML nodes",
            ),
            (
                "<r><a/></r>",
                XmlLimits {
                    xml_bytes: 100,
                    elements: 10,
                    dom_nodes: 40,
                    depth: 1,
                },
                "XML depth",
            ),
        ] {
            assert!(
                matches!(preflight(input, &limits), Err(XmlError::Limit(label)) if label == expected)
            );
        }
        document("<r><!-- <!DOCTYPE fake> --><![CDATA[<!DOCTYPE fake>]]></r>").unwrap();
    }
}
