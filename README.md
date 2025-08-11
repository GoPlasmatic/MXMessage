<div align="center">
  <img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">

# MXMessage

**A Rust library for parsing, validating, and serializing ISO 20022 (MX) financial messages.**

*Full CBPR+ compliance with comprehensive validation and test data generation.*

  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
  [![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
  [![Crates.io](https://img.shields.io/crates/v/mx-message.svg)](https://crates.io/crates/mx-message)
  [![Swift CBPR+](https://img.shields.io/badge/Swift-CBPR%2B%20SR2025-green.svg)](https://www.swift.com)

  <p>
    <a href="https://github.com/GoPlasmatic">🏢 Organization</a> •
    <a href="https://docs.rs/mx-message">📖 Documentation</a> •
    <a href="https://github.com/GoPlasmatic/MXMessage/issues">🐛 Issues</a>  
  </p>
</div>

-----

MXMessage is a comprehensive Rust library for handling ISO 20022 (MX) financial messages. It provides type-safe parsing, validation, and serialization for CBPR+ compliant messages, with powerful test data generation capabilities for all supported message types.

## 🚀 Key Features

  - **CBPR+ SR2025 Compliant:** Full support for Central Bank Payment Regulation Plus SR2025 schemas.
  - **Simplified v3 API:** Two main functions (`to_json` and `to_xml`) handle all message types with automatic envelope generation.
  - **Type-Safe:** Strongly typed Rust structures generated from official XSD schemas.
  - **Comprehensive Validation:** Field-level, pattern, and business rule validation with detailed error codes.
  - **Complete MX XML Generation:** Automatic ISO 20022 compliant XML with proper AppHdr envelope.
  - **Multiple Formats:** Native support for both JSON and XML serialization.
  - **Test Data Generation:** Automatic generation of valid test messages using configurable scenarios.
  - **Extensive Coverage:** Support for pacs, pain, and camt message families.
  - **Zero-Copy Parsing:** Efficient processing with minimal allocations.

## 🏗️ How It Works: Schema-Driven Architecture

### Generated Types from XSD

All message types are automatically generated from official ISO 20022 XSD schemas.

```rust
use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;

// Create a strongly-typed payment message
let payment = FIToFICustomerCreditTransferV08 {
    grp_hdr: GroupHeader93 {
        msg_id: "MSGID123456".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        nb_of_txs: "1".to_string(),
        // ... other fields
    },
    cdt_trf_tx_inf: vec![/* transactions */],
    splmtry_data: None,
};

// Automatic validation
match Document::FIToFICustomerCreditTransferV08(Box::new(payment)).validate() {
    Ok(_) => println!("✓ Valid CBPR+ compliant message"),
    Err(e) => println!("✗ Validation error: {} (code: {})", e.message, e.code),
}
```

### Message Families

```rust
// Payment messages
use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;  // Customer Credit Transfer
use mx_message::pacs_009_001_08::*;  // Financial Institution Credit Transfer

// Cash management messages  
use mx_message::camt_053_001_08::*;  // Bank to Customer Statement
use mx_message::camt_056_001_08::*;  // Payment Cancellation Request

// Payment initiation messages
use mx_message::pain_001_001_09::*;  // Customer Credit Transfer Initiation
use mx_message::pain_008_001_08::*;  // Customer Direct Debit Initiation
```

## 🎯 Serialization Support

MXMessage provides seamless serialization between JSON and XML formats.

```rust
use mx_message::document::Document;
use serde_json;
use quick_xml;

// Parse from JSON
let doc: Document = serde_json::from_str(json_str)?;

// Serialize to pretty JSON
let json_output = serde_json::to_string_pretty(&doc)?;

// Serialize to XML
let xml_output = quick_xml::se::to_string(&doc)?;
```

**Example JSON Output:**

```json
{
  "FIToFICstmrCdtTrf": {
    "GrpHdr": {
      "MsgId": "MSGID123456",
      "CreDtTm": "2024-01-15T10:30:00Z",
      "NbOfTxs": "1",
      "TtlIntrBkSttlmAmt": {
        "@Ccy": "EUR",
        "$value": 1000.00
      }
    },
    "CdtTrfTxInf": [{
      "PmtId": {
        "EndToEndId": "E2E123456",
        "UETR": "12345678-1234-5678-1234-567812345678"
      },
      "IntrBkSttlmAmt": {
        "@Ccy": "EUR", 
        "$value": 1000.00
      }
    }]
  }
}
```

## 🔧 Installation

Add `mx-message` to your `Cargo.toml`:

```toml
[dependencies]
mx-message = "3.0"
serde_json = "1.0"  # For JSON support
quick-xml = { version = "0.31", features = ["serialize"] }  # For XML support
```

## 📖 Usage

### Basic Message Creation (v3 API)

```rust
use mx_message::{to_json, to_xml};
use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;

// Create a payment message
let payment = FIToFICustomerCreditTransferV08 {
    grp_hdr: GroupHeader93 {
        msg_id: "MSGID123456".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        nb_of_txs: "1".to_string(),
        // ... other fields
    },
    cdt_trf_tx_inf: vec![/* transactions */],
    splmtry_data: None,
};

let document = Document::FIToFICustomerCreditTransferV08(Box::new(payment));

// Validate against CBPR+ SR2025 rules
match document.validate() {
    Ok(_) => println!("✓ Message is valid"),
    Err(e) => eprintln!("✗ Validation failed: {}", e.message),
}

// Convert to JSON (simplified v3 API)
let json = to_json(&document)?;
println!("{}", json);

// Convert to XML with automatic envelope (simplified v3 API)
let xml = to_xml(&document)?;
println!("{}", xml);
```

### Test Data Generation

```rust
use mx_message::sample::generate_sample;

// Generate test data from scenario files
let sample = generate_sample("pacs008", Some("cbpr_cross_border_payment"))?;
println!("Generated sample: {}", serde_json::to_string_pretty(&sample)?);

// Scenarios support fake data generation
// See test_scenarios/ directory for examples
```

### Complete MX XML Generation (v3 Simplified API)

Generate ISO 20022 compliant XML with proper envelope and Business Application Header using the simplified v3 API:

```rust
use mx_message::{to_json, to_xml};
use mx_message::document::Document;

// Create or load your message
let document = Document::FIToFICustomerCreditTransferV08(/* ... */);

// Convert to JSON - simple one-liner
let json = to_json(&document)?;
println!("{}", json);

// Convert to XML with automatic envelope generation
let xml = to_xml(&document)?;
println!("{}", xml);
```

The v3 API automatically:
- Generates the correct Business Application Header based on message type
- Creates a properly formatted MX envelope with AppHdr
- Sets appropriate namespaces and CBPR+ compliance indicators
- Handles all message types uniformly

**Note:** The simplified API uses sensible defaults. For advanced customization of headers and envelopes, the lower-level APIs are still available:

```rust
use mx_message::mx_envelope::create_mx_envelope;
use mx_message::header::head_001_001_02::BusinessApplicationHeaderV02;

// Create custom header for advanced use cases
let custom_header = BusinessApplicationHeaderV02 {
    fr: Party44Choice {
        fiid: Some(FinancialInstitutionIdentification18 {
            bicfi: Some("BANKUS33XXX".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    },
    to: Party44Choice {
        fiid: Some(FinancialInstitutionIdentification18 {
            bicfi: Some("BANKGB22XXX".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    },
    biz_msg_idr: "MSG-2024-001".to_string(),
    msg_def_idr: "pacs.008.001.08".to_string(),
    biz_svc: Some("swift.cbprplus.02".to_string()),
    cre_dt: "2024-01-15T10:30:00Z".to_string(),
    ..Default::default()
};

// Create envelope with custom header
let envelope = create_mx_envelope(custom_header, document)?;
let xml = quick_xml::se::to_string(&envelope)?;
```

### Validation Error Handling

```rust
use mx_message::document::Document;

match document.validate() {
    Ok(_) => println!("✓ Valid"),
    Err(e) => {
        match e.code {
            1001 => println!("Field too short: {}", e.message),
            1002 => println!("Field too long: {}", e.message),
            1005 => println!("Invalid pattern: {}", e.message),
            _ => println!("Validation error: {}", e.message),
        }
    }
}
```

### Error Collection (Comprehensive Validation)

MXMessage supports collecting all validation errors instead of stopping at the first error:

```rust
use mx_message::validation::Validate;
use mx_message::parse_result::{ParserConfig, ErrorCollector};
use mx_message::pacs_008_001_08::FIToFICustomerCreditTransferV08;

// Create a message with multiple validation issues
let payment = FIToFICustomerCreditTransferV08 {
    grp_hdr: GroupHeader93 {
        msg_id: "ID",  // Too short (min 5 chars)
        cre_dt_tm: "invalid-date",  // Wrong format
        nb_of_txs: "ABC",  // Should be numeric
        // ... other fields
    },
    // ... other fields
};

// Collect all validation errors
let config = ParserConfig::default();
let mut collector = ErrorCollector::new();
payment.validate("", &config, &mut collector);

// Process all errors at once
if collector.has_errors() {
    println!("Found {} validation errors:", collector.errors().len());
    for error in collector.errors() {
        println!("  - {}: {} (code: {})", 
            error.path.as_ref().unwrap_or(&"root".to_string()),
            error.message, 
            error.code
        );
    }
} else {
    println!("✓ Message is valid");
}
```

## 🧪 Testing Strategy

MXMessage uses comprehensive testing with 168 real-world scenarios migrated from MT messages, covering cross-border payments, securities, cash management, and more.

### Key Testing Features

- **Scenario-Based Testing**: 168 scenarios across 16 message types
- **Round-Trip Validation**: JSON → Generate → Validate → JSON testing
- **MT to MX Migration**: 99.4% coverage of MT message scenarios
- **Sample Generation**: Automatic test data using `datafake` library
- **100% Success Rate**: All 1,680 tests pass (10 samples per scenario)

### Quick Start

```bash
# Run all test scenarios
cargo test round_trip_scenarios

# Test specific message type
TEST_MESSAGE_TYPE=pacs.008 cargo test round_trip_scenarios

# Debug a specific scenario
TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=cbpr_cross_border_payment TEST_DEBUG=1 cargo test round_trip_scenarios -- --nocapture
```

For detailed test scenarios and MT to MX mapping, see the [Test Scenarios Documentation](test_scenarios/README.md).

## 🏛️ CBPR+ SR2025 Compliance

This library implements CBPR+ SR2025 (Central Bank Payment Regulation Plus Standards Release 2025) compliant schemas, providing:

- **Enhanced Validation**: Stricter rules for regulatory compliance
- **UETR Support**: Unique End-to-end Transaction Reference tracking
- **Central Bank Integration**: Support for central bank payment systems
- **Cross-Border Payments**: Full support for international transactions
- **Regulatory Reporting**: Compliance with reporting requirements
- **BizSvc SR2025**: Updated Business Service identifier `swift.cbprplus.02` for SR2025 compliance

## 📊 Supported Message Types

### Payment Messages (pacs)
- **pacs.002.001.10**: Payment Status Report
- **pacs.003.001.08**: Direct Debit
- **pacs.008.001.08**: Customer Credit Transfer
- **pacs.009.001.08**: Financial Institution Credit Transfer

### Payment Initiation (pain)
- **pain.001.001.09**: Customer Credit Transfer Initiation
- **pain.008.001.08**: Customer Direct Debit Initiation

### Cash Management (camt)
- **camt.025.001.05**: Receipt
- **camt.027.001.07**: Claim Non Receipt
- **camt.028.001.09**: Additional Payment Information
- **camt.029.001.09**: Resolution of Investigation
- **camt.052.001.08**: Bank to Customer Account Report
- **camt.053.001.08**: Bank to Customer Statement
- **camt.054.001.08**: Bank to Customer Debit/Credit Notification
- **camt.056.001.08**: Payment Cancellation Request
- **camt.057.001.06**: Notification to Receive
- **camt.060.001.05**: Account Reporting Request

## 🤝 Contributing

Contributions are welcome! If you'd like to help, please feel free to fork the repository, make your changes, and submit a pull request. We ask that you:

- Ensure all new message types follow CBPR+ compliance standards
- Add comprehensive tests for new functionality
- Update documentation for any new features
- Follow existing code style and validation patterns

## 🏢 About Plasmatic

MXMessage is developed by [Plasmatic](https://github.com/GoPlasmatic), an organization focused on building open-source tools for financial infrastructure. We believe in transparency, security, and performance.

Check out our other projects:

  - [SwiftMTMessage](https://github.com/GoPlasmatic/SwiftMTMessage): A SWIFT MT message parsing library.
  - [Reframe](https://github.com/GoPlasmatic/Reframe): A SWIFT MT to ISO 20022 (and back) transformation engine.

## 📄 License

This library is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

-----

<div align="center">
<p>Built with ❤️ by the <a href="https://github.com/GoPlasmatic">Plasmatic</a> team</p>
</div>