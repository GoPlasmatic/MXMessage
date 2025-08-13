// Plasmatic MX Message Parsing Library
// https://github.com/GoPlasmatic/MXMessage
//
// Copyright (c) 2025 Plasmatic
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// You may obtain a copy of this library at
// https://github.com/GoPlasmatic/MXMessage

use crate::error::ValidationError;
use crate::scenario_config::{
    ScenarioConfig, find_scenario_by_name_with_config, find_scenario_for_message_type_with_config,
};
use crate::xml::to_mx_xml;
use datafake_rs::DataGenerator;
use serde_json::{Value, json};

type Result<T> = std::result::Result<T, ValidationError>;

/// Generate a sample MX XML message based on test scenarios
///
/// This function loads a test scenario configuration for the specified message type
/// and generates a complete MX XML message with envelope and header.
///
/// # Arguments
///
/// * `message_type` - The MX message type (e.g., "pacs008", "camt053")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
/// * `config` - Configuration for scenario file paths
///
/// # Returns
///
/// Returns a complete MX XML string with envelope and header
///
/// # Example
///
/// ```no_run
/// # use mx_message::sample::generate_sample_xml;
/// # use mx_message::scenario_config::ScenarioConfig;
/// // Generate a standard pacs.008 message as XML
/// let pacs008_xml = generate_sample_xml("pacs008", None, &ScenarioConfig::default()).unwrap();
///
/// // Generate a specific scenario
/// let pacs008_high_value_xml = generate_sample_xml("pacs008", Some("high_value"), &ScenarioConfig::default()).unwrap();
/// ```
pub fn generate_sample_xml(
    message_type: &str,
    scenario_name: Option<&str>,
    config: &ScenarioConfig,
) -> Result<String> {
    // Load the scenario configuration JSON
    let scenario_json = if let Some(name) = scenario_name {
        find_scenario_by_name_with_config(message_type, name, config)?
    } else {
        find_scenario_for_message_type_with_config(message_type, config)?
    };

    // Process the scenario to generate complete envelope using datafake-rs
    let envelope_data = generate_envelope_from_scenario(&scenario_json)?;

    // Convert envelope to XML based on message type
    envelope_to_xml(envelope_data, message_type)
}

/// Generate a sample MX message object based on test scenarios
///
/// This function loads a test scenario configuration for the specified message type
/// and generates the document part of the message as a typed object.
///
/// # Arguments
///
/// * `message_type` - The MX message type (e.g., "pacs008", "camt053")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
/// * `config` - Configuration for scenario file paths
///
/// # Returns
///
/// Returns the document part of the message as a deserializable type
///
/// # Example
///
/// ```no_run
/// # use mx_message::sample::generate_sample_object;
/// # use mx_message::scenario_config::ScenarioConfig;
/// # use mx_message::document::pacs_008_001_08::FIToFICustomerCreditTransferV08;
/// // Generate a pacs.008 message object
/// let pacs008: FIToFICustomerCreditTransferV08 = generate_sample_object("pacs008", None, &ScenarioConfig::default()).unwrap();
/// ```
pub fn generate_sample_object<T>(
    message_type: &str,
    scenario_name: Option<&str>,
    config: &ScenarioConfig,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    // Load the scenario configuration JSON
    let scenario_json = if let Some(name) = scenario_name {
        find_scenario_by_name_with_config(message_type, name, config)?
    } else {
        find_scenario_for_message_type_with_config(message_type, config)?
    };

    // Process the scenario to generate complete envelope
    let envelope_data = generate_envelope_from_scenario(&scenario_json)?;

    // Extract the document part based on message type
    let document_data = extract_document_from_envelope(&envelope_data, message_type)?;

    // Convert generated data to string for parsing
    let generated_json = serde_json::to_string_pretty(&document_data).map_err(|e| {
        ValidationError::new(9997, format!("Failed to serialize generated data: {e}"))
    })?;

    // Parse the generated JSON into the message type
    serde_json::from_str(&generated_json).map_err(|e| {
        ValidationError::new(
            9997,
            format!("Failed to parse generated JSON into {message_type}: {e}"),
        )
    })
}

// ============================================================================
// Private implementation functions
// ============================================================================

/// Generate complete envelope from scenario JSON using datafake-rs
fn generate_envelope_from_scenario(scenario: &Value) -> Result<Value> {
    // Convert the scenario format to datafake-rs compatible format
    let datafake_config = convert_to_datafake_config(scenario)?;

    // Create a DataGenerator from the config
    let generator = DataGenerator::from_value(datafake_config)
        .map_err(|e| ValidationError::new(9997, format!("Failed to create generator: {e}")))?;

    // Generate the data
    generator
        .generate()
        .map_err(|e| ValidationError::new(9997, format!("Failed to generate data: {e}")))
}

/// Convert scenario JSON to datafake-rs compatible configuration
fn convert_to_datafake_config(scenario: &Value) -> Result<Value> {
    // Extract variables and schema
    let variables = scenario.get("variables").ok_or_else(|| {
        ValidationError::new(9997, "Scenario missing 'variables' section".to_string())
    })?;

    let schema = scenario.get("schema").ok_or_else(|| {
        ValidationError::new(9997, "Scenario missing 'schema' section".to_string())
    })?;

    // Convert the configuration to datafake-rs format
    // Variables section stays mostly the same, but we need to ensure compatibility
    let converted_variables = convert_variables(variables)?;

    // Schema section needs to be processed to handle special cases
    let converted_schema = convert_schema(schema)?;

    Ok(json!({
        "variables": converted_variables,
        "schema": converted_schema
    }))
}

/// Convert variables section to datafake-rs compatible format
fn convert_variables(variables: &Value) -> Result<Value> {
    match variables {
        Value::Object(vars) => {
            let mut converted = serde_json::Map::new();

            for (key, spec) in vars {
                let converted_spec = convert_variable_spec(spec)?;
                converted.insert(key.clone(), converted_spec);
            }

            Ok(Value::Object(converted))
        }
        _ => Ok(variables.clone()),
    }
}

/// Convert a single variable specification
fn convert_variable_spec(spec: &Value) -> Result<Value> {
    match spec {
        Value::Object(obj) => {
            // Handle cat operations - datafake-rs supports these natively
            if obj.contains_key("cat") {
                return Ok(spec.clone());
            }

            // Handle fake operations - ensure compatibility with datafake-rs
            if let Some(fake_spec) = obj.get("fake") {
                if let Value::Array(fake_arr) = fake_spec {
                    // Map our custom fake types to datafake-rs supported types
                    if let Some(Value::String(fake_type)) = fake_arr.first() {
                        let mapped_spec = map_fake_type(fake_type, &fake_arr[1..])?;
                        return Ok(mapped_spec);
                    }
                }
                return Ok(spec.clone());
            }

            // Handle pick operations - convert to fake enum
            if let Some(Value::Array(options)) = obj.get("pick") {
                // Convert pick to a fake enum operation
                let mut fake_array = vec![json!("enum")];
                for option in options {
                    fake_array.push(option.clone());
                }
                return Ok(json!({"fake": fake_array}));
            }

            Ok(spec.clone())
        }
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            // Static values pass through
            Ok(spec.clone())
        }
        _ => Ok(spec.clone()),
    }
}

/// Map custom fake types to datafake-rs supported types
fn map_fake_type(fake_type: &str, args: &[Value]) -> Result<Value> {
    // Most fake types are compatible, but we need to handle some special cases
    match fake_type {
        // ISO datetime is a custom type in our implementation
        "iso8601_datetime" => {
            // Since datafake-rs doesn't have datetime support, generate a static ISO datetime
            let now = chrono::Utc::now();
            Ok(json!(now.to_rfc3339()))
        }
        // Currency code needs mapping
        "currency_code" => {
            // Use enum with common currency codes
            Ok(json!({"fake": ["enum", "EUR", "USD", "GBP", "CHF", "JPY", "CAD", "AUD"]}))
        }
        // IBAN generation with country code
        "iban" => {
            let country = args.first().and_then(|v| v.as_str()).unwrap_or("DE");
            Ok(json!({"fake": ["iban", country]}))
        }
        // BIC code generation
        "bic" => Ok(json!({"fake": ["bic"]})),
        // LEI code generation
        "lei" => Ok(json!({"fake": ["lei"]})),
        // Alphanumeric with length
        "alphanumeric" => {
            let min_len = args.first().and_then(|v| v.as_u64()).unwrap_or(10);
            let max_len = args.get(1).and_then(|v| v.as_u64()).unwrap_or(min_len);
            Ok(json!({"fake": ["alphanumeric", min_len, max_len]}))
        }
        // Words generation
        "words" => {
            let min = args.first().and_then(|v| v.as_u64()).unwrap_or(1);
            let max = args.get(1).and_then(|v| v.as_u64()).unwrap_or(3);
            Ok(json!({"fake": ["words", min, max]}))
        }
        // Regex patterns - pass through as-is, datafake-rs now supports it
        "regex" => {
            let mut fake_array = vec![json!("regex")];
            for arg in args {
                fake_array.push(arg.clone());
            }
            Ok(json!({"fake": fake_array}))
        }
        // Pass through other types as-is, datafake-rs supports most common ones
        _ => {
            let mut fake_array = vec![json!(fake_type)];
            for arg in args {
                fake_array.push(arg.clone());
            }
            Ok(json!({"fake": fake_array}))
        }
    }
}

/// Convert schema section to datafake-rs compatible format
fn convert_schema(schema: &Value) -> Result<Value> {
    process_schema_value(schema)
}

/// Process schema value recursively
fn process_schema_value(value: &Value) -> Result<Value> {
    match value {
        Value::Object(obj) => {
            let mut result = serde_json::Map::new();

            for (key, val) in obj {
                // Handle special keys
                if key == "@Ccy" {
                    // This is an attribute in XML, keep as-is
                    result.insert(key.clone(), process_schema_value(val)?);
                } else if key == "$value" {
                    // This is the element value in XML, keep as-is
                    result.insert(key.clone(), process_schema_value(val)?);
                } else if key == "var" {
                    // Variable reference, keep as-is (datafake-rs supports this)
                    return Ok(value.clone());
                } else if key == "fake" || key == "cat" || key == "pick" {
                    // These are datafake-rs operations, process the spec
                    return convert_variable_spec(value);
                } else {
                    // Regular field, process recursively
                    result.insert(key.clone(), process_schema_value(val)?);
                }
            }

            Ok(Value::Object(result))
        }
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                result.push(process_schema_value(item)?);
            }
            Ok(Value::Array(result))
        }
        _ => Ok(value.clone()),
    }
}

/// Extract document from envelope for backward compatibility
fn extract_document_from_envelope(envelope: &Value, message_type: &str) -> Result<Value> {
    let document = envelope.get("Document").ok_or_else(|| {
        ValidationError::new(9997, "Envelope missing 'Document' section".to_string())
    })?;

    // Extract the inner document based on message type
    let doc_root = get_document_root_element(message_type);

    document
        .get(&doc_root)
        .cloned()
        .ok_or_else(|| ValidationError::new(9997, format!("Document missing '{doc_root}' element")))
}

/// Get the document root element name for a message type
fn get_document_root_element(message_type: &str) -> String {
    match message_type {
        "pacs008" => "FIToFICstmrCdtTrf",
        "pacs009" => "FinInstnCdtTrf",
        "pacs003" => "FIToFICstmrDrctDbt",
        "pacs004" => "PmtRtr",
        "pacs002" => "FIToFIPmtStsRpt",
        "pain001" => "CstmrCdtTrfInitn",
        "pain008" => "CstmrDrctDbtInitn",
        "camt025" => "Rcpt",
        "camt029" => "RsltnOfInvstgtn",
        "camt052" => "BkToCstmrAcctRpt",
        "camt053" => "BkToCstmrStmt",
        "camt054" => "BkToCstmrDbtCdtNtfctn",
        "camt056" => "FIToFIPmtCxlReq",
        "camt057" => "NtfctnToRcv",
        "camt060" => "AcctRptgReq",
        _ => "Document", // Fallback
    }
    .to_string()
}

/// Convert envelope JSON to typed XML
fn envelope_to_xml(envelope: Value, message_type: &str) -> Result<String> {
    // Parse and generate XML based on message type
    match message_type {
        "pacs008" => {
            use crate::document::pacs_008_001_08::FIToFICustomerCreditTransferV08;
            use crate::header::bah_pacs_008_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pacs008 header: {e}"))
                })?;

            let message: FIToFICustomerCreditTransferV08 = serde_json::from_value(
                envelope["Document"]["FIToFICstmrCdtTrf"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse pacs008 document: {e}"))
            })?;

            to_mx_xml(&message, header, "pacs.008", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs009" => {
            use crate::document::pacs_009_001_08::FinancialInstitutionCreditTransferV08;
            use crate::header::bah_pacs_009_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pacs009 header: {e}"))
                })?;

            let message: FinancialInstitutionCreditTransferV08 = serde_json::from_value(
                envelope["Document"]["FinInstnCdtTrf"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse pacs009 document: {e}"))
            })?;

            to_mx_xml(&message, header, "pacs.009", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs003" => {
            use crate::document::pacs_003_001_08::FIToFICustomerDirectDebitV08;
            use crate::header::bah_pacs_003_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pacs003 header: {e}"))
                })?;

            let message: FIToFICustomerDirectDebitV08 =
                serde_json::from_value(envelope["Document"]["FIToFICstmrDrctDbt"].clone())
                    .map_err(|e| {
                        ValidationError::new(9997, format!("Failed to parse pacs003 document: {e}"))
                    })?;

            to_mx_xml(&message, header, "pacs.003", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs004" => {
            use crate::document::pacs_004_001_09::PaymentReturnV09;
            use crate::header::bah_pacs_004_001_09::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pacs004 header: {e}"))
                })?;

            let message: PaymentReturnV09 =
                serde_json::from_value(envelope["Document"]["PmtRtr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pacs004 document: {e}"))
                })?;

            to_mx_xml(&message, header, "pacs.004", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs002" => {
            use crate::document::pacs_002_001_10::FIToFIPaymentStatusReportV10;
            use crate::header::bah_pacs_002_001_10::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pacs002 header: {e}"))
                })?;

            let message: FIToFIPaymentStatusReportV10 = serde_json::from_value(
                envelope["Document"]["FIToFIPmtStsRpt"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse pacs002 document: {e}"))
            })?;

            to_mx_xml(&message, header, "pacs.002", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pain001" => {
            use crate::document::pain_001_001_09::CustomerCreditTransferInitiationV09;
            use crate::header::bah_pain_001_001_09::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pain001 header: {e}"))
                })?;

            let message: CustomerCreditTransferInitiationV09 = serde_json::from_value(
                envelope["Document"]["CstmrCdtTrfInitn"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse pain001 document: {e}"))
            })?;

            to_mx_xml(&message, header, "pain.001", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pain008" => {
            use crate::document::pain_008_001_08::CustomerDirectDebitInitiationV08;
            use crate::header::bah_pain_008_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse pain008 header: {e}"))
                })?;

            let message: CustomerDirectDebitInitiationV08 = serde_json::from_value(
                envelope["Document"]["CstmrDrctDbtInitn"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse pain008 document: {e}"))
            })?;

            to_mx_xml(&message, header, "pain.008", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt025" => {
            use crate::document::camt_025_001_08::ReceiptV08;
            use crate::header::bah_camt_025_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt025 header: {e}"))
                })?;

            let message: ReceiptV08 = serde_json::from_value(envelope["Document"]["Rcpt"].clone())
                .map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt025 document: {e}"))
                })?;

            to_mx_xml(&message, header, "camt.025", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt029" => {
            use crate::document::camt_029_001_09::ResolutionOfInvestigationV09;
            use crate::header::bah_camt_029_001::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt029 header: {e}"))
                })?;

            let message: ResolutionOfInvestigationV09 = serde_json::from_value(
                envelope["Document"]["RsltnOfInvstgtn"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse camt029 document: {e}"))
            })?;

            to_mx_xml(&message, header, "camt.029", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt052" => {
            use crate::document::camt_052_001_08::BankToCustomerAccountReportV08;
            use crate::header::bah_camt_052_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt052 header: {e}"))
                })?;

            let message: BankToCustomerAccountReportV08 = serde_json::from_value(
                envelope["Document"]["BkToCstmrAcctRpt"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse camt052 document: {e}"))
            })?;

            to_mx_xml(&message, header, "camt.052", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt053" => {
            use crate::document::camt_053_001_08::BankToCustomerStatementV08;
            use crate::header::bah_camt_053_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt053 header: {e}"))
                })?;

            let message: BankToCustomerStatementV08 = serde_json::from_value(
                envelope["Document"]["BkToCstmrStmt"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse camt053 document: {e}"))
            })?;

            to_mx_xml(&message, header, "camt.053", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt054" => {
            use crate::document::camt_054_001_08::BankToCustomerDebitCreditNotificationV08;
            use crate::header::bah_camt_054_001::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt054 header: {e}"))
                })?;

            let message: BankToCustomerDebitCreditNotificationV08 =
                serde_json::from_value(envelope["Document"]["BkToCstmrDbtCdtNtfctn"].clone())
                    .map_err(|e| {
                        ValidationError::new(9997, format!("Failed to parse camt054 document: {e}"))
                    })?;

            to_mx_xml(&message, header, "camt.054", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt056" => {
            use crate::document::camt_056_001_08::FIToFIPaymentCancellationRequestV08;
            use crate::header::bah_camt_056_001_08::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt056 header: {e}"))
                })?;

            let message: FIToFIPaymentCancellationRequestV08 = serde_json::from_value(
                envelope["Document"]["FIToFIPmtCxlReq"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse camt056 document: {e}"))
            })?;

            to_mx_xml(&message, header, "camt.056", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt057" => {
            use crate::document::camt_057_001_06::NotificationToReceiveV06;
            use crate::header::bah_camt_057_001_06::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt057 header: {e}"))
                })?;

            let message: NotificationToReceiveV06 = serde_json::from_value(
                envelope["Document"]["NtfctnToRcv"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse camt057 document: {e}"))
            })?;

            to_mx_xml(&message, header, "camt.057", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt060" => {
            use crate::document::camt_060_001_05::AccountReportingRequestV05;
            use crate::header::bah_camt_060_001_05::BusinessApplicationHeaderV02;

            let header: BusinessApplicationHeaderV02 =
                serde_json::from_value(envelope["AppHdr"].clone()).map_err(|e| {
                    ValidationError::new(9997, format!("Failed to parse camt060 header: {e}"))
                })?;

            let message: AccountReportingRequestV05 = serde_json::from_value(
                envelope["Document"]["AcctRptgReq"].clone(),
            )
            .map_err(|e| {
                ValidationError::new(9997, format!("Failed to parse camt060 document: {e}"))
            })?;

            to_mx_xml(&message, header, "camt.060", None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        _ => Err(ValidationError::new(
            9997,
            format!("Unsupported message type: {message_type}"),
        )),
    }
}
