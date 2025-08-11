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

// BusinessApplicationHeader51: Relative indication of the processing precedence of the message over a (set of) Business Messages with assigned priorities.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BusinessApplicationHeader51 {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    pub char_set: Option<String>,
    #[serde(rename = "Fr")]
    pub fr: Party44Choice2,
    #[serde(rename = "To")]
    pub to: Party44Choice2,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: String,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: String,
    #[serde(rename = "BizSvc", skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<String>,
    #[serde(rename = "CreDt")]
    pub cre_dt: String,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<String>,
}

impl Validate for BusinessApplicationHeader51 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fr
            .validate(&helpers::child_path(path, "Fr"), config, collector);
        self.to
            .validate(&helpers::child_path(path, "To"), config, collector);
        helpers::validate_length(
            &self.biz_msg_idr,
            "BizMsgIdr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "BizMsgIdr"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.msg_def_idr,
            "MsgDefIdr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "MsgDefIdr"),
            config,
            collector,
        );
        if let Some(ref val) = self.biz_svc {
            helpers::validate_length(
                val,
                "BizSvc",
                Some(1),
                Some(35),
                &helpers::child_path(path, "BizSvc"),
                config,
                collector,
            );
        }
        helpers::validate_pattern(
            &self.cre_dt,
            "CreDt",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "CreDt"),
            config,
            collector,
        );
        if let Some(ref val) = self.cpy_dplct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CpyDplct"), config, collector);
        }
    }
}

// BusinessApplicationHeaderV02: Specifies the Business Application Header(s) of the Business Message(s) to which this Business Message relates.
// Can be used when replying to a query; can also be used when canceling or amending.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename = "AppHdr")]
pub struct BusinessApplicationHeaderV02 {
    #[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
    pub char_set: Option<String>,
    #[serde(rename = "Fr")]
    pub fr: Party44Choice1,
    #[serde(rename = "To")]
    pub to: Party44Choice1,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: String,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: String,
    #[serde(rename = "BizSvc")]
    pub biz_svc: String,
    #[serde(rename = "MktPrctc", skip_serializing_if = "Option::is_none")]
    pub mkt_prctc: Option<ImplementationSpecification1>,
    #[serde(rename = "CreDt")]
    pub cre_dt: String,
    #[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<bool>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<Priority2Code>,
    #[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
    pub rltd: Option<Vec<BusinessApplicationHeader51>>,
}

impl Validate for BusinessApplicationHeaderV02 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.fr
            .validate(&helpers::child_path(path, "Fr"), config, collector);
        self.to
            .validate(&helpers::child_path(path, "To"), config, collector);
        helpers::validate_length(
            &self.biz_msg_idr,
            "BizMsgIdr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "BizMsgIdr"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.msg_def_idr,
            "MsgDefIdr",
            Some(1),
            Some(35),
            &helpers::child_path(path, "MsgDefIdr"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.biz_svc,
            "BizSvc",
            Some(6),
            Some(35),
            &helpers::child_path(path, "BizSvc"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.biz_svc,
            "BizSvc",
            "[a-z0-9]{1,10}\\.([a-z0-9]{1,10}\\.)+\\d\\d",
            &helpers::child_path(path, "BizSvc"),
            config,
            collector,
        );
        if let Some(ref val) = self.mkt_prctc
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "MktPrctc"), config, collector);
        }
        helpers::validate_pattern(
            &self.cre_dt,
            "CreDt",
            ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
            &helpers::child_path(path, "CreDt"),
            config,
            collector,
        );
        if let Some(ref val) = self.cpy_dplct
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "CpyDplct"), config, collector);
        }
        if let Some(ref val) = self.prty
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Prty"), config, collector);
        }
        if let Some(ref vec) = self.rltd
            && config.validate_optional_fields
        {
            for item in vec {
                item.validate(&helpers::child_path(path, "Rltd"), config, collector);
            }
        }
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

// FinancialInstitutionIdentification183: Legal entity identifier of the financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification183 {
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
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
    }
}

// ImplementationSpecification1: Identifier which unambiguously identifies, within the implementation specification registry, the implementation specification to which the ISO 20022 message is compliant. This can be done via a URN. It can also contain a version number or date.
// For instance, "2018-01-01 – Version 2" or "urn:uuid:6e8bc430-9c3a-11d9-9669-0800200c9a66".
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImplementationSpecification1 {
    #[serde(rename = "Regy")]
    pub regy: String,
    #[serde(rename = "Id")]
    pub id: String,
}

impl Validate for ImplementationSpecification1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.regy,
            "Regy",
            Some(1),
            Some(350),
            &helpers::child_path(path, "Regy"),
            config,
            collector,
        );
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

// Party44Choice1: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party44Choice1 {
    #[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
    pub fi_id: Option<BranchAndFinancialInstitutionIdentification62>,
}

impl Validate for Party44Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.fi_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FIId"), config, collector);
        }
    }
}

// Party44Choice2: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party44Choice2 {
    #[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
    pub fi_id: Option<BranchAndFinancialInstitutionIdentification63>,
}

impl Validate for Party44Choice2 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.fi_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FIId"), config, collector);
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
