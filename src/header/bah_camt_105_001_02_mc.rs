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

// app_hdr ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AppHdr {
    #[serde(rename = "AppHdr")]
    pub app_hdr: BusinessApplicationHeaderV02,
}

impl AppHdr {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.app_hdr.validate()?;
        Ok(())
    }
}

// BranchAndFinancialInstitutionIdentification61: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification61 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification181,
}

impl BranchAndFinancialInstitutionIdentification61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        Ok(())
    }
}

// BusinessApplicationHeader51: Relative indication of the processing precedence of the message over a (set of) Business Messages with assigned priorities.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BusinessApplicationHeader51 {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    pub char_set: Option<String>,
    #[serde(rename = "Fr")]
    pub fr: Party44Choice1,
    #[serde(rename = "To")]
    pub to: Party44Choice1,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: String,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: String,
    #[serde(rename = "BizSvc", skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<String>,
    #[serde(rename = "CreDt")]
    pub cre_dt: String,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<String>,
}

impl BusinessApplicationHeader51 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fr.validate()?;
        self.to.validate()?;
        if self.biz_msg_idr.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "biz_msg_idr is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.biz_msg_idr.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "biz_msg_idr exceeds the maximum length of 35".to_string(),
            ));
        }
        if self.msg_def_idr.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "msg_def_idr is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.msg_def_idr.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "msg_def_idr exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.biz_svc {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "biz_svc is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "biz_svc exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.cre_dt) {
            return Err(ValidationError::new(
                1005,
                "cre_dt does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.cpy_dplct {
            val.validate()?
        }
        Ok(())
    }
}

// BusinessApplicationHeaderV02: Specifies the Business Application Header(s) of the Business Message(s) to which this Business Message relates.
// Can be used when replying to a query; can also be used when canceling or amending.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BusinessApplicationHeaderV02 {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    pub char_set: Option<String>,
    #[serde(rename = "Fr")]
    pub fr: Party44Choice1,
    #[serde(rename = "To")]
    pub to: Party44Choice1,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: String,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: String,
    #[serde(rename = "BizSvc")]
    pub biz_svc: Max35Textfixed,
    #[serde(rename = "MktPrctc", skip_serializing_if = "Option::is_none")]
    pub mkt_prctc: Option<ImplementationSpecification1>,
    #[serde(rename = "CreDt")]
    pub cre_dt: String,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<bool>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<Priority2Code>,
    #[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
    pub rltd: Option<BusinessApplicationHeader51>,
}

impl BusinessApplicationHeaderV02 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fr.validate()?;
        self.to.validate()?;
        if self.biz_msg_idr.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "biz_msg_idr is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.biz_msg_idr.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "biz_msg_idr exceeds the maximum length of 35".to_string(),
            ));
        }
        if self.msg_def_idr.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "msg_def_idr is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.msg_def_idr.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "msg_def_idr exceeds the maximum length of 35".to_string(),
            ));
        }
        self.biz_svc.validate()?;
        if let Some(ref val) = self.mkt_prctc {
            val.validate()?
        }
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.cre_dt) {
            return Err(ValidationError::new(
                1005,
                "cre_dt does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.cpy_dplct {
            val.validate()?
        }
        if let Some(ref val) = self.prty {
            val.validate()?
        }
        if let Some(ref val) = self.rltd {
            val.validate()?
        }
        Ok(())
    }
}

// ClearingSystemIdentification2Choice: Identification code for a clearing system, that has not yet been identified in the list of clearing systems.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ClearingSystemIdentification2Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 5 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 5".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prtry {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prtry is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prtry exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ClearingSystemMemberIdentification22: Identification of a member of a clearing system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemMemberIdentification22 {
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[serde(rename = "MmbId")]
    pub mmb_id: String,
}

impl ClearingSystemMemberIdentification22 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.clr_sys_id {
            val.validate()?
        }
        if self.mmb_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "mmb_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.mmb_id.chars().count() > 28 {
            return Err(ValidationError::new(
                1002,
                "mmb_id exceeds the maximum length of 28".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.mmb_id) {
            return Err(ValidationError::new(
                1005,
                "mmb_id does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// CopyDuplicate1Code: Message is for information/confirmation purposes. It is a duplicate of a message previously sent.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CopyDuplicate1Code {
    #[default]
    #[serde(rename = "CODU")]
    CodeCODU,
    #[serde(rename = "COPY")]
    CodeCOPY,
    #[serde(rename = "DUPL")]
    CodeDUPL,
}

impl CopyDuplicate1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// FinancialInstitutionIdentification181: Legal entity identifier of the financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification181 {
    #[serde(rename = "BICFI")]
    pub bicfi: String,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification22>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
}

impl FinancialInstitutionIdentification181 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern =
            Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
        if !pattern.is_match(&self.bicfi) {
            return Err(ValidationError::new(
                1005,
                "bicfi does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.clr_sys_mmb_id {
            val.validate()?
        }
        if let Some(ref val) = self.lei {
            let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "lei does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ImplementationSpecification1: Identifier which unambiguously identifies, within the implementation specification registry, the implementation specification to which the ISO 20022 message is compliant. This can be done via a URN. It can also contain a version number or date.
// For instance, "2018-01-01 – Version 2" or "urn:uuid:6e8bc430-9c3a-11d9-9669-0800200c9a66".
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImplementationSpecification1 {
    #[serde(rename = "Regy")]
    pub regy: String,
    #[serde(rename = "Id")]
    pub id: String,
}

impl ImplementationSpecification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.regy.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "regy is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.regy.chars().count() > 350 {
            return Err(ValidationError::new(
                1002,
                "regy exceeds the maximum length of 350".to_string(),
            ));
        }
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 2048 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 2048".to_string(),
            ));
        }
        Ok(())
    }
}

// Max35Text_fixed: swift.cbprplus.mlp.02
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Max35Textfixed {
    #[default]
    #[serde(rename = "swift.cbprplus.mlp.02")]
    CodeSWIFTCBPRPLUSMLP02,
}

impl Max35Textfixed {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// Party44Choice1: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party44Choice1 {
    #[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
    pub fi_id: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Party44Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.fi_id {
            val.validate()?
        }
        Ok(())
    }
}

// Priority2Code: Priority level is normal.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority2Code {
    #[default]
    #[serde(rename = "HIGH")]
    CodeHIGH,
    #[serde(rename = "NORM")]
    CodeNORM,
}

impl Priority2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
