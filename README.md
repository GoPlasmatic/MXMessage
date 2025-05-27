# MX Message - ISO20022 Parser Library

A Rust library for parsing, validating, and serializing ISO20022 financial messages, with initial support for pacs.008 (FI to FI Customer Credit Transfer) messages.

## Features

- **Full pacs.008.001.13 Support**: Complete implementation of the FI to FI Customer Credit Transfer message format
- **Validation**: Built-in validation for all message fields according to ISO20022 specifications
- **Serialization**: Support for JSON and XML serialization/deserialization using serde
- **Type Safety**: Strongly typed Rust structures for all message components
- **Error Handling**: Comprehensive error reporting with specific validation codes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mx-message = "0.1.0"
serde_json = "1.0"  # For JSON serialization
quick-xml = { version = "0.31", features = ["serialize"] }  # For XML serialization
```

## Quick Start

### Creating a pacs.008 Message

```rust
use mx_message::document::Document;
use mx_message::pacs_008_001_13::*;

fn create_payment_message() -> Document {
    // Create group header
    let group_header = GroupHeader131 {
        msg_id: "MSG123456789".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        nb_of_txs: "1".to_string(),
        sttlm_inf: SettlementInstruction15 {
            sttlm_mtd: SettlementMethod1Code::CodeCLRG,
            // ... other fields
        },
        // ... other fields
    };

    // Create credit transfer transaction
    let credit_transfer_tx = CreditTransferTransaction70 {
        pmt_id: PaymentIdentification13 {
            end_to_end_id: "E2E123456789".to_string(),
            // ... other fields
        },
        intr_bk_sttlm_amt: ActiveCurrencyAndAmount {
            ccy: "EUR".to_string(),
            value: 1000.00,
        },
        chrg_br: ChargeBearerType1Code::CodeSHAR,
        dbtr: PartyIdentification272 {
            nm: Some("ACME Corporation".to_string()),
            // ... other fields
        },
        cdtr: PartyIdentification272 {
            nm: Some("Global Suppliers Ltd".to_string()),
            // ... other fields
        },
        // ... other required fields
    };

    // Create the complete message
    let fi_to_fi_msg = FIToFICustomerCreditTransferV13 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: vec![credit_transfer_tx],
        splmtry_data: None,
    };

    Document::FIToFICustomerCreditTransferV13(Box::new(fi_to_fi_msg))
}
```

### Validation

```rust
use mx_message::document::Document;

fn validate_message(document: &Document) -> Result<(), String> {
    match document.validate() {
        Ok(()) => {
            println!("✓ Message is valid");
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

## Validation

The library provides comprehensive validation including:

- **Field Length Validation**: Ensures all fields meet minimum and maximum length requirements
- **Pattern Validation**: Validates formats like IBAN, BIC codes, and numeric patterns
- **Required Field Validation**: Ensures all mandatory fields are present
- **Business Rule Validation**: Implements ISO20022 business rules

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

Currently supported:
- **pacs.008.001.13**: FI to FI Customer Credit Transfer

Planned support:
- pacs.002: FI to FI Payment Status Report
- pacs.004: Payment Return
- pain.001: Customer Credit Transfer Initiation
- pain.002: Customer Payment Status Report

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

This library implements the ISO20022 standard as defined by the International Organization for Standardization (ISO).
