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

// AccountInterest41: Provides details on the tax applied to charges.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountInterest41 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestType1Choice1>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<Vec<Rate41>>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod11>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<String>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges21>,
}

impl AccountInterest41 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref vec) = self.rate {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate()?
        }
        if let Some(ref val) = self.rsn {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "rsn is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "rsn exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "rsn does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tax {
            val.validate()?
        }
        Ok(())
    }
}

// AccountReport251: Further details of the account report.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountReport251 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "RptPgntn")]
    pub rpt_pgntn: Pagination1,
    #[serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none")]
    pub elctrnc_seq_nb: Option<f64>,
    #[serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none")]
    pub rptg_seq: Option<SequenceRange1Choice1>,
    #[serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none")]
    pub lgl_seq_nb: Option<f64>,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<String>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod11>,
    #[serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
    #[serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none")]
    pub rptg_src: Option<ReportingSource1Choice1>,
    #[serde(rename = "Acct")]
    pub acct: CashAccount391,
    #[serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none")]
    pub rltd_acct: Option<CashAccount381>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<Vec<AccountInterest41>>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<Vec<CashBalance81>>,
    #[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
    pub txs_summry: Option<TotalTransactions61>,
    #[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
    pub ntry: Option<Vec<ReportEntry101>>,
    #[serde(rename = "AddtlRptInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rpt_inf: Option<String>,
}

impl AccountReport251 {
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
        self.rpt_pgntn.validate()?;
        if let Some(ref val) = self.rptg_seq {
            val.validate()?
        }
        if let Some(ref val) = self.cre_dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "cre_dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate()?
        }
        if let Some(ref val) = self.cpy_dplct_ind {
            val.validate()?
        }
        if let Some(ref val) = self.rptg_src {
            val.validate()?
        }
        self.acct.validate()?;
        if let Some(ref val) = self.rltd_acct {
            val.validate()?
        }
        if let Some(ref vec) = self.intrst {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.bal {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.txs_summry {
            val.validate()?
        }
        if let Some(ref vec) = self.ntry {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.addtl_rpt_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_rpt_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 500 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_rpt_inf exceeds the maximum length of 500".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_rpt_inf does not match the required pattern".to_string(),
                ));
            }
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

// ActiveOrHistoricCurrencyAndAmountRange2: Medium of exchange of value, used to qualify an amount.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAmountRange1Choice,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Ccy")]
    pub ccy: String,
}

impl ActiveOrHistoricCurrencyAndAmountRange2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate()?
        }
        let pattern = Regex::new("[A-Z]{3,3}").unwrap();
        if !pattern.is_match(&self.ccy) {
            return Err(ValidationError::new(
                1005,
                "ccy does not match the required pattern".to_string(),
            ));
        }
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

// AmountAndCurrencyExchange31: Set of elements used to provide information on the original amount and currency exchange.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchange31 {
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<AmountAndCurrencyExchangeDetails31>,
    #[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<AmountAndCurrencyExchangeDetails31>,
    #[serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none")]
    pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails31>,
    #[serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none")]
    pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails31>,
    #[serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none")]
    pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails41>>,
}

impl AmountAndCurrencyExchange31 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.instd_amt {
            val.validate()?
        }
        if let Some(ref val) = self.tx_amt {
            val.validate()?
        }
        if let Some(ref val) = self.cntr_val_amt {
            val.validate()?
        }
        if let Some(ref val) = self.anncd_pstng_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.prtry_amt {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// AmountAndCurrencyExchange32: Set of elements used to provide information on the original amount and currency exchange.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchange32 {
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<AmountAndCurrencyExchangeDetails32>,
    #[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<AmountAndCurrencyExchangeDetails32>,
    #[serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none")]
    pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails32>,
    #[serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none")]
    pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails32>,
    #[serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none")]
    pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails42>>,
}

impl AmountAndCurrencyExchange32 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.instd_amt {
            val.validate()?
        }
        if let Some(ref val) = self.tx_amt {
            val.validate()?
        }
        if let Some(ref val) = self.cntr_val_amt {
            val.validate()?
        }
        if let Some(ref val) = self.anncd_pstng_amt {
            val.validate()?
        }
        if let Some(ref vec) = self.prtry_amt {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// AmountAndCurrencyExchangeDetails31: Set of elements used to provide details on the currency exchange.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchangeDetails31 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange51>,
}

impl AmountAndCurrencyExchangeDetails31 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        if let Some(ref val) = self.ccy_xchg {
            val.validate()?
        }
        Ok(())
    }
}

// AmountAndCurrencyExchangeDetails32: Set of elements used to provide details on the currency exchange.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchangeDetails32 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange52>,
}

impl AmountAndCurrencyExchangeDetails32 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        if let Some(ref val) = self.ccy_xchg {
            val.validate()?
        }
        Ok(())
    }
}

// AmountAndCurrencyExchangeDetails41: Set of elements used to provide details on the currency exchange.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchangeDetails41 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange51>,
}

impl AmountAndCurrencyExchangeDetails41 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        self.amt.validate()?;
        if let Some(ref val) = self.ccy_xchg {
            val.validate()?
        }
        Ok(())
    }
}

// AmountAndCurrencyExchangeDetails42: Set of elements used to provide details on the currency exchange.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchangeDetails42 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange52>,
}

impl AmountAndCurrencyExchangeDetails42 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        self.amt.validate()?;
        if let Some(ref val) = self.ccy_xchg {
            val.validate()?
        }
        Ok(())
    }
}

// AmountAndDirection35: Indicates whether the amount is a credit or a debit amount.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndDirection35 {
    #[serde(rename = "Amt")]
    pub amt: f64,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
}

impl AmountAndDirection35 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.amt < 0.000000 {
            return Err(ValidationError::new(
                1003,
                "amt is less than the minimum value of 0.000000".to_string(),
            ));
        }
        self.cdt_dbt_ind.validate()?;
        Ok(())
    }
}

// AmountRangeBoundary1: Indicates whether the boundary amount is included in the range of amount values.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountRangeBoundary1 {
    #[serde(rename = "BdryAmt")]
    pub bdry_amt: f64,
    #[serde(rename = "Incl")]
    pub incl: bool,
}

impl AmountRangeBoundary1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.bdry_amt < 0.000000 {
            return Err(ValidationError::new(
                1003,
                "bdry_amt is less than the minimum value of 0.000000".to_string(),
            ));
        }
        Ok(())
    }
}

// AttendanceContext1Code: Unattended payment, no attendant present.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum AttendanceContext1Code {
    #[default]
    #[serde(rename = "ATTD")]
    CodeATTD,
    #[serde(rename = "SATT")]
    CodeSATT,
    #[serde(rename = "UATT")]
    CodeUATT,
}

impl AttendanceContext1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// AuthenticationEntity1Code: Merchant (for example signature verification by the attendant).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum AuthenticationEntity1Code {
    #[default]
    #[serde(rename = "ICCD")]
    CodeICCD,
    #[serde(rename = "AGNT")]
    CodeAGNT,
    #[serde(rename = "MERC")]
    CodeMERC,
}

impl AuthenticationEntity1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// AuthenticationMethod1Code: Channel-encrypted transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum AuthenticationMethod1Code {
    #[default]
    #[serde(rename = "UKNW")]
    CodeUKNW,
    #[serde(rename = "BYPS")]
    CodeBYPS,
    #[serde(rename = "NPIN")]
    CodeNPIN,
    #[serde(rename = "FPIN")]
    CodeFPIN,
    #[serde(rename = "CPSG")]
    CodeCPSG,
    #[serde(rename = "PPSG")]
    CodePPSG,
    #[serde(rename = "MANU")]
    CodeMANU,
    #[serde(rename = "MERC")]
    CodeMERC,
    #[serde(rename = "SCRT")]
    CodeSCRT,
    #[serde(rename = "SNCT")]
    CodeSNCT,
    #[serde(rename = "SCNL")]
    CodeSCNL,
}

impl AuthenticationMethod1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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

// BankToCustomerAccountReportV08: Reports on a cash account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankToCustomerAccountReportV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader811,
    #[serde(rename = "Rpt")]
    pub rpt: AccountReport251,
}

impl BankToCustomerAccountReportV08 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        self.rpt.validate()?;
        Ok(())
    }
}

// BankTransactionCodeStructure41: Bank transaction code in a proprietary form, as defined by the issuer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankTransactionCodeStructure41 {
    #[serde(rename = "Domn", skip_serializing_if = "Option::is_none")]
    pub domn: Option<BankTransactionCodeStructure5>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryBankTransactionCodeStructure11>,
}

impl BankTransactionCodeStructure41 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.domn {
            val.validate()?
        }
        if let Some(ref val) = self.prtry {
            val.validate()?
        }
        Ok(())
    }
}

// BankTransactionCodeStructure5: Specifies the family and the sub-family of the bank transaction code, within a specific domain, in a structured and hierarchical format.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankTransactionCodeStructure5 {
    #[serde(rename = "Cd")]
    pub cd: String,
    #[serde(rename = "Fmly")]
    pub fmly: BankTransactionCodeStructure6,
}

impl BankTransactionCodeStructure5 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.cd.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "cd is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.cd.chars().count() > 4 {
            return Err(ValidationError::new(
                1002,
                "cd exceeds the maximum length of 4".to_string(),
            ));
        }
        self.fmly.validate()?;
        Ok(())
    }
}

// BankTransactionCodeStructure6: Specifies the sub-product family within a specific family.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankTransactionCodeStructure6 {
    #[serde(rename = "Cd")]
    pub cd: String,
    #[serde(rename = "SubFmlyCd")]
    pub sub_fmly_cd: String,
}

impl BankTransactionCodeStructure6 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.cd.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "cd is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.cd.chars().count() > 4 {
            return Err(ValidationError::new(
                1002,
                "cd exceeds the maximum length of 4".to_string(),
            ));
        }
        if self.sub_fmly_cd.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "sub_fmly_cd is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.sub_fmly_cd.chars().count() > 4 {
            return Err(ValidationError::new(
                1002,
                "sub_fmly_cd exceeds the maximum length of 4".to_string(),
            ));
        }
        Ok(())
    }
}

// BatchInformation21: Indicates whether the batch entry is a credit or a debit entry.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BatchInformation21 {
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id: Option<String>,
    #[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
    pub nb_of_txs: Option<String>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
}

impl BatchInformation21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.msg_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "msg_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "msg_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "msg_id does not match the required pattern".to_string(),
                ));
            }
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pmt_inf_id does not match the required pattern".to_string(),
                ));
            }
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
        if let Some(ref val) = self.ttl_amt {
            val.validate()?
        }
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate()?
        }
        Ok(())
    }
}

// BranchAndFinancialInstitutionIdentification61: Identifies a specific branch of a financial institution.
//
// Usage: This component should be used in case the identification information in the financial institution component does not provide identification up to branch level.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification61 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification181,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData31>,
}

impl BranchAndFinancialInstitutionIdentification61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        if let Some(ref val) = self.brnch_id {
            val.validate()?
        }
        Ok(())
    }
}

// BranchAndFinancialInstitutionIdentification62: Identifies a specific branch of a financial institution.
//
// Usage: This component should be used in case the identification information in the financial institution component does not provide identification up to branch level.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification62 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification181,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData32>,
}

impl BranchAndFinancialInstitutionIdentification62 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        if let Some(ref val) = self.brnch_id {
            val.validate()?
        }
        Ok(())
    }
}

// BranchAndFinancialInstitutionIdentification63: Identifies a specific branch of a financial institution.
//
// Usage: This component should be used in case the identification information in the financial institution component does not provide identification up to branch level.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification63 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification182,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData32>,
}

impl BranchAndFinancialInstitutionIdentification63 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        if let Some(ref val) = self.brnch_id {
            val.validate()?
        }
        Ok(())
    }
}

// BranchAndFinancialInstitutionIdentification64: Identifies a specific branch of a financial institution.
//
// Usage: This component should be used in case the identification information in the financial institution component does not provide identification up to branch level.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification64 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification181,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData33>,
}

impl BranchAndFinancialInstitutionIdentification64 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        if let Some(ref val) = self.brnch_id {
            val.validate()?
        }
        Ok(())
    }
}

// BranchData31: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchData31 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
}

impl BranchData31 {
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

// BranchData32: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchData32 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
}

impl BranchData32 {
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

// BranchData33: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchData33 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
}

impl BranchData33 {
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

// CSCManagement1Code: No card security code imprint.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CSCManagement1Code {
    #[default]
    #[serde(rename = "PRST")]
    CodePRST,
    #[serde(rename = "BYPS")]
    CodeBYPS,
    #[serde(rename = "UNRD")]
    CodeUNRD,
    #[serde(rename = "NCSC")]
    CodeNCSC,
}

impl CSCManagement1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// CardAggregated21: Date range on which the globalisation applies.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardAggregated21 {
    #[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
    pub addtl_svc: Option<CardPaymentServiceType2Code>,
    #[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
    pub tx_ctgy: Option<String>,
    #[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id: Option<String>,
    #[serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none")]
    pub seq_nb_rg: Option<CardSequenceNumberRange11>,
    #[serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none")]
    pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice1>,
}

impl CardAggregated21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.addtl_svc {
            val.validate()?
        }
        if let Some(ref val) = self.tx_ctgy {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tx_ctgy is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "tx_ctgy exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sale_rcncltn_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "sale_rcncltn_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sale_rcncltn_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.seq_nb_rg {
            val.validate()?
        }
        if let Some(ref val) = self.tx_dt_rg {
            val.validate()?
        }
        Ok(())
    }
}

// CardDataReading1Code: Contactless proximity reader, with application conform to the standard EMV (standard initiated by Europay, Mastercard and Visa).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardDataReading1Code {
    #[default]
    #[serde(rename = "TAGC")]
    CodeTAGC,
    #[serde(rename = "PHYS")]
    CodePHYS,
    #[serde(rename = "BRCD")]
    CodeBRCD,
    #[serde(rename = "MGST")]
    CodeMGST,
    #[serde(rename = "CICC")]
    CodeCICC,
    #[serde(rename = "DFLE")]
    CodeDFLE,
    #[serde(rename = "CTLS")]
    CodeCTLS,
    #[serde(rename = "ECTL")]
    CodeECTL,
}

impl CardDataReading1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// CardEntry41: Prepaid account for the transfer or loading of an amount of money.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardEntry41 {
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard41>,
    #[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction11>,
    #[serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none")]
    pub aggtd_ntry: Option<CardAggregated21>,
    #[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount382>,
}

impl CardEntry41 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.card {
            val.validate()?
        }
        if let Some(ref val) = self.poi {
            val.validate()?
        }
        if let Some(ref val) = self.aggtd_ntry {
            val.validate()?
        }
        if let Some(ref val) = self.pre_pd_acct {
            val.validate()?
        }
        Ok(())
    }
}

// CardIndividualTransaction21: Sequential number of the validation of the cash deposit.
// Usage: The sequential number is increased incrementally for each transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardIndividualTransaction21 {
    #[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<String>,
    #[serde(rename = "PmtCntxt", skip_serializing_if = "Option::is_none")]
    pub pmt_cntxt: Option<PaymentContext3>,
    #[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
    pub addtl_svc: Option<CardPaymentServiceType2Code>,
    #[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
    pub tx_ctgy: Option<String>,
    #[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id: Option<String>,
    #[serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none")]
    pub sale_ref_nb: Option<String>,
    #[serde(rename = "RePresntmntRsn", skip_serializing_if = "Option::is_none")]
    pub re_presntmnt_rsn: Option<String>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<String>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<TransactionIdentifier11>,
    #[serde(rename = "Pdct", skip_serializing_if = "Option::is_none")]
    pub pdct: Option<Product21>,
    #[serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none")]
    pub vldtn_dt: Option<String>,
    #[serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none")]
    pub vldtn_seq_nb: Option<String>,
}

impl CardIndividualTransaction21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.icc_rltd_data {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "icc_rltd_data is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 1025 {
                return Err(ValidationError::new(
                    1002,
                    "icc_rltd_data exceeds the maximum length of 1025".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "icc_rltd_data does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pmt_cntxt {
            val.validate()?
        }
        if let Some(ref val) = self.addtl_svc {
            val.validate()?
        }
        if let Some(ref val) = self.tx_ctgy {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "tx_ctgy is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "tx_ctgy exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sale_rcncltn_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "sale_rcncltn_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sale_rcncltn_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.sale_ref_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sale_ref_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "sale_ref_nb exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sale_ref_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.re_presntmnt_rsn {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "re_presntmnt_rsn is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "re_presntmnt_rsn exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.seq_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "seq_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "seq_nb exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "seq_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tx_id {
            val.validate()?
        }
        if let Some(ref val) = self.pdct {
            val.validate()?
        }
        if let Some(ref val) = self.vldtn_seq_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "vldtn_seq_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "vldtn_seq_nb exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "vldtn_seq_nb does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CardPaymentServiceType2Code: Voice authorisation.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardPaymentServiceType2Code {
    #[default]
    #[serde(rename = "AGGR")]
    CodeAGGR,
    #[serde(rename = "DCCV")]
    CodeDCCV,
    #[serde(rename = "GRTT")]
    CodeGRTT,
    #[serde(rename = "INSP")]
    CodeINSP,
    #[serde(rename = "LOYT")]
    CodeLOYT,
    #[serde(rename = "NRES")]
    CodeNRES,
    #[serde(rename = "PUCO")]
    CodePUCO,
    #[serde(rename = "RECP")]
    CodeRECP,
    #[serde(rename = "SOAF")]
    CodeSOAF,
    #[serde(rename = "UNAF")]
    CodeUNAF,
    #[serde(rename = "VCAU")]
    CodeVCAU,
}

impl CardPaymentServiceType2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// CardSecurityInformation1: Card security code (CSC).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardSecurityInformation1 {
    #[serde(rename = "CSCMgmt")]
    pub csc_mgmt: CSCManagement1Code,
    #[serde(rename = "CSCVal", skip_serializing_if = "Option::is_none")]
    pub csc_val: Option<String>,
}

impl CardSecurityInformation1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.csc_mgmt.validate()?;
        if let Some(ref val) = self.csc_val {
            let pattern = Regex::new("[0-9]{3,4}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "csc_val does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CardSequenceNumberRange11: CardSequenceNumberRange1: LastTransactionSequenceNumberMessage element to be finalised once feedback from Card SEG has been received.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardSequenceNumberRange11 {
    #[serde(rename = "FrstTx", skip_serializing_if = "Option::is_none")]
    pub frst_tx: Option<String>,
    #[serde(rename = "LastTx", skip_serializing_if = "Option::is_none")]
    pub last_tx: Option<String>,
}

impl CardSequenceNumberRange11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.frst_tx {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "frst_tx is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "frst_tx exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "frst_tx does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.last_tx {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "last_tx is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "last_tx exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "last_tx does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CardTransaction171: Prepaid account for the transfer or loading of an amount of money.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardTransaction171 {
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard41>,
    #[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction11>,
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<CardTransaction3Choice1>,
    #[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount382>,
}

impl CardTransaction171 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.card {
            val.validate()?
        }
        if let Some(ref val) = self.poi {
            val.validate()?
        }
        if let Some(ref val) = self.tx {
            val.validate()?
        }
        if let Some(ref val) = self.pre_pd_acct {
            val.validate()?
        }
        Ok(())
    }
}

// CardTransaction3Choice1: Card transaction details for the individual transaction, as recorded at the POI (point of interaction).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardTransaction3Choice1 {
    #[serde(rename = "Aggtd", skip_serializing_if = "Option::is_none")]
    pub aggtd: Option<CardAggregated21>,
    #[serde(rename = "Indv", skip_serializing_if = "Option::is_none")]
    pub indv: Option<CardIndividualTransaction21>,
}

impl CardTransaction3Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.aggtd {
            val.validate()?
        }
        if let Some(ref val) = self.indv {
            val.validate()?
        }
        Ok(())
    }
}

// CardholderAuthentication2: Entity or object in charge of verifying the cardholder authenticity.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardholderAuthentication2 {
    #[serde(rename = "AuthntcnMtd")]
    pub authntcn_mtd: AuthenticationMethod1Code,
    #[serde(rename = "AuthntcnNtty")]
    pub authntcn_ntty: AuthenticationEntity1Code,
}

impl CardholderAuthentication2 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.authntcn_mtd.validate()?;
        self.authntcn_ntty.validate()?;
        Ok(())
    }
}

// CardholderVerificationCapability1Code: Three domain secure (three domain secure authentication of the cardholder).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardholderVerificationCapability1Code {
    #[default]
    #[serde(rename = "MNSG")]
    CodeMNSG,
    #[serde(rename = "NPIN")]
    CodeNPIN,
    #[serde(rename = "FCPN")]
    CodeFCPN,
    #[serde(rename = "FEPN")]
    CodeFEPN,
    #[serde(rename = "FDSG")]
    CodeFDSG,
    #[serde(rename = "FBIO")]
    CodeFBIO,
    #[serde(rename = "MNVR")]
    CodeMNVR,
    #[serde(rename = "FBIG")]
    CodeFBIG,
    #[serde(rename = "APKI")]
    CodeAPKI,
    #[serde(rename = "PKIS")]
    CodePKIS,
    #[serde(rename = "CHDT")]
    CodeCHDT,
    #[serde(rename = "SCEC")]
    CodeSCEC,
}

impl CardholderVerificationCapability1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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
    #[serde(rename = "Ccy")]
    pub ccy: String,
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
        let pattern = Regex::new("[A-Z]{3,3}").unwrap();
        if !pattern.is_match(&self.ccy) {
            return Err(ValidationError::new(
                1005,
                "ccy does not match the required pattern".to_string(),
            ));
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

// CashAccount382: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount382 {
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

impl CashAccount382 {
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

// CashAccount391: Party that manages the account on behalf of the account owner, that is manages the registration and booking of entries on the account, calculates balances on the account and provides information about the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount391 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice1,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CashAccountType2Choice1>,
    #[serde(rename = "Ccy")]
    pub ccy: String,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAccountIdentification11>,
    #[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
    pub ownr: Option<PartyIdentification1352>,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl CashAccount391 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.id.validate()?;
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        let pattern = Regex::new("[A-Z]{3,3}").unwrap();
        if !pattern.is_match(&self.ccy) {
            return Err(ValidationError::new(
                1005,
                "ccy does not match the required pattern".to_string(),
            ));
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
        if let Some(ref val) = self.ownr {
            val.validate()?
        }
        if let Some(ref val) = self.svcr {
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

// CashAvailability1: Indicates whether the availability balance is a credit or a debit balance.
// Usage: A zero balance is considered to be a credit balance.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAvailability1 {
    #[serde(rename = "Dt")]
    pub dt: CashAvailabilityDate1Choice,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
}

impl CashAvailability1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.dt.validate()?;
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        Ok(())
    }
}

// CashAvailabilityDate1Choice: Identifies the actual availability date.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAvailabilityDate1Choice {
    #[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
    pub nb_of_days: Option<String>,
    #[serde(rename = "ActlDt", skip_serializing_if = "Option::is_none")]
    pub actl_dt: Option<String>,
}

impl CashAvailabilityDate1Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nb_of_days {
            let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb_of_days does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CashBalance81: Set of elements used to indicate when the booked amount of money will become available, that is can be accessed and starts generating interest.
//
// Usage: This type of information is used in the US and is linked to particular instruments such as cheques.
// Example: When a cheque is deposited, it will be booked on the deposit day, but the amount of money will only be accessible as of the indicated availability day (according to national banking regulations).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashBalance81 {
    #[serde(rename = "Tp")]
    pub tp: BalanceType131,
    #[serde(rename = "CdtLine", skip_serializing_if = "Option::is_none")]
    pub cdt_line: Option<Vec<CreditLine31>>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "Dt")]
    pub dt: DateAndDateTime2Choice1,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
}

impl CashBalance81 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        if let Some(ref vec) = self.cdt_line {
            for item in vec {
                item.validate()?
            }
        }
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        self.dt.validate()?;
        if let Some(ref vec) = self.avlbty {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// CashDeposit11: Specifies the total amount of money in the cash deposit, that is the note denomination times the number of notes.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashDeposit11 {
    #[serde(rename = "NoteDnmtn")]
    pub note_dnmtn: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "NbOfNotes")]
    pub nb_of_notes: String,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl CashDeposit11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.note_dnmtn.validate()?;
        let pattern = Regex::new("[0-9]{1,15}").unwrap();
        if !pattern.is_match(&self.nb_of_notes) {
            return Err(ValidationError::new(
                1005,
                "nb_of_notes does not match the required pattern".to_string(),
            ));
        }
        self.amt.validate()?;
        Ok(())
    }
}

// ChargeBearerType1Code: Charges are to be applied following the rules agreed in the service level and/or scheme.
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

// ChargeType3Choice1: Type of charge in a proprietary form, as defined by the issuer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargeType3Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification31>,
}

impl ChargeType3Choice1 {
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

// Charges61: Provides details of the individual charges record.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charges61 {
    #[serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
    pub rcrd: Option<Vec<ChargesRecord31>>,
}

impl Charges61 {
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

// ChargesRecord31: Provides details on the tax applied to charges.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargesRecord31 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none")]
    pub chrg_incl_ind: Option<bool>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ChargeType3Choice1>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Br", skip_serializing_if = "Option::is_none")]
    pub br: Option<ChargeBearerType1Code>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges21>,
}

impl ChargesRecord31 {
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

// ClearingSystemMemberIdentification22: Identification of a member of a clearing system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemMemberIdentification22 {
    #[serde(rename = "ClrSysId")]
    pub clr_sys_id: ClearingSystemIdentification2Choice1,
    #[serde(rename = "MmbId")]
    pub mmb_id: String,
}

impl ClearingSystemMemberIdentification22 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.clr_sys_id.validate()?;
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

// Contact41: Name by which a party is known and which is usually used to identify that party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contact41 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl Contact41 {
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
        Ok(())
    }
}

// Contact42: Name by which a party is known and which is usually used to identify that party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contact42 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl Contact42 {
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
        Ok(())
    }
}

// Contact43: Preferred method used to reach the contact.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contact43 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}

impl Contact43 {
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
        if let Some(ref val) = self.prefrd_mtd {
            val.validate()?
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

// CorporateAction91: Identification of a corporate action assigned by an official central body/entity within a given market.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorporateAction91 {
    #[serde(rename = "EvtTp")]
    pub evt_tp: String,
    #[serde(rename = "EvtId")]
    pub evt_id: String,
}

impl CorporateAction91 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.evt_tp.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "evt_tp is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.evt_tp.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "evt_tp exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.evt_tp) {
            return Err(ValidationError::new(
                1005,
                "evt_tp does not match the required pattern".to_string(),
            ));
        }
        if self.evt_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "evt_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.evt_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "evt_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.evt_id) {
            return Err(ValidationError::new(
                1005,
                "evt_id does not match the required pattern".to_string(),
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

// CreditLine31: Date of the credit line provided when multiple credit lines may be provided.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditLine31 {
    #[serde(rename = "Incl")]
    pub incl: bool,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditLineType1Choice1>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice1>,
}

impl CreditLine31 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        if let Some(ref val) = self.dt {
            val.validate()?
        }
        Ok(())
    }
}

// CreditLineType1Choice1: Type of the credit line, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditLineType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CreditLineType1Choice1 {
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

// CreditorReferenceInformation21: Unique reference, as assigned by the creditor, to unambiguously refer to the payment transaction.
//
// Usage: If available, the initiating party should provide this reference in the structured remittance information, to enable reconciliation by the creditor upon receipt of the amount of money.
//
// If the business context requires the use of a creditor reference or a payment remit identification, and only one identifier can be passed through the end-to-end chain, the creditor's reference or payment remittance identification should be quoted in the end-to-end transaction identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceInformation21 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditorReferenceType21>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation21 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ref_attr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CreditorReferenceType1Choice1: Creditor reference type, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DocumentType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CreditorReferenceType1Choice1 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// CreditorReferenceType21: Entity that assigns the credit reference type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType21 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: CreditorReferenceType1Choice1,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl CreditorReferenceType21 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// CurrencyExchange51: Date and time at which an exchange rate is quoted.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CurrencyExchange51 {
    #[serde(rename = "SrcCcy")]
    pub src_ccy: String,
    #[serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none")]
    pub trgt_ccy: Option<String>,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<String>,
    #[serde(rename = "XchgRate")]
    pub xchg_rate: f64,
    #[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
    pub ctrct_id: Option<String>,
    #[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<String>,
}

impl CurrencyExchange51 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[A-Z]{3,3}").unwrap();
        if !pattern.is_match(&self.src_ccy) {
            return Err(ValidationError::new(
                1005,
                "src_ccy does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.trgt_ccy {
            let pattern = Regex::new("[A-Z]{3,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "trgt_ccy does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.unit_ccy {
            let pattern = Regex::new("[A-Z]{3,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "unit_ccy does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctrct_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ctrct_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ctrct_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctrct_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.qtn_dt {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "qtn_dt does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// CurrencyExchange52: Date and time at which an exchange rate is quoted.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CurrencyExchange52 {
    #[serde(rename = "SrcCcy")]
    pub src_ccy: String,
    #[serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none")]
    pub trgt_ccy: Option<String>,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<String>,
    #[serde(rename = "XchgRate")]
    pub xchg_rate: f64,
    #[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
    pub ctrct_id: Option<String>,
    #[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<String>,
}

impl CurrencyExchange52 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[A-Z]{3,3}").unwrap();
        if !pattern.is_match(&self.src_ccy) {
            return Err(ValidationError::new(
                1005,
                "src_ccy does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.trgt_ccy {
            let pattern = Regex::new("[A-Z]{3,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "trgt_ccy does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.unit_ccy {
            let pattern = Regex::new("[A-Z]{3,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "unit_ccy does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctrct_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ctrct_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ctrct_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctrct_id does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DateAndDateTime2Choice1: Specified date and time.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndDateTime2Choice1 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DateAndPlaceOfBirth1: Country where a person was born.
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

// DateOrDateTimePeriod1Choice1: Period expressed a dates and times.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateOrDateTimePeriod1Choice1 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DatePeriod2>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<DateTimePeriod11>,
}

impl DateOrDateTimePeriod1Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dt {
            val.validate()?
        }
        if let Some(ref val) = self.dt_tm {
            val.validate()?
        }
        Ok(())
    }
}

// DatePeriod2: End date of the range.
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

// DateTimePeriod11: Date and time at which the period ends.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateTimePeriod11 {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: String,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: String,
}

impl DateTimePeriod11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.fr_dt_tm) {
            return Err(ValidationError::new(
                1005,
                "fr_dt_tm does not match the required pattern".to_string(),
            ));
        }
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.to_dt_tm) {
            return Err(ValidationError::new(
                1005,
                "to_dt_tm does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// DiscountAmountAndType1: Amount of money, which has been typed.
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

// DiscountAmountAndType11: Amount of money, which has been typed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountAndType11 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DiscountAmountType1Choice1>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DiscountAmountAndType11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        self.amt.validate()?;
        Ok(())
    }
}

// DiscountAmountType1Choice: Specifies the amount type, in a free-text form.
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

// DiscountAmountType1Choice1: Specifies the amount type, in a free-text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl DiscountAmountType1Choice1 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// DisplayCapabilities1: Number of columns of the display component.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisplayCapabilities1 {
    #[serde(rename = "DispTp")]
    pub disp_tp: UserInterface2Code,
    #[serde(rename = "NbOfLines")]
    pub nb_of_lines: String,
    #[serde(rename = "LineWidth")]
    pub line_width: String,
}

impl DisplayCapabilities1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.disp_tp.validate()?;
        let pattern = Regex::new("[0-9]{1,3}").unwrap();
        if !pattern.is_match(&self.nb_of_lines) {
            return Err(ValidationError::new(
                1005,
                "nb_of_lines does not match the required pattern".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9]{1,3}").unwrap();
        if !pattern.is_match(&self.line_width) {
            return Err(ValidationError::new(
                1005,
                "line_width does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// DocumentAdjustment11: Provides further details on the document adjustment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentAdjustment11 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<String>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<String>,
}

impl DocumentAdjustment11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "rsn does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DocumentLineIdentification11: Date associated with the referred document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineIdentification11 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DocumentLineType11>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<String>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// DocumentLineInformation11: Provides details on the amounts of the document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineInformation11 {
    #[serde(rename = "Id")]
    pub id: Vec<DocumentLineIdentification11>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RemittanceAmount31>,
}

impl DocumentLineInformation11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "desc does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        Ok(())
    }
}

// DocumentLineType1Choice1: Proprietary identification of the type of the remittance document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl DocumentLineType1Choice1 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// DocumentLineType11: Identification of the issuer of the reference document line identificationtype.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType11 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: DocumentLineType1Choice1,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl DocumentLineType11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// DocumentType3Code: Document is a structured communication reference provided by the creditor to identify the referred transaction.
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

// DocumentType6Code: Document is a purchase order.
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

// EntryDetails91: Provides information on the underlying transaction(s).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryDetails91 {
    #[serde(rename = "Btch", skip_serializing_if = "Option::is_none")]
    pub btch: Option<BatchInformation21>,
    #[serde(rename = "TxDtls")]
    pub tx_dtls: EntryTransaction101,
}

impl EntryDetails91 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.btch {
            val.validate()?
        }
        self.tx_dtls.validate()?;
        Ok(())
    }
}

// EntryStatus1Choice1: Entry status, in a coded form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryStatus1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
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
        Ok(())
    }
}

// EntryTransaction101: Further details of the transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryTransaction101 {
    #[serde(rename = "Refs")]
    pub refs: TransactionReferences61,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<AmountAndCurrencyExchange32>,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
    #[serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none")]
    pub bk_tx_cd: Option<BankTransactionCodeStructure41>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Charges61>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<TransactionInterest41>,
    #[serde(rename = "RltdPties", skip_serializing_if = "Option::is_none")]
    pub rltd_pties: Option<TransactionParties61>,
    #[serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none")]
    pub rltd_agts: Option<TransactionAgents51>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice1>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice1>,
    #[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
    pub rltd_rmt_inf: Option<Vec<RemittanceLocation71>>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation161>,
    #[serde(rename = "RltdDts", skip_serializing_if = "Option::is_none")]
    pub rltd_dts: Option<TransactionDates31>,
    #[serde(rename = "RltdPric", skip_serializing_if = "Option::is_none")]
    pub rltd_pric: Option<TransactionPrice4Choice1>,
    #[serde(rename = "RltdQties", skip_serializing_if = "Option::is_none")]
    pub rltd_qties: Option<Vec<TransactionQuantities3Choice1>>,
    #[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification191>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInformation81>,
    #[serde(rename = "RtrInf", skip_serializing_if = "Option::is_none")]
    pub rtr_inf: Option<PaymentReturnReason51>,
    #[serde(rename = "CorpActn", skip_serializing_if = "Option::is_none")]
    pub corp_actn: Option<CorporateAction91>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount191>,
    #[serde(rename = "CshDpst", skip_serializing_if = "Option::is_none")]
    pub csh_dpst: Option<Vec<CashDeposit11>>,
    #[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardTransaction171>,
    #[serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none")]
    pub addtl_tx_inf: Option<String>,
}

impl EntryTransaction101 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.refs.validate()?;
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        if let Some(ref val) = self.amt_dtls {
            val.validate()?
        }
        if let Some(ref vec) = self.avlbty {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.bk_tx_cd {
            val.validate()?
        }
        if let Some(ref val) = self.chrgs {
            val.validate()?
        }
        if let Some(ref val) = self.intrst {
            val.validate()?
        }
        if let Some(ref val) = self.rltd_pties {
            val.validate()?
        }
        if let Some(ref val) = self.rltd_agts {
            val.validate()?
        }
        if let Some(ref val) = self.lcl_instrm {
            val.validate()?
        }
        if let Some(ref val) = self.purp {
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
        if let Some(ref val) = self.rltd_dts {
            val.validate()?
        }
        if let Some(ref val) = self.rltd_pric {
            val.validate()?
        }
        if let Some(ref vec) = self.rltd_qties {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.fin_instrm_id {
            val.validate()?
        }
        if let Some(ref val) = self.tax {
            val.validate()?
        }
        if let Some(ref val) = self.rtr_inf {
            val.validate()?
        }
        if let Some(ref val) = self.corp_actn {
            val.validate()?
        }
        if let Some(ref val) = self.sfkpg_acct {
            val.validate()?
        }
        if let Some(ref vec) = self.csh_dpst {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.card_tx {
            val.validate()?
        }
        if let Some(ref val) = self.addtl_tx_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_tx_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 500 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_tx_inf exceeds the maximum length of 500".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_tx_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// FinancialIdentificationSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl FinancialIdentificationSchemeName1Choice1 {
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

// FinancialInstitutionIdentification181: Unique identification of an agent, as assigned by an institution, using an identification scheme.
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
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification11>,
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
        if let Some(ref val) = self.othr {
            val.validate()?
        }
        Ok(())
    }
}

// FinancialInstitutionIdentification182: Unique identification of an agent, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification182 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification22>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification11>,
}

impl FinancialInstitutionIdentification182 {
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
        if let Some(ref val) = self.othr {
            val.validate()?
        }
        Ok(())
    }
}

// FinancialInstrumentQuantity1Choice: Quantity expressed as an amount representing the current amortised face amount of a bond, for example, a periodic reduction/increase of a bond's principal amount.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstrumentQuantity1Choice {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<f64>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<f64>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<f64>,
}

impl FinancialInstrumentQuantity1Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.face_amt {
            if *val < 0.000000 {
                return Err(ValidationError::new(
                    1003,
                    "face_amt is less than the minimum value of 0.000000".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.amtsd_val {
            if *val < 0.000000 {
                return Err(ValidationError::new(
                    1003,
                    "amtsd_val is less than the minimum value of 0.000000".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// FromToAmountRange1: Upper boundary of a range of amount values.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FromToAmountRange1 {
    #[serde(rename = "FrAmt")]
    pub fr_amt: AmountRangeBoundary1,
    #[serde(rename = "ToAmt")]
    pub to_amt: AmountRangeBoundary1,
}

impl FromToAmountRange1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fr_amt.validate()?;
        self.to_amt.validate()?;
        Ok(())
    }
}

// Garnishment31: Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Garnishment31 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType11,
    #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification1355>,
    #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification1356>,
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

impl Garnishment31 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ref_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate()?
        }
        Ok(())
    }
}

// GarnishmentType1Choice1: Proprietary identification of the type of garnishment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl GarnishmentType1Choice1 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// GarnishmentType11: Identification of the issuer of the garnishment type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType11 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: GarnishmentType1Choice1,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GarnishmentType11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// GenericFinancialIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericFinancialIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<FinancialIdentificationSchemeName1Choice1>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericFinancialIdentification11 {
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

// GenericIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<String>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericIdentification11 {
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
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// GenericIdentification302: Short textual description of the scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification302 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Issr")]
    pub issr: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<String>,
}

impl GenericIdentification302 {
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

// GenericIdentification321: Name of the entity.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification321 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType3Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType4Code>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<String>,
}

impl GenericIdentification321 {
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
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            val.validate()?
        }
        if let Some(ref val) = self.shrt_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "shrt_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "shrt_nm exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "shrt_nm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GenericIdentification31: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification31 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericIdentification31 {
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

// GenericOrganisationIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification12 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice2>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericOrganisationIdentification12 {
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
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// GenericPersonIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification12 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice2>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericPersonIdentification12 {
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
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// GroupHeader811: Further details of the message.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader811 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none")]
    pub msg_rcpt: Option<PartyIdentification1351>,
    #[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
    pub orgnl_biz_qry: Option<OriginalBusinessQuery11>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<String>,
}

impl GroupHeader811 {
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
        if let Some(ref val) = self.msg_rcpt {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_biz_qry {
            val.validate()?
        }
        if let Some(ref val) = self.addtl_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 500 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_inf exceeds the maximum length of 500".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// IdentificationSource3Choice1: Unique and unambiguous identification source using a proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct IdentificationSource3Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl IdentificationSource3Choice1 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// ImpliedCurrencyAmountRange1Choice: Value that an amount must not match to be considered valid.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImpliedCurrencyAmountRange1Choice {
    #[serde(rename = "FrAmt", skip_serializing_if = "Option::is_none")]
    pub fr_amt: Option<AmountRangeBoundary1>,
    #[serde(rename = "ToAmt", skip_serializing_if = "Option::is_none")]
    pub to_amt: Option<AmountRangeBoundary1>,
    #[serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none")]
    pub fr_to_amt: Option<FromToAmountRange1>,
    #[serde(rename = "EQAmt", skip_serializing_if = "Option::is_none")]
    pub eq_amt: Option<f64>,
    #[serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none")]
    pub neq_amt: Option<f64>,
}

impl ImpliedCurrencyAmountRange1Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.fr_amt {
            val.validate()?
        }
        if let Some(ref val) = self.to_amt {
            val.validate()?
        }
        if let Some(ref val) = self.fr_to_amt {
            val.validate()?
        }
        if let Some(ref val) = self.eq_amt {
            if *val < 0.000000 {
                return Err(ValidationError::new(
                    1003,
                    "eq_amt is less than the minimum value of 0.000000".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.neq_amt {
            if *val < 0.000000 {
                return Err(ValidationError::new(
                    1003,
                    "neq_amt is less than the minimum value of 0.000000".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// InterestRecord21: Provides details on the tax applied to charges.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InterestRecord21 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestType1Choice1>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate41>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod11>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<String>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges21>,
}

impl InterestRecord21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        if let Some(ref val) = self.rate {
            val.validate()?
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate()?
        }
        if let Some(ref val) = self.rsn {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "rsn is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "rsn exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "rsn does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tax {
            val.validate()?
        }
        Ok(())
    }
}

// InterestType1Choice1: Specifies the type of interest in uncoded form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InterestType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl InterestType1Choice1 {
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

// InterestType1Code: Period of time between the end of a business day and the start of the next business day (usually the day after).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum InterestType1Code {
    #[default]
    #[serde(rename = "INDY")]
    CodeINDY,
    #[serde(rename = "OVRN")]
    CodeOVRN,
}

impl InterestType1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// LocalInstrument2Choice1: Specifies the local instrument, as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalInstrument2Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl LocalInstrument2Choice1 {
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

// MessageIdentification21: Specifies the identification of the message that will be used to provide additional details.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageIdentification21 {
    #[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<String>,
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
}

impl MessageIdentification21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.msg_nm_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "msg_nm_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "msg_nm_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "msg_nm_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.msg_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "msg_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "msg_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "msg_id does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// NameAndAddress161: Postal address of a party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NameAndAddress161 {
    #[serde(rename = "Nm")]
    pub nm: String,
    #[serde(rename = "Adr")]
    pub adr: PostalAddress241,
}

impl NameAndAddress161 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.nm.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "nm is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.nm.chars().count() > 140 {
            return Err(ValidationError::new(
                1002,
                "nm exceeds the maximum length of 140".to_string(),
            ));
        }
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
        if !pattern.is_match(&self.nm) {
            return Err(ValidationError::new(
                1005,
                "nm does not match the required pattern".to_string(),
            ));
        }
        self.adr.validate()?;
        Ok(())
    }
}

// NumberAndSumOfTransactions1: Total of all individual entries included in the report.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NumberAndSumOfTransactions1 {
    #[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries: Option<String>,
    #[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
}

impl NumberAndSumOfTransactions1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nb_of_ntries {
            let pattern = Regex::new("[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb_of_ntries does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// NumberAndSumOfTransactions4: Resulting debit or credit amount of the netted amounts for all debit and credit entries.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NumberAndSumOfTransactions4 {
    #[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries: Option<String>,
    #[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none")]
    pub ttl_net_ntry: Option<AmountAndDirection35>,
}

impl NumberAndSumOfTransactions4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nb_of_ntries {
            let pattern = Regex::new("[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb_of_ntries does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ttl_net_ntry {
            val.validate()?
        }
        Ok(())
    }
}

// OnLineCapability1Code: Off-line capable with possible on-line requests to the acquirer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum OnLineCapability1Code {
    #[default]
    #[serde(rename = "OFLN")]
    CodeOFLN,
    #[serde(rename = "ONLN")]
    CodeONLN,
    #[serde(rename = "SMON")]
    CodeSMON,
}

impl OnLineCapability1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
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

// OrganisationIdentification292: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification292 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericOrganisationIdentification12>>,
}

impl OrganisationIdentification292 {
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

// OrganisationIdentificationSchemeName1Choice2: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice2 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice2 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// OriginalAndCurrentQuantities1: Quantity expressed as an amount representing the current amortised face amount of a bond, for example, a periodic reduction/increase of a bond's principal amount.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalAndCurrentQuantities1 {
    #[serde(rename = "FaceAmt")]
    pub face_amt: f64,
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: f64,
}

impl OriginalAndCurrentQuantities1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// OriginalBusinessQuery11: Date and time at which the message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalBusinessQuery11 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<String>,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery11 {
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
        if let Some(ref val) = self.msg_nm_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "msg_nm_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "msg_nm_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "msg_nm_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.cre_dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "cre_dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// OtherIdentification11: Type of the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OtherIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<String>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice1,
}

impl OtherIdentification11 {
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
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.sfx {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sfx is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "sfx exceeds the maximum length of 16".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sfx does not match the required pattern".to_string(),
                ));
            }
        }
        self.tp.validate()?;
        Ok(())
    }
}

// POIComponentType1Code: Personal identification number (or PIN) entry device (PED).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum POIComponentType1Code {
    #[default]
    #[serde(rename = "SOFT")]
    CodeSOFT,
    #[serde(rename = "EMVK")]
    CodeEMVK,
    #[serde(rename = "EMVO")]
    CodeEMVO,
    #[serde(rename = "MRIT")]
    CodeMRIT,
    #[serde(rename = "CHIT")]
    CodeCHIT,
    #[serde(rename = "SECM")]
    CodeSECM,
    #[serde(rename = "PEDV")]
    CodePEDV,
}

impl POIComponentType1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// Pagination1: Indicates the last page.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pagination1 {
    #[serde(rename = "PgNb")]
    pub pg_nb: String,
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: bool,
}

impl Pagination1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[0-9]{1,5}").unwrap();
        if !pattern.is_match(&self.pg_nb) {
            return Err(ValidationError::new(
                1005,
                "pg_nb does not match the required pattern".to_string(),
            ));
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

// Party38Choice2: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice2 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification292>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification132>,
}

impl Party38Choice2 {
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

// Party38Choice3: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice3 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification292>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification133>,
}

impl Party38Choice3 {
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
    pub pty: Option<PartyIdentification1353>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification62>,
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

// Party40Choice2: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party40Choice2 {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification1354>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification62>,
}

impl Party40Choice2 {
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

// Party40Choice3: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party40Choice3 {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification1353>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification63>,
}

impl Party40Choice3 {
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

// PartyIdentification1351: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1351 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact41>,
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
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PartyIdentification1352: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1352 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl PartyIdentification1352 {
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

// PartyIdentification1353: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1353 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact42>,
}

impl PartyIdentification1353 {
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
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PartyIdentification1354: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1354 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact43>,
}

impl PartyIdentification1354 {
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
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PartyIdentification1355: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1355 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice2>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact42>,
}

impl PartyIdentification1355 {
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
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PartyIdentification1356: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1356 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice3>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact42>,
}

impl PartyIdentification1356 {
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
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PartyIdentification1357: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1357 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact43>,
}

impl PartyIdentification1357 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
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
        if let Some(ref val) = self.ctct_dtls {
            val.validate()?
        }
        Ok(())
    }
}

// PartyType3Code: Party to whom the card issuer delegates to authorise card payment transactions.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PartyType3Code {
    #[default]
    #[serde(rename = "OPOI")]
    CodeOPOI,
    #[serde(rename = "MERC")]
    CodeMERC,
    #[serde(rename = "ACCP")]
    CodeACCP,
    #[serde(rename = "ITAG")]
    CodeITAG,
    #[serde(rename = "ACQR")]
    CodeACQR,
    #[serde(rename = "CISS")]
    CodeCISS,
    #[serde(rename = "DLIS")]
    CodeDLIS,
}

impl PartyType3Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// PartyType4Code: Tax authority.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PartyType4Code {
    #[default]
    #[serde(rename = "MERC")]
    CodeMERC,
    #[serde(rename = "ACCP")]
    CodeACCP,
    #[serde(rename = "ITAG")]
    CodeITAG,
    #[serde(rename = "ACQR")]
    CodeACQR,
    #[serde(rename = "CISS")]
    CodeCISS,
    #[serde(rename = "TAXH")]
    CodeTAXH,
}

impl PartyType4Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// PaymentCard41: Additional card issuer specific data.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentCard41 {
    #[serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none")]
    pub plain_card_data: Option<PlainCardData11>,
    #[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<String>,
    #[serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none")]
    pub card_brnd: Option<GenericIdentification11>,
    #[serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none")]
    pub addtl_card_data: Option<String>,
}

impl PaymentCard41 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.plain_card_data {
            val.validate()?
        }
        if let Some(ref val) = self.card_ctry_cd {
            let pattern = Regex::new("[0-9]{3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "card_ctry_cd does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.card_brnd {
            val.validate()?
        }
        if let Some(ref val) = self.addtl_card_data {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_card_data is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_card_data exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_card_data does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// PaymentContext3: Method used to authenticate a cardholder.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentContext3 {
    #[serde(rename = "CardPres", skip_serializing_if = "Option::is_none")]
    pub card_pres: Option<bool>,
    #[serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none")]
    pub crdhldr_pres: Option<bool>,
    #[serde(rename = "OnLineCntxt", skip_serializing_if = "Option::is_none")]
    pub on_line_cntxt: Option<bool>,
    #[serde(rename = "AttndncCntxt", skip_serializing_if = "Option::is_none")]
    pub attndnc_cntxt: Option<AttendanceContext1Code>,
    #[serde(rename = "TxEnvt", skip_serializing_if = "Option::is_none")]
    pub tx_envt: Option<TransactionEnvironment1Code>,
    #[serde(rename = "TxChanl", skip_serializing_if = "Option::is_none")]
    pub tx_chanl: Option<TransactionChannel1Code>,
    #[serde(rename = "AttndntMsgCpbl", skip_serializing_if = "Option::is_none")]
    pub attndnt_msg_cpbl: Option<bool>,
    #[serde(rename = "AttndntLang", skip_serializing_if = "Option::is_none")]
    pub attndnt_lang: Option<String>,
    #[serde(rename = "CardDataNtryMd")]
    pub card_data_ntry_md: CardDataReading1Code,
    #[serde(rename = "FllbckInd", skip_serializing_if = "Option::is_none")]
    pub fllbck_ind: Option<bool>,
    #[serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none")]
    pub authntcn_mtd: Option<CardholderAuthentication2>,
}

impl PaymentContext3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.attndnc_cntxt {
            val.validate()?
        }
        if let Some(ref val) = self.tx_envt {
            val.validate()?
        }
        if let Some(ref val) = self.tx_chanl {
            val.validate()?
        }
        if let Some(ref val) = self.attndnt_lang {
            let pattern = Regex::new("[a-z]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "attndnt_lang does not match the required pattern".to_string(),
                ));
            }
        }
        self.card_data_ntry_md.validate()?;
        if let Some(ref val) = self.authntcn_mtd {
            val.validate()?
        }
        Ok(())
    }
}

// PaymentReturnReason51: Further details on the return reason.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentReturnReason51 {
    #[serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none")]
    pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure41>,
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification1357>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<ReturnReason5Choice1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl PaymentReturnReason51 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgnl_bk_tx_cd {
            val.validate()?
        }
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
                let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
                if !pattern.is_match(item) {
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

// PersonIdentification132: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification132 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification12>>,
}

impl PersonIdentification132 {
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

// PersonIdentification133: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification133 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification12>>,
}

impl PersonIdentification133 {
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

// PersonIdentificationSchemeName1Choice2: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice2 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice2 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// PlainCardData11: Card security code (CSC) associated with the card performing the transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlainCardData11 {
    #[serde(rename = "PAN")]
    pub pan: String,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<String>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<String>,
    #[serde(rename = "XpryDt")]
    pub xpry_dt: String,
    #[serde(rename = "SvcCd", skip_serializing_if = "Option::is_none")]
    pub svc_cd: Option<String>,
    #[serde(rename = "TrckData", skip_serializing_if = "Option::is_none")]
    pub trck_data: Option<Vec<TrackData11>>,
    #[serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none")]
    pub card_scty_cd: Option<CardSecurityInformation1>,
}

impl PlainCardData11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new("[0-9]{8,28}").unwrap();
        if !pattern.is_match(&self.pan) {
            return Err(ValidationError::new(
                1005,
                "pan does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.card_seq_nb {
            let pattern = Regex::new("[0-9]{2,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "card_seq_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.svc_cd {
            let pattern = Regex::new("[0-9]{3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "svc_cd does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.trck_data {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.card_scty_cd {
            val.validate()?
        }
        Ok(())
    }
}

// PointOfInteraction11: Data related to a component of the POI performing the transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteraction11 {
    #[serde(rename = "Id")]
    pub id: GenericIdentification321,
    #[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
    pub sys_nm: Option<String>,
    #[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
    pub grp_id: Option<String>,
    #[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<PointOfInteractionCapabilities1>,
    #[serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none")]
    pub cmpnt: Option<Vec<PointOfInteractionComponent11>>,
}

impl PointOfInteraction11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.id.validate()?;
        if let Some(ref val) = self.sys_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sys_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "sys_nm exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sys_nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.grp_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "grp_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "grp_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "grp_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.cpblties {
            val.validate()?
        }
        if let Some(ref vec) = self.cmpnt {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// PointOfInteractionCapabilities1: Number of columns of the printer component.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteractionCapabilities1 {
    #[serde(rename = "CardRdngCpblties", skip_serializing_if = "Option::is_none")]
    pub card_rdng_cpblties: Option<Vec<CardDataReading1Code>>,
    #[serde(
        rename = "CrdhldrVrfctnCpblties",
        skip_serializing_if = "Option::is_none"
    )]
    pub crdhldr_vrfctn_cpblties: Option<Vec<CardholderVerificationCapability1Code>>,
    #[serde(rename = "OnLineCpblties", skip_serializing_if = "Option::is_none")]
    pub on_line_cpblties: Option<OnLineCapability1Code>,
    #[serde(rename = "DispCpblties", skip_serializing_if = "Option::is_none")]
    pub disp_cpblties: Option<Vec<DisplayCapabilities1>>,
    #[serde(rename = "PrtLineWidth", skip_serializing_if = "Option::is_none")]
    pub prt_line_width: Option<String>,
}

impl PointOfInteractionCapabilities1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref vec) = self.card_rdng_cpblties {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref vec) = self.crdhldr_vrfctn_cpblties {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.on_line_cpblties {
            val.validate()?
        }
        if let Some(ref vec) = self.disp_cpblties {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.prt_line_width {
            let pattern = Regex::new("[0-9]{1,3}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prt_line_width does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// PointOfInteractionComponent11: Unique approval number for a component, delivered by a certification body.
// Usage: More than one approval number could be present, when assigned by different bodies. The certification body identification must be provided within the approval number (for example at the beginning of the value).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteractionComponent11 {
    #[serde(rename = "POICmpntTp")]
    pub poi_cmpnt_tp: POIComponentType1Code,
    #[serde(rename = "ManfctrId", skip_serializing_if = "Option::is_none")]
    pub manfctr_id: Option<String>,
    #[serde(rename = "Mdl", skip_serializing_if = "Option::is_none")]
    pub mdl: Option<String>,
    #[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
    pub vrsn_nb: Option<String>,
    #[serde(rename = "SrlNb", skip_serializing_if = "Option::is_none")]
    pub srl_nb: Option<String>,
    #[serde(rename = "ApprvlNb", skip_serializing_if = "Option::is_none")]
    pub apprvl_nb: Option<Vec<String>>,
}

impl PointOfInteractionComponent11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.poi_cmpnt_tp.validate()?;
        if let Some(ref val) = self.manfctr_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "manfctr_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "manfctr_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "manfctr_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.mdl {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "mdl is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "mdl exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "mdl does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.vrsn_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "vrsn_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "vrsn_nb exceeds the maximum length of 16".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "vrsn_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.srl_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "srl_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "srl_nb exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "srl_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.apprvl_nb {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "apprvl_nb is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 70 {
                    return Err(ValidationError::new(
                        1002,
                        "apprvl_nb exceeds the maximum length of 70".to_string(),
                    ));
                }
                let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "apprvl_nb does not match the required pattern".to_string(),
                    ));
                }
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

impl PostalAddress242 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
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

// PreferredContactMethod1Code: Preferred method used to reach the contact is per mobile or cell phone.
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

// Price71: Value of the price, for example, as a currency and value.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Price71 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType1Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice1,
}

impl Price71 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        self.val.validate()?;
        Ok(())
    }
}

// PriceRateOrAmount3Choice1: Price expressed as a currency and value.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PriceRateOrAmount3Choice1 {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PriceRateOrAmount3Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        Ok(())
    }
}

// PriceValueType1Code: Price is the face amount.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PriceValueType1Code {
    #[default]
    #[serde(rename = "DISC")]
    CodeDISC,
    #[serde(rename = "PREM")]
    CodePREM,
    #[serde(rename = "PARV")]
    CodePARV,
}

impl PriceValueType1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// Product21: Additional information related to the product.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Product21 {
    #[serde(rename = "PdctCd")]
    pub pdct_cd: String,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
    pub pdct_qty: Option<f64>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<f64>,
    #[serde(rename = "PdctAmt", skip_serializing_if = "Option::is_none")]
    pub pdct_amt: Option<f64>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
    #[serde(rename = "AddtlPdctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_inf: Option<String>,
}

impl Product21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.pdct_cd.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "pdct_cd is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.pdct_cd.chars().count() > 70 {
            return Err(ValidationError::new(
                1002,
                "pdct_cd exceeds the maximum length of 70".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.pdct_cd) {
            return Err(ValidationError::new(
                1005,
                "pdct_cd does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.unit_of_measr {
            val.validate()?
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_tp does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.addtl_pdct_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_pdct_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_pdct_inf exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_pdct_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ProprietaryAgent41: Organisation established primarily to provide financial services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryAgent41 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification62,
}

impl ProprietaryAgent41 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        self.agt.validate()?;
        Ok(())
    }
}

// ProprietaryBankTransactionCodeStructure11: Identification of the issuer of the proprietary bank transaction code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryBankTransactionCodeStructure11 {
    #[serde(rename = "Cd")]
    pub cd: String,
    #[serde(rename = "Issr")]
    pub issr: String,
}

impl ProprietaryBankTransactionCodeStructure11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.cd.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "cd is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.cd.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "cd exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.cd) {
            return Err(ValidationError::new(
                1005,
                "cd does not match the required pattern".to_string(),
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
        Ok(())
    }
}

// ProprietaryDate31: Date in ISO format.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryDate31 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Dt")]
    pub dt: DateAndDateTime2Choice1,
}

impl ProprietaryDate31 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        self.dt.validate()?;
        Ok(())
    }
}

// ProprietaryParty51: Proprietary party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryParty51 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Pty")]
    pub pty: Party40Choice1,
}

impl ProprietaryParty51 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        self.pty.validate()?;
        Ok(())
    }
}

// ProprietaryPrice21: Proprietary price specification related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryPrice21 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Pric")]
    pub pric: ActiveOrHistoricCurrencyAndAmount,
}

impl ProprietaryPrice21 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        self.pric.validate()?;
        Ok(())
    }
}

// ProprietaryQuantity11: Provides the proprietary quantity in free format.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryQuantity11 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Qty")]
    pub qty: String,
}

impl ProprietaryQuantity11 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
        if self.qty.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "qty is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.qty.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "qty exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.qty) {
            return Err(ValidationError::new(
                1005,
                "qty does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// ProprietaryReference11: Proprietary reference specification related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryReference11 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Ref")]
    pub ref_attr: String,
}

impl ProprietaryReference11 {
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tp) {
            return Err(ValidationError::new(
                1005,
                "tp does not match the required pattern".to_string(),
            ));
        }
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
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.ref_attr) {
            return Err(ValidationError::new(
                1005,
                "ref_attr does not match the required pattern".to_string(),
            ));
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

// Purpose2Choice1: Purpose, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Purpose2Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Purpose2Choice1 {
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

// Rate41: An amount range where the interest rate is applicable.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Rate41 {
    #[serde(rename = "Tp")]
    pub tp: RateType4Choice1,
    #[serde(rename = "VldtyRg", skip_serializing_if = "Option::is_none")]
    pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}

impl Rate41 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        if let Some(ref val) = self.vldty_rg {
            val.validate()?
        }
        Ok(())
    }
}

// RateType4Choice1: Rate type expressed, in an other form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RateType4Choice1 {
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<f64>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<String>,
}

impl RateType4Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.othr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "othr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "othr exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "othr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ReferredDocumentInformation71: Set of elements used to provide the content of the referred document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentInformation71 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ReferredDocumentType41>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<String>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<String>,
    #[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
    pub line_dtls: Option<Vec<DocumentLineInformation11>>,
}

impl ReferredDocumentInformation71 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb does not match the required pattern".to_string(),
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

// ReferredDocumentType3Choice: Proprietary identification of the type of the remittance document.
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

// ReferredDocumentType41: Identification of the issuer of the reference document type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType41 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: ReferredDocumentType3Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl ReferredDocumentType41 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// RemittanceAmount21: Amount of money remitted for the referred document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount21 {
    #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
    #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<Vec<TaxAmountAndType1>>,
    #[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment11>>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount21 {
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

// RemittanceAmount31: Amount of money remitted.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount31 {
    #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
    pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType11>>,
    #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<Vec<TaxAmountAndType11>>,
    #[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
    pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment11>>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount31 {
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

// RemittanceInformation161: Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in a structured form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceInformation161 {
    #[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
    pub ustrd: Option<String>,
    #[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
    pub strd: Option<Vec<StructuredRemittanceInformation161>>,
}

impl RemittanceInformation161 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.ustrd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ustrd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "ustrd exceeds the maximum length of 140".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ustrd does not match the required pattern".to_string(),
                ));
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

// RemittanceLocation71: Set of elements used to provide information on the location and/or delivery of the remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceLocation71 {
    #[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
    pub rmt_id: Option<String>,
    #[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
    pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData11>>,
}

impl RemittanceLocation71 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.rmt_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "rmt_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "rmt_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "rmt_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.rmt_lctn_dtls {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// RemittanceLocationData11: Postal address to which an agent is to send the remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceLocationData11 {
    #[serde(rename = "Mtd")]
    pub mtd: RemittanceLocationMethod2Code,
    #[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
    pub elctrnc_adr: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<NameAndAddress161>,
}

impl RemittanceLocationData11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.mtd.validate()?;
        if let Some(ref val) = self.elctrnc_adr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "elctrnc_adr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 2048 {
                return Err(ValidationError::new(
                    1002,
                    "elctrnc_adr exceeds the maximum length of 2048".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "elctrnc_adr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        Ok(())
    }
}

// RemittanceLocationMethod2Code: Remittance advice information must be sent through by phone as a short message service (SMS).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum RemittanceLocationMethod2Code {
    #[default]
    #[serde(rename = "FAXI")]
    CodeFAXI,
    #[serde(rename = "EDIC")]
    CodeEDIC,
    #[serde(rename = "URID")]
    CodeURID,
    #[serde(rename = "EMAL")]
    CodeEMAL,
    #[serde(rename = "POST")]
    CodePOST,
    #[serde(rename = "SMSM")]
    CodeSMSM,
}

impl RemittanceLocationMethod2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// ReportEntry101: Further details of the entry.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReportEntry101 {
    #[serde(rename = "NtryRef", skip_serializing_if = "Option::is_none")]
    pub ntry_ref: Option<String>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "RvslInd", skip_serializing_if = "Option::is_none")]
    pub rvsl_ind: Option<bool>,
    #[serde(rename = "Sts")]
    pub sts: EntryStatus1Choice1,
    #[serde(rename = "BookgDt", skip_serializing_if = "Option::is_none")]
    pub bookg_dt: Option<DateAndDateTime2Choice1>,
    #[serde(rename = "ValDt")]
    pub val_dt: DateAndDateTime2Choice1,
    #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<String>,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: BankTransactionCodeStructure41,
    #[serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none")]
    pub comssn_wvr_ind: Option<bool>,
    #[serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none")]
    pub addtl_inf_ind: Option<MessageIdentification21>,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<AmountAndCurrencyExchange31>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Charges61>,
    #[serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none")]
    pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice1>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<TransactionInterest41>,
    #[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardEntry41>,
    #[serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none")]
    pub ntry_dtls: Option<Vec<EntryDetails91>>,
    #[serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ntry_inf: Option<String>,
}

impl ReportEntry101 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.ntry_ref {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ntry_ref is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ntry_ref exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ntry_ref does not match the required pattern".to_string(),
                ));
            }
        }
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        self.sts.validate()?;
        if let Some(ref val) = self.bookg_dt {
            val.validate()?
        }
        self.val_dt.validate()?;
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "acct_svcr_ref does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.avlbty {
            for item in vec {
                item.validate()?
            }
        }
        self.bk_tx_cd.validate()?;
        if let Some(ref val) = self.addtl_inf_ind {
            val.validate()?
        }
        if let Some(ref val) = self.amt_dtls {
            val.validate()?
        }
        if let Some(ref val) = self.chrgs {
            val.validate()?
        }
        if let Some(ref val) = self.tech_inpt_chanl {
            val.validate()?
        }
        if let Some(ref val) = self.intrst {
            val.validate()?
        }
        if let Some(ref val) = self.card_tx {
            val.validate()?
        }
        if let Some(ref vec) = self.ntry_dtls {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.addtl_ntry_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_ntry_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 500 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_ntry_inf exceeds the maximum length of 500".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_ntry_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// ReportingSource1Choice1: Reporting source, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReportingSource1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ReportingSource1Choice1 {
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

// ReturnReason5Choice1: Reason for the return, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReturnReason5Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ReturnReason5Choice1 {
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

// SecuritiesAccount191: Description of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SecuritiesAccount191 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification302>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl SecuritiesAccount191 {
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
        if let Some(ref val) = self.tp {
            val.validate()?
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
        Ok(())
    }
}

// SecurityIdentification191: Textual description of a security instrument.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SecurityIdentification191 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<Vec<OtherIdentification11>>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

impl SecurityIdentification191 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.isin {
            let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "isin does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.othr_id {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.desc {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "desc is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "desc exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "desc does not match the required pattern".to_string(),
                ));
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

// StructuredRemittanceInformation161: Additional information, in free text form, to complement the structured remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRemittanceInformation161 {
    #[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation71>>,
    #[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<RemittanceAmount21>,
    #[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation21>,
    #[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
    pub invcr: Option<PartyIdentification1353>,
    #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification1353>,
    #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxInformation71>,
    #[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt: Option<Garnishment31>,
    #[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rmt_inf: Option<Vec<String>>,
}

impl StructuredRemittanceInformation161 {
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
                let pattern = Regex::new(
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                )
                .unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "addtl_rmt_inf does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// TaxAmount2: Set of elements used to provide details on the tax period and amount.
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

// TaxAmountAndType1: Amount of money, which has been typed.
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

// TaxAmountAndType11: Amount of money, which has been typed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmountAndType11 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxAmountType1Choice1>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxAmountAndType11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.tp {
            val.validate()?
        }
        self.amt.validate()?;
        Ok(())
    }
}

// TaxAmountType1Choice: Specifies the amount type, in a free-text form.
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

// TaxAmountType1Choice1: Specifies the amount type, in a free-text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmountType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl TaxAmountType1Choice1 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
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

// TaxAuthorisation11: Name of the debtor or the debtor's authorised representative.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAuthorisation11 {
    #[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
    pub titl: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl TaxAuthorisation11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "titl does not match the required pattern".to_string(),
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
        Ok(())
    }
}

// TaxAuthorisation12: Name of the debtor or the debtor's authorised representative.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAuthorisation12 {
    #[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
    pub titl: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl TaxAuthorisation12 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "titl does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxCharges21: Amount of money resulting from the calculation of the tax.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxCharges21 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TaxCharges21 {
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
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        Ok(())
    }
}

// TaxInformation71: Record of tax details.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxInformation71 {
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<TaxParty11>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<TaxParty21>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<TaxParty21>,
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
    pub rcrd: Option<Vec<TaxRecord21>>,
}

impl TaxInformation71 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "admstn_zone does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ref_nb does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "mtd does not match the required pattern".to_string(),
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

// TaxInformation81: Record of tax details.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxInformation81 {
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<TaxParty12>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<TaxParty22>,
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
    pub rcrd: Option<Vec<TaxRecord22>>,
}

impl TaxInformation81 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cdtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "admstn_zone does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ref_nb does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "mtd does not match the required pattern".to_string(),
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

// TaxParty11: Type of tax payer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty11 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
}

impl TaxParty11 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "regn_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_tp does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxParty12: Type of tax payer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty12 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
}

impl TaxParty12 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "regn_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_tp does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxParty21: Details of the authorised tax paying party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty21 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
    #[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
    pub authstn: Option<TaxAuthorisation11>,
}

impl TaxParty21 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "regn_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_tp does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.authstn {
            val.validate()?
        }
        Ok(())
    }
}

// TaxParty22: Details of the authorised tax paying party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty22 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
    #[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
    pub authstn: Option<TaxAuthorisation12>,
}

impl TaxParty22 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "regn_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tax_tp does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.authstn {
            val.validate()?
        }
        Ok(())
    }
}

// TaxPeriod2: Range of time between a start date and an end date for which the tax report is provided.
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

// TaxRecord21: Further details of the tax record.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxRecord21 {
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

impl TaxRecord21 {
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tp does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctgy does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctgy_dtls does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dbtr_sts does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "cert_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "frms_cd does not match the required pattern".to_string(),
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
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxRecord22: Further details of the tax record.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxRecord22 {
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

impl TaxRecord22 {
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tp does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctgy does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctgy_dtls does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dbtr_sts does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "cert_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "frms_cd does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "addtl_inf does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// TaxRecordDetails2: Underlying tax amount related to the specified period.
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

// TaxRecordPeriod1Code: Tax is related to the second half of the period.
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

// TechnicalInputChannel1Choice1: Technical channel used to input the instruction, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TechnicalInputChannel1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl TechnicalInputChannel1Choice1 {
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

// TotalTransactions61: Specifies the total number and sum of entries per bank transaction code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TotalTransactions61 {
    #[serde(rename = "TtlNtries", skip_serializing_if = "Option::is_none")]
    pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
    #[serde(rename = "TtlCdtNtries", skip_serializing_if = "Option::is_none")]
    pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "TtlDbtNtries", skip_serializing_if = "Option::is_none")]
    pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "TtlNtriesPerBkTxCd", skip_serializing_if = "Option::is_none")]
    pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode51>>,
}

impl TotalTransactions61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.ttl_ntries {
            val.validate()?
        }
        if let Some(ref val) = self.ttl_cdt_ntries {
            val.validate()?
        }
        if let Some(ref val) = self.ttl_dbt_ntries {
            val.validate()?
        }
        if let Some(ref vec) = self.ttl_ntries_per_bk_tx_cd {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TotalsPerBankTransactionCode51: Indicates the date (and time) of the transaction summary.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TotalsPerBankTransactionCode51 {
    #[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries: Option<String>,
    #[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none")]
    pub ttl_net_ntry: Option<AmountAndDirection35>,
    #[serde(rename = "CdtNtries", skip_serializing_if = "Option::is_none")]
    pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "DbtNtries", skip_serializing_if = "Option::is_none")]
    pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "FcstInd", skip_serializing_if = "Option::is_none")]
    pub fcst_ind: Option<bool>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: BankTransactionCodeStructure41,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice1>,
}

impl TotalsPerBankTransactionCode51 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nb_of_ntries {
            let pattern = Regex::new("[0-9]{1,15}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nb_of_ntries does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ttl_net_ntry {
            val.validate()?
        }
        if let Some(ref val) = self.cdt_ntries {
            val.validate()?
        }
        if let Some(ref val) = self.dbt_ntries {
            val.validate()?
        }
        self.bk_tx_cd.validate()?;
        if let Some(ref vec) = self.avlbty {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.dt {
            val.validate()?
        }
        Ok(())
    }
}

// TrackData11: Card track content or equivalent.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrackData11 {
    #[serde(rename = "TrckNb", skip_serializing_if = "Option::is_none")]
    pub trck_nb: Option<String>,
    #[serde(rename = "TrckVal")]
    pub trck_val: String,
}

impl TrackData11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.trck_nb {
            let pattern = Regex::new("[0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "trck_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if self.trck_val.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "trck_val is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.trck_val.chars().count() > 140 {
            return Err(ValidationError::new(
                1002,
                "trck_val exceeds the maximum length of 140".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.trck_val) {
            return Err(ValidationError::new(
                1005,
                "trck_val does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// TransactionAgents51: Proprietary agent related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionAgents51 {
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification64>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification63>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none")]
    pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none")]
    pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification63>,
    #[serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none")]
    pub issg_agt: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none")]
    pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification63>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryAgent41>>,
}

impl TransactionAgents51 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.instg_agt {
            val.validate()?
        }
        if let Some(ref val) = self.instd_agt {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.cdtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt1 {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt2 {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt3 {
            val.validate()?
        }
        if let Some(ref val) = self.rcvg_agt {
            val.validate()?
        }
        if let Some(ref val) = self.dlvrg_agt {
            val.validate()?
        }
        if let Some(ref val) = self.issg_agt {
            val.validate()?
        }
        if let Some(ref val) = self.sttlm_plc {
            val.validate()?
        }
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TransactionChannel1Code: Payment on television.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionChannel1Code {
    #[default]
    #[serde(rename = "MAIL")]
    CodeMAIL,
    #[serde(rename = "TLPH")]
    CodeTLPH,
    #[serde(rename = "ECOM")]
    CodeECOM,
    #[serde(rename = "TVPY")]
    CodeTVPY,
}

impl TransactionChannel1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// TransactionDates31: Proprietary date related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionDates31 {
    #[serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<String>,
    #[serde(
        rename = "TradActvtyCtrctlSttlmDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub trad_actvty_ctrctl_sttlm_dt: Option<String>,
    #[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<String>,
    #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<String>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<String>,
    #[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<String>,
    #[serde(rename = "TxDtTm", skip_serializing_if = "Option::is_none")]
    pub tx_dt_tm: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryDate31>>,
}

impl TransactionDates31 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.accptnc_dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "accptnc_dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.tx_dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tx_dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TransactionEnvironment1Code: Public environment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionEnvironment1Code {
    #[default]
    #[serde(rename = "MERC")]
    CodeMERC,
    #[serde(rename = "PRIV")]
    CodePRIV,
    #[serde(rename = "PUBL")]
    CodePUBL,
}

impl TransactionEnvironment1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// TransactionIdentifier11: Identification of the transaction that has to be unique for a time period.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionIdentifier11 {
    #[serde(rename = "TxDtTm")]
    pub tx_dt_tm: String,
    #[serde(rename = "TxRef")]
    pub tx_ref: String,
}

impl TransactionIdentifier11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.tx_dt_tm) {
            return Err(ValidationError::new(
                1005,
                "tx_dt_tm does not match the required pattern".to_string(),
            ));
        }
        if self.tx_ref.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "tx_ref is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.tx_ref.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "tx_ref exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.tx_ref) {
            return Err(ValidationError::new(
                1005,
                "tx_ref does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// TransactionInterest41: Individual interest record.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionInterest41 {
    #[serde(rename = "TtlIntrstAndTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
    pub rcrd: Option<Vec<InterestRecord21>>,
}

impl TransactionInterest41 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.ttl_intrst_and_tax_amt {
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

// TransactionParties61: Proprietary party related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionParties61 {
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<Party40Choice1>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice2>,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount382>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<Party40Choice1>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<Party40Choice1>,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount382>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<Party40Choice1>,
    #[serde(rename = "TradgPty", skip_serializing_if = "Option::is_none")]
    pub tradg_pty: Option<Party40Choice3>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryParty51>>,
}

impl TransactionParties61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.initg_pty {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_acct {
            val.validate()?
        }
        if let Some(ref val) = self.ultmt_dbtr {
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
        if let Some(ref val) = self.tradg_pty {
            val.validate()?
        }
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TransactionPrice4Choice1: Proprietary price specification related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionPrice4Choice1 {
    #[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
    pub deal_pric: Option<Price71>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryPrice21>>,
}

impl TransactionPrice4Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.deal_pric {
            val.validate()?
        }
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TransactionQuantities3Choice1: Proprietary quantities specification defined in the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionQuantities3Choice1 {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryQuantity11>,
}

impl TransactionQuantities3Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.qty {
            val.validate()?
        }
        if let Some(ref val) = self.orgnl_and_cur_face_amt {
            val.validate()?
        }
        if let Some(ref val) = self.prtry {
            val.validate()?
        }
        Ok(())
    }
}

// TransactionReferences61: Proprietary reference related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionReferences61 {
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<String>,
    #[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id: Option<String>,
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<String>,
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<String>,
    #[serde(rename = "ChqNb", skip_serializing_if = "Option::is_none")]
    pub chq_nb: Option<String>,
    #[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<String>,
    #[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_tx_id: Option<String>,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<String>,
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<String>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryReference11>>,
}

impl TransactionReferences61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.msg_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "msg_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "msg_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "msg_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "acct_svcr_ref does not match the required pattern".to_string(),
                ));
            }
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pmt_inf_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "instr_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "end_to_end_id does not match the required pattern".to_string(),
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "tx_id does not match the required pattern".to_string(),
                ));
            }
        }
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
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "mndt_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.chq_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "chq_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "chq_nb exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "chq_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.clr_sys_ref {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "clr_sys_ref is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "clr_sys_ref exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "clr_sys_ref does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.acct_ownr_tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "acct_ownr_tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "acct_ownr_tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "acct_ownr_tx_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.acct_svcr_tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "acct_svcr_tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "acct_svcr_tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "acct_svcr_tx_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.mkt_infrstrctr_tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "mkt_infrstrctr_tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "mkt_infrstrctr_tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "mkt_infrstrctr_tx_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prcg_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prcg_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prcg_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prcg_id does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// UnitOfMeasure1Code: Unit of measure equal to 4, 840 square yards.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum UnitOfMeasure1Code {
    #[default]
    #[serde(rename = "PIEC")]
    CodePIEC,
    #[serde(rename = "TONS")]
    CodeTONS,
    #[serde(rename = "FOOT")]
    CodeFOOT,
    #[serde(rename = "GBGA")]
    CodeGBGA,
    #[serde(rename = "USGA")]
    CodeUSGA,
    #[serde(rename = "GRAM")]
    CodeGRAM,
    #[serde(rename = "INCH")]
    CodeINCH,
    #[serde(rename = "KILO")]
    CodeKILO,
    #[serde(rename = "PUND")]
    CodePUND,
    #[serde(rename = "METR")]
    CodeMETR,
    #[serde(rename = "CMET")]
    CodeCMET,
    #[serde(rename = "MMET")]
    CodeMMET,
    #[serde(rename = "LITR")]
    CodeLITR,
    #[serde(rename = "CELI")]
    CodeCELI,
    #[serde(rename = "MILI")]
    CodeMILI,
    #[serde(rename = "GBOU")]
    CodeGBOU,
    #[serde(rename = "USOU")]
    CodeUSOU,
    #[serde(rename = "GBQA")]
    CodeGBQA,
    #[serde(rename = "USQA")]
    CodeUSQA,
    #[serde(rename = "GBPI")]
    CodeGBPI,
    #[serde(rename = "USPI")]
    CodeUSPI,
    #[serde(rename = "MILE")]
    CodeMILE,
    #[serde(rename = "KMET")]
    CodeKMET,
    #[serde(rename = "YARD")]
    CodeYARD,
    #[serde(rename = "SQKI")]
    CodeSQKI,
    #[serde(rename = "HECT")]
    CodeHECT,
    #[serde(rename = "ARES")]
    CodeARES,
    #[serde(rename = "SMET")]
    CodeSMET,
    #[serde(rename = "SCMT")]
    CodeSCMT,
    #[serde(rename = "SMIL")]
    CodeSMIL,
    #[serde(rename = "SQMI")]
    CodeSQMI,
    #[serde(rename = "SQYA")]
    CodeSQYA,
    #[serde(rename = "SQFO")]
    CodeSQFO,
    #[serde(rename = "SQIN")]
    CodeSQIN,
    #[serde(rename = "ACRE")]
    CodeACRE,
}

impl UnitOfMeasure1Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// UserInterface2Code: Cardholder display or interface.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserInterface2Code {
    #[default]
    #[serde(rename = "MDSP")]
    CodeMDSP,
    #[serde(rename = "CDSP")]
    CodeCDSP,
}

impl UserInterface2Code {
    pub fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// YieldedOrValueType1Choice: Type of value in which the price is expressed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct YieldedOrValueType1Choice {
    #[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
    pub yldd: Option<bool>,
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType1Code>,
}

impl YieldedOrValueType1Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.val_tp {
            val.validate()?
        }
        Ok(())
    }
}
