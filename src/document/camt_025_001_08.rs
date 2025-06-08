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

use crate::error::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

// MessageHeader91: Date and time at which the message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageHeader91 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
}

impl MessageHeader91 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.msg_id) {
            return Err(ValidationError::new(
                1005,
                "msg_id does not match the required pattern".to_string(),
            ));
        }
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.cre_dt_tm) {
            return Err(ValidationError::new(
                1005,
                "cre_dt_tm does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// OriginalMessageAndIssuer11: Specifies the original message name identifier to which the message refers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalMessageAndIssuer11 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "MsgNmId")]
    pub msg_nm_id: String,
}

impl OriginalMessageAndIssuer11 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.msg_id) {
            return Err(ValidationError::new(
                1005,
                "msg_id does not match the required pattern".to_string(),
            ));
        }
        if self.msg_nm_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "msg_nm_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.msg_nm_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "msg_nm_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.msg_nm_id) {
            return Err(ValidationError::new(
                1005,
                "msg_nm_id does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// Receipt61: Gives the status of the request.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Receipt61 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: OriginalMessageAndIssuer11,
    #[serde(rename = "ReqHdlg")]
    pub req_hdlg: RequestHandling31,
}

impl Receipt61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.orgnl_msg_id.validate()?;
        self.req_hdlg.validate()?;
        Ok(())
    }
}

// ReceiptV08: Details of the receipt.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReceiptV08 {
    #[serde(rename = "MsgHdr")]
    pub msg_hdr: MessageHeader91,
    #[serde(rename = "RctDtls")]
    pub rct_dtls: Vec<Receipt61>,
}

impl ReceiptV08 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.msg_hdr.validate()?;
        for item in &self.rct_dtls {
            item.validate()?
        }
        Ok(())
    }
}

// RequestHandling31: Provides detailed information on the status reason.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestHandling31 {
    #[serde(rename = "Sts")]
    pub sts: RequestStatus1Choice1,
    #[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
    pub sts_rsn: Option<StatusReasonInformation141>,
}

impl RequestHandling31 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.sts.validate()?;
        if let Some(ref val) = self.sts_rsn {
            val.validate()?
        }
        Ok(())
    }
}

// RequestStatus1Choice1: Request status, as published in an external request status code set.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestStatus1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl RequestStatus1Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// StatusReason6Choice1: Reason for the status, as published in an external reason code list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReason6Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl StatusReason6Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// StatusReasonInformation141: Further details on the status reason.
//
// Usage: Additional information can be used for several purposes such as the reporting of repaired information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReasonInformation141 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<StatusReason6Choice1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl StatusReasonInformation141 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.rsn {
            val.validate()?
        }
        if let Some(ref vec) = self.addtl_inf {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "addtl_inf is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 105 {
                    return Err(ValidationError::new(
                        1002,
                        "addtl_inf exceeds the maximum length of 105".to_string(),
                    ));
                }
                let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
                if !pattern.is_match(&item) {
                    return Err(ValidationError::new(
                        1005,
                        "addtl_inf does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}
