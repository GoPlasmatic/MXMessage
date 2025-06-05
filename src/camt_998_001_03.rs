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

use crate::common::ValidationError;
use serde::{Deserialize, Serialize};

// CashManagementProprietaryMessageV03: Business content of this element is not specified.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashManagementProprietaryMessageV03 {
    #[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
    pub msg_hdr: Option<MessageHeader1>,
    #[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
    pub rltd: Option<MessageReference2>,
    #[serde(rename = "Prvs", skip_serializing_if = "Option::is_none")]
    pub prvs: Option<MessageReference2>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<MessageReference2>,
    #[serde(rename = "PrtryData")]
    pub prtry_data: ProprietaryData5,
}

impl CashManagementProprietaryMessageV03 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.msg_hdr {
            val.validate()?
        }
        if let Some(ref val) = self.rltd {
            val.validate()?
        }
        if let Some(ref val) = self.prvs {
            val.validate()?
        }
        if let Some(ref val) = self.othr {
            val.validate()?
        }
        self.prtry_data.validate()?;
        Ok(())
    }
}

// MessageHeader1: Date and time at which the message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageHeader1 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<String>,
}

impl MessageHeader1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.msg_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "msg_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.msg_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "msg_id exceeds the maximum length of 35".to_string(),
            ));
        }
        Ok(())
    }
}

// MessageReference2: Business reference of the present message assigned by the party issuing the message. This reference must be unique amongst all messages of the same name sent by the same party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageReference2 {
    #[serde(rename = "Ref")]
    pub ref_attr: String,
}

impl MessageReference2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.ref_attr.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "ref_attr is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.ref_attr.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "ref_attr exceeds the maximum length of 35".to_string(),
            ));
        }
        Ok(())
    }
}

// ProprietaryData5: Technical element wrapping the proprietary message.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryData5 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Data")]
    pub data: SupplementaryDataEnvelope1,
}

impl ProprietaryData5 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.tp.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "tp is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.tp.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "tp exceeds the maximum length of 35".to_string(),
            ));
        }
        self.data.validate()?;
        Ok(())
    }
}

// SupplementaryDataEnvelope1: Technical component that contains the validated supplementary data information. This technical envelope allows to segregate the supplementary data information from any other information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SupplementaryDataEnvelope1 {}

impl SupplementaryDataEnvelope1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
