use mx_message::document::pacs_008_001_08::FIToFICustomerCreditTransferV08;
use mx_message::parse_result::{ErrorCollector, ParserConfig};
use mx_message::validation::Validate;

fn main() {
    println!("=== Validation Example ===\n");

    // Create a parser config with default settings
    let config = ParserConfig::default();

    // Create an empty transfer message (invalid - missing required fields)
    let transfer = FIToFICustomerCreditTransferV08::default();

    // Validate with error collection
    println!("1. Validating empty message (should fail with multiple errors):");
    let mut collector = ErrorCollector::new();
    transfer.validate("", &config, &mut collector);

    if collector.has_errors() {
        println!(
            "   Validation failed with {} errors:",
            collector.error_count()
        );
        for error in collector.errors() {
            println!(
                "     - Error {}: {} (field: {:?}, path: {:?})",
                error.code, error.message, error.field, error.path
            );
        }
    } else {
        println!("   Validation successful!");
    }

    println!("\n2. Testing fail_fast mode:");

    // Create a config with fail_fast enabled
    let fail_fast_config = ParserConfig {
        fail_fast: true,
        validate_optional_fields: false,
        collect_all_errors: false,
    };

    // Validate with fail_fast - should stop at first error
    let mut collector2 = ErrorCollector::new();
    transfer.validate("", &fail_fast_config, &mut collector2);

    if collector2.has_errors() {
        println!(
            "   Validation failed with {} error(s) (fail_fast mode):",
            collector2.error_count()
        );
        for error in collector2.errors() {
            println!(
                "     - Error {}: {} (field: {:?}, path: {:?})",
                error.code, error.message, error.field, error.path
            );
        }
    } else {
        println!("   Validation successful!");
    }

    println!("\n3. Testing lenient mode (skip optional fields):");

    // Create a lenient config
    let lenient_config = ParserConfig::lenient();

    // Validate with lenient config
    let mut collector3 = ErrorCollector::new();
    transfer.validate("", &lenient_config, &mut collector3);

    if collector3.has_errors() {
        println!(
            "   Validation failed with {} errors (lenient mode):",
            collector3.error_count()
        );
        for error in collector3.errors() {
            println!(
                "     - Error {}: {} (field: {:?}, path: {:?})",
                error.code, error.message, error.field, error.path
            );
        }
    } else {
        println!("   Validation successful!");
    }

    println!("\n=== End of Validation Example ===");
}
