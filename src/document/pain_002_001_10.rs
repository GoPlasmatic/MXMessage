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
            Some(35),
            &helpers::child_path(path, "MmbId"),
            config,
            collector,
        );
    }
}

// CustomerPaymentStatusReportV10: Information concerning the original payment information, to which the status report message refers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomerPaymentStatusReportV10 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader861,
    #[serde(rename = "OrgnlGrpInfAndSts")]
    pub orgnl_grp_inf_and_sts: OriginalGroupHeader171,
    #[serde(rename = "OrgnlPmtInfAndSts")]
    pub orgnl_pmt_inf_and_sts: OriginalPaymentInstruction321,
}

impl Validate for CustomerPaymentStatusReportV10 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.grp_hdr
            .validate(&helpers::child_path(path, "GrpHdr"), config, collector);
        self.orgnl_grp_inf_and_sts.validate(
            &helpers::child_path(path, "OrgnlGrpInfAndSts"),
            config,
            collector,
        );
        self.orgnl_pmt_inf_and_sts.validate(
            &helpers::child_path(path, "OrgnlPmtInfAndSts"),
            config,
            collector,
        );
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

// GroupHeader861: Financial institution that receives the instruction from the initiating party and forwards it to the next agent in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader861 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "InitgPty")]
    pub initg_pty: PartyIdentification1351,
    #[serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none")]
    pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification61>,
}

impl Validate for GroupHeader861 {
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
        self.initg_pty
            .validate(&helpers::child_path(path, "InitgPty"), config, collector);
        if let Some(ref val) = self.fwdg_agt
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "FwdgAgt"), config, collector);
        }
    }
}

// OrganisationIdentification291: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification291 {
    #[serde(rename = "AnyBIC")]
    pub any_bic: String,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl Validate for OrganisationIdentification291 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_pattern(
            &self.any_bic,
            "AnyBIC",
            "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}",
            &helpers::child_path(path, "AnyBIC"),
            config,
            collector,
        );
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
    pub othr: Option<Vec<GenericOrganisationIdentification1>>,
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

// OriginalGroupHeader171: Date and time at which the original message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalGroupHeader171 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: String,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<String>,
}

impl Validate for OriginalGroupHeader171 {
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
        helpers::validate_length(
            &self.orgnl_msg_nm_id,
            "OrgnlMsgNmId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "OrgnlMsgNmId"),
            config,
            collector,
        );
    }
}

// OriginalPaymentInstruction321: Provides information on the original transactions to which the status report message refers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalPaymentInstruction321 {
    #[serde(rename = "OrgnlPmtInfId")]
    pub orgnl_pmt_inf_id: String,
    #[serde(rename = "TxInfAndSts")]
    pub tx_inf_and_sts: PaymentTransaction1051,
}

impl Validate for OriginalPaymentInstruction321 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        helpers::validate_length(
            &self.orgnl_pmt_inf_id,
            "OrgnlPmtInfId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "OrgnlPmtInfId"),
            config,
            collector,
        );
        self.tx_inf_and_sts
            .validate(&helpers::child_path(path, "TxInfAndSts"), config, collector);
    }
}

// Party38Choice1: Unique and unambiguous way to identify an organisation.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice1 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification291>,
}

impl Validate for Party38Choice1 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.org_id
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "OrgId"), config, collector);
        }
    }
}

// Party38Choice2: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice2 {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification292>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification131>,
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

// PartyIdentification1351: Unique and unambiguous identification of a party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1351 {
    #[serde(rename = "Id")]
    pub id: Party38Choice1,
}

impl Validate for PartyIdentification1351 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        self.id
            .validate(&helpers::child_path(path, "Id"), config, collector);
    }
}

// PartyIdentification1352: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1352 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress241>,
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

// PaymentTransaction1051: Provides detailed information on the status reason.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTransaction1051 {
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<String>,
    #[serde(rename = "OrgnlEndToEndId")]
    pub orgnl_end_to_end_id: String,
    #[serde(rename = "OrgnlUETR")]
    pub orgnl_uetr: String,
    #[serde(rename = "TxSts")]
    pub tx_sts: String,
    #[serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none")]
    pub sts_rsn_inf: Option<StatusReasonInformation121>,
}

impl Validate for PaymentTransaction1051 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.orgnl_instr_id {
            helpers::validate_length(
                val,
                "OrgnlInstrId",
                Some(1),
                Some(35),
                &helpers::child_path(path, "OrgnlInstrId"),
                config,
                collector,
            );
        }
        helpers::validate_length(
            &self.orgnl_end_to_end_id,
            "OrgnlEndToEndId",
            Some(1),
            Some(35),
            &helpers::child_path(path, "OrgnlEndToEndId"),
            config,
            collector,
        );
        helpers::validate_pattern(
            &self.orgnl_uetr,
            "OrgnlUETR",
            "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}",
            &helpers::child_path(path, "OrgnlUETR"),
            config,
            collector,
        );
        helpers::validate_length(
            &self.tx_sts,
            "TxSts",
            Some(1),
            Some(4),
            &helpers::child_path(path, "TxSts"),
            config,
            collector,
        );
        if let Some(ref val) = self.sts_rsn_inf
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "StsRsnInf"), config, collector);
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

// StatusReasonInformation121: Further details on the status reason.
//
// Usage: Additional information can be used for several purposes such as the reporting of repaired information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReasonInformation121 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification1352>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<StatusReason6Choice1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl Validate for StatusReasonInformation121 {
    fn validate(&self, path: &str, config: &ParserConfig, collector: &mut ErrorCollector) {
        if let Some(ref val) = self.orgtr
            && config.validate_optional_fields
        {
            val.validate(&helpers::child_path(path, "Orgtr"), config, collector);
        }
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
    }
}
