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

// AccountIdentification4Choice1: Unique identification of an account, as assigned by the account servicer, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountIdentification4Choice1 {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification11>,
}

impl AccountIdentification4Choice1 {
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

// AccountReportingRequestV05: Set of elements used to provide further details on the reporting request.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountReportingRequestV05 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader771,
    #[serde(rename = "RptgReq")]
    pub rptg_req: Vec<ReportingRequest51>,
}

impl AccountReportingRequestV05 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        for item in &self.rptg_req {
            item.validate()?
        }
        Ok(())
    }
}

// AccountSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountSchemeName1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl AccountSchemeName1Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ActiveOrHistoricCurrencyAndAmount: A number of monetary units specified in an active or a historic currency where the unit of currency is explicit and compliant with ISO 4217.
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

// AddressType2Code: Address is the address to which delivery is to take place.
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

// AddressType3Choice1: Type of address expressed as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddressType3Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification301>,
}

impl AddressType3Choice1 {
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

// BalanceSubType1Choice1: Specifies a proprietary code for the balance type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BalanceSubType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl BalanceSubType1Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// BalanceType10Choice1: Balance type, in a proprietary format.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BalanceType10Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl BalanceType10Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// BalanceType131: Specifies the balance sub-type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BalanceType131 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: BalanceType10Choice1,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<BalanceSubType1Choice1>,
}

impl BalanceType131 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.cd_or_prtry.validate()?;
        if let Some(ref val) = self.sub_tp {
            val.validate()?
        }
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

// CashAccount381: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount381 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice1,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2Choice1>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAccountIdentification11>,
}

impl CashAccount381 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prxy {
            val.validate()?
        }
        Ok(())
    }
}

// CashAccountType2Choice1: Nature or use of the account in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccountType2Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CashAccountType2Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ClearingSystemIdentification2Choice1: Identification of a clearing system, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification2Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl ClearingSystemIdentification2Choice1 {
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
        Ok(())
    }
}

// ClearingSystemMemberIdentification21: Identification of a member of a clearing system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemMemberIdentification21 {
    #[serde(rename = "ClrSysId")]
    pub clr_sys_id: ClearingSystemIdentification2Choice1,
    #[serde(rename = "MmbId")]
    pub mmb_id: String,
}

impl ClearingSystemMemberIdentification21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.clr_sys_id.validate()?;
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

// CreditDebitCode: Operation is a decrease.
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

// DateAndPlaceOfBirth11: Country where a person was born.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndPlaceOfBirth11 {
    #[serde(rename = "BirthDt")]
    pub birth_dt: String,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<String>,
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: String,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prvc_of_birth does not match the required pattern".to_string(),
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
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
        if !pattern.is_match(&self.city_of_birth) {
            return Err(ValidationError::new(
                1005,
                "city_of_birth does not match the required pattern".to_string(),
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

// DatePeriodDetails1: End date of the range.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DatePeriodDetails1 {
    #[serde(rename = "FrDt")]
    pub fr_dt: String,
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<String>,
}

impl DatePeriodDetails1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// EntryStatus1Choice1: Entry status, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryStatus1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl EntryStatus1Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// FinancialInstitutionIdentification181: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification181 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
}

impl FinancialInstitutionIdentification181 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        Ok(())
    }
}

// FloorLimitType1Code: Floor limit applies to both credit and debit entries.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum FloorLimitType1Code {
    #[default]
    #[serde(rename = "CRED")]
    CodeCRED,
    #[serde(rename = "DEBT")]
    CodeDEBT,
    #[serde(rename = "BOTH")]
    CodeBOTH,
}

impl FloorLimitType1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// GenericAccountIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericAccountIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice1>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericAccountIdentification11 {
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
        let pattern = Regex::new("([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]*(/[0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ])?)*)").unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "issr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericIdentification301: Short textual description of the scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification301 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Issr")]
    pub issr: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<String>,
}

impl GenericIdentification301 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.issr) {
            return Err(ValidationError::new(
                1005,
                "issr does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "schme_nm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericOrganisationIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice1>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericOrganisationIdentification11 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "issr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericPersonIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice1>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericPersonIdentification11 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "issr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GroupHeader771: Identification of the party that is sending the message, when different from the account owner.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader771 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "MsgSndr", skip_serializing_if = "Option::is_none")]
    pub msg_sndr: Option<Party40Choice1>,
}

impl GroupHeader771 {
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
        if let Some(ref val) = self.msg_sndr {
            val.validate()?
        }
        Ok(())
    }
}

// Limit2: Indicates whether the floor limit applies to credit, to debit or to both credit and debit entries.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Limit2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: FloorLimitType1Code,
}

impl Limit2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        Ok(())
    }
}

// OrganisationIdentification291: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification291 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericOrganisationIdentification11>>,
}

impl OrganisationIdentification291 {
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

// OrganisationIdentificationSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// Party38Choice1: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice1 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification291>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification131>,
}

impl Party38Choice1 {
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

// Party40Choice1: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party40Choice1 {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification1351>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Party40Choice1 {
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

// PartyIdentification1351: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1351 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl PartyIdentification1351 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nm does not match the required pattern".to_string(),
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
        Ok(())
    }
}

// PersonIdentification131: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification131 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification11>>,
}

impl PersonIdentification131 {
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

// PersonIdentificationSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// PostalAddress241: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress241 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType3Choice1>,
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

impl PostalAddress241 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dept does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sub_dept does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "strt_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bldg_nb does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bldg_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "flr does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pst_bx does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "room does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pst_cd does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "twn_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "twn_lctn_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dstrct_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry_sub_dvsn does not match the required pattern".to_string(),
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
                let pattern = Regex::new(
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                )
                .unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "adr_line does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// PostalAddress242: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress242 {
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

impl PostalAddress242 {
    pub fn validate(&self) -> Result<(), ValidationError> {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dept does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sub_dept does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "strt_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bldg_nb does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bldg_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "flr does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pst_bx does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "room does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pst_cd does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "twn_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "twn_lctn_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dstrct_nm does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry_sub_dvsn does not match the required pattern".to_string(),
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
                let pattern = Regex::new(
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                )
                .unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "adr_line does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// ProxyAccountIdentification11: Identification used to indicate the account identification under another specified name.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountIdentification11 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProxyAccountType1Choice1>,
    #[serde(rename = "Id")]
    pub id: String,
}

impl ProxyAccountIdentification11 {
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
        if self.id.chars().count() > 320 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 320".to_string(),
            ));
        }
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// ProxyAccountType1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ProxyAccountType1Choice1 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// QueryType3Code: Specifies that the query requests that only items that have changed since the last query be returned.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum QueryType3Code {
    #[default]
    #[serde(rename = "ALLL")]
    CodeALLL,
    #[serde(rename = "CHNG")]
    CodeCHNG,
    #[serde(rename = "MODF")]
    CodeMODF,
}

impl QueryType3Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// ReportingPeriod21: Specifies whether all matching items need to be reported or only those items that are new or have changed since the last similar request was made.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReportingPeriod21 {
    #[serde(rename = "FrToDt")]
    pub fr_to_dt: DatePeriodDetails1,
    #[serde(rename = "FrToTm", skip_serializing_if = "Option::is_none")]
    pub fr_to_tm: Option<TimePeriodDetails11>,
    #[serde(rename = "Tp")]
    pub tp: QueryType3Code,
}

impl ReportingPeriod21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fr_to_dt.validate()?;
        if let Some(ref val) = self.fr_to_tm {
            val.validate()?
        }
        self.tp.validate()?;
        Ok(())
    }
}

// ReportingRequest51: Provides details on the requested balance reporting.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReportingRequest51 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ReqdMsgNmId")]
    pub reqd_msg_nm_id: String,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<CashAccount381>,
    #[serde(rename = "AcctOwnr")]
    pub acct_ownr: Party40Choice1,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none")]
    pub rptg_prd: Option<ReportingPeriod21>,
    #[serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none")]
    pub rptg_seq: Option<SequenceRange1Choice1>,
    #[serde(rename = "ReqdTxTp", skip_serializing_if = "Option::is_none")]
    pub reqd_tx_tp: Option<TransactionType21>,
    #[serde(rename = "ReqdBalTp", skip_serializing_if = "Option::is_none")]
    pub reqd_bal_tp: Option<Vec<BalanceType131>>,
}

impl ReportingRequest51 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "id does not match the required pattern".to_string(),
                ));
            }
        }
        if self.reqd_msg_nm_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "reqd_msg_nm_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.reqd_msg_nm_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "reqd_msg_nm_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.reqd_msg_nm_id) {
            return Err(ValidationError::new(
                1005,
                "reqd_msg_nm_id does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.acct {
            val.validate()?
        }
        self.acct_ownr.validate()?;
        if let Some(ref val) = self.acct_svcr {
            val.validate()?
        }
        if let Some(ref val) = self.rptg_prd {
            val.validate()?
        }
        if let Some(ref val) = self.rptg_seq {
            val.validate()?
        }
        if let Some(ref val) = self.reqd_tx_tp {
            val.validate()?
        }
        if let Some(ref vec) = self.reqd_bal_tp {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// SequenceRange1Choice1: Specified sequence to be excluded.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SequenceRange1Choice1 {
    #[serde(rename = "FrSeq", skip_serializing_if = "Option::is_none")]
    pub fr_seq: Option<String>,
    #[serde(rename = "ToSeq", skip_serializing_if = "Option::is_none")]
    pub to_seq: Option<String>,
    #[serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none")]
    pub fr_to_seq: Option<Vec<SequenceRange11>>,
    #[serde(rename = "EQSeq", skip_serializing_if = "Option::is_none")]
    pub eq_seq: Option<Vec<String>>,
    #[serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none")]
    pub neq_seq: Option<Vec<String>>,
}

impl SequenceRange1Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.fr_seq {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "fr_seq is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "fr_seq exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "fr_seq does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.to_seq {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "to_seq is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "to_seq exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "to_seq does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.fr_to_seq {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.eq_seq {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "eq_seq is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 35 {
                    return Err(ValidationError::new(
                        1002,
                        "eq_seq exceeds the maximum length of 35".to_string(),
                    ));
                }
                let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "eq_seq does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        if let Some(ref vec) = self.neq_seq {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "neq_seq is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 35 {
                    return Err(ValidationError::new(
                        1002,
                        "neq_seq exceeds the maximum length of 35".to_string(),
                    ));
                }
                let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "neq_seq does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// SequenceRange11: End sequence of the range.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SequenceRange11 {
    #[serde(rename = "FrSeq")]
    pub fr_seq: String,
    #[serde(rename = "ToSeq")]
    pub to_seq: String,
}

impl SequenceRange11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.fr_seq.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "fr_seq is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.fr_seq.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "fr_seq exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.fr_seq) {
            return Err(ValidationError::new(
                1005,
                "fr_seq does not match the required pattern".to_string(),
            ));
        }
        if self.to_seq.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "to_seq is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.to_seq.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "to_seq exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.to_seq) {
            return Err(ValidationError::new(
                1005,
                "to_seq does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// TimePeriodDetails11: Time at which the time span ends.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TimePeriodDetails11 {
    #[serde(rename = "FrTm")]
    pub fr_tm: String,
    #[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
    pub to_tm: Option<String>,
}

impl TimePeriodDetails11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.fr_tm) {
            return Err(ValidationError::new(
                1005,
                "fr_tm does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.to_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "to_tm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TransactionType21: Specifies the minimum value of entries to be reported in the requested message.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionType21 {
    #[serde(rename = "Sts")]
    pub sts: EntryStatus1Choice1,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "FlrLmt", skip_serializing_if = "Option::is_none")]
    pub flr_lmt: Option<Vec<Limit2>>,
}

impl TransactionType21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.sts.validate()?;
        self.cdt_dbt_ind.validate()?;
        if let Some(ref vec) = self.flr_lmt {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}
