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
use tracing::{debug, instrument};

use super::common::{detect_format, extract_message_type, extract_mx_content};

pub struct Validate;

#[async_trait]
impl AsyncFunctionHandler for Validate {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting MX message validation");

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

        let validation_result_field = input
            .get("validation_result")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation("'validation_result' parameter is required".to_string())
            })?;

        let format = input
            .get("format")
            .and_then(Value::as_str)
            .unwrap_or("auto");

        // Get the MX message to validate
        let mx_content = extract_mx_content(message.data(), mx_message_field, &message.payload)?;

        debug!(
            mx_message_field = %mx_message_field,
            validation_result_field = %validation_result_field,
            format = %format,
            "Validating MX message"
        );

        // Perform validation
        let validation_result = self.validate_mx_message(&mx_content, format)?;

        // Store validation result
        message.data_mut().as_object_mut().unwrap().insert(
            validation_result_field.to_string(),
            validation_result.clone(),
        );

        // Update metadata with validation summary
        message.metadata_mut().as_object_mut().unwrap().insert(
            "validation".to_string(),
            json!({
                "validated": true,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }),
        );

        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", validation_result_field)),
                old_value: Arc::new(Value::Null),
                new_value: Arc::new(validation_result),
            }],
        ))
    }
}

impl Validate {
    fn validate_mx_message(&self, mx_content: &str, format_hint: &str) -> Result<Value> {
        // Auto-detect format if needed
        let format = if format_hint == "auto" {
            detect_format(mx_content)
        } else {
            format_hint.to_string()
        };

        debug!(format = %format, "Validating MX message");

        let mut errors: Vec<String> = Vec::new();
        let warnings: Vec<String> = Vec::new();

        match format.as_str() {
            "xml" => {
                // For XML validation, try to deserialize directly to typed structs
                // First parse to get message type
                match self.parse_xml(mx_content) {
                    Ok(data) => {
                        match extract_message_type(&data) {
                            Ok(message_type) => {
                                // Try to deserialize XML directly to typed struct using envelope-aware parsing
                                // This validates both AppHdr (if present) and Document structure
                                match crate::xml::from_mx_xml_envelope_str(
                                    mx_content,
                                    &message_type,
                                ) {
                                    Ok(_) => {
                                        // Successfully deserialized - message is valid
                                        debug!(
                                            "XML message validated successfully (with envelope support)"
                                        );
                                    }
                                    Err(e) => {
                                        errors.push(format!("XML deserialization failed: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                errors.push(format!("Could not determine message type: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!("XML parsing failed: {}", e));
                    }
                }
            }
            "json" => {
                // For JSON validation, parse and deserialize to typed struct
                match self.parse_json(mx_content) {
                    Ok(data) => {
                        match extract_message_type(&data) {
                            Ok(message_type) => {
                                // Try to serialize to XML (validates structure)
                                match crate::xml::json_to_typed_xml(&data, &message_type) {
                                    Ok(_xml_str) => {
                                        debug!("JSON message validated successfully");
                                    }
                                    Err(e) => {
                                        errors.push(format!("JSON validation failed: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                errors.push(format!("Could not determine message type: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!("JSON parsing failed: {}", e));
                    }
                }
            }
            _ => {
                errors.push(format!("Unsupported format: {}", format));
            }
        }

        let is_valid = errors.is_empty();

        Ok(json!({
            "valid": is_valid,
            "errors": errors,
            "warnings": warnings,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))
    }

    /// Parse XML to JSON
    fn parse_xml(&self, xml_str: &str) -> std::result::Result<Value, String> {
        crate::xml::from_mx_xml_to_json(xml_str).map_err(|e| format!("XML parsing error: {}", e))
    }

    /// Parse JSON string to Value
    fn parse_json(&self, json_str: &str) -> std::result::Result<Value, String> {
        serde_json::from_str(json_str).map_err(|e| format!("JSON parsing error: {}", e))
    }
}
