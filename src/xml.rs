// XML Serialization and Deserialization utilities for MX Messages

use crate::mx_envelope::{
    BusinessApplicationHeaderBuilder, BusinessApplicationHeaderV02, MxEnvelope,
};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use quick_xml::{de::from_str as xml_from_str, se::to_string as xml_to_string};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::Cursor;

/// Configuration for XML serialization
#[derive(Debug, Clone)]
pub struct XmlConfig {
    /// Whether to include XML declaration
    pub include_declaration: bool,
    /// Whether to format with indentation
    pub pretty_print: bool,
    /// Indentation string (e.g., "  " for 2 spaces)
    pub indent: String,
    /// Whether to include schema location
    pub include_schema_location: bool,
}

impl Default for XmlConfig {
    fn default() -> Self {
        Self {
            include_declaration: true,
            pretty_print: true,
            indent: "  ".to_string(),
            include_schema_location: false,
        }
    }
}

/// Error type for XML operations
#[derive(Debug)]
pub enum XmlError {
    SerializationError(String),
    DeserializationError(String),
    ValidationError(String),
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlError::SerializationError(msg) => write!(f, "XML Serialization Error: {msg}"),
            XmlError::DeserializationError(msg) => write!(f, "XML Deserialization Error: {msg}"),
            XmlError::ValidationError(msg) => write!(f, "XML Validation Error: {msg}"),
        }
    }
}

impl Error for XmlError {}

/// Serialize any MX message to complete XML with envelope
pub fn to_mx_xml<D>(
    message: D,
    header: BusinessApplicationHeaderV02,
    message_type: &str,
    config: Option<XmlConfig>,
) -> Result<String, XmlError>
where
    D: Serialize,
{
    let config = config.unwrap_or_default();

    // Determine the namespace based on message type
    let document_namespace = get_namespace_for_message_type(message_type);

    // Create the envelope
    let envelope = MxEnvelope::new(header, message, document_namespace);

    // Use custom XML writer for proper formatting
    if config.pretty_print {
        format_mx_xml(&envelope, &config)
    } else {
        // Use quick-xml for compact output
        xml_to_string(&envelope).map_err(|e| XmlError::SerializationError(e.to_string()))
    }
}

/// Parse complete MX XML with envelope
pub fn from_mx_xml<H, D>(xml: &str) -> Result<MxEnvelope<H, D>, XmlError>
where
    H: for<'de> Deserialize<'de>,
    D: for<'de> Deserialize<'de>,
{
    xml_from_str(xml).map_err(|e| XmlError::DeserializationError(e.to_string()))
}

/// Format MX XML with proper indentation and structure
fn format_mx_xml<H, D>(envelope: &MxEnvelope<H, D>, config: &XmlConfig) -> Result<String, XmlError>
where
    H: Serialize,
    D: Serialize,
{
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', config.indent.len());

    // Write XML declaration
    if config.include_declaration {
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| XmlError::SerializationError(e.to_string()))?;
    }

    // Start envelope element with namespaces
    let mut envelope_elem = BytesStart::new("Envelope");
    envelope_elem.push_attribute(("xmlns", "urn:iso:std:iso:20022:tech:xsd:head.001.001.02"));
    envelope_elem.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));

    if config.include_schema_location {
        envelope_elem.push_attribute((
            "xsi:schemaLocation",
            "urn:iso:std:iso:20022:tech:xsd:head.001.001.02 head.001.001.02.xsd",
        ));
    }

    writer
        .write_event(Event::Start(envelope_elem))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Write AppHdr
    let app_hdr_xml = xml_to_string(&envelope.app_hdr)
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Remove the XML declaration from the inner serialization if present
    let app_hdr_xml = app_hdr_xml
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim();

    writer
        .write_event(Event::Text(BytesText::from_escaped(app_hdr_xml)))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Write Document with its namespace
    let mut doc_elem = BytesStart::new("Document");
    if let Some(ref xmlns) = envelope.document.xmlns {
        doc_elem.push_attribute(("xmlns", xmlns.as_str()));
    }

    writer
        .write_event(Event::Start(doc_elem))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Write the actual message content
    let message_xml = xml_to_string(&envelope.document.message)
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Remove the XML declaration from the inner serialization if present
    let message_xml = message_xml
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim();

    writer
        .write_event(Event::Text(BytesText::from_escaped(message_xml)))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Close Document
    writer
        .write_event(Event::End(BytesEnd::new("Document")))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Close Envelope
    writer
        .write_event(Event::End(BytesEnd::new("Envelope")))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| XmlError::SerializationError(e.to_string()))
}

/// Get the appropriate namespace for a message type
fn get_namespace_for_message_type(message_type: &str) -> String {
    let namespace = match message_type {
        "pacs.008" | "pacs.008.001.08" => "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
        "pacs.009" | "pacs.009.001.08" => "urn:iso:std:iso:20022:tech:xsd:pacs.009.001.08",
        "pacs.003" | "pacs.003.001.08" => "urn:iso:std:iso:20022:tech:xsd:pacs.003.001.08",
        "pacs.002" | "pacs.002.001.10" => "urn:iso:std:iso:20022:tech:xsd:pacs.002.001.10",
        "pain.001" | "pain.001.001.09" => "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09",
        "pain.008" | "pain.008.001.08" => "urn:iso:std:iso:20022:tech:xsd:pain.008.001.08",
        "camt.052" | "camt.052.001.08" => "urn:iso:std:iso:20022:tech:xsd:camt.052.001.08",
        "camt.053" | "camt.053.001.08" => "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08",
        "camt.054" | "camt.054.001.08" => "urn:iso:std:iso:20022:tech:xsd:camt.054.001.08",
        "camt.056" | "camt.056.001.08" => "urn:iso:std:iso:20022:tech:xsd:camt.056.001.08",
        "camt.057" | "camt.057.001.06" => "urn:iso:std:iso:20022:tech:xsd:camt.057.001.06",
        "camt.060" | "camt.060.001.05" => "urn:iso:std:iso:20022:tech:xsd:camt.060.001.05",
        "camt.027" | "camt.027.001.07" => "urn:iso:std:iso:20022:tech:xsd:camt.027.001.07",
        "camt.029" | "camt.029.001.09" => "urn:iso:std:iso:20022:tech:xsd:camt.029.001.09",
        _ => {
            return format!("urn:iso:std:iso:20022:tech:xsd:{message_type}");
        }
    };
    namespace.to_string()
}

/// Helper function to create XML for a specific message type
pub fn create_pacs008_xml<D: Serialize>(
    message: D,
    from_bic: String,
    to_bic: String,
    business_msg_id: String,
) -> Result<String, XmlError> {
    let header = BusinessApplicationHeaderBuilder::new("pacs.008.001.08".to_string())
        .from_bicfi(from_bic)
        .to_bicfi(to_bic)
        .business_message_identifier(business_msg_id)
        .build();

    to_mx_xml(message, header, "pacs.008", None)
}

/// Helper function to create XML for pain.001 message
pub fn create_pain001_xml<D: Serialize>(
    message: D,
    from_bic: String,
    to_bic: String,
    business_msg_id: String,
) -> Result<String, XmlError> {
    let header = BusinessApplicationHeaderBuilder::new("pain.001.001.09".to_string())
        .from_bicfi(from_bic)
        .to_bicfi(to_bic)
        .business_message_identifier(business_msg_id)
        .build();

    to_mx_xml(message, header, "pain.001", None)
}

/// Helper function to create XML for camt.053 message
pub fn create_camt053_xml<D: Serialize>(
    message: D,
    from_bic: String,
    to_bic: String,
    business_msg_id: String,
) -> Result<String, XmlError> {
    let header = BusinessApplicationHeaderBuilder::new("camt.053.001.08".to_string())
        .from_bicfi(from_bic)
        .to_bicfi(to_bic)
        .business_message_identifier(business_msg_id)
        .build();

    to_mx_xml(message, header, "camt.053", None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_lookup() {
        assert_eq!(
            get_namespace_for_message_type("pacs.008"),
            "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"
        );
        assert_eq!(
            get_namespace_for_message_type("pain.001"),
            "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09"
        );
        assert_eq!(
            get_namespace_for_message_type("camt.053"),
            "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08"
        );
    }

    #[test]
    fn test_xml_config_default() {
        let config = XmlConfig::default();
        assert!(config.include_declaration);
        assert!(config.pretty_print);
        assert_eq!(config.indent, "  ");
        assert!(!config.include_schema_location);
    }
}
