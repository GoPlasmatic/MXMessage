use dataflow_rs::Engine;
use dataflow_rs::engine::{AsyncFunctionHandler, Message, Workflow};
use mx_message::plugin::register_mx_functions;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct ScenarioInfo {
    file: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScenarioIndex {
    message_type: String,
    description: String,
    scenarios: Vec<ScenarioInfo>,
}

/// End-to-end test for ISO20022 MX message processing pipeline using dataflow engine
///
/// This test creates a complete dataflow workflow that:
/// 1. Generates sample data from schema using datafake
/// 2. Publishes the data to XML/JSON format
/// 3. Validates the message against schema and business rules
/// 4. Parses the message back to structured format
/// 5. Performs round-trip verification
///
/// The workflow is executed as a pipeline with each task passing its output
/// to the next step through the message context. The workflow is defined in
/// JSON format and executed by the dataflow engine.
///
/// Environment variables:
/// - `TEST_MESSAGE_TYPE`: Test specific message type (e.g., "pacs.008")
/// - `TEST_SCENARIO`: Test specific scenario (e.g., "cbpr_business_payment")
/// - `TEST_DEBUG`: Enable debug output for failures
/// - `TEST_STOP_ON_FAILURE`: Stop testing on first failure (useful with TEST_DEBUG)
/// - `TEST_SAMPLE_COUNT`: Number of samples per scenario (default: 10)
///
/// Examples:
/// ```bash
/// # Test all scenarios
/// cargo test test_mx_workflow_pipeline
///
/// # Test specific message type
/// TEST_MESSAGE_TYPE=pacs.008 cargo test test_mx_workflow_pipeline
///
/// # Debug specific scenario with single sample
/// TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=cbpr_business_payment TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test test_mx_workflow_pipeline -- --nocapture
///
/// # Test with stop on first failure
/// TEST_DEBUG=1 TEST_STOP_ON_FAILURE=1 cargo test test_mx_workflow_pipeline -- --nocapture
/// ```
#[tokio::test]
async fn test_mx_workflow_pipeline() {
    // Get test parameters from environment variables
    let message_type = env::var("TEST_MESSAGE_TYPE").ok();
    let scenario_name = env::var("TEST_SCENARIO").ok();
    let debug_mode = env::var("TEST_DEBUG").is_ok();
    let stop_on_failure = env::var("TEST_STOP_ON_FAILURE").is_ok();
    let samples_str = env::var("TEST_SAMPLE_COUNT").unwrap_or_else(|_| "10".to_string());
    let samples_per_scenario = samples_str.parse::<usize>().unwrap_or(10);

    // Create the dataflow engine with registered MX functions
    let mut custom_functions: HashMap<String, Box<dyn AsyncFunctionHandler + Send + Sync>> =
        HashMap::new();

    // Register all MX plugin functions
    for (name, handler) in register_mx_functions() {
        custom_functions.insert(name.to_string(), handler);
    }

    // Prepare test cases based on environment variables
    let test_cases = get_test_cases(message_type.as_deref(), scenario_name.as_deref());

    if test_cases.is_empty() {
        panic!("No test cases found for the given parameters");
    }

    // Create workflows for unique message types
    let workflow = create_mx_workflow();

    // Create the engine with workflows and custom functions
    let engine = Engine::new([workflow].to_vec(), Some(custom_functions));

    let mut all_results = Vec::new();
    let mut failure_count = 0;

    // Run tests for each scenario
    for (message_type, scenario, description) in &test_cases {
        if debug_mode {
            println!("\n========================================");
            println!("Testing {} - {}", message_type, scenario);
            if scenario != description {
                println!("Description: {}", description);
            }
            println!("========================================");
        }

        // Run multiple samples per scenario
        for sample_idx in 0..samples_per_scenario {
            let schema = match load_scenario_schema(message_type, scenario) {
                Ok(schema) => schema,
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "Failed to load schema for {}/{}: {}",
                            message_type, scenario, e
                        );
                    }
                    failure_count += 1;
                    continue;
                }
            };

            // Create message with scenario in payload
            let mut message = Message::from_value(&schema);

            // Process the message through the engine
            let result = engine.process_message(&mut message).await;

            if debug_mode {
                println!("\n========================================");
                println!("DEBUG - WORKFLOW EXECUTION RESULT");
                println!("========================================");
                println!("{:?}", result);

                println!("\n========================================");
                println!("DEBUG - COMPLETE MESSAGE DATA");
                println!("========================================");
                if let Some(obj) = message.data().as_object() {
                    println!("Total fields in data: {}", obj.len());
                    println!("\nAll fields:");
                    for key in obj.keys() {
                        println!("  - {}", key);
                    }

                    // Show each step's output in detail
                    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("STEP 1: GENERATE - sample_json");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    if let Some(value) = obj.get("sample_json") {
                        // Debug: Extract and print MsgId to trace data flow
                        if let Some(msg_id) = value.pointer("/json_data/AppHdr/BizMsgIdr") {
                            eprintln!("🔍 TEST DEBUG: sample_json contains MsgId: {}", msg_id);
                        }
                        println!(
                            "{}",
                            serde_json::to_string_pretty(value).unwrap_or_default()
                        );
                    } else {
                        println!("❌ sample_json NOT FOUND");
                    }

                    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("STEP 2: PUBLISH - sample_xml");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    if let Some(value) = obj.get("sample_xml") {
                        if let Some(s) = value.as_str() {
                            println!("{}", s);
                        }
                    } else {
                        println!("❌ sample_xml NOT FOUND");
                    }

                    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("STEP 3: VALIDATE - validation_result");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    if let Some(value) = obj.get("validation_result") {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(value).unwrap_or_default()
                        );
                    } else {
                        println!("❌ validation_result NOT FOUND");
                    }

                    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("STEP 4: PARSE - mx_json");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    if let Some(value) = obj.get("mx_json") {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(value).unwrap_or_default()
                        );
                    } else {
                        println!("❌ mx_json NOT FOUND");
                    }

                    // Show all other fields
                    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("OTHER FIELDS");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    for (key, value) in obj {
                        if !["sample_json", "sample_xml", "validation_result", "mx_json"]
                            .contains(&key.as_str())
                        {
                            println!(
                                "\n{}: {}",
                                key,
                                serde_json::to_string_pretty(value).unwrap_or_default()
                            );
                        }
                    }
                }
                println!("\n========================================\n");
            }

            // Analyze the results
            let test_result = match result {
                Ok(_) => {
                    // Analyze the workflow results
                    let workflow_result = analyze_workflow_results(&message, debug_mode);

                    TestResult {
                        message_type: message_type.to_string(),
                        scenario: scenario.to_string(),
                        workflow_completed: true,
                        generate_success: workflow_result.generate_success,
                        publish_success: workflow_result.publish_success,
                        parse_success: workflow_result.parse_success,
                        validation_passed: workflow_result.validate_success,
                        round_trip_success: workflow_result.round_trip_match,
                        error: None,
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\nWorkflow execution failed: {:?}", e);
                    }

                    TestResult {
                        message_type: message_type.to_string(),
                        scenario: scenario.to_string(),
                        workflow_completed: false,
                        generate_success: false,
                        publish_success: false,
                        parse_success: false,
                        validation_passed: false,
                        round_trip_success: false,
                        error: Some(format!("{:?}", e)),
                    }
                }
            };

            // Track failures
            if !test_result.is_fully_successful() {
                failure_count += 1;
                if debug_mode {
                    println!("\n❌ Sample {} failed:", sample_idx);
                    println!("  Workflow Steps:");
                    println!(
                        "    1. Generate: {}",
                        status_symbol(test_result.generate_success)
                    );
                    println!(
                        "    2. Publish: {}",
                        status_symbol(test_result.publish_success)
                    );
                    println!(
                        "    3. Validate: {}",
                        status_symbol(test_result.validation_passed)
                    );
                    println!("    4. Parse: {}", status_symbol(test_result.parse_success));
                    println!(
                        "  Round-trip: {}",
                        status_symbol(test_result.round_trip_success)
                    );

                    if let Some(ref error) = test_result.error {
                        println!("  Error: {}", error);
                    }
                }

                if stop_on_failure {
                    eprintln!("\n⛔ Stopping on first failure (TEST_STOP_ON_FAILURE=1)");
                    all_results.push(test_result);
                    break;
                }
            }

            all_results.push(test_result);
        }

        if stop_on_failure && failure_count > 0 {
            break;
        }
    }

    // Print summary
    print_test_summary(&all_results);

    // Assert based on results
    if failure_count > 0 {
        panic!(
            "\n❌ Workflow test failed: {} out of {} tests failed",
            failure_count,
            all_results.len()
        );
    } else {
        println!("\n✅ All {} workflow tests passed!", all_results.len());
    }
}

/// Get test cases based on environment variables
fn get_test_cases(
    message_type: Option<&str>,
    scenario: Option<&str>,
) -> Vec<(String, String, String)> {
    match (message_type, scenario) {
        (None, None) => {
            // Test all message types and scenarios
            get_all_test_cases()
        }
        (Some(mt), None) => {
            // Test all scenarios for given message type
            let scenarios = get_scenarios_for_message_type(mt);
            scenarios
                .into_iter()
                .map(|s| (mt.to_string(), s.clone(), s))
                .collect()
        }
        (Some(mt), Some(sc)) => {
            // Test specific message type and scenario
            vec![(mt.to_string(), sc.to_string(), sc.to_string())]
        }
        (None, Some(_)) => {
            // Invalid: scenario without message type
            eprintln!("Warning: TEST_SCENARIO requires TEST_MESSAGE_TYPE");
            vec![]
        }
    }
}

/// Get all available test cases from the test_scenarios directory
fn get_all_test_cases() -> Vec<(String, String, String)> {
    let mut test_cases = Vec::new();
    let scenarios_dir = Path::new("test_scenarios");

    if let Ok(entries) = fs::read_dir(scenarios_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Some(dir_name) = path.file_name().and_then(|s| s.to_str())
            {
                // Check if this is a message type directory (e.g., pacs008, camt053)
                if is_message_type_dir(dir_name) {
                    let message_type = format_message_type(dir_name);

                    // Try to load index.json first
                    let index_path = path.join("index.json");
                    if index_path.exists()
                        && let Ok(content) = fs::read_to_string(&index_path)
                        && let Ok(index) = serde_json::from_str::<ScenarioIndex>(&content)
                    {
                        for scenario_info in index.scenarios {
                            let scenario_name = scenario_info.file.trim_end_matches(".json");
                            test_cases.push((
                                message_type.clone(),
                                scenario_name.to_string(),
                                scenario_info.description,
                            ));
                        }
                        continue;
                    }

                    // Fallback to getting scenarios without index
                    let scenarios = get_scenarios_fallback(&message_type);
                    for scenario in scenarios {
                        test_cases.push((message_type.clone(), scenario.clone(), scenario));
                    }
                }
            }
        }
    }

    test_cases
}

/// Check if directory name represents a message type
fn is_message_type_dir(dir_name: &str) -> bool {
    // Check for common ISO20022 message type patterns
    dir_name.starts_with("pacs")
        || dir_name.starts_with("camt")
        || dir_name.starts_with("pain")
        || dir_name.starts_with("admi")
}

/// Format directory name to message type (e.g., "pacs008" -> "pacs.008")
fn format_message_type(dir_name: &str) -> String {
    // Handle formats like pacs008, camt053, etc.
    if dir_name.len() >= 7 {
        let prefix = &dir_name[0..4];
        let suffix = &dir_name[4..];
        format!("{}.{}", prefix, suffix)
    } else {
        dir_name.to_string()
    }
}

/// Get scenarios for a specific message type
fn get_scenarios_for_message_type(message_type: &str) -> Vec<String> {
    // Convert message type to directory name (e.g., "pacs.008" -> "pacs008")
    let dir_name = message_type.replace('.', "");
    let index_path = format!("test_scenarios/{}/index.json", dir_name);

    match fs::read_to_string(&index_path) {
        Ok(content) => match serde_json::from_str::<ScenarioIndex>(&content) {
            Ok(index) => index
                .scenarios
                .into_iter()
                .map(|s| s.file.trim_end_matches(".json").to_string())
                .collect(),
            Err(_) => get_scenarios_fallback(message_type),
        },
        Err(_) => get_scenarios_fallback(message_type),
    }
}

/// Fallback method to get scenarios by directory listing
fn get_scenarios_fallback(message_type: &str) -> Vec<String> {
    let mut scenarios = Vec::new();
    let dir_name = message_type.replace('.', "");
    let scenario_dir = Path::new("test_scenarios").join(&dir_name);

    if let Ok(entries) = fs::read_dir(&scenario_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json")
                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                && stem != "index"
            {
                scenarios.push(stem.to_string());
            }
        }
    }

    scenarios
}

/// Load scenario schema from file
fn load_scenario_schema(message_type: &str, scenario: &str) -> Result<Value, String> {
    let dir_name = message_type.replace('.', "");
    let file_path = format!("test_scenarios/{}/{}.json", dir_name, scenario);

    fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))
        .and_then(|content| {
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON from {}: {}", file_path, e))
        })
}

/// Create the ISO20022 MX processing workflow from JSON definition
fn create_mx_workflow() -> Workflow {
    // Define the workflow in JSON format for better readability and maintainability
    let workflow_json = json!({
        "id": "mx_workflow",
        "name": "ISO20022 MX Processing Pipeline",
        "description": "End-to-end processing pipeline for MX messages",
        "priority": 0,
        "tasks": [
            {
                "id": "step_1_generate",
                "name": "Generate Sample Data",
                "description": "Generate sample data from datafake scenario in payload",
                "function": {
                    "name": "generate_mx",
                    "input": {
                        "target": "sample_json"
                    }
                },
            },
            {
                "id": "step_2_publish",
                "name": "Publish to XML Format",
                "description": "Convert JSON to ISO20022 XML format",
                "function": {
                    "name": "publish_mx",
                    "input": {
                        "source": "sample_json",
                        "target": "sample_xml"
                    }
                },
            },
            {
                "id": "step_3_validate",
                "name": "Validate XML Message",
                "description": "Validate message against schema and business rules",
                "function": {
                    "name": "validate_mx",
                    "input": {
                        "source": "sample_xml",
                        "target": "validation_result",
                    }
                },
            },
            {
                "id": "step_4_parse",
                "name": "Parse XML Message",
                "description": "Parse XML back to structured JSON",
                "function": {
                    "name": "parse_mx",
                    "input": {
                        "source": "sample_xml",
                        "target": "mx_json"
                    }
                },
            }
        ],
    });

    // Convert JSON to Workflow struct using from_json
    let workflow_str =
        serde_json::to_string(&workflow_json).expect("Failed to serialize workflow JSON");
    Workflow::from_json(&workflow_str).expect("Failed to parse workflow JSON")
}

/// Check workflow step results
fn analyze_workflow_results(message: &Message, debug_mode: bool) -> WorkflowResult {
    let mut result = WorkflowResult {
        generate_success: message.data().get("sample_json").is_some(),
        publish_success: message.data().get("sample_xml").is_some(),
        ..Default::default()
    };

    // Step 3: Validate - Check validation results
    if let Some(validation) = message.data().get("validation_result") {
        result.validate_success = validation
            .get("valid")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if let Some(errors) = validation.get("errors").and_then(|e| e.as_array()) {
            result.validation_errors = errors
                .iter()
                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                .collect();
        }
    } else if debug_mode {
        println!("\n  WARNING: No validation_result found in message data");
    }

    // Step 4: Parse - Check if mx_json was created
    result.parse_success = message.data().get("mx_json").is_some();

    // Round-trip: Compare sample_json with mx_json
    result.round_trip_match = check_round_trip_success(message);

    if debug_mode && !result.is_fully_successful() {
        println!("\nWorkflow Step Results:");
        println!("  1. Generate: {}", status_symbol(result.generate_success));
        println!("  2. Publish: {}", status_symbol(result.publish_success));
        println!("  3. Validate: {}", status_symbol(result.validate_success));
        println!("  4. Parse: {}", status_symbol(result.parse_success));
        println!(
            "  Round-trip match: {}",
            status_symbol(result.round_trip_match)
        );

        if !result.validation_errors.is_empty() {
            println!("\n  Validation errors:");
            for error in &result.validation_errors {
                println!("    - {}", error);
            }
        }
    }

    result
}

/// Check if round-trip was successful by comparing sample_json with mx_json
fn check_round_trip_success(message: &Message) -> bool {
    // Extract the original sample_json and the parsed mx_json
    let sample_json = message.data().get("sample_json");
    let mx_json = message.data().get("mx_json");

    if env::var("TEST_DEBUG").is_ok() {
        eprintln!("\n🔍 ROUND-TRIP DEBUG:");
        eprintln!("  sample_json present: {}", sample_json.is_some());
        eprintln!("  mx_json present: {}", mx_json.is_some());

        // Debug: Show MsgId from sample_json
        if let Some(sample) = sample_json {
            if let Some(msg_id) = sample
                .pointer("/json_data/AppHdr/BizMsgIdr")
                .or_else(|| sample.pointer("/json_data/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId"))
            {
                eprintln!("  sample_json MsgId: {}", msg_id);
            }
        }

        // Debug: Show MsgId from mx_json
        if let Some(mx) = mx_json {
            if let Some(msg_id) = mx
                .pointer("/AppHdr/BizMsgIdr")
                .or_else(|| mx.pointer("/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId"))
            {
                eprintln!("  mx_json MsgId: {}", msg_id);
            }
        }
    }

    match (sample_json, mx_json) {
        (Some(original), Some(parsed)) => {
            // sample_json has a json_data wrapper, mx_json might not
            let original_data = original.get("json_data").unwrap_or(original);

            if env::var("TEST_DEBUG").is_ok() {
                eprintln!(
                    "  original has json_data wrapper: {}",
                    original.get("json_data").is_some()
                );
                eprintln!(
                    "  original_data keys: {:?}",
                    original_data
                        .as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                );
            }

            // The publish step extracts only the Document element, so AppHdr is stripped
            // Compare only the Document elements from both sides
            let original_document = original_data.get("Document").unwrap_or(original_data);
            let parsed_document = parsed.get("Document").unwrap_or(parsed);

            if env::var("TEST_DEBUG").is_ok() {
                eprintln!(
                    "  original_document keys: {:?}",
                    original_document
                        .as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                );
                eprintln!(
                    "  parsed_document keys: {:?}",
                    parsed_document
                        .as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                );
            }

            // Extract the actual content - the Document contains one root element with the message data
            // The element name might differ (e.g., "FIToFICstmrCdtTrf" vs "FIToFICustomerCreditTransferV08")
            // so we compare the content of the first element, not the wrapper
            let original_content = original_document
                .as_object()
                .and_then(|obj| obj.values().next())
                .unwrap_or(original_document);
            let parsed_content = parsed_document
                .as_object()
                .and_then(|obj| obj.values().next())
                .unwrap_or(parsed_document);

            // Normalize both documents for comparison
            // XML parsing converts all values to strings, so we need to normalize types
            let normalized_original = normalize_json_types(original_content);
            let normalized_parsed = normalize_json_types(parsed_content);

            // Remove empty optional fields (fields with null, empty objects, or empty arrays)
            // This handles the case where serde's skip_serializing_if drops optional fields
            let cleaned_original = remove_empty_fields(&normalized_original);
            let cleaned_parsed = remove_empty_fields(&normalized_parsed);

            // Debug: compare and show differences if they don't match
            let matches = cleaned_original == cleaned_parsed;
            if !matches && env::var("TEST_DEBUG").is_ok() {
                eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                eprintln!("ROUND-TRIP COMPARISON FAILED");
                eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                eprintln!("\nNormalized & Cleaned Original Document:");
                eprintln!(
                    "{}",
                    serde_json::to_string_pretty(&cleaned_original).unwrap()
                );
                eprintln!("\nNormalized & Cleaned Parsed Document:");
                eprintln!("{}", serde_json::to_string_pretty(&cleaned_parsed).unwrap());
                eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            }

            // Compare normalized JSON
            matches
        }
        _ => false,
    }
}

/// Format a float with a specific number of decimal places for round-trip comparison
/// This rounds to a fixed number of decimal places to handle XML round-trip precision loss
fn format_for_comparison(f: f64, decimal_places: usize) -> String {
    if f == 0.0 {
        return "0".to_string();
    }

    // Format with fixed decimal places
    let formatted = format!("{:.prec$}", f, prec = decimal_places);

    // Remove trailing zeros and decimal point if unnecessary
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

/// Normalize JSON types for round-trip comparison
/// Converts numbers in $value fields to strings to match XML parsing behavior
/// Rounds floating-point numbers to handle precision loss in XML round-trip
fn normalize_json_types(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut normalized = serde_json::Map::new();
            for (key, val) in map {
                if key == "$value" {
                    // Normalize $value to string, removing trailing .0 for floats
                    let normalized_val = match val {
                        serde_json::Value::Number(n) => {
                            // Parse as f64 and format with fixed decimal places
                            let rounded = if let Some(f) = n.as_f64() {
                                f
                            } else if let Some(i) = n.as_i64() {
                                i as f64
                            } else if let Some(u) = n.as_u64() {
                                u as f64
                            } else {
                                n.as_f64().unwrap_or(0.0)
                            };

                            // Round to 4 decimal places to handle XML round-trip precision loss
                            // This is typical precision for financial amounts
                            let s = format_for_comparison(rounded, 4);
                            serde_json::Value::String(s)
                        }
                        serde_json::Value::String(s) => {
                            // If it's already a string, try to parse and normalize it too
                            if let Ok(f) = s.parse::<f64>() {
                                let normalized_str = format_for_comparison(f, 4);
                                serde_json::Value::String(normalized_str)
                            } else {
                                serde_json::Value::String(s.clone())
                            }
                        }
                        serde_json::Value::Bool(b) => serde_json::Value::String(b.to_string()),
                        other => other.clone(),
                    };
                    normalized.insert(key.clone(), normalized_val);
                } else {
                    // Recursively normalize nested objects
                    normalized.insert(key.clone(), normalize_json_types(val));
                }
            }
            serde_json::Value::Object(normalized)
        }
        serde_json::Value::Array(arr) => {
            // Keep arrays as arrays - DO NOT unwrap single-element arrays
            // Many ISO20022 fields are defined as Vec even when they have 1 element
            serde_json::Value::Array(arr.iter().map(normalize_json_types).collect())
        }
        // Normalize numbers to strings (XML converts everything to strings)
        serde_json::Value::Number(n) => {
            // Parse as f64 and format with fixed decimal places
            let rounded = if let Some(f) = n.as_f64() {
                f
            } else if let Some(i) = n.as_i64() {
                i as f64
            } else if let Some(u) = n.as_u64() {
                u as f64
            } else {
                n.as_f64().unwrap_or(0.0)
            };

            // Round to 4 decimal places to handle XML round-trip precision loss
            let s = format_for_comparison(rounded, 4);
            serde_json::Value::String(s)
        }
        // Normalize booleans to strings
        serde_json::Value::Bool(b) => serde_json::Value::String(b.to_string()),
        other => other.clone(),
    }
}

/// Remove empty optional fields from JSON for round-trip comparison
/// This removes:
/// - null values
/// - empty objects {}
/// - empty arrays []
/// This handles serde's skip_serializing_if behavior where optional None fields are dropped
fn remove_empty_fields(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut cleaned = serde_json::Map::new();
            for (key, val) in map {
                // Recursively clean the value first
                let cleaned_val = remove_empty_fields(val);

                // Only include non-empty values
                let should_include = match &cleaned_val {
                    serde_json::Value::Null => false,
                    serde_json::Value::Object(obj) => !obj.is_empty(),
                    serde_json::Value::Array(arr) => !arr.is_empty(),
                    _ => true,
                };

                if should_include {
                    cleaned.insert(key.clone(), cleaned_val);
                }
            }
            serde_json::Value::Object(cleaned)
        }
        serde_json::Value::Array(arr) => {
            let cleaned: Vec<_> = arr
                .iter()
                .map(remove_empty_fields)
                .filter(|v| {
                    // Filter out empty objects and arrays from arrays
                    match v {
                        serde_json::Value::Null => false,
                        serde_json::Value::Object(obj) => !obj.is_empty(),
                        serde_json::Value::Array(arr) => !arr.is_empty(),
                        _ => true,
                    }
                })
                .collect();
            serde_json::Value::Array(cleaned)
        }
        other => other.clone(),
    }
}

#[derive(Debug, Default)]
struct WorkflowResult {
    generate_success: bool,
    publish_success: bool,
    parse_success: bool,
    validate_success: bool,
    round_trip_match: bool,
    validation_errors: Vec<String>,
}

impl WorkflowResult {
    fn is_fully_successful(&self) -> bool {
        self.generate_success
            && self.publish_success
            && self.parse_success
            && self.validate_success
            && self.round_trip_match
    }
}

/// Test result structure tracking each workflow step
#[derive(Debug)]
struct TestResult {
    message_type: String,
    scenario: String,
    workflow_completed: bool,
    generate_success: bool,
    publish_success: bool,
    parse_success: bool,
    validation_passed: bool,
    round_trip_success: bool,
    error: Option<String>,
}

impl TestResult {
    fn is_fully_successful(&self) -> bool {
        self.workflow_completed
            && self.generate_success
            && self.publish_success
            && self.parse_success
            && self.validation_passed
            && self.round_trip_success
    }
}

/// Print test summary
fn print_test_summary(results: &[TestResult]) {
    // Group results by scenario
    let mut scenario_results: HashMap<String, Vec<&TestResult>> = HashMap::new();

    for result in results {
        let key = format!("{}/{}", result.message_type, result.scenario);
        scenario_results.entry(key).or_default().push(result);
    }

    // Sort scenarios for consistent output
    let mut sorted_scenarios: Vec<_> = scenario_results.iter().collect();
    sorted_scenarios.sort_by_key(|(key, _)| key.as_str());

    println!(
        "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗"
    );
    println!(
        "║                                                     ISO20022 MX Workflow Pipeline Test Results                                                            ║"
    );
    println!(
        "╠════════════╤══════════════════════════════════════════════════════════╤════════╤══════════════╤═════════════╤═════════════╤══════════════╤════════════════╣"
    );
    println!(
        "║  Message   │ Scenario                                                 │Samples │   Generate   │   Publish   │   Validate  │    Parse     │   Round-trip   ║"
    );
    println!(
        "╟────────────┼──────────────────────────────────────────────────────────┼────────┼──────────────┼─────────────┼─────────────┼──────────────┼────────────────╢"
    );

    for (scenario_key, scenario_tests) in sorted_scenarios {
        let parts: Vec<&str> = scenario_key.split('/').collect();
        let message_type = parts.first().unwrap_or(&"");
        let scenario_name = parts.get(1).unwrap_or(&"").trim_end_matches(".json");

        let total = scenario_tests.len();
        let generate_pass = scenario_tests.iter().filter(|r| r.generate_success).count();
        let publish_pass = scenario_tests.iter().filter(|r| r.publish_success).count();
        let validation_pass = scenario_tests
            .iter()
            .filter(|r| r.validation_passed)
            .count();
        let parse_pass = scenario_tests.iter().filter(|r| r.parse_success).count();
        let roundtrip_pass = scenario_tests
            .iter()
            .filter(|r| r.round_trip_success)
            .count();

        // Format status strings with exact widths
        let generate_str = format!(
            "{:>3}/{:<3} {}",
            generate_pass,
            total,
            pass_fail_symbol(generate_pass, total)
        );
        let publish_str = format!(
            "{:>3}/{:<3} {}",
            publish_pass,
            total,
            pass_fail_symbol(publish_pass, total)
        );
        let validate_str = format!(
            "{:>3}/{:<3} {}",
            validation_pass,
            total,
            pass_fail_symbol(validation_pass, total)
        );
        let parse_str = format!(
            "{:>3}/{:<3} {}",
            parse_pass,
            total,
            pass_fail_symbol(parse_pass, total)
        );
        let roundtrip_str = format!(
            "{:>3}/{:<3} {:>2}",
            roundtrip_pass,
            total,
            pass_fail_symbol(roundtrip_pass, total)
        );

        println!(
            "║ {:^10} │ {:<56} │{:^8}│ {:^11} │ {:^10} │ {:^10} │ {:^11} │ {:^13} ║",
            message_type,
            scenario_name,
            total,
            generate_str,
            publish_str,
            validate_str,
            parse_str,
            roundtrip_str
        );
    }

    println!(
        "╚════════════╧══════════════════════════════════════════════════════════╧════════╧══════════════╧═════════════╧═════════════╧══════════════╧════════════════╝"
    );

    // Summary statistics
    let total = results.len();
    let generate_success = results.iter().filter(|r| r.generate_success).count();
    let publish_success = results.iter().filter(|r| r.publish_success).count();
    let parse_success = results.iter().filter(|r| r.parse_success).count();
    let validation_success = results.iter().filter(|r| r.validation_passed).count();
    let roundtrip_success = results.iter().filter(|r| r.round_trip_success).count();
    let fully_successful = results.iter().filter(|r| r.is_fully_successful()).count();

    println!("\n📊 Summary:");
    println!("   Total test samples: {}", total);
    println!(
        "   Fully successful: {} ({}%)",
        fully_successful,
        percentage(fully_successful, total)
    );
    println!("\n   Step Success Rates:");
    println!(
        "   1. Generate: {} ({}%)",
        generate_success,
        percentage(generate_success, total)
    );
    println!(
        "   2. Publish: {} ({}%)",
        publish_success,
        percentage(publish_success, total)
    );
    println!(
        "   3. Validate: {} ({}%)",
        validation_success,
        percentage(validation_success, total)
    );
    println!(
        "   4. Parse: {} ({}%)",
        parse_success,
        percentage(parse_success, total)
    );
    println!(
        "   Round-trip match: {} ({}%)",
        roundtrip_success,
        percentage(roundtrip_success, total)
    );
}

fn pass_fail_symbol(pass_count: usize, total_count: usize) -> &'static str {
    if total_count == 0 {
        "⏭️"
    } else if pass_count == total_count {
        "✅"
    } else if pass_count == 0 {
        "❌"
    } else {
        " ⚠️  "
    }
}

fn status_symbol(success: bool) -> &'static str {
    if success { "✅" } else { "❌" }
}

fn percentage(value: usize, total: usize) -> usize {
    if total == 0 { 0 } else { (value * 100) / total }
}
