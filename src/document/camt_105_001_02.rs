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

// BranchAndFinancialInstitutionIdentification81: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification81 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification231,
}

impl Validate for BranchAndFinancialInstitutionIdentification81 {
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

// CashAccount401: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount401 {
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

impl Validate for CashAccount401 {
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

// ChargeType3Choice1: Charge type, in a coded form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargeType3Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
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
    }
}

// Charges4Choice1: Charges broken down per payment transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charges4Choice1 {
    #[serde(rename = "PerTx", skip_serializing_if = "Option::is_none")]
    pub per_tx: Option<ChargesPerTransaction41>,
}

impl Validate for Charges4Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.per_tx {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "PerTx"), config, collector);
            }
        }
    }
}

// ChargesBreakdown11: Specifies the type of charge.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargesBreakdown11 {
    #[serde(rename = "Amt")]
    pub amt: CBPRAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode1,
    #[serde(rename = "Tp")]
    pub tp: ChargeType3Choice1,
}

impl Validate for ChargesBreakdown11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
        self.tp
            .validate(&helpers::child_path(path, "Tp"), config, collector);
    }
}

// ChargesPaymentNotificationV02: Provides information on the charges to be paid by the charge bearer(s) related to the processing of the underlying transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargesPaymentNotificationV02 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader1261,
    #[serde(rename = "Chrgs")]
    pub chrgs: Charges4Choice1,
}

impl Validate for ChargesPaymentNotificationV02 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.chrgs
            .validate(&helpers::child_path(path, "Chrgs"), config, collector);
    }
}

// ChargesPerTransaction41: Itemised charges record per transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargesPerTransaction41 {
    #[serde(rename = "ChrgsId")]
    pub chrgs_id: String,
    #[serde(rename = "Rcrd")]
    pub rcrd: ChargesPerTransactionRecord41,
}

impl Validate for ChargesPerTransaction41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.chrgs_id,
            "ChrgsId",
            Some(1),
            Some(16),
            &helpers::child_path(path, "ChrgsId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.chrgs_id,
            "ChrgsId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "ChrgsId"),
            config,
            collector,
        );
        self.rcrd
            .validate(&helpers::child_path(path, "Rcrd"), config, collector);
    }
}

// ChargesPerTransactionRecord41: Specifies the account of the debtor agent of the initial transaction, when instructing agent is different from the charges account owner.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargesPerTransactionRecord41 {
    #[serde(rename = "UndrlygTx")]
    pub undrlyg_tx: TransactionReferences71,
    #[serde(rename = "TtlChrgsPerRcrd")]
    pub ttl_chrgs_per_rcrd: TotalCharges81,
    #[serde(rename = "ChrgsBrkdwn")]
    pub chrgs_brkdwn: Vec<ChargesBreakdown11>,
    #[serde(rename = "ValDt")]
    pub val_dt: DateAndDateTime2Choice1,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification81>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount401>,
}

impl Validate for ChargesPerTransactionRecord41 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.undrlyg_tx
            .validate(&helpers::child_path(path, "UndrlygTx"), config, collector);
        self.ttl_chrgs_per_rcrd.validate(
            &helpers::child_path(path, "TtlChrgsPerRcrd"),
            config,
            collector,
        );
        for item in &self.chrgs_brkdwn {
            item.validate(&helpers::child_path(path, "ChrgsBrkdwn"), config, collector);
        }
        self.val_dt
            .validate(&helpers::child_path(path, "ValDt"), config, collector);
        if let Some(ref val) = self.dbtr_agt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
            }
        }
        if let Some(ref val) = self.dbtr_agt_acct {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "DbtrAgtAcct"), config, collector);
            }
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

// CreditDebitCode__1: Operation is a decrease.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CreditDebitCode1 {
    #[default]
    #[serde(rename = "DBIT")]
    CodeDBIT,
}

impl Validate for CreditDebitCode1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// DateAndDateTime2Choice1: Specified date.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndDateTime2Choice1 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
}

impl Validate for DateAndDateTime2Choice1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {}
}

// FinancialInstitutionIdentification231: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification231 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress271>,
}

impl Validate for FinancialInstitutionIdentification231 {
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

// GroupHeader1261: Agent that owns the charges account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader1261 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "ChrgsRqstr")]
    pub chrgs_rqstr: BranchAndFinancialInstitutionIdentification81,
    #[serde(rename = "ChrgsAcct")]
    pub chrgs_acct: CashAccount401,
    #[serde(rename = "ChrgsAcctOwnr", skip_serializing_if = "Option::is_none")]
    pub chrgs_acct_ownr: Option<BranchAndFinancialInstitutionIdentification81>,
}

impl Validate for GroupHeader1261 {
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
        self.chrgs_rqstr
            .validate(&helpers::child_path(path, "ChrgsRqstr"), config, collector);
        self.chrgs_acct
            .validate(&helpers::child_path(path, "ChrgsAcct"), config, collector);
        if let Some(ref val) = self.chrgs_acct_ownr {
            if config.validate_optional_fields {
                val.validate(
                    &helpers::child_path(path, "ChrgsAcctOwnr"),
                    config,
                    collector,
                );
            }
        }
    }
}

// PostalAddress271: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress271 {
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

impl Validate for PostalAddress271 {
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

// TotalCharges81: Indicates whether the total charges amount is a credit or a debit amount.
// Usage: A zero amount is considered to be a credit.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TotalCharges81 {
    #[serde(rename = "NbOfChrgsBrkdwnItms")]
    pub nb_of_chrgs_brkdwn_itms: String,
    #[serde(rename = "TtlChrgsAmt")]
    pub ttl_chrgs_amt: CBPRAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode1,
}

impl Validate for TotalCharges81 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.nb_of_chrgs_brkdwn_itms,
            "NbOfChrgsBrkdwnItms",
            "[0-9]{1,15}",
            &helpers::child_path(path, "NbOfChrgsBrkdwnItms"),
            config,
            collector,
        );
        self.ttl_chrgs_amt
            .validate(&helpers::child_path(path, "TtlChrgsAmt"), config, collector);
        self.cdt_dbt_ind
            .validate(&helpers::child_path(path, "CdtDbtInd"), config, collector);
    }
}

// TransactionReferences71: Identification of the securities transaction assigned by the processor of the instruction other than the securities account owner, the securities account servicer and the market infrastructure.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionReferences71 {
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<String>,
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
    #[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_tx_id: Option<String>,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<String>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<String>,
}

impl Validate for TransactionReferences71 {
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
                Some(16),
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
    }
}
