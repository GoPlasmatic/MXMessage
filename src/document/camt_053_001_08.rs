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
use crate::parse_result::{ErrorCollector, ParserConfig};
use crate::validation::{Validate, helpers};
use serde::{Deserialize, Serialize};

// AccountIdentification4Choice1: Unique identification of an account, as assigned by the account servicer, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountIdentification4Choice1 {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification11>,
}

impl Validate for AccountIdentification4Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.iban {
            helpers::validate_pattern(
                val,
                "IBAN",
                "[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}",
                &helpers::child_path(path, "IBAN"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.othr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Othr"), config, collector);
        }
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

impl Validate for AccountInterest41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref vec) = self.rate
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Rate"), config, collector);
            }
        }
        if let Some(ref val) = self.fr_to_dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrToDt"), config, collector);
        }
        if let Some(ref val) = self.rsn {
            helpers::validate_length(
                val,
                "Rsn",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Rsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rsn {
            helpers::validate_pattern(
                val,
                "Rsn",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Rsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tax"), config, collector);
        }
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

impl Validate for AccountSchemeName1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
    }
}

// AccountStatement91: Further details of the account statement.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountStatement91 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "StmtPgntn")]
    pub stmt_pgntn: Pagination1,
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
    #[serde(rename = "Bal")]
    pub bal: Vec<CashBalance81>,
    #[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
    pub txs_summry: Option<TotalTransactions61>,
    #[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
    pub ntry: Option<Vec<Box<ReportEntry101>>>,
    #[serde(rename = "AddtlStmtInf", skip_serializing_if = "Option::is_none")]
    pub addtl_stmt_inf: Option<String>,
}

impl Validate for AccountStatement91 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        self.stmt_pgntn
            .validate(&helpers::child_path(path, "StmtPgntn"), config, collector);
        if let Some(ref val) = self.rptg_seq
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RptgSeq"), config, collector);
        }
        if let Some(ref val) = self.cre_dt_tm {
            helpers::validate_pattern(
                val,
                "CreDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "CreDtTm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.fr_to_dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrToDt"), config, collector);
        }
        if let Some(ref val) = self.cpy_dplct_ind
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CpyDplctInd"), config, collector);
        }
        if let Some(ref val) = self.rptg_src
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RptgSrc"), config, collector);
        }
        self.acct
            .validate(&helpers::child_path(path, "Acct"), config, collector);
        if let Some(ref val) = self.rltd_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RltdAcct"), config, collector);
        }
        if let Some(ref vec) = self.intrst
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Intrst"), config, collector);
            }
        }
        for item in &self.bal {
            item.validate(&helpers::child_path(path, "Bal"), config, collector);
        }
        if let Some(ref val) = self.txs_summry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxsSummry"), config, collector);
        }
        if let Some(ref vec) = self.ntry
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Ntry"), config, collector);
            }
        }
        if let Some(ref val) = self.addtl_stmt_inf {
            helpers::validate_length(
                val,
                "AddtlStmtInf",
                Some(1),
                Some(500),
                &helpers::child_path(path, "AddtlStmtInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_stmt_inf {
            helpers::validate_pattern(
                val,
                "AddtlStmtInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AddtlStmtInf"),
                config,
                collector,
            );
        }
    }
}

// ActiveCurrencyAndAmount: A number of monetary units specified in an active currency where the unit of currency is explicit and compliant with ISO 4217.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl Validate for ActiveCurrencyAndAmount {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// ActiveOrHistoricCurrencyAnd13DecimalAmount: A number of monetary units specified in an active or a historic currency where the unit of currency is explicit and compliant with ISO 4217. The number of fractional digits (or minor unit of currency) is not checked as per ISO 4217: It must be lesser than or equal to 13.
// Note: The decimal separator is a dot.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl Validate for ActiveOrHistoricCurrencyAnd13DecimalAmount {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// ActiveOrHistoricCurrencyAndAmount: A number of monetary units specified in an active or a historic currency where the unit of currency is explicit and compliant with ISO 4217.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl Validate for ActiveOrHistoricCurrencyAndAmount {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
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

impl Validate for ActiveOrHistoricCurrencyAndAmountRange2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.cdt_dbt_ind
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        }
        helpers::validate_pattern(
            &self.ccy,
            "Ccy",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "Ccy"),
            config,
            collector,
        );
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

impl Validate for AddressType2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for AddressType3Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
        if let Some(ref val) = self.prtry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prtry"), config, collector);
        }
    }
}

// AddressType3Choice2: Type of address expressed as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddressType3Choice2 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification302>,
}

impl Validate for AddressType3Choice2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
        if let Some(ref val) = self.prtry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prtry"), config, collector);
        }
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

impl Validate for AmountAndCurrencyExchange31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instd_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InstdAmt"), config, collector);
        }
        if let Some(ref val) = self.tx_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxAmt"), config, collector);
        }
        if let Some(ref val) = self.cntr_val_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CntrValAmt"), config, collector);
        }
        if let Some(ref val) = self.anncd_pstng_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "AnncdPstngAmt"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.prtry_amt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "PrtryAmt"), config, collector);
            }
        }
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

impl Validate for AmountAndCurrencyExchange32 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instd_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InstdAmt"), config, collector);
        }
        if let Some(ref val) = self.tx_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxAmt"), config, collector);
        }
        if let Some(ref val) = self.cntr_val_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CntrValAmt"), config, collector);
        }
        if let Some(ref val) = self.anncd_pstng_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "AnncdPstngAmt"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.prtry_amt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "PrtryAmt"), config, collector);
            }
        }
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

impl Validate for AmountAndCurrencyExchangeDetails31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.ccy_xchg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CcyXchg"), config, collector);
        }
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

impl Validate for AmountAndCurrencyExchangeDetails32 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.ccy_xchg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CcyXchg"), config, collector);
        }
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

impl Validate for AmountAndCurrencyExchangeDetails41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.ccy_xchg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CcyXchg"), config, collector);
        }
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
    pub ccy_xchg: Option<CurrencyExchange5>,
}

impl Validate for AmountAndCurrencyExchangeDetails42 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.ccy_xchg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CcyXchg"), config, collector);
        }
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

impl Validate for AmountAndDirection35 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
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

impl Validate for AmountRangeBoundary1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
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

impl Validate for AttendanceContext1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for AuthenticationEntity1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for AuthenticationMethod1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for BalanceSubType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for BalanceType10Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for BalanceType131 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.cd_or_prtry
            .validate(&helpers::child_path(path, "CdOrPrtry"), config, collector);
        if let Some(ref val) = self.sub_tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SubTp"), config, collector);
        }
    }
}

// BankToCustomerStatementV08: Reports on booked entries and balances for a cash account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BankToCustomerStatementV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader811,
    #[serde(rename = "Stmt")]
    pub stmt: AccountStatement91,
}

impl Validate for BankToCustomerStatementV08 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.stmt
            .validate(&helpers::child_path(path, "Stmt"), config, collector);
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

impl Validate for BankTransactionCodeStructure41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.domn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Domn"), config, collector);
        }
        if let Some(ref val) = self.prtry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prtry"), config, collector);
        }
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

impl Validate for BankTransactionCodeStructure5 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.cd,
            "Cd",
            Some(1),
            Some(4),
            &helpers::child_path(path, "Cd"),
            config,
            collector,
        );
        self.fmly
            .validate(&helpers::child_path(path, "Fmly"), config, collector);
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

impl Validate for BankTransactionCodeStructure6 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.cd,
            "Cd",
            Some(1),
            Some(4),
            &helpers::child_path(path, "Cd"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.sub_fmly_cd,
            "SubFmlyCd",
            Some(1),
            Some(4),
            &helpers::child_path(path, "SubFmlyCd"),
            config,
            collector,
        );
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

impl Validate for BatchInformation21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.msg_id {
            helpers::validate_length(
                val,
                "MsgId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MsgId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.msg_id {
            helpers::validate_pattern(
                val,
                "MsgId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MsgId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pmt_inf_id {
            helpers::validate_length(
                val,
                "PmtInfId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "PmtInfId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pmt_inf_id {
            helpers::validate_pattern(
                val,
                "PmtInfId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "PmtInfId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nb_of_txs {
            helpers::validate_pattern(
                val,
                "NbOfTxs",
                "[0-9]{1,15}",
                &helpers::child_path(path, "NbOfTxs"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlAmt"), config, collector);
        }
        if let Some(ref val) = self.cdt_dbt_ind
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        }
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

impl Validate for BranchAndFinancialInstitutionIdentification61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
        if let Some(ref val) = self.brnch_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "BrnchId"), config, collector);
        }
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

impl Validate for BranchAndFinancialInstitutionIdentification62 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
        if let Some(ref val) = self.brnch_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "BrnchId"), config, collector);
        }
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

impl Validate for BranchData31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.id {
            helpers::validate_length(
                val,
                "Id",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.id {
            helpers::validate_pattern(
                val,
                "Id",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.lei {
            helpers::validate_pattern(
                val,
                "LEI",
                "[A-Z0-9]{18,18}[0-9]{2,2}",
                &helpers::child_path(path, "LEI"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
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

impl Validate for BranchData32 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.id {
            helpers::validate_length(
                val,
                "Id",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.id {
            helpers::validate_pattern(
                val,
                "Id",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.lei {
            helpers::validate_pattern(
                val,
                "LEI",
                "[A-Z0-9]{18,18}[0-9]{2,2}",
                &helpers::child_path(path, "LEI"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
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

impl Validate for CSCManagement1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for CardAggregated21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.addtl_svc
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AddtlSvc"), config, collector);
        }
        if let Some(ref val) = self.tx_ctgy {
            helpers::validate_length(
                val,
                "TxCtgy",
                Some(1),
                Some(4),
                &helpers::child_path(path, "TxCtgy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            helpers::validate_length(
                val,
                "SaleRcncltnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SaleRcncltnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            helpers::validate_pattern(
                val,
                "SaleRcncltnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SaleRcncltnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.seq_nb_rg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SeqNbRg"), config, collector);
        }
        if let Some(ref val) = self.tx_dt_rg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxDtRg"), config, collector);
        }
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

impl Validate for CardDataReading1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for CardEntry41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.card
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Card"), config, collector);
        }
        if let Some(ref val) = self.poi
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "POI"), config, collector);
        }
        if let Some(ref val) = self.aggtd_ntry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AggtdNtry"), config, collector);
        }
        if let Some(ref val) = self.pre_pd_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PrePdAcct"), config, collector);
        }
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

impl Validate for CardIndividualTransaction21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.icc_rltd_data {
            helpers::validate_length(
                val,
                "ICCRltdData",
                Some(1),
                Some(1025),
                &helpers::child_path(path, "ICCRltdData"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.icc_rltd_data {
            helpers::validate_pattern(
                val,
                "ICCRltdData",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ICCRltdData"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pmt_cntxt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PmtCntxt"), config, collector);
        }
        if let Some(ref val) = self.addtl_svc
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AddtlSvc"), config, collector);
        }
        if let Some(ref val) = self.tx_ctgy {
            helpers::validate_length(
                val,
                "TxCtgy",
                Some(1),
                Some(4),
                &helpers::child_path(path, "TxCtgy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            helpers::validate_length(
                val,
                "SaleRcncltnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SaleRcncltnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            helpers::validate_pattern(
                val,
                "SaleRcncltnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SaleRcncltnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sale_ref_nb {
            helpers::validate_length(
                val,
                "SaleRefNb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SaleRefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sale_ref_nb {
            helpers::validate_pattern(
                val,
                "SaleRefNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SaleRefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.re_presntmnt_rsn {
            helpers::validate_length(
                val,
                "RePresntmntRsn",
                Some(1),
                Some(4),
                &helpers::child_path(path, "RePresntmntRsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.seq_nb {
            helpers::validate_length(
                val,
                "SeqNb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SeqNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.seq_nb {
            helpers::validate_pattern(
                val,
                "SeqNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SeqNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tx_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxId"), config, collector);
        }
        if let Some(ref val) = self.pdct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Pdct"), config, collector);
        }
        if let Some(ref val) = self.vldtn_seq_nb {
            helpers::validate_length(
                val,
                "VldtnSeqNb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "VldtnSeqNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.vldtn_seq_nb {
            helpers::validate_pattern(
                val,
                "VldtnSeqNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "VldtnSeqNb"),
                config,
                collector,
            );
        }
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

impl Validate for CardPaymentServiceType2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for CardSecurityInformation1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.csc_mgmt
            .validate(&helpers::child_path(path, "CSCMgmt"), config, collector);
        if let Some(ref val) = self.csc_val {
            helpers::validate_pattern(
                val,
                "CSCVal",
                "[0-9]{3,4}",
                &helpers::child_path(path, "CSCVal"),
                config,
                collector,
            );
        }
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

impl Validate for CardSequenceNumberRange11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.frst_tx {
            helpers::validate_length(
                val,
                "FrstTx",
                Some(1),
                Some(35),
                &helpers::child_path(path, "FrstTx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.frst_tx {
            helpers::validate_pattern(
                val,
                "FrstTx",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "FrstTx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.last_tx {
            helpers::validate_length(
                val,
                "LastTx",
                Some(1),
                Some(35),
                &helpers::child_path(path, "LastTx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.last_tx {
            helpers::validate_pattern(
                val,
                "LastTx",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "LastTx"),
                config,
                collector,
            );
        }
    }
}

// CardTransaction171: Prepaid account for the transfer or loading of an amount of money.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardTransaction171 {
    #[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard41>,
    #[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction12>,
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<CardTransaction3Choice1>,
    #[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount382>,
}

impl Validate for CardTransaction171 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.card
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Card"), config, collector);
        }
        if let Some(ref val) = self.poi
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "POI"), config, collector);
        }
        if let Some(ref val) = self.tx
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tx"), config, collector);
        }
        if let Some(ref val) = self.pre_pd_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PrePdAcct"), config, collector);
        }
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

impl Validate for CardTransaction3Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.aggtd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Aggtd"), config, collector);
        }
        if let Some(ref val) = self.indv
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Indv"), config, collector);
        }
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

impl Validate for CardholderAuthentication2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.authntcn_mtd
            .validate(&helpers::child_path(path, "AuthntcnMtd"), config, collector);
        self.authntcn_ntty.validate(
            &helpers::child_path(path, "AuthntcnNtty"),
            config,
            collector,
        );
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

impl Validate for CardholderVerificationCapability1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for CashAccount381 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        helpers::validate_pattern(
            &self.ccy,
            "Ccy",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "Ccy"),
            config,
            collector,
        );
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prxy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prxy"), config, collector);
        }
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
    pub prxy: Option<ProxyAccountIdentification12>,
}

impl Validate for CashAccount382 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.ccy {
            helpers::validate_pattern(
                val,
                "Ccy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "Ccy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prxy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prxy"), config, collector);
        }
    }
}

// CashAccount383: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount383 {
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

impl Validate for CashAccount383 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.ccy {
            helpers::validate_pattern(
                val,
                "Ccy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "Ccy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prxy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prxy"), config, collector);
        }
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

impl Validate for CashAccount391 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        helpers::validate_pattern(
            &self.ccy,
            "Ccy",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "Ccy"),
            config,
            collector,
        );
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prxy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prxy"), config, collector);
        }
        if let Some(ref val) = self.ownr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Ownr"), config, collector);
        }
        if let Some(ref val) = self.svcr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Svcr"), config, collector);
        }
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

impl Validate for CashAccountType2Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for CashAvailability1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.dt
            .validate(&helpers::child_path(path, "Dt"), config, collector);
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
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

impl Validate for CashAvailabilityDate1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nb_of_days {
            helpers::validate_pattern(
                val,
                "NbOfDays",
                "[\\+]{0,1}[0-9]{1,15}",
                &helpers::child_path(path, "NbOfDays"),
                config,
                collector,
            );
        }
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

impl Validate for CashBalance81 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
        if let Some(ref vec) = self.cdt_line
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "CdtLine"), config, collector);
            }
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        self.dt
            .validate(&helpers::child_path(path, "Dt"), config, collector);
        if let Some(ref vec) = self.avlbty
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Avlbty"), config, collector);
            }
        }
    }
}

// CashDeposit1: Specifies the total amount of money in the cash deposit, that is the note denomination times the number of notes.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashDeposit1 {
    #[serde(rename = "NoteDnmtn")]
    pub note_dnmtn: ActiveCurrencyAndAmount,
    #[serde(rename = "NbOfNotes")]
    pub nb_of_notes: String,
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}

impl Validate for CashDeposit1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.note_dnmtn
            .validate(&helpers::child_path(path, "NoteDnmtn"), config, collector);
        helpers::validate_pattern(
            &self.nb_of_notes,
            "NbOfNotes",
            "[0-9]{1,15}",
            &helpers::child_path(path, "NbOfNotes"),
            config,
            collector,
        );
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
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

impl Validate for ChargeBearerType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for ChargeType3Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prtry"), config, collector);
        }
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

impl Validate for Charges61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.ttl_chrgs_and_tax_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TtlChrgsAndTaxAmt"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.rcrd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Rcrd"), config, collector);
            }
        }
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
    pub agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges21>,
}

impl Validate for ChargesRecord31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.cdt_dbt_ind
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        }
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.br
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Br"), config, collector);
        }
        if let Some(ref val) = self.agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Agt"), config, collector);
        }
        if let Some(ref val) = self.tax
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tax"), config, collector);
        }
    }
}

// ClearingSystemIdentification2Choice1: Identification of a clearing system, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification2Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl Validate for ClearingSystemIdentification2Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(5),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
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

impl Validate for ClearingSystemMemberIdentification21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.clr_sys_id
            .validate(&helpers::child_path(path, "ClrSysId"), config, collector);
        helpers::validate_length(
            &self.mmb_id,
            "MmbId",
            Some(1),
            Some(28),
            &helpers::child_path(path, "MmbId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.mmb_id,
            "MmbId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "MmbId"),
            config,
            collector,
        );
    }
}

// Contact41: Name by which a party is known and which is usually used to identify that party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contact41 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl Validate for Contact41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
    }
}

// Contact42: Preferred method used to reach the contact.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contact42 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}

impl Validate for Contact42 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prefrd_mtd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PrefrdMtd"), config, collector);
        }
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

impl Validate for CopyDuplicate1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for CorporateAction91 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.evt_tp,
            "EvtTp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "EvtTp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.evt_tp,
            "EvtTp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "EvtTp"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.evt_id,
            "EvtId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "EvtId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.evt_id,
            "EvtId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "EvtId"),
            config,
            collector,
        );
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

impl Validate for CreditDebitCode {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for CreditLine31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Amt"), config, collector);
        }
        if let Some(ref val) = self.dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dt"), config, collector);
        }
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

impl Validate for CreditLineType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for CreditorReferenceInformation21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.ref_attr {
            helpers::validate_length(
                val,
                "Ref",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Ref"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_attr {
            helpers::validate_pattern(
                val,
                "Ref",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Ref"),
                config,
                collector,
            );
        }
    }
}

// CreditorReferenceType1Choice: Creditor reference type, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DocumentType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for CreditorReferenceType1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
    }
}

// CreditorReferenceType21: Entity that assigns the credit reference type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType21 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: CreditorReferenceType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for CreditorReferenceType21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.cd_or_prtry
            .validate(&helpers::child_path(path, "CdOrPrtry"), config, collector);
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
    }
}

// CurrencyExchange5: Date and time at which an exchange rate is quoted.
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

impl Validate for CurrencyExchange5 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.src_ccy,
            "SrcCcy",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "SrcCcy"),
            config,
            collector,
        );
        if let Some(ref val) = self.trgt_ccy {
            helpers::validate_pattern(
                val,
                "TrgtCcy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "TrgtCcy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.unit_ccy {
            helpers::validate_pattern(
                val,
                "UnitCcy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "UnitCcy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctrct_id {
            helpers::validate_length(
                val,
                "CtrctId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtrctId"),
                config,
                collector,
            );
        }
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

impl Validate for CurrencyExchange51 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.src_ccy,
            "SrcCcy",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "SrcCcy"),
            config,
            collector,
        );
        if let Some(ref val) = self.trgt_ccy {
            helpers::validate_pattern(
                val,
                "TrgtCcy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "TrgtCcy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.unit_ccy {
            helpers::validate_pattern(
                val,
                "UnitCcy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "UnitCcy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctrct_id {
            helpers::validate_length(
                val,
                "CtrctId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtrctId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctrct_id {
            helpers::validate_pattern(
                val,
                "CtrctId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "CtrctId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.qtn_dt {
            helpers::validate_pattern(
                val,
                "QtnDt",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "QtnDt"),
                config,
                collector,
            );
        }
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

impl Validate for CurrencyExchange52 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.src_ccy,
            "SrcCcy",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "SrcCcy"),
            config,
            collector,
        );
        if let Some(ref val) = self.trgt_ccy {
            helpers::validate_pattern(
                val,
                "TrgtCcy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "TrgtCcy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.unit_ccy {
            helpers::validate_pattern(
                val,
                "UnitCcy",
                "[A-Z]{3,3}",
                &helpers::child_path(path, "UnitCcy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctrct_id {
            helpers::validate_length(
                val,
                "CtrctId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtrctId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctrct_id {
            helpers::validate_pattern(
                val,
                "CtrctId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "CtrctId"),
                config,
                collector,
            );
        }
    }
}

// DateAndDateTime2Choice: Specified date and time.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndDateTime2Choice {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<String>,
}

impl Validate for DateAndDateTime2Choice {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// DateAndDateTime2Choice1: Specified date and time.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndDateTime2Choice1 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<String>,
}

impl Validate for DateAndDateTime2Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dt_tm {
            helpers::validate_pattern(
                val,
                "DtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "DtTm"),
                config,
                collector,
            );
        }
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

impl Validate for DateAndPlaceOfBirth11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.prvc_of_birth {
            helpers::validate_length(
                val,
                "PrvcOfBirth",
                Some(1),
                Some(35),
                &helpers::child_path(path, "PrvcOfBirth"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prvc_of_birth {
            helpers::validate_pattern(
                val,
                "PrvcOfBirth",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "PrvcOfBirth"),
                config,
                collector,
            );
        }
        helpers::validate_length(
            &self.city_of_birth,
            "CityOfBirth",
            Some(1),
            Some(35),
            &helpers::child_path(path, "CityOfBirth"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.city_of_birth,
            "CityOfBirth",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            &helpers::child_path(path, "CityOfBirth"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.ctry_of_birth,
            "CtryOfBirth",
            "[A-Z]{2,2}",
            &helpers::child_path(path, "CtryOfBirth"),
            config,
            collector,
        );
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

impl Validate for DateOrDateTimePeriod1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dt"), config, collector);
        }
        if let Some(ref val) = self.dt_tm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DtTm"), config, collector);
        }
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

impl Validate for DatePeriod2 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// DateTimePeriod11: Date and time at which the period ends.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateTimePeriod11 {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: String,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: String,
}

impl Validate for DateTimePeriod11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.fr_dt_tm,
            "FrDtTm",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "FrDtTm"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.to_dt_tm,
            "ToDtTm",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "ToDtTm"),
            config,
            collector,
        );
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

impl Validate for DiscountAmountAndType11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
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

impl Validate for DiscountAmountType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for DisplayCapabilities1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.disp_tp
            .validate(&helpers::child_path(path, "DispTp"), config, collector);
        helpers::validate_pattern(
            &self.nb_of_lines,
            "NbOfLines",
            "[0-9]{1,3}",
            &helpers::child_path(path, "NbOfLines"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.line_width,
            "LineWidth",
            "[0-9]{1,3}",
            &helpers::child_path(path, "LineWidth"),
            config,
            collector,
        );
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

impl Validate for DocumentAdjustment11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.cdt_dbt_ind
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        }
        if let Some(ref val) = self.rsn {
            helpers::validate_length(
                val,
                "Rsn",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Rsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rsn {
            helpers::validate_pattern(
                val,
                "Rsn",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Rsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_length(
                val,
                "AddtlInf",
                Some(1),
                Some(140),
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_pattern(
                val,
                "AddtlInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
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

impl Validate for DocumentLineIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.nb {
            helpers::validate_length(
                val,
                "Nb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Nb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nb {
            helpers::validate_pattern(
                val,
                "Nb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nb"),
                config,
                collector,
            );
        }
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

impl Validate for DocumentLineInformation11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        for item in &self.id {
            item.validate(&helpers::child_path(path, "Id"), config, collector);
        }
        if let Some(ref val) = self.desc {
            helpers::validate_length(
                val,
                "Desc",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Desc"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.desc {
            helpers::validate_pattern(
                val,
                "Desc",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Desc"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Amt"), config, collector);
        }
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

impl Validate for DocumentLineType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for DocumentLineType11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.cd_or_prtry
            .validate(&helpers::child_path(path, "CdOrPrtry"), config, collector);
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for DocumentType3Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for DocumentType6Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// EntryDetails91: Provides information on the underlying transaction(s).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryDetails91 {
    #[serde(rename = "Btch", skip_serializing_if = "Option::is_none")]
    pub btch: Option<BatchInformation21>,
    #[serde(rename = "TxDtls")]
    pub tx_dtls: Box<EntryTransaction101>,
}

impl Validate for EntryDetails91 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.btch
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Btch"), config, collector);
        }
        self.tx_dtls
            .validate(&helpers::child_path(path, "TxDtls"), config, collector);
    }
}

// EntryStatus1Choice1: Entry status, in a coded form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryStatus1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalEntryStatus1Codefixed>,
}

impl Validate for EntryStatus1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
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
    pub csh_dpst: Option<Vec<CashDeposit1>>,
    #[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardTransaction171>,
    #[serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none")]
    pub addtl_tx_inf: Option<String>,
}

impl Validate for EntryTransaction101 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.refs
            .validate(&helpers::child_path(path, "Refs"), config, collector);
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        if let Some(ref val) = self.amt_dtls
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AmtDtls"), config, collector);
        }
        if let Some(ref vec) = self.avlbty
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Avlbty"), config, collector);
            }
        }
        if let Some(ref val) = self.bk_tx_cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "BkTxCd"), config, collector);
        }
        if let Some(ref val) = self.chrgs
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Chrgs"), config, collector);
        }
        if let Some(ref val) = self.intrst
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Intrst"), config, collector);
        }
        if let Some(ref val) = self.rltd_pties
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RltdPties"), config, collector);
        }
        if let Some(ref val) = self.rltd_agts
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RltdAgts"), config, collector);
        }
        if let Some(ref val) = self.lcl_instrm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "LclInstrm"), config, collector);
        }
        if let Some(ref val) = self.purp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Purp"), config, collector);
        }
        if let Some(ref vec) = self.rltd_rmt_inf
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "RltdRmtInf"), config, collector);
            }
        }
        if let Some(ref val) = self.rmt_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RmtInf"), config, collector);
        }
        if let Some(ref val) = self.rltd_dts
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RltdDts"), config, collector);
        }
        if let Some(ref val) = self.rltd_pric
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RltdPric"), config, collector);
        }
        if let Some(ref vec) = self.rltd_qties
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "RltdQties"), config, collector);
            }
        }
        if let Some(ref val) = self.fin_instrm_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FinInstrmId"), config, collector);
        }
        if let Some(ref val) = self.tax
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tax"), config, collector);
        }
        if let Some(ref val) = self.rtr_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RtrInf"), config, collector);
        }
        if let Some(ref val) = self.corp_actn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CorpActn"), config, collector);
        }
        if let Some(ref val) = self.sfkpg_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SfkpgAcct"), config, collector);
        }
        if let Some(ref vec) = self.csh_dpst
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "CshDpst"), config, collector);
            }
        }
        if let Some(ref val) = self.card_tx
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CardTx"), config, collector);
        }
        if let Some(ref val) = self.addtl_tx_inf {
            helpers::validate_length(
                val,
                "AddtlTxInf",
                Some(1),
                Some(500),
                &helpers::child_path(path, "AddtlTxInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_tx_inf {
            helpers::validate_pattern(
                val,
                "AddtlTxInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AddtlTxInf"),
                config,
                collector,
            );
        }
    }
}

// ExternalEntryStatus1Code_fixed: BOOK
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ExternalEntryStatus1Codefixed {
    #[default]
    #[serde(rename = "BOOK")]
    CodeBOOK,
}

impl Validate for ExternalEntryStatus1Codefixed {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for FinancialIdentificationSchemeName1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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
    pub pstl_adr: Option<PostalAddress242>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification11>,
}

impl Validate for FinancialInstitutionIdentification181 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.bicfi {
            helpers::validate_pattern(
                val,
                "BICFI",
                "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
                &helpers::child_path(path, "BICFI"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.clr_sys_mmb_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ClrSysMmbId"), config, collector);
        }
        if let Some(ref val) = self.lei {
            helpers::validate_pattern(
                val,
                "LEI",
                "[A-Z0-9]{18,18}[0-9]{2,2}",
                &helpers::child_path(path, "LEI"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
        if let Some(ref val) = self.othr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Othr"), config, collector);
        }
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

impl Validate for FinancialInstrumentQuantity1Choice {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// FromToAmountRange1: Upper boundary of a range of amount values.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FromToAmountRange1 {
    #[serde(rename = "FrAmt")]
    pub fr_amt: AmountRangeBoundary1,
    #[serde(rename = "ToAmt")]
    pub to_amt: AmountRangeBoundary1,
}

impl Validate for FromToAmountRange1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fr_amt
            .validate(&helpers::child_path(path, "FrAmt"), config, collector);
        self.to_amt
            .validate(&helpers::child_path(path, "ToAmt"), config, collector);
    }
}

// Garnishment31: Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Garnishment31 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType11,
    #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification1353>,
    #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification1353>,
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

impl Validate for Garnishment31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
        if let Some(ref val) = self.grnshee
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Grnshee"), config, collector);
        }
        if let Some(ref val) = self.grnshmt_admstr
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "GrnshmtAdmstr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_nb {
            helpers::validate_length(
                val,
                "RefNb",
                Some(1),
                Some(140),
                &helpers::child_path(path, "RefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_nb {
            helpers::validate_pattern(
                val,
                "RefNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "RefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rmtd_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
        }
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

impl Validate for GarnishmentType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for GarnishmentType11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.cd_or_prtry
            .validate(&helpers::child_path(path, "CdOrPrtry"), config, collector);
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GenericAccountIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(34),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]*(/[0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ])?)*)",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
        }
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GenericFinancialIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
        }
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GenericIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm {
            helpers::validate_length(
                val,
                "SchmeNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SchmeNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.schme_nm {
            helpers::validate_pattern(
                val,
                "SchmeNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SchmeNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GenericIdentification301 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[a-zA-Z0-9]{4}",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.issr,
            "Issr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Issr"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.issr,
            "Issr",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Issr"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm {
            helpers::validate_length(
                val,
                "SchmeNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SchmeNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.schme_nm {
            helpers::validate_pattern(
                val,
                "SchmeNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SchmeNm"),
                config,
                collector,
            );
        }
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

impl Validate for GenericIdentification302 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[a-zA-Z0-9]{4}",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.issr,
            "Issr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Issr"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.issr,
            "Issr",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            &helpers::child_path(path, "Issr"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm {
            helpers::validate_length(
                val,
                "SchmeNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SchmeNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.schme_nm {
            helpers::validate_pattern(
                val,
                "SchmeNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "SchmeNm"),
                config,
                collector,
            );
        }
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

impl Validate for GenericIdentification321 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.issr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Issr"), config, collector);
        }
        if let Some(ref val) = self.shrt_nm {
            helpers::validate_length(
                val,
                "ShrtNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "ShrtNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.shrt_nm {
            helpers::validate_pattern(
                val,
                "ShrtNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ShrtNm"),
                config,
                collector,
            );
        }
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

impl Validate for GenericIdentification31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GenericOrganisationIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
        }
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GenericPersonIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.schme_nm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
        }
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
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

impl Validate for GroupHeader811 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.msg_id,
            "MsgId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "MsgId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.msg_id,
            "MsgId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "MsgId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.cre_dt_tm,
            "CreDtTm",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "CreDtTm"),
            config,
            collector,
        );
        if let Some(ref val) = self.msg_rcpt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "MsgRcpt"), config, collector);
        }
        if let Some(ref val) = self.orgnl_biz_qry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgnlBizQry"), config, collector);
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_length(
                val,
                "AddtlInf",
                Some(1),
                Some(500),
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_pattern(
                val,
                "AddtlInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
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

impl Validate for IdentificationSource3Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for ImpliedCurrencyAmountRange1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.fr_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrAmt"), config, collector);
        }
        if let Some(ref val) = self.to_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ToAmt"), config, collector);
        }
        if let Some(ref val) = self.fr_to_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrToAmt"), config, collector);
        }
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

impl Validate for InterestRecord21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.rate
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Rate"), config, collector);
        }
        if let Some(ref val) = self.fr_to_dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrToDt"), config, collector);
        }
        if let Some(ref val) = self.rsn {
            helpers::validate_length(
                val,
                "Rsn",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Rsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rsn {
            helpers::validate_pattern(
                val,
                "Rsn",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Rsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tax"), config, collector);
        }
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

impl Validate for InterestType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for InterestType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for LocalInstrument2Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for MessageIdentification21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.msg_nm_id {
            helpers::validate_length(
                val,
                "MsgNmId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MsgNmId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.msg_nm_id {
            helpers::validate_pattern(
                val,
                "MsgNmId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MsgNmId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.msg_id {
            helpers::validate_length(
                val,
                "MsgId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MsgId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.msg_id {
            helpers::validate_pattern(
                val,
                "MsgId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MsgId"),
                config,
                collector,
            );
        }
    }
}

// NameAndAddress161: Postal address of a party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NameAndAddress161 {
    #[serde(rename = "Nm")]
    pub nm: String,
    #[serde(rename = "Adr")]
    pub adr: PostalAddress242,
}

impl Validate for NameAndAddress161 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.nm,
            "Nm",
            Some(1),
            Some(140),
            &helpers::child_path(path, "Nm"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.nm,
            "Nm",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            &helpers::child_path(path, "Nm"),
            config,
            collector,
        );
        self.adr
            .validate(&helpers::child_path(path, "Adr"), config, collector);
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

impl Validate for NumberAndSumOfTransactions1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nb_of_ntries {
            helpers::validate_pattern(
                val,
                "NbOfNtries",
                "[0-9]{1,15}",
                &helpers::child_path(path, "NbOfNtries"),
                config,
                collector,
            );
        }
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

impl Validate for NumberAndSumOfTransactions4 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nb_of_ntries {
            helpers::validate_pattern(
                val,
                "NbOfNtries",
                "[0-9]{1,15}",
                &helpers::child_path(path, "NbOfNtries"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_net_ntry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlNetNtry"), config, collector);
        }
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

impl Validate for OnLineCapability1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for OrganisationIdentification291 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.any_bic {
            helpers::validate_pattern(
                val,
                "AnyBIC",
                "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
                &helpers::child_path(path, "AnyBIC"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.lei {
            helpers::validate_pattern(
                val,
                "LEI",
                "[A-Z0-9]{18,18}[0-9]{2,2}",
                &helpers::child_path(path, "LEI"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.othr
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Othr"), config, collector);
            }
        }
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

impl Validate for OrganisationIdentificationSchemeName1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for OriginalAndCurrentQuantities1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
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

impl Validate for OriginalBusinessQuery11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.msg_id,
            "MsgId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "MsgId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.msg_id,
            "MsgId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "MsgId"),
            config,
            collector,
        );
        if let Some(ref val) = self.msg_nm_id {
            helpers::validate_length(
                val,
                "MsgNmId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MsgNmId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.msg_nm_id {
            helpers::validate_pattern(
                val,
                "MsgNmId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MsgNmId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cre_dt_tm {
            helpers::validate_pattern(
                val,
                "CreDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "CreDtTm"),
                config,
                collector,
            );
        }
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

impl Validate for OtherIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.sfx {
            helpers::validate_length(
                val,
                "Sfx",
                Some(1),
                Some(16),
                &helpers::child_path(path, "Sfx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sfx {
            helpers::validate_pattern(
                val,
                "Sfx",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Sfx"),
                config,
                collector,
            );
        }
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
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

impl Validate for POIComponentType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for Pagination1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.pg_nb,
            "PgNb",
            "[0-9]{1,5}",
            &helpers::child_path(path, "PgNb"),
            config,
            collector,
        );
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

impl Validate for Party38Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.org_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgId"), config, collector);
        }
        if let Some(ref val) = self.prvt_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PrvtId"), config, collector);
        }
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

impl Validate for Party40Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.pty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Pty"), config, collector);
        }
        if let Some(ref val) = self.agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Agt"), config, collector);
        }
    }
}

// Party40Choice2: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party40Choice2 {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification1354>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Validate for Party40Choice2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.pty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Pty"), config, collector);
        }
        if let Some(ref val) = self.agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Agt"), config, collector);
        }
    }
}

// Party40Choice3: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party40Choice3 {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification1353>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Validate for Party40Choice3 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.pty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Pty"), config, collector);
        }
        if let Some(ref val) = self.agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Agt"), config, collector);
        }
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

impl Validate for PartyIdentification1351 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
        if let Some(ref val) = self.id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Id"), config, collector);
        }
        if let Some(ref val) = self.ctct_dtls
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CtctDtls"), config, collector);
        }
    }
}

// PartyIdentification1352: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1352 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl Validate for PartyIdentification1352 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
        if let Some(ref val) = self.id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Id"), config, collector);
        }
        if let Some(ref val) = self.ctry_of_res {
            helpers::validate_pattern(
                val,
                "CtryOfRes",
                "[A-Z]{2,2}",
                &helpers::child_path(path, "CtryOfRes"),
                config,
                collector,
            );
        }
    }
}

// PartyIdentification1353: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1353 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact41>,
}

impl Validate for PartyIdentification1353 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
        if let Some(ref val) = self.id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Id"), config, collector);
        }
        if let Some(ref val) = self.ctry_of_res {
            helpers::validate_pattern(
                val,
                "CtryOfRes",
                "[A-Z]{2,2}",
                &helpers::child_path(path, "CtryOfRes"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctct_dtls
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CtctDtls"), config, collector);
        }
    }
}

// PartyIdentification1354: Set of elements used to indicate how to contact the party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1354 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice1>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact42>,
}

impl Validate for PartyIdentification1354 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
        if let Some(ref val) = self.id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Id"), config, collector);
        }
        if let Some(ref val) = self.ctry_of_res {
            helpers::validate_pattern(
                val,
                "CtryOfRes",
                "[A-Z]{2,2}",
                &helpers::child_path(path, "CtryOfRes"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctct_dtls
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CtctDtls"), config, collector);
        }
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

impl Validate for PartyType3Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for PartyType4Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for PaymentCard41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.plain_card_data
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PlainCardData"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.card_ctry_cd {
            helpers::validate_pattern(
                val,
                "CardCtryCd",
                "[0-9]{3}",
                &helpers::child_path(path, "CardCtryCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.card_brnd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CardBrnd"), config, collector);
        }
        if let Some(ref val) = self.addtl_card_data {
            helpers::validate_length(
                val,
                "AddtlCardData",
                Some(1),
                Some(70),
                &helpers::child_path(path, "AddtlCardData"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_card_data {
            helpers::validate_pattern(
                val,
                "AddtlCardData",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AddtlCardData"),
                config,
                collector,
            );
        }
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

impl Validate for PaymentContext3 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.attndnc_cntxt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "AttndncCntxt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tx_envt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxEnvt"), config, collector);
        }
        if let Some(ref val) = self.tx_chanl
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TxChanl"), config, collector);
        }
        if let Some(ref val) = self.attndnt_lang {
            helpers::validate_pattern(
                val,
                "AttndntLang",
                "[a-z]{2,2}",
                &helpers::child_path(path, "AttndntLang"),
                config,
                collector,
            );
        }
        self.card_data_ntry_md.validate(
            &helpers::child_path(path, "CardDataNtryMd"),
            config,
            collector,
        );
        if let Some(ref val) = self.authntcn_mtd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AuthntcnMtd"), config, collector);
        }
    }
}

// PaymentReturnReason51: Further details on the return reason.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentReturnReason51 {
    #[serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none")]
    pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure41>,
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification1354>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<ReturnReason5Choice1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl Validate for PaymentReturnReason51 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.orgnl_bk_tx_cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgnlBkTxCd"), config, collector);
        }
        if let Some(ref val) = self.orgtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Orgtr"), config, collector);
        }
        if let Some(ref val) = self.rsn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Rsn"), config, collector);
        }
        if let Some(ref vec) = self.addtl_inf {
            for item in vec {
                helpers::validate_length(
                    item,
                    "AddtlInf",
                    Some(1),
                    Some(105),
                    &helpers::child_path(path, "AddtlInf"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.addtl_inf {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "AddtlInf",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                    &helpers::child_path(path, "AddtlInf"),
                    config,
                    collector,
                );
            }
        }
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

impl Validate for PersonIdentification131 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dt_and_plc_of_birth
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "DtAndPlcOfBirth"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.othr
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Othr"), config, collector);
            }
        }
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

impl Validate for PersonIdentificationSchemeName1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for PlainCardData11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.pan,
            "PAN",
            "[0-9]{8,28}",
            &helpers::child_path(path, "PAN"),
            config,
            collector,
        );
        if let Some(ref val) = self.card_seq_nb {
            helpers::validate_pattern(
                val,
                "CardSeqNb",
                "[0-9]{2,3}",
                &helpers::child_path(path, "CardSeqNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.svc_cd {
            helpers::validate_pattern(
                val,
                "SvcCd",
                "[0-9]{3}",
                &helpers::child_path(path, "SvcCd"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.trck_data
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "TrckData"), config, collector);
            }
        }
        if let Some(ref val) = self.card_scty_cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CardSctyCd"), config, collector);
        }
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

impl Validate for PointOfInteraction11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.sys_nm {
            helpers::validate_length(
                val,
                "SysNm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "SysNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sys_nm {
            helpers::validate_pattern(
                val,
                "SysNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SysNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.grp_id {
            helpers::validate_length(
                val,
                "GrpId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "GrpId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.grp_id {
            helpers::validate_pattern(
                val,
                "GrpId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "GrpId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cpblties
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cpblties"), config, collector);
        }
        if let Some(ref vec) = self.cmpnt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Cmpnt"), config, collector);
            }
        }
    }
}

// PointOfInteraction12: Data related to a component of the POI performing the transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteraction12 {
    #[serde(rename = "Id")]
    pub id: GenericIdentification321,
    #[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
    pub sys_nm: Option<String>,
    #[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
    pub grp_id: Option<String>,
    #[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<PointOfInteractionCapabilities1>,
    #[serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none")]
    pub cmpnt: Option<Vec<PointOfInteractionComponent12>>,
}

impl Validate for PointOfInteraction12 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.sys_nm {
            helpers::validate_length(
                val,
                "SysNm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "SysNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sys_nm {
            helpers::validate_pattern(
                val,
                "SysNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SysNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.grp_id {
            helpers::validate_length(
                val,
                "GrpId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "GrpId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.grp_id {
            helpers::validate_pattern(
                val,
                "GrpId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "GrpId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cpblties
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cpblties"), config, collector);
        }
        if let Some(ref vec) = self.cmpnt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Cmpnt"), config, collector);
            }
        }
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

impl Validate for PointOfInteractionCapabilities1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref vec) = self.card_rdng_cpblties
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "CardRdngCpblties"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.crdhldr_vrfctn_cpblties
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "CrdhldrVrfctnCpblties"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.on_line_cpblties
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OnLineCpblties"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.disp_cpblties
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "DispCpblties"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.prt_line_width {
            helpers::validate_pattern(
                val,
                "PrtLineWidth",
                "[0-9]{1,3}",
                &helpers::child_path(path, "PrtLineWidth"),
                config,
                collector,
            );
        }
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

impl Validate for PointOfInteractionComponent11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.poi_cmpnt_tp
            .validate(&helpers::child_path(path, "POICmpntTp"), config, collector);
        if let Some(ref val) = self.manfctr_id {
            helpers::validate_length(
                val,
                "ManfctrId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "ManfctrId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.manfctr_id {
            helpers::validate_pattern(
                val,
                "ManfctrId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ManfctrId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mdl {
            helpers::validate_length(
                val,
                "Mdl",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Mdl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mdl {
            helpers::validate_pattern(
                val,
                "Mdl",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Mdl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.vrsn_nb {
            helpers::validate_length(
                val,
                "VrsnNb",
                Some(1),
                Some(16),
                &helpers::child_path(path, "VrsnNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.vrsn_nb {
            helpers::validate_pattern(
                val,
                "VrsnNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "VrsnNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.srl_nb {
            helpers::validate_length(
                val,
                "SrlNb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SrlNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.srl_nb {
            helpers::validate_pattern(
                val,
                "SrlNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SrlNb"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.apprvl_nb {
            for item in vec {
                helpers::validate_length(
                    item,
                    "ApprvlNb",
                    Some(1),
                    Some(70),
                    &helpers::child_path(path, "ApprvlNb"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.apprvl_nb {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "ApprvlNb",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                    &helpers::child_path(path, "ApprvlNb"),
                    config,
                    collector,
                );
            }
        }
    }
}

// PointOfInteractionComponent12: Unique approval number for a component, delivered by a certification body.

// Usage: More than one approval number could be present, when assigned by different bodies. The certification body identification must be provided within the approval number (for example at the beginning of the value).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInteractionComponent12 {
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

impl Validate for PointOfInteractionComponent12 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.poi_cmpnt_tp
            .validate(&helpers::child_path(path, "POICmpntTp"), config, collector);
        if let Some(ref val) = self.manfctr_id {
            helpers::validate_length(
                val,
                "ManfctrId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "ManfctrId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.manfctr_id {
            helpers::validate_pattern(
                val,
                "ManfctrId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ManfctrId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mdl {
            helpers::validate_length(
                val,
                "Mdl",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Mdl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mdl {
            helpers::validate_pattern(
                val,
                "Mdl",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Mdl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.vrsn_nb {
            helpers::validate_length(
                val,
                "VrsnNb",
                Some(1),
                Some(16),
                &helpers::child_path(path, "VrsnNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.vrsn_nb {
            helpers::validate_pattern(
                val,
                "VrsnNb",
                "([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]*(/[0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ])?)*)",
                &helpers::child_path(path, "VrsnNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.srl_nb {
            helpers::validate_length(
                val,
                "SrlNb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "SrlNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.srl_nb {
            helpers::validate_pattern(
                val,
                "SrlNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "SrlNb"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.apprvl_nb {
            for item in vec {
                helpers::validate_length(
                    item,
                    "ApprvlNb",
                    Some(1),
                    Some(70),
                    &helpers::child_path(path, "ApprvlNb"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.apprvl_nb {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "ApprvlNb",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                    &helpers::child_path(path, "ApprvlNb"),
                    config,
                    collector,
                );
            }
        }
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

impl Validate for PostalAddress241 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.adr_tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AdrTp"), config, collector);
        }
        if let Some(ref val) = self.dept {
            helpers::validate_length(
                val,
                "Dept",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Dept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dept {
            helpers::validate_pattern(
                val,
                "Dept",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Dept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sub_dept {
            helpers::validate_length(
                val,
                "SubDept",
                Some(1),
                Some(70),
                &helpers::child_path(path, "SubDept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sub_dept {
            helpers::validate_pattern(
                val,
                "SubDept",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "SubDept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.strt_nm {
            helpers::validate_length(
                val,
                "StrtNm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "StrtNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.strt_nm {
            helpers::validate_pattern(
                val,
                "StrtNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "StrtNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nb {
            helpers::validate_length(
                val,
                "BldgNb",
                Some(1),
                Some(16),
                &helpers::child_path(path, "BldgNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nb {
            helpers::validate_pattern(
                val,
                "BldgNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "BldgNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nm {
            helpers::validate_length(
                val,
                "BldgNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "BldgNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nm {
            helpers::validate_pattern(
                val,
                "BldgNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "BldgNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.flr {
            helpers::validate_length(
                val,
                "Flr",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Flr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.flr {
            helpers::validate_pattern(
                val,
                "Flr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Flr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_bx {
            helpers::validate_length(
                val,
                "PstBx",
                Some(1),
                Some(16),
                &helpers::child_path(path, "PstBx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_bx {
            helpers::validate_pattern(
                val,
                "PstBx",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "PstBx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.room {
            helpers::validate_length(
                val,
                "Room",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Room"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.room {
            helpers::validate_pattern(
                val,
                "Room",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Room"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_cd {
            helpers::validate_length(
                val,
                "PstCd",
                Some(1),
                Some(16),
                &helpers::child_path(path, "PstCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_cd {
            helpers::validate_pattern(
                val,
                "PstCd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "PstCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_nm {
            helpers::validate_length(
                val,
                "TwnNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TwnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_nm {
            helpers::validate_pattern(
                val,
                "TwnNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TwnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_lctn_nm {
            helpers::validate_length(
                val,
                "TwnLctnNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TwnLctnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_lctn_nm {
            helpers::validate_pattern(
                val,
                "TwnLctnNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TwnLctnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dstrct_nm {
            helpers::validate_length(
                val,
                "DstrctNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "DstrctNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dstrct_nm {
            helpers::validate_pattern(
                val,
                "DstrctNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "DstrctNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            helpers::validate_length(
                val,
                "CtrySubDvsn",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtrySubDvsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            helpers::validate_pattern(
                val,
                "CtrySubDvsn",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "CtrySubDvsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctry {
            helpers::validate_pattern(
                val,
                "Ctry",
                "[A-Z]{2,2}",
                &helpers::child_path(path, "Ctry"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.adr_line {
            for item in vec {
                helpers::validate_length(
                    item,
                    "AdrLine",
                    Some(1),
                    Some(70),
                    &helpers::child_path(path, "AdrLine"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.adr_line {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "AdrLine",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                    &helpers::child_path(path, "AdrLine"),
                    config,
                    collector,
                );
            }
        }
    }
}

// PostalAddress242: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress242 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType3Choice2>,
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

impl Validate for PostalAddress242 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.adr_tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AdrTp"), config, collector);
        }
        if let Some(ref val) = self.dept {
            helpers::validate_length(
                val,
                "Dept",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Dept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dept {
            helpers::validate_pattern(
                val,
                "Dept",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Dept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sub_dept {
            helpers::validate_length(
                val,
                "SubDept",
                Some(1),
                Some(70),
                &helpers::child_path(path, "SubDept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sub_dept {
            helpers::validate_pattern(
                val,
                "SubDept",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "SubDept"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.strt_nm {
            helpers::validate_length(
                val,
                "StrtNm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "StrtNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.strt_nm {
            helpers::validate_pattern(
                val,
                "StrtNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "StrtNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nb {
            helpers::validate_length(
                val,
                "BldgNb",
                Some(1),
                Some(16),
                &helpers::child_path(path, "BldgNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nb {
            helpers::validate_pattern(
                val,
                "BldgNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "BldgNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nm {
            helpers::validate_length(
                val,
                "BldgNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "BldgNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.bldg_nm {
            helpers::validate_pattern(
                val,
                "BldgNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "BldgNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.flr {
            helpers::validate_length(
                val,
                "Flr",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Flr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.flr {
            helpers::validate_pattern(
                val,
                "Flr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Flr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_bx {
            helpers::validate_length(
                val,
                "PstBx",
                Some(1),
                Some(16),
                &helpers::child_path(path, "PstBx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_bx {
            helpers::validate_pattern(
                val,
                "PstBx",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "PstBx"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.room {
            helpers::validate_length(
                val,
                "Room",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Room"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.room {
            helpers::validate_pattern(
                val,
                "Room",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Room"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_cd {
            helpers::validate_length(
                val,
                "PstCd",
                Some(1),
                Some(16),
                &helpers::child_path(path, "PstCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pst_cd {
            helpers::validate_pattern(
                val,
                "PstCd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "PstCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_nm {
            helpers::validate_length(
                val,
                "TwnNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TwnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_nm {
            helpers::validate_pattern(
                val,
                "TwnNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TwnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_lctn_nm {
            helpers::validate_length(
                val,
                "TwnLctnNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TwnLctnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.twn_lctn_nm {
            helpers::validate_pattern(
                val,
                "TwnLctnNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TwnLctnNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dstrct_nm {
            helpers::validate_length(
                val,
                "DstrctNm",
                Some(1),
                Some(35),
                &helpers::child_path(path, "DstrctNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dstrct_nm {
            helpers::validate_pattern(
                val,
                "DstrctNm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "DstrctNm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            helpers::validate_length(
                val,
                "CtrySubDvsn",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtrySubDvsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            helpers::validate_pattern(
                val,
                "CtrySubDvsn",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "CtrySubDvsn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctry {
            helpers::validate_pattern(
                val,
                "Ctry",
                "[A-Z]{2,2}",
                &helpers::child_path(path, "Ctry"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.adr_line {
            for item in vec {
                helpers::validate_length(
                    item,
                    "AdrLine",
                    Some(1),
                    Some(70),
                    &helpers::child_path(path, "AdrLine"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.adr_line {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "AdrLine",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                    &helpers::child_path(path, "AdrLine"),
                    config,
                    collector,
                );
            }
        }
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

impl Validate for PreferredContactMethod1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// Price7: Value of the price, for example, as a currency and value.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Price7 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType1Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice,
}

impl Validate for Price7 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
        self.val
            .validate(&helpers::child_path(path, "Val"), config, collector);
    }
}

// PriceRateOrAmount3Choice: Price expressed as a currency and value.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PriceRateOrAmount3Choice {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl Validate for PriceRateOrAmount3Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Amt"), config, collector);
        }
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

impl Validate for PriceValueType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for Product21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.pdct_cd,
            "PdctCd",
            Some(1),
            Some(70),
            &helpers::child_path(path, "PdctCd"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.pdct_cd,
            "PdctCd",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "PdctCd"),
            config,
            collector,
        );
        if let Some(ref val) = self.unit_of_measr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "UnitOfMeasr"), config, collector);
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_length(
                val,
                "TaxTp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_pattern(
                val,
                "TaxTp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_pdct_inf {
            helpers::validate_length(
                val,
                "AddtlPdctInf",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AddtlPdctInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_pdct_inf {
            helpers::validate_pattern(
                val,
                "AddtlPdctInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AddtlPdctInf"),
                config,
                collector,
            );
        }
    }
}

// ProprietaryAgent41: Organisation established primarily to provide financial services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryAgent41 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification61,
}

impl Validate for ProprietaryAgent41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        self.agt
            .validate(&helpers::child_path(path, "Agt"), config, collector);
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

impl Validate for ProprietaryBankTransactionCodeStructure11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.cd,
            "Cd",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Cd"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.cd,
            "Cd",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Cd"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.issr,
            "Issr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Issr"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.issr,
            "Issr",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Issr"),
            config,
            collector,
        );
    }
}

// ProprietaryDate31: Date in ISO format.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryDate31 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Dt")]
    pub dt: DateAndDateTime2Choice,
}

impl Validate for ProprietaryDate31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        self.dt
            .validate(&helpers::child_path(path, "Dt"), config, collector);
    }
}

// ProprietaryParty51: Proprietary party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProprietaryParty51 {
    #[serde(rename = "Tp")]
    pub tp: String,
    #[serde(rename = "Pty")]
    pub pty: Party40Choice3,
}

impl Validate for ProprietaryParty51 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        self.pty
            .validate(&helpers::child_path(path, "Pty"), config, collector);
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

impl Validate for ProprietaryPrice21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        self.pric
            .validate(&helpers::child_path(path, "Pric"), config, collector);
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

impl Validate for ProprietaryQuantity11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.qty,
            "Qty",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Qty"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.qty,
            "Qty",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Qty"),
            config,
            collector,
        );
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

impl Validate for ProprietaryReference11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.tp,
            "Tp",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tp,
            "Tp",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Tp"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.ref_attr,
            "Ref",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Ref"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.ref_attr,
            "Ref",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Ref"),
            config,
            collector,
        );
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

impl Validate for ProxyAccountIdentification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(320),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
    }
}

// ProxyAccountIdentification12: Identification used to indicate the account identification under another specified name.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountIdentification12 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProxyAccountType1Choice>,
    #[serde(rename = "Id")]
    pub id: String,
}

impl Validate for ProxyAccountIdentification12 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(320),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
    }
}

// ProxyAccountType1Choice: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for ProxyAccountType1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for ProxyAccountType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for Purpose2Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for Rate41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
        if let Some(ref val) = self.vldty_rg
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "VldtyRg"), config, collector);
        }
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

impl Validate for RateType4Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.othr {
            helpers::validate_length(
                val,
                "Othr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Othr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.othr {
            helpers::validate_pattern(
                val,
                "Othr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Othr"),
                config,
                collector,
            );
        }
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

impl Validate for ReferredDocumentInformation71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.nb {
            helpers::validate_length(
                val,
                "Nb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Nb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nb {
            helpers::validate_pattern(
                val,
                "Nb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nb"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.line_dtls
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "LineDtls"), config, collector);
            }
        }
    }
}

// ReferredDocumentType3Choice1: Proprietary identification of the type of the remittance document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType3Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DocumentType6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for ReferredDocumentType3Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
    }
}

// ReferredDocumentType41: Identification of the issuer of the reference document type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType41 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: ReferredDocumentType3Choice1,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for ReferredDocumentType41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.cd_or_prtry
            .validate(&helpers::child_path(path, "CdOrPrtry"), config, collector);
        if let Some(ref val) = self.issr {
            helpers::validate_length(
                val,
                "Issr",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.issr {
            helpers::validate_pattern(
                val,
                "Issr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Issr"),
                config,
                collector,
            );
        }
    }
}

// RemittanceAmount21: Amount of money remitted for the referred document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount21 {
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

impl Validate for RemittanceAmount21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.due_pybl_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DuePyblAmt"), config, collector);
        }
        if let Some(ref vec) = self.dscnt_apld_amt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "DscntApldAmt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.cdt_note_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtNoteAmt"), config, collector);
        }
        if let Some(ref vec) = self.tax_amt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.adjstmnt_amt_and_rsn
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "AdjstmntAmtAndRsn"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.rmtd_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
        }
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

impl Validate for RemittanceAmount31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.due_pybl_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DuePyblAmt"), config, collector);
        }
        if let Some(ref vec) = self.dscnt_apld_amt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "DscntApldAmt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.cdt_note_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtNoteAmt"), config, collector);
        }
        if let Some(ref vec) = self.tax_amt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.adjstmnt_amt_and_rsn
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "AdjstmntAmtAndRsn"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.rmtd_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
        }
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

impl Validate for RemittanceInformation161 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.ustrd {
            helpers::validate_length(
                val,
                "Ustrd",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Ustrd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ustrd {
            helpers::validate_pattern(
                val,
                "Ustrd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Ustrd"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.strd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Strd"), config, collector);
            }
        }
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

impl Validate for RemittanceLocation71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.rmt_id {
            helpers::validate_length(
                val,
                "RmtId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "RmtId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rmt_id {
            helpers::validate_pattern(
                val,
                "RmtId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "RmtId"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.rmt_lctn_dtls
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "RmtLctnDtls"), config, collector);
            }
        }
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

impl Validate for RemittanceLocationData11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.mtd
            .validate(&helpers::child_path(path, "Mtd"), config, collector);
        if let Some(ref val) = self.elctrnc_adr {
            helpers::validate_length(
                val,
                "ElctrncAdr",
                Some(1),
                Some(2048),
                &helpers::child_path(path, "ElctrncAdr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.elctrnc_adr {
            helpers::validate_pattern(
                val,
                "ElctrncAdr",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "ElctrncAdr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
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

impl Validate for RemittanceLocationMethod2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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
    pub ntry_dtls: Option<Vec<Box<EntryDetails91>>>,
    #[serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ntry_inf: Option<String>,
}

impl Validate for ReportEntry101 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.ntry_ref {
            helpers::validate_length(
                val,
                "NtryRef",
                Some(1),
                Some(35),
                &helpers::child_path(path, "NtryRef"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ntry_ref {
            helpers::validate_pattern(
                val,
                "NtryRef",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "NtryRef"),
                config,
                collector,
            );
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        self.sts
            .validate(&helpers::child_path(path, "Sts"), config, collector);
        if let Some(ref val) = self.bookg_dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "BookgDt"), config, collector);
        }
        self.val_dt
            .validate(&helpers::child_path(path, "ValDt"), config, collector);
        if let Some(ref val) = self.acct_svcr_ref {
            helpers::validate_length(
                val,
                "AcctSvcrRef",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AcctSvcrRef"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_svcr_ref {
            helpers::validate_pattern(
                val,
                "AcctSvcrRef",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AcctSvcrRef"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.avlbty
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Avlbty"), config, collector);
            }
        }
        self.bk_tx_cd
            .validate(&helpers::child_path(path, "BkTxCd"), config, collector);
        if let Some(ref val) = self.addtl_inf_ind
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AddtlInfInd"), config, collector);
        }
        if let Some(ref val) = self.amt_dtls
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AmtDtls"), config, collector);
        }
        if let Some(ref val) = self.chrgs
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Chrgs"), config, collector);
        }
        if let Some(ref val) = self.tech_inpt_chanl
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TechInptChanl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.intrst
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Intrst"), config, collector);
        }
        if let Some(ref val) = self.card_tx
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CardTx"), config, collector);
        }
        if let Some(ref vec) = self.ntry_dtls
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "NtryDtls"), config, collector);
            }
        }
        if let Some(ref val) = self.addtl_ntry_inf {
            helpers::validate_length(
                val,
                "AddtlNtryInf",
                Some(1),
                Some(500),
                &helpers::child_path(path, "AddtlNtryInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_ntry_inf {
            helpers::validate_pattern(
                val,
                "AddtlNtryInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AddtlNtryInf"),
                config,
                collector,
            );
        }
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

impl Validate for ReportingSource1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for ReturnReason5Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
    }
}

// SecuritiesAccount191: Description of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SecuritiesAccount191 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification301>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl Validate for SecuritiesAccount191 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(35),
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.id,
            "Id",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "Id"),
            config,
            collector,
        );
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(70),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
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

impl Validate for SecurityIdentification191 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.isin {
            helpers::validate_pattern(
                val,
                "ISIN",
                "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}",
                &helpers::child_path(path, "ISIN"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.othr_id
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "OthrId"), config, collector);
            }
        }
        if let Some(ref val) = self.desc {
            helpers::validate_length(
                val,
                "Desc",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Desc"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.desc {
            helpers::validate_pattern(
                val,
                "Desc",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Desc"),
                config,
                collector,
            );
        }
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

impl Validate for SequenceRange1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.fr_seq {
            helpers::validate_length(
                val,
                "FrSeq",
                Some(1),
                Some(35),
                &helpers::child_path(path, "FrSeq"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.fr_seq {
            helpers::validate_pattern(
                val,
                "FrSeq",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "FrSeq"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.to_seq {
            helpers::validate_length(
                val,
                "ToSeq",
                Some(1),
                Some(35),
                &helpers::child_path(path, "ToSeq"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.to_seq {
            helpers::validate_pattern(
                val,
                "ToSeq",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ToSeq"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.fr_to_seq
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "FrToSeq"), config, collector);
            }
        }
        if let Some(ref vec) = self.eq_seq {
            for item in vec {
                helpers::validate_length(
                    item,
                    "EQSeq",
                    Some(1),
                    Some(35),
                    &helpers::child_path(path, "EQSeq"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.eq_seq {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "EQSeq",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                    &helpers::child_path(path, "EQSeq"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.neq_seq {
            for item in vec {
                helpers::validate_length(
                    item,
                    "NEQSeq",
                    Some(1),
                    Some(35),
                    &helpers::child_path(path, "NEQSeq"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.neq_seq {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "NEQSeq",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                    &helpers::child_path(path, "NEQSeq"),
                    config,
                    collector,
                );
            }
        }
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

impl Validate for SequenceRange11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.fr_seq,
            "FrSeq",
            Some(1),
            Some(35),
            &helpers::child_path(path, "FrSeq"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.fr_seq,
            "FrSeq",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "FrSeq"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.to_seq,
            "ToSeq",
            Some(1),
            Some(35),
            &helpers::child_path(path, "ToSeq"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.to_seq,
            "ToSeq",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "ToSeq"),
            config,
            collector,
        );
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

impl Validate for StructuredRemittanceInformation161 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref vec) = self.rfrd_doc_inf
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "RfrdDocInf"), config, collector);
            }
        }
        if let Some(ref val) = self.rfrd_doc_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RfrdDocAmt"), config, collector);
        }
        if let Some(ref val) = self.cdtr_ref_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrRefInf"), config, collector);
        }
        if let Some(ref val) = self.invcr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Invcr"), config, collector);
        }
        if let Some(ref val) = self.invcee
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Invcee"), config, collector);
        }
        if let Some(ref val) = self.tax_rmt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TaxRmt"), config, collector);
        }
        if let Some(ref val) = self.grnshmt_rmt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "GrnshmtRmt"), config, collector);
        }
        if let Some(ref vec) = self.addtl_rmt_inf {
            for item in vec {
                helpers::validate_length(
                    item,
                    "AddtlRmtInf",
                    Some(1),
                    Some(140),
                    &helpers::child_path(path, "AddtlRmtInf"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.addtl_rmt_inf {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "AddtlRmtInf",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                    &helpers::child_path(path, "AddtlRmtInf"),
                    config,
                    collector,
                );
            }
        }
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

impl Validate for TaxAmount2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.taxbl_base_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TaxblBaseAmt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlAmt"), config, collector);
        }
        if let Some(ref vec) = self.dtls
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Dtls"), config, collector);
            }
        }
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

impl Validate for TaxAmountAndType11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
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

impl Validate for TaxAmountType1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for TaxAuthorisation11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.titl {
            helpers::validate_length(
                val,
                "Titl",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Titl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.titl {
            helpers::validate_pattern(
                val,
                "Titl",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Titl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
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

impl Validate for TaxAuthorisation12 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.titl {
            helpers::validate_length(
                val,
                "Titl",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Titl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.titl {
            helpers::validate_pattern(
                val,
                "Titl",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Titl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_length(
                val,
                "Nm",
                Some(1),
                Some(140),
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.nm {
            helpers::validate_pattern(
                val,
                "Nm",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Nm"),
                config,
                collector,
            );
        }
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

impl Validate for TaxCharges21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.id {
            helpers::validate_length(
                val,
                "Id",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.id {
            helpers::validate_pattern(
                val,
                "Id",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Amt"), config, collector);
        }
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

impl Validate for TaxInformation71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cdtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cdtr"), config, collector);
        }
        if let Some(ref val) = self.dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
        }
        if let Some(ref val) = self.ultmt_dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
        }
        if let Some(ref val) = self.admstn_zone {
            helpers::validate_length(
                val,
                "AdmstnZone",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AdmstnZone"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.admstn_zone {
            helpers::validate_pattern(
                val,
                "AdmstnZone",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "AdmstnZone"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_nb {
            helpers::validate_length(
                val,
                "RefNb",
                Some(1),
                Some(140),
                &helpers::child_path(path, "RefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_nb {
            helpers::validate_pattern(
                val,
                "RefNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "RefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mtd {
            helpers::validate_length(
                val,
                "Mtd",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Mtd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mtd {
            helpers::validate_pattern(
                val,
                "Mtd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Mtd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_taxbl_base_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TtlTaxblBaseAmt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_tax_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlTaxAmt"), config, collector);
        }
        if let Some(ref vec) = self.rcrd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Rcrd"), config, collector);
            }
        }
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

impl Validate for TaxInformation81 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cdtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cdtr"), config, collector);
        }
        if let Some(ref val) = self.dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
        }
        if let Some(ref val) = self.admstn_zone {
            helpers::validate_length(
                val,
                "AdmstnZone",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AdmstnZone"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.admstn_zone {
            helpers::validate_pattern(
                val,
                "AdmstnZone",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AdmstnZone"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_nb {
            helpers::validate_length(
                val,
                "RefNb",
                Some(1),
                Some(140),
                &helpers::child_path(path, "RefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ref_nb {
            helpers::validate_pattern(
                val,
                "RefNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.\\n\\r,'\\+ ]{1,140}",
                &helpers::child_path(path, "RefNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mtd {
            helpers::validate_length(
                val,
                "Mtd",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Mtd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mtd {
            helpers::validate_pattern(
                val,
                "Mtd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Mtd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_taxbl_base_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TtlTaxblBaseAmt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_tax_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlTaxAmt"), config, collector);
        }
        if let Some(ref vec) = self.rcrd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Rcrd"), config, collector);
            }
        }
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

impl Validate for TaxParty11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tax_id {
            helpers::validate_length(
                val,
                "TaxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_id {
            helpers::validate_pattern(
                val,
                "TaxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_length(
                val,
                "RegnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_pattern(
                val,
                "RegnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_length(
                val,
                "TaxTp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_pattern(
                val,
                "TaxTp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
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

impl Validate for TaxParty12 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tax_id {
            helpers::validate_length(
                val,
                "TaxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_id {
            helpers::validate_pattern(
                val,
                "TaxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_length(
                val,
                "RegnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_pattern(
                val,
                "RegnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_length(
                val,
                "TaxTp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_pattern(
                val,
                "TaxTp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
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

impl Validate for TaxParty21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tax_id {
            helpers::validate_length(
                val,
                "TaxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_id {
            helpers::validate_pattern(
                val,
                "TaxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_length(
                val,
                "RegnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_pattern(
                val,
                "RegnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_length(
                val,
                "TaxTp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_pattern(
                val,
                "TaxTp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.authstn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Authstn"), config, collector);
        }
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

impl Validate for TaxParty22 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tax_id {
            helpers::validate_length(
                val,
                "TaxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_id {
            helpers::validate_pattern(
                val,
                "TaxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "TaxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_length(
                val,
                "RegnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.regn_id {
            helpers::validate_pattern(
                val,
                "RegnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "RegnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_length(
                val,
                "TaxTp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tax_tp {
            helpers::validate_pattern(
                val,
                "TaxTp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "TaxTp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.authstn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Authstn"), config, collector);
        }
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

impl Validate for TaxPeriod2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.fr_to_dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrToDt"), config, collector);
        }
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

impl Validate for TaxRecord21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            helpers::validate_length(
                val,
                "Tp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Tp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tp {
            helpers::validate_pattern(
                val,
                "Tp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Tp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy {
            helpers::validate_length(
                val,
                "Ctgy",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Ctgy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy {
            helpers::validate_pattern(
                val,
                "Ctgy",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "Ctgy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy_dtls {
            helpers::validate_length(
                val,
                "CtgyDtls",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtgyDtls"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy_dtls {
            helpers::validate_pattern(
                val,
                "CtgyDtls",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "CtgyDtls"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dbtr_sts {
            helpers::validate_length(
                val,
                "DbtrSts",
                Some(1),
                Some(35),
                &helpers::child_path(path, "DbtrSts"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dbtr_sts {
            helpers::validate_pattern(
                val,
                "DbtrSts",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "DbtrSts"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cert_id {
            helpers::validate_length(
                val,
                "CertId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CertId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cert_id {
            helpers::validate_pattern(
                val,
                "CertId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "CertId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.frms_cd {
            helpers::validate_length(
                val,
                "FrmsCd",
                Some(1),
                Some(35),
                &helpers::child_path(path, "FrmsCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.frms_cd {
            helpers::validate_pattern(
                val,
                "FrmsCd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "FrmsCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prd"), config, collector);
        }
        if let Some(ref val) = self.tax_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_length(
                val,
                "AddtlInf",
                Some(1),
                Some(140),
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_pattern(
                val,
                "AddtlInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
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

impl Validate for TaxRecord22 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            helpers::validate_length(
                val,
                "Tp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Tp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tp {
            helpers::validate_pattern(
                val,
                "Tp",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Tp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy {
            helpers::validate_length(
                val,
                "Ctgy",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Ctgy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy {
            helpers::validate_pattern(
                val,
                "Ctgy",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Ctgy"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy_dtls {
            helpers::validate_length(
                val,
                "CtgyDtls",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CtgyDtls"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ctgy_dtls {
            helpers::validate_pattern(
                val,
                "CtgyDtls",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "CtgyDtls"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dbtr_sts {
            helpers::validate_length(
                val,
                "DbtrSts",
                Some(1),
                Some(35),
                &helpers::child_path(path, "DbtrSts"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.dbtr_sts {
            helpers::validate_pattern(
                val,
                "DbtrSts",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "DbtrSts"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cert_id {
            helpers::validate_length(
                val,
                "CertId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "CertId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cert_id {
            helpers::validate_pattern(
                val,
                "CertId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "CertId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.frms_cd {
            helpers::validate_length(
                val,
                "FrmsCd",
                Some(1),
                Some(35),
                &helpers::child_path(path, "FrmsCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.frms_cd {
            helpers::validate_pattern(
                val,
                "FrmsCd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "FrmsCd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prd"), config, collector);
        }
        if let Some(ref val) = self.tax_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_length(
                val,
                "AddtlInf",
                Some(1),
                Some(140),
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.addtl_inf {
            helpers::validate_pattern(
                val,
                "AddtlInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.\\n\\r,'\\+ ]{1,140}",
                &helpers::child_path(path, "AddtlInf"),
                config,
                collector,
            );
        }
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

impl Validate for TaxRecordDetails2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.prd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prd"), config, collector);
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
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

impl Validate for TaxRecordPeriod1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for TechnicalInputChannel1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(4),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_pattern(
                val,
                "Prtry",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
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

impl Validate for TotalTransactions61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.ttl_ntries
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlNtries"), config, collector);
        }
        if let Some(ref val) = self.ttl_cdt_ntries
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TtlCdtNtries"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_dbt_ntries
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TtlDbtNtries"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.ttl_ntries_per_bk_tx_cd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "TtlNtriesPerBkTxCd"),
                    config,
                    collector,
                );
            }
        }
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

impl Validate for TotalsPerBankTransactionCode51 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nb_of_ntries {
            helpers::validate_pattern(
                val,
                "NbOfNtries",
                "[0-9]{1,15}",
                &helpers::child_path(path, "NbOfNtries"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ttl_net_ntry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TtlNetNtry"), config, collector);
        }
        if let Some(ref val) = self.cdt_ntries
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtNtries"), config, collector);
        }
        if let Some(ref val) = self.dbt_ntries
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtNtries"), config, collector);
        }
        self.bk_tx_cd
            .validate(&helpers::child_path(path, "BkTxCd"), config, collector);
        if let Some(ref vec) = self.avlbty
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Avlbty"), config, collector);
            }
        }
        if let Some(ref val) = self.dt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dt"), config, collector);
        }
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

impl Validate for TrackData11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.trck_nb {
            helpers::validate_pattern(
                val,
                "TrckNb",
                "[0-9]",
                &helpers::child_path(path, "TrckNb"),
                config,
                collector,
            );
        }
        helpers::validate_length(
            &self.trck_val,
            "TrckVal",
            Some(1),
            Some(140),
            &helpers::child_path(path, "TrckVal"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.trck_val,
            "TrckVal",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "TrckVal"),
            config,
            collector,
        );
    }
}

// TransactionAgents51: Proprietary agent related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionAgents51 {
    #[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none")]
    pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none")]
    pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none")]
    pub issg_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none")]
    pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryAgent41>>,
}

impl Validate for TransactionAgents51 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instg_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InstgAgt"), config, collector);
        }
        if let Some(ref val) = self.instd_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InstdAgt"), config, collector);
        }
        if let Some(ref val) = self.dbtr_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
        }
        if let Some(ref val) = self.cdtr_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrAgt"), config, collector);
        }
        if let Some(ref val) = self.intrmy_agt1
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IntrmyAgt1"), config, collector);
        }
        if let Some(ref val) = self.intrmy_agt2
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IntrmyAgt2"), config, collector);
        }
        if let Some(ref val) = self.intrmy_agt3
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IntrmyAgt3"), config, collector);
        }
        if let Some(ref val) = self.rcvg_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RcvgAgt"), config, collector);
        }
        if let Some(ref val) = self.dlvrg_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DlvrgAgt"), config, collector);
        }
        if let Some(ref val) = self.issg_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IssgAgt"), config, collector);
        }
        if let Some(ref val) = self.sttlm_plc
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SttlmPlc"), config, collector);
        }
        if let Some(ref vec) = self.prtry
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Prtry"), config, collector);
            }
        }
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

impl Validate for TransactionChannel1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for TransactionDates31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.accptnc_dt_tm {
            helpers::validate_pattern(
                val,
                "AccptncDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "AccptncDtTm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tx_dt_tm {
            helpers::validate_pattern(
                val,
                "TxDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "TxDtTm"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.prtry
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Prtry"), config, collector);
            }
        }
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

impl Validate for TransactionEnvironment1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for TransactionIdentifier11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.tx_dt_tm,
            "TxDtTm",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "TxDtTm"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.tx_ref,
            "TxRef",
            Some(1),
            Some(35),
            &helpers::child_path(path, "TxRef"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.tx_ref,
            "TxRef",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "TxRef"),
            config,
            collector,
        );
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

impl Validate for TransactionInterest41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.ttl_intrst_and_tax_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "TtlIntrstAndTaxAmt"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.rcrd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Rcrd"), config, collector);
            }
        }
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
    pub dbtr_acct: Option<CashAccount383>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<Party40Choice3>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<Party40Choice3>,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount383>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<Party40Choice3>,
    #[serde(rename = "TradgPty", skip_serializing_if = "Option::is_none")]
    pub tradg_pty: Option<Party40Choice3>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryParty51>>,
}

impl Validate for TransactionParties61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.initg_pty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InitgPty"), config, collector);
        }
        if let Some(ref val) = self.dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
        }
        if let Some(ref val) = self.dbtr_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAcct"), config, collector);
        }
        if let Some(ref val) = self.ultmt_dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
        }
        if let Some(ref val) = self.cdtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cdtr"), config, collector);
        }
        if let Some(ref val) = self.cdtr_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrAcct"), config, collector);
        }
        if let Some(ref val) = self.ultmt_cdtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "UltmtCdtr"), config, collector);
        }
        if let Some(ref val) = self.tradg_pty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "TradgPty"), config, collector);
        }
        if let Some(ref vec) = self.prtry
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Prtry"), config, collector);
            }
        }
    }
}

// TransactionPrice4Choice1: Proprietary price specification related to the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionPrice4Choice1 {
    #[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
    pub deal_pric: Option<Price7>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Vec<ProprietaryPrice21>>,
}

impl Validate for TransactionPrice4Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.deal_pric
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DealPric"), config, collector);
        }
        if let Some(ref vec) = self.prtry
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Prtry"), config, collector);
            }
        }
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

impl Validate for TransactionQuantities3Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.qty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Qty"), config, collector);
        }
        if let Some(ref val) = self.orgnl_and_cur_face_amt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlAndCurFaceAmt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prtry
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prtry"), config, collector);
        }
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

impl Validate for TransactionReferences61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.msg_id {
            helpers::validate_length(
                val,
                "MsgId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MsgId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.msg_id {
            helpers::validate_pattern(
                val,
                "MsgId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MsgId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_svcr_ref {
            helpers::validate_length(
                val,
                "AcctSvcrRef",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AcctSvcrRef"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_svcr_ref {
            helpers::validate_pattern(
                val,
                "AcctSvcrRef",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AcctSvcrRef"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pmt_inf_id {
            helpers::validate_length(
                val,
                "PmtInfId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "PmtInfId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pmt_inf_id {
            helpers::validate_pattern(
                val,
                "PmtInfId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "PmtInfId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.instr_id {
            helpers::validate_length(
                val,
                "InstrId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "InstrId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.instr_id {
            helpers::validate_pattern(
                val,
                "InstrId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "InstrId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.end_to_end_id {
            helpers::validate_length(
                val,
                "EndToEndId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "EndToEndId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.end_to_end_id {
            helpers::validate_pattern(
                val,
                "EndToEndId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "EndToEndId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.uetr {
            helpers::validate_pattern(
                val,
                "UETR",
                "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}",
                &helpers::child_path(path, "UETR"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tx_id {
            helpers::validate_length(
                val,
                "TxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "TxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.tx_id {
            helpers::validate_pattern(
                val,
                "TxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "TxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mndt_id {
            helpers::validate_length(
                val,
                "MndtId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MndtId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mndt_id {
            helpers::validate_pattern(
                val,
                "MndtId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MndtId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.chq_nb {
            helpers::validate_length(
                val,
                "ChqNb",
                Some(1),
                Some(35),
                &helpers::child_path(path, "ChqNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.chq_nb {
            helpers::validate_pattern(
                val,
                "ChqNb",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ChqNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.clr_sys_ref {
            helpers::validate_length(
                val,
                "ClrSysRef",
                Some(1),
                Some(35),
                &helpers::child_path(path, "ClrSysRef"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.clr_sys_ref {
            helpers::validate_pattern(
                val,
                "ClrSysRef",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "ClrSysRef"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_ownr_tx_id {
            helpers::validate_length(
                val,
                "AcctOwnrTxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AcctOwnrTxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_ownr_tx_id {
            helpers::validate_pattern(
                val,
                "AcctOwnrTxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AcctOwnrTxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_svcr_tx_id {
            helpers::validate_length(
                val,
                "AcctSvcrTxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "AcctSvcrTxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.acct_svcr_tx_id {
            helpers::validate_pattern(
                val,
                "AcctSvcrTxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "AcctSvcrTxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mkt_infrstrctr_tx_id {
            helpers::validate_length(
                val,
                "MktInfrstrctrTxId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "MktInfrstrctrTxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mkt_infrstrctr_tx_id {
            helpers::validate_pattern(
                val,
                "MktInfrstrctrTxId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "MktInfrstrctrTxId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prcg_id {
            helpers::validate_length(
                val,
                "PrcgId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "PrcgId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prcg_id {
            helpers::validate_pattern(
                val,
                "PrcgId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "PrcgId"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.prtry
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Prtry"), config, collector);
            }
        }
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

impl Validate for UnitOfMeasure1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for UserInterface2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for YieldedOrValueType1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.val_tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ValTp"), config, collector);
        }
    }
}
