# MX Message - ISO20022 Parser Library

A Rust library for parsing, validating, and serializing ISO20022 financial messages with support for CBPR+ (Central Bank Payment Regulation Plus) compliant schemas.

## Features

- **CBPR+ Compliance**: Full support for Central Bank Payment Regulation Plus based ISO20022 XSD schemas
- **Multiple Message Types**: Support for pacs.008, pacs.009, and camt message families
- **Validation**: Built-in validation for all message fields according to ISO20022 and CBPR+ specifications
- **Serialization**: Support for JSON and XML serialization/deserialization using serde
- **Type Safety**: Strongly typed Rust structures for all message components
- **Error Handling**: Comprehensive error reporting with specific validation codes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mx-message = "0.1"
serde_json = "1.0"  # For JSON serialization
quick-xml = { version = "0.31", features = ["serialize"] }  # For XML serialization
```

## Quick Start

### Creating a pacs.008 Message

```rust
use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;

fn create_payment_message() -> Document {
    // Create group header
    let group_header = GroupHeader93 {
        msg_id: "MSG123456789".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        btch_bookg: Some(false),
        nb_of_txs: "1".to_string(),
        ctrl_sum: Some(1000.00),
        ttl_intr_bk_sttlm_amt: Some(ActiveCurrencyAndAmount {
            ccy: "EUR".to_string(),
            value: 1000.00,
        }),
        intr_bk_sttlm_dt: Some("2024-01-15".to_string()),
        sttlm_inf: SettlementInstruction7 {
            sttlm_mtd: SettlementMethod1Code::CodeCLRG,
            sttlm_acct: None,
            clr_sys: Some(ClearingSystemIdentification3Choice {
                cd: Some("T2".to_string()),
                prtry: None,
            }),
            // ... other fields
        },
        pmt_tp_inf: None,
        instg_agt: None,
        instd_agt: None,
    };

    // Create credit transfer transaction
    let credit_transfer_tx = CreditTransferTransaction39 {
        pmt_id: PaymentIdentification7 {
            instr_id: Some("INSTR123".to_string()),
            end_to_end_id: "E2E123456789".to_string(),
            tx_id: Some("TXN123456789".to_string()),
            uetr: Some("12345678-1234-4567-8901-123456789012".to_string()),
            clr_sys_ref: None,
        },
        intr_bk_sttlm_amt: ActiveCurrencyAndAmount {
            ccy: "EUR".to_string(),
            value: 1000.00,
        },
        intr_bk_sttlm_dt: Some("2024-01-15".to_string()),
        sttlm_prty: Some(Priority3Code::CodeNORM),
        accptnc_dt_tm: None,
        poolg_adjstmnt_dt: None,
        chrg_br: ChargeBearerType1Code::CodeSHAR,
        dbtr: PartyIdentification135 {
            nm: Some("ACME Corporation".to_string()),
            ctry_of_res: Some("DE".to_string()),
            // ... other fields
        },
        cdtr: PartyIdentification135 {
            nm: Some("Global Suppliers Ltd".to_string()),
            ctry_of_res: Some("FR".to_string()),
            // ... other fields
        },
        // ... other required fields
    };

    // Create the complete message
    let fi_to_fi_msg = FIToFICustomerCreditTransferV08 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: vec![credit_transfer_tx],
        splmtry_data: None,
    };

    Document::FIToFICustomerCreditTransferV08(Box::new(fi_to_fi_msg))
}
```

### Validation

```rust
use mx_message::document::Document;

fn validate_message(document: &Document) -> Result<(), String> {
    match document.validate() {
        Ok(()) => {
            println!("✓ Message is valid and CBPR+ compliant");
            Ok(())
        }
        Err(e) => {
            println!("✗ Validation failed: {} (code: {})", e.message, e.code);
            Err(e.message)
        }
    }
}
```

### JSON Serialization

```rust
use serde_json;

fn serialize_to_json(document: &Document) -> Result<String, serde_json::Error> {
    // Pretty-printed JSON
    serde_json::to_string_pretty(document)
}

fn deserialize_from_json(json_str: &str) -> Result<Document, serde_json::Error> {
    serde_json::from_str(json_str)
}
```

### XML Serialization

```rust
use quick_xml::se::to_string as xml_to_string;
use quick_xml::de::from_str as xml_from_str;

fn serialize_to_xml(document: &Document) -> Result<String, quick_xml::Error> {
    xml_to_string(document)
}

fn deserialize_from_xml(xml_str: &str) -> Result<Document, quick_xml::Error> {
    xml_from_str(xml_str)
}
```

## Examples

Run the included examples to see the library in action:

```bash
# Basic pacs.008 message creation and validation
cargo run --example pacs008_example

# XML serialization demonstration
cargo run --example xml_serialization
```

## CBPR+ Compliance

This library implements CBPR+ (Central Bank Payment Regulation Plus) compliant schemas, which provide enhanced payment processing capabilities including:

- **Enhanced Validation**: Stricter validation rules for regulatory compliance
- **Extended Message Support**: Additional message types for comprehensive payment processing
- **Improved Error Handling**: Detailed error reporting for regulatory requirements
- **Central Bank Integration**: Support for central bank payment system requirements

## Validation

The library provides comprehensive validation including:

- **Field Length Validation**: Ensures all fields meet minimum and maximum length requirements
- **Pattern Validation**: Validates formats like IBAN, BIC codes, and numeric patterns
- **Required Field Validation**: Ensures all mandatory fields are present
- **Business Rule Validation**: Implements ISO20022 and CBPR+ business rules
- **Regulatory Compliance**: Validates against CBPR+ specific requirements

### Validation Error Codes

- `1001`: Field is shorter than minimum length
- `1002`: Field exceeds maximum length  
- `1005`: Field does not match required pattern
- `9999`: Unknown document type

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_json_serialization_roundtrip
```

## Supported Message Types

### Currently Supported (CBPR+ Compliant)

**Payment Messages (pacs)**
- **pacs.008.001.08**: FI to FI Customer Credit Transfer
- **pacs.009.001.08**: FI to FI Customer Direct Debit

**Cash Management Messages (camt)**
- **camt.027.001.07**: Claim Non Receipt
- **camt.028.001.09**: Additional Payment Information
- **camt.029.001.09**: Resolution of Investigation
- **camt.052.001.08**: Bank to Customer Account Report
- **camt.053.001.08**: Bank to Customer Statement
- **camt.056.001.08**: FI to FI Payment Cancellation Request
- **camt.057.001.06**: Notification to Receive
- **camt.998.001.03**: Cash Management Proprietary Message

### Planned Support
- pacs.002: FI to FI Payment Status Report
- pacs.004: Payment Return
- pain.001: Customer Credit Transfer Initiation
- pain.002: Customer Payment Status Report
- Additional CAMT message types

## Architecture

The library is structured around CBPR+ compliant schemas:

```
src/
├── lib.rs                 # Module declarations
├── document.rs            # Main document types and validation
├── common.rs              # Shared types and validation errors
├── pacs_008_001_08.rs     # FI to FI Customer Credit Transfer
├── pacs_009_001_08.rs     # FI to FI Customer Direct Debit
├── camt_027_001_07.rs     # Claim Non Receipt
├── camt_028_001_09.rs     # Additional Payment Information
├── camt_029_001_09.rs     # Resolution of Investigation
├── camt_052_001_08.rs     # Bank to Customer Account Report
├── camt_053_001_08.rs     # Bank to Customer Statement
├── camt_056_001_08.rs     # Payment Cancellation Request
├── camt_057_001_06.rs     # Notification to Receive
└── camt_998_001_03.rs     # Cash Management Proprietary Message
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. When contributing:

1. Ensure all new message types follow CBPR+ compliance standards
2. Add comprehensive tests for new functionality
3. Update documentation for any new features
4. Follow existing code style and validation patterns

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

- This library implements the ISO20022 standard as defined by the International Organization for Standardization (ISO)
- CBPR+ compliance ensures compatibility with Central Bank Payment Regulation Plus requirements
- Thanks to the Rust community for excellent serialization and XML processing libraries
