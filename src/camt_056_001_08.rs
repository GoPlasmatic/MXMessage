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
use regex::Regex;
use serde::{Deserialize, Serialize};

// CancellationReason33Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CancellationReason33Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CancellationReason33Choice {
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

// ControlData1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ControlData1 {
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: String,
    #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<f64>,
}

impl ControlData1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[0-9]{1,15}").unwrap();
        if !pattern.is_match(&self.nb_of_txs) {
            return Err(ValidationError::new(
                1005,
                "nb_of_txs does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// FIToFIPaymentCancellationRequestV08 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FIToFIPaymentCancellationRequestV08 {
    #[serde(rename = "Assgnmt")]
    pub assgnmt: CaseAssignment5,
    #[serde(rename = "Case", skip_serializing_if = "Option::is_none")]
    pub case: Option<Case5>,
    #[serde(rename = "CtrlData", skip_serializing_if = "Option::is_none")]
    pub ctrl_data: Option<ControlData1>,
    #[serde(rename = "Undrlyg")]
    pub undrlyg: Vec<UnderlyingTransaction23>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FIToFIPaymentCancellationRequestV08 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.assgnmt.validate()?;
        if let Some(ref val) = self.case {
            val.validate()?
        }
        if let Some(ref val) = self.ctrl_data {
            val.validate()?
        }
        for item in &self.undrlyg {
            item.validate()?
        }
        if let Some(ref vec) = self.splmtry_data {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// OriginalGroupHeader15 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalGroupHeader15 {
    #[serde(rename = "GrpCxlId", skip_serializing_if = "Option::is_none")]
    pub grp_cxl_id: Option<String>,
    #[serde(rename = "Case", skip_serializing_if = "Option::is_none")]
    pub case: Option<Case5>,
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: String,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<String>,
    #[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
    pub nb_of_txs: Option<String>,
    #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<f64>,
    #[serde(rename = "GrpCxl", skip_serializing_if = "Option::is_none")]
    pub grp_cxl: Option<bool>,
    #[serde(rename = "CxlRsnInf", skip_serializing_if = "Option::is_none")]
    pub cxl_rsn_inf: Option<Vec<PaymentCancellationReason5>>,
}

impl OriginalGroupHeader15 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.grp_cxl_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "grp_cxl_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "grp_cxl_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.case {
            val.validate()?
        }
        if self.orgnl_msg_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "orgnl_msg_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.orgnl_msg_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "orgnl_msg_id exceeds the maximum length of 35".to_string(),
            ));
        }
        if self.orgnl_msg_nm_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.orgnl_msg_nm_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "orgnl_msg_nm_id exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.nb_of_txs {
            let pattern = Regex::new("[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb_of_txs does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.cxl_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// PaymentCancellationReason5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentCancellationReason5 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification135>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancellationReason33Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl PaymentCancellationReason5 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgtr {
            val.validate()?
        }
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
            }
        }
        Ok(())
    }
}

// PaymentTransaction106 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTransaction106 {
    #[serde(rename = "CxlId", skip_serializing_if = "Option::is_none")]
    pub cxl_id: Option<String>,
    #[serde(rename = "Case", skip_serializing_if = "Option::is_none")]
    pub case: Option<Case5>,
    #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<String>,
    #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<String>,
    #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id: Option<String>,
    #[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr: Option<String>,
    #[serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_clr_sys_ref: Option<String>,
    #[serde(
        rename = "OrgnlIntrBkSttlmAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Assgne", skip_serializing_if = "Option::is_none")]
    pub assgne: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CxlRsnInf", skip_serializing_if = "Option::is_none")]
    pub cxl_rsn_inf: Option<Vec<PaymentCancellationReason5>>,
    #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction106 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cxl_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cxl_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "cxl_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.case {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_grp_inf {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_instr_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_instr_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_instr_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.orgnl_end_to_end_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_end_to_end_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.orgnl_tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.orgnl_uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_uetr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.orgnl_clr_sys_ref {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_clr_sys_ref is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_clr_sys_ref exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.orgnl_intr_bk_sttlm_amt {
            val.validate()?
        }
        if let Some(ref val) = self.assgnr {
            val.validate()?
        }
        if let Some(ref val) = self.assgne {
            val.validate()?
        }
        if let Some(ref vec) = self.cxl_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.orgnl_tx_ref {
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

// UnderlyingTransaction23 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct UnderlyingTransaction23 {
    #[serde(rename = "OrgnlGrpInfAndCxl", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf_and_cxl: Option<OriginalGroupHeader15>,
    #[serde(rename = "TxInf", skip_serializing_if = "Option::is_none")]
    pub tx_inf: Option<Vec<PaymentTransaction106>>,
}

impl UnderlyingTransaction23 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_grp_inf_and_cxl {
            val.validate()?
        }
        if let Some(ref vec) = self.tx_inf {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}
