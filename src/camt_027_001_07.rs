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

use crate::common::*;
use serde::{Deserialize, Serialize};

// ClaimNonReceiptV07 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClaimNonReceiptV07 {
    #[serde(rename = "Assgnmt")]
    pub assgnmt: CaseAssignment5,
    #[serde(rename = "Case", skip_serializing_if = "Option::is_none")]
    pub case: Option<Case5>,
    #[serde(rename = "Undrlyg")]
    pub undrlyg: UnderlyingTransaction5Choice,
    #[serde(rename = "CoverDtls", skip_serializing_if = "Option::is_none")]
    pub cover_dtls: Option<MissingCover4>,
    #[serde(rename = "InstrForAssgne", skip_serializing_if = "Option::is_none")]
    pub instr_for_assgne: Option<InstructionForAssignee1>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ClaimNonReceiptV07 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.assgnmt.validate()?;
        if let Some(ref val) = self.case {
            val.validate()?
        }
        self.undrlyg.validate()?;
        if let Some(ref val) = self.cover_dtls {
            val.validate()?
        }
        if let Some(ref val) = self.instr_for_assgne {
            val.validate()?
        }
        if let Some(ref vec) = self.splmtry_data {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// InstructionForAssignee1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstructionForAssignee1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<String>,
}

impl InstructionForAssignee1 {
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
        if let Some(ref val) = self.instr_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "instr_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "instr_inf exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// MissingCover4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MissingCover4 {
    #[serde(rename = "MssngCoverInd")]
    pub mssng_cover_ind: bool,
    #[serde(rename = "CoverCrrctn", skip_serializing_if = "Option::is_none")]
    pub cover_crrctn: Option<SettlementInstruction6>,
}

impl MissingCover4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cover_crrctn {
            val.validate()?
        }
        Ok(())
    }
}

// SettlementInstruction6 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementInstruction6 {
    #[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(
        rename = "InstgRmbrsmntAgtAcct",
        skip_serializing_if = "Option::is_none"
    )]
    pub instg_rmbrsmnt_agt_acct: Option<CashAccount38>,
    #[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(
        rename = "InstdRmbrsmntAgtAcct",
        skip_serializing_if = "Option::is_none"
    )]
    pub instd_rmbrsmnt_agt_acct: Option<CashAccount38>,
}

impl SettlementInstruction6 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.instg_rmbrsmnt_agt {
            val.validate()?
        }
        if let Some(ref val) = self.instg_rmbrsmnt_agt_acct {
            val.validate()?
        }
        if let Some(ref val) = self.instd_rmbrsmnt_agt {
            val.validate()?
        }
        if let Some(ref val) = self.instd_rmbrsmnt_agt_acct {
            val.validate()?
        }
        Ok(())
    }
}
