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
// Generate JSON Schema files and manifest for all supported MX message types.
//
// Usage:
//   cargo run --example generate_manifest --features jsonschema -- <version>
//
// Example:
//   cargo run --example generate_manifest --features jsonschema -- 3.1.4

use mx_message::document;
use mx_message::message_registry::{get_message_description, get_message_title};
use schemars::schema_for;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct Manifest {
    name: String,
    version: String,
    description: String,
    repository: String,
    license: String,
    authors: Vec<String>,
    supported_messages: Vec<MessageInfo>,
}

#[derive(Serialize)]
struct MessageInfo {
    #[serde(rename = "type")]
    message_type: String,
    title: String,
    description: String,
    category: String,
    schema_url: String,
}

fn map_category(msg_type: &str) -> &'static str {
    if msg_type.starts_with("pacs") {
        "Payment Clearing & Settlement"
    } else if msg_type.starts_with("camt") {
        "Cash Management"
    } else if msg_type.starts_with("pain") {
        "Payment Initiation"
    } else if msg_type.starts_with("admi") {
        "Administration"
    } else {
        "Other"
    }
}

macro_rules! generate_schema {
    ($output_dir:expr, $version:expr, $module:ident, $type:ident, $full_form:expr, $messages:expr) => {{
        let schema = schema_for!(document::$module::$type);
        let json = serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");
        let path = $output_dir.join(format!("{}.schema.json", $full_form));
        fs::write(&path, json).expect("Failed to write schema file");
        println!("  Generated: {}", path.display());

        $messages.push(MessageInfo {
            message_type: $full_form.to_string(),
            title: get_message_title($full_form)
                .unwrap_or("Unknown Message Type")
                .to_string(),
            description: get_message_description($full_form)
                .unwrap_or("Unknown message type description.")
                .to_string(),
            category: map_category($full_form).to_string(),
            schema_url: format!(
                "https://github.com/GoPlasmatic/MXMessage/releases/download/v{}/{}.schema.json",
                $version, $full_form
            ),
        });
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: cargo run --example generate_manifest --features jsonschema -- <version>"
        );
        eprintln!("Example: cargo run --example generate_manifest --features jsonschema -- 3.1.4");
        std::process::exit(1);
    }

    let version = &args[1];
    let output_dir = Path::new("schemas");

    // Create output directory
    fs::create_dir_all(output_dir).expect("Failed to create schemas directory");

    println!("Generating JSON schemas for MX Message types...\n");

    let mut messages: Vec<MessageInfo> = Vec::new();

    // Generate schemas for all 25 message types
    // PACS - Payment Clearing & Settlement
    generate_schema!(
        output_dir,
        version,
        pacs_002_001_10,
        FIToFIPaymentStatusReportV10,
        "pacs.002.001.10",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pacs_003_001_08,
        FIToFICustomerDirectDebitV08,
        "pacs.003.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pacs_004_001_09,
        PaymentReturnV09,
        "pacs.004.001.09",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pacs_008_001_08,
        FIToFICustomerCreditTransferV08,
        "pacs.008.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pacs_009_001_08,
        FinancialInstitutionCreditTransferV08,
        "pacs.009.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pacs_010_001_03,
        FinancialInstitutionDirectDebitV03,
        "pacs.010.001.03",
        messages
    );

    // PAIN - Payment Initiation
    generate_schema!(
        output_dir,
        version,
        pain_001_001_09,
        CustomerCreditTransferInitiationV09,
        "pain.001.001.09",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pain_002_001_10,
        CustomerPaymentStatusReportV10,
        "pain.002.001.10",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        pain_008_001_08,
        CustomerDirectDebitInitiationV08,
        "pain.008.001.08",
        messages
    );

    // CAMT - Cash Management
    generate_schema!(
        output_dir,
        version,
        camt_025_001_08,
        ReceiptV08,
        "camt.025.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_029_001_09,
        ResolutionOfInvestigationV09,
        "camt.029.001.09",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_052_001_08,
        BankToCustomerAccountReportV08,
        "camt.052.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_053_001_08,
        BankToCustomerStatementV08,
        "camt.053.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_054_001_08,
        BankToCustomerDebitCreditNotificationV08,
        "camt.054.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_055_001_08,
        CustomerPaymentCancellationRequestV08,
        "camt.055.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_056_001_08,
        FIToFIPaymentCancellationRequestV08,
        "camt.056.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_057_001_06,
        NotificationToReceiveV06,
        "camt.057.001.06",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_058_001_08,
        NotificationToReceiveCancellationAdviceV08,
        "camt.058.001.08",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_060_001_05,
        AccountReportingRequestV05,
        "camt.060.001.05",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_105_001_02,
        ChargesPaymentNotificationV02,
        "camt.105.001.02",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_106_001_02,
        ChargesPaymentRequestV02,
        "camt.106.001.02",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_107_001_01,
        ChequePresentmentNotificationV01,
        "camt.107.001.01",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_108_001_01,
        ChequeCancellationOrStopRequestV01,
        "camt.108.001.01",
        messages
    );
    generate_schema!(
        output_dir,
        version,
        camt_109_001_01,
        ChequeCancellationOrStopReportV01,
        "camt.109.001.01",
        messages
    );

    // ADMI - Administration
    generate_schema!(
        output_dir,
        version,
        admi_024_001_01,
        NotificationOfCorrespondenceV01,
        "admi.024.001.01",
        messages
    );

    // Create manifest
    let manifest = Manifest {
        name: "mx-message".to_string(),
        version: version.clone(),
        description:
            "A fast, type-safe Rust implementation of MXMessage for parsing ISO20022 MX messages."
                .to_string(),
        repository: "https://github.com/GoPlasmatic/MXMessage".to_string(),
        license: "Apache-2.0".to_string(),
        authors: vec!["Plasmatic Engineering <shankar@goplasmatic.io>".to_string()],
        supported_messages: messages,
    };

    let manifest_json =
        serde_json::to_string_pretty(&manifest).expect("Failed to serialize manifest");
    let manifest_path = output_dir.join("manifest.json");
    fs::write(&manifest_path, manifest_json).expect("Failed to write manifest file");
    println!("\n  Generated: {}", manifest_path.display());

    println!("\nSchema generation complete!");
    println!("Generated {} schema files + manifest.json", 25);
}
