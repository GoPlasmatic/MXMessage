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

// CancellationIndividualStatus1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CancellationIndividualStatus1Code {
    #[default]
    #[serde(rename = "RJCR")]
    CodeRJCR,
    #[serde(rename = "ACCR")]
    CodeACCR,
    #[serde(rename = "PDCR")]
    CodePDCR,
}

impl CancellationIndividualStatus1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// CancellationStatusReason3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CancellationStatusReason3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CancellationStatusReason3Choice {
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

// CancellationStatusReason4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CancellationStatusReason4 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification135>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancellationStatusReason3Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl CancellationStatusReason4 {
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

// ClaimNonReceipt2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClaimNonReceipt2 {
    #[serde(rename = "DtPrcd")]
    pub dt_prcd: String,
    #[serde(rename = "OrgnlNxtAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_nxt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl ClaimNonReceipt2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_nxt_agt {
            val.validate()?
        }
        Ok(())
    }
}

// ClaimNonReceipt2Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClaimNonReceipt2Choice {
    #[serde(rename = "Accptd", skip_serializing_if = "Option::is_none")]
    pub accptd: Option<ClaimNonReceipt2>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<ClaimNonReceiptRejectReason1Choice>,
}

impl ClaimNonReceipt2Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.accptd {
            val.validate()?
        }
        if let Some(ref val) = self.rjctd {
            val.validate()?
        }
        Ok(())
    }
}

// ClaimNonReceiptRejectReason1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClaimNonReceiptRejectReason1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ClaimNonReceiptRejectReason1Choice {
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

// Compensation2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Compensation2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "Rsn")]
    pub rsn: CompensationReason1Choice,
}

impl Compensation2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        self.dbtr_agt.validate()?;
        self.cdtr_agt.validate()?;
        self.rsn.validate()?;
        Ok(())
    }
}

// CompensationReason1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompensationReason1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CompensationReason1Choice {
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

// CorrectiveGroupInformation1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrectiveGroupInformation1 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "MsgNmId")]
    pub msg_nm_id: String,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<String>,
}

impl CorrectiveGroupInformation1 {
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
        Ok(())
    }
}

// CorrectiveInterbankTransaction2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrectiveInterbankTransaction2 {
    #[serde(rename = "GrpHdr", skip_serializing_if = "Option::is_none")]
    pub grp_hdr: Option<CorrectiveGroupInformation1>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<String>,
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: String,
}

impl CorrectiveInterbankTransaction2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.grp_hdr {
            val.validate()?
        }
        if let Some(ref val) = self.instr_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "instr_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "instr_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.end_to_end_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "end_to_end_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "end_to_end_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "uetr does not match the required pattern".to_string(),
                ));
            }
        }
        self.intr_bk_sttlm_amt.validate()?;
        Ok(())
    }
}

// CorrectivePaymentInitiation4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrectivePaymentInitiation4 {
    #[serde(rename = "GrpHdr", skip_serializing_if = "Option::is_none")]
    pub grp_hdr: Option<CorrectiveGroupInformation1>,
    #[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id: Option<String>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<String>,
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "InstdAmt")]
    pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
    pub reqd_colltn_dt: Option<String>,
}

impl CorrectivePaymentInitiation4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.grp_hdr {
            val.validate()?
        }
        if let Some(ref val) = self.pmt_inf_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "pmt_inf_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "pmt_inf_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.instr_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "instr_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "instr_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.end_to_end_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "end_to_end_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "end_to_end_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "uetr does not match the required pattern".to_string(),
                ));
            }
        }
        self.instd_amt.validate()?;
        if let Some(ref val) = self.reqd_exctn_dt {
            val.validate()?
        }
        Ok(())
    }
}

// CorrectiveTransaction4Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrectiveTransaction4Choice {
    #[serde(rename = "Initn", skip_serializing_if = "Option::is_none")]
    pub initn: Option<CorrectivePaymentInitiation4>,
    #[serde(rename = "IntrBk", skip_serializing_if = "Option::is_none")]
    pub intr_bk: Option<CorrectiveInterbankTransaction2>,
}

impl CorrectiveTransaction4Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.initn {
            val.validate()?
        }
        if let Some(ref val) = self.intr_bk {
            val.validate()?
        }
        Ok(())
    }
}

// GroupCancellationStatus1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum GroupCancellationStatus1Code {
    #[default]
    #[serde(rename = "PACR")]
    CodePACR,
    #[serde(rename = "RJCR")]
    CodeRJCR,
    #[serde(rename = "ACCR")]
    CodeACCR,
    #[serde(rename = "PDCR")]
    CodePDCR,
}

impl GroupCancellationStatus1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// InvestigationStatus5Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InvestigationStatus5Choice {
    #[serde(rename = "Conf", skip_serializing_if = "Option::is_none")]
    pub conf: Option<String>,
    #[serde(rename = "RjctdMod", skip_serializing_if = "Option::is_none")]
    pub rjctd_mod: Option<Vec<ModificationStatusReason1Choice>>,
    #[serde(rename = "DplctOf", skip_serializing_if = "Option::is_none")]
    pub dplct_of: Option<Case5>,
    #[serde(rename = "AssgnmtCxlConf", skip_serializing_if = "Option::is_none")]
    pub assgnmt_cxl_conf: Option<bool>,
}

impl InvestigationStatus5Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.conf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "conf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "conf exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.rjctd_mod {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.dplct_of {
            val.validate()?
        }
        Ok(())
    }
}

// ModificationStatusReason1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModificationStatusReason1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ModificationStatusReason1Choice {
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

// ModificationStatusReason2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModificationStatusReason2 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification135>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<ModificationStatusReason1Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl ModificationStatusReason2 {
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

// NumberOfCancellationsPerStatus1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NumberOfCancellationsPerStatus1 {
    #[serde(rename = "DtldNbOfTxs")]
    pub dtld_nb_of_txs: String,
    #[serde(rename = "DtldSts")]
    pub dtld_sts: CancellationIndividualStatus1Code,
    #[serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none")]
    pub dtld_ctrl_sum: Option<f64>,
}

impl NumberOfCancellationsPerStatus1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[0-9]{1,15}").unwrap();
        if !pattern.is_match(&self.dtld_nb_of_txs) {
            return Err(ValidationError::new(
                1005,
                "dtld_nb_of_txs does not match the required pattern".to_string(),
            ));
        }
        self.dtld_sts.validate()?;
        Ok(())
    }
}

// NumberOfTransactionsPerStatus1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NumberOfTransactionsPerStatus1 {
    #[serde(rename = "DtldNbOfTxs")]
    pub dtld_nb_of_txs: String,
    #[serde(rename = "DtldSts")]
    pub dtld_sts: TransactionIndividualStatus1Code,
    #[serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none")]
    pub dtld_ctrl_sum: Option<f64>,
}

impl NumberOfTransactionsPerStatus1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[0-9]{1,15}").unwrap();
        if !pattern.is_match(&self.dtld_nb_of_txs) {
            return Err(ValidationError::new(
                1005,
                "dtld_nb_of_txs does not match the required pattern".to_string(),
            ));
        }
        self.dtld_sts.validate()?;
        Ok(())
    }
}

// OriginalGroupHeader14 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalGroupHeader14 {
    #[serde(rename = "OrgnlGrpCxlId", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_cxl_id: Option<String>,
    #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
    pub rslvd_case: Option<Case5>,
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: String,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<String>,
    #[serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none")]
    pub orgnl_nb_of_txs: Option<String>,
    #[serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none")]
    pub orgnl_ctrl_sum: Option<f64>,
    #[serde(rename = "GrpCxlSts", skip_serializing_if = "Option::is_none")]
    pub grp_cxl_sts: Option<GroupCancellationStatus1Code>,
    #[serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
    #[serde(rename = "NbOfTxsPerCxlSts", skip_serializing_if = "Option::is_none")]
    pub nb_of_txs_per_cxl_sts: Option<Vec<NumberOfTransactionsPerStatus1>>,
}

impl OriginalGroupHeader14 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_grp_cxl_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_grp_cxl_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_grp_cxl_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rslvd_case {
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
        if let Some(ref val) = self.orgnl_nb_of_txs {
            let pattern = Regex::new("[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_nb_of_txs does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.grp_cxl_sts {
            val.validate()?
        }
        if let Some(ref vec) = self.cxl_sts_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.nb_of_txs_per_cxl_sts {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// OriginalPaymentInstruction30 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalPaymentInstruction30 {
    #[serde(rename = "OrgnlPmtInfCxlId", skip_serializing_if = "Option::is_none")]
    pub orgnl_pmt_inf_cxl_id: Option<String>,
    #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
    pub rslvd_case: Option<Case5>,
    #[serde(rename = "OrgnlPmtInfId")]
    pub orgnl_pmt_inf_id: String,
    #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
    #[serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none")]
    pub orgnl_nb_of_txs: Option<String>,
    #[serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none")]
    pub orgnl_ctrl_sum: Option<f64>,
    #[serde(rename = "PmtInfCxlSts", skip_serializing_if = "Option::is_none")]
    pub pmt_inf_cxl_sts: Option<GroupCancellationStatus1Code>,
    #[serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
    #[serde(rename = "NbOfTxsPerCxlSts", skip_serializing_if = "Option::is_none")]
    pub nb_of_txs_per_cxl_sts: Option<Vec<NumberOfCancellationsPerStatus1>>,
    #[serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none")]
    pub tx_inf_and_sts: Option<Vec<PaymentTransaction103>>,
}

impl OriginalPaymentInstruction30 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_pmt_inf_cxl_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_pmt_inf_cxl_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_pmt_inf_cxl_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rslvd_case {
            val.validate()?
        }
        if self.orgnl_pmt_inf_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.orgnl_pmt_inf_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.orgnl_grp_inf {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_nb_of_txs {
            let pattern = Regex::new("[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_nb_of_txs does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pmt_inf_cxl_sts {
            val.validate()?
        }
        if let Some(ref vec) = self.cxl_sts_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.nb_of_txs_per_cxl_sts {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.tx_inf_and_sts {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// PaymentTransaction102 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTransaction102 {
    #[serde(rename = "CxlStsId", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_id: Option<String>,
    #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
    pub rslvd_case: Option<Case5>,
    #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<String>,
    #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<String>,
    #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id: Option<String>,
    #[serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_clr_sys_ref: Option<String>,
    #[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr: Option<String>,
    #[serde(rename = "TxCxlSts", skip_serializing_if = "Option::is_none")]
    pub tx_cxl_sts: Option<CancellationIndividualStatus1Code>,
    #[serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
    #[serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none")]
    pub rsltn_rltd_inf: Option<ResolutionData1>,
    #[serde(
        rename = "OrgnlIntrBkSttlmAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Party40Choice>,
    #[serde(rename = "Assgne", skip_serializing_if = "Option::is_none")]
    pub assgne: Option<Party40Choice>,
    #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl PaymentTransaction102 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cxl_sts_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cxl_sts_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "cxl_sts_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rslvd_case {
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
        if let Some(ref val) = self.tx_cxl_sts {
            val.validate()?
        }
        if let Some(ref vec) = self.cxl_sts_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.rsltn_rltd_inf {
            val.validate()?
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
        if let Some(ref val) = self.orgnl_tx_ref {
            val.validate()?
        }
        Ok(())
    }
}

// PaymentTransaction103 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTransaction103 {
    #[serde(rename = "CxlStsId", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_id: Option<String>,
    #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
    pub rslvd_case: Option<Case5>,
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<String>,
    #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "TxCxlSts", skip_serializing_if = "Option::is_none")]
    pub tx_cxl_sts: Option<CancellationIndividualStatus1Code>,
    #[serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none")]
    pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
    #[serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlReqdExctnDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_reqd_exctn_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "OrgnlReqdColltnDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_reqd_colltn_dt: Option<String>,
    #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl PaymentTransaction103 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cxl_sts_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cxl_sts_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "cxl_sts_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rslvd_case {
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
        if let Some(ref val) = self.uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "uetr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tx_cxl_sts {
            val.validate()?
        }
        if let Some(ref vec) = self.cxl_sts_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.orgnl_instd_amt {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_reqd_exctn_dt {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_tx_ref {
            val.validate()?
        }
        Ok(())
    }
}

// PaymentTransaction107 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTransaction107 {
    #[serde(rename = "ModStsId", skip_serializing_if = "Option::is_none")]
    pub mod_sts_id: Option<String>,
    #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
    pub rslvd_case: Option<Case5>,
    #[serde(rename = "OrgnlGrpInf")]
    pub orgnl_grp_inf: OriginalGroupInformation29,
    #[serde(rename = "OrgnlPmtInfId", skip_serializing_if = "Option::is_none")]
    pub orgnl_pmt_inf_id: Option<String>,
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<String>,
    #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<String>,
    #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id: Option<String>,
    #[serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_clr_sys_ref: Option<String>,
    #[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr: Option<String>,
    #[serde(rename = "ModStsRsnInf", skip_serializing_if = "Option::is_none")]
    pub mod_sts_rsn_inf: Option<Vec<ModificationStatusReason2>>,
    #[serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none")]
    pub rsltn_rltd_inf: Option<ResolutionData1>,
    #[serde(
        rename = "OrgnlIntrBkSttlmAmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Party40Choice>,
    #[serde(rename = "Assgne", skip_serializing_if = "Option::is_none")]
    pub assgne: Option<Party40Choice>,
    #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl PaymentTransaction107 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.mod_sts_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "mod_sts_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "mod_sts_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rslvd_case {
            val.validate()?
        }
        self.orgnl_grp_inf.validate()?;
        if let Some(ref val) = self.orgnl_pmt_inf_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string(),
                ));
            }
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
        if let Some(ref vec) = self.mod_sts_rsn_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.rsltn_rltd_inf {
            val.validate()?
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
        if let Some(ref val) = self.orgnl_tx_ref {
            val.validate()?
        }
        Ok(())
    }
}

// ResolutionData1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResolutionData1 {
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<ClearingChannel2Code>,
    #[serde(rename = "Compstn", skip_serializing_if = "Option::is_none")]
    pub compstn: Option<Compensation2>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Vec<Charges7>>,
}

impl ResolutionData1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.end_to_end_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "end_to_end_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "end_to_end_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "uetr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.intr_bk_sttlm_amt {
            val.validate()?
        }
        if let Some(ref val) = self.clr_chanl {
            val.validate()?
        }
        if let Some(ref val) = self.compstn {
            val.validate()?
        }
        if let Some(ref vec) = self.chrgs {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// ResolutionOfInvestigationV09 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResolutionOfInvestigationV09 {
    #[serde(rename = "Assgnmt")]
    pub assgnmt: CaseAssignment5,
    #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
    pub rslvd_case: Option<Case5>,
    #[serde(rename = "Sts")]
    pub sts: InvestigationStatus5Choice,
    #[serde(rename = "CxlDtls", skip_serializing_if = "Option::is_none")]
    pub cxl_dtls: Option<Vec<UnderlyingTransaction22>>,
    #[serde(rename = "ModDtls", skip_serializing_if = "Option::is_none")]
    pub mod_dtls: Option<PaymentTransaction107>,
    #[serde(rename = "ClmNonRctDtls", skip_serializing_if = "Option::is_none")]
    pub clm_non_rct_dtls: Option<ClaimNonReceipt2Choice>,
    #[serde(rename = "StmtDtls", skip_serializing_if = "Option::is_none")]
    pub stmt_dtls: Option<StatementResolutionEntry4>,
    #[serde(rename = "CrrctnTx", skip_serializing_if = "Option::is_none")]
    pub crrctn_tx: Option<CorrectiveTransaction4Choice>,
    #[serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none")]
    pub rsltn_rltd_inf: Option<ResolutionData1>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ResolutionOfInvestigationV09 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.assgnmt.validate()?;
        if let Some(ref val) = self.rslvd_case {
            val.validate()?
        }
        self.sts.validate()?;
        if let Some(ref vec) = self.cxl_dtls {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.mod_dtls {
            val.validate()?
        }
        if let Some(ref val) = self.clm_non_rct_dtls {
            val.validate()?
        }
        if let Some(ref val) = self.stmt_dtls {
            val.validate()?
        }
        if let Some(ref val) = self.crrctn_tx {
            val.validate()?
        }
        if let Some(ref val) = self.rsltn_rltd_inf {
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

// StatementResolutionEntry4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatementResolutionEntry4 {
    #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
    #[serde(rename = "OrgnlStmtId", skip_serializing_if = "Option::is_none")]
    pub orgnl_stmt_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<String>,
    #[serde(rename = "CrrctdAmt", skip_serializing_if = "Option::is_none")]
    pub crrctd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Vec<Charges6>>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice>,
}

impl StatementResolutionEntry4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_grp_inf {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_stmt_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_stmt_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_stmt_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "uetr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.acct_svcr_ref {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "acct_svcr_ref is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "acct_svcr_ref exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.crrctd_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.chrgs {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.purp {
            val.validate()?
        }
        Ok(())
    }
}

// TransactionIndividualStatus1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionIndividualStatus1Code {
    #[default]
    #[serde(rename = "ACTC")]
    CodeACTC,
    #[serde(rename = "RJCT")]
    CodeRJCT,
    #[serde(rename = "PDNG")]
    CodePDNG,
    #[serde(rename = "ACCP")]
    CodeACCP,
    #[serde(rename = "ACSP")]
    CodeACSP,
    #[serde(rename = "ACSC")]
    CodeACSC,
    #[serde(rename = "ACCR")]
    CodeACCR,
    #[serde(rename = "ACWC")]
    CodeACWC,
}

impl TransactionIndividualStatus1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// UnderlyingTransaction22 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct UnderlyingTransaction22 {
    #[serde(rename = "OrgnlGrpInfAndSts", skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf_and_sts: Option<OriginalGroupHeader14>,
    #[serde(rename = "OrgnlPmtInfAndSts", skip_serializing_if = "Option::is_none")]
    pub orgnl_pmt_inf_and_sts: Option<Vec<OriginalPaymentInstruction30>>,
    #[serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none")]
    pub tx_inf_and_sts: Option<Vec<PaymentTransaction102>>,
}

impl UnderlyingTransaction22 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_grp_inf_and_sts {
            val.validate()?
        }
        if let Some(ref vec) = self.orgnl_pmt_inf_and_sts {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.tx_inf_and_sts {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}
