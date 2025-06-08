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

use crate::error::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

// BranchAndFinancialInstitutionIdentification61: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification61 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification181,
}

impl BranchAndFinancialInstitutionIdentification61 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.fin_instn_id.validate()?;
        Ok(())
    }
}

// ClearingSystemIdentification2Choice1: Identification of a clearing system, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification2Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
}

impl ClearingSystemIdentification2Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 5 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 5".to_string(),
                ));
            }
        }
        Ok(())
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

impl ClearingSystemMemberIdentification21 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.clr_sys_id.validate()?;
        if self.mmb_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "mmb_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.mmb_id.chars().count() > 28 {
            return Err(ValidationError::new(
                1002,
                "mmb_id exceeds the maximum length of 28".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.mmb_id) {
            return Err(ValidationError::new(
                1005,
                "mmb_id does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// DateAndDateTime2Choice1: Specified date and time.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndDateTime2Choice1 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
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

impl DateAndPlaceOfBirth11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.prvc_of_birth {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prvc_of_birth is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prvc_of_birth exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prvc_of_birth does not match the required pattern".to_string(),
                ));
            }
        }
        if self.city_of_birth.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "city_of_birth is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.city_of_birth.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "city_of_birth exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern =
            Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+")
                .unwrap();
        if !pattern.is_match(&self.city_of_birth) {
            return Err(ValidationError::new(
                1005,
                "city_of_birth does not match the required pattern".to_string(),
            ));
        }
        let pattern = Regex::new("[A-Z]{2,2}").unwrap();
        if !pattern.is_match(&self.ctry_of_birth) {
            return Err(ValidationError::new(
                1005,
                "ctry_of_birth does not match the required pattern".to_string(),
            ));
        }
        Ok(())
    }
}

// FIToFIPaymentStatusReportV10: Information concerning the original transactions, to which the status report message refers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FIToFIPaymentStatusReportV10 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader911,
    #[serde(rename = "TxInfAndSts")]
    pub tx_inf_and_sts: PaymentTransaction1101,
}

impl FIToFIPaymentStatusReportV10 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        self.tx_inf_and_sts.validate()?;
        Ok(())
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

impl FinancialInstitutionIdentification181 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let pattern =
            Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
        if !pattern.is_match(&self.bicfi) {
            return Err(ValidationError::new(
                1005,
                "bicfi does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.clr_sys_mmb_id {
            val.validate()?
        }
        if let Some(ref val) = self.lei {
            let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "lei does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
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

impl GenericOrganisationIdentification11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "issr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
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

impl GenericPersonIdentification11 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.id) {
            return Err(ValidationError::new(
                1005,
                "id does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.schme_nm {
            val.validate()?
        }
        if let Some(ref val) = self.issr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "issr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "issr exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "issr does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// GroupHeader911: Date and time at which the message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader911 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
}

impl GroupHeader911 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.msg_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "msg_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.msg_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "msg_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.msg_id) {
            return Err(ValidationError::new(
                1005,
                "msg_id does not match the required pattern".to_string(),
            ));
        }
        let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
        if !pattern.is_match(&self.cre_dt_tm) {
            return Err(ValidationError::new(
                1005,
                "cre_dt_tm does not match the required pattern".to_string(),
            ));
        }
        Ok(())
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

impl OrganisationIdentification291 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.any_bic {
            let pattern =
                Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "any_bic does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.lei {
            let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "lei does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.othr {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
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

impl OrganisationIdentificationSchemeName1Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prtry {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prtry is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prtry exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// OriginalGroupInformation291: Original date and time at which the message was created.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OriginalGroupInformation291 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: String,
    #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<String>,
}

impl OriginalGroupInformation291 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.orgnl_msg_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "orgnl_msg_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.orgnl_msg_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "orgnl_msg_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.orgnl_msg_id) {
            return Err(ValidationError::new(
                1005,
                "orgnl_msg_id does not match the required pattern".to_string(),
            ));
        }
        if self.orgnl_msg_nm_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.orgnl_msg_nm_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "orgnl_msg_nm_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.orgnl_msg_nm_id) {
            return Err(ValidationError::new(
                1005,
                "orgnl_msg_nm_id does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.orgnl_cre_dt_tm {
            let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_cre_dt_tm does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
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

impl Party38Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.org_id {
            val.validate()?
        }
        if let Some(ref val) = self.prvt_id {
            val.validate()?
        }
        Ok(())
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

impl PartyIdentification1351 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 140 {
                return Err(ValidationError::new(
                    1002,
                    "nm exceeds the maximum length of 140".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate()?
        }
        if let Some(ref val) = self.id {
            val.validate()?
        }
        if let Some(ref val) = self.ctry_of_res {
            let pattern = Regex::new("[A-Z]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry_of_res does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// PaymentTransaction1101: Agent that is instructed by the previous party in the chain to carry out the (set of) instruction(s).
//
// Usage: The instructed agent is the party receiving the status message and not the party that received the original instruction that is being reported on.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTransaction1101 {
    #[serde(rename = "OrgnlGrpInf")]
    pub orgnl_grp_inf: OriginalGroupInformation291,
    #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<String>,
    #[serde(rename = "OrgnlEndToEndId")]
    pub orgnl_end_to_end_id: String,
    #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id: Option<String>,
    #[serde(rename = "OrgnlUETR")]
    pub orgnl_uetr: String,
    #[serde(rename = "TxSts")]
    pub tx_sts: String,
    #[serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none")]
    pub sts_rsn_inf: Option<StatusReasonInformation121>,
    #[serde(rename = "FctvIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
    pub fctv_intr_bk_sttlm_dt: Option<DateAndDateTime2Choice1>,
    #[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<String>,
    #[serde(rename = "InstgAgt")]
    pub instg_agt: BranchAndFinancialInstitutionIdentification61,
    #[serde(rename = "InstdAgt")]
    pub instd_agt: BranchAndFinancialInstitutionIdentification61,
}

impl PaymentTransaction1101 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.orgnl_grp_inf.validate()?;
        if let Some(ref val) = self.orgnl_instr_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_instr_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_instr_id exceeds the maximum length of 16".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_instr_id does not match the required pattern".to_string(),
                ));
            }
        }
        if self.orgnl_end_to_end_id.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.orgnl_end_to_end_id.chars().count() > 35 {
            return Err(ValidationError::new(
                1002,
                "orgnl_end_to_end_id exceeds the maximum length of 35".to_string(),
            ));
        }
        let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
        if !pattern.is_match(&self.orgnl_end_to_end_id) {
            return Err(ValidationError::new(
                1005,
                "orgnl_end_to_end_id does not match the required pattern".to_string(),
            ));
        }
        if let Some(ref val) = self.orgnl_tx_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "orgnl_tx_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "orgnl_tx_id exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "orgnl_tx_id does not match the required pattern".to_string(),
                ));
            }
        }
        let pattern =
            Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                .unwrap();
        if !pattern.is_match(&self.orgnl_uetr) {
            return Err(ValidationError::new(
                1005,
                "orgnl_uetr does not match the required pattern".to_string(),
            ));
        }
        if self.tx_sts.chars().count() < 1 {
            return Err(ValidationError::new(
                1001,
                "tx_sts is shorter than the minimum length of 1".to_string(),
            ));
        }
        if self.tx_sts.chars().count() > 4 {
            return Err(ValidationError::new(
                1002,
                "tx_sts exceeds the maximum length of 4".to_string(),
            ));
        }
        if let Some(ref val) = self.sts_rsn_inf {
            val.validate()?
        }
        if let Some(ref val) = self.fctv_intr_bk_sttlm_dt {
            val.validate()?
        }
        if let Some(ref val) = self.clr_sys_ref {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "clr_sys_ref is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "clr_sys_ref exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "clr_sys_ref does not match the required pattern".to_string(),
                ));
            }
        }
        self.instg_agt.validate()?;
        self.instd_agt.validate()?;
        Ok(())
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

impl PersonIdentification131 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dt_and_plc_of_birth {
            val.validate()?
        }
        if let Some(ref vec) = self.othr {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
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

impl PersonIdentificationSchemeName1Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prtry {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prtry is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prtry exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
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

impl PostalAddress241 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.dept {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "dept is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "dept exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dept does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.sub_dept {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "sub_dept is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "sub_dept exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "sub_dept does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.strt_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "strt_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "strt_nm exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "strt_nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.bldg_nb {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "bldg_nb is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "bldg_nb exceeds the maximum length of 16".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bldg_nb does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.bldg_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "bldg_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "bldg_nm exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "bldg_nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.flr {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "flr is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "flr exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "flr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pst_bx {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "pst_bx is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "pst_bx exceeds the maximum length of 16".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pst_bx does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.room {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "room is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 70 {
                return Err(ValidationError::new(
                    1002,
                    "room exceeds the maximum length of 70".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "room does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.pst_cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "pst_cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 16 {
                return Err(ValidationError::new(
                    1002,
                    "pst_cd exceeds the maximum length of 16".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "pst_cd does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.twn_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "twn_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "twn_nm exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "twn_nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.twn_lctn_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "twn_lctn_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "twn_lctn_nm exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "twn_lctn_nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.dstrct_nm {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "dstrct_nm is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "dstrct_nm exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "dstrct_nm does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "ctry_sub_dvsn is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "ctry_sub_dvsn exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new(
                "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
            )
            .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry_sub_dvsn does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.ctry {
            let pattern = Regex::new("[A-Z]{2,2}").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "ctry does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref vec) = self.adr_line {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "adr_line is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 70 {
                    return Err(ValidationError::new(
                        1002,
                        "adr_line exceeds the maximum length of 70".to_string(),
                    ));
                }
                let pattern = Regex::new(
                    "[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+",
                )
                .unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "adr_line does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// StatusReason6Choice1: Reason for the status, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReason6Choice1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<String>,
}

impl StatusReason6Choice1 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.cd {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "cd is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 4 {
                return Err(ValidationError::new(
                    1002,
                    "cd exceeds the maximum length of 4".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.prtry {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "prtry is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "prtry exceeds the maximum length of 35".to_string(),
                ));
            }
            let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "prtry does not match the required pattern".to_string(),
                ));
            }
        }
        Ok(())
    }
}

// StatusReasonInformation121: Further details on the status reason.
//
// Usage: Additional information can be used for several purposes such as the reporting of repaired information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusReasonInformation121 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification1351>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<StatusReason6Choice1>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Vec<String>>,
}

impl StatusReasonInformation121 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref val) = self.orgtr {
            val.validate()?
        }
        if let Some(ref val) = self.rsn {
            val.validate()?
        }
        if let Some(ref vec) = self.addtl_inf {
            for item in vec {
                if item.chars().count() < 1 {
                    return Err(ValidationError::new(
                        1001,
                        "addtl_inf is shorter than the minimum length of 1".to_string(),
                    ));
                }
                if item.chars().count() > 105 {
                    return Err(ValidationError::new(
                        1002,
                        "addtl_inf exceeds the maximum length of 105".to_string(),
                    ));
                }
                let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
                if !pattern.is_match(item) {
                    return Err(ValidationError::new(
                        1005,
                        "addtl_inf does not match the required pattern".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}
