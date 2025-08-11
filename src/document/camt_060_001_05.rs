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

// AccountReportingRequestV05: Set of elements used to provide further details on the reporting request.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountReportingRequestV05 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader771,
    #[serde(rename = "RptgReq")]
    pub rptg_req: Vec<ReportingRequest51>,
}

impl Validate for AccountReportingRequestV05 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        for item in &self.rptg_req {
            item.validate(&helpers::child_path(path, "RptgReq"), config, collector);
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

// DatePeriodDetails1: End date of the range.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DatePeriodDetails1 {
    #[serde(rename = "FrDt")]
    pub fr_dt: String,
    #[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
    pub to_dt: Option<String>,
}

impl Validate for DatePeriodDetails1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// EntryStatus1Choice1: Entry status, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntryStatus1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl Validate for EntryStatus1Choice1 {
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

impl Validate for FloorLimitType1Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for GroupHeader771 {
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
        if let Some(ref val) = self.msg_sndr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "MsgSndr"), config, collector);
        }
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

impl Validate for Limit2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
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
    pub pty: Option<PartyIdentification1351>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification61>,
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

impl Validate for QueryType3Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
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

impl Validate for ReportingPeriod21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fr_to_dt
            .validate(&helpers::child_path(path, "FrToDt"), config, collector);
        if let Some(ref val) = self.fr_to_tm
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FrToTm"), config, collector);
        }
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
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

impl Validate for ReportingRequest51 {
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
        helpers::validate_length(
            &self.reqd_msg_nm_id,
            "ReqdMsgNmId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "ReqdMsgNmId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.reqd_msg_nm_id,
            "ReqdMsgNmId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "ReqdMsgNmId"),
            config,
            collector,
        );
        if let Some(ref val) = self.acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Acct"), config, collector);
        }
        self.acct_ownr
            .validate(&helpers::child_path(path, "AcctOwnr"), config, collector);
        if let Some(ref val) = self.acct_svcr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "AcctSvcr"), config, collector);
        }
        if let Some(ref val) = self.rptg_prd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RptgPrd"), config, collector);
        }
        if let Some(ref val) = self.rptg_seq
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RptgSeq"), config, collector);
        }
        if let Some(ref val) = self.reqd_tx_tp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ReqdTxTp"), config, collector);
        }
        if let Some(ref vec) = self.reqd_bal_tp
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "ReqdBalTp"), config, collector);
            }
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

// TimePeriodDetails11: Time at which the time span ends.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TimePeriodDetails11 {
    #[serde(rename = "FrTm")]
    pub fr_tm: String,
    #[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
    pub to_tm: Option<String>,
}

impl Validate for TimePeriodDetails11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.fr_tm,
            "FrTm",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "FrTm"),
            config,
            collector,
        );
        if let Some(ref val) = self.to_tm {
            helpers::validate_pattern(
                val,
                "ToTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "ToTm"),
                config,
                collector,
            );
        }
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

impl Validate for TransactionType21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.sts
            .validate(&helpers::child_path(path, "Sts"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        if let Some(ref vec) = self.flr_lmt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "FlrLmt"), config, collector);
            }
        }
    }
}
