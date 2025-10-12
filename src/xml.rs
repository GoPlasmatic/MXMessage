// XML Serialization and Deserialization utilities for MX Messages
// This module provides utilities for converting between XML and JSON for ISO20022 messages
// Used by the plugin system for validation and parsing operations

use crate::error::MxError;
use crate::message_registry;
use quick_xml::de::from_str as xml_from_str;

// Re-export MxError as XmlError for backward compatibility
pub type XmlError = MxError;

/// Deserialize complete MX XML envelope (with AppHdr) to typed structs for validation
/// This validates both the header and document structure
/// Used by validation plugin for envelope validation
pub fn from_mx_xml_envelope_str(xml: &str, message_type: &str) -> Result<(), XmlError> {
    use crate::mx_envelope::MxMessage;

    // Check if XML contains AppHdr (full envelope) or just Document
    let has_envelope = xml.contains("<AppHdr") || xml.contains("<Envelope");

    if !has_envelope {
        // No envelope, validate using MxMessage which handles Document-only
        MxMessage::from_xml(xml).map_err(|e| {
            MxError::XmlDeserialization(format!(
                "Failed to validate Document (message_type={}): {}",
                message_type, e
            ))
        })?;
        return Ok(());
    }

    // Parse as full envelope with AppHdr - validate by attempting deserialization
    // All message types use the same BusinessApplicationHeaderV02
    xml_from_str::<MxMessage>(xml).map_err(|e| {
        MxError::XmlDeserialization(format!("Failed to parse {} envelope: {}", message_type, e))
    })?;

    Ok(())
}

/// Helper function to get the Document element name for a message type
/// Used internally by xml_to_json_via_document
fn get_document_element_name(message_type: &str) -> &'static str {
    message_registry::message_type_to_element(message_type).unwrap_or("Unknown")
}

/// Macro to reduce XML to JSON conversion boilerplate
macro_rules! xml_to_json_doc {
    ($xml:expr, $path:path, $msg_type:expr) => {{
        let doc = xml_from_str::<$path>($xml).map_err(|e| {
            MxError::XmlDeserialization(format!("Failed to parse {}: {}", $msg_type, e))
        })?;
        serde_json::to_value(&doc).map_err(|e| MxError::XmlSerialization(e.to_string()))?
    }};
}

/// Parse XML to JSON using typed ISO20022 document structures
/// This is the correct way to parse XML that preserves arrays and complex structures
/// Used by parse plugin for Document-only XML parsing
pub fn xml_to_json_via_document(
    xml: &str,
    message_type: &str,
) -> Result<serde_json::Value, XmlError> {
    use crate::document::*;
    use serde_json::json;

    // Extract inner XML (strip Document wrapper and XML declaration)
    let inner_xml = xml
        .trim()
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim();

    // Find the Document wrapper and extract inner content
    let inner_xml = if let Some(start_idx) = inner_xml.find("<Document") {
        if let Some(end_bracket) = inner_xml[start_idx..].find('>') {
            let content_start = start_idx + end_bracket + 1;
            if let Some(end_idx) = inner_xml.rfind("</Document>") {
                &inner_xml[content_start..end_idx]
            } else {
                inner_xml
            }
        } else {
            inner_xml
        }
    } else {
        inner_xml
    };

    let inner_xml = inner_xml.trim();

    // Deserialize XML to typed struct, then serialize to JSON
    let json_value = match message_type {
        "pacs.008" => xml_to_json_doc!(
            inner_xml,
            pacs_008_001_08::FIToFICustomerCreditTransferV08,
            "pacs.008"
        ),
        "pacs.009" => xml_to_json_doc!(
            inner_xml,
            pacs_009_001_08::FinancialInstitutionCreditTransferV08,
            "pacs.009"
        ),
        "pacs.003" => xml_to_json_doc!(
            inner_xml,
            pacs_003_001_08::FIToFICustomerDirectDebitV08,
            "pacs.003"
        ),
        "pacs.004" => xml_to_json_doc!(inner_xml, pacs_004_001_09::PaymentReturnV09, "pacs.004"),
        "pacs.010" => xml_to_json_doc!(
            inner_xml,
            pacs_010_001_03::FinancialInstitutionDirectDebitV03,
            "pacs.010"
        ),
        "pacs.002" => xml_to_json_doc!(
            inner_xml,
            pacs_002_001_10::FIToFIPaymentStatusReportV10,
            "pacs.002"
        ),
        "pain.001" => xml_to_json_doc!(
            inner_xml,
            pain_001_001_09::CustomerCreditTransferInitiationV09,
            "pain.001"
        ),
        "pain.008" => xml_to_json_doc!(
            inner_xml,
            pain_008_001_08::CustomerDirectDebitInitiationV08,
            "pain.008"
        ),
        "camt.025" => xml_to_json_doc!(inner_xml, camt_025_001_08::ReceiptV08, "camt.025"),
        "camt.029" => xml_to_json_doc!(
            inner_xml,
            camt_029_001_09::ResolutionOfInvestigationV09,
            "camt.029"
        ),
        "camt.052" => xml_to_json_doc!(
            inner_xml,
            camt_052_001_08::BankToCustomerAccountReportV08,
            "camt.052"
        ),
        "camt.053" => xml_to_json_doc!(
            inner_xml,
            camt_053_001_08::BankToCustomerStatementV08,
            "camt.053"
        ),
        "camt.054" => xml_to_json_doc!(
            inner_xml,
            camt_054_001_08::BankToCustomerDebitCreditNotificationV08,
            "camt.054"
        ),
        "camt.056" => xml_to_json_doc!(
            inner_xml,
            camt_056_001_08::FIToFIPaymentCancellationRequestV08,
            "camt.056"
        ),
        "camt.057" => xml_to_json_doc!(
            inner_xml,
            camt_057_001_06::NotificationToReceiveV06,
            "camt.057"
        ),
        "camt.060" => xml_to_json_doc!(
            inner_xml,
            camt_060_001_05::AccountReportingRequestV05,
            "camt.060"
        ),
        "camt.107" => xml_to_json_doc!(
            inner_xml,
            camt_107_001_01::ChequePresentmentNotificationV01,
            "camt.107"
        ),
        "camt.108" => xml_to_json_doc!(
            inner_xml,
            camt_108_001_01::ChequeCancellationOrStopRequestV01,
            "camt.108"
        ),
        "camt.109" => xml_to_json_doc!(
            inner_xml,
            camt_109_001_01::ChequeCancellationOrStopReportV01,
            "camt.109"
        ),
        "admi.024" => xml_to_json_doc!(
            inner_xml,
            admi_024_001_01::NotificationOfCorrespondenceV01,
            "admi.024"
        ),
        _ => {
            return Err(MxError::XmlDeserialization(format!(
                "Unsupported message type: {}",
                message_type
            )));
        }
    };

    // Wrap in Document structure with correct element name
    let element_name = get_document_element_name(message_type);
    Ok(json!({
        "Document": {
            element_name: json_value
        }
    }))
}

/// Helper function to parse XML to JSON Value (for dataflow plugins)
/// This function parses XML and converts it to a JSON structure
///
/// Note: This is a simplified XML parser for plugin use.
/// It converts XML elements to JSON objects preserving the structure.
///
/// Used by mx_envelope for fallback parsing and validate plugin for legacy validation.
pub fn from_mx_xml_to_json(xml: &str) -> Result<serde_json::Value, XmlError> {
    // Use quick_xml to deserialize directly to a serde_json::Value
    // This is simpler than manual parsing and handles the XML-to-JSON conversion
    let value: serde_json::Value = quick_xml::de::from_str(xml)
        .map_err(|e| MxError::XmlDeserialization(format!("XML parsing error: {}", e)))?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_document_element_name() {
        assert_eq!(get_document_element_name("pacs.008"), "FIToFICstmrCdtTrf");
        assert_eq!(get_document_element_name("camt.053"), "BkToCstmrStmt");
        assert_eq!(get_document_element_name("pain.001"), "CstmrCdtTrfInitn");
    }
}
