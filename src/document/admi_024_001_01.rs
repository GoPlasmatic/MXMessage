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

// CorrespondenceNotification11: Provides information about the notification in narrative form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CorrespondenceNotification11 {
    #[serde(rename = "SndrNtfctnId")]
    pub sndr_ntfctn_id: String,
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: NotificationType1Choice1,
    #[serde(rename = "NtfctnNrrtv")]
    pub ntfctn_nrrtv: Vec<String>,
}

impl Validate for CorrespondenceNotification11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.sndr_ntfctn_id,
            "SndrNtfctnId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "SndrNtfctnId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.sndr_ntfctn_id,
            "SndrNtfctnId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "SndrNtfctnId"),
            config,
            collector,
        );
        self.ntfctn_tp
            .validate(&helpers::child_path(path, "NtfctnTp"), config, collector);
        for item in &self.ntfctn_nrrtv {
            helpers::validate_length(
                item,
                "NtfctnNrrtv",
                Some(1),
                Some(2000),
                &helpers::child_path(path, "NtfctnNrrtv"),
                config,
                collector,
            );
        }
        for item in &self.ntfctn_nrrtv {
            helpers::validate_pattern(
                item,
                "NtfctnNrrtv",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                &helpers::child_path(path, "NtfctnNrrtv"),
                config,
                collector,
            );
        }
    }
}

// FinancialInstitutionIdentification231: Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362 "Banking - Banking telecommunication messages - Business identifier code (BIC)".
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification231 {
    #[serde(rename = "BICFI")]
    pub bicfi: String,
}

impl Validate for FinancialInstitutionIdentification231 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.bicfi,
            "BICFI",
            "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
            &helpers::child_path(path, "BICFI"),
            config,
            collector,
        );
    }
}

// GroupHeader1291: Party that receives the notification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader1291 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "Sndr")]
    pub sndr: Party50Choice1,
    #[serde(rename = "Rcvr")]
    pub rcvr: Party50Choice1,
}

impl Validate for GroupHeader1291 {
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
        self.sndr
            .validate(&helpers::child_path(path, "Sndr"), config, collector);
        self.rcvr
            .validate(&helpers::child_path(path, "Rcvr"), config, collector);
    }
}

// NotificationOfCorrespondenceV01: Set of elements used to provide further details on the notification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationOfCorrespondenceV01 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader1291,
    #[serde(rename = "NtfctnData")]
    pub ntfctn_data: CorrespondenceNotification11,
}

impl Validate for NotificationOfCorrespondenceV01 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.ntfctn_data
            .validate(&helpers::child_path(path, "NtfctnData"), config, collector);
    }
}

// NotificationType1Choice1: Notification type, as published in an external notification type code set.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationType1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl Validate for NotificationType1Choice1 {
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

// Party50Choice1: Identification of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party50Choice1 {
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification81>,
}

impl Validate for Party50Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.agt {
            if config.validate_optional_fields {
                val.validate(&helpers::child_path(path, "Agt"), config, collector);
            }
        }
    }
}
