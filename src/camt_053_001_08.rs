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

// AccountStatement9 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountStatement9 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "StmtPgntn", skip_serializing_if = "Option::is_none")]
    pub stmt_pgntn: Option<Pagination1>,
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
    #[serde(rename = "Bal")]
    pub bal: Vec<CashBalance8>,
    #[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
    pub txs_summry: Option<TotalTransactions6>,
    #[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
    pub ntry: Option<Vec<ReportEntry10>>,
    #[serde(rename = "AddtlStmtInf", skip_serializing_if = "Option::is_none")]
    pub addtl_stmt_inf: Option<String>,
}

impl AccountStatement9 {
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
        if let Some(ref val) = self.stmt_pgntn {
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
        for item in &self.bal {
            item.validate()?
        }
        if let Some(ref val) = self.txs_summry {
            val.validate()?
        }
        if let Some(ref vec) = self.ntry {
            for item in vec {
                item.validate()?
            }
        }
        if let Some(ref val) = self.addtl_stmt_inf {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "addtl_stmt_inf is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 500 {
                return Err(ValidationError::new(
                    1002,
                    "addtl_stmt_inf exceeds the maximum length of 500".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// BankToCustomerStatementV08 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankToCustomerStatementV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader81,
    #[serde(rename = "Stmt")]
    pub stmt: Vec<AccountStatement9>,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BankToCustomerStatementV08 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        for item in &self.stmt {
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
