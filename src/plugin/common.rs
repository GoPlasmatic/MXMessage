//! Common utilities for MX message plugin operations
//!
//! This module contains shared functionality used across multiple plugin handlers
//! to reduce code duplication and improve maintainability.

use dataflow_rs::engine::error::{DataflowError, Result};
use serde_json::Value;

/// Detect the format of an MX message payload (XML or JSON)
///
/// # Arguments
/// * `payload` - The message payload to analyze
///
/// # Returns
/// * "xml" if the payload appears to be XML
/// * "json" if the payload appears to be JSON
/// * "xml" as default if format cannot be determined
pub fn detect_format(payload: &str) -> String {
    let trimmed = payload.trim();
    if trimmed.starts_with('<') {
        "xml".to_string()
    } else if trimmed.starts_with('{') || trimmed.starts_with('[') {
        "json".to_string()
    } else {
        "xml".to_string() // Default to XML
    }
}

/// Map ISO20022 document element names to message types
///
/// # Arguments
/// * `element_name` - The document element name (e.g., "FIToFICstmrCdtTrf")
///
/// # Returns
/// * Ok(message_type) - The corresponding message type (e.g., "pacs.008")
/// * Err(error) - If the element name is not recognized
pub fn map_document_element_to_message_type(element_name: &str) -> Result<String> {
    let message_type = match element_name {
        "FIToFICstmrCdtTrf" | "FIToFICustomerCreditTransferV08" => "pacs.008",
        "FIToFIPmtStsRpt" | "FIToFIPaymentStatusReportV10" => "pacs.002",
        "FinInstnCdtTrf" | "FinancialInstitutionCreditTransferV08" => "pacs.009",
        "PmtRtr" | "PaymentReturnV09" => "pacs.004",
        "FIToFICstmrDrctDbt" | "FIToFICustomerDirectDebitV08" => "pacs.003",
        "FIDrctDbt" | "FinancialInstitutionDirectDebitV03" => "pacs.010",
        "BkToCstmrStmt" | "BankToCustomerStatementV08" => "camt.053",
        "BkToCstmrDbtCdtNtfctn" | "BankToCustomerDebitCreditNotificationV08" => "camt.054",
        "AcctRptgReq" | "AccountReportingRequestV05" => "camt.060",
        "BkToCstmrAcctRpt" | "BankToCustomerAccountReportV08" => "camt.052",
        "RsltnOfInvstgtn" | "ResolutionOfInvestigationV09" => "camt.029",
        "Rct" | "ReceiptV08" => "camt.025",
        "FIToFIPmtCxlReq" | "FIToFIPaymentCancellationRequestV08" => "camt.056",
        "NtfctnToRcv" | "NotificationToReceiveV06" => "camt.057",
        "CstmrCdtTrfInitn" | "CustomerCreditTransferInitiationV09" => "pain.001",
        "CstmrDrctDbtInitn" | "CustomerDirectDebitInitiationV08" => "pain.008",
        "ChqPresntmntNtfctn" | "ChequePresentmentNotificationV01" => "camt.107",
        "ChqCxlOrStopReq" | "ChequeCancellationOrStopRequestV01" => "camt.108",
        "ChqCxlOrStopRpt" | "ChequeCancellationOrStopReportV01" => "camt.109",
        "ClmNonRct" | "ClaimNonReceiptV07" => "camt.027",
        _ => {
            return Err(DataflowError::Validation(format!(
                "Unknown document element: {}",
                element_name
            )));
        }
    };
    Ok(message_type.to_string())
}

/// Extract message type from parsed JSON data
///
/// This function attempts to extract the message type from various locations
/// in the parsed JSON structure:
/// 1. Direct "message_type" field at root level
/// 2. From Document structure by examining the first key
///
/// # Arguments
/// * `data` - The parsed JSON data
///
/// # Returns
/// * Ok(message_type) - The extracted message type
/// * Err(error) - If message type cannot be determined
pub fn extract_message_type(data: &Value) -> Result<String> {
    // Try to get message_type from root level
    if let Some(mt) = data.get("message_type").and_then(Value::as_str) {
        return Ok(mt.to_string());
    }

    // Try to extract from Document structure
    if let Some(doc) = data.get("Document") {
        // Get the first key under Document (e.g., "FIToFICstmrCdtTrf")
        if let Some(obj) = doc.as_object()
            && let Some(first_key) = obj.keys().next()
        {
            // Map document element name to message type
            return map_document_element_to_message_type(first_key);
        }
    }

    Err(DataflowError::Validation(
        "Could not determine message type from parsed data".to_string(),
    ))
}

/// Extract message type from XML string
///
/// This function performs a lightweight parsing of XML to extract the message type
/// without fully parsing the entire document.
///
/// # Arguments
/// * `xml_str` - The XML string to analyze
///
/// # Returns
/// * Ok(message_type) - The extracted message type
/// * Err(error) - If message type cannot be determined
pub fn extract_message_type_from_xml(xml_str: &str) -> Result<String> {
    let xml_str = xml_str.trim();

    // Find the Document opening tag
    if let Some(doc_start) = xml_str.find("<Document") {
        // Find the end of the opening Document tag
        if let Some(doc_end) = xml_str[doc_start..].find('>') {
            let after_doc = &xml_str[doc_start + doc_end + 1..];

            // Find the first element after Document
            if let Some(elem_start) = after_doc.find('<')
                && after_doc.as_bytes()[elem_start + 1] != b'/'
            {
                let elem_name_start = elem_start + 1;
                // Find the end of the element name (space or >)
                let elem_name_end = after_doc[elem_name_start..]
                    .find([' ', '>', '/'])
                    .map(|i| elem_name_start + i)
                    .unwrap_or(after_doc.len());

                let element_name = &after_doc[elem_name_start..elem_name_end];
                // Remove namespace prefix if present
                let element_name = if let Some(colon_pos) = element_name.rfind(':') {
                    &element_name[colon_pos + 1..]
                } else {
                    element_name
                };

                return map_document_element_to_message_type(element_name);
            }
        }
    }

    Err(DataflowError::Validation(
        "Could not extract message type from XML".to_string(),
    ))
}

/// Extract MX content from a message field
///
/// This function handles various formats of MX message content:
/// - Direct string values
/// - Objects with mx_message field (from generate_mx output)
/// - Payload field
///
/// # Arguments
/// * `message_data` - The message data object
/// * `field_name` - The field name to extract from
/// * `message_payload` - The message payload (if field_name is "payload")
///
/// # Returns
/// * Ok(content) - The extracted MX message content
/// * Err(error) - If content cannot be extracted
pub fn extract_mx_content(
    message_data: &Value,
    field_name: &str,
    message_payload: &Value,
) -> Result<String> {
    if field_name == "payload" {
        // Extract string value from the payload JSON
        if let Some(s) = message_payload.as_str() {
            Ok(s.to_string())
        } else {
            // If it's not a string directly, try to convert
            Ok(message_payload.to_string().trim_matches('"').to_string())
        }
    } else {
        // Check if the field contains an object with mx_message (from generate_mx output)
        let field_value = message_data.get(field_name).ok_or_else(|| {
            DataflowError::Validation(format!(
                "MX message field '{}' not found in message data",
                field_name
            ))
        })?;

        // If it's an object with mx_message field, extract that
        if let Some(mx_msg) = field_value.get("mx_message").and_then(Value::as_str) {
            Ok(mx_msg.to_string())
        } else if let Some(s) = field_value.as_str() {
            // If it's a direct string, use it
            Ok(s.to_string())
        } else {
            Err(DataflowError::Validation(format!(
                "Field '{}' does not contain a valid MX message",
                field_name
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format("<?xml version=\"1.0\"?>"), "xml");
        assert_eq!(detect_format("<Document>"), "xml");
        assert_eq!(detect_format("{\"key\": \"value\"}"), "json");
        assert_eq!(detect_format("[1, 2, 3]"), "json");
        assert_eq!(detect_format("unknown"), "xml"); // default
        assert_eq!(detect_format("  <tag>  "), "xml"); // with whitespace
    }

    #[test]
    fn test_map_document_element_to_message_type() {
        assert_eq!(
            map_document_element_to_message_type("FIToFICstmrCdtTrf").unwrap(),
            "pacs.008"
        );
        assert_eq!(
            map_document_element_to_message_type("BkToCstmrStmt").unwrap(),
            "camt.053"
        );
        assert_eq!(
            map_document_element_to_message_type("CstmrCdtTrfInitn").unwrap(),
            "pain.001"
        );
        assert!(map_document_element_to_message_type("UnknownElement").is_err());
    }

    #[test]
    fn test_extract_message_type() {
        // Test with message_type at root
        let data = json!({"message_type": "pacs.008", "other": "data"});
        assert_eq!(extract_message_type(&data).unwrap(), "pacs.008");

        // Test with Document structure
        let data = json!({
            "Document": {
                "FIToFICstmrCdtTrf": {
                    "GrpHdr": {}
                }
            }
        });
        assert_eq!(extract_message_type(&data).unwrap(), "pacs.008");

        // Test with no message type
        let data = json!({"other": "data"});
        assert!(extract_message_type(&data).is_err());
    }

    #[test]
    fn test_extract_mx_content() {
        let payload = json!("test content");
        let data = json!({
            "field1": "direct string",
            "field2": {
                "mx_message": "nested message"
            }
        });

        // Test payload extraction
        assert_eq!(
            extract_mx_content(&data, "payload", &payload).unwrap(),
            "test content"
        );

        // Test direct string field
        assert_eq!(
            extract_mx_content(&data, "field1", &payload).unwrap(),
            "direct string"
        );

        // Test nested mx_message field
        assert_eq!(
            extract_mx_content(&data, "field2", &payload).unwrap(),
            "nested message"
        );

        // Test missing field
        assert!(extract_mx_content(&data, "missing", &payload).is_err());
    }
}
