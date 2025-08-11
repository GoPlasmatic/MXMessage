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

// CBPR_NotificationToReceiveCancellationReason1Code: The payment is cancelled since the entry is no longer expected.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CBPRNotificationToReceiveCancellationReason1Code {
    #[default]
    #[serde(rename = "DUPL")]
    CodeDUPL,
    #[serde(rename = "NARR")]
    CodeNARR,
    #[serde(rename = "NOLE")]
    CodeNOLE,
}

impl Validate for CBPRNotificationToReceiveCancellationReason1Code {
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
            Some(16),
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

// NotificationCancellationReason1Choice1: Reason for the cancellation request, in a coded form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationCancellationReason1Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CBPRNotificationToReceiveCancellationReason1Code>,
}

impl Validate for NotificationCancellationReason1Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.cd
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Cd"), config, collector);
        }
    }
}

// NotificationCancellationReason11: Further details on the cancellation request reason.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationCancellationReason11 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification1351>,
    #[serde(rename = "Rsn")]
    pub rsn: NotificationCancellationReason1Choice1,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl Validate for NotificationCancellationReason11 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.orgtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Orgtr"), config, collector);
        }
        self.rsn
            .validate(&helpers::child_path(path, "Rsn"), config, collector);
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

// NotificationToReceiveCancellationAdviceV08: Reason for the cancellation of the notification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationToReceiveCancellationAdviceV08 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader771,
    #[serde(rename = "OrgnlNtfctn")]
    pub orgnl_ntfctn: OriginalNotification141,
    #[serde(rename = "CxlRsn")]
    pub cxl_rsn: NotificationCancellationReason11,
}

impl Validate for NotificationToReceiveCancellationAdviceV08 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.orgnl_ntfctn
            .validate(&helpers::child_path(path, "OrgnlNtfctn"), config, collector);
        self.cxl_rsn
            .validate(&helpers::child_path(path, "CxlRsn"), config, collector);
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

// OriginalItem71: Provides further information in order to identify a previous payment notification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalItem71 {
    #[serde(rename = "OrgnlItmId")]
    pub orgnl_itm_id: String,
    #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "Amt")]
    pub amt: CBPRAmount,
    #[serde(rename = "XpctdValDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_val_dt: Option<String>,
    #[serde(rename = "OrgnlItmRef", skip_serializing_if = "Option::is_none")]
    pub orgnl_itm_ref: Option<OriginalItemReference61>,
}

impl Validate for OriginalItem71 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.orgnl_itm_id,
            "OrgnlItmId",
            Some(1),
            Some(16),
            &helpers::child_path(path, "OrgnlItmId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.orgnl_itm_id,
            "OrgnlItmId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "OrgnlItmId"),
            config,
            collector,
        );
        if let Some(ref val) = self.orgnl_end_to_end_id {
            helpers::validate_length(
                val,
                "OrgnlEndToEndId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "OrgnlEndToEndId"),
                config,
                collector,
            );
        }
        if let Some(ref val) = self.orgnl_end_to_end_id {
            helpers::validate_pattern(
                val,
                "OrgnlEndToEndId",
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
                &helpers::child_path(path, "OrgnlEndToEndId"),
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
        self.amt
            .validate(&helpers::child_path(path, "Amt"), config, collector);
        if let Some(ref val) = self.orgnl_itm_ref
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgnlItmRef"), config, collector);
        }
    }
}

// OriginalItemReference61: Financial institution servicing an account for the debtor.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalItemReference61 {
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice1>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Validate for OriginalItemReference61 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
        }
        if let Some(ref val) = self.dbtr_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
        }
    }
}

// OriginalNotification141: Identifies the original notification item, to which the cancellation advice refers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalNotification141 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<String>,
    #[serde(rename = "OrgnlNtfctnId")]
    pub orgnl_ntfctn_id: String,
    #[serde(rename = "OrgnlNtfctnRef")]
    pub orgnl_ntfctn_ref: OriginalNotificationReference121,
}

impl Validate for OriginalNotification141 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.orgnl_msg_id,
            "OrgnlMsgId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "OrgnlMsgId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.orgnl_msg_id,
            "OrgnlMsgId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "OrgnlMsgId"),
            config,
            collector,
        );
        if let Some(ref val) = self.orgnl_cre_dt_tm {
            helpers::validate_pattern(
                val,
                "OrgnlCreDtTm",
                ".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]",
                &helpers::child_path(path, "OrgnlCreDtTm"),
                config,
                collector,
            );
        }
        helpers::validate_length(
            &self.orgnl_ntfctn_id,
            "OrgnlNtfctnId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "OrgnlNtfctnId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.orgnl_ntfctn_id,
            "OrgnlNtfctnId",
            "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+",
            &helpers::child_path(path, "OrgnlNtfctnId"),
            config,
            collector,
        );
        self.orgnl_ntfctn_ref.validate(
            &helpers::child_path(path, "OrgnlNtfctnRef"),
            config,
            collector,
        );
    }
}

// OriginalNotificationReference121: Provides details of the expected amount on the account serviced by the account servicer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalNotificationReference121 {
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice1>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification61>,
    #[serde(rename = "OrgnlItm")]
    pub orgnl_itm: Vec<OriginalItem71>,
}

impl Validate for OriginalNotificationReference121 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.dbtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Dbtr"), config, collector);
        }
        if let Some(ref val) = self.dbtr_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "DbtrAgt"), config, collector);
        }
        for item in &self.orgnl_itm {
            item.validate(&helpers::child_path(path, "OrgnlItm"), config, collector);
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
