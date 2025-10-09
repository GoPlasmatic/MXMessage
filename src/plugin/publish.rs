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
        eprintln!("🔍 PUBLISH PLUGIN: Starting JSON to MX message publishing");
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

        let format = input.get("format").and_then(Value::as_str).unwrap_or("xml");

        let include_bah = input
            .get("include_bah")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let namespace_prefix = input.get("namespace_prefix").and_then(Value::as_str);

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
            format = %format,
            include_bah = %include_bah,
            "Processing JSON to MX conversion"
        );

        // Extract the actual JSON data if it's wrapped in a generate result
        let json_to_convert = if let Some(inner_json) = json_data.get("json_data") {
            // This is output from the generate function
            eprintln!("🔍 PUBLISH: Found json_data wrapper, extracting inner JSON");
            debug!("Found json_data wrapper, extracting inner JSON");
            inner_json.clone()
        } else {
            // Direct JSON data
            eprintln!("🔍 PUBLISH: Using JSON data directly");
            debug!("Using JSON data directly");
            json_data.clone()
        };

        // Debug: log the structure we're trying to convert
        if let Some(obj) = json_to_convert.as_object() {
            let keys: Vec<&String> = obj.keys().collect();
            eprintln!(
                "🔍 PUBLISH: JSON structure has {} keys: {:?}",
                keys.len(),
                keys
            );
            debug!("JSON structure has {} keys: {:?}", keys.len(), keys);
        }

        // Extract message type from the JSON data
        let message_type = json_data
            .get("message_type")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation(
                    "Missing 'message_type' field in JSON data. The message_type field is required at the root level.".to_string()
                )
            })?;

        debug!(message_type = %message_type, format = %format, "Converting JSON to MX message");

        eprintln!(
            "🔍 PUBLISH: About to call json_to_mx with format={}",
            format
        );
        // Convert JSON to MX message
        let mx_message = self.json_to_mx(
            &json_to_convert,
            message_type,
            format,
            include_bah,
            namespace_prefix,
        )?;
        eprintln!("🔍 PUBLISH: Successfully converted to MX message");

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
    /// Convert JSON to MX message (XML or JSON format)
    fn json_to_mx(
        &self,
        json_data: &Value,
        message_type: &str,
        format: &str,
        include_bah: bool,
        namespace_prefix: Option<&str>,
    ) -> Result<String> {
        match format {
            "xml" => self.json_to_xml(json_data, message_type, include_bah, namespace_prefix),
            "json" => self.json_to_json(json_data),
            _ => Err(DataflowError::Validation(format!(
                "Unsupported format: {}. Supported formats are 'xml' and 'json'",
                format
            ))),
        }
    }

    /// Convert JSON to XML format
    fn json_to_xml(
        &self,
        json_data: &Value,
        message_type: &str,
        _include_bah: bool,
        _namespace_prefix: Option<&str>,
    ) -> Result<String> {
        // Handle different JSON structures:
        // 1. Just Document: { "Document": {...} }
        // 2. Envelope with AppHdr and Document: { "AppHdr": {...}, "Document": {...} }
        // For now, we only serialize the Document part (AppHdr will be supported later)

        eprintln!("🔍 PUBLISH json_to_xml: Checking for Document element");
        let data_to_serialize = if let Some(doc) = json_data.get("Document") {
            // Has Document - check if there are other root keys
            if let Some(obj) = json_data.as_object() {
                eprintln!(
                    "🔍 PUBLISH json_to_xml: Found Document. Object has {} keys",
                    obj.len()
                );
                if obj.len() == 1 {
                    // Only Document - serialize as-is
                    eprintln!("🔍 PUBLISH json_to_xml: Only Document key, serializing as-is");
                    debug!("Serializing Document only");
                    json_data.clone()
                } else {
                    // Multiple root keys (e.g., AppHdr + Document) - extract Document only
                    eprintln!(
                        "🔍 PUBLISH json_to_xml: Multiple root keys detected ({:?}), extracting Document only",
                        obj.keys().collect::<Vec<_>>()
                    );
                    debug!("Multiple root keys detected, extracting Document only");
                    json!({ "Document": doc })
                }
            } else {
                json_data.clone()
            }
        } else {
            eprintln!("🔍 PUBLISH json_to_xml: ERROR - No Document element found");
            return Err(DataflowError::Validation(
                "JSON data must contain a 'Document' element for XML serialization".to_string(),
            ));
        };

        // Create XML config (kept for future AppHdr support)
        let _config = crate::xml::XmlConfig::default();

        // Debug: show what we're about to serialize
        debug!("About to serialize to XML. Data structure:");
        if let Some(obj) = data_to_serialize.as_object() {
            eprintln!(
                "🔍 PUBLISH json_to_xml: About to serialize. Root keys: {:?}",
                obj.keys().collect::<Vec<_>>()
            );
            debug!("  Root keys: {:?}", obj.keys().collect::<Vec<_>>());
        }

        // Use the new typed serialization function from the library
        eprintln!("🔍 PUBLISH json_to_xml: Calling json_to_typed_xml...");
        let result = crate::xml::json_to_typed_xml(&data_to_serialize, message_type);
        match &result {
            Ok(_) => eprintln!("🔍 PUBLISH json_to_xml: json_to_typed_xml succeeded"),
            Err(e) => eprintln!("🔍 PUBLISH json_to_xml: json_to_typed_xml FAILED: {:?}", e),
        }

        result.map_err(|e| {
            error!(error = ?e, data = ?data_to_serialize, message_type = %message_type, "XML serialization failed");
            DataflowError::Validation(format!("XML serialization error: {}", e))
        })
    }

    /// Convert JSON to JSON format (essentially just stringify with formatting)
    fn json_to_json(&self, json_data: &Value) -> Result<String> {
        serde_json::to_string_pretty(json_data).map_err(|e| {
            error!(error = ?e, "JSON serialization failed");
            DataflowError::Validation(format!("JSON serialization error: {}", e))
        })
    }
}
