// XML Serialization and Deserialization utilities for MX Messages
// This module provides utilities for converting between XML and JSON for ISO20022 messages
// Used by the plugin system for validation and parsing operations

use quick_xml::de::from_str as xml_from_str;
use std::error::Error;
use std::fmt;

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
            XmlError::DeserializationError(format!(
                "Failed to validate Document (message_type={}): {}",
                message_type, e
            ))
        })?;
        return Ok(());
    }

    // Parse as full envelope with AppHdr - validate by attempting deserialization
    // All message types use the same BusinessApplicationHeaderV02
    xml_from_str::<MxMessage>(xml).map_err(|e| {
        XmlError::DeserializationError(format!("Failed to parse {} envelope: {}", message_type, e))
    })?;

    Ok(())
}

/// Helper function to get the Document element name for a message type
/// Used internally by xml_to_json_via_document
fn get_document_element_name(message_type: &str) -> &'static str {
    match message_type {
        "pacs.008" => "FIToFICstmrCdtTrf",
        "pacs.009" => "FIToFICstmrCdtTrf",
        "pacs.003" => "FIToFICstmrDrctDbt",
        "pacs.004" => "PmtRtr",
        "pacs.010" => "FIDrctDbt",
        "pacs.002" => "FIToFIPmtStsRpt",
        "pain.001" => "CstmrCdtTrfInitn",
        "pain.008" => "CstmrDrctDbtInitn",
        "camt.025" => "Rct",
        "camt.029" => "RsltnOfInvstgtn",
        "camt.052" => "BkToCstmrAcctRpt",
        "camt.053" => "BkToCstmrStmt",
        "camt.054" => "BkToCstmrDbtCdtNtfctn",
        "camt.056" => "FIToFIPmtCxlReq",
        "camt.057" => "NtfctnToRcv",
        "camt.060" => "AcctRptgReq",
        "camt.107" => "ChqPresntmntNtfctn",
        "camt.108" => "ChqCxlOrStopReq",
        "camt.109" => "ChqCxlOrStopRpt",
        "admi.024" => "NtfctnOfCrspdc",
        _ => "Unknown",
    }
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
        "pacs.008" => {
            let doc = xml_from_str::<pacs_008_001_08::FIToFICustomerCreditTransferV08>(inner_xml)
                .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.008: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.009" => {
            let doc =
                xml_from_str::<pacs_009_001_08::FinancialInstitutionCreditTransferV08>(inner_xml)
                    .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.009: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.003" => {
            let doc = xml_from_str::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.003: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.004" => {
            let doc =
                xml_from_str::<pacs_004_001_09::PaymentReturnV09>(inner_xml).map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.004: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.010" => {
            let doc =
                xml_from_str::<pacs_010_001_03::FinancialInstitutionDirectDebitV03>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse pacs.010: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.002" => {
            let doc = xml_from_str::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.002: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pain.001" => {
            let doc =
                xml_from_str::<pain_001_001_09::CustomerCreditTransferInitiationV09>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse pain.001: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pain.008" => {
            let doc = xml_from_str::<pain_008_001_08::CustomerDirectDebitInitiationV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pain.008: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.025" => {
            let doc = xml_from_str::<camt_025_001_08::ReceiptV08>(inner_xml).map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.025: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.029" => {
            let doc = xml_from_str::<camt_029_001_09::ResolutionOfInvestigationV09>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.029: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.052" => {
            let doc = xml_from_str::<camt_052_001_08::BankToCustomerAccountReportV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.052: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.053" => {
            let doc = xml_from_str::<camt_053_001_08::BankToCustomerStatementV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.053: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.054" => {
            let doc = xml_from_str::<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>(
                inner_xml,
            )
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.054: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.056" => {
            let doc =
                xml_from_str::<camt_056_001_08::FIToFIPaymentCancellationRequestV08>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse camt.056: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.057" => {
            let doc = xml_from_str::<camt_057_001_06::NotificationToReceiveV06>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.057: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.060" => {
            let doc = xml_from_str::<camt_060_001_05::AccountReportingRequestV05>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.060: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.107" => {
            let doc = xml_from_str::<camt_107_001_01::ChequePresentmentNotificationV01>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.107: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.108" => {
            let doc =
                xml_from_str::<camt_108_001_01::ChequeCancellationOrStopRequestV01>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse camt.108: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.109" => {
            let doc = xml_from_str::<camt_109_001_01::ChequeCancellationOrStopReportV01>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.109: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "admi.024" => {
            let doc = xml_from_str::<admi_024_001_01::NotificationOfCorrespondenceV01>(inner_xml)
                .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse admi.024: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        _ => {
            return Err(XmlError::DeserializationError(format!(
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
        .map_err(|e| XmlError::DeserializationError(format!("XML parsing error: {}", e)))?;

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
