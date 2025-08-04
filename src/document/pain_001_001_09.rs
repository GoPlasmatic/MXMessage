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

// AccountIdentification4Choice: Unique identification of an account, as assigned by the account servicer, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountIdentification4Choice {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
}

impl Validate for AccountIdentification4Choice {
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
        if let Some(ref val) = self.othr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Othr"), config, collector);
            }
        }
    }
}

// AccountSchemeName1Choice: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for AccountSchemeName1Choice {
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

// AddressType3Choice: Type of address expressed as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddressType3Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}

impl Validate for AddressType3Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cd"), config, collector);
            }
        }
        if let Some(ref val) = self.prtry {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prtry"), config, collector);
            }
        }
    }
}

// AmountType4Choice: Amount of money to be moved between the debtor and creditor, expressed in the currency of the debtor's account, and the currency in which the amount is to be moved.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmountType4Choice {
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none")]
    pub eqvt_amt: Option<EquivalentAmount2>,
}

impl Validate for AmountType4Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instd_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "InstdAmt"), config, collector);
            }
        }
        if let Some(ref val) = self.eqvt_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "EqvtAmt"), config, collector);
            }
        }
    }
}

// Authorisation1Choice: Specifies the authorisation, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Authorisation1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Authorisation1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for Authorisation1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cd"), config, collector);
            }
        }
        if let Some(ref val) = self.prtry {
            helpers::validate_length(
                val,
                "Prtry",
                Some(1),
                Some(128),
                &helpers::child_path(path, "Prtry"),
                config,
                collector,
            );
        }
    }
}

// Authorisation1Code: Indicates that a file requires all customer transactions to be authorised or approved.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Authorisation1Code {
    #[default]
    #[serde(rename = "AUTH")]
    CodeAUTH,
    #[serde(rename = "FDET")]
    CodeFDET,
    #[serde(rename = "FSUM")]
    CodeFSUM,
    #[serde(rename = "ILEV")]
    CodeILEV,
}

impl Validate for Authorisation1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// BranchAndFinancialInstitutionIdentification6: Identifies a specific branch of a financial institution.
//
// Usage: This component should be used in case the identification information in the financial institution component does not provide identification up to branch level.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData3>,
}

impl Validate for BranchAndFinancialInstitutionIdentification6 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
        if let Some(ref val) = self.brnch_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "BrnchId"), config, collector);
            }
        }
    }
}

// BranchAndFinancialInstitutionIdentification61: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification61 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification181,
}

impl Validate for BranchAndFinancialInstitutionIdentification61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
    }
}

// BranchAndFinancialInstitutionIdentification62: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification62 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification182,
}

impl Validate for BranchAndFinancialInstitutionIdentification62 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
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
    pub brnch_id: Option<BranchData31>,
}

impl Validate for BranchAndFinancialInstitutionIdentification63 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
        if let Some(ref val) = self.brnch_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "BrnchId"), config, collector);
            }
        }
    }
}

// BranchData3: Information that locates and identifies a specific address, as defined by postal services.
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

impl Validate for BranchData3 {
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
    }
}

// BranchData31: Unique and unambiguous identification of a branch of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchData31 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    }
}

// CashAccount38: Specifies an alternate assumed name for the identification of the account.
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

impl Validate for CashAccount38 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
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
        if let Some(ref val) = self.prxy {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prxy"), config, collector);
            }
        }
    }
}

// CashAccountType2Choice: Nature or use of the account in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccountType2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for CashAccountType2Choice {
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

// CategoryPurpose1Choice: Category purpose, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CategoryPurpose1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for CategoryPurpose1Choice {
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

// ChargeBearerType1Code__1: In a credit transfer context, means that transaction charges on the sender side are to be borne by the debtor, transaction charges on the receiver side are to be borne by the creditor. In a direct debit context, means that transaction charges on the sender side are to be borne by the creditor, transaction charges on the receiver side are to be borne by the debtor.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChargeBearerType1Code1 {
    #[default]
    #[serde(rename = "DEBT")]
    CodeDEBT,
    #[serde(rename = "CRED")]
    CodeCRED,
    #[serde(rename = "SHAR")]
    CodeSHAR,
}

impl Validate for ChargeBearerType1Code1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// Cheque111: Signature to be used by the cheque servicer on a specific cheque to be printed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Cheque111 {
    #[serde(rename = "ChqTp", skip_serializing_if = "Option::is_none")]
    pub chq_tp: Option<ChequeType2Code1>,
    #[serde(rename = "ChqNb", skip_serializing_if = "Option::is_none")]
    pub chq_nb: Option<String>,
    #[serde(rename = "ChqFr", skip_serializing_if = "Option::is_none")]
    pub chq_fr: Option<NameAndAddress161>,
    #[serde(rename = "DlvryMtd", skip_serializing_if = "Option::is_none")]
    pub dlvry_mtd: Option<ChequeDeliveryMethod1Choice>,
    #[serde(rename = "DlvrTo", skip_serializing_if = "Option::is_none")]
    pub dlvr_to: Option<NameAndAddress161>,
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ChqMtrtyDt", skip_serializing_if = "Option::is_none")]
    pub chq_mtrty_dt: Option<String>,
    #[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
    pub frms_cd: Option<String>,
    #[serde(rename = "MemoFld", skip_serializing_if = "Option::is_none")]
    pub memo_fld: Option<Vec<String>>,
    #[serde(rename = "RgnlClrZone", skip_serializing_if = "Option::is_none")]
    pub rgnl_clr_zone: Option<String>,
    #[serde(rename = "PrtLctn", skip_serializing_if = "Option::is_none")]
    pub prt_lctn: Option<String>,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<Vec<String>>,
}

impl Validate for Cheque111 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.chq_tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ChqTp"), config, collector);
            }
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
        if let Some(ref val) = self.chq_fr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ChqFr"), config, collector);
            }
        }
        if let Some(ref val) = self.dlvry_mtd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DlvryMtd"), config, collector);
            }
        }
        if let Some(ref val) = self.dlvr_to {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DlvrTo"), config, collector);
            }
        }
        if let Some(ref val) = self.instr_prty {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "InstrPrty"), config, collector);
            }
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
        if let Some(ref vec) = self.memo_fld {
            for item in vec {
                helpers::validate_length(
                    item,
                    "MemoFld",
                    Some(1),
                    Some(35),
                    &helpers::child_path(path, "MemoFld"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.rgnl_clr_zone {
            helpers::validate_length(
                val,
                "RgnlClrZone",
                Some(1),
                Some(35),
                &helpers::child_path(path, "RgnlClrZone"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prt_lctn {
            helpers::validate_length(
                val,
                "PrtLctn",
                Some(1),
                Some(35),
                &helpers::child_path(path, "PrtLctn"),
                config,
                collector,
            );
        }
        if let Some(ref vec) = self.sgntr {
            for item in vec {
                helpers::validate_length(
                    item,
                    "Sgntr",
                    Some(1),
                    Some(70),
                    &helpers::child_path(path, "Sgntr"),
                    config,
                    collector,
                );
            }
        }
    }
}

// ChequeDelivery1Code: Cheque is to be sent through registered mail services to creditor agent.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChequeDelivery1Code {
    #[default]
    #[serde(rename = "MLDB")]
    CodeMLDB,
    #[serde(rename = "MLCD")]
    CodeMLCD,
    #[serde(rename = "MLFA")]
    CodeMLFA,
    #[serde(rename = "CRDB")]
    CodeCRDB,
    #[serde(rename = "CRCD")]
    CodeCRCD,
    #[serde(rename = "CRFA")]
    CodeCRFA,
    #[serde(rename = "PUDB")]
    CodePUDB,
    #[serde(rename = "PUCD")]
    CodePUCD,
    #[serde(rename = "PUFA")]
    CodePUFA,
    #[serde(rename = "RGDB")]
    CodeRGDB,
    #[serde(rename = "RGCD")]
    CodeRGCD,
    #[serde(rename = "RGFA")]
    CodeRGFA,
}

impl Validate for ChequeDelivery1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// ChequeDeliveryMethod1Choice: Specifies a proprietary delivery method of the cheque by the debtor's agent.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChequeDeliveryMethod1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ChequeDelivery1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for ChequeDeliveryMethod1Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cd"), config, collector);
            }
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

// ChequeType2Code__1: A guaranteed bank cheque with a future value date (do not pay before], which in commercial terms is a 'negotiatable instrument': the beneficiary can receive early payment from any bank under subtraction of a discount. The ordering customer's account is debited on value date.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChequeType2Code1 {
    #[default]
    #[serde(rename = "CCHQ")]
    CodeCCHQ,
    #[serde(rename = "BCHQ")]
    CodeBCHQ,
    #[serde(rename = "DRFT")]
    CodeDRFT,
}

impl Validate for ChequeType2Code1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// ClearingSystemIdentification2Choice: Identification code for a clearing system, that has not yet been identified in the list of clearing systems.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for ClearingSystemIdentification2Choice {
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

// ClearingSystemMemberIdentification2: Identification of a member of a clearing system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[serde(rename = "MmbId")]
    pub mmb_id: String,
}

impl Validate for ClearingSystemMemberIdentification2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.clr_sys_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ClrSysId"), config, collector);
            }
        }
        helpers::validate_length(
            &self.mmb_id,
            "MmbId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "MmbId"),
            config,
            collector,
        );
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
            Some(35),
            &helpers::child_path(path, "MmbId"),
            config,
            collector,
        );
    }
}

// Contact4: Preferred method used to reach the contact.
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

impl Validate for Contact4 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.nm_prfx {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "NmPrfx"), config, collector);
            }
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
        if let Some(ref val) = self.phne_nb {
            helpers::validate_pattern(
                val,
                "PhneNb",
                "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}",
                &helpers::child_path(path, "PhneNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.mob_nb {
            helpers::validate_pattern(
                val,
                "MobNb",
                "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}",
                &helpers::child_path(path, "MobNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.fax_nb {
            helpers::validate_pattern(
                val,
                "FaxNb",
                "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}",
                &helpers::child_path(path, "FaxNb"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.email_adr {
            helpers::validate_length(
                val,
                "EmailAdr",
                Some(1),
                Some(2048),
                &helpers::child_path(path, "EmailAdr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.email_purp {
            helpers::validate_length(
                val,
                "EmailPurp",
                Some(1),
                Some(35),
                &helpers::child_path(path, "EmailPurp"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.job_titl {
            helpers::validate_length(
                val,
                "JobTitl",
                Some(1),
                Some(35),
                &helpers::child_path(path, "JobTitl"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rspnsblty {
            helpers::validate_length(
                val,
                "Rspnsblty",
                Some(1),
                Some(35),
                &helpers::child_path(path, "Rspnsblty"),
                config,
                collector,
            );
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
        if let Some(ref vec) = self.othr {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Othr"), config, collector);
                }
            }
        }
        if let Some(ref val) = self.prefrd_mtd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PrefrdMtd"), config, collector);
            }
        }
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

// CreditTransferTransaction341: Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditTransferTransaction341 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification61,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation261>,
    #[serde(rename = "Amt")]
    pub amt: AmountType4Choice,
    #[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
    pub xchg_rate_inf: Option<ExchangeRate1>,
    #[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<ChargeBearerType1Code1>,
    #[serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none")]
    pub chq_instr: Option<Cheque111>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification1353>,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1_acct: Option<CashAccount38>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2_acct: Option<CashAccount38>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification62>,
    #[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3_acct: Option<CashAccount38>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification63>,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification1354,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount38>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<PartyIdentification1353>,
    #[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent1>>,
    #[serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt: Option<String>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice>,
    #[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
    pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
    #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInformation8>,
    #[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
    pub rltd_rmt_inf: Option<Vec<RemittanceLocation71>>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation161>,
}

impl Validate for CreditTransferTransaction341 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.pmt_id
            .validate(&helpers::child_path(path, "PmtId"), config, collector);
        if let Some(ref val) = self.pmt_tp_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PmtTpInf"), config, collector);
            }
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.xchg_rate_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "XchgRateInf"), config, collector);
            }
        }
        if let Some(ref val) = self.chrg_br {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ChrgBr"), config, collector);
            }
        }
        if let Some(ref val) = self.chq_instr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ChqInstr"), config, collector);
            }
        }
        if let Some(ref val) = self.ultmt_dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
            }
        }
        if let Some(ref val) = self.intrmy_agt1 {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "IntrmyAgt1"), config, collector);
            }
        }
        if let Some(ref val) = self.intrmy_agt1_acct {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "IntrmyAgt1Acct"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.intrmy_agt2 {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "IntrmyAgt2"), config, collector);
            }
        }
        if let Some(ref val) = self.intrmy_agt2_acct {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "IntrmyAgt2Acct"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.intrmy_agt3 {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "IntrmyAgt3"), config, collector);
            }
        }
        if let Some(ref val) = self.intrmy_agt3_acct {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "IntrmyAgt3Acct"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.cdtr_agt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrAgt"), config, collector);
            }
        }
        if let Some(ref val) = self.cdtr_agt_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrAgtAcct"), config, collector);
            }
        }
        self.cdtr
            .validate(&helpers::child_path(path, "Cdtr"), config, collector);
        if let Some(ref val) = self.cdtr_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrAcct"), config, collector);
            }
        }
        if let Some(ref val) = self.ultmt_cdtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "UltmtCdtr"), config, collector);
            }
        }
        if let Some(ref vec) = self.instr_for_cdtr_agt {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(
                        &helpers::child_path(path, "InstrForCdtrAgt"),
                        config,
                        collector,
                    );
                }
            }
        }
        if let Some(ref val) = self.instr_for_dbtr_agt {
            helpers::validate_length(
                val,
                "InstrForDbtrAgt",
                Some(1),
                Some(140),
                &helpers::child_path(path, "InstrForDbtrAgt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.purp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Purp"), config, collector);
            }
        }
        if let Some(ref vec) = self.rgltry_rptg {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "RgltryRptg"), config, collector);
                }
            }
        }
        if let Some(ref val) = self.tax {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tax"), config, collector);
            }
        }
        if let Some(ref vec) = self.rltd_rmt_inf {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "RltdRmtInf"), config, collector);
                }
            }
        }
        if let Some(ref val) = self.rmt_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RmtInf"), config, collector);
            }
        }
    }
}

// CreditorReferenceInformation2: Unique reference, as assigned by the creditor, to unambiguously refer to the payment transaction.
//
// Usage: If available, the initiating party should provide this reference in the structured remittance information, to enable reconciliation by the creditor upon receipt of the amount of money.
//
// If the business context requires the use of a creditor reference or a payment remit identification, and only one identifier can be passed through the end-to-end chain, the creditor's reference or payment remittance identification should be quoted in the end-to-end transaction identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceInformation2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditorReferenceType2>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub ref_attr: Option<String>,
}

impl Validate for CreditorReferenceInformation2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
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
        if let Some(ref val) = self.cd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cd"), config, collector);
            }
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

// CreditorReferenceType2: Entity that assigns the credit reference type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType2 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: CreditorReferenceType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for CreditorReferenceType2 {
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
    }
}

// CustomerCreditTransferInitiationV09: Set of characteristics that applies to the debit side of the payment transactions included in the credit transfer initiation.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomerCreditTransferInitiationV09 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader851,
    #[serde(rename = "PmtInf")]
    pub pmt_inf: PaymentInstruction301,
}

impl Validate for CustomerCreditTransferInitiationV09 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.pmt_inf
            .validate(&helpers::child_path(path, "PmtInf"), config, collector);
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

impl Validate for DateAndPlaceOfBirth1 {
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
            &self.ctry_of_birth,
            "CtryOfBirth",
            "[A-Z]{2,2}",
            &helpers::child_path(path, "CtryOfBirth"),
            config,
            collector,
        );
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

// DiscountAmountAndType1: Amount of money, which has been typed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountAndType1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DiscountAmountType1Choice>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl Validate for DiscountAmountAndType1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
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

impl Validate for DiscountAmountType1Choice {
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

// DocumentAdjustment1: Provides further details on the document adjustment.
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

impl Validate for DocumentAdjustment1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.cdt_dbt_ind {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
            }
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
    }
}

// DocumentLineIdentification1: Date associated with the referred document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<DocumentLineType1>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<String>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<String>,
}

impl Validate for DocumentLineIdentification1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
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
    }
}

// DocumentLineInformation1: Provides details on the amounts of the document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineInformation1 {
    #[serde(rename = "Id")]
    pub id: Vec<DocumentLineIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<RemittanceAmount3>,
}

impl Validate for DocumentLineInformation1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        for item in &self.id {
            item.validate(&helpers::child_path(path, "Id"), config, collector);
        }
        if let Some(ref val) = self.desc {
            helpers::validate_length(
                val,
                "Desc",
                Some(1),
                Some(2048),
                &helpers::child_path(path, "Desc"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Amt"), config, collector);
            }
        }
    }
}

// DocumentLineType1: Identification of the issuer of the reference document line identificationtype.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: DocumentLineType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for DocumentLineType1 {
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
    }
}

// DocumentLineType1Choice: Proprietary identification of the type of the remittance document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for DocumentLineType1Choice {
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

// EquivalentAmount2: Specifies the currency of the to be transferred amount, which is different from the currency of the debtor's account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EquivalentAmount2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyOfTrf")]
    pub ccy_of_trf: String,
}

impl Validate for EquivalentAmount2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        helpers::validate_pattern(
            &self.ccy_of_trf,
            "CcyOfTrf",
            "[A-Z]{3,3}",
            &helpers::child_path(path, "CcyOfTrf"),
            config,
            collector,
        );
    }
}

// ExchangeRate1: Unique and unambiguous reference to the foreign exchange contract agreed between the initiating party/creditor and the debtor agent.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExchangeRate1 {
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<String>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<f64>,
    #[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
    pub rate_tp: Option<ExchangeRateType1Code>,
    #[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
    pub ctrct_id: Option<String>,
}

impl Validate for ExchangeRate1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
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
        if let Some(ref val) = self.rate_tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RateTp"), config, collector);
            }
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

// ExchangeRateType1Code: Exchange rate applied is the rate agreed between the parties.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ExchangeRateType1Code {
    #[default]
    #[serde(rename = "SPOT")]
    CodeSPOT,
    #[serde(rename = "SALE")]
    CodeSALE,
    #[serde(rename = "AGRD")]
    CodeAGRD,
}

impl Validate for ExchangeRateType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// FinancialIdentificationSchemeName1Choice: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for FinancialIdentificationSchemeName1Choice {
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

// FinancialInstitutionIdentification18: Unique identification of an agent, as assigned by an institution, using an identification scheme.
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

impl Validate for FinancialInstitutionIdentification18 {
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
        if let Some(ref val) = self.clr_sys_mmb_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ClrSysMmbId"), config, collector);
            }
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
        if let Some(ref val) = self.othr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Othr"), config, collector);
            }
        }
    }
}

// FinancialInstitutionIdentification181: Legal entity identifier of the financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification181 {
    #[serde(rename = "BICFI")]
    pub bicfi: String,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
}

impl Validate for FinancialInstitutionIdentification181 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.bicfi,
            "BICFI",
            "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
            &helpers::child_path(path, "BICFI"),
            config,
            collector,
        );
        if let Some(ref val) = self.clr_sys_mmb_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ClrSysMmbId"), config, collector);
            }
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
    }
}

// FinancialInstitutionIdentification182: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification182 {
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

impl Validate for FinancialInstitutionIdentification182 {
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
        if let Some(ref val) = self.clr_sys_mmb_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ClrSysMmbId"), config, collector);
            }
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
    }
}

// Garnishment31: Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Garnishment31 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType1,
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
        if let Some(ref val) = self.grnshee {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Grnshee"), config, collector);
            }
        }
        if let Some(ref val) = self.grnshmt_admstr {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "GrnshmtAdmstr"),
                    config,
                    collector,
                );
            }
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
        if let Some(ref val) = self.rmtd_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
            }
        }
    }
}

// GarnishmentType1: Identification of the issuer of the garnishment type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: GarnishmentType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GarnishmentType1 {
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
    }
}

// GarnishmentType1Choice: Proprietary identification of the type of garnishment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for GarnishmentType1Choice {
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

// GenericAccountIdentification1: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericAccountIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericAccountIdentification1 {
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
        if let Some(ref val) = self.schme_nm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
            }
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
    }
}

// GenericFinancialIdentification1: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericFinancialIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericFinancialIdentification1 {
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
        if let Some(ref val) = self.schme_nm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
            }
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
    }
}

// GenericIdentification30: Short textual description of the scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Issr")]
    pub issr: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<String>,
}

impl Validate for GenericIdentification30 {
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
    }
}

// GenericOrganisationIdentification1: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericOrganisationIdentification1 {
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
        if let Some(ref val) = self.schme_nm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
            }
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
        if let Some(ref val) = self.schme_nm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
            }
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
    }
}

// GenericPersonIdentification1: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification1 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericPersonIdentification1 {
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
        if let Some(ref val) = self.schme_nm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SchmeNm"), config, collector);
            }
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
    }
}

// GenericPersonIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm")]
    pub schme_nm: PersonIdentificationSchemeName1Choice1,
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
        self.schme_nm
            .validate(&helpers::child_path(path, "SchmeNm"), config, collector);
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
    }
}

// GroupHeader851: Financial institution that receives the instruction from the initiating party and forwards it to the next agent in the payment chain for execution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader851 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
    pub authstn: Option<Vec<Authorisation1Choice>>,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericTextfixed,
    #[serde(rename = "InitgPty")]
    pub initg_pty: PartyIdentification1351,
    #[serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none")]
    pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Validate for GroupHeader851 {
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
        if let Some(ref vec) = self.authstn {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Authstn"), config, collector);
                }
            }
        }
        self.nb_of_txs
            .validate(&helpers::child_path(path, "NbOfTxs"), config, collector);
        self.initg_pty
            .validate(&helpers::child_path(path, "InitgPty"), config, collector);
        if let Some(ref val) = self.fwdg_agt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "FwdgAgt"), config, collector);
            }
        }
    }
}

// Instruction3Code: Please advise/contact (ultimate) creditor/claimant by the most efficient means of telecommunication.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Instruction3Code {
    #[default]
    #[serde(rename = "CHQB")]
    CodeCHQB,
    #[serde(rename = "HOLD")]
    CodeHOLD,
    #[serde(rename = "PHOB")]
    CodePHOB,
    #[serde(rename = "TELB")]
    CodeTELB,
}

impl Validate for Instruction3Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// InstructionForCreditorAgent1: Further information complementing the coded instruction or instruction to the creditor's agent that is bilaterally agreed or specific to a user community.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstructionForCreditorAgent1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Instruction3Code>,
    #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<String>,
}

impl Validate for InstructionForCreditorAgent1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cd"), config, collector);
            }
        }
        if let Some(ref val) = self.instr_inf {
            helpers::validate_length(
                val,
                "InstrInf",
                Some(1),
                Some(140),
                &helpers::child_path(path, "InstrInf"),
                config,
                collector,
            );
        }
    }
}

// LocalInstrument2Choice: Specifies the local instrument, as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalInstrument2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for LocalInstrument2Choice {
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
    }
}

// Max15NumericText_fixed: 1
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Max15NumericTextfixed {
    #[default]
    #[serde(rename = "1")]
    Code1,
}

impl Validate for Max15NumericTextfixed {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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
        self.adr
            .validate(&helpers::child_path(path, "Adr"), config, collector);
    }
}

// NamePrefix2Code: Title of the person is gender neutral (Mx).
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

impl Validate for NamePrefix2Code {
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
    pub othr: Option<Vec<GenericOrganisationIdentification1>>,
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
        if let Some(ref vec) = self.othr {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Othr"), config, collector);
                }
            }
        }
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
    pub othr: Option<Vec<GenericOrganisationIdentification11>>,
}

impl Validate for OrganisationIdentification292 {
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
        if let Some(ref vec) = self.othr {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Othr"), config, collector);
                }
            }
        }
    }
}

// OrganisationIdentificationSchemeName1Choice: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for OrganisationIdentificationSchemeName1Choice {
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

// OrganisationIdentificationSchemeName1Choice1: Name of the identification scheme, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
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
    }
}

// OtherContact1: Communication value such as phone number or email address.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    pub chanl_tp: String,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Validate for OtherContact1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.chanl_tp,
            "ChanlTp",
            Some(1),
            Some(4),
            &helpers::child_path(path, "ChanlTp"),
            config,
            collector,
        );
        if let Some(ref val) = self.id {
            helpers::validate_length(
                val,
                "Id",
                Some(1),
                Some(128),
                &helpers::child_path(path, "Id"),
                config,
                collector,
            );
        }
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
        if let Some(ref val) = self.org_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "OrgId"), config, collector);
            }
        }
        if let Some(ref val) = self.prvt_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PrvtId"), config, collector);
            }
        }
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

impl Validate for Party38Choice2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.org_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "OrgId"), config, collector);
            }
        }
        if let Some(ref val) = self.prvt_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PrvtId"), config, collector);
            }
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
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
        if let Some(ref val) = self.id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Id"), config, collector);
            }
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
        if let Some(ref val) = self.ctct_dtls {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CtctDtls"), config, collector);
            }
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
    pub id: Option<Party38Choice2>,
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
        if let Some(ref val) = self.id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Id"), config, collector);
            }
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

// PartyIdentification1353: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
        if let Some(ref val) = self.id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Id"), config, collector);
            }
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

// PartyIdentification1354: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
        if let Some(ref val) = self.id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Id"), config, collector);
            }
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

// PaymentIdentification61: Universally unique identifier to provide an end-to-end reference of a payment transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentIdentification61 {
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<String>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: String,
    #[serde(rename = "UETR")]
    pub uetr: String,
}

impl Validate for PaymentIdentification61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
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
        helpers::validate_length(
            &self.end_to_end_id,
            "EndToEndId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "EndToEndId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.uetr,
            "UETR",
            "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}",
            &helpers::child_path(path, "UETR"),
            config,
            collector,
        );
    }
}

// PaymentInstruction301: Provides information on the individual transaction(s) included in the message.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentInstruction301 {
    #[serde(rename = "PmtInfId")]
    pub pmt_inf_id: String,
    #[serde(rename = "PmtMtd")]
    pub pmt_mtd: PaymentMethod3Code1,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation261>,
    #[serde(rename = "ReqdExctnDt")]
    pub reqd_exctn_dt: DateAndDateTime2Choice,
    #[serde(rename = "PoolgAdjstmntDt", skip_serializing_if = "Option::is_none")]
    pub poolg_adjstmnt_dt: Option<String>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification1352,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: CashAccount38,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt: Option<String>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification1353>,
    #[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<ChargeBearerType1Code1>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccount38>,
    #[serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtTrfTxInf")]
    pub cdt_trf_tx_inf: CreditTransferTransaction341,
}

impl Validate for PaymentInstruction301 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.pmt_inf_id,
            "PmtInfId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "PmtInfId"),
            config,
            collector,
        );
        self.pmt_mtd
            .validate(&helpers::child_path(path, "PmtMtd"), config, collector);
        if let Some(ref val) = self.pmt_tp_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PmtTpInf"), config, collector);
            }
        }
        self.reqd_exctn_dt
            .validate(&helpers::child_path(path, "ReqdExctnDt"), config, collector);
        self.dbtr
            .validate(&helpers::child_path(path, "Dbtr"), config, collector);
        self.dbtr_acct
            .validate(&helpers::child_path(path, "DbtrAcct"), config, collector);
        self.dbtr_agt
            .validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
        if let Some(ref val) = self.dbtr_agt_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DbtrAgtAcct"), config, collector);
            }
        }
        if let Some(ref val) = self.instr_for_dbtr_agt {
            helpers::validate_length(
                val,
                "InstrForDbtrAgt",
                Some(1),
                Some(140),
                &helpers::child_path(path, "InstrForDbtrAgt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.ultmt_dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
            }
        }
        if let Some(ref val) = self.chrg_br {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ChrgBr"), config, collector);
            }
        }
        if let Some(ref val) = self.chrgs_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ChrgsAcct"), config, collector);
            }
        }
        if let Some(ref val) = self.chrgs_acct_agt {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "ChrgsAcctAgt"),
                    config,
                    collector,
                );
            }
        }
        self.cdt_trf_tx_inf
            .validate(&helpers::child_path(path, "CdtTrfTxInf"), config, collector);
    }
}

// PaymentMethod3Code__1: Transfer of an amount of money in the books of the account servicer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PaymentMethod3Code1 {
    #[default]
    #[serde(rename = "CHK")]
    CodeCHK,
    #[serde(rename = "TRF")]
    CodeTRF,
}

impl Validate for PaymentMethod3Code1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// PaymentTypeInformation261: Specifies the high level purpose of the instruction based on a set of pre-defined categories.
// Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTypeInformation261 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl Validate for PaymentTypeInformation261 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instr_prty {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "InstrPrty"), config, collector);
            }
        }
        if let Some(ref vec) = self.svc_lvl {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "SvcLvl"), config, collector);
                }
            }
        }
        if let Some(ref val) = self.lcl_instrm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "LclInstrm"), config, collector);
            }
        }
        if let Some(ref val) = self.ctgy_purp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CtgyPurp"), config, collector);
            }
        }
    }
}

// PersonIdentification131: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification131 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl Validate for PersonIdentification131 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dt_and_plc_of_birth {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "DtAndPlcOfBirth"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.othr {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Othr"), config, collector);
                }
            }
        }
    }
}

// PersonIdentification132: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification132 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification11>>,
}

impl Validate for PersonIdentification132 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dt_and_plc_of_birth {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "DtAndPlcOfBirth"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.othr {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Othr"), config, collector);
                }
            }
        }
    }
}

// PersonIdentificationSchemeName1Choice: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for PersonIdentificationSchemeName1Choice {
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

// PersonIdentificationSchemeName1Choice1: Name of the identification scheme, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
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
    }
}

// PostalAddress24: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
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

impl Validate for PostalAddress24 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.adr_tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "AdrTp"), config, collector);
            }
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
    }
}

// PostalAddress241: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress241 {
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
    #[serde(rename = "TwnNm")]
    pub twn_nm: String,
    #[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<String>,
    #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<String>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<String>,
    #[serde(rename = "Ctry")]
    pub ctry: String,
    #[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
    pub adr_line: Option<Vec<String>>,
}

impl Validate for PostalAddress241 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
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
        helpers::validate_length(
            &self.twn_nm,
            "TwnNm",
            Some(1),
            Some(35),
            &helpers::child_path(path, "TwnNm"),
            config,
            collector,
        );
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
        helpers::validate_pattern(
            &self.ctry,
            "Ctry",
            "[A-Z]{2,2}",
            &helpers::child_path(path, "Ctry"),
            config,
            collector,
        );
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

impl Validate for PostalAddress242 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
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

// Priority2Code: Priority level is normal.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority2Code {
    #[default]
    #[serde(rename = "HIGH")]
    CodeHIGH,
    #[serde(rename = "NORM")]
    CodeNORM,
}

impl Validate for Priority2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// ProxyAccountIdentification1: Identification used to indicate the account identification under another specified name.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProxyAccountType1Choice>,
    #[serde(rename = "Id")]
    pub id: String,
}

impl Validate for ProxyAccountIdentification1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
        }
        helpers::validate_length(
            &self.id,
            "Id",
            Some(1),
            Some(2048),
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

// Purpose2Choice: Purpose, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Purpose2Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for Purpose2Choice {
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

// ReferredDocumentInformation7: Set of elements used to provide the content of the referred document line.
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

impl Validate for ReferredDocumentInformation7 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
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
        if let Some(ref vec) = self.line_dtls {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "LineDtls"), config, collector);
                }
            }
        }
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

impl Validate for ReferredDocumentType3Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cd"), config, collector);
            }
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

// ReferredDocumentType4: Identification of the issuer of the reference document type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType4 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: ReferredDocumentType3Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for ReferredDocumentType4 {
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
    }
}

// RegulatoryAuthority2: Country of the entity that requires the regulatory reporting information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<String>,
}

impl Validate for RegulatoryAuthority2 {
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
    }
}

// RegulatoryReporting3: Set of elements used to provide details on the regulatory reporting information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
    #[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
    pub authrty: Option<RegulatoryAuthority2>,
    #[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
    pub dtls: Option<Vec<StructuredRegulatoryReporting3>>,
}

impl Validate for RegulatoryReporting3 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dbt_cdt_rptg_ind {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "DbtCdtRptgInd"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.authrty {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Authrty"), config, collector);
            }
        }
        if let Some(ref vec) = self.dtls {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Dtls"), config, collector);
                }
            }
        }
    }
}

// RegulatoryReportingType1Code: Regulatory information applies to both credit and debit sides.
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

impl Validate for RegulatoryReportingType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// RemittanceAmount2: Amount of money remitted for the referred document.
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

impl Validate for RemittanceAmount2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.due_pybl_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DuePyblAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.dscnt_apld_amt {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(
                        &helpers::child_path(path, "DscntApldAmt"),
                        config,
                        collector,
                    );
                }
            }
        }
        if let Some(ref val) = self.cdt_note_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtNoteAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.tax_amt {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
                }
            }
        }
        if let Some(ref vec) = self.adjstmnt_amt_and_rsn {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(
                        &helpers::child_path(path, "AdjstmntAmtAndRsn"),
                        config,
                        collector,
                    );
                }
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
            }
        }
    }
}

// RemittanceAmount3: Amount of money remitted.
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

impl Validate for RemittanceAmount3 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.due_pybl_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DuePyblAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.dscnt_apld_amt {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(
                        &helpers::child_path(path, "DscntApldAmt"),
                        config,
                        collector,
                    );
                }
            }
        }
        if let Some(ref val) = self.cdt_note_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtNoteAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.tax_amt {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
                }
            }
        }
        if let Some(ref vec) = self.adjstmnt_amt_and_rsn {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(
                        &helpers::child_path(path, "AdjstmntAmtAndRsn"),
                        config,
                        collector,
                    );
                }
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
            }
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
        if let Some(ref vec) = self.strd {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Strd"), config, collector);
                }
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
        if let Some(ref vec) = self.rmt_lctn_dtls {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "RmtLctnDtls"), config, collector);
                }
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
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

// ServiceLevel8Choice: Specifies a pre-agreed service or level of service between the parties, as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServiceLevel8Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for ServiceLevel8Choice {
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

// StructuredRegulatoryReporting3: Additional details that cater for specific domestic regulatory requirements.
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

impl Validate for StructuredRegulatoryReporting3 {
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
        if let Some(ref val) = self.cd {
            helpers::validate_length(
                val,
                "Cd",
                Some(1),
                Some(10),
                &helpers::child_path(path, "Cd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Amt"), config, collector);
            }
        }
        if let Some(ref vec) = self.inf {
            for item in vec {
                helpers::validate_length(
                    item,
                    "Inf",
                    Some(1),
                    Some(35),
                    &helpers::child_path(path, "Inf"),
                    config,
                    collector,
                );
            }
        }
    }
}

// StructuredRemittanceInformation161: Additional information, in free text form, to complement the structured remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRemittanceInformation161 {
    #[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
    #[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<RemittanceAmount2>,
    #[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
    #[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
    pub invcr: Option<PartyIdentification1353>,
    #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification1353>,
    #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxInformation7>,
    #[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt: Option<Garnishment31>,
    #[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rmt_inf: Option<Vec<String>>,
}

impl Validate for StructuredRemittanceInformation161 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref vec) = self.rfrd_doc_inf {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "RfrdDocInf"), config, collector);
                }
            }
        }
        if let Some(ref val) = self.rfrd_doc_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RfrdDocAmt"), config, collector);
            }
        }
        if let Some(ref val) = self.cdtr_ref_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrRefInf"), config, collector);
            }
        }
        if let Some(ref val) = self.invcr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Invcr"), config, collector);
            }
        }
        if let Some(ref val) = self.invcee {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Invcee"), config, collector);
            }
        }
        if let Some(ref val) = self.tax_rmt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "TaxRmt"), config, collector);
            }
        }
        if let Some(ref val) = self.grnshmt_rmt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "GrnshmtRmt"), config, collector);
            }
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
        if let Some(ref val) = self.taxbl_base_amt {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "TaxblBaseAmt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.ttl_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "TtlAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.dtls {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Dtls"), config, collector);
                }
            }
        }
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

impl Validate for TaxAmountAndType1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
        }
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
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

impl Validate for TaxAmountType1Choice {
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

// TaxAuthorisation1: Name of the debtor or the debtor's authorised representative.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAuthorisation1 {
    #[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
    pub titl: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
}

impl Validate for TaxAuthorisation1 {
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
    }
}

// TaxInformation7: Record of tax details.
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

impl Validate for TaxInformation7 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cdtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cdtr"), config, collector);
            }
        }
        if let Some(ref val) = self.dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
            }
        }
        if let Some(ref val) = self.ultmt_dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
            }
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
        if let Some(ref val) = self.ttl_taxbl_base_amt {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "TtlTaxblBaseAmt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.ttl_tax_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "TtlTaxAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.rcrd {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Rcrd"), config, collector);
                }
            }
        }
    }
}

// TaxInformation8: Record of tax details.
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

impl Validate for TaxInformation8 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cdtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Cdtr"), config, collector);
            }
        }
        if let Some(ref val) = self.dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
            }
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
        if let Some(ref val) = self.ttl_taxbl_base_amt {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "TtlTaxblBaseAmt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.ttl_tax_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "TtlTaxAmt"), config, collector);
            }
        }
        if let Some(ref vec) = self.rcrd {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "Rcrd"), config, collector);
                }
            }
        }
    }
}

// TaxParty1: Type of tax payer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty1 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<String>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<String>,
}

impl Validate for TaxParty1 {
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
    }
}

// TaxParty2: Details of the authorised tax paying party.
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

impl Validate for TaxParty2 {
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
        if let Some(ref val) = self.authstn {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Authstn"), config, collector);
            }
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
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
        }
        if let Some(ref val) = self.fr_to_dt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "FrToDt"), config, collector);
            }
        }
    }
}

// TaxRecord2: Further details of the tax record.
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

impl Validate for TaxRecord2 {
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
        if let Some(ref val) = self.prd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prd"), config, collector);
            }
        }
        if let Some(ref val) = self.tax_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "TaxAmt"), config, collector);
            }
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
        if let Some(ref val) = self.prd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prd"), config, collector);
            }
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
