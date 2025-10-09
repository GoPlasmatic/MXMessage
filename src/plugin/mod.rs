//! # Dataflow Plugin
//!
//! Dataflow-rs integration for ISO20022 MX message processing.
//! Provides async function handlers for parse, validate, generate, and publish operations.

pub mod common;
pub mod generate;
pub mod parse;
pub mod publish;
pub mod validate;

use dataflow_rs::engine::AsyncFunctionHandler;

// Re-export the main plugin functions
pub use generate::Generate;
pub use parse::Parse;
pub use publish::Publish;
pub use validate::Validate;

/// Register all MX plugin functions for use in dataflow engine
pub fn register_mx_functions() -> Vec<(&'static str, Box<dyn AsyncFunctionHandler + Send + Sync>)> {
    vec![
        ("parse_mx", Box::new(Parse)),
        ("publish_mx", Box::new(Publish)),
        ("validate_mx", Box::new(Validate)),
        ("generate_mx", Box::new(Generate)),
    ]
}
