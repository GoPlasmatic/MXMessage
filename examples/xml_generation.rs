// Complete example demonstrating ISO 20022 MX XML generation with envelope and header
// Uses automatic sample generation for various message types and scenarios

use mx_message::sample::generate_sample;

fn main() {
    println!("=== MX XML Generation with Sample Data ===\n");

    // Generate sample messages using different scenarios
    if let Err(e) = demonstrate_pacs008_scenarios() {
        eprintln!("Error in pacs008 scenarios: {e:?}");
    }

    if let Err(e) = demonstrate_different_message_types() {
        eprintln!("Error in different message types: {e:?}");
    }

    if let Err(e) = demonstrate_xml_configurations() {
        eprintln!("Error in XML configurations: {e:?}");
    }

    println!("\n=== Example Completed ===");
}

fn demonstrate_pacs008_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. PACS.008 with Different Scenarios:\n");

    // Scenario 1: Standard payment
    println!("a) Standard Payment Scenario:");
    let standard_xml = generate_sample("pacs008", Some("standard"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("Standard Payment XML (first 800 chars):");
    println!("{}\n", &standard_xml[..standard_xml.len().min(800)]);

    // Scenario 2: High value payment
    println!("b) High Value Payment Scenario:");
    let high_value_xml = generate_sample("pacs008", Some("high_value"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("High Value Payment XML (first 800 chars):");
    println!("{}\n", &high_value_xml[..high_value_xml.len().min(800)]);

    // Scenario 3: CBPR+ compliant payment
    println!("c) CBPR+ Business Payment:");
    let cbpr_xml = generate_sample("pacs008", Some("cbpr_business_payment"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("CBPR+ Payment XML (first 1000 chars):");
    println!("{}\n", &cbpr_xml[..cbpr_xml.len().min(1000)]);

    Ok(())
}

fn demonstrate_different_message_types() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. Different Message Types with Sample Data:\n");

    // PAIN.001 - Payment Initiation
    println!("a) PAIN.001 Payment Initiation:");
    let pain001_xml = generate_sample("pain001", Some("standard"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("PAIN.001 XML (first 600 chars):");
    println!("{}\n", &pain001_xml[..pain001_xml.len().min(600)]);

    // CAMT.053 - Bank Statement
    println!("b) CAMT.053 Bank Statement:");
    let camt053_xml = generate_sample("camt053", Some("daily_account_statement"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("CAMT.053 XML (first 600 chars):");
    println!("{}\n", &camt053_xml[..camt053_xml.len().min(600)]);

    // PACS.009 - Financial Institution Credit Transfer
    println!("c) PACS.009 FI Credit Transfer:");
    let pacs009_xml = generate_sample("pacs009", Some("standard"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("PACS.009 XML (first 600 chars):");
    println!("{}\n", &pacs009_xml[..pacs009_xml.len().min(600)]);

    // CAMT.052 - Account Report
    println!("d) CAMT.052 Account Report:");
    let camt052_xml = generate_sample("camt052", None)
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;

    println!("CAMT.052 XML (first 600 chars):");
    println!("{}\n", &camt052_xml[..camt052_xml.len().min(600)]);

    Ok(())
}

fn demonstrate_xml_configurations() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. Various Message Types and Scenarios:\n");

    // Demonstrate different message types with different scenarios
    println!("a) PAIN.008 Direct Debit:");
    let pain008_xml = generate_sample("pain008", Some("general_direct_debit_basic"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;
    println!("PAIN.008 XML (first 600 chars):");
    println!("{}\n", &pain008_xml[..pain008_xml.len().min(600)]);

    println!("b) CAMT.056 Cancellation Request:");
    let camt056_xml = generate_sample("camt056", Some("cbpr_cancellation_request"))
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;
    println!("CAMT.056 XML (first 600 chars):");
    println!("{}\n", &camt056_xml[..camt056_xml.len().min(600)]);

    println!("c) PACS.002 Payment Status Report:");
    let pacs002_xml = generate_sample("pacs002", None)
        .map_err(|e| format!("Failed to generate sample: {e:?}"))?;
    println!("PACS.002 XML (first 600 chars):");
    println!("{}\n", &pacs002_xml[..pacs002_xml.len().min(600)]);

    Ok(())
}
