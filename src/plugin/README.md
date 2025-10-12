# MX Message Dataflow Plugins

This directory contains dataflow-rs plugin implementations for ISO20022 MX message processing. These plugins integrate with the [dataflow-rs](https://github.com/Plasmatic/dataflow-rs) workflow engine to provide a complete message processing pipeline.

## Overview

The MX plugin system provides four core operations for ISO20022 message handling:

1. **Generate** - Create sample MX messages from datafake schemas
2. **Publish** - Convert JSON to ISO20022 XML format
3. **Validate** - Validate XML messages against schemas and business rules
4. **Parse** - Convert XML back to structured JSON

## Parameter Naming Convention

All plugins follow a consistent **source/target** naming pattern:

- **`source`** - The field name containing input data (where data comes from)
- **`target`** - The field name where output will be stored (where data goes to)

This convention:
- ✅ Provides clear semantic meaning
- ✅ Avoids confusion with dataflow-rs `input` configuration object
- ✅ Makes data flow direction explicit in workflows
- ✅ Maintains consistency across all plugins

## Plugin Reference

### 1. Generate Plugin (`generate_mx`)

Generates sample ISO20022 MX messages using datafake scenarios.

**Parameters:**
- `target` (required): Field name where generated JSON data will be stored

**Input:**
- Reads datafake scenario from message payload

**Output:**
- JSON object containing:
  - `json_data`: Complete MX message structure
  - `message_type`: Detected message type (e.g., "pacs.008")

**Example:**
```json
{
  "function": {
    "name": "generate_mx",
    "input": {
      "target": "sample_json"
    }
  }
}
```

**Use Cases:**
- Test data generation
- Schema validation testing
- Sample message creation for documentation

---

### 2. Publish Plugin (`publish_mx`)

Converts JSON MX message data to ISO20022 XML format.

**Parameters:**
- `source` (required): Field name containing JSON message data
- `target` (required): Field name where XML output will be stored

**Input:**
- JSON object with MX message structure (with or without AppHdr envelope)

**Output:**
- ISO20022 compliant XML string

**Example:**
```json
{
  "function": {
    "name": "publish_mx",
    "input": {
      "source": "sample_json",
      "target": "sample_xml"
    }
  }
}
```

**Features:**
- Auto-detects message type from JSON structure
- Supports both Document-only and full envelope (AppHdr + Document) formats
- Generates namespace-compliant XML
- Validates structure during serialization

---

### 3. Validate Plugin (`validate_mx`)

Validates ISO20022 XML messages against schemas and business rules.

**Parameters:**
- `source` (required): Field name containing XML message to validate
- `target` (required): Field name where validation results will be stored

**Input:**
- ISO20022 XML string (with or without AppHdr envelope)

**Output:**
- JSON object with validation results:
  ```json
  {
    "valid": true/false,
    "errors": ["error message 1", "error message 2", ...],
    "warnings": [],
    "timestamp": "2025-10-12T10:30:00Z"
  }
  ```

**Example:**
```json
{
  "function": {
    "name": "validate_mx",
    "input": {
      "source": "sample_xml",
      "target": "validation_result"
    }
  }
}
```

**Validation Checks:**
- XML structure and syntax
- ISO20022 schema compliance
- CBPR+ (Central Bank Payment Regulation Plus) rules
- Field length and pattern constraints
- Business rule validation

---

### 4. Parse Plugin (`parse_mx`)

Parses ISO20022 XML messages into structured JSON format.

**Parameters:**
- `source` (required): Field name containing XML message
- `target` (required): Field name where parsed JSON will be stored

**Input:**
- ISO20022 XML string (with or without AppHdr envelope)

**Output:**
- JSON object with structured message data
- Metadata including message type and format

**Example:**
```json
{
  "function": {
    "name": "parse_mx",
    "input": {
      "source": "sample_xml",
      "target": "mx_json"
    }
  }
}
```

**Features:**
- Auto-detects message type from XML
- Handles both envelope and Document-only formats
- Preserves all message structure and data
- Type-safe parsing with validation

---

## Complete Workflow Example

Here's a complete end-to-end workflow demonstrating all four plugins:

```json
{
  "id": "mx_processing_pipeline",
  "name": "ISO20022 MX Processing Pipeline",
  "description": "Complete message processing with generation, publishing, validation, and parsing",
  "priority": 0,
  "tasks": [
    {
      "id": "step_1_generate",
      "name": "Generate Sample Data",
      "description": "Generate sample MX message from datafake scenario",
      "function": {
        "name": "generate_mx",
        "input": {
          "target": "sample_json"
        }
      }
    },
    {
      "id": "step_2_publish",
      "name": "Publish to XML",
      "description": "Convert JSON to ISO20022 XML format",
      "function": {
        "name": "publish_mx",
        "input": {
          "source": "sample_json",
          "target": "sample_xml"
        }
      }
    },
    {
      "id": "step_3_validate",
      "name": "Validate XML",
      "description": "Validate message against ISO20022 schemas",
      "function": {
        "name": "validate_mx",
        "input": {
          "source": "sample_xml",
          "target": "validation_result"
        }
      }
    },
    {
      "id": "step_4_parse",
      "name": "Parse XML",
      "description": "Parse XML back to structured JSON",
      "function": {
        "name": "parse_mx",
        "input": {
          "source": "sample_xml",
          "target": "mx_json"
        }
      }
    }
  ]
}
```

**Data Flow Visualization:**

```
                    ┌─────────────────┐
                    │  Datafake       │
                    │  Scenario       │
                    │  (Payload)      │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │   generate_mx   │
                    │  target: json   │
                    └────────┬────────┘
                             │
                    sample_json (JSON)
                             │
                             ▼
                    ┌─────────────────┐
                    │   publish_mx    │
                    │  source: json   │
                    │  target: xml    │
                    └────────┬────────┘
                             │
                    sample_xml (XML String)
                             │
                    ┌────────┴────────┐
                    │                 │
                    ▼                 ▼
           ┌─────────────────┐ ┌─────────────────┐
           │   validate_mx   │ │    parse_mx     │
           │  source: xml    │ │  source: xml    │
           │  target: result │ │  target: parsed │
           └────────┬────────┘ └────────┬────────┘
                    │                   │
                    ▼                   ▼
          validation_result        mx_json (JSON)
```

## Common Usage Patterns

### Pattern 1: Generate and Publish

Generate test data and convert to XML:

```json
{
  "tasks": [
    {
      "function": {
        "name": "generate_mx",
        "input": {"target": "msg_data"}
      }
    },
    {
      "function": {
        "name": "publish_mx",
        "input": {
          "source": "msg_data",
          "target": "msg_xml"
        }
      }
    }
  ]
}
```

### Pattern 2: Parse and Validate

Parse incoming XML and validate:

```json
{
  "tasks": [
    {
      "function": {
        "name": "parse_mx",
        "input": {
          "source": "incoming_xml",
          "target": "parsed_data"
        }
      }
    },
    {
      "function": {
        "name": "validate_mx",
        "input": {
          "source": "incoming_xml",
          "target": "validation"
        }
      }
    }
  ]
}
```

### Pattern 3: Round-Trip Testing

Test JSON → XML → JSON conversion:

```json
{
  "tasks": [
    {
      "function": {
        "name": "publish_mx",
        "input": {
          "source": "original_json",
          "target": "xml_output"
        }
      }
    },
    {
      "function": {
        "name": "parse_mx",
        "input": {
          "source": "xml_output",
          "target": "parsed_json"
        }
      }
    }
  ]
}
```

## Integration with Dataflow-rs

### Registering Plugins

```rust
use mx_message::plugin::register_mx_functions;
use dataflow_rs::Engine;

// Register all MX plugins
let custom_functions = register_mx_functions()
    .into_iter()
    .collect::<HashMap<_, _>>();

// Create engine with workflows and plugins
let engine = Engine::new(workflows, Some(custom_functions));
```

### Processing Messages

```rust
use dataflow_rs::engine::Message;

// Create message with datafake scenario
let scenario = load_datafake_scenario("pacs.008", "business_payment");
let mut message = Message::from_value(&scenario);

// Process through workflow
let result = engine.process_message(&mut message).await?;

// Access results
let generated_json = message.data().get("sample_json");
let xml_output = message.data().get("sample_xml");
let validation_result = message.data().get("validation_result");
let parsed_json = message.data().get("mx_json");
```

## Error Handling

All plugins return structured errors through the dataflow-rs error system:

```rust
use dataflow_rs::engine::error::{DataflowError, Result};

// Example error types:
// - DataflowError::Validation("'source' parameter is required")
// - DataflowError::Validation("Field 'data' not found in message")
// - DataflowError::Validation("XML parsing error: ...")
// - DataflowError::Validation("JSON serialization error: ...")
```

## Testing

Run the end-to-end test suite:

```bash
# Test all message types and scenarios
cargo test test_mx_workflow_pipeline

# Test specific message type
TEST_MESSAGE_TYPE=pacs.008 cargo test test_mx_workflow_pipeline

# Debug specific scenario
TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=cbpr_business_payment \
  TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 \
  cargo test test_mx_workflow_pipeline -- --nocapture
```

## Architecture

### Module Structure

```
src/plugin/
├── mod.rs           # Plugin registration and exports
├── common.rs        # Shared utilities (message type detection, etc.)
├── generate.rs      # Generate plugin implementation
├── parse.rs         # Parse plugin implementation
├── publish.rs       # Publish plugin implementation
├── validate.rs      # Validate plugin implementation
└── README.md        # This file
```

### Key Design Decisions

1. **Consistent Parameter Naming**: `source`/`target` pattern avoids confusion and improves clarity
2. **Field-based I/O**: Plugins read from and write to message data fields (not direct payload manipulation)
3. **Type Safety**: All operations use strongly-typed structs from generated ISO20022 code
4. **Format Detection**: Automatic message type detection from XML/JSON structure
5. **Error Propagation**: Structured error handling with clear error messages

## Supported Message Types

The plugin system supports all ISO20022 MX message types in this library:

- **pacs** - Payment Clearing and Settlement (pacs.008, pacs.009, pacs.003, pacs.004, pacs.002, pacs.010)
- **pain** - Payment Initiation (pain.001, pain.008)
- **camt** - Cash Management (camt.025, camt.027, camt.029, camt.052, camt.053, camt.054, camt.056, camt.057, camt.060, camt.107, camt.108, camt.109)

See `test_scenarios/` directory for example scenarios for each message type.

## Contributing

When adding new plugins or modifying existing ones:

1. Follow the `source`/`target` parameter naming convention
2. Update this README with new functionality
3. Add comprehensive tests in `tests/end2end.rs`
4. Ensure error messages are clear and actionable
5. Add datafake scenarios for new message types

## License

This plugin system is part of the MXMessage library and follows the same license terms.
