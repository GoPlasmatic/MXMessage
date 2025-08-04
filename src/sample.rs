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
use crate::scenario_config::{find_scenario_by_name, find_scenario_for_message_type};
use fake::Fake;
use rand::Rng;
use serde_json::{Value, json};
use uuid::Uuid;

type Result<T> = std::result::Result<T, ValidationError>;

/// Generate a sample MX message based on test scenarios
///
/// This function loads a test scenario configuration for the specified message type
/// and generates a realistic MX message using the fake crate for dynamic data generation.
///
/// # Arguments
///
/// * `message_type` - The MX message type (e.g., "pacs008", "camt053")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
///
/// # Returns
///
/// Returns a complete message object of type T
///
/// # Example
///
/// ```no_run
/// # use mx_message::sample::generate_sample;
/// # use mx_message::document::pacs_008_001_08::FIToFICustomerCreditTransferV08;
/// // Generate a standard pacs.008 message
/// let pacs008_msg: FIToFICustomerCreditTransferV08 = generate_sample("pacs008", None).unwrap();
///
/// // Generate a specific scenario
/// let pacs008_high_value: FIToFICustomerCreditTransferV08 =
///     generate_sample("pacs008", Some("high_value")).unwrap();
/// ```
pub fn generate_sample<T>(message_type: &str, scenario_name: Option<&str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    // Load the scenario configuration JSON
    let scenario_json = if let Some(name) = scenario_name {
        find_scenario_by_name(message_type, name)?
    } else {
        find_scenario_for_message_type(message_type)?
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
                    if let Some(Value::String(fake_type)) = fake_arr.get(0) {
                        return generate_fake_value(fake_type, &fake_arr[1..], rng);
                    }
                }
            } else if let Some(cat_spec) = obj.get("cat") {
                if let Value::Array(parts) = cat_spec {
                    // If only one element and it's an object, evaluate it directly
                    if parts.len() == 1 {
                        if let Some(Value::Object(_)) = parts.get(0) {
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
            } else if let Some(pick_spec) = obj.get("pick") {
                if let Value::Array(options) = pick_spec {
                    let idx = rng.gen_range(0..options.len());
                    return Ok(options[idx].clone());
                }
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
                "{}{}{}{}",
                letters, country, location, branch
            )))
        }
        "uuid" => Ok(Value::String(Uuid::new_v4().to_string())),
        "i64" => {
            let min = args.get(0).and_then(|v| v.as_i64()).unwrap_or(0);
            let max = args.get(1).and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
            Ok(Value::Number(rng.gen_range(min..=max).into()))
        }
        "f64" => {
            let min = args.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0);
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
            let min = args.get(0).and_then(|v| v.as_u64()).unwrap_or(1) as usize;
            let max = args.get(1).and_then(|v| v.as_u64()).unwrap_or(3) as usize;
            let count = rng.gen_range(min..=max);
            let words: Vec<String> = (0..count).map(|_| Word().fake()).collect();
            Ok(Value::String(words.join(" ")))
        }
        "iban" => {
            let country = args.get(0).and_then(|v| v.as_str()).unwrap_or("DE");
            // Generate a simple IBAN-like string
            let check = format!("{:02}", rng.gen_range(10..99));
            let account: String = (0..18).map(|_| rng.gen_range(0..10).to_string()).collect();
            Ok(Value::String(format!("{}{}{}", country, check, account)))
        }
        "lei" => {
            // Generate a realistic LEI (18 uppercase alphanumeric + 2 check digits)
            let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let lei: String = (0..18)
                .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
                .collect();
            let check = format!("{:02}", rng.gen_range(10..99));
            Ok(Value::String(format!("{}{}", lei, check)))
        }
        "alphanumeric" => {
            // Generate alphanumeric string of specified length
            let min_len = args.get(0).and_then(|v| v.as_u64()).unwrap_or(10) as usize;
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
            let format = args.get(0).and_then(|v| v.as_str()).unwrap_or("%Y-%m-%d");
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
            if let Some(pattern_val) = args.get(0) {
                if let Value::String(pattern) = pattern_val {
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
            Ok(Value::String(format!("{}@{}", first, domain)))
        }
        "phone_number" => {
            // Generate a phone number
            let area = rng.gen_range(200..999);
            let prefix = rng.gen_range(200..999);
            let line = rng.gen_range(1000..9999);
            Ok(Value::String(format!("+1-{}-{}-{}", area, prefix, line)))
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
            Ok(Value::String(format!(
                "{:02}:{:02}:{:02}",
                hour, minute, second
            )))
        }
        _ => Ok(Value::String(format!("FAKE_{}", fake_type.to_uppercase()))),
    }
}

/// Process schema value with variable substitution
fn process_schema_value(value: &Value, vars: &serde_json::Map<String, Value>) -> Result<Value> {
    match value {
        Value::Object(obj) => {
            // Check if this is a variable reference first
            if let Some(var_ref) = obj.get("var") {
                if let Value::String(var_name) = var_ref {
                    if let Some(var_value) = vars.get(var_name) {
                        return Ok(var_value.clone());
                    }
                }
            }

            // Handle cat spec with variable resolution
            if let Some(cat_spec) = obj.get("cat") {
                if let Value::Array(parts) = cat_spec {
                    // Process each part to resolve variables
                    let resolved_parts: Vec<Value> = parts
                        .iter()
                        .map(|part| process_schema_value(part, vars))
                        .collect::<Result<Vec<_>>>()?;
                    let resolved_obj = json!({"cat": resolved_parts});
                    let mut rng = rand::thread_rng();
                    return generate_value(&resolved_obj, &mut rng);
                }
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
