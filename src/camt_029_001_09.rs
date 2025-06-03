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
use regex::Regex;
use serde::{Deserialize, Serialize};

// AccountIdentification4Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountIdentification4Choice {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.iban {
            let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "iban does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.othr {
            val.validate()?
        }
        Ok(())
    }
}

// AccountSchemeName1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl AccountSchemeName1Choice {
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

// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl ActiveCurrencyAndAmount {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// AddressType2Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum AddressType2Code {
    #[default]
    #[serde(rename = "ADDR")]
    CodeADDR,
    #[serde(rename = "PBOX")]
    CodePBOX,
    #[serde(rename = "HOME")]
    CodeHOME,
    #[serde(rename = "BIZZ")]
    CodeBIZZ,
    #[serde(rename = "MLTO")]
    CodeMLTO,
    #[serde(rename = "DLVY")]
    CodeDLVY,
}

impl AddressType2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// AddressType3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddressType3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            val.validate()?
        }
        if let Some(ref val) = self.prtry {
            val.validate()?
        }
        Ok(())
    }
}

// AmendmentInformationDetails13 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmendmentInformationDetails13 {
    #[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
    pub orgnl_mndt_id: Option<String>,
    #[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
    #[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr: Option<PartyIdentification135>,
    #[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_acct: Option<CashAccount38>,
    #[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_fnl_colltn_dt: Option<String>,
    #[serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none")]
    pub orgnl_frqcy: Option<Frequency36Choice>,
    #[serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none")]
    pub orgnl_rsn: Option<MandateSetupReason1Choice>,
    #[serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none")]
    pub orgnl_trckg_days: Option<String>,
}

impl AmendmentInformationDetails13 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_mndt_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_mndt_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_mndt_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.orgnl_cdtr_schme_id {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_cdtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_cdtr_agt_acct {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_dbtr_acct {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_dbtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_dbtr_agt_acct {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_frqcy {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_rsn {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_trckg_days {
            let pattern = Regex::new("[0-9]{2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_trckg_days does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// AmountType4Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountType4Choice {
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none")]
    pub eqvt_amt: Option<EquivalentAmount2>,
}

impl AmountType4Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.instd_amt {
            val.validate()?
        }
        if let Some(ref val) = self.eqvt_amt {
            val.validate()?
        }
        Ok(())
    }
}

// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData3>,
}

impl BranchAndFinancialInstitutionIdentification6 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        if let Some(ref val) = self.brnch_id {
            val.validate()?
        }
        Ok(())
    }
}

// BranchData3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchData3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
}

impl BranchData3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "id exceeds the maximum length of 35".to_string(),
                ));
            }
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
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        Ok(())
    }
}

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

// Case5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Case5 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Cretr")]
    pub cretr: Party40Choice,
    #[serde(rename = "ReopCaseIndctn", skip_serializing_if = "Option::is_none")]
    pub reop_case_indctn: Option<bool>,
}

impl Case5 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        self.cretr.validate()?;
        Ok(())
    }
}

// CaseAssignment5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CaseAssignment5 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Assgnr")]
    pub assgnr: Party40Choice,
    #[serde(rename = "Assgne")]
    pub assgne: Party40Choice,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
}

impl CaseAssignment5 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        self.assgnr.validate()?;
        self.assgne.validate()?;
        Ok(())
    }
}

// CashAccount38 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount38 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2Choice>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount38 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.id.validate()?;
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.ccy {
            let pattern = Regex::new("[A-Z]{3,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ccy does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "nm exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prxy {
            val.validate()?
        }
        Ok(())
    }
}

// CashAccountType2Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccountType2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CashAccountType2Choice {
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

// CategoryPurpose1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CategoryPurpose1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CategoryPurpose1Choice {
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

// ChargeBearerType1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChargeBearerType1Code {
    #[default]
    #[serde(rename = "DEBT")]
    CodeDEBT,
    #[serde(rename = "CRED")]
    CodeCRED,
    #[serde(rename = "SHAR")]
    CodeSHAR,
    #[serde(rename = "SLEV")]
    CodeSLEV,
}

impl ChargeBearerType1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// ChargeType3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargeType3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification3>,
}

impl ChargeType3Choice {
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
            val.validate()?
        }
        Ok(())
    }
}

// Charges6 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charges6 {
    #[serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
    pub rcrd: Option<Vec<ChargesRecord3>>,
}

impl Charges6 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.ttl_chrgs_and_tax_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.rcrd {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// Charges7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charges7 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification6,
}

impl Charges7 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        self.agt.validate()?;
        Ok(())
    }
}

// ChargesRecord3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargesRecord3 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none")]
    pub chrg_incl_ind: Option<bool>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ChargeType3Choice>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Br", skip_serializing_if = "Option::is_none")]
    pub br: Option<ChargeBearerType1Code>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges2>,
}

impl ChargesRecord3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate()?
        }
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.br {
            val.validate()?
        }
        if let Some(ref val) = self.agt {
            val.validate()?
        }
        if let Some(ref val) = self.tax {
            val.validate()?
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

// ClearingChannel2Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClearingChannel2Code {
    #[default]
    #[serde(rename = "RTGS")]
    CodeRTGS,
    #[serde(rename = "RTNS")]
    CodeRTNS,
    #[serde(rename = "MPNS")]
    CodeMPNS,
    #[serde(rename = "BOOK")]
    CodeBOOK,
}

impl ClearingChannel2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// ClearingSystemIdentification2Choice ...
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

// ClearingSystemIdentification3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ClearingSystemIdentification3Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 3 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 3".to_string(),
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

// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[serde(rename = "MmbId")]
    pub mmb_id: String,
}

impl ClearingSystemMemberIdentification2 {
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
        if self.mmb_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "mmb_id exceeds the maximum length of 35".to_string(),
            ));
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

// Contact4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<String>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<String>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<String>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<String>,
    #[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
    pub email_purp: Option<String>,
    #[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<String>,
    #[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<String>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<OtherContact1>>,
    #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}

impl Contact4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nm_prfx {
            val.validate()?
        }
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
        if let Some(ref val) = self.phne_nb {
            let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "phne_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.mob_nb {
            let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "mob_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.fax_nb {
            let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "fax_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.email_adr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "email_adr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 2048 {
                return Err(ValidationError::new(
                    1002,
                    "email_adr exceeds the maximum length of 2048".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.email_purp {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "email_purp is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "email_purp exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.job_titl {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "job_titl is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "job_titl exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rspnsblty {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "rspnsblty is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "rspnsblty exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.dept {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "dept is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "dept exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.othr {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.prefrd_mtd {
            val.validate()?
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

// CreditDebitCode ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CreditDebitCode {
    #[default]
    #[serde(rename = "CRDT")]
    CodeCRDT,
    #[serde(rename = "DBIT")]
    CodeDBIT,
}

impl CreditDebitCode {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// CreditorReferenceInformation2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceInformation2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditorReferenceType2>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.ref_attr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ref_attr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ref_attr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CreditorReferenceType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DocumentType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CreditorReferenceType1Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            val.validate()?
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

// CreditorReferenceType2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType2 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: CreditorReferenceType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl CreditorReferenceType2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.cd_or_prtry.validate()?;
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DateAndDateTime2Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndDateTime2Choice {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// DateAndPlaceOfBirth1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndPlaceOfBirth1 {
    #[serde(rename = "BirthDt")]
    pub birth_dt: String,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<String>,
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: String,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.prvc_of_birth {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prvc_of_birth is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prvc_of_birth exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if self.city_of_birth.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "city_of_birth is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.city_of_birth.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "city_of_birth exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[A-Z]{2,2}").unwrap();
        if !pattern.is_match(&self.ctry_of_birth) {
            return Err(ValidationError::new(
                1005,
                "ctry_of_birth does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// DatePeriod2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DatePeriod2 {
    #[serde(rename = "FrDt")]
    pub fr_dt: String,
    #[serde(rename = "ToDt")]
    pub to_dt: String,
}

impl DatePeriod2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// DiscountAmountAndType1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountAndType1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DiscountAmountType1Choice>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DiscountAmountAndType1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        self.amt.validate()?;
        Ok(())
    }
}

// DiscountAmountType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl DiscountAmountType1Choice {
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

// DocumentAdjustment1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentAdjustment1 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<String>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<String>,
}

impl DocumentAdjustment1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate()?
        }
        if let Some(ref val) = self.rsn {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "rsn is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "rsn exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.addtl_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_inf exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DocumentLineIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DocumentLineType1>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<String>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "nb exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DocumentLineInformation1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineInformation1 {
    #[serde(rename = "Id")]
    pub id: Vec<DocumentLineIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RemittanceAmount3>,
}

impl DocumentLineInformation1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        for item in &self.id {
            item.validate()?
        }
        if let Some(ref val) = self.desc {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "desc is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 2048 {
                return Err(ValidationError::new(
                    1002,
                    "desc exceeds the maximum length of 2048".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        Ok(())
    }
}

// DocumentLineType1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: DocumentLineType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl DocumentLineType1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.cd_or_prtry.validate()?;
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DocumentLineType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl DocumentLineType1Choice {
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

// DocumentType3Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum DocumentType3Code {
    #[default]
    #[serde(rename = "RADM")]
    CodeRADM,
    #[serde(rename = "RPIN")]
    CodeRPIN,
    #[serde(rename = "FXDR")]
    CodeFXDR,
    #[serde(rename = "DISP")]
    CodeDISP,
    #[serde(rename = "PUOR")]
    CodePUOR,
    #[serde(rename = "SCOR")]
    CodeSCOR,
}

impl DocumentType3Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// DocumentType6Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum DocumentType6Code {
    #[default]
    #[serde(rename = "MSIN")]
    CodeMSIN,
    #[serde(rename = "CNFA")]
    CodeCNFA,
    #[serde(rename = "DNFA")]
    CodeDNFA,
    #[serde(rename = "CINV")]
    CodeCINV,
    #[serde(rename = "CREN")]
    CodeCREN,
    #[serde(rename = "DEBN")]
    CodeDEBN,
    #[serde(rename = "HIRI")]
    CodeHIRI,
    #[serde(rename = "SBIN")]
    CodeSBIN,
    #[serde(rename = "CMCN")]
    CodeCMCN,
    #[serde(rename = "SOAC")]
    CodeSOAC,
    #[serde(rename = "DISP")]
    CodeDISP,
    #[serde(rename = "BOLD")]
    CodeBOLD,
    #[serde(rename = "VCHR")]
    CodeVCHR,
    #[serde(rename = "AROI")]
    CodeAROI,
    #[serde(rename = "TSUT")]
    CodeTSUT,
    #[serde(rename = "PUOR")]
    CodePUOR,
}

impl DocumentType6Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// EquivalentAmount2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EquivalentAmount2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyOfTrf")]
    pub ccy_of_trf: String,
}

impl EquivalentAmount2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        let pattern = Regex::new("[A-Z]{3,3}").unwrap();
        if !pattern.is_match(&self.ccy_of_trf) {
            return Err(ValidationError::new(
                1005,
                "ccy_of_trf does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl FinancialIdentificationSchemeName1Choice {
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

// FinancialInstitutionIdentification18 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification18 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification18 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.bicfi {
            let pattern =
                Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bicfi does not match the required pattern".to_string(),
                ));
            }
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
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        if let Some(ref val) = self.othr {
            val.validate()?
        }
        Ok(())
    }
}

// Frequency36Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Frequency36Choice {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Frequency6Code>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<FrequencyPeriod1>,
    #[serde(rename = "PtInTm", skip_serializing_if = "Option::is_none")]
    pub pt_in_tm: Option<FrequencyAndMoment1>,
}

impl Frequency36Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.prd {
            val.validate()?
        }
        if let Some(ref val) = self.pt_in_tm {
            val.validate()?
        }
        Ok(())
    }
}

// Frequency6Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Frequency6Code {
    #[default]
    #[serde(rename = "YEAR")]
    CodeYEAR,
    #[serde(rename = "MNTH")]
    CodeMNTH,
    #[serde(rename = "QURT")]
    CodeQURT,
    #[serde(rename = "MIAN")]
    CodeMIAN,
    #[serde(rename = "WEEK")]
    CodeWEEK,
    #[serde(rename = "DAIL")]
    CodeDAIL,
    #[serde(rename = "ADHO")]
    CodeADHO,
    #[serde(rename = "INDA")]
    CodeINDA,
    #[serde(rename = "FRTN")]
    CodeFRTN,
}

impl Frequency6Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// FrequencyAndMoment1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FrequencyAndMoment1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,
    #[serde(rename = "PtInTm")]
    pub pt_in_tm: String,
}

impl FrequencyAndMoment1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        let pattern = Regex::new("[0-9]{2}").unwrap();
        if !pattern.is_match(&self.pt_in_tm) {
            return Err(ValidationError::new(
                1005,
                "pt_in_tm does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// FrequencyPeriod1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FrequencyPeriod1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,
    #[serde(rename = "CntPerPrd")]
    pub cnt_per_prd: f64,
}

impl FrequencyPeriod1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        Ok(())
    }
}

// Garnishment3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Garnishment3 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType1,
    #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification135>,
    #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification135>,
    #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<String>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
    pub fmly_mdcl_insrnc_ind: Option<bool>,
    #[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
    pub mplyee_termntn_ind: Option<bool>,
}

impl Garnishment3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        if let Some(ref val) = self.grnshee {
            val.validate()?
        }
        if let Some(ref val) = self.grnshmt_admstr {
            val.validate()?
        }
        if let Some(ref val) = self.ref_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ref_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "ref_nb exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate()?
        }
        Ok(())
    }
}

// GarnishmentType1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: GarnishmentType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GarnishmentType1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.cd_or_prtry.validate()?;
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GarnishmentType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl GarnishmentType1Choice {
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

// GenericAccountIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericAccountIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericAccountIdentification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 34 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 34".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericFinancialIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericFinancialIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericFinancialIdentification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericIdentification3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification3 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericIdentification3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericIdentification30 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Issr")]
    pub issr: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<String>,
}

impl GenericIdentification30 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
            ));
        }
        if self.issr.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "issr is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.issr.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "issr exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "schme_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "schme_nm exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericOrganisationIdentification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericPersonIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericPersonIdentification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
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

// LocalInstrument2Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalInstrument2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl LocalInstrument2Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 35".to_string(),
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

// MandateRelatedInformation14 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MandateRelatedInformation14 {
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<String>,
    #[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr: Option<String>,
    #[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind: Option<bool>,
    #[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
    pub amdmnt_inf_dtls: Option<AmendmentInformationDetails13>,
    #[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr: Option<String>,
    #[serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none")]
    pub frst_colltn_dt: Option<String>,
    #[serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none")]
    pub fnl_colltn_dt: Option<String>,
    #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<Frequency36Choice>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<MandateSetupReason1Choice>,
    #[serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none")]
    pub trckg_days: Option<String>,
}

impl MandateRelatedInformation14 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.mndt_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "mndt_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "mndt_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.amdmnt_inf_dtls {
            val.validate()?
        }
        if let Some(ref val) = self.elctrnc_sgntr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "elctrnc_sgntr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 1025 {
                return Err(ValidationError::new(
                    1002,
                    "elctrnc_sgntr exceeds the maximum length of 1025".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.frqcy {
            val.validate()?
        }
        if let Some(ref val) = self.rsn {
            val.validate()?
        }
        if let Some(ref val) = self.trckg_days {
            let pattern = Regex::new("[0-9]{2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "trckg_days does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// MandateSetupReason1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MandateSetupReason1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl MandateSetupReason1Choice {
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
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "prtry exceeds the maximum length of 70".to_string(),
                ));
            }
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

// NamePrefix2Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum NamePrefix2Code {
    #[default]
    #[serde(rename = "DOCT")]
    CodeDOCT,
    #[serde(rename = "MADM")]
    CodeMADM,
    #[serde(rename = "MISS")]
    CodeMISS,
    #[serde(rename = "MIST")]
    CodeMIST,
    #[serde(rename = "MIKS")]
    CodeMIKS,
}

impl NamePrefix2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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

// OrganisationIdentification29 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl OrganisationIdentification29 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.any_bic {
            let pattern =
                Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "any_bic does not match the required pattern".to_string(),
                ));
            }
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
        if let Some(ref vec) = self.othr {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice {
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

// OriginalGroupInformation29 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalGroupInformation29 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: String,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<String>,
}

impl OriginalGroupInformation29 {
    pub fn validate(&self) -> Result<(), ValidationError> {
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

// OriginalTransactionReference28 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalTransactionReference28 {
    #[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<AmountType4Choice>,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
    pub reqd_colltn_dt: Option<String>,
    #[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
    pub cdtr_schme_id: Option<PartyIdentification135>,
    #[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
    pub sttlm_inf: Option<SettlementInstruction7>,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation27>,
    #[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
    pub pmt_mtd: Option<PaymentMethod4Code>,
    #[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf: Option<MandateRelatedInformation14>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation16>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<Party40Choice>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice>,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount38>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<Party40Choice>,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount38>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<Party40Choice>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice>,
}

impl OriginalTransactionReference28 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.intr_bk_sttlm_amt {
            val.validate()?
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        if let Some(ref val) = self.reqd_exctn_dt {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr_schme_id {
            val.validate()?
        }
        if let Some(ref val) = self.sttlm_inf {
            val.validate()?
        }
        if let Some(ref val) = self.pmt_tp_inf {
            val.validate()?
        }
        if let Some(ref val) = self.pmt_mtd {
            val.validate()?
        }
        if let Some(ref val) = self.mndt_rltd_inf {
            val.validate()?
        }
        if let Some(ref val) = self.rmt_inf {
            val.validate()?
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_acct {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_agt_acct {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr_agt_acct {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr_acct {
            val.validate()?
        }
        if let Some(ref val) = self.ultmt_cdtr {
            val.validate()?
        }
        if let Some(ref val) = self.purp {
            val.validate()?
        }
        Ok(())
    }
}

// OtherContact1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    pub chanl_tp: String,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl OtherContact1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.chanl_tp.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "chanl_tp is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.chanl_tp.chars().count() > 4 {
            return Err(ValidationError::new(
                1002,
                "chanl_tp exceeds the maximum length of 4".to_string(),
            ));
        }
        if let Some(ref val) = self.id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 128 {
                return Err(ValidationError::new(
                    1002,
                    "id exceeds the maximum length of 128".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// Party38Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification29>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification13>,
}

impl Party38Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.org_id {
            val.validate()?
        }
        if let Some(ref val) = self.prvt_id {
            val.validate()?
        }
        Ok(())
    }
}

// Party40Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party40Choice {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification135>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl Party40Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.pty {
            val.validate()?
        }
        if let Some(ref val) = self.agt {
            val.validate()?
        }
        Ok(())
    }
}

// PartyIdentification135 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification135 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
}

impl PartyIdentification135 {
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
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        if let Some(ref val) = self.id {
            val.validate()?
        }
        if let Some(ref val) = self.ctry_of_res {
            let pattern = Regex::new("[A-Z]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry_of_res does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PaymentMethod4Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PaymentMethod4Code {
    #[default]
    #[serde(rename = "CHK")]
    CodeCHK,
    #[serde(rename = "TRF")]
    CodeTRF,
    #[serde(rename = "DD")]
    CodeDD,
    #[serde(rename = "TRA")]
    CodeTRA,
}

impl PaymentMethod4Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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

// PaymentTypeInformation27 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTypeInformation27 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<ClearingChannel2Code>,
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,
    #[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
    pub seq_tp: Option<SequenceType3Code>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation27 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.instr_prty {
            val.validate()?
        }
        if let Some(ref val) = self.clr_chanl {
            val.validate()?
        }
        if let Some(ref vec) = self.svc_lvl {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.lcl_instrm {
            val.validate()?
        }
        if let Some(ref val) = self.seq_tp {
            val.validate()?
        }
        if let Some(ref val) = self.ctgy_purp {
            val.validate()?
        }
        Ok(())
    }
}

// PersonIdentification13 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl PersonIdentification13 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dt_and_plc_of_birth {
            val.validate()?
        }
        if let Some(ref vec) = self.othr {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice {
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

// PostalAddress24 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress24 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType3Choice>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<String>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<String>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<String>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<String>,
    #[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<String>,
    #[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
    pub flr: Option<String>,
    #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<String>,
    #[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<String>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<String>,
    #[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<String>,
    #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<String>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<String>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<String>,
    #[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
    pub adr_line: Option<Vec<String>>,
}

impl PostalAddress24 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.adr_tp {
            val.validate()?
        }
        if let Some(ref val) = self.dept {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "dept is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "dept exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.sub_dept {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sub_dept is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "sub_dept exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.strt_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "strt_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "strt_nm exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.bldg_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "bldg_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "bldg_nb exceeds the maximum length of 16".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.bldg_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "bldg_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "bldg_nm exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.flr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "flr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "flr exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pst_bx {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "pst_bx is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "pst_bx exceeds the maximum length of 16".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.room {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "room is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "room exceeds the maximum length of 70".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pst_cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "pst_cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "pst_cd exceeds the maximum length of 16".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.twn_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "twn_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "twn_nm exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.twn_lctn_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "twn_lctn_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "twn_lctn_nm exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.dstrct_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "dstrct_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "dstrct_nm exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ctry_sub_dvsn is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ctry_sub_dvsn exceeds the maximum length of 35".to_string(),
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
        if let Some(ref vec) = self.adr_line {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "adr_line is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 70 {
                    return Err(ValidationError::new(
                        1002,
                        "adr_line exceeds the maximum length of 70".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// PreferredContactMethod1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PreferredContactMethod1Code {
    #[default]
    #[serde(rename = "LETT")]
    CodeLETT,
    #[serde(rename = "MAIL")]
    CodeMAIL,
    #[serde(rename = "PHON")]
    CodePHON,
    #[serde(rename = "FAXX")]
    CodeFAXX,
    #[serde(rename = "CELL")]
    CodeCELL,
}

impl PreferredContactMethod1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// Priority2Code ...
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

// ProxyAccountIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProxyAccountType1Choice>,
    #[serde(rename = "Id")]
    pub id: String,
}

impl ProxyAccountIdentification1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
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

// ProxyAccountType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ProxyAccountType1Choice {
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

// Purpose2Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Purpose2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Purpose2Choice {
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

// ReferredDocumentInformation7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentInformation7 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ReferredDocumentType4>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<String>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<String>,
    #[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
    pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}

impl ReferredDocumentInformation7 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "nb exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.line_dtls {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// ReferredDocumentType3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DocumentType6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ReferredDocumentType3Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            val.validate()?
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

// ReferredDocumentType4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType4 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: ReferredDocumentType3Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl ReferredDocumentType4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.cd_or_prtry.validate()?;
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// RemittanceAmount2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount2 {
    #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
    #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<Vec<TaxAmountAndType1>>,
    #[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.due_pybl_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.dscnt_apld_amt {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.cdt_note_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.tax_amt {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.adjstmnt_amt_and_rsn {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate()?
        }
        Ok(())
    }
}

// RemittanceAmount3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount3 {
    #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
    #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<Vec<TaxAmountAndType1>>,
    #[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.due_pybl_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.dscnt_apld_amt {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.cdt_note_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.tax_amt {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.adjstmnt_amt_and_rsn {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate()?
        }
        Ok(())
    }
}

// RemittanceInformation16 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceInformation16 {
    #[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
    pub ustrd: Option<Vec<String>>,
    #[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
    pub strd: Option<Vec<StructuredRemittanceInformation16>>,
}

impl RemittanceInformation16 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref vec) = self.ustrd {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "ustrd is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 140 {
                    return Err(ValidationError::new(
                        1002,
                        "ustrd exceeds the maximum length of 140".to_string(),
                    ));
                }
            }
        }
        if let Some(ref vec) = self.strd {
            for item in vec {
                item.validate()?
            }
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

// SequenceType3Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SequenceType3Code {
    #[default]
    #[serde(rename = "FRST")]
    CodeFRST,
    #[serde(rename = "RCUR")]
    CodeRCUR,
    #[serde(rename = "FNAL")]
    CodeFNAL,
    #[serde(rename = "OOFF")]
    CodeOOFF,
    #[serde(rename = "RPRE")]
    CodeRPRE,
}

impl SequenceType3Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// ServiceLevel8Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServiceLevel8Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ServiceLevel8Choice {
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

// SettlementInstruction7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementInstruction7 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: SettlementMethod1Code,
    #[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
    pub sttlm_acct: Option<CashAccount38>,
    #[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
    pub clr_sys: Option<ClearingSystemIdentification3Choice>,
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
    #[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(
        rename = "ThrdRmbrsmntAgtAcct",
        skip_serializing_if = "Option::is_none"
    )]
    pub thrd_rmbrsmnt_agt_acct: Option<CashAccount38>,
}

impl SettlementInstruction7 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.sttlm_mtd.validate()?;
        if let Some(ref val) = self.sttlm_acct {
            val.validate()?
        }
        if let Some(ref val) = self.clr_sys {
            val.validate()?
        }
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
        if let Some(ref val) = self.thrd_rmbrsmnt_agt {
            val.validate()?
        }
        if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct {
            val.validate()?
        }
        Ok(())
    }
}

// SettlementMethod1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SettlementMethod1Code {
    #[default]
    #[serde(rename = "INDA")]
    CodeINDA,
    #[serde(rename = "INGA")]
    CodeINGA,
    #[serde(rename = "COVE")]
    CodeCOVE,
    #[serde(rename = "CLRG")]
    CodeCLRG,
}

impl SettlementMethod1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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

// StructuredRemittanceInformation16 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRemittanceInformation16 {
    #[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
    #[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<RemittanceAmount2>,
    #[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
    #[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
    pub invcr: Option<PartyIdentification135>,
    #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification135>,
    #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxInformation7>,
    #[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt: Option<Garnishment3>,
    #[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rmt_inf: Option<Vec<String>>,
}

impl StructuredRemittanceInformation16 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref vec) = self.rfrd_doc_inf {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.rfrd_doc_amt {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr_ref_inf {
            val.validate()?
        }
        if let Some(ref val) = self.invcr {
            val.validate()?
        }
        if let Some(ref val) = self.invcee {
            val.validate()?
        }
        if let Some(ref val) = self.tax_rmt {
            val.validate()?
        }
        if let Some(ref val) = self.grnshmt_rmt {
            val.validate()?
        }
        if let Some(ref vec) = self.addtl_rmt_inf {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "addtl_rmt_inf is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 140 {
                    return Err(ValidationError::new(
                        1002,
                        "addtl_rmt_inf exceeds the maximum length of 140".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// SupplementaryData1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SupplementaryData1 {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<String>,
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.plc_and_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "plc_and_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 350 {
                return Err(ValidationError::new(
                    1002,
                    "plc_and_nm exceeds the maximum length of 350".to_string(),
                ));
            }
        }
        self.envlp.validate()?;
        Ok(())
    }
}

// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SupplementaryDataEnvelope1 {}

impl SupplementaryDataEnvelope1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// TaxAmount2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmount2 {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
    pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
    pub dtls: Option<Vec<TaxRecordDetails2>>,
}

impl TaxAmount2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.taxbl_base_amt {
            val.validate()?
        }
        if let Some(ref val) = self.ttl_amt {
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

// TaxAmountAndType1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmountAndType1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxAmountType1Choice>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxAmountAndType1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        self.amt.validate()?;
        Ok(())
    }
}

// TaxAmountType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmountType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl TaxAmountType1Choice {
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

// TaxAuthorisation1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAuthorisation1 {
    #[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
    pub titl: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl TaxAuthorisation1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.titl {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "titl is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "titl exceeds the maximum length of 35".to_string(),
                ));
            }
        }
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
        Ok(())
    }
}

// TaxCharges2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxCharges2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TaxCharges2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        Ok(())
    }
}

// TaxInformation7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxInformation7 {
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<TaxParty1>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<TaxParty2>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<TaxParty2>,
    #[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
    pub admstn_zone: Option<String>,
    #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<String>,
    #[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
    pub mtd: Option<String>,
    #[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<f64>,
    #[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
    pub rcrd: Option<Vec<TaxRecord2>>,
}

impl TaxInformation7 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cdtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.admstn_zone {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "admstn_zone is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "admstn_zone exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ref_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ref_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "ref_nb exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.mtd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "mtd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "mtd exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ttl_taxbl_base_amt {
            val.validate()?
        }
        if let Some(ref val) = self.ttl_tax_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.rcrd {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TaxParty1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty1 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
}

impl TaxParty1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tax_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tax_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tax_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.regn_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "regn_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "regn_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tax_tp {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tax_tp is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tax_tp exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxParty2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty2 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
    #[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
    pub authstn: Option<TaxAuthorisation1>,
}

impl TaxParty2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tax_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tax_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tax_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.regn_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "regn_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "regn_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tax_tp {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tax_tp is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "tax_tp exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.authstn {
            val.validate()?
        }
        Ok(())
    }
}

// TaxPeriod2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxPeriod2 {
    #[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
    pub yr: Option<String>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxRecordPeriod1Code>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate()?
        }
        Ok(())
    }
}

// TaxRecord2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxRecord2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<String>,
    #[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
    pub ctgy: Option<String>,
    #[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
    pub ctgy_dtls: Option<String>,
    #[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
    pub dbtr_sts: Option<String>,
    #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<String>,
    #[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
    pub frms_cd: Option<String>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<TaxPeriod2>,
    #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<TaxAmount2>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<String>,
}

impl TaxRecord2 {
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
        if let Some(ref val) = self.ctgy {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ctgy is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ctgy exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctgy_dtls {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ctgy_dtls is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ctgy_dtls exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.dbtr_sts {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "dbtr_sts is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "dbtr_sts exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.cert_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cert_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "cert_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.frms_cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "frms_cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "frms_cd exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prd {
            val.validate()?
        }
        if let Some(ref val) = self.tax_amt {
            val.validate()?
        }
        if let Some(ref val) = self.addtl_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_inf exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxRecordDetails2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxRecordDetails2 {
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<TaxPeriod2>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.prd {
            val.validate()?
        }
        self.amt.validate()?;
        Ok(())
    }
}

// TaxRecordPeriod1Code ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaxRecordPeriod1Code {
    #[default]
    #[serde(rename = "MM01")]
    CodeMM01,
    #[serde(rename = "MM02")]
    CodeMM02,
    #[serde(rename = "MM03")]
    CodeMM03,
    #[serde(rename = "MM04")]
    CodeMM04,
    #[serde(rename = "MM05")]
    CodeMM05,
    #[serde(rename = "MM06")]
    CodeMM06,
    #[serde(rename = "MM07")]
    CodeMM07,
    #[serde(rename = "MM08")]
    CodeMM08,
    #[serde(rename = "MM09")]
    CodeMM09,
    #[serde(rename = "MM10")]
    CodeMM10,
    #[serde(rename = "MM11")]
    CodeMM11,
    #[serde(rename = "MM12")]
    CodeMM12,
    #[serde(rename = "QTR1")]
    CodeQTR1,
    #[serde(rename = "QTR2")]
    CodeQTR2,
    #[serde(rename = "QTR3")]
    CodeQTR3,
    #[serde(rename = "QTR4")]
    CodeQTR4,
    #[serde(rename = "HLF1")]
    CodeHLF1,
    #[serde(rename = "HLF2")]
    CodeHLF2,
}

impl TaxRecordPeriod1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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
