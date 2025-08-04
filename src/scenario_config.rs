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
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, ValidationError>;

/// Load a scenario configuration from a JSON file
pub fn load_scenario_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let content = fs::read_to_string(path)
        .map_err(|e| ValidationError::new(9998, format!("Failed to read scenario file: {e}")))?;

    serde_json::from_str(&content)
        .map_err(|e| ValidationError::new(9998, format!("Failed to parse scenario JSON: {e}")))
}

/// Find and load a scenario for a specific message type
///
/// Looks for configuration files in the following order:
/// 1. test_scenarios/{message_type}/standard.json
/// 2. test_scenarios/{message_type}/default.json
/// 3. First .json file in test_scenarios/{message_type}/
pub fn find_scenario_for_message_type(message_type: &str) -> Result<Value> {
    // Try both potential locations for test_scenarios
    let base_paths = vec![
        PathBuf::from("test_scenarios"),
        PathBuf::from("../test_scenarios"),
    ];

    for base_path in &base_paths {
        let mt_dir = base_path.join(message_type.to_lowercase());

        // Check if directory exists
        if !mt_dir.exists() {
            continue;
        }

        // Try standard.json first
        let standard_path = mt_dir.join("standard.json");
        if standard_path.exists() {
            return load_scenario_json(standard_path);
        }

        // Try default.json
        let default_path = mt_dir.join("default.json");
        if default_path.exists() {
            return load_scenario_json(default_path);
        }

        // Find the first .json file
        if let Ok(entries) = fs::read_dir(&mt_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    return load_scenario_json(path);
                }
            }
        }
    }

    Err(ValidationError::new(
        9998,
        format!(
            "No test scenarios found for message type: {}. Searched in: test_scenarios/{0} and ../test_scenarios/{0}",
            message_type.to_lowercase()
        ),
    ))
}

/// Find and load a specific scenario by name
pub fn find_scenario_by_name(message_type: &str, scenario_name: &str) -> Result<Value> {
    // Try both potential locations
    let paths = vec![
        format!(
            "test_scenarios/{}/{}.json",
            message_type.to_lowercase(),
            scenario_name
        ),
        format!(
            "../test_scenarios/{}/{}.json",
            message_type.to_lowercase(),
            scenario_name
        ),
    ];

    for path in &paths {
        if Path::new(path).exists() {
            return load_scenario_json(path);
        }
    }

    Err(ValidationError::new(
        9998,
        format!("Scenario '{scenario_name}' not found for {message_type}. Tried paths: {paths:?}"),
    ))
}
