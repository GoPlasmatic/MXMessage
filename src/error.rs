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

use thiserror::Error;

/// MX Message processing errors
#[derive(Error, Debug)]
pub enum MxError {
    /// XML serialization error
    #[error("XML serialization error: {0}")]
    XmlSerialization(String),

    /// XML deserialization error
    #[error("XML deserialization error: {0}")]
    XmlDeserialization(String),

    /// XML validation error
    #[error("XML validation error: {0}")]
    XmlValidation(String),

    /// General XML error
    #[error("XML error: {0}")]
    Xml(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Validation error with details
    #[error("Validation error: {message}")]
    Validation {
        code: u32,
        message: String,
        field: Option<String>,
        path: Option<String>,
    },

    /// Format detection error
    #[error("Cannot detect message format")]
    FormatDetection,

    /// Unknown message type
    #[error("Unknown message type: {0}")]
    UnknownMessageType(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Legacy ValidationError for backward compatibility
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub code: u32,
    pub message: String,
    pub field: Option<String>,
    pub path: Option<String>,
}

impl ValidationError {
    pub fn new(code: u32, message: String) -> Self {
        ValidationError {
            code,
            message,
            field: None,
            path: None,
        }
    }

    pub fn with_field(mut self, field: String) -> Self {
        self.field = Some(field);
        self
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
}

impl From<ValidationError> for MxError {
    fn from(err: ValidationError) -> Self {
        MxError::Validation {
            code: err.code,
            message: err.message,
            field: err.field,
            path: err.path,
        }
    }
}
