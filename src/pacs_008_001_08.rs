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

// CreditTransferTransaction39 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditTransferTransaction39 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification7,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,
    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<Priority3Code>,
    #[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
    #[serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_req: Option<SettlementTimeRequest2>,
    #[serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<String>,
    #[serde(rename = "PoolgAdjstmntDt", skip_serializing_if = "Option::is_none")]
    pub poolg_adjstmnt_dt: Option<String>,
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<f64>,
    #[serde(rename = "ChrgBr")]
    pub chrg_br: ChargeBearerType1Code,
    #[serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none")]
    pub chrgs_inf: Option<Vec<Charges7>>,
    #[serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1_acct: Option<CashAccount38>,
    #[serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2_acct: Option<CashAccount38>,
    #[serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3_acct: Option<CashAccount38>,
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1_acct: Option<CashAccount38>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2_acct: Option<CashAccount38>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3_acct: Option<CashAccount38>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification135>,
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<PartyIdentification135>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification135,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount38>,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification135,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount38>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<PartyIdentification135>,
    #[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent1>>,
    #[serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice>,
    #[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
    pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInformation8>,
    #[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
    pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation16>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction39 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.pmt_id.validate()?;
        if let Some(ref val) = self.pmt_tp_inf {
            val.validate()?
        }
        self.intr_bk_sttlm_amt.validate()?;
        if let Some(ref val) = self.sttlm_prty {
            val.validate()?
        }
        if let Some(ref val) = self.sttlm_tm_indctn {
            val.validate()?
        }
        if let Some(ref val) = self.sttlm_tm_req {
            val.validate()?
        }
        if let Some(ref val) = self.instd_amt {
            val.validate()?
        }
        self.chrg_br.validate()?;
        if let Some(ref vec) = self.chrgs_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.prvs_instg_agt1 {
            val.validate()?
        }
        if let Some(ref val) = self.prvs_instg_agt1_acct {
            val.validate()?
        }
        if let Some(ref val) = self.prvs_instg_agt2 {
            val.validate()?
        }
        if let Some(ref val) = self.prvs_instg_agt2_acct {
            val.validate()?
        }
        if let Some(ref val) = self.prvs_instg_agt3 {
            val.validate()?
        }
        if let Some(ref val) = self.prvs_instg_agt3_acct {
            val.validate()?
        }
        if let Some(ref val) = self.instg_agt {
            val.validate()?
        }
        if let Some(ref val) = self.instd_agt {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt1 {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt1_acct {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt2 {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt2_acct {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt3 {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt3_acct {
            val.validate()?
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.initg_pty {
            val.validate()?
        }
        self.dbtr.validate()?;
        if let Some(ref val) = self.dbtr_acct {
            val.validate()?
        }
        self.dbtr_agt.validate()?;
        if let Some(ref val) = self.dbtr_agt_acct {
            val.validate()?
        }
        self.cdtr_agt.validate()?;
        if let Some(ref val) = self.cdtr_agt_acct {
            val.validate()?
        }
        self.cdtr.validate()?;
        if let Some(ref val) = self.cdtr_acct {
            val.validate()?
        }
        if let Some(ref val) = self.ultmt_cdtr {
            val.validate()?
        }
        if let Some(ref vec) = self.instr_for_cdtr_agt {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.instr_for_nxt_agt {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.purp {
            val.validate()?
        }
        if let Some(ref vec) = self.rgltry_rptg {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.tax {
            val.validate()?
        }
        if let Some(ref vec) = self.rltd_rmt_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.rmt_inf {
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

// FIToFICustomerCreditTransferV08 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FIToFICustomerCreditTransferV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader93,
    #[serde(rename = "CdtTrfTxInf")]
    pub cdt_trf_tx_inf: Vec<CreditTransferTransaction39>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FIToFICustomerCreditTransferV08 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        for item in &self.cdt_trf_tx_inf {
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

// RegulatoryAuthority2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<String>,
}

impl RegulatoryAuthority2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "nm exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctry {
            let pattern = Regex::new("[A-Z]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// RegulatoryReporting3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
    #[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
    pub authrty: Option<RegulatoryAuthority2>,
    #[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
    pub dtls: Option<Vec<StructuredRegulatoryReporting3>>,
}

impl RegulatoryReporting3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dbt_cdt_rptg_ind {
            val.validate()?
        }
        if let Some(ref val) = self.authrty {
            val.validate()?
        }
        if let Some(ref vec) = self.dtls {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// RegulatoryReportingType1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum RegulatoryReportingType1Code {
    #[default]
    #[serde(rename = "CRED")]
    CodeCRED,
    #[serde(rename = "DEBT")]
    CodeDEBT,
    #[serde(rename = "BOTH")]
    CodeBOTH,
}

impl RegulatoryReportingType1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// StructuredRegulatoryReporting3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRegulatoryReporting3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<String>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<String>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Inf", skip_serializing_if = "Option::is_none")]
    pub inf: Option<Vec<String>>,
}

impl StructuredRegulatoryReporting3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tp is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tp exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctry {
            let pattern = Regex::new("[A-Z]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 10 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 10".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        if let Some(ref vec) = self.inf {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "inf is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 35 {
                    return Err(ValidationError::new(
                        1002,
                        "inf exceeds the maximum length of 35".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}
