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

use super::common::{
    detect_format, extract_message_type, extract_message_type_from_xml, extract_mx_content,
};

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
        debug!("Starting MX message parsing");

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

        let format = input
            .get("format")
            .and_then(Value::as_str)
            .unwrap_or("auto");

        let payload = extract_mx_content(message.data(), mx_message_field, &message.payload)?
            .replace("\\n", "\n");

        debug!(
            mx_message_field = %mx_message_field,
            parsed_field = %parsed_field,
            payload_length = payload.len(),
            format = %format,
            "Extracted MX payload for parsing"
        );

        self.parse_mx_message(message, &payload, parsed_field, format)
    }
}

impl Parse {
    fn parse_mx_message(
        &self,
        message: &mut Message,
        payload: &str,
        parsed_field: &str,
        format_hint: &str,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Parsing MX message");

        // Auto-detect format if needed
        let format = if format_hint == "auto" {
            detect_format(payload)
        } else {
            format_hint.to_string()
        };

        debug!(format = %format, "Detected/using format");

        let parsed_data = match format.as_str() {
            "xml" => self.parse_xml(payload)?,
            "json" => self.parse_json(payload)?,
            _ => {
                return Err(DataflowError::Validation(format!(
                    "Unsupported format: {}",
                    format
                )));
            }
        };

        // Extract message type from parsed data
        let message_type = extract_message_type(&parsed_data)?;

        debug!(message_type = %message_type, "Successfully parsed MX message");

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
                "format": format,
            }),
        );

        debug!(
            message_type = %message_type,
            format = %format,
            parsed_field = %parsed_field,
            "MX message parsing completed successfully"
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

    /// Parse XML to JSON
    fn parse_xml(&self, xml_str: &str) -> Result<Value> {
        // First, extract message type from XML to determine which typed struct to use
        let message_type = extract_message_type_from_xml(xml_str)?;

        debug!(message_type = %message_type, "Extracted message type from XML");

        // Use the typed struct parser which correctly handles arrays
        crate::xml::xml_to_json_via_document(xml_str, &message_type).map_err(|e| {
            error!(error = ?e, "XML parsing failed");
            DataflowError::Validation(format!("XML parsing error: {}", e))
        })
    }

    /// Parse JSON string to Value
    fn parse_json(&self, json_str: &str) -> Result<Value> {
        serde_json::from_str(json_str).map_err(|e| {
            error!(error = ?e, "JSON parsing failed");
            DataflowError::Validation(format!("JSON parsing error: {}", e))
        })
    }
}
