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
use fake::Fake;
use rand::Rng;
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

    // Process the scenario to generate complete envelope
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

/// Generate complete envelope from scenario JSON
fn generate_envelope_from_scenario(scenario: &Value) -> Result<Value> {
    let mut rng = rand::thread_rng();

    // Extract variables and schema
    let variables = scenario.get("variables").ok_or_else(|| {
        ValidationError::new(9997, "Scenario missing 'variables' section".to_string())
    })?;

    let schema = scenario.get("schema").ok_or_else(|| {
        ValidationError::new(9997, "Scenario missing 'schema' section".to_string())
    })?;

    // Generate variables
    let mut generated_vars = serde_json::Map::new();
    if let Value::Object(vars) = variables {
        for (key, spec) in vars {
            let value = generate_value(spec, &mut rng)?;
            generated_vars.insert(key.clone(), value);
        }
    }

    // Process complete schema with AppHdr and Document
    let app_hdr = schema.get("AppHdr").ok_or_else(|| {
        ValidationError::new(9997, "Scenario schema missing 'AppHdr' section".to_string())
    })?;

    let document = schema.get("Document").ok_or_else(|| {
        ValidationError::new(
            9997,
            "Scenario schema missing 'Document' section".to_string(),
        )
    })?;

    // Process both header and document with generated variables
    let processed_app_hdr = process_schema_value(app_hdr, &generated_vars)?;
    let processed_document = process_schema_value(document, &generated_vars)?;

    Ok(json!({
        "AppHdr": processed_app_hdr,
        "Document": processed_document
    }))
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

/// Generate a value based on specification
fn generate_value(spec: &Value, rng: &mut impl Rng) -> Result<Value> {
    match spec {
        Value::Object(obj) => {
            if let Some(fake_spec) = obj.get("fake") {
                if let Value::Array(fake_arr) = fake_spec {
                    if let Some(Value::String(fake_type)) = fake_arr.first() {
                        return generate_fake_value(fake_type, &fake_arr[1..], rng);
                    }
                }
            } else if let Some(cat_spec) = obj.get("cat") {
                if let Value::Array(parts) = cat_spec {
                    // If only one element and it's an object, evaluate it directly
                    if parts.len() == 1 {
                        if let Some(Value::Object(_)) = parts.first() {
                            let generated = generate_value(&parts[0], rng)?;
                            // Convert numbers to strings for single element cat
                            if let Value::Number(n) = generated {
                                return Ok(Value::String(n.to_string()));
                            }
                            return Ok(generated);
                        }
                    }

                    let mut result = String::new();
                    for part in parts {
                        match part {
                            Value::String(s) => result.push_str(s),
                            Value::Object(_) => {
                                let generated = generate_value(part, rng)?;
                                if let Value::String(s) = generated {
                                    result.push_str(&s);
                                } else if let Value::Number(n) = generated {
                                    result.push_str(&n.to_string());
                                } else {
                                    result.push_str(&generated.to_string());
                                }
                            }
                            _ => result.push_str(&part.to_string()),
                        }
                    }
                    return Ok(Value::String(result));
                }
            } else if let Some(Value::Array(options)) = obj.get("pick") {
                let idx = rng.gen_range(0..options.len());
                return Ok(options[idx].clone());
            }
        }
        _ => return Ok(spec.clone()),
    }

    Ok(spec.clone())
}

/// Generate fake values based on type
fn generate_fake_value(fake_type: &str, args: &[Value], rng: &mut impl Rng) -> Result<Value> {
    use fake::faker::address::en::{CityName, CountryName, PostCode, StreetName};
    use fake::faker::company::en::CompanyName;
    use fake::faker::lorem::en::Word;

    match fake_type {
        "bic" => {
            // Generate a realistic BIC code
            let letters: String = (0..4).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
            let country: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
            let location: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
            let branch = if rng.gen_bool(0.5) {
                "XXX".to_string()
            } else {
                (0..3).map(|_| rng.gen_range(b'0'..=b'9') as char).collect()
            };
            Ok(Value::String(format!(
                "{letters}{country}{location}{branch}"
            )))
        }
        "uuid" => Ok(Value::String(uuid::Uuid::new_v4().to_string())),
        "i64" => {
            let min = args.first().and_then(|v| v.as_i64()).unwrap_or(0);
            let max = args.get(1).and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
            Ok(Value::Number(rng.gen_range(min..=max).into()))
        }
        "f64" => {
            let min = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0);
            let max = args.get(1).and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
            let value = rng.gen_range(min..=max);
            // Round to 2 decimal places for currency amounts
            let rounded = (value * 100.0).round() / 100.0;
            Ok(json!(rounded))
        }
        "currency_code" => {
            let codes = ["EUR", "USD", "GBP", "CHF", "JPY", "CAD", "AUD"];
            Ok(Value::String(
                codes[rng.gen_range(0..codes.len())].to_string(),
            ))
        }
        "iso8601_datetime" => {
            let now = chrono::Utc::now();
            Ok(Value::String(now.to_rfc3339()))
        }
        "company_name" => Ok(Value::String(CompanyName().fake())),
        "country_name" => Ok(Value::String(CountryName().fake())),
        "word" => Ok(Value::String(Word().fake())),
        "words" => {
            let min = args.first().and_then(|v| v.as_u64()).unwrap_or(1) as usize;
            let max = args.get(1).and_then(|v| v.as_u64()).unwrap_or(3) as usize;
            let count = rng.gen_range(min..=max);
            let words: Vec<String> = (0..count).map(|_| Word().fake()).collect();
            Ok(Value::String(words.join(" ")))
        }
        "iban" => {
            let country = args.first().and_then(|v| v.as_str()).unwrap_or("DE");
            // Generate a simple IBAN-like string
            let check = format!("{:02}", rng.gen_range(10..99));
            let account: String = (0..18).map(|_| rng.gen_range(0..10).to_string()).collect();
            Ok(Value::String(format!("{country}{check}{account}")))
        }
        "lei" => {
            // Generate a realistic LEI (18 uppercase alphanumeric + 2 check digits)
            let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let lei: String = (0..18)
                .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
                .collect();
            let check = format!("{:02}", rng.gen_range(10..99));
            Ok(Value::String(format!("{lei}{check}")))
        }
        "alphanumeric" => {
            // Generate alphanumeric string of specified length
            let min_len = args.first().and_then(|v| v.as_u64()).unwrap_or(10) as usize;
            let max_len = args
                .get(1)
                .and_then(|v| v.as_u64())
                .unwrap_or(min_len as u64) as usize;
            let len = if min_len == max_len {
                min_len
            } else {
                rng.gen_range(min_len..=max_len)
            };
            let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let result: String = (0..len)
                .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
                .collect();
            Ok(Value::String(result))
        }
        "country_code" => {
            let codes = [
                "US", "GB", "DE", "FR", "IT", "ES", "NL", "BE", "CH", "AT", "SE", "NO", "DK", "FI",
                "IE", "PT", "GR", "PL", "CZ", "HU",
            ];
            Ok(Value::String(
                codes[rng.gen_range(0..codes.len())].to_string(),
            ))
        }
        "postal_code" => Ok(Value::String(PostCode().fake())),
        "postcode" => Ok(Value::String(PostCode().fake())),
        "street_address" => Ok(Value::String(StreetName().fake())),
        "street_name" => Ok(Value::String(StreetName().fake())),
        "city" => Ok(Value::String(CityName().fake())),
        "city_name" => Ok(Value::String(CityName().fake())),
        "date" => {
            // Generate date in specified format
            let format = args.first().and_then(|v| v.as_str()).unwrap_or("%Y-%m-%d");
            let _start_offset = args.get(1).and_then(|v| v.as_str()).unwrap_or("0d");
            let _end_offset = args.get(2).and_then(|v| v.as_str()).unwrap_or("0d");

            // For simplicity, generate today's date
            let now = chrono::Utc::now();
            Ok(Value::String(now.format(format).to_string()))
        }
        "enum" => {
            // Handle enum type: pick one of the provided options
            if !args.is_empty() {
                let idx = rng.gen_range(0..args.len());
                Ok(args[idx].clone())
            } else {
                Ok(Value::String("ENUM_NO_OPTIONS".to_string()))
            }
        }
        "regex" => {
            // Handle regex patterns that define a set of options
            if let Some(Value::String(pattern)) = args.first() {
                // Simple handling for common patterns
                if pattern.starts_with('(') && pattern.ends_with(')') {
                    // Extract options from pattern like "(GB|DE|FR|SG|HK)"
                    let inner = &pattern[1..pattern.len() - 1];
                    let options: Vec<&str> = inner.split('|').collect();
                    if !options.is_empty() {
                        let idx = rng.gen_range(0..options.len());
                        return Ok(Value::String(options[idx].to_string()));
                    }
                }
            }
            Ok(Value::String("REGEX_PATTERN".to_string()))
        }
        "email" => {
            // Generate a realistic email address
            let first_names = [
                "john",
                "jane",
                "admin",
                "info",
                "support",
                "treasury",
                "compliance",
            ];
            let domains = ["example.com", "company.com", "corp.com", "bank.com"];
            let first = first_names[rng.gen_range(0..first_names.len())];
            let domain = domains[rng.gen_range(0..domains.len())];
            Ok(Value::String(format!("{first}@{domain}")))
        }
        "phone_number" => {
            // Generate a phone number
            let area = rng.gen_range(200..999);
            let prefix = rng.gen_range(200..999);
            let line = rng.gen_range(1000..9999);
            Ok(Value::String(format!("+1-{area}-{prefix}-{line}")))
        }
        "state_abbr" => {
            // US state abbreviations
            let states = ["CA", "NY", "TX", "FL", "IL", "PA", "OH", "GA", "NC", "MI"];
            Ok(Value::String(
                states[rng.gen_range(0..states.len())].to_string(),
            ))
        }
        "time" => {
            // Generate time in HH:MM:SS format
            let hour = rng.gen_range(0..24);
            let minute = rng.gen_range(0..60);
            let second = rng.gen_range(0..60);
            Ok(Value::String(format!("{hour:02}:{minute:02}:{second:02}")))
        }
        _ => Ok(Value::String(format!("FAKE_{}", fake_type.to_uppercase()))),
    }
}

/// Process schema value with variable substitution
fn process_schema_value(value: &Value, vars: &serde_json::Map<String, Value>) -> Result<Value> {
    match value {
        Value::Object(obj) => {
            // Check if this is a variable reference first
            if let Some(Value::String(var_name)) = obj.get("var") {
                if let Some(var_value) = vars.get(var_name) {
                    return Ok(var_value.clone());
                }
            }

            // Handle cat spec with variable resolution
            if let Some(Value::Array(parts)) = obj.get("cat") {
                // Process each part to resolve variables
                let resolved_parts: Vec<Value> = parts
                    .iter()
                    .map(|part| process_schema_value(part, vars))
                    .collect::<Result<Vec<_>>>()?;
                let resolved_obj = json!({"cat": resolved_parts});
                let mut rng = rand::thread_rng();
                return generate_value(&resolved_obj, &mut rng);
            }

            // Check if this object itself is a fake/pick spec
            if obj.contains_key("fake") || obj.contains_key("pick") {
                // This is a generator spec, generate it
                let mut rng = rand::thread_rng();
                return generate_value(value, &mut rng);
            }

            let mut result = serde_json::Map::new();

            // Otherwise process each field
            for (key, val) in obj {
                if key != "var" {
                    result.insert(key.clone(), process_schema_value(val, vars)?);
                }
            }
            Ok(Value::Object(result))
        }
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                result.push(process_schema_value(item, vars)?);
            }
            Ok(Value::Array(result))
        }
        _ => Ok(value.clone()),
    }
}
