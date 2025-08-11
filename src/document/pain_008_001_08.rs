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
        if let Some(ref val) = self.othr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Othr"), config, collector);
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

// AmendmentInformationDetails131: Original number of tracking days that has been modified.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AmendmentInformationDetails131 {
    #[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
    pub orgnl_mndt_id: Option<String>,
    #[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
    #[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification64>,
    #[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr: Option<PartyIdentification135>,
    #[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_acct: Option<CashAccount381>,
    #[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification65>,
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
        if let Some(ref val) = self.orgnl_cdtr_schme_id
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlCdtrSchmeId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_cdtr_agt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlCdtrAgt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_cdtr_agt_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlCdtrAgtAcct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgnlDbtr"), config, collector);
        }
        if let Some(ref val) = self.orgnl_dbtr_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlDbtrAcct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_dbtr_agt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlDbtrAgt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_dbtr_agt_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "OrgnlDbtrAgtAcct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_frqcy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgnlFrqcy"), config, collector);
        }
        if let Some(ref val) = self.orgnl_rsn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgnlRsn"), config, collector);
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

// BranchAndFinancialInstitutionIdentification64: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification64 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification184,
}

impl Validate for BranchAndFinancialInstitutionIdentification64 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
    }
}

// BranchAndFinancialInstitutionIdentification65: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification65 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
}

impl Validate for BranchAndFinancialInstitutionIdentification65 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fin_instn_id
            .validate(&helpers::child_path(path, "FinInstnId"), config, collector);
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
        if let Some(ref val) = self.prxy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prxy"), config, collector);
        }
    }
}

// CashAccount381: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount381 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAccountIdentification1>,
}

impl Validate for CashAccount381 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
        if let Some(ref val) = self.prxy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prxy"), config, collector);
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
        if let Some(ref val) = self.clr_sys_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ClrSysId"), config, collector);
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
        if let Some(ref val) = self.nm_prfx
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "NmPrfx"), config, collector);
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
        if let Some(ref vec) = self.othr
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Othr"), config, collector);
            }
        }
        if let Some(ref val) = self.prefrd_mtd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PrefrdMtd"), config, collector);
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

// CustomerDirectDebitInitiationV08: Set of characteristics that apply to the credit side of the payment transactions included in the direct debit transaction initiation.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomerDirectDebitInitiationV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader831,
    #[serde(rename = "PmtInf")]
    pub pmt_inf: PaymentInstruction291,
}

impl Validate for CustomerDirectDebitInitiationV08 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.pmt_inf
            .validate(&helpers::child_path(path, "PmtInf"), config, collector);
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
    pub cdtr_schme_id: Option<PartyIdentification1353>,
    #[serde(rename = "PreNtfctnId", skip_serializing_if = "Option::is_none")]
    pub pre_ntfctn_id: Option<String>,
    #[serde(rename = "PreNtfctnDt", skip_serializing_if = "Option::is_none")]
    pub pre_ntfctn_dt: Option<String>,
}

impl Validate for DirectDebitTransaction101 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.mndt_rltd_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "MndtRltdInf"), config, collector);
        }
        if let Some(ref val) = self.cdtr_schme_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrSchmeId"), config, collector);
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
    }
}

// DirectDebitTransactionInformation231: Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectDebitTransactionInformation231 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification6,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation291>,
    #[serde(rename = "InstdAmt")]
    pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<ChargeBearerType1Code1>,
    #[serde(rename = "DrctDbtTx", skip_serializing_if = "Option::is_none")]
    pub drct_dbt_tx: Option<DirectDebitTransaction101>,
    #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<PartyIdentification1351>,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification1354,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: CashAccount38,
    #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification1351>,
    #[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_cdtr_agt: Option<String>,
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

impl Validate for DirectDebitTransactionInformation231 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.pmt_id
            .validate(&helpers::child_path(path, "PmtId"), config, collector);
        if let Some(ref val) = self.pmt_tp_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PmtTpInf"), config, collector);
        }
        self.instd_amt
            .validate(&helpers::child_path(path, "InstdAmt"), config, collector);
        if let Some(ref val) = self.chrg_br
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ChrgBr"), config, collector);
        }
        if let Some(ref val) = self.drct_dbt_tx
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DrctDbtTx"), config, collector);
        }
        if let Some(ref val) = self.ultmt_cdtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "UltmtCdtr"), config, collector);
        }
        self.dbtr_agt
            .validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
        if let Some(ref val) = self.dbtr_agt_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAgtAcct"), config, collector);
        }
        self.dbtr
            .validate(&helpers::child_path(path, "Dbtr"), config, collector);
        self.dbtr_acct
            .validate(&helpers::child_path(path, "DbtrAcct"), config, collector);
        if let Some(ref val) = self.ultmt_dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "UltmtDbtr"), config, collector);
        }
        if let Some(ref val) = self.instr_for_cdtr_agt {
            helpers::validate_length(
                val,
                "InstrForCdtrAgt",
                Some(1),
                Some(140),
                &helpers::child_path(path, "InstrForCdtrAgt"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.purp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Purp"), config, collector);
        }
        if let Some(ref vec) = self.rgltry_rptg
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "RgltryRptg"), config, collector);
            }
        }
        if let Some(ref val) = self.tax
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tax"), config, collector);
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
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
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
        if let Some(ref val) = self.amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Amt"), config, collector);
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
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
    }
}

// FinancialInstitutionIdentification183: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification183 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
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
        if let Some(ref val) = self.pstl_adr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PstlAdr"), config, collector);
        }
    }
}

// FinancialInstitutionIdentification184: Unique identification of an agent, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification184 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress243>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}

impl Validate for FinancialInstitutionIdentification184 {
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
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
        }
        if let Some(ref val) = self.prd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prd"), config, collector);
        }
        if let Some(ref val) = self.pt_in_tm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PtInTm"), config, collector);
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
    pub tp: GarnishmentType1,
    #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification1351>,
    #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification1351>,
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
        if let Some(ref val) = self.rmtd_amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RmtdAmt"), config, collector);
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
    }
}

// GenericOrganisationIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification11 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SchmeNm")]
    pub schme_nm: OrganisationIdentificationSchemeName1Choice1,
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

// GroupHeader831: Financial institution that receives the instruction from the initiating party and forwards it to the next agent in the payment chain for execution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader831 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
    pub authstn: Option<Vec<Authorisation1Choice>>,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: String,
    #[serde(rename = "InitgPty")]
    pub initg_pty: PartyIdentification1351,
    #[serde(rename = "FwdgAgt")]
    pub fwdg_agt: BranchAndFinancialInstitutionIdentification61,
}

impl Validate for GroupHeader831 {
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
        if let Some(ref vec) = self.authstn
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Authstn"), config, collector);
            }
        }
        helpers::validate_pattern(
            &self.nb_of_txs,
            "NbOfTxs",
            "[0-9]{1,15}",
            &helpers::child_path(path, "NbOfTxs"),
            config,
            collector,
        );
        self.initg_pty
            .validate(&helpers::child_path(path, "InitgPty"), config, collector);
        self.fwdg_agt
            .validate(&helpers::child_path(path, "FwdgAgt"), config, collector);
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
    pub rsn: Option<MandateSetupReason1Choice>,
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
        if let Some(ref val) = self.amdmnt_inf_dtls
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "AmdmntInfDtls"),
                config,
                collector,
            );
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
        if let Some(ref val) = self.frqcy
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Frqcy"), config, collector);
        }
        if let Some(ref val) = self.rsn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Rsn"), config, collector);
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

// MandateSetupReason1Choice: Reason for the mandate setup, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MandateSetupReason1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for MandateSetupReason1Choice {
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
    }
}

// NameAndAddress161: Postal address of a party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NameAndAddress161 {
    #[serde(rename = "Nm")]
    pub nm: String,
    #[serde(rename = "Adr")]
    pub adr: PostalAddress244,
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

// OrganisationIdentification29: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<String>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl Validate for OrganisationIdentification29 {
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
        if let Some(ref vec) = self.othr
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Othr"), config, collector);
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
        if let Some(ref vec) = self.othr
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Othr"), config, collector);
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

// Party38Choice: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification29>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification13>,
}

impl Validate for Party38Choice {
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

// PartyIdentification135: Set of elements used to indicate how to contact the party.
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

impl Validate for PartyIdentification135 {
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
    pub id: Option<Party38Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
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

// PartyIdentification1354: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1354 {
    #[serde(rename = "Nm")]
    pub nm: String,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress242>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice2>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,
}

impl Validate for PartyIdentification1354 {
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

// PaymentIdentification6: Universally unique identifier to provide an end-to-end reference of a payment transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentIdentification6 {
    #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<String>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: String,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
}

impl Validate for PaymentIdentification6 {
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
    }
}

// PaymentInstruction291: Provides information on the individual transaction(s) included in the message.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentInstruction291 {
    #[serde(rename = "PmtInfId")]
    pub pmt_inf_id: String,
    #[serde(rename = "PmtMtd")]
    pub pmt_mtd: PaymentMethod2Code,
    #[serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none")]
    pub btch_bookg: Option<bool>,
    #[serde(rename = "ReqdColltnDt")]
    pub reqd_colltn_dt: String,
    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification1352,
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: CashAccount38,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount38>,
    #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccount38>,
    #[serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification63>,
    #[serde(rename = "DrctDbtTxInf")]
    pub drct_dbt_tx_inf: Vec<DirectDebitTransactionInformation231>,
}

impl Validate for PaymentInstruction291 {
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
        self.cdtr
            .validate(&helpers::child_path(path, "Cdtr"), config, collector);
        self.cdtr_acct
            .validate(&helpers::child_path(path, "CdtrAcct"), config, collector);
        self.cdtr_agt
            .validate(&helpers::child_path(path, "CdtrAgt"), config, collector);
        if let Some(ref val) = self.cdtr_agt_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrAgtAcct"), config, collector);
        }
        if let Some(ref val) = self.chrgs_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ChrgsAcct"), config, collector);
        }
        if let Some(ref val) = self.chrgs_acct_agt
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "ChrgsAcctAgt"),
                config,
                collector,
            );
        }
        for item in &self.drct_dbt_tx_inf {
            item.validate(
                &helpers::child_path(path, "DrctDbtTxInf"),
                config,
                collector,
            );
        }
    }
}

// PaymentMethod2Code: Collection of an amount of money from the debtor's bank account by the creditor. The amount of money and dates of collections may vary.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum PaymentMethod2Code {
    #[default]
    #[serde(rename = "DD")]
    CodeDD,
}

impl Validate for PaymentMethod2Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// PaymentTypeInformation291: Specifies the high level purpose of the instruction based on a set of pre-defined categories.
// Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTypeInformation291 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,
    #[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
    pub seq_tp: Option<SequenceType3Code>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl Validate for PaymentTypeInformation291 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instr_prty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InstrPrty"), config, collector);
        }
        if let Some(ref vec) = self.svc_lvl
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "SvcLvl"), config, collector);
            }
        }
        if let Some(ref val) = self.lcl_instrm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "LclInstrm"), config, collector);
        }
        if let Some(ref val) = self.seq_tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SeqTp"), config, collector);
        }
        if let Some(ref val) = self.ctgy_purp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CtgyPurp"), config, collector);
        }
    }
}

// PersonIdentification13: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl Validate for PersonIdentification13 {
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
                    Some(35),
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

impl Validate for PostalAddress244 {
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
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
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
        if let Some(ref vec) = self.line_dtls
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "LineDtls"), config, collector);
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
        if let Some(ref val) = self.dbt_cdt_rptg_ind
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "DbtCdtRptgInd"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.authrty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Authrty"), config, collector);
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
        if let Some(ref val) = self.amt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Amt"), config, collector);
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
    pub invcr: Option<PartyIdentification1351>,
    #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification1351>,
    #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxInformation7>,
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
        if let Some(ref val) = self.tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Tp"), config, collector);
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
