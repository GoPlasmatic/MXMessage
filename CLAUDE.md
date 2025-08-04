# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Build and Compile
```bash
# Build the library
cargo build

# Build with release optimizations
cargo build --release

# Check compilation without building
cargo check
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output displayed
cargo test -- --nocapture

# Run a specific test
cargo test test_json_serialization_roundtrip

# Run integration tests only
cargo test --test integration_tests
```

### Formatting and Linting
```bash
# Format code
cargo fmt

# Check formatting without applying changes
cargo fmt -- --check

# Run clippy for linting
cargo clippy
```

### Documentation
```bash
# Generate and open documentation
cargo doc --open

# Generate documentation without dependencies
cargo doc --no-deps
```

### Examples
```bash
# Run pacs.008 example
cargo run --example pacs008_example

# Run XML serialization example
cargo run --example xml_serialization

# Run sample generation example
cargo run --example generate_samples
```

### Sample Generation
```bash
# Generate sample messages using scenarios
cargo run --example generate_samples

# Create your own scenarios in test_scenarios/[message_type]/
# See test_scenarios/pacs008/ for examples
```

### Code Generation
```bash
# Regenerate all message types from XSD schemas
sh generate.sh

# Generate specific module types
sh generate-module.sh xsd/document/ src/document/
sh generate-module.sh xsd/header/ src/header/
```

## High-Level Architecture

### Core Structure
The MX Message library is a Rust implementation for parsing, validating, and serializing ISO20022 financial messages with CBPR+ (Central Bank Payment Regulation Plus) compliance.

### Key Components

1. **Document Module** (`src/document/`)
   - Contains auto-generated Rust structs for each ISO20022 message type
   - Each file corresponds to a specific message type (e.g., `pacs_008_001_08.rs` for payment messages)
   - Generated from XSD schemas in the `xsd/document/` directory

2. **Header Module** (`src/header/`)
   - Business Application Header (BAH) definitions for each message type
   - Follows the pattern `bah_[message_type].rs`
   - Generated from XSD schemas in the `xsd/header/` directory

3. **Error Handling** (`src/error.rs`)
   - Centralized error types and validation error codes
   - Implements comprehensive validation including field length, pattern matching, and business rules

### Code Generation Process
The library uses a code generation approach where:
1. XSD schemas define the message structure and validation rules
2. `generate-common.py` Python script parses XSD and generates Rust code
3. `generate.sh` orchestrates the generation process
4. Generated code includes full serde support for JSON/XML serialization

### Message Type Organization
- **pacs** (Payment Clearing and Settlement): Payment messages like credit transfers
- **camt** (Cash Management): Account statements, reports, and investigations
- **pain** (Payment Initiation): Customer payment instructions
- **admi** (Administration): System administration messages

### Validation Architecture
Each generated message type includes:
- Field-level validation (length, pattern, required fields)
- Type-safe enums for code values
- Business rule validation specific to CBPR+ compliance
- Comprehensive error reporting with specific error codes

### Key Design Patterns
1. **Type Safety**: Strong typing with generated structs ensures compile-time validation
2. **Serialization**: All types implement `serde::Serialize` and `serde::Deserialize`
3. **Box Usage**: Large nested structures use `Box<T>` to prevent stack overflow
4. **Option Types**: Optional fields are represented as `Option<T>`
5. **Validation Traits**: Common validation logic abstracted into reusable functions