use async_trait::async_trait;
use dataflow_rs::engine::error::DataflowError;
use dataflow_rs::engine::{
    AsyncFunctionHandler, FunctionConfig,
    error::Result,
    message::{Change, Message},
};
use datalogic_rs::DataLogic;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, instrument};

pub struct Publish;

#[async_trait]
impl AsyncFunctionHandler for Publish {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting JSON to MX message publishing");

        // Extract custom configuration
        let input = match config {
            FunctionConfig::Custom { input, .. } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        // Get json_data and mx_message field names
        let json_data_field = input
            .get("json_data")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation("'json_data' parameter is required".to_string())
            })?;

        let mx_message_field =
            input
                .get("mx_message")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    DataflowError::Validation("'mx_message' parameter is required".to_string())
                })?;

        // Extract JSON data from the message
        let json_data = message.data().get(json_data_field).cloned().ok_or_else(|| {
            error!(
                json_data_field = %json_data_field,
                available_fields = ?message.data().as_object().map(|obj| obj.keys().collect::<Vec<_>>()),
                "JSON data field not found in message data"
            );
            DataflowError::Validation(format!(
                "Field '{}' not found in message data",
                json_data_field
            ))
        })?;

        debug!(
            json_data_field = %json_data_field,
            mx_message_field = %mx_message_field,
            "Processing JSON to MX conversion"
        );

        // Convert JSON to MX message (message type is auto-detected from AppHdr.MsgDefIdr)
        let mx_message = self.json_to_mx(&json_data)?;

        debug!(
            message_length = mx_message.len(),
            "MX message published successfully"
        );

        // Store the MX message in the output field
        let old_value = message
            .data()
            .get(mx_message_field)
            .cloned()
            .unwrap_or(Value::Null);

        message.data_mut()[mx_message_field] = Value::String(mx_message.clone());

        // Invalidate cache after modifications
        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", mx_message_field)),
                old_value: Arc::new(old_value),
                new_value: Arc::new(Value::String(mx_message)),
            }],
        ))
    }
}

impl Publish {
    /// Convert JSON to MX message using the new MxMessage API
    fn json_to_mx(&self, json_data: &Value) -> Result<String> {
        use crate::mx_envelope::MxMessage;

        // Serialize JSON to string for parsing
        let json_str = serde_json::to_string(json_data).map_err(|e| {
            error!(error = ?e, "Failed to serialize JSON");
            DataflowError::Validation(format!("JSON serialization error: {}", e))
        })?;

        // Use MxMessage to deserialize and re-serialize to XML
        let mx_message = MxMessage::from_json(&json_str).map_err(|e| {
            error!(error = ?e, "Failed to deserialize MX message");
            DataflowError::Validation(format!("MX deserialization error: {}", e))
        })?;

        mx_message.to_xml().map_err(|e| {
            error!(error = ?e, "Failed to serialize to XML");
            DataflowError::Validation(format!("XML serialization error: {}", e))
        })
    }
}
