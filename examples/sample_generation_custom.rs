//! Example of using custom scenario paths for MX XML message sample generation

use mx_message::sample::{SampleGenerator, generate_sample, generate_sample_with_config};
use mx_message::scenario_config::ScenarioConfig;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MX Message Sample Generation with Custom Paths Example\n");

    // Method 1: Using environment variable
    // Set MX_SCENARIO_PATH=/path1:/path2:/path3 (Unix) or
    // Set MX_SCENARIO_PATH=C:\path1;C:\path2;C:\path3 (Windows)
    println!("1. Using default configuration (checks MX_SCENARIO_PATH env var):");
    match generate_sample("pacs008", None) {
        Ok(xml) => {
            println!("   ✓ Generated pacs.008 XML using default paths");
            // Extract message ID from XML
            if let Some(start) = xml.find("<BizMsgIdr>") {
                if let Some(end) = xml[start..].find("</BizMsgIdr>") {
                    let msg_id = &xml[start + 11..start + end];
                    println!("   Business Message ID: {msg_id}");
                }
            }
            println!("   XML length: {} bytes", xml.len());
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

    match generate_sample_with_config("pacs008", Some("standard"), &config) {
        Ok(xml) => {
            println!("   ✓ Generated pacs.008 XML with custom config");
            // Extract message ID from XML
            if let Some(start) = xml.find("<BizMsgIdr>") {
                if let Some(end) = xml[start..].find("</BizMsgIdr>") {
                    let msg_id = &xml[start + 11..start + end];
                    println!("   Business Message ID: {msg_id}");
                }
            }
            println!("   XML length: {} bytes", xml.len());
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

    match generator.generate("pacs008", Some("standard")) {
        Ok(xml) => {
            println!("   ✓ Generated pacs.008 XML using SampleGenerator");
            // Extract message ID from XML
            if let Some(start) = xml.find("<BizMsgIdr>") {
                if let Some(end) = xml[start..].find("</BizMsgIdr>") {
                    let msg_id = &xml[start + 11..start + end];
                    println!("   Business Message ID: {msg_id}");
                }
            }
            println!("   XML length: {} bytes", xml.len());
        }
        Err(e) => println!("   ✗ Failed: {e:?}"),
    }

    println!();

    // Method 4: Demonstrate multiple scenario generation with same generator
    println!("4. Generating multiple scenarios with same generator:");
    let generator = SampleGenerator::new();

    let scenarios = vec!["standard", "cbpr_business_payment", "high_value"];
    for scenario in scenarios {
        match generator.generate("pacs008", Some(scenario)) {
            Ok(xml) => {
                // Extract message ID from XML
                let msg_id = if let Some(start) = xml.find("<BizMsgIdr>") {
                    if let Some(end) = xml[start..].find("</BizMsgIdr>") {
                        &xml[start + 11..start + end]
                    } else {
                        "N/A"
                    }
                } else {
                    "N/A"
                };
                println!("   ✓ Generated pacs008/{scenario}: {msg_id}");
            }
            Err(e) => println!("   ✗ Failed to generate pacs008/{scenario}: {e:?}"),
        }
    }

    println!("\nDone!");
    Ok(())
}
