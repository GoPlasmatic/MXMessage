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

pub mod document;
pub mod error;
pub mod header;
pub mod mx_envelope;
pub mod parse_result;
pub mod sample;
pub mod scenario_config;
pub mod validation;
pub mod xml;

// Re-export sample generation utilities
pub use sample::{generate_sample_object, generate_sample_xml};
pub use scenario_config::ScenarioConfig;

// Re-export MX envelope and XML utilities
pub use mx_envelope::{MxDocument, MxEnvelope};
pub use xml::{XmlConfig, from_mx_xml, to_mx_xml};
