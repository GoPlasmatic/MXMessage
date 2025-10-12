use async_trait::async_trait;
use dataflow_rs::engine::error::DataflowError;
use dataflow_rs::engine::{
    AsyncFunctionHandler, FunctionConfig,
    error::Result,
    message::{Change, Message},
};
use datalogic_rs::DataLogic;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::{debug, error, instrument};

use super::common::{extract_message_type, extract_mx_content};

pub struct Parse;

#[async_trait]
impl AsyncFunctionHandler for Parse {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting MX message parsing (XML to JSON)");

        // Extract custom configuration
        let input = match config {
            FunctionConfig::Custom { input, .. } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        let mx_message_field =
            input
                .get("mx_message")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    DataflowError::Validation("'mx_message' parameter is required".to_string())
                })?;

        let parsed_field = input.get("parsed").and_then(Value::as_str).ok_or_else(|| {
            DataflowError::Validation("'parsed' parameter is required".to_string())
        })?;

        let xml_payload = extract_mx_content(message.data(), mx_message_field, &message.payload)?
            .replace("\\n", "\n");

        debug!(
            mx_message_field = %mx_message_field,
            parsed_field = %parsed_field,
            payload_length = xml_payload.len(),
            "Extracted XML payload for parsing"
        );

        self.parse_xml_to_json(message, &xml_payload, parsed_field)
    }
}

impl Parse {
    /// Parse XML to JSON using MxMessage API
    /// This is the primary parsing method - always XML input to JSON output
    fn parse_xml_to_json(
        &self,
        message: &mut Message,
        xml_str: &str,
        parsed_field: &str,
    ) -> Result<(usize, Vec<Change>)> {
        use crate::mx_envelope::MxMessage;

        debug!("Parsing XML to JSON using MxMessage API");

        // Check if XML has full envelope or just Document
        let has_envelope = xml_str.contains("<AppHdr") || xml_str.contains("<Envelope");

        let parsed_data = if has_envelope {
            debug!("XML has full envelope with AppHdr");

            // Use MxMessage to deserialize XML with envelope
            let mx_message = MxMessage::from_xml(xml_str).map_err(|e| {
                error!(error = ?e, "Failed to parse XML with MxMessage");
                DataflowError::Validation(format!("XML parsing error: {}", e))
            })?;

            // Convert to JSON string then parse to Value
            let json_str = mx_message.to_json().map_err(|e| {
                error!(error = ?e, "Failed to convert to JSON");
                DataflowError::Validation(format!("JSON conversion error: {}", e))
            })?;

            serde_json::from_str(&json_str).map_err(|e| {
                error!(error = ?e, "Failed to parse JSON");
                DataflowError::Validation(format!("JSON parsing error: {}", e))
            })?
        } else {
            debug!("XML has Document only, using typed parser");

            // Fall back to Document-only parser for XML without envelope
            use super::common::extract_message_type_from_xml;
            let message_type = extract_message_type_from_xml(xml_str)?;

            crate::xml::xml_to_json_via_document(xml_str, &message_type).map_err(|e| {
                error!(error = ?e, "XML parsing failed");
                DataflowError::Validation(format!("XML parsing error: {}", e))
            })?
        };

        // Extract message type from parsed data
        let message_type = extract_message_type(&parsed_data)?;

        debug!(message_type = %message_type, "Successfully parsed XML to JSON");

        // Store the parsed result in message data
        message
            .data_mut()
            .as_object_mut()
            .unwrap()
            .insert(parsed_field.to_string(), parsed_data.clone());

        message.metadata_mut().as_object_mut().unwrap().insert(
            parsed_field.to_string(),
            json!({
                "message_type": message_type,
                "format": "json",
            }),
        );

        debug!(
            message_type = %message_type,
            parsed_field = %parsed_field,
            "XML to JSON parsing completed successfully"
        );

        // Important: invalidate cache after modifications
        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", parsed_field)),
                old_value: Arc::new(Value::Null),
                new_value: Arc::new(parsed_data),
            }],
        ))
    }
}
