//! Example of using custom scenario paths for MX message sample generation

use mx_message::{SampleGenerator, ScenarioConfig, generate_sample, generate_sample_with_config};
use serde_json::Value;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MX Message Sample Generation with Custom Paths Example\n");

    // Method 1: Using environment variable
    // Set MX_SCENARIO_PATH=/path1:/path2:/path3 (Unix) or
    // Set MX_SCENARIO_PATH=C:\path1;C:\path2;C:\path3 (Windows)
    println!("1. Using default configuration (checks MX_SCENARIO_PATH env var):");
    match generate_sample::<Value>("pacs008", None) {
        Ok(msg) => {
            println!("   ✓ Generated pacs.008 using default paths");
            if let Some(grp_hdr) = msg.get("GrpHdr") {
                if let Some(msg_id) = grp_hdr.get("MsgId") {
                    println!("   Message ID: {msg_id}");
                }
            }
        }
        Err(e) => println!("   ✗ Failed: {e:?}"),
    }

    println!();

    // Method 2: Using ScenarioConfig with custom paths
    println!("2. Using custom configuration with specific paths:");
    let config = ScenarioConfig::with_paths(vec![
        PathBuf::from("test_scenarios"),
        PathBuf::from("../test_scenarios"),
        PathBuf::from("./custom_scenarios"), // Add custom path
    ]);

    match generate_sample_with_config::<Value>("pacs008", Some("standard"), &config) {
        Ok(msg) => {
            println!("   ✓ Generated pacs.008 with custom config");
            if let Some(grp_hdr) = msg.get("GrpHdr") {
                if let Some(msg_id) = grp_hdr.get("MsgId") {
                    println!("   Message ID: {msg_id}");
                }
            }
        }
        Err(e) => println!("   ✗ Failed: {e:?}"),
    }

    println!();

    // Method 3: Using SampleGenerator builder pattern
    println!("3. Using SampleGenerator with builder pattern:");
    let generator = SampleGenerator::new()
        .with_path(PathBuf::from("./my_scenarios"))
        .with_path(PathBuf::from("./backup_scenarios"))
        .with_path(PathBuf::from("test_scenarios")); // Add default as fallback

    match generator.generate::<Value>("pacs008", Some("standard")) {
        Ok(msg) => {
            println!("   ✓ Generated pacs.008 using SampleGenerator");
            if let Some(grp_hdr) = msg.get("GrpHdr") {
                if let Some(msg_id) = grp_hdr.get("MsgId") {
                    println!("   Message ID: {msg_id}");
                }
            }
        }
        Err(e) => println!("   ✗ Failed: {e:?}"),
    }

    println!();

    // Method 4: Demonstrate multiple scenario generation with same generator
    println!("4. Generating multiple scenarios with same generator:");
    let generator = SampleGenerator::new();

    let scenarios = vec!["standard", "cbpr_business_payment", "high_value"];
    for scenario in scenarios {
        match generator.generate::<Value>("pacs008", Some(scenario)) {
            Ok(msg) => {
                let msg_id = msg
                    .get("GrpHdr")
                    .and_then(|h| h.get("MsgId"))
                    .and_then(|id| id.as_str())
                    .unwrap_or("N/A");
                println!("   ✓ Generated pacs008/{scenario}: {msg_id}");
            }
            Err(e) => println!("   ✗ Failed to generate pacs008/{scenario}: {e:?}"),
        }
    }

    println!("\nDone!");
    Ok(())
}
