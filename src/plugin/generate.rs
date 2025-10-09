use async_trait::async_trait;
use datafake_rs::DataGenerator;
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

pub struct Generate;

#[async_trait]
impl AsyncFunctionHandler for Generate {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        eprintln!("🔍 GENERATE PLUGIN: Starting datafake generation for MX message");
        debug!("Starting datafake generation for MX message");

        // Extract configuration
        let input = match config {
            FunctionConfig::Custom { input, name: _ } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        // Get the output field name for generated data
        let generated_field = input
            .get("generated")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation("'generated' parameter is required".to_string())
            })?;

        let message_type = input
            .get("message_type")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation("'message_type' parameter is required".to_string())
            })?;

        // Get the datafake scenario from payload (contains both variables and schema)
        let scenario = message.payload.clone();

        debug!(
            generated_field = %generated_field,
            message_type = %message_type,
            "Generating MX message data from payload using datafake"
        );

        // Extract variables and schema from scenario
        let variables = scenario.get("variables").ok_or_else(|| {
            DataflowError::Validation("Scenario missing 'variables' section in payload".to_string())
        })?;

        let schema = scenario.get("schema").ok_or_else(|| {
            DataflowError::Validation("Scenario missing 'schema' section in payload".to_string())
        })?;

        // Create datafake config with variables and schema
        let datafake_config = serde_json::json!({
            "variables": variables,
            "schema": schema
        });

        // Generate data using datafake
        let generated_data = match DataGenerator::from_value(datafake_config) {
            Ok(generator) => generator.generate().map_err(|e| {
                error!(error = ?e, "Datafake generation failed");
                DataflowError::Validation(format!("Datafake generation failed: {}", e))
            })?,
            Err(e) => {
                error!(error = ?e, "Failed to create datafake generator from scenario");
                return Err(DataflowError::Validation(format!(
                    "Invalid datafake scenario: {}",
                    e
                )));
            }
        };

        // Wrap the generated data with message_type and json_data structure
        // This matches the expected structure for the publish plugin
        let wrapped_data = serde_json::json!({
            "message_type": message_type,
            "json_data": generated_data.clone()
        });

        // Debug: Log a sample value to trace data flow
        if let Some(cdtr_nm) = generated_data
            .pointer("/AppHdr/BizMsgIdr")
            .or_else(|| generated_data.pointer("/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId"))
        {
            eprintln!("🔍 GENERATE: Generated data MsgId/BizMsgIdr: {}", cdtr_nm);
        }

        // Store the generated data in the generated field
        let old_value = message
            .data()
            .get(generated_field)
            .cloned()
            .unwrap_or(Value::Null);

        message
            .data_mut()
            .as_object_mut()
            .ok_or_else(|| DataflowError::Validation("Message data must be an object".to_string()))?
            .insert(generated_field.to_string(), wrapped_data.clone());

        // Invalidate cache after modification
        message.invalidate_context_cache();

        // Debug: Verify the data was stored
        if let Some(stored) = message.data().get(generated_field) {
            if let Some(stored_msg_id) = stored.pointer("/json_data/AppHdr/BizMsgIdr") {
                eprintln!("🔍 GENERATE: Verified stored MsgId: {}", stored_msg_id);
            }
        } else {
            eprintln!(
                "❌ GENERATE: Failed to store data in field '{}'",
                generated_field
            );
        }

        debug!(
            message_type = %message_type,
            "Successfully generated MX message data"
        );

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", generated_field)),
                old_value: Arc::new(old_value),
                new_value: Arc::new(wrapped_data),
            }],
        ))
    }
}
