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
// You may obtain a copy of this library at
// https://github.com/GoPlasmatic/MXMessage

use crate::error::ValidationError;
use serde::{Deserialize, Serialize};

/// Result of parsing with error collection support
#[derive(Debug, Clone, PartialEq)]
pub enum ParseResult<T> {
    /// All fields parsed successfully
    Success(T),
    /// Some errors occurred but structure is valid
    PartialSuccess(T, Vec<ValidationError>),
    /// Too many critical errors to continue
    Failure(Vec<ValidationError>),
}

impl<T> ParseResult<T> {
    /// Check if the parse was successful (no errors)
    pub fn is_success(&self) -> bool {
        matches!(self, ParseResult::Success(_))
    }

    /// Check if the parse resulted in failure
    pub fn is_failure(&self) -> bool {
        matches!(self, ParseResult::Failure(_))
    }

    /// Get the parsed value if available
    pub fn value(&self) -> Option<&T> {
        match self {
            ParseResult::Success(val) | ParseResult::PartialSuccess(val, _) => Some(val),
            ParseResult::Failure(_) => None,
        }
    }

    /// Get all errors if any
    pub fn errors(&self) -> Vec<&ValidationError> {
        match self {
            ParseResult::Success(_) => vec![],
            ParseResult::PartialSuccess(_, errors) | ParseResult::Failure(errors) => {
                errors.iter().collect()
            }
        }
    }

    /// Convert to Result for compatibility
    pub fn to_result(self) -> Result<T, Vec<ValidationError>> {
        match self {
            ParseResult::Success(val) => Ok(val),
            ParseResult::PartialSuccess(val, _) => Ok(val),
            ParseResult::Failure(errors) => Err(errors),
        }
    }
}

/// Configuration for parsing behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// If true, stop at first error. If false, collect all errors.
    pub fail_fast: bool,
    /// If true, validate optional fields. If false, skip validation on None fields.
    pub validate_optional_fields: bool,
    /// If true, attempt to collect all possible errors even when structure is invalid.
    pub collect_all_errors: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        ParserConfig {
            fail_fast: false,
            validate_optional_fields: true,
            collect_all_errors: true,
        }
    }
}

impl ParserConfig {
    /// Create a new parser configuration with fail_fast enabled
    pub fn fail_fast() -> Self {
        ParserConfig {
            fail_fast: true,
            validate_optional_fields: true,
            collect_all_errors: false,
        }
    }

    /// Create a new parser configuration for lenient parsing
    pub fn lenient() -> Self {
        ParserConfig {
            fail_fast: false,
            validate_optional_fields: false,
            collect_all_errors: false,
        }
    }
}

/// Helper struct for collecting validation errors
#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<ValidationError>,
    has_critical_errors: bool,
}

impl ErrorCollector {
    pub fn new() -> Self {
        ErrorCollector {
            errors: Vec::new(),
            has_critical_errors: false,
        }
    }

    /// Add an error to the collection
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Add a critical error (e.g., missing required field)
    pub fn add_critical_error(&mut self, error: ValidationError) {
        self.has_critical_errors = true;
        self.errors.push(error);
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are critical errors
    pub fn has_critical_errors(&self) -> bool {
        self.has_critical_errors
    }

    /// Get all collected errors
    pub fn errors(self) -> Vec<ValidationError> {
        self.errors
    }

    /// Get the count of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}
