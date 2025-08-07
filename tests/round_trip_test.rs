use mx_message::parse_result::ParserConfig;
use mx_message::sample::generate_sample;
use mx_message::validation::Validate;
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct ScenarioIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mx_message_type: Option<String>,
    scenarios: Vec<ScenarioInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScenarioInfo {
    file: String,
    description: String,
}

#[derive(Debug)]
enum ValidationOrSerializationError {
    Validation(Vec<String>),
    Serialization(String),
}

#[derive(Debug)]
struct TestResult {
    message_type: String,
    scenario: String,
    parse_status: &'static str,
    validation_status: &'static str,
    xml_serialization_status: &'static str,
    xml_roundtrip_status: &'static str,
    error_stage: Option<String>,
}

impl TestResult {
    fn new(message_type: String, scenario: String) -> Self {
        Self {
            message_type,
            scenario,
            parse_status: "⏳",
            validation_status: "⏳",
            xml_serialization_status: "⏳",
            xml_roundtrip_status: "⏳",
            error_stage: None,
        }
    }

    fn mark_parse_failed(&mut self, error: String) {
        self.parse_status = "❌";
        self.validation_status = "❔";
        self.xml_serialization_status = "❔";
        self.xml_roundtrip_status = "❔";
        self.error_stage = Some(format!("Parse: {error}"));
    }

    fn mark_parse_success(&mut self) {
        self.parse_status = "✅";
    }

    fn mark_validation_failed(&mut self, errors: Vec<String>) {
        self.validation_status = "❌";
        let error_count = errors.len();
        if error_count == 1 {
            self.error_stage = Some(format!("Validation: {}", errors[0]));
        } else {
            self.error_stage = Some(format!("Validation: {error_count} errors"));
        }
    }

    fn mark_validation_success(&mut self) {
        self.validation_status = "✅";
    }

    fn mark_xml_serialization_failed(&mut self, error: String) {
        self.xml_serialization_status = "❌";
        self.error_stage = Some(format!("XML Serialization: {error}"));
    }

    fn mark_xml_serialization_success(&mut self) {
        self.xml_serialization_status = "✅";
    }

    fn mark_xml_roundtrip_failed(&mut self, stage: &str) {
        self.xml_roundtrip_status = "❌";
        self.error_stage = Some(format!("XML Roundtrip: {stage}"));
    }

    fn mark_xml_roundtrip_success(&mut self) {
        self.xml_roundtrip_status = "✅";
    }
}

// Helper function to validate using the new API
fn validate_message<T: Validate>(msg: &T) -> Result<(), Vec<String>> {
    let config = ParserConfig::default();
    let mut collector = mx_message::parse_result::ErrorCollector::new();
    
    // Call validate which collects errors
    msg.validate("", &config, &mut collector);
    
    if !collector.has_errors() {
        Ok(())
    } else {
        let error_messages: Vec<String> = collector
            .errors()
            .into_iter()
            .map(|e| format!("{} (code: {})", e.message, e.code))
            .collect();
        Err(error_messages)
    }
}

fn test_single_scenario(
    message_type: &str,
    scenario_name: &str,
    test_index: usize,
    debug_mode: bool,
) -> TestResult {
    let mut result = TestResult::new(message_type.to_string(), scenario_name.to_string());

    // Skip unsupported message types entirely
    if message_type == "camt.027" || message_type == "camt.028" {
        // Mark as skipped by setting all statuses to a special value
        result.parse_status = "⏭️";
        result.validation_status = "⏭️";
        result.xml_serialization_status = "⏭️";
        result.xml_roundtrip_status = "⏭️";
        result.error_stage = Some(format!(
            "Skipped: Message type {} not implemented",
            message_type
        ));
        return result;
    }

    // Convert message type format for generate_sample (e.g., "pacs.008" -> "pacs008")
    let msg_type_for_generate = message_type.replace(".", "");

    // Generate sample message
    let generated_json: serde_json::Value =
        match generate_sample(&msg_type_for_generate, Some(scenario_name)) {
            Ok(json) => json,
            Err(e) => {
                result.mark_parse_failed(format!("Generation error: {:?}", e));
                if debug_mode {
                    eprintln!("\n[Test {test_index}] Generation failed: {:?}", e);
                    eprintln!("Message type: {message_type} (using {msg_type_for_generate})");
                    eprintln!("Scenario: {scenario_name}");

                    // Try to read the scenario file for debugging
                    let scenario_file = format!(
                        "test_scenarios/{}/{}.json",
                        msg_type_for_generate, scenario_name
                    );
                    if let Ok(content) = std::fs::read_to_string(&scenario_file) {
                        eprintln!("Scenario file content (first 500 chars):");
                        eprintln!("{}", &content[..content.len().min(500)]);
                    }
                }
                return result;
            }
        };

    result.mark_parse_success();

    // Parse JSON to MX structure and validate
    let mx_validation_result =
        parse_and_validate_mx_message(&generated_json, message_type, debug_mode, test_index);

    match mx_validation_result {
        Ok((mx_struct_json, xml_string)) => {
            result.mark_validation_success();
            result.mark_xml_serialization_success();

            // Test XML round-trip
            match parse_xml_back_to_json(&xml_string, message_type, debug_mode, test_index) {
                Ok(roundtrip_json) => {
                    // Compare the MX structure JSON (not the original generated JSON)
                    if mx_struct_json == roundtrip_json {
                        result.mark_xml_roundtrip_success();
                    } else {
                        result.mark_xml_roundtrip_failed("JSON mismatch after XML roundtrip");
                        if debug_mode {
                            eprintln!("\n[Test {test_index}] XML roundtrip mismatch");
                            eprintln!("Original MX JSON: {:?}", mx_struct_json);
                            eprintln!("Roundtrip JSON: {:?}", roundtrip_json);
                        }
                    }
                }
                Err(e) => {
                    result.mark_xml_roundtrip_failed(&e);
                }
            }
        }
        Err(ValidationOrSerializationError::Validation(errors)) => {
            result.mark_validation_failed(errors);
        }
        Err(ValidationOrSerializationError::Serialization(error)) => {
            result.mark_xml_serialization_failed(error);
        }
    }

    result
}

fn get_message_types() -> Vec<String> {
    let scenarios_dir = Path::new("test_scenarios");
    let mut message_types = Vec::new();

    if let Ok(entries) = fs::read_dir(scenarios_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                        // Convert directory names like "pacs008" to "pacs.008"
                        if name.starts_with("pacs")
                            || name.starts_with("pain")
                            || name.starts_with("camt")
                        {
                            let msg_type = if name.len() >= 7 {
                                format!("{}.{}", &name[0..4], &name[4..7])
                            } else {
                                name.to_string()
                            };
                            message_types.push(msg_type);
                        }
                    }
                }
            }
        }
    }

    message_types.sort();
    message_types
}

fn get_scenarios_for_message_type(message_type: &str) -> Vec<String> {
    let dir_name = message_type.replace(".", "");
    let index_path = format!("test_scenarios/{}/index.json", dir_name);

    match fs::read_to_string(&index_path) {
        Ok(content) => match serde_json::from_str::<ScenarioIndex>(&content) {
            Ok(index) => index
                .scenarios
                .into_iter()
                .map(|s| {
                    // Extract scenario name from file name (remove .json extension)
                    s.file.trim_end_matches(".json").to_string()
                })
                .collect(),
            Err(e) => {
                eprintln!("Failed to parse index.json for {message_type}: {e}");
                Vec::new()
            }
        },
        Err(e) => {
            eprintln!("Failed to read index.json for {message_type}: {e}");
            Vec::new()
        }
    }
}

/// Round-trip test for MX (ISO20022) message scenarios
///
/// Environment variables:
/// - `TEST_MESSAGE_TYPE`: Test specific message type (e.g., "pacs.008")
/// - `TEST_SCENARIO`: Test specific scenario (e.g., "standard")
/// - `TEST_DEBUG`: Enable debug output for failures
/// - `TEST_STOP_ON_FAILURE`: Stop testing on first failure (useful with TEST_DEBUG)
/// - `TEST_SAMPLE_COUNT`: Number of samples per scenario (default: 10)
///
/// Examples:
/// ```bash
/// # Test all scenarios
/// cargo test round_trip_scenarios
///
/// # Test specific message type
/// TEST_MESSAGE_TYPE=pacs.008 cargo test round_trip_scenarios
///
/// # Debug specific scenario with single sample
/// TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=standard TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test round_trip_scenarios -- --nocapture
///
/// # Debug with stop on first failure
/// TEST_MESSAGE_TYPE=pacs.008 TEST_DEBUG=1 TEST_STOP_ON_FAILURE=1 cargo test round_trip_scenarios -- --nocapture
/// ```
#[test]
fn test_round_trip_scenarios() {
    // Get test parameters from environment variables
    let message_type = env::var("TEST_MESSAGE_TYPE").ok();
    let scenario_name = env::var("TEST_SCENARIO").ok();
    let debug_mode = env::var("TEST_DEBUG").is_ok();
    let stop_on_failure = env::var("TEST_STOP_ON_FAILURE").is_ok();
    let samples_str = env::var("TEST_SAMPLE_COUNT").unwrap_or_else(|_| "10".to_string());
    let samples_per_scenario = samples_str.parse::<usize>().unwrap_or(10);

    let mut test_results = Vec::new();

    if debug_mode {
        eprintln!("🔍 Debug mode enabled");
        eprintln!("   Samples per scenario: {samples_per_scenario}");
        if stop_on_failure {
            eprintln!("   Stop on first failure: enabled");
        }
        if let Some(ref mt) = message_type {
            eprintln!("   Message type: {mt}");
        }
        if let Some(ref sc) = scenario_name {
            eprintln!("   Scenario: {sc}");
        }
        eprintln!();
    }

    match (message_type, scenario_name) {
        (None, None) => {
            // No parameters: test all message types and all scenarios
            let message_types = get_message_types();
            println!("Testing all message types: {message_types:?}");

            for message_type in message_types {
                let scenarios = get_scenarios_for_message_type(&message_type);
                println!("\n{}: Testing {} scenarios", message_type, scenarios.len());

                for scenario in scenarios {
                    println!(
                        "  Testing {message_type}/{scenario} ({samples_per_scenario} samples)..."
                    );
                    for i in 0..samples_per_scenario {
                        let result =
                            test_single_scenario(&message_type, &scenario, i + 1, debug_mode);
                        let is_failure = result.parse_status == "❌"
                            || result.validation_status == "❌"
                            || result.xml_serialization_status == "❌"
                            || result.xml_roundtrip_status == "❌";
                        test_results.push(result);

                        if is_failure && stop_on_failure {
                            eprintln!("\n⛔ Stopping on first failure (TEST_STOP_ON_FAILURE=1)");
                            break;
                        }
                    }
                }
            }
        }
        (Some(message_type), None) => {
            // One parameter: test all scenarios for given message type
            let scenarios = get_scenarios_for_message_type(&message_type);

            if scenarios.is_empty() {
                panic!("No scenarios found for message type: {message_type}");
            }

            println!("Testing {}: {} scenarios", message_type, scenarios.len());

            for scenario in scenarios {
                println!("  Testing {message_type}/{scenario} ({samples_per_scenario} samples)...");
                for i in 0..samples_per_scenario {
                    let result = test_single_scenario(&message_type, &scenario, i + 1, debug_mode);
                    test_results.push(result);
                }
            }
        }
        (Some(message_type), Some(scenario)) => {
            // Two parameters: test specific scenario for given message type
            println!("Testing {message_type}/{scenario} ({samples_per_scenario} samples)...");

            for i in 0..samples_per_scenario {
                let result = test_single_scenario(&message_type, &scenario, i + 1, debug_mode);
                test_results.push(result);
            }
        }
        _ => {
            // Invalid combination
            panic!(
                "Invalid test parameters. Use TEST_MESSAGE_TYPE and TEST_SCENARIO environment variables."
            );
        }
    }

    // Print results summary
    print_results_summary(&test_results);

    // Determine if test should fail
    let failed_results: Vec<_> = test_results
        .iter()
        .enumerate()
        .filter(|(_, r)| {
            // Don't count skipped tests as failures
            r.parse_status != "⏭️"
                && (r.parse_status == "❌"
                    || r.validation_status == "❌"
                    || r.xml_serialization_status == "❌"
                    || r.xml_roundtrip_status == "❌")
        })
        .collect();

    if !failed_results.is_empty() {
        println!("\n❌ Failed tests: {}", failed_results.len());

        // Group failures by message type and scenario
        let mut failure_summary: std::collections::HashMap<String, Vec<(usize, &TestResult)>> =
            std::collections::HashMap::new();

        for (idx, result) in &failed_results {
            let key = format!("{}/{}", result.message_type, result.scenario);
            failure_summary
                .entry(key)
                .or_default()
                .push((*idx, *result));
        }

        println!("\nFailure summary:");
        for (key, failures) in &failure_summary {
            println!("  {} - {} failures", key, failures.len());

            if debug_mode && failures.len() <= 5 {
                // Show details for first few failures
                for (test_idx, result) in failures.iter().take(3) {
                    println!(
                        "    Test #{}: {}",
                        test_idx + 1,
                        result
                            .error_stage
                            .as_ref()
                            .unwrap_or(&"Unknown error".to_string())
                    );
                }
                if failures.len() > 3 {
                    println!("    ... and {} more", failures.len() - 3);
                }
            }
        }

        if debug_mode && failed_results.len() <= 10 {
            println!("\n💡 Tip: To debug a specific failure, run:");
            let (_, first_failure) = &failed_results[0];
            println!(
                "   TEST_MESSAGE_TYPE={} TEST_SCENARIO={} TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test round_trip_scenarios -- --nocapture",
                first_failure.message_type, first_failure.scenario
            );
        }

        panic!(
            "Round-trip test failed: {} out of {} tests failed",
            failed_results.len(),
            test_results.len()
        );
    }

    // Count skipped tests
    let skipped_count = test_results
        .iter()
        .filter(|r| r.parse_status == "⏭️")
        .count();
    let passed_count = test_results.len() - skipped_count;

    if skipped_count > 0 {
        println!(
            "\n✅ All {} tests passed! ({} tests skipped for unsupported message types)",
            passed_count, skipped_count
        );
    } else {
        println!("\n✅ All {} tests passed!", test_results.len());
    }
}

fn print_results_summary(results: &[TestResult]) {
    // Group results by scenario for aggregate view
    let mut scenario_results: std::collections::HashMap<String, Vec<&TestResult>> =
        std::collections::HashMap::new();

    for result in results {
        let key = format!("{}/{}", result.message_type, result.scenario);
        scenario_results.entry(key).or_default().push(result);
    }

    // Sort scenarios by name
    let mut scenarios: Vec<_> = scenario_results.into_iter().collect();
    scenarios.sort_by_key(|k| k.0.clone());

    // Calculate column widths
    let scenario_width = scenarios
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(20)
        .max(20);

    // Print table header
    println!(
        "\n╔═{}═╤═{}═╤═{}═╤═{}═╤═{}═╗",
        "═".repeat(scenario_width),
        "═".repeat(10),
        "═".repeat(12),
        "═".repeat(12),
        "═".repeat(15)
    );
    println!(
        "║ {:<width$} │  {:^8}  │ {:^12} │ {:^12} │ {:^15} ║",
        "Scenario",
        "Parser",
        "Validation",
        "XML Serialize",
        "XML Roundtrip",
        width = scenario_width
    );
    println!(
        "╟─{}─┼─{}─┼─{}─┼─{}─┼─{}─╢",
        "─".repeat(scenario_width),
        "─".repeat(10),
        "─".repeat(12),
        "─".repeat(12),
        "─".repeat(15)
    );

    // Print each scenario's aggregate results
    for (scenario_name, results_group) in scenarios {
        let total = results_group.len();
        let parse_success = results_group
            .iter()
            .filter(|r| r.parse_status == "✅")
            .count();
        let validation_success = results_group
            .iter()
            .filter(|r| r.validation_status == "✅")
            .count();
        let xml_serialization_success = results_group
            .iter()
            .filter(|r| r.xml_serialization_status == "✅")
            .count();
        let xml_roundtrip_success = results_group
            .iter()
            .filter(|r| r.xml_roundtrip_status == "✅")
            .count();

        // Determine status symbols based on success rates
        let parse_symbol = get_status_symbol(parse_success, total);
        let validation_symbol = get_status_symbol(validation_success, total);
        let xml_serialization_symbol = get_status_symbol(xml_serialization_success, total);
        let xml_roundtrip_symbol = get_status_symbol(xml_roundtrip_success, total);

        println!(
            "║ {scenario_name:<scenario_width$} │    {parse_symbol:^2}    │      {validation_symbol:^2}     │      {xml_serialization_symbol:^2}     │       {xml_roundtrip_symbol:^2}       ║"
        );
    }

    // Print table footer
    println!(
        "╚═{}═╧═{}═╧═{}═╧═{}═╧═{}═╝",
        "═".repeat(scenario_width),
        "═".repeat(10),
        "═".repeat(12),
        "═".repeat(12),
        "═".repeat(15)
    );

    // Print summary statistics
    let total_tests = results.len();
    let parse_success_total = results.iter().filter(|r| r.parse_status == "✅").count();
    let validation_success_total = results
        .iter()
        .filter(|r| r.validation_status == "✅")
        .count();
    let xml_serialization_success_total = results
        .iter()
        .filter(|r| r.xml_serialization_status == "✅")
        .count();
    let xml_roundtrip_success_total = results
        .iter()
        .filter(|r| r.xml_roundtrip_status == "✅")
        .count();

    println!("\n📊 Summary:");
    println!("   Total tests: {total_tests}");
    println!(
        "   Parse successful: {} ({}%)",
        parse_success_total,
        if total_tests > 0 {
            (parse_success_total * 100) / total_tests
        } else {
            0
        }
    );
    println!(
        "   Validation successful: {} ({}%)",
        validation_success_total,
        if total_tests > 0 {
            (validation_success_total * 100) / total_tests
        } else {
            0
        }
    );
    println!(
        "   XML Serialization successful: {} ({}%)",
        xml_serialization_success_total,
        if total_tests > 0 {
            (xml_serialization_success_total * 100) / total_tests
        } else {
            0
        }
    );
    println!(
        "   XML Roundtrip successful: {} ({}%)",
        xml_roundtrip_success_total,
        if total_tests > 0 {
            (xml_roundtrip_success_total * 100) / total_tests
        } else {
            0
        }
    );
}

fn get_status_symbol(success_count: usize, total_count: usize) -> &'static str {
    if total_count == 0 {
        "❔"
    } else if success_count == total_count {
        "✅"
    } else if success_count == 0 {
        "❌"
    } else {
        "⚠️"
    }
}

/// Parse JSON to MX structure, validate it, and serialize to XML
fn parse_and_validate_mx_message(
    json_value: &serde_json::Value,
    message_type: &str,
    debug_mode: bool,
    test_index: usize,
) -> Result<(serde_json::Value, String), ValidationOrSerializationError> {
    use mx_message::document::*;

    match message_type {
        "pacs.008" => {
            match serde_json::from_value::<pacs_008_001_08::FIToFICustomerCreditTransferV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        if debug_mode {
                            eprintln!("\n[Test {test_index}] MX structure validation failed:");
                            eprintln!("Errors: {:?}", errors);
                            eprintln!("MX structure that failed validation:");
                            eprintln!(
                                "{}",
                                serde_json::to_string_pretty(&mx_msg).unwrap_or_else(|_| {
                                    "Failed to serialize MX structure".to_string()
                                })
                            );
                        }
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse pacs.008: {:?}", e);
                        eprintln!("Generated JSON that failed to parse:");
                        eprintln!(
                            "{}",
                            serde_json::to_string_pretty(json_value)
                                .unwrap_or_else(|_| "Failed to serialize JSON".to_string())
                        );
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "pacs.009" => {
            match serde_json::from_value::<pacs_009_001_08::FinancialInstitutionCreditTransferV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse pacs.009: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "pacs.003" => {
            match serde_json::from_value::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        if debug_mode {
                            eprintln!("\n[Test {test_index}] MX structure validation failed:");
                            eprintln!("Errors: {:?}", errors);
                            eprintln!("MX structure that failed validation:");
                            eprintln!(
                                "{}",
                                serde_json::to_string_pretty(&mx_msg).unwrap_or_else(|_| {
                                    "Failed to serialize MX structure".to_string()
                                })
                            );
                        }
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse pacs.003: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "pacs.002" => {
            match serde_json::from_value::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse pacs.002: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "pain.001" => {
            match serde_json::from_value::<pain_001_001_09::CustomerCreditTransferInitiationV09>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        if debug_mode {
                            eprintln!("\n[Test {test_index}] MX structure validation failed:");
                            eprintln!("Errors: {:?}", errors);
                            eprintln!("MX structure that failed validation:");
                            eprintln!(
                                "{}",
                                serde_json::to_string_pretty(&mx_msg).unwrap_or_else(|_| {
                                    "Failed to serialize MX structure".to_string()
                                })
                            );
                        }
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse pain.001: {:?}", e);
                        eprintln!("Generated JSON that failed to parse:");
                        eprintln!(
                            "{}",
                            serde_json::to_string_pretty(json_value)
                                .unwrap_or_else(|_| "Failed to serialize JSON".to_string())
                        );
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "pain.008" => {
            match serde_json::from_value::<pain_008_001_08::CustomerDirectDebitInitiationV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse pain.008: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.025" => {
            match serde_json::from_value::<camt_025_001_08::ReceiptV08>(json_value.clone()) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.025: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        // camt.027 and camt.028 are not available in this MX library version
        "camt.029" => {
            match serde_json::from_value::<camt_029_001_09::ResolutionOfInvestigationV09>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.029: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.052" => {
            match serde_json::from_value::<camt_052_001_08::BankToCustomerAccountReportV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.052: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.053" => {
            match serde_json::from_value::<camt_053_001_08::BankToCustomerStatementV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.053: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.054" => {
            match serde_json::from_value::<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.054: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.056" => {
            match serde_json::from_value::<camt_056_001_08::FIToFIPaymentCancellationRequestV08>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.056: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.057" => {
            match serde_json::from_value::<camt_057_001_06::NotificationToReceiveV06>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.057: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        "camt.060" => {
            match serde_json::from_value::<camt_060_001_05::AccountReportingRequestV05>(
                json_value.clone(),
            ) {
                Ok(mx_msg) => {
                    if let Err(errors) = validate_message(&mx_msg) {
                        return Err(ValidationOrSerializationError::Validation(errors));
                    }

                    match xml_to_string(&mx_msg) {
                        Ok(xml) => {
                            let mx_json = serde_json::to_value(&mx_msg).unwrap();
                            Ok((mx_json, xml))
                        }
                        Err(e) => Err(ValidationOrSerializationError::Serialization(format!(
                            "XML serialization error: {:?}",
                            e
                        ))),
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\n[Test {test_index}] Failed to parse camt.060: {:?}", e);
                    }
                    Err(ValidationOrSerializationError::Validation(vec![format!(
                        "Parse error: {:?}",
                        e
                    )]))
                }
            }
        }
        _ => {
            // For other message types, add support as needed
            Err(ValidationOrSerializationError::Validation(vec![format!(
                "Message type {} not yet supported in round-trip test",
                message_type
            )]))
        }
    }
}

/// Parse XML back to JSON for comparison
fn parse_xml_back_to_json(
    xml_string: &str,
    message_type: &str,
    debug_mode: bool,
    test_index: usize,
) -> Result<serde_json::Value, String> {
    use mx_message::document::*;

    match message_type {
        "pacs.008" => {
            match xml_from_str::<pacs_008_001_08::FIToFICustomerCreditTransferV08>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for pacs.008: {:?}",
                            e
                        );
                        eprintln!("XML content: {}", &xml_string[..xml_string.len().min(500)]);
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "pacs.009" => {
            match xml_from_str::<pacs_009_001_08::FinancialInstitutionCreditTransferV08>(xml_string)
            {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for pacs.009: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "pacs.003" => {
            match xml_from_str::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for pacs.003: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "pacs.002" => {
            match xml_from_str::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for pacs.002: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "pain.001" => {
            match xml_from_str::<pain_001_001_09::CustomerCreditTransferInitiationV09>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for pain.001: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "pain.008" => {
            match xml_from_str::<pain_008_001_08::CustomerDirectDebitInitiationV08>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for pain.008: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "camt.025" => match xml_from_str::<camt_025_001_08::ReceiptV08>(xml_string) {
            Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
            Err(e) => {
                if debug_mode {
                    eprintln!(
                        "\n[Test {test_index}] Failed to parse XML for camt.025: {:?}",
                        e
                    );
                }
                Err(format!("XML parse error: {:?}", e))
            }
        },
        // camt.027 and camt.028 are not available in this MX library version
        "camt.029" => {
            match xml_from_str::<camt_029_001_09::ResolutionOfInvestigationV09>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for camt.029: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "camt.052" => {
            match xml_from_str::<camt_052_001_08::BankToCustomerAccountReportV08>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for camt.052: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "camt.053" => {
            match xml_from_str::<camt_053_001_08::BankToCustomerStatementV08>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for camt.053: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "camt.054" => {
            match xml_from_str::<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>(
                xml_string,
            ) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for camt.054: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "camt.056" => {
            match xml_from_str::<camt_056_001_08::FIToFIPaymentCancellationRequestV08>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for camt.056: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        "camt.057" => match xml_from_str::<camt_057_001_06::NotificationToReceiveV06>(xml_string) {
            Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
            Err(e) => {
                if debug_mode {
                    eprintln!(
                        "\n[Test {test_index}] Failed to parse XML for camt.057: {:?}",
                        e
                    );
                }
                Err(format!("XML parse error: {:?}", e))
            }
        },
        "camt.060" => {
            match xml_from_str::<camt_060_001_05::AccountReportingRequestV05>(xml_string) {
                Ok(mx_msg) => Ok(serde_json::to_value(&mx_msg).unwrap()),
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "\n[Test {test_index}] Failed to parse XML for camt.060: {:?}",
                            e
                        );
                    }
                    Err(format!("XML parse error: {:?}", e))
                }
            }
        }
        _ => Err(format!(
            "Message type {} not yet supported in round-trip test",
            message_type
        )),
    }
}
