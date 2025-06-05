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

// AccountInterest4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountInterest4 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestType1Choice>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<Vec<Rate4>>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod1>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<String>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges2>,
}

impl AccountInterest4 {
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
        }
        if let Some(ref val) = self.tax {
            val.validate()?
        }
        Ok(())
    }
}

// AccountReport25 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountReport25 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "RptPgntn", skip_serializing_if = "Option::is_none")]
    pub rpt_pgntn: Option<Pagination1>,
    #[serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none")]
    pub elctrnc_seq_nb: Option<f64>,
    #[serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none")]
    pub rptg_seq: Option<SequenceRange1Choice>,
    #[serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none")]
    pub lgl_seq_nb: Option<f64>,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<String>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod1>,
    #[serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
    #[serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none")]
    pub rptg_src: Option<ReportingSource1Choice>,
    #[serde(rename = "Acct")]
    pub acct: CashAccount39,
    #[serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none")]
    pub rltd_acct: Option<CashAccount38>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<Vec<AccountInterest4>>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<Vec<CashBalance8>>,
    #[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
    pub txs_summry: Option<TotalTransactions6>,
    #[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
    pub ntry: Option<Vec<ReportEntry10>>,
    #[serde(rename = "AddtlRptInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rpt_inf: Option<String>,
}

impl AccountReport25 {
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
        if let Some(ref val) = self.rpt_pgntn {
            val.validate()?
        }
        if let Some(ref val) = self.rptg_seq {
            val.validate()?
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

// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
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

// ActiveOrHistoricCurrencyAndAmountRange2 ...
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

// AmountAndCurrencyExchange3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchange3 {
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<AmountAndCurrencyExchangeDetails3>,
    #[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<AmountAndCurrencyExchangeDetails3>,
    #[serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none")]
    pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails3>,
    #[serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none")]
    pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails3>,
    #[serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none")]
    pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails4>>,
}

impl AmountAndCurrencyExchange3 {
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

// AmountAndCurrencyExchangeDetails3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchangeDetails3 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.amt.validate()?;
        if let Some(ref val) = self.ccy_xchg {
            val.validate()?
        }
        Ok(())
    }
}

// AmountAndCurrencyExchangeDetails4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountAndCurrencyExchangeDetails4 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails4 {
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
        self.amt.validate()?;
        if let Some(ref val) = self.ccy_xchg {
            val.validate()?
        }
        Ok(())
    }
}

// AmountAndDirection35 ...
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

// AmountRangeBoundary1 ...
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

// AttendanceContext1Code ...
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

// AuthenticationEntity1Code ...
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

// AuthenticationMethod1Code ...
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

// BalanceSubType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BalanceSubType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl BalanceSubType1Choice {
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

// BalanceType10Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BalanceType10Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl BalanceType10Choice {
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

// BalanceType13 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BalanceType13 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: BalanceType10Choice,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<BalanceSubType1Choice>,
}

impl BalanceType13 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.cd_or_prtry.validate()?;
        if let Some(ref val) = self.sub_tp {
            val.validate()?
        }
        Ok(())
    }
}

// BankToCustomerAccountReportV08 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankToCustomerAccountReportV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader81,
    #[serde(rename = "Rpt")]
    pub rpt: Vec<AccountReport25>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BankToCustomerAccountReportV08 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        for item in &self.rpt {
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

// BankTransactionCodeStructure4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankTransactionCodeStructure4 {
    #[serde(rename = "Domn", skip_serializing_if = "Option::is_none")]
    pub domn: Option<BankTransactionCodeStructure5>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}

impl BankTransactionCodeStructure4 {
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

// BankTransactionCodeStructure5 ...
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

// BankTransactionCodeStructure6 ...
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

// BatchInformation2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BatchInformation2 {
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

impl BatchInformation2 {
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

// CSCManagement1Code ...
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

// CardAggregated2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardAggregated2 {
    #[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
    pub addtl_svc: Option<CardPaymentServiceType2Code>,
    #[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
    pub tx_ctgy: Option<String>,
    #[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id: Option<String>,
    #[serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none")]
    pub seq_nb_rg: Option<CardSequenceNumberRange1>,
    #[serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none")]
    pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
}

impl CardAggregated2 {
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

// CardDataReading1Code ...
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

// CardEntry4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardEntry4 {
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard4>,
    #[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction1>,
    #[serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none")]
    pub aggtd_ntry: Option<CardAggregated2>,
    #[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount38>,
}

impl CardEntry4 {
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

// CardIndividualTransaction2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardIndividualTransaction2 {
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
    pub tx_id: Option<TransactionIdentifier1>,
    #[serde(rename = "Pdct", skip_serializing_if = "Option::is_none")]
    pub pdct: Option<Product2>,
    #[serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none")]
    pub vldtn_dt: Option<String>,
    #[serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none")]
    pub vldtn_seq_nb: Option<String>,
}

impl CardIndividualTransaction2 {
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
        }
        Ok(())
    }
}

// CardPaymentServiceType2Code ...
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

// CardSecurityInformation1 ...
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

// CardSequenceNumberRange1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardSequenceNumberRange1 {
    #[serde(rename = "FrstTx", skip_serializing_if = "Option::is_none")]
    pub frst_tx: Option<String>,
    #[serde(rename = "LastTx", skip_serializing_if = "Option::is_none")]
    pub last_tx: Option<String>,
}

impl CardSequenceNumberRange1 {
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
        }
        Ok(())
    }
}

// CardTransaction17 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardTransaction17 {
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard4>,
    #[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction1>,
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<CardTransaction3Choice>,
    #[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount38>,
}

impl CardTransaction17 {
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

// CardTransaction3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardTransaction3Choice {
    #[serde(rename = "Aggtd", skip_serializing_if = "Option::is_none")]
    pub aggtd: Option<CardAggregated2>,
    #[serde(rename = "Indv", skip_serializing_if = "Option::is_none")]
    pub indv: Option<CardIndividualTransaction2>,
}

impl CardTransaction3Choice {
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

// CardholderAuthentication2 ...
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

// CardholderVerificationCapability1Code ...
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

// CashAccount39 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount39 {
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
    #[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
    pub ownr: Option<PartyIdentification135>,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl CashAccount39 {
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
        if let Some(ref val) = self.ownr {
            val.validate()?
        }
        if let Some(ref val) = self.svcr {
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

// CashAvailability1 ...
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

// CashAvailabilityDate1Choice ...
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

// CashBalance8 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashBalance8 {
    #[serde(rename = "Tp")]
    pub tp: BalanceType13,
    #[serde(rename = "CdtLine", skip_serializing_if = "Option::is_none")]
    pub cdt_line: Option<Vec<CreditLine3>>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "Dt")]
    pub dt: DateAndDateTime2Choice,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
}

impl CashBalance8 {
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

// CashDeposit1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashDeposit1 {
    #[serde(rename = "NoteDnmtn")]
    pub note_dnmtn: ActiveCurrencyAndAmount,
    #[serde(rename = "NbOfNotes")]
    pub nb_of_notes: String,
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}

impl CashDeposit1 {
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

// CopyDuplicate1Code ...
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

// CorporateAction9 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorporateAction9 {
    #[serde(rename = "EvtTp")]
    pub evt_tp: String,
    #[serde(rename = "EvtId")]
    pub evt_id: String,
}

impl CorporateAction9 {
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

// CreditLine3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditLine3 {
    #[serde(rename = "Incl")]
    pub incl: bool,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditLineType1Choice>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
}

impl CreditLine3 {
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

// CreditLineType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditLineType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl CreditLineType1Choice {
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

// CurrencyExchange5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CurrencyExchange5 {
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

impl CurrencyExchange5 {
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

// DateOrDateTimePeriod1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateOrDateTimePeriod1Choice {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DatePeriod2>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<DateTimePeriod1>,
}

impl DateOrDateTimePeriod1Choice {
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

// DateTimePeriod1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateTimePeriod1 {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: String,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: String,
}

impl DateTimePeriod1 {
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

// DisplayCapabilities1 ...
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

// EntryDetails9 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryDetails9 {
    #[serde(rename = "Btch", skip_serializing_if = "Option::is_none")]
    pub btch: Option<BatchInformation2>,
    #[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
    pub tx_dtls: Option<Vec<EntryTransaction10>>,
}

impl EntryDetails9 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.btch {
            val.validate()?
        }
        if let Some(ref vec) = self.tx_dtls {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// EntryStatus1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryStatus1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl EntryStatus1Choice {
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

// EntryTransaction10 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryTransaction10 {
    #[serde(rename = "Refs", skip_serializing_if = "Option::is_none")]
    pub refs: Option<TransactionReferences6>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<AmountAndCurrencyExchange3>,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
    #[serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none")]
    pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Charges6>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<TransactionInterest4>,
    #[serde(rename = "RltdPties", skip_serializing_if = "Option::is_none")]
    pub rltd_pties: Option<TransactionParties6>,
    #[serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none")]
    pub rltd_agts: Option<TransactionAgents5>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice>,
    #[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
    pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation16>,
    #[serde(rename = "RltdDts", skip_serializing_if = "Option::is_none")]
    pub rltd_dts: Option<TransactionDates3>,
    #[serde(rename = "RltdPric", skip_serializing_if = "Option::is_none")]
    pub rltd_pric: Option<TransactionPrice4Choice>,
    #[serde(rename = "RltdQties", skip_serializing_if = "Option::is_none")]
    pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
    #[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification19>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInformation8>,
    #[serde(rename = "RtrInf", skip_serializing_if = "Option::is_none")]
    pub rtr_inf: Option<PaymentReturnReason5>,
    #[serde(rename = "CorpActn", skip_serializing_if = "Option::is_none")]
    pub corp_actn: Option<CorporateAction9>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "CshDpst", skip_serializing_if = "Option::is_none")]
    pub csh_dpst: Option<Vec<CashDeposit1>>,
    #[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardTransaction17>,
    #[serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none")]
    pub addtl_tx_inf: Option<String>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl EntryTransaction10 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.refs {
            val.validate()?
        }
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate()?
        }
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
        }
        if let Some(ref vec) = self.splmtry_data {
            for item in vec {
                item.validate()?
            }
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

// FinancialInstrumentQuantity1Choice ...
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

// FromToAmountRange1 ...
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

// GenericIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<String>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl GenericIdentification1 {
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

// GenericIdentification32 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification32 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType3Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType4Code>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<String>,
}

impl GenericIdentification32 {
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

// GroupHeader81 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader81 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none")]
    pub msg_rcpt: Option<PartyIdentification135>,
    #[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
    pub msg_pgntn: Option<Pagination1>,
    #[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
    pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<String>,
}

impl GroupHeader81 {
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
        if let Some(ref val) = self.msg_rcpt {
            val.validate()?
        }
        if let Some(ref val) = self.msg_pgntn {
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
        }
        Ok(())
    }
}

// IdentificationSource3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct IdentificationSource3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl IdentificationSource3Choice {
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

// ImpliedCurrencyAmountRange1Choice ...
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

// InterestRecord2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InterestRecord2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestType1Choice>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate4>,
    #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod1>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<String>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges2>,
}

impl InterestRecord2 {
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
        }
        if let Some(ref val) = self.tax {
            val.validate()?
        }
        Ok(())
    }
}

// InterestType1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InterestType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InterestType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl InterestType1Choice {
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

// InterestType1Code ...
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

// MessageIdentification2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageIdentification2 {
    #[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<String>,
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
}

impl MessageIdentification2 {
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
        }
        Ok(())
    }
}

// NameAndAddress16 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NameAndAddress16 {
    #[serde(rename = "Nm")]
    pub nm: String,
    #[serde(rename = "Adr")]
    pub adr: PostalAddress24,
}

impl NameAndAddress16 {
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
        self.adr.validate()?;
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

// NumberAndSumOfTransactions1 ...
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

// NumberAndSumOfTransactions4 ...
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

// OnLineCapability1Code ...
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

// OriginalAndCurrentQuantities1 ...
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

// OriginalBusinessQuery1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalBusinessQuery1 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<String>,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
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

// OtherIdentification1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OtherIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<String>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
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
        }
        self.tp.validate()?;
        Ok(())
    }
}

// POIComponentType1Code ...
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

// Pagination1 ...
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

// PartyType3Code ...
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

// PartyType4Code ...
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

// PaymentCard4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentCard4 {
    #[serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none")]
    pub plain_card_data: Option<PlainCardData1>,
    #[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<String>,
    #[serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none")]
    pub card_brnd: Option<GenericIdentification1>,
    #[serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none")]
    pub addtl_card_data: Option<String>,
}

impl PaymentCard4 {
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
        }
        Ok(())
    }
}

// PaymentContext3 ...
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

// PaymentReturnReason5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentReturnReason5 {
    #[serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none")]
    pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification135>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<ReturnReason5Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl PaymentReturnReason5 {
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
            }
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

// PlainCardData1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlainCardData1 {
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
    pub trck_data: Option<Vec<TrackData1>>,
    #[serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none")]
    pub card_scty_cd: Option<CardSecurityInformation1>,
}

impl PlainCardData1 {
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

// PointOfInteraction1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteraction1 {
    #[serde(rename = "Id")]
    pub id: GenericIdentification32,
    #[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
    pub sys_nm: Option<String>,
    #[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
    pub grp_id: Option<String>,
    #[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<PointOfInteractionCapabilities1>,
    #[serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none")]
    pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
}

impl PointOfInteraction1 {
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

// PointOfInteractionCapabilities1 ...
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

// PointOfInteractionComponent1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteractionComponent1 {
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

impl PointOfInteractionComponent1 {
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

// Price7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Price7 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType1Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice,
}

impl Price7 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        self.val.validate()?;
        Ok(())
    }
}

// PriceRateOrAmount3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PriceRateOrAmount3Choice {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl PriceRateOrAmount3Choice {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.amt {
            val.validate()?
        }
        Ok(())
    }
}

// PriceValueType1Code ...
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

// Product2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Product2 {
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

impl Product2 {
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
        }
        Ok(())
    }
}

// ProprietaryAgent4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryAgent4 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification6,
}

impl ProprietaryAgent4 {
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
        self.agt.validate()?;
        Ok(())
    }
}

// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryBankTransactionCodeStructure1 {
    #[serde(rename = "Cd")]
    pub cd: String,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl ProprietaryBankTransactionCodeStructure1 {
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

// ProprietaryDate3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryDate3 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Dt")]
    pub dt: DateAndDateTime2Choice,
}

impl ProprietaryDate3 {
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
        self.dt.validate()?;
        Ok(())
    }
}

// ProprietaryParty5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryParty5 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Pty")]
    pub pty: Party40Choice,
}

impl ProprietaryParty5 {
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
        self.pty.validate()?;
        Ok(())
    }
}

// ProprietaryPrice2 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryPrice2 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Pric")]
    pub pric: ActiveOrHistoricCurrencyAndAmount,
}

impl ProprietaryPrice2 {
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
        self.pric.validate()?;
        Ok(())
    }
}

// ProprietaryQuantity1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryQuantity1 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Qty")]
    pub qty: String,
}

impl ProprietaryQuantity1 {
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
        Ok(())
    }
}

// ProprietaryReference1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryReference1 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Ref")]
    pub ref_attr: String,
}

impl ProprietaryReference1 {
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

// Rate4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Rate4 {
    #[serde(rename = "Tp")]
    pub tp: RateType4Choice,
    #[serde(rename = "VldtyRg", skip_serializing_if = "Option::is_none")]
    pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}

impl Rate4 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.tp.validate()?;
        if let Some(ref val) = self.vldty_rg {
            val.validate()?
        }
        Ok(())
    }
}

// RateType4Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RateType4Choice {
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<f64>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<String>,
}

impl RateType4Choice {
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

// RemittanceLocation7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceLocation7 {
    #[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
    pub rmt_id: Option<String>,
    #[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
    pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData1>>,
}

impl RemittanceLocation7 {
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
        }
        if let Some(ref vec) = self.rmt_lctn_dtls {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// RemittanceLocationData1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceLocationData1 {
    #[serde(rename = "Mtd")]
    pub mtd: RemittanceLocationMethod2Code,
    #[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
    pub elctrnc_adr: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<NameAndAddress16>,
}

impl RemittanceLocationData1 {
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
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        Ok(())
    }
}

// RemittanceLocationMethod2Code ...
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

// ReportEntry10 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReportEntry10 {
    #[serde(rename = "NtryRef", skip_serializing_if = "Option::is_none")]
    pub ntry_ref: Option<String>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "RvslInd", skip_serializing_if = "Option::is_none")]
    pub rvsl_ind: Option<bool>,
    #[serde(rename = "Sts")]
    pub sts: EntryStatus1Choice,
    #[serde(rename = "BookgDt", skip_serializing_if = "Option::is_none")]
    pub bookg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<String>,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: BankTransactionCodeStructure4,
    #[serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none")]
    pub comssn_wvr_ind: Option<bool>,
    #[serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none")]
    pub addtl_inf_ind: Option<MessageIdentification2>,
    #[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<AmountAndCurrencyExchange3>,
    #[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Charges6>,
    #[serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none")]
    pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
    #[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
    pub intrst: Option<TransactionInterest4>,
    #[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardEntry4>,
    #[serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none")]
    pub ntry_dtls: Option<Vec<EntryDetails9>>,
    #[serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ntry_inf: Option<String>,
}

impl ReportEntry10 {
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
        }
        self.amt.validate()?;
        self.cdt_dbt_ind.validate()?;
        self.sts.validate()?;
        if let Some(ref val) = self.bookg_dt {
            val.validate()?
        }
        if let Some(ref val) = self.val_dt {
            val.validate()?
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
        }
        Ok(())
    }
}

// ReportingSource1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReportingSource1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ReportingSource1Choice {
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

// ReturnReason5Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReturnReason5Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl ReturnReason5Choice {
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

// SecuritiesAccount19 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SecuritiesAccount19 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl SecuritiesAccount19 {
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
        }
        Ok(())
    }
}

// SecurityIdentification19 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<Vec<OtherIdentification1>>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

impl SecurityIdentification19 {
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
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "desc exceeds the maximum length of 140".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// SequenceRange1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SequenceRange1 {
    #[serde(rename = "FrSeq")]
    pub fr_seq: String,
    #[serde(rename = "ToSeq")]
    pub to_seq: String,
}

impl SequenceRange1 {
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
        Ok(())
    }
}

// SequenceRange1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SequenceRange1Choice {
    #[serde(rename = "FrSeq", skip_serializing_if = "Option::is_none")]
    pub fr_seq: Option<String>,
    #[serde(rename = "ToSeq", skip_serializing_if = "Option::is_none")]
    pub to_seq: Option<String>,
    #[serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none")]
    pub fr_to_seq: Option<Vec<SequenceRange1>>,
    #[serde(rename = "EQSeq", skip_serializing_if = "Option::is_none")]
    pub eq_seq: Option<Vec<String>>,
    #[serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none")]
    pub neq_seq: Option<Vec<String>>,
}

impl SequenceRange1Choice {
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
            }
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

// TaxInformation8 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxInformation8 {
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<TaxParty1>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<TaxParty2>,
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

impl TaxInformation8 {
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

// TechnicalInputChannel1Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TechnicalInputChannel1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl TechnicalInputChannel1Choice {
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

// TotalTransactions6 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TotalTransactions6 {
    #[serde(rename = "TtlNtries", skip_serializing_if = "Option::is_none")]
    pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
    #[serde(rename = "TtlCdtNtries", skip_serializing_if = "Option::is_none")]
    pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "TtlDbtNtries", skip_serializing_if = "Option::is_none")]
    pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "TtlNtriesPerBkTxCd", skip_serializing_if = "Option::is_none")]
    pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode5>>,
}

impl TotalTransactions6 {
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

// TotalsPerBankTransactionCode5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TotalsPerBankTransactionCode5 {
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
    pub bk_tx_cd: BankTransactionCodeStructure4,
    #[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
    pub avlbty: Option<Vec<CashAvailability1>>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
}

impl TotalsPerBankTransactionCode5 {
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

// TrackData1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrackData1 {
    #[serde(rename = "TrckNb", skip_serializing_if = "Option::is_none")]
    pub trck_nb: Option<String>,
    #[serde(rename = "TrckVal")]
    pub trck_val: String,
}

impl TrackData1 {
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
        Ok(())
    }
}

// TransactionAgents5 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionAgents5 {
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none")]
    pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none")]
    pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none")]
    pub issg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none")]
    pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryAgent4>>,
}

impl TransactionAgents5 {
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

// TransactionChannel1Code ...
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

// TransactionDates3 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionDates3 {
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
    pub prtry: Option<Vec<ProprietaryDate3>>,
}

impl TransactionDates3 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// TransactionEnvironment1Code ...
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

// TransactionIdentifier1 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionIdentifier1 {
    #[serde(rename = "TxDtTm")]
    pub tx_dt_tm: String,
    #[serde(rename = "TxRef")]
    pub tx_ref: String,
}

impl TransactionIdentifier1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
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
        Ok(())
    }
}

// TransactionInterest4 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionInterest4 {
    #[serde(rename = "TtlIntrstAndTaxAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
    pub rcrd: Option<Vec<InterestRecord2>>,
}

impl TransactionInterest4 {
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

// TransactionParties6 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionParties6 {
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<Party40Choice>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice>,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount38>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<Party40Choice>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<Party40Choice>,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount38>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<Party40Choice>,
    #[serde(rename = "TradgPty", skip_serializing_if = "Option::is_none")]
    pub tradg_pty: Option<Party40Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryParty5>>,
}

impl TransactionParties6 {
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

// TransactionPrice4Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionPrice4Choice {
    #[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
    pub deal_pric: Option<Price7>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryPrice2>>,
}

impl TransactionPrice4Choice {
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

// TransactionQuantities3Choice ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionQuantities3Choice {
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryQuantity1>,
}

impl TransactionQuantities3Choice {
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

// TransactionReferences6 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionReferences6 {
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
    pub prtry: Option<Vec<ProprietaryReference1>>,
}

impl TransactionReferences6 {
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
        }
        if let Some(ref vec) = self.prtry {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}

// UnitOfMeasure1Code ...
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

// UserInterface2Code ...
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

// YieldedOrValueType1Choice ...
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
