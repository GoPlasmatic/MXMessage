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

// CreditTransferTransaction361: Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditTransferTransaction361 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification71,
    #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation281>,
    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: CBPRAmount,
    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: String,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<Priority3Code>,
    #[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_indctn: Option<SettlementDateTimeIndication11>,
    #[serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_req: Option<SettlementTimeRequest21>,
    #[serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1_acct: Option<CashAccount381>,
    #[serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2_acct: Option<CashAccount381>,
    #[serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3_acct: Option<CashAccount381>,
    #[serde(rename = "InstgAgt")]
    pub instg_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "InstdAgt")]
    pub instd_agt: BranchAndFinancialInstitutionIdentification62,
    #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1_acct: Option<CashAccount381>,
    #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2_acct: Option<CashAccount381>,
    #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3_acct: Option<CashAccount381>,
    #[serde(rename = "Dbtr")]
    pub dbtr: BranchAndFinancialInstitutionIdentification61,
    #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount381>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount381>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount381>,
    #[serde(rename = "Cdtr")]
    pub cdtr: BranchAndFinancialInstitutionIdentification61,
    #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount381>,
    #[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent21>>,
    #[serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none")]
    pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent11>>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice1>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation21>,
}

impl Validate for CreditTransferTransaction361 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.pmt_id
            .validate(&helpers::child_path(path, "PmtId"), config, collector);
        if let Some(ref val) = self.pmt_tp_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "PmtTpInf"), config, collector);
        }
        self.intr_bk_sttlm_amt.validate(
            &helpers::child_path(path, "IntrBkSttlmAmt"),
            config,
            collector,
        );
        if let Some(ref val) = self.sttlm_prty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SttlmPrty"), config, collector);
        }
        if let Some(ref val) = self.sttlm_tm_indctn
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "SttlmTmIndctn"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.sttlm_tm_req
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SttlmTmReq"), config, collector);
        }
        if let Some(ref val) = self.prvs_instg_agt1
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PrvsInstgAgt1"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prvs_instg_agt1_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PrvsInstgAgt1Acct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prvs_instg_agt2
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PrvsInstgAgt2"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prvs_instg_agt2_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PrvsInstgAgt2Acct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prvs_instg_agt3
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PrvsInstgAgt3"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.prvs_instg_agt3_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "PrvsInstgAgt3Acct"),
                config,
                collector,
            );
        }
        self.instg_agt
            .validate(&helpers::child_path(path, "InstgAgt"), config, collector);
        self.instd_agt
            .validate(&helpers::child_path(path, "InstdAgt"), config, collector);
        if let Some(ref val) = self.intrmy_agt1
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IntrmyAgt1"), config, collector);
        }
        if let Some(ref val) = self.intrmy_agt1_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "IntrmyAgt1Acct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.intrmy_agt2
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IntrmyAgt2"), config, collector);
        }
        if let Some(ref val) = self.intrmy_agt2_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "IntrmyAgt2Acct"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.intrmy_agt3
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "IntrmyAgt3"), config, collector);
        }
        if let Some(ref val) = self.intrmy_agt3_acct
            && config.validate_optional_fields
        {
            val.validate(
                &helpers::child_path(path, "IntrmyAgt3Acct"),
                config,
                collector,
            );
        }
        self.dbtr
            .validate(&helpers::child_path(path, "Dbtr"), config, collector);
        if let Some(ref val) = self.dbtr_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAcct"), config, collector);
        }
        if let Some(ref val) = self.dbtr_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
        }
        if let Some(ref val) = self.dbtr_agt_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAgtAcct"), config, collector);
        }
        if let Some(ref val) = self.cdtr_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrAgt"), config, collector);
        }
        if let Some(ref val) = self.cdtr_agt_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrAgtAcct"), config, collector);
        }
        self.cdtr
            .validate(&helpers::child_path(path, "Cdtr"), config, collector);
        if let Some(ref val) = self.cdtr_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CdtrAcct"), config, collector);
        }
        if let Some(ref vec) = self.instr_for_cdtr_agt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "InstrForCdtrAgt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref vec) = self.instr_for_nxt_agt
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(
                    &helpers::child_path(path, "InstrForNxtAgt"),
                    config,
                    collector,
                );
            }
        }
        if let Some(ref val) = self.purp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Purp"), config, collector);
        }
        if let Some(ref val) = self.rmt_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "RmtInf"), config, collector);
        }
    }
}

// FinancialInstitutionCreditTransferV08: Set of elements providing information specific to the individual credit transfer(s).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionCreditTransferV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader931,
    #[serde(rename = "CdtTrfTxInf")]
    pub cdt_trf_tx_inf: CreditTransferTransaction361,
}

impl Validate for FinancialInstitutionCreditTransferV08 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.cdt_trf_tx_inf
            .validate(&helpers::child_path(path, "CdtTrfTxInf"), config, collector);
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

// FinancialInstitutionIdentification182: Legal entity identifier of the financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification182 {
    #[serde(rename = "BICFI")]
    pub bicfi: String,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
}

impl Validate for FinancialInstitutionIdentification182 {
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

// GroupHeader931: Specifies the details on how the settlement of the transaction(s) between the instructing agent and the instructed agent is completed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader931 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericTextfixed,
    #[serde(rename = "SttlmInf")]
    pub sttlm_inf: SettlementInstruction71,
}

impl Validate for GroupHeader931 {
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

// Instruction5Code: Please advise/contact (ultimate) creditor/claimant by the most efficient means of telecommunication.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Instruction5Code {
    #[default]
    #[serde(rename = "PHOB")]
    CodePHOB,
    #[serde(rename = "TELB")]
    CodeTELB,
}

impl Validate for Instruction5Code {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// InstructionForCreditorAgent21: Further information complementing the coded instruction or instruction to the creditor's agent that is bilaterally agreed or specific to a user community.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstructionForCreditorAgent21 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Instruction5Code>,
    #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<String>,
}

impl Validate for InstructionForCreditorAgent21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
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
        if let Some(ref val) = self.instr_inf {
            helpers::validate_pattern(
                val,
                "InstrInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "InstrInf"),
                config,
                collector,
            );
        }
    }
}

// InstructionForNextAgent11: Further information complementing the coded instruction or instruction to the next agent that is bilaterally agreed or specific to a user community.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstructionForNextAgent11 {
    #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<String>,
}

impl Validate for InstructionForNextAgent11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instr_inf {
            helpers::validate_length(
                val,
                "InstrInf",
                Some(1),
                Some(35),
                &helpers::child_path(path, "InstrInf"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.instr_inf {
            helpers::validate_pattern(
                val,
                "InstrInf",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "InstrInf"),
                config,
                collector,
            );
        }
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

// PaymentIdentification71: Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentIdentification71 {
    #[serde(rename = "InstrId")]
    pub instr_id: String,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: String,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "UETR")]
    pub uetr: String,
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
        helpers::validate_pattern(
            &self.uetr,
            "UETR",
            "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}",
            &helpers::child_path(path, "UETR"),
            config,
            collector,
        );
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

// PaymentTypeInformation281: Specifies the high level purpose of the instruction based on a set of pre-defined categories.
// Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTypeInformation281 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<ClearingChannel2Code>,
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<Vec<ServiceLevel8Choice1>>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<LocalInstrument2Choice1>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<CategoryPurpose1Choice1>,
}

impl Validate for PaymentTypeInformation281 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.instr_prty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "InstrPrty"), config, collector);
        }
        if let Some(ref val) = self.clr_chanl
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "ClrChanl"), config, collector);
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
        if let Some(ref val) = self.ctgy_purp
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CtgyPurp"), config, collector);
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

// RemittanceInformation21: Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, for example, commercial invoices in an accounts' receivable system in an unstructured form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceInformation21 {
    #[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
    pub ustrd: Option<String>,
}

impl Validate for RemittanceInformation21 {
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

// SettlementInstruction71: A specific purpose account used to post debit and credit entries as a result of the transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementInstruction71 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: SettlementMethod1Code1,
    #[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
    pub sttlm_acct: Option<CashAccount381>,
}

impl Validate for SettlementInstruction71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.sttlm_mtd
            .validate(&helpers::child_path(path, "SttlmMtd"), config, collector);
        if let Some(ref val) = self.sttlm_acct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "SttlmAcct"), config, collector);
        }
    }
}

// SettlementMethod1Code__1: Settlement is done by the agent instructing and forwarding the payment to the next party in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SettlementMethod1Code1 {
    #[default]
    #[serde(rename = "INDA")]
    CodeINDA,
    #[serde(rename = "INGA")]
    CodeINGA,
}

impl Validate for SettlementMethod1Code1 {
    fn validate(&self, _path: &str, _config: &ParserConfig, _collector: &mut ErrorCollector) {
        // Enum validation is typically empty
    }
}

// SettlementTimeRequest21: Time by when the payment must be settled to avoid rejection.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementTimeRequest21 {
    #[serde(rename = "CLSTm", skip_serializing_if = "Option::is_none")]
    pub cls_tm: Option<String>,
    #[serde(rename = "TillTm", skip_serializing_if = "Option::is_none")]
    pub till_tm: Option<String>,
    #[serde(rename = "FrTm", skip_serializing_if = "Option::is_none")]
    pub fr_tm: Option<String>,
    #[serde(rename = "RjctTm", skip_serializing_if = "Option::is_none")]
    pub rjct_tm: Option<String>,
}

impl Validate for SettlementTimeRequest21 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cls_tm {
            helpers::validate_pattern(
                val,
                "CLSTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "CLSTm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.till_tm {
            helpers::validate_pattern(
                val,
                "TillTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "TillTm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.fr_tm {
            helpers::validate_pattern(
                val,
                "FrTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "FrTm"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.rjct_tm {
            helpers::validate_pattern(
                val,
                "RjctTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "RjctTm"),
                config,
                collector,
            );
        }
    }
}
