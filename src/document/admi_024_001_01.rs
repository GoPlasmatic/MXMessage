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

// BranchAndFinancialInstitutionIdentification81: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification81 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification231,
}

impl BranchAndFinancialInstitutionIdentification81 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        Ok(())
    }
}

// CorrespondenceNotification11: Provides information about the notification in narrative form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrespondenceNotification11 {
    #[serde(rename = "SndrNtfctnId")]
    pub sndr_ntfctn_id: String,
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: NotificationType1Choice1,
    #[serde(rename = "NtfctnNrrtv")]
    pub ntfctn_nrrtv: Vec<String>,
}

impl CorrespondenceNotification11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.sndr_ntfctn_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "sndr_ntfctn_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.sndr_ntfctn_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "sndr_ntfctn_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.sndr_ntfctn_id) {
            return Err(ValidationError::new(
                1005,
                "sndr_ntfctn_id does not match the required pattern".to_string(),
            ));
        }
        self.ntfctn_tp.validate()?;
        for item in &self.ntfctn_nrrtv {
            if item.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ntfctn_nrrtv is shorter than the minimum length of 1".to_string(),
                ));
            }
            if item.chars().count() > 2000 {
                return Err(ValidationError::new(
                    1002,
                    "ntfctn_nrrtv exceeds the maximum length of 2000".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(item) {
                return Err(ValidationError::new(
                    1005,
                    "ntfctn_nrrtv does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// FinancialInstitutionIdentification231: Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362 "Banking - Banking telecommunication messages - Business identifier code (BIC)".
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification231 {
    #[serde(rename = "BICFI")]
    pub bicfi: String,
}

impl FinancialInstitutionIdentification231 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern =
            Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
        if !pattern.is_match(&self.bicfi) {
            return Err(ValidationError::new(
                1005,
                "bicfi does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// GroupHeader1291: Party that receives the notification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader1291 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "Sndr")]
    pub sndr: Party50Choice1,
    #[serde(rename = "Rcvr")]
    pub rcvr: Party50Choice1,
}

impl GroupHeader1291 {
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
        self.sndr.validate()?;
        self.rcvr.validate()?;
        Ok(())
    }
}

// NotificationOfCorrespondenceV01: Set of elements used to provide further details on the notification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationOfCorrespondenceV01 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader1291,
    #[serde(rename = "NtfctnData")]
    pub ntfctn_data: CorrespondenceNotification11,
}

impl NotificationOfCorrespondenceV01 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        self.ntfctn_data.validate()?;
        Ok(())
    }
}

// NotificationType1Choice1: Notification type, as published in an external notification type code set.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl NotificationType1Choice1 {
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

// Party50Choice1: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party50Choice1 {
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification81>,
}

impl Party50Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.agt {
            val.validate()?
        }
        Ok(())
    }
}
