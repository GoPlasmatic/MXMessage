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
use crate::mx_envelope::BusinessApplicationHeaderBuilder;
use crate::scenario_config::{
    ScenarioConfig, find_scenario_by_name_with_config, find_scenario_for_message_type_with_config,
};
use crate::xml::to_mx_xml;
use fake::Fake;
use rand::Rng;
use serde_json::{Value, json};
use std::path::PathBuf;
use uuid::Uuid;

type Result<T> = std::result::Result<T, ValidationError>;

/// Internal function to generate a sample MX message object (for testing)
///
/// This function is used internally by tests that need the message object
/// rather than the XML string.
#[doc(hidden)]
pub fn generate_sample_object<T>(message_type: &str, scenario_name: Option<&str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    generate_sample_object_with_config(message_type, scenario_name, &ScenarioConfig::default())
}

/// Internal function to generate a sample MX message object with custom configuration (for testing)
#[doc(hidden)]
pub fn generate_sample_object_with_config<T>(
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

    // Process the scenario to generate data
    let generated_data = process_scenario(&scenario_json)?;

    // Convert generated data to string for parsing
    let generated_json = serde_json::to_string_pretty(&generated_data).map_err(|e| {
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

/// Generate a sample MX XML message based on test scenarios
///
/// This function loads a test scenario configuration for the specified message type
/// and generates a complete MX XML message with envelope and header.
///
/// # Arguments
///
/// * `message_type` - The MX message type (e.g., "pacs008", "camt053")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
///
/// # Returns
///
/// Returns a complete MX XML string with envelope and header
///
/// # Example
///
/// ```no_run
/// # use mx_message::sample::generate_sample;
/// // Generate a standard pacs.008 message as XML
/// let pacs008_xml = generate_sample("pacs008", None).unwrap();
///
/// // Generate a specific scenario
/// let pacs008_high_value_xml = generate_sample("pacs008", Some("high_value")).unwrap();
/// ```
pub fn generate_sample(message_type: &str, scenario_name: Option<&str>) -> Result<String> {
    generate_sample_with_config(message_type, scenario_name, &ScenarioConfig::default())
}

/// Generate a sample MX XML message with custom configuration
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
/// # use mx_message::sample::generate_sample_with_config;
/// # use mx_message::scenario_config::ScenarioConfig;
/// # use std::path::PathBuf;
/// // Generate with custom paths
/// let config = ScenarioConfig::with_paths(vec![PathBuf::from("./my_scenarios")]);
/// let pacs008_xml = generate_sample_with_config("pacs008", None, &config).unwrap();
/// ```
pub fn generate_sample_with_config(
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

    // Process the scenario to generate data
    let generated_data = process_scenario(&scenario_json)?;

    // Convert generated data to string for parsing
    let generated_json = serde_json::to_string_pretty(&generated_data).map_err(|e| {
        ValidationError::new(9997, format!("Failed to serialize generated data: {e}"))
    })?;

    // Generate the XML based on message type
    generate_xml_for_message_type(message_type, &generated_json, scenario_name)
}

/// A builder for generating MX XML message samples with custom configuration
///
/// The `SampleGenerator` provides a fluent interface for configuring and generating
/// MX XML message samples with custom scenario paths.
///
/// # Example
///
/// ```no_run
/// # use mx_message::sample::SampleGenerator;
/// # use std::path::PathBuf;
/// let generator = SampleGenerator::new()
///     .with_path(PathBuf::from("./custom_scenarios"))
///     .with_path(PathBuf::from("./backup_scenarios"));
///
/// let pacs008_xml = generator.generate("pacs008", None).unwrap();
/// let pacs008_cbpr_xml = generator.generate("pacs008", Some("cbpr_business_payment")).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct SampleGenerator {
    config: ScenarioConfig,
}

impl Default for SampleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SampleGenerator {
    /// Create a new sample generator with default configuration
    pub fn new() -> Self {
        Self {
            config: ScenarioConfig::default(),
        }
    }

    /// Create a sample generator with specific configuration
    pub fn with_config(config: ScenarioConfig) -> Self {
        Self { config }
    }

    /// Add a path to search for scenario files
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.config = self.config.add_path(path);
        self
    }

    /// Set multiple paths to search for scenario files (replaces existing paths)
    pub fn with_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.config = self.config.set_paths(paths);
        self
    }

    /// Generate a sample MX XML message
    ///
    /// # Arguments
    ///
    /// * `message_type` - The MX message type (e.g., "pacs008", "camt053")
    /// * `scenario_name` - Optional scenario name. If None, uses the default scenario
    pub fn generate(&self, message_type: &str, scenario_name: Option<&str>) -> Result<String> {
        generate_sample_with_config(message_type, scenario_name, &self.config)
    }

    /// Get a reference to the current configuration
    pub fn config(&self) -> &ScenarioConfig {
        &self.config
    }
}

/// Process scenario JSON and generate fake data
fn process_scenario(scenario: &Value) -> Result<Value> {
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

    // Process schema with generated variables
    process_schema_value(schema, &generated_vars)
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
        "uuid" => Ok(Value::String(Uuid::new_v4().to_string())),
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
/// Generate XML for a specific message type
fn generate_xml_for_message_type(
    message_type: &str,
    json_data: &str,
    scenario_name: Option<&str>,
) -> Result<String> {
    // Determine message definition ID and namespace
    let (msg_def_id, namespace_suffix) = match message_type {
        "pacs008" => ("pacs.008.001.08", "pacs.008"),
        "pacs009" => ("pacs.009.001.08", "pacs.009"),
        "pacs003" => ("pacs.003.001.08", "pacs.003"),
        "pacs002" => ("pacs.002.001.10", "pacs.002"),
        "pain001" => ("pain.001.001.09", "pain.001"),
        "pain008" => ("pain.008.001.08", "pain.008"),
        "camt025" => ("camt.025.001.08", "camt.025"),
        "camt029" => ("camt.029.001.09", "camt.029"),
        "camt052" => ("camt.052.001.08", "camt.052"),
        "camt053" => ("camt.053.001.08", "camt.053"),
        "camt054" => ("camt.054.001.08", "camt.054"),
        "camt056" => ("camt.056.001.08", "camt.056"),
        "camt057" => ("camt.057.001.06", "camt.057"),
        "camt060" => ("camt.060.001.05", "camt.060"),
        _ => {
            return Err(ValidationError::new(
                9997,
                format!("Unsupported message type: {message_type}"),
            ));
        }
    };

    // Generate BICs and message ID based on scenario
    let (from_bic, to_bic, msg_id) = generate_header_info(message_type, scenario_name);

    // Build the header
    let mut header_builder = BusinessApplicationHeaderBuilder::new(msg_def_id.to_string())
        .from_bicfi(from_bic)
        .to_bicfi(to_bic)
        .business_message_identifier(msg_id);

    // Add CBPR+ service for CBPR scenarios
    if let Some(scenario) = scenario_name {
        if scenario.contains("cbpr") {
            header_builder = header_builder.business_service("swift.cbprplus.01".to_string());
        }
    }

    let header = header_builder.build();

    // Parse the JSON data and generate XML based on message type
    match message_type {
        "pacs008" => {
            use crate::document::pacs_008_001_08::FIToFICustomerCreditTransferV08;
            let message: FIToFICustomerCreditTransferV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse pacs008: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs009" => {
            use crate::document::pacs_009_001_08::FinancialInstitutionCreditTransferV08;
            let message: FinancialInstitutionCreditTransferV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse pacs009: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs003" => {
            use crate::document::pacs_003_001_08::FIToFICustomerDirectDebitV08;
            let message: FIToFICustomerDirectDebitV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse pacs003: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pacs002" => {
            use crate::document::pacs_002_001_10::FIToFIPaymentStatusReportV10;
            let message: FIToFIPaymentStatusReportV10 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse pacs002: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pain001" => {
            use crate::document::pain_001_001_09::CustomerCreditTransferInitiationV09;
            let message: CustomerCreditTransferInitiationV09 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse pain001: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "pain008" => {
            use crate::document::pain_008_001_08::CustomerDirectDebitInitiationV08;
            let message: CustomerDirectDebitInitiationV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse pain008: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt025" => {
            use crate::document::camt_025_001_08::ReceiptV08;
            let message: ReceiptV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt025: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt029" => {
            use crate::document::camt_029_001_09::ResolutionOfInvestigationV09;
            let message: ResolutionOfInvestigationV09 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt029: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt052" => {
            use crate::document::camt_052_001_08::BankToCustomerAccountReportV08;
            let message: BankToCustomerAccountReportV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt052: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt053" => {
            use crate::document::camt_053_001_08::BankToCustomerStatementV08;
            let message: BankToCustomerStatementV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt053: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt054" => {
            use crate::document::camt_054_001_08::BankToCustomerDebitCreditNotificationV08;
            let message: BankToCustomerDebitCreditNotificationV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt054: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt056" => {
            use crate::document::camt_056_001_08::FIToFIPaymentCancellationRequestV08;
            let message: FIToFIPaymentCancellationRequestV08 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt056: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt057" => {
            use crate::document::camt_057_001_06::NotificationToReceiveV06;
            let message: NotificationToReceiveV06 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt057: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        "camt060" => {
            use crate::document::camt_060_001_05::AccountReportingRequestV05;
            let message: AccountReportingRequestV05 = serde_json::from_str(json_data)
                .map_err(|e| ValidationError::new(9997, format!("Failed to parse camt060: {e}")))?;
            to_mx_xml(&message, header, namespace_suffix, None)
                .map_err(|e| ValidationError::new(9997, format!("Failed to generate XML: {e}")))
        }
        _ => Err(ValidationError::new(
            9997,
            format!("Unsupported message type: {message_type}"),
        )),
    }
}

/// Generate header information based on message type and scenario
fn generate_header_info(
    message_type: &str,
    scenario_name: Option<&str>,
) -> (String, String, String) {
    let scenario_suffix = scenario_name
        .map(|s| s.replace('_', "-").to_uppercase())
        .unwrap_or_else(|| "DEFAULT".to_string());

    let msg_prefix = message_type.to_uppercase();

    // Generate contextual BICs based on scenario
    let (from_bic, to_bic) = if let Some(scenario) = scenario_name {
        match scenario {
            s if s.contains("cbpr") => ("CBPRBNK1XXX".to_string(), "CBPRBNK2XXX".to_string()),
            s if s.contains("high_value") => ("HVBANK1XXX".to_string(), "HVBANK2XXX".to_string()),
            s if s.contains("corporate") => ("CORPBNK1XXX".to_string(), "CORPBNK2XXX".to_string()),
            s if s.contains("treasury") => ("TREASBNKXXX".to_string(), "TREASBNKYYY".to_string()),
            _ => ("SAMPLEB1XXX".to_string(), "SAMPLEB2XXX".to_string()),
        }
    } else {
        ("SAMPLEB1XXX".to_string(), "SAMPLEB2XXX".to_string())
    };

    let msg_id = format!(
        "{}-{}-{}",
        msg_prefix,
        scenario_suffix,
        uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    (from_bic, to_bic, msg_id)
}

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
