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

use crate::parse_result::{ErrorCollector, ParserConfig};

/// Trait for types that support validation with error collection
pub trait Validate {
    /// Validate the instance and collect errors with path information
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector);
}

/// Helper functions for validation
pub mod helpers {
    use crate::error::ValidationError;
    use crate::parse_result::{ErrorCollector, ParserConfig};
    use regex::Regex;

    /// Validate string length
    pub fn validate_length(
        value: &str,
        field_name: &str,
        min: Option<usize>,
        max: Option<usize>,
        path: &str,
        config: &ParserConfig,
        collector: &mut ErrorCollector,
    ) -> bool {
        let mut valid = true;

        if let Some(min_len) = min {
            if value.chars().count() < min_len {
                let error = ValidationError::new(
                    1001,
                    format!("{field_name} is shorter than the minimum length of {min_len}"),
                )
                .with_field(field_name.to_string())
                .with_path(path.to_string());

                if config.fail_fast {
                    collector.add_critical_error(error);
                    return false;
                } else {
                    collector.add_error(error);
                    valid = false;
                }
            }
        }

        if let Some(max_len) = max {
            if value.chars().count() > max_len {
                let error = ValidationError::new(
                    1002,
                    format!("{field_name} exceeds the maximum length of {max_len}"),
                )
                .with_field(field_name.to_string())
                .with_path(path.to_string());

                if config.fail_fast {
                    collector.add_critical_error(error);
                    return false;
                } else {
                    collector.add_error(error);
                    valid = false;
                }
            }
        }

        valid
    }

    /// Validate string pattern
    pub fn validate_pattern(
        value: &str,
        field_name: &str,
        pattern: &str,
        path: &str,
        config: &ParserConfig,
        collector: &mut ErrorCollector,
    ) -> bool {
        // Trim whitespace before validation
        let trimmed_value = value.trim();
        
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => {
                collector.add_critical_error(
                    ValidationError::new(
                        9999,
                        format!("Invalid regex pattern for {field_name}: {pattern}"),
                    )
                    .with_field(field_name.to_string())
                    .with_path(path.to_string()),
                );
                return false;
            }
        };

        if !regex.is_match(trimmed_value) {
            let error = ValidationError::new(
                1005,
                format!("{field_name} does not match the required pattern (value: '{}')", value),
            )
            .with_field(field_name.to_string())
            .with_path(path.to_string());

            if config.fail_fast {
                collector.add_critical_error(error);
                return false;
            } else {
                collector.add_error(error);
                return false;
            }
        }

        true
    }

    /// Validate required field
    pub fn validate_required<T>(
        value: &Option<T>,
        field_name: &str,
        path: &str,
        _config: &ParserConfig,
        collector: &mut ErrorCollector,
    ) -> bool {
        if value.is_none() {
            let error = ValidationError::new(1003, format!("{field_name} is required"))
                .with_field(field_name.to_string())
                .with_path(path.to_string());

            collector.add_critical_error(error);
            return false;
        }
        true
    }

    /// Create a child path for nested validation
    pub fn child_path(parent: &str, field: &str) -> String {
        if parent.is_empty() {
            field.to_string()
        } else {
            format!("{parent}.{field}")
        }
    }
}
