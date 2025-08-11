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
        if let Some(ref val) = self.othr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Othr"), config, collector);
            }
        }
    }
}

// AccountIdentification4Choice2: Unique identification of an account, as assigned by the account servicer, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountIdentification4Choice2 {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification12>,
}

impl Validate for AccountIdentification4Choice2 {
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

// AmendmentInformationDetails131: Original number of tracking days that has been modified.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmendmentInformationDetails131 {
    #[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
    pub orgnl_mndt_id: Option<String>,
    #[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_schme_id: Option<PartyIdentification1351>,
    #[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt_acct: Option<CashAccount382>,
    #[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr: Option<PartyIdentification1351>,
    #[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_acct: Option<CashAccount382>,
    #[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt_acct: Option<CashAccount382>,
    #[serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none")]
    pub orgnl_fnl_colltn_dt: Option<String>,
    #[serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none")]
    pub orgnl_frqcy: Option<Frequency36Choice>,
    #[serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none")]
    pub orgnl_rsn: Option<MandateSetupReason1Choice1>,
    #[serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none")]
    pub orgnl_trckg_days: Option<String>,
}

impl Validate for AmendmentInformationDetails131 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.orgnl_mndt_id {
            helpers::validate_length(
                val,
                "OrgnlMndtId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "OrgnlMndtId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_mndt_id {
            helpers::validate_pattern(
                val,
                "OrgnlMndtId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "OrgnlMndtId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_cdtr_schme_id {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "OrgnlCdtrSchmeId"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.orgnl_cdtr_agt {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "OrgnlCdtrAgt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.orgnl_cdtr_agt_acct {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "OrgnlCdtrAgtAcct"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.orgnl_dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "OrgnlDbtr"), config, collector);
            }
        }
        if let Some(ref val) = self.orgnl_dbtr_acct {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "OrgnlDbtrAcct"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.orgnl_dbtr_agt {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "OrgnlDbtrAgt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.orgnl_dbtr_agt_acct {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "OrgnlDbtrAgtAcct"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.orgnl_frqcy {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "OrgnlFrqcy"), config, collector);
            }
        }
        if let Some(ref val) = self.orgnl_rsn {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "OrgnlRsn"), config, collector);
            }
        }
        if let Some(ref val) = self.orgnl_trckg_days {
            helpers::validate_pattern(
                val,
                "OrgnlTrckgDays",
                "[0-9]{2}",
                &helpers::child_path(path, "OrgnlTrckgDays"),
                config,
                collector,
            );
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

// BranchAndFinancialInstitutionIdentification63: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification63 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification183,
}

impl Validate for BranchAndFinancialInstitutionIdentification63 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
    }
}

// CBPRAmount: CBPR_Amount
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CBPRAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

impl Validate for CBPRAmount {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
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

impl Validate for CashAccount381 {
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
        if let Some(ref val) = self.prxy {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prxy"), config, collector);
            }
        }
    }
}

// CashAccount382: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount382 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice2,
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
        if let Some(ref val) = self.prxy {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prxy"), config, collector);
            }
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
    pub prxy: Option<ProxyAccountIdentification12>,
}

impl Validate for CashAccount383 {
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
        if let Some(ref val) = self.prxy {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prxy"), config, collector);
            }
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

// CategoryPurpose1Choice1: Category purpose, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CategoryPurpose1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for CategoryPurpose1Choice1 {
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

// Charges71: Agent that takes the transaction charges or to which the transaction charges are due.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charges71 {
    #[serde(rename = "Amt")]
    pub amt: CBPRAmount,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification61,
}

impl Validate for Charges71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.agt
            .validate(&helpers::child_path(path, "Agt"), config, collector);
    }
}

// ClearingChannel2Code: Payment through internal book transfer.
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

impl Validate for ClearingChannel2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

// DirectDebitTransaction101: Date on which the creditor notifies the debtor about the amount and date on which the direct debit instruction will be presented to the debtor's agent.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectDebitTransaction101 {
    #[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf: Option<MandateRelatedInformation141>,
    #[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
    pub cdtr_schme_id: Option<PartyIdentification1352>,
    #[serde(rename = "PreNtfctnId", skip_serializing_if = "Option::is_none")]
    pub pre_ntfctn_id: Option<String>,
    #[serde(rename = "PreNtfctnDt", skip_serializing_if = "Option::is_none")]
    pub pre_ntfctn_dt: Option<String>,
}

impl Validate for DirectDebitTransaction101 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.mndt_rltd_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "MndtRltdInf"), config, collector);
            }
        }
        if let Some(ref val) = self.cdtr_schme_id {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrSchmeId"), config, collector);
            }
        }
        if let Some(ref val) = self.pre_ntfctn_id {
            helpers::validate_length(
                val,
                "PreNtfctnId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "PreNtfctnId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.pre_ntfctn_id {
            helpers::validate_pattern(
                val,
                "PreNtfctnId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "PreNtfctnId"),
                config,
                collector,
            );
        }
    }
}

// DirectDebitTransactionInformation241: Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectDebitTransactionInformation241 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification71,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation271>,
    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: CBPRAmount,
    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: String,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<Priority3Code>,
    #[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_indctn: Option<SettlementDateTimeIndication11>,
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<CBPRAmount>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<f64>,
    #[serde(rename = "ChrgBr")]
    pub chrg_br: ChargeBearerType1Code1,
    #[serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none")]
    pub chrgs_inf: Option<Vec<Charges71>>,
    #[serde(rename = "ReqdColltnDt")]
    pub reqd_colltn_dt: String,
    #[serde(rename = "DrctDbtTx", skip_serializing_if = "Option::is_none")]
    pub drct_dbt_tx: Option<DirectDebitTransaction101>,
    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification1351,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount382>,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification61,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount382>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<PartyIdentification1353>,
    #[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<PartyIdentification1353>,
    #[serde(rename = "InstgAgt")]
    pub instg_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "InstdAgt")]
    pub instd_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification63>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1_acct: Option<CashAccount382>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2_acct: Option<CashAccount382>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3_acct: Option<CashAccount383>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification1354,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: CashAccount382,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification61,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount383>,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification1353>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice1>,
    #[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
    pub rgltry_rptg: Option<Vec<RegulatoryReporting31>>,
    #[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
    pub rltd_rmt_inf: Option<Vec<RemittanceLocation71>>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation161>,
}

impl Validate for DirectDebitTransactionInformation241 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.pmt_id
            .validate(&helpers::child_path(path, "PmtId"), config, collector);
        if let Some(ref val) = self.pmt_tp_inf {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PmtTpInf"), config, collector);
            }
        }
        self.intr_bk_sttlm_amt.validate(
            &helpers::child_path(path, "IntrBkSttlmAmt"),
            config,
            collector,
        );
        if let Some(ref val) = self.sttlm_prty {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SttlmPrty"), config, collector);
            }
        }
        if let Some(ref val) = self.sttlm_tm_indctn {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "SttlmTmIndctn"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.instd_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "InstdAmt"), config, collector);
            }
        }
        self.chrg_br
            .validate(&helpers::child_path(path, "ChrgBr"), config, collector);
        if let Some(ref vec) = self.chrgs_inf {
            if config.validate_optional_fields {
                for item in vec {
                    item.validate(&helpers::child_path(path, "ChrgsInf"), config, collector);
                }
            }
        }
        if let Some(ref val) = self.drct_dbt_tx {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DrctDbtTx"), config, collector);
            }
        }
        self.cdtr
            .validate(&helpers::child_path(path, "Cdtr"), config, collector);
        if let Some(ref val) = self.cdtr_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrAcct"), config, collector);
            }
        }
        self.cdtr_agt
            .validate(&helpers::child_path(path, "CdtrAgt"), config, collector);
        if let Some(ref val) = self.cdtr_agt_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "CdtrAgtAcct"), config, collector);
            }
        }
        if let Some(ref val) = self.ultmt_cdtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "UltmtCdtr"), config, collector);
            }
        }
        if let Some(ref val) = self.initg_pty {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "InitgPty"), config, collector);
            }
        }
        self.instg_agt
            .validate(&helpers::child_path(path, "InstgAgt"), config, collector);
        self.instd_agt
            .validate(&helpers::child_path(path, "InstdAgt"), config, collector);
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
        if let Some(ref val) = self.ultmt_dbtr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
            }
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
        if let Some(ref val) = self.amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Amt"), config, collector);
            }
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

// FIToFICustomerDirectDebitV08: Set of elements providing information specific to the individual direct debit(s).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FIToFICustomerDirectDebitV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader941,
    #[serde(rename = "DrctDbtTxInf")]
    pub drct_dbt_tx_inf: DirectDebitTransactionInformation241,
}

impl Validate for FIToFICustomerDirectDebitV08 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.drct_dbt_tx_inf.validate(
            &helpers::child_path(path, "DrctDbtTxInf"),
            config,
            collector,
        );
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
    pub pstl_adr: Option<PostalAddress241>,
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
    }
}

// FinancialInstitutionIdentification182: Legal entity identifier of the financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification182 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
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
    }
}

// FinancialInstitutionIdentification183: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification183 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress244>,
}

impl Validate for FinancialInstitutionIdentification183 {
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
        if let Some(ref val) = self.pstl_adr {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
            }
        }
    }
}

// Frequency36Choice: Specifies a frequency in terms of an exact point in time or moment within a specified period type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Frequency36Choice {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Frequency6Code>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<FrequencyPeriod1>,
    #[serde(rename = "PtInTm", skip_serializing_if = "Option::is_none")]
    pub pt_in_tm: Option<FrequencyAndMoment1>,
}

impl Validate for Frequency36Choice {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Tp"), config, collector);
            }
        }
        if let Some(ref val) = self.prd {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Prd"), config, collector);
            }
        }
        if let Some(ref val) = self.pt_in_tm {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PtInTm"), config, collector);
            }
        }
    }
}

// Frequency6Code: Event takes place every two weeks.
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

impl Validate for Frequency6Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// FrequencyAndMoment1: Further information on the exact point in time the event should take place.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FrequencyAndMoment1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,
    #[serde(rename = "PtInTm")]
    pub pt_in_tm: String,
}

impl Validate for FrequencyAndMoment1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
        helpers::validate_pattern(
            &self.pt_in_tm,
            "PtInTm",
            "[0-9]{2}",
            &helpers::child_path(path, "PtInTm"),
            config,
            collector,
        );
    }
}

// FrequencyPeriod1: Number of instructions to be created and processed during the specified period.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FrequencyPeriod1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,
    #[serde(rename = "CntPerPrd")]
    pub cnt_per_prd: f64,
}

impl Validate for FrequencyPeriod1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
    }
}

// Garnishment31: Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Garnishment31 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType11,
    #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification1356>,
    #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification1357>,
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
        if let Some(ref val) = self.rmtd_amt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
            }
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

// GarnishmentType11: Identification of the issuer of the garnishment type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType11 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: GarnishmentType1Choice,
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
    pub schme_nm: Option<AccountSchemeName1Choice>,
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

// GenericAccountIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericAccountIdentification12 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice1>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericAccountIdentification12 {
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

// GenericOrganisationIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification12 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm")]
    pub schme_nm: OrganisationIdentificationSchemeName1Choice2,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericOrganisationIdentification12 {
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

// GenericOrganisationIdentification13: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification13 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice3>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericOrganisationIdentification13 {
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

// GenericPersonIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification12 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm")]
    pub schme_nm: PersonIdentificationSchemeName1Choice2,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericPersonIdentification12 {
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

// GenericPersonIdentification13: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification13 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice3>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<String>,
}

impl Validate for GenericPersonIdentification13 {
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

// GroupHeader941: Specifies the details on how the settlement of the transaction(s) between the instructing agent and the instructed agent is completed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader941 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericTextfixed,
    #[serde(rename = "SttlmInf")]
    pub sttlm_inf: SettlementInstruction81,
}

impl Validate for GroupHeader941 {
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
        self.nb_of_txs
            .validate(&helpers::child_path(path, "NbOfTxs"), config, collector);
        self.sttlm_inf
            .validate(&helpers::child_path(path, "SttlmInf"), config, collector);
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

// MandateRelatedInformation141: Specifies the number of days the direct debit instruction must be tracked.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MandateRelatedInformation141 {
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<String>,
    #[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr: Option<String>,
    #[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind: Option<bool>,
    #[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
    pub amdmnt_inf_dtls: Option<AmendmentInformationDetails131>,
    #[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr: Option<String>,
    #[serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none")]
    pub frst_colltn_dt: Option<String>,
    #[serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none")]
    pub fnl_colltn_dt: Option<String>,
    #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<Frequency36Choice>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<MandateSetupReason1Choice1>,
    #[serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none")]
    pub trckg_days: Option<String>,
}

impl Validate for MandateRelatedInformation141 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
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
        if let Some(ref val) = self.amdmnt_inf_dtls {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "AmdmntInfDtls"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.elctrnc_sgntr {
            helpers::validate_length(
                val,
                "ElctrncSgntr",
                Some(1),
                Some(1025),
                &helpers::child_path(path, "ElctrncSgntr"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.frqcy {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Frqcy"), config, collector);
            }
        }
        if let Some(ref val) = self.rsn {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Rsn"), config, collector);
            }
        }
        if let Some(ref val) = self.trckg_days {
            helpers::validate_pattern(
                val,
                "TrckgDays",
                "[0-9]{2}",
                &helpers::child_path(path, "TrckgDays"),
                config,
                collector,
            );
        }
    }
}

// MandateSetupReason1Choice1: Reason for the mandate setup, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MandateSetupReason1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for MandateSetupReason1Choice1 {
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
                Some(70),
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
    pub adr: PostalAddress241,
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
    pub othr: Option<Vec<GenericOrganisationIdentification12>>,
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

// OrganisationIdentification293: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification293 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericOrganisationIdentification13>>,
}

impl Validate for OrganisationIdentification293 {
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

// OrganisationIdentificationSchemeName1Choice2: Name of the identification scheme, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice2 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl Validate for OrganisationIdentificationSchemeName1Choice2 {
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

// OrganisationIdentificationSchemeName1Choice3: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice3 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for OrganisationIdentificationSchemeName1Choice3 {
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
    pub org_id: Option<OrganisationIdentification291>,
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

// Party38Choice3: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice3 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification292>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification133>,
}

impl Validate for Party38Choice3 {
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

// Party38Choice4: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice4 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification293>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification134>,
}

impl Validate for Party38Choice4 {
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

// Party38Choice5: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice5 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification293>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification135>,
}

impl Validate for Party38Choice5 {
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
    pub pstl_adr: Option<PostalAddress243>,
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
    pub pstl_adr: Option<PostalAddress241>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice3>,
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

// PartyIdentification1355: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1355 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress243>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice4>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl Validate for PartyIdentification1355 {
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

// PartyIdentification1356: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1356 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress245>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice5>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl Validate for PartyIdentification1356 {
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

// PartyIdentification1357: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1357 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress245>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice4>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl Validate for PartyIdentification1357 {
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

// PaymentIdentification71: Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentIdentification71 {
    #[serde(rename = "InstrId")]
    pub instr_id: String,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: String,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<String>,
}

impl Validate for PaymentIdentification71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.instr_id,
            "InstrId",
            Some(1),
            Some(16),
            &helpers::child_path(path, "InstrId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.instr_id,
            "InstrId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "InstrId"),
            config,
            collector,
        );
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
            &self.end_to_end_id,
            "EndToEndId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "EndToEndId"),
            config,
            collector,
        );
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
    }
}

// PaymentTypeInformation271: Specifies the high level purpose of the instruction based on a set of pre-defined categories.
// Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTypeInformation271 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<ClearingChannel2Code>,
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<Vec<ServiceLevel8Choice1>>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice1>,
    #[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
    pub seq_tp: Option<SequenceType3Code>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<CategoryPurpose1Choice1>,
}

impl Validate for PaymentTypeInformation271 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instr_prty {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "InstrPrty"), config, collector);
            }
        }
        if let Some(ref val) = self.clr_chanl {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "ClrChanl"), config, collector);
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
        if let Some(ref val) = self.seq_tp {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SeqTp"), config, collector);
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
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification11>>,
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

// PersonIdentification133: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification133 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification12>>,
}

impl Validate for PersonIdentification133 {
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

// PersonIdentification134: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification134 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification13>>,
}

impl Validate for PersonIdentification134 {
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

// PersonIdentification135: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification135 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification13>>,
}

impl Validate for PersonIdentification135 {
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

// PersonIdentificationSchemeName1Choice2: Name of the identification scheme, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice2 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl Validate for PersonIdentificationSchemeName1Choice2 {
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

// PersonIdentificationSchemeName1Choice3: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice3 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for PersonIdentificationSchemeName1Choice3 {
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

// PostalAddress243: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress243 {
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

impl Validate for PostalAddress243 {
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
        helpers::validate_length(
            &self.twn_nm,
            "TwnNm",
            Some(1),
            Some(35),
            &helpers::child_path(path, "TwnNm"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.twn_nm,
            "TwnNm",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
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

// PostalAddress244: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress244 {
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

impl Validate for PostalAddress244 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dept {
            helpers::validate_length(
                val,
                "Dept",
                Some(1),
                Some(35),
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
                Some(35),
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
                Some(35),
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
                Some(35),
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
                Some(35),
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

// PostalAddress245: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress245 {
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

impl Validate for PostalAddress245 {
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

// Priority3Code: Priority level is normal.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority3Code {
    #[default]
    #[serde(rename = "URGT")]
    CodeURGT,
    #[serde(rename = "HIGH")]
    CodeHIGH,
    #[serde(rename = "NORM")]
    CodeNORM,
}

impl Validate for Priority3Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// ProxyAccountIdentification11: Identification used to indicate the account identification under another specified name.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountIdentification11 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProxyAccountType1Choice>,
    #[serde(rename = "Id")]
    pub id: String,
}

impl Validate for ProxyAccountIdentification11 {
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
    pub tp: Option<ProxyAccountType1Choice1>,
    #[serde(rename = "Id")]
    pub id: String,
}

impl Validate for ProxyAccountIdentification12 {
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

// ReferredDocumentType41: Identification of the issuer of the reference document type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType41 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: ReferredDocumentType3Choice,
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

// RegulatoryAuthority21: Country of the entity that requires the regulatory reporting information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryAuthority21 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<String>,
}

impl Validate for RegulatoryAuthority21 {
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
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
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

// RegulatoryReporting31: Set of elements used to provide details on the regulatory reporting information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryReporting31 {
    #[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
    #[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
    pub authrty: Option<RegulatoryAuthority21>,
    #[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
    pub dtls: Option<Vec<StructuredRegulatoryReporting31>>,
}

impl Validate for RegulatoryReporting31 {
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
        if let Some(ref val) = self.rmt_id {
            helpers::validate_pattern(
                val,
                "RmtId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
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

// SequenceType3Code: Collection used to re-present previously reversed or returned direct debit transactions.
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

impl Validate for SequenceType3Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// ServiceLevel8Choice1: Specifies a pre-agreed service or level of service between the parties, as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServiceLevel8Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for ServiceLevel8Choice1 {
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

// SettlementDateTimeIndication11: Date and time at which a payment has been credited at the transaction administrator. In the case of TARGET, the date and time at which the payment has been credited at the receiving central bank, expressed in Central European Time (CET).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementDateTimeIndication11 {
    #[serde(rename = "DbtDtTm", skip_serializing_if = "Option::is_none")]
    pub dbt_dt_tm: Option<String>,
    #[serde(rename = "CdtDtTm", skip_serializing_if = "Option::is_none")]
    pub cdt_dt_tm: Option<String>,
}

impl Validate for SettlementDateTimeIndication11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dbt_dt_tm {
            helpers::validate_pattern(
                val,
                "DbtDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "DbtDtTm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.cdt_dt_tm {
            helpers::validate_pattern(
                val,
                "CdtDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "CdtDtTm"),
                config,
                collector,
            );
        }
    }
}

// SettlementInstruction81: A specific purpose account used to post debit and credit entries as a result of the transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementInstruction81 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: SettlementMethod2Code1,
    #[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
    pub sttlm_acct: Option<CashAccount381>,
}

impl Validate for SettlementInstruction81 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.sttlm_mtd
            .validate(&helpers::child_path(path, "SttlmMtd"), config, collector);
        if let Some(ref val) = self.sttlm_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "SttlmAcct"), config, collector);
            }
        }
    }
}

// SettlementMethod2Code__1: Settlement is done by the agent instructing and forwarding the payment to the next party in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SettlementMethod2Code1 {
    #[default]
    #[serde(rename = "INDA")]
    CodeINDA,
    #[serde(rename = "INGA")]
    CodeINGA,
}

impl Validate for SettlementMethod2Code1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// StructuredRegulatoryReporting31: Additional details that cater for specific domestic regulatory requirements.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRegulatoryReporting31 {
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

impl Validate for StructuredRegulatoryReporting31 {
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
        if let Some(ref val) = self.cd {
            helpers::validate_pattern(
                val,
                "Cd",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
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
        if let Some(ref vec) = self.inf {
            for item in vec {
                helpers::validate_pattern(
                    item,
                    "Inf",
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
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
    pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation71>>,
    #[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<RemittanceAmount2>,
    #[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation21>,
    #[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
    pub invcr: Option<PartyIdentification1355>,
    #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification1355>,
    #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxInformation71>,
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
