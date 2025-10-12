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

use super::common::extract_mx_content;

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
        debug!("Starting MX message validation (XML)");

        // Extract custom configuration
        let input = match config {
            FunctionConfig::Custom { input, .. } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        let source_field = input.get("source").and_then(Value::as_str).ok_or_else(|| {
            DataflowError::Validation("'source' parameter is required".to_string())
        })?;

        let target_field = input.get("target").and_then(Value::as_str).ok_or_else(|| {
            DataflowError::Validation("'target' parameter is required".to_string())
        })?;

        // Get the MX XML message to validate
        let xml_content = extract_mx_content(message.data(), source_field, &message.payload)?;

        debug!(
            source_field = %source_field,
            target_field = %target_field,
            "Validating MX XML message"
        );

        // Perform XML validation
        let validation_result = self.validate_xml(&xml_content)?;

        // Store validation result
        message
            .data_mut()
            .as_object_mut()
            .unwrap()
            .insert(target_field.to_string(), validation_result.clone());

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
                path: Arc::from(format!("data.{}", target_field)),
                old_value: Arc::new(Value::Null),
                new_value: Arc::new(validation_result),
            }],
        ))
    }
}

impl Validate {
    /// Validate XML MX message by attempting to parse into typed structs
    /// This validates both structure and content according to ISO20022 schemas
    fn validate_xml(&self, xml_content: &str) -> Result<Value> {
        use crate::mx_envelope::MxMessage;

        debug!("Validating XML MX message");

        let mut errors: Vec<String> = Vec::new();
        let warnings: Vec<String> = Vec::new();

        // Check if XML has full envelope or just Document
        let has_envelope = xml_content.contains("<AppHdr") || xml_content.contains("<Envelope");

        if has_envelope {
            debug!("Validating XML with full envelope using MxMessage");

            // Validate by attempting to deserialize with MxMessage
            match MxMessage::from_xml(xml_content) {
                Ok(_) => {
                    debug!("XML message with envelope validated successfully");
                }
                Err(e) => {
                    errors.push(format!("XML validation failed: {}", e));
                }
            }
        } else {
            debug!("Validating Document-only XML using from_mx_xml_envelope_str");

            // For Document-only XML, we need to determine the message type first
            use super::common::extract_message_type_from_xml;

            match extract_message_type_from_xml(xml_content) {
                Ok(message_type) => {
                    // Validate using the envelope validator which handles both cases
                    match crate::xml::from_mx_xml_envelope_str(xml_content, &message_type) {
                        Ok(_) => {
                            debug!("Document-only XML validated successfully");
                        }
                        Err(e) => {
                            errors.push(format!("XML validation failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("Could not determine message type: {}", e));
                }
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
}
