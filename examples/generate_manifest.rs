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
use mx_message::header;
use mx_message::message_registry::{get_message_description, get_message_title};
use schemars::schema_for;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;

// Macro to define a document wrapper struct with the correct XML element name
macro_rules! define_document_wrapper {
    ($name:ident, $doc_type:ty, $xml_name:expr) => {
        #[derive(Serialize, schemars::JsonSchema)]
        pub struct $name {
            #[serde(rename = $xml_name)]
            pub inner: $doc_type,
        }
    };
}

// Macro to define an envelope struct with AppHdr and Document
macro_rules! define_envelope {
    ($name:ident, $doc_wrapper:ty) => {
        #[derive(Serialize, schemars::JsonSchema)]
        pub struct $name {
            #[serde(rename = "AppHdr")]
            pub app_hdr: header::AppHdr,
            #[serde(rename = "Document")]
            pub document: $doc_wrapper,
        }
    };
}

// PACS - Payment Clearing & Settlement
define_document_wrapper!(Pacs002Document, document::pacs_002_001_10::FIToFIPaymentStatusReportV10, "FIToFIPmtStsRpt");
define_envelope!(Pacs002Envelope, Pacs002Document);

define_document_wrapper!(Pacs003Document, document::pacs_003_001_08::FIToFICustomerDirectDebitV08, "FIToFICstmrDrctDbt");
define_envelope!(Pacs003Envelope, Pacs003Document);

define_document_wrapper!(Pacs004Document, document::pacs_004_001_09::PaymentReturnV09, "PmtRtr");
define_envelope!(Pacs004Envelope, Pacs004Document);

define_document_wrapper!(Pacs008Document, document::pacs_008_001_08::FIToFICustomerCreditTransferV08, "FIToFICstmrCdtTrf");
define_envelope!(Pacs008Envelope, Pacs008Document);

define_document_wrapper!(Pacs009Document, document::pacs_009_001_08::FinancialInstitutionCreditTransferV08, "FICdtTrf");
define_envelope!(Pacs009Envelope, Pacs009Document);

define_document_wrapper!(Pacs010Document, document::pacs_010_001_03::FinancialInstitutionDirectDebitV03, "FIDrctDbt");
define_envelope!(Pacs010Envelope, Pacs010Document);

// PAIN - Payment Initiation
define_document_wrapper!(Pain001Document, document::pain_001_001_09::CustomerCreditTransferInitiationV09, "CstmrCdtTrfInitn");
define_envelope!(Pain001Envelope, Pain001Document);

define_document_wrapper!(Pain002Document, document::pain_002_001_10::CustomerPaymentStatusReportV10, "CstmrPmtStsRpt");
define_envelope!(Pain002Envelope, Pain002Document);

define_document_wrapper!(Pain008Document, document::pain_008_001_08::CustomerDirectDebitInitiationV08, "CstmrDrctDbtInitn");
define_envelope!(Pain008Envelope, Pain008Document);

// CAMT - Cash Management
define_document_wrapper!(Camt025Document, document::camt_025_001_08::ReceiptV08, "Rcpt");
define_envelope!(Camt025Envelope, Camt025Document);

define_document_wrapper!(Camt029Document, document::camt_029_001_09::ResolutionOfInvestigationV09, "RsltnOfInvstgtn");
define_envelope!(Camt029Envelope, Camt029Document);

define_document_wrapper!(Camt052Document, document::camt_052_001_08::BankToCustomerAccountReportV08, "BkToCstmrAcctRpt");
define_envelope!(Camt052Envelope, Camt052Document);

define_document_wrapper!(Camt053Document, document::camt_053_001_08::BankToCustomerStatementV08, "BkToCstmrStmt");
define_envelope!(Camt053Envelope, Camt053Document);

define_document_wrapper!(Camt054Document, document::camt_054_001_08::BankToCustomerDebitCreditNotificationV08, "BkToCstmrDbtCdtNtfctn");
define_envelope!(Camt054Envelope, Camt054Document);

define_document_wrapper!(Camt055Document, document::camt_055_001_08::CustomerPaymentCancellationRequestV08, "CstmrPmtCxlReq");
define_envelope!(Camt055Envelope, Camt055Document);

define_document_wrapper!(Camt056Document, document::camt_056_001_08::FIToFIPaymentCancellationRequestV08, "FIToFIPmtCxlReq");
define_envelope!(Camt056Envelope, Camt056Document);

define_document_wrapper!(Camt057Document, document::camt_057_001_06::NotificationToReceiveV06, "NtfctnToRcv");
define_envelope!(Camt057Envelope, Camt057Document);

define_document_wrapper!(Camt058Document, document::camt_058_001_08::NotificationToReceiveCancellationAdviceV08, "NtfctnToRcvCxlAdvc");
define_envelope!(Camt058Envelope, Camt058Document);

define_document_wrapper!(Camt060Document, document::camt_060_001_05::AccountReportingRequestV05, "AcctRptgReq");
define_envelope!(Camt060Envelope, Camt060Document);

define_document_wrapper!(Camt105Document, document::camt_105_001_02::ChargesPaymentNotificationV02, "ChrgsPmtNtfctn");
define_envelope!(Camt105Envelope, Camt105Document);

define_document_wrapper!(Camt106Document, document::camt_106_001_02::ChargesPaymentRequestV02, "ChrgsPmtReq");
define_envelope!(Camt106Envelope, Camt106Document);

define_document_wrapper!(Camt107Document, document::camt_107_001_01::ChequePresentmentNotificationV01, "ChqPresntmntNtfctn");
define_envelope!(Camt107Envelope, Camt107Document);

define_document_wrapper!(Camt108Document, document::camt_108_001_01::ChequeCancellationOrStopRequestV01, "ChqCxlOrStopReq");
define_envelope!(Camt108Envelope, Camt108Document);

define_document_wrapper!(Camt109Document, document::camt_109_001_01::ChequeCancellationOrStopReportV01, "ChqCxlOrStopRpt");
define_envelope!(Camt109Envelope, Camt109Document);

// ADMI - Administration
define_document_wrapper!(Admi024Document, document::admi_024_001_01::NotificationOfCorrespondenceV01, "NtfctnOfCrspdc");
define_envelope!(Admi024Envelope, Admi024Document);

// Message Variants (STP, ADV, COV, MC)
// PACS Variants
define_document_wrapper!(Pacs008StpDocument, document::pacs_008_001_08_stp::FIToFICustomerCreditTransferV08, "FIToFICstmrCdtTrf");
define_envelope!(Pacs008StpEnvelope, Pacs008StpDocument);

define_document_wrapper!(Pacs009AdvDocument, document::pacs_009_001_08_adv::FinancialInstitutionCreditTransferV08, "FICdtTrf");
define_envelope!(Pacs009AdvEnvelope, Pacs009AdvDocument);

define_document_wrapper!(Pacs009CovDocument, document::pacs_009_001_08_cov::FinancialInstitutionCreditTransferV08, "FICdtTrf");
define_envelope!(Pacs009CovEnvelope, Pacs009CovDocument);

define_document_wrapper!(Pacs010McDocument, document::pacs_010_001_03_mc::FinancialInstitutionDirectDebitV03, "FIDrctDbt");
define_envelope!(Pacs010McEnvelope, Pacs010McDocument);

// CAMT Variants
define_document_wrapper!(Camt105McDocument, document::camt_105_001_02_mc::ChargesPaymentNotificationV02, "ChrgsPmtNtfctn");
define_envelope!(Camt105McEnvelope, Camt105McDocument);

define_document_wrapper!(Camt106McDocument, document::camt_106_001_02_mc::ChargesPaymentRequestV02, "ChrgsPmtReq");
define_envelope!(Camt106McEnvelope, Camt106McDocument);

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
    ($output_dir:expr, $version:expr, $envelope:ty, $full_form:expr, $messages:expr) => {{
        let schema = schema_for!($envelope);
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

    // Generate schemas for all 31 message types (25 base + 6 variants)
    // PACS - Payment Clearing & Settlement
    generate_schema!(output_dir, version, Pacs002Envelope, "pacs.002.001.10", messages);
    generate_schema!(output_dir, version, Pacs003Envelope, "pacs.003.001.08", messages);
    generate_schema!(output_dir, version, Pacs004Envelope, "pacs.004.001.09", messages);
    generate_schema!(output_dir, version, Pacs008Envelope, "pacs.008.001.08", messages);
    generate_schema!(output_dir, version, Pacs009Envelope, "pacs.009.001.08", messages);
    generate_schema!(output_dir, version, Pacs010Envelope, "pacs.010.001.03", messages);

    // PAIN - Payment Initiation
    generate_schema!(output_dir, version, Pain001Envelope, "pain.001.001.09", messages);
    generate_schema!(output_dir, version, Pain002Envelope, "pain.002.001.10", messages);
    generate_schema!(output_dir, version, Pain008Envelope, "pain.008.001.08", messages);

    // CAMT - Cash Management
    generate_schema!(output_dir, version, Camt025Envelope, "camt.025.001.08", messages);
    generate_schema!(output_dir, version, Camt029Envelope, "camt.029.001.09", messages);
    generate_schema!(output_dir, version, Camt052Envelope, "camt.052.001.08", messages);
    generate_schema!(output_dir, version, Camt053Envelope, "camt.053.001.08", messages);
    generate_schema!(output_dir, version, Camt054Envelope, "camt.054.001.08", messages);
    generate_schema!(output_dir, version, Camt055Envelope, "camt.055.001.08", messages);
    generate_schema!(output_dir, version, Camt056Envelope, "camt.056.001.08", messages);
    generate_schema!(output_dir, version, Camt057Envelope, "camt.057.001.06", messages);
    generate_schema!(output_dir, version, Camt058Envelope, "camt.058.001.08", messages);
    generate_schema!(output_dir, version, Camt060Envelope, "camt.060.001.05", messages);
    generate_schema!(output_dir, version, Camt105Envelope, "camt.105.001.02", messages);
    generate_schema!(output_dir, version, Camt106Envelope, "camt.106.001.02", messages);
    generate_schema!(output_dir, version, Camt107Envelope, "camt.107.001.01", messages);
    generate_schema!(output_dir, version, Camt108Envelope, "camt.108.001.01", messages);
    generate_schema!(output_dir, version, Camt109Envelope, "camt.109.001.01", messages);

    // ADMI - Administration
    generate_schema!(output_dir, version, Admi024Envelope, "admi.024.001.01", messages);

    // Message Variants (STP, ADV, COV, MC)
    generate_schema!(output_dir, version, Pacs008StpEnvelope, "pacs.008.001.08.stp", messages);
    generate_schema!(output_dir, version, Pacs009AdvEnvelope, "pacs.009.001.08.adv", messages);
    generate_schema!(output_dir, version, Pacs009CovEnvelope, "pacs.009.001.08.cov", messages);
    generate_schema!(output_dir, version, Pacs010McEnvelope, "pacs.010.001.03.mc", messages);
    generate_schema!(output_dir, version, Camt105McEnvelope, "camt.105.001.02.mc", messages);
    generate_schema!(output_dir, version, Camt106McEnvelope, "camt.106.001.02.mc", messages);

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
    println!("Generated {} schema files + manifest.json", 31);
}
