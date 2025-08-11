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

// MessageHeader91: Date and time at which the message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageHeader91 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
}

impl Validate for MessageHeader91 {
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
    }
}

// OriginalMessageAndIssuer11: Specifies the original message name identifier to which the message refers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalMessageAndIssuer11 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "MsgNmId")]
    pub msg_nm_id: String,
}

impl Validate for OriginalMessageAndIssuer11 {
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
        helpers::validate_length(
            &self.msg_nm_id,
            "MsgNmId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "MsgNmId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.msg_nm_id,
            "MsgNmId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "MsgNmId"),
            config,
            collector,
        );
    }
}

// Receipt61: Gives the status of the request.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Receipt61 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: OriginalMessageAndIssuer11,
    #[serde(rename = "ReqHdlg")]
    pub req_hdlg: RequestHandling31,
}

impl Validate for Receipt61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.orgnl_msg_id
            .validate(&helpers::child_path(path, "OrgnlMsgId"), config, collector);
        self.req_hdlg
            .validate(&helpers::child_path(path, "ReqHdlg"), config, collector);
    }
}

// ReceiptV08: Details of the receipt.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReceiptV08 {
    #[serde(rename = "MsgHdr")]
    pub msg_hdr: MessageHeader91,
    #[serde(rename = "RctDtls")]
    pub rct_dtls: Vec<Receipt61>,
}

impl Validate for ReceiptV08 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.msg_hdr
            .validate(&helpers::child_path(path, "MsgHdr"), config, collector);
        for item in &self.rct_dtls {
            item.validate(&helpers::child_path(path, "RctDtls"), config, collector);
        }
    }
}

// RequestHandling31: Provides detailed information on the status reason.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestHandling31 {
    #[serde(rename = "Sts")]
    pub sts: RequestStatus1Choice1,
    #[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
    pub sts_rsn: Option<StatusReasonInformation141>,
}

impl Validate for RequestHandling31 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.sts
            .validate(&helpers::child_path(path, "Sts"), config, collector);
        if let Some(ref val) = self.sts_rsn
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "StsRsn"), config, collector);
        }
    }
}

// RequestStatus1Choice1: Request status, as published in an external request status code set.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestStatus1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl Validate for RequestStatus1Choice1 {
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

// StatusReason6Choice1: Reason for the status, as published in an external reason code list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReason6Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl Validate for StatusReason6Choice1 {
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

// StatusReasonInformation141: Further details on the status reason.
//
// Usage: Additional information can be used for several purposes such as the reporting of repaired information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReasonInformation141 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<StatusReason6Choice1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl Validate for StatusReasonInformation141 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
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
