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

use crate::common::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

// AccountNotification16 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountNotification16 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<CashAccount38>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<Party40Choice>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none")]
    pub rltd_acct: Option<CashAccount38>,
    #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XpctdValDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_val_dt: Option<String>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Itm")]
    pub itm: Vec<NotificationItem7>,
}

impl AccountNotification16 {
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
        if let Some(ref val) = self.acct {
            val.validate()?
        }
        if let Some(ref val) = self.acct_ownr {
            val.validate()?
        }
        if let Some(ref val) = self.acct_svcr {
            val.validate()?
        }
        if let Some(ref val) = self.rltd_acct {
            val.validate()?
        }
        if let Some(ref val) = self.ttl_amt {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt {
            val.validate()?
        }
        for item in &self.itm {
            item.validate()?
        }
        Ok(())
    }
}

// GroupHeader77 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader77 {
    #[serde(rename = "MsgId")]
    pub msg_id: String,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: String,
    #[serde(rename = "MsgSndr", skip_serializing_if = "Option::is_none")]
    pub msg_sndr: Option<Party40Choice>,
}

impl GroupHeader77 {
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
        if let Some(ref val) = self.msg_sndr {
            val.validate()?
        }
        Ok(())
    }
}

// NotificationItem7 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationItem7 {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
    pub uetr: Option<String>,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<CashAccount38>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<Party40Choice>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none")]
    pub rltd_acct: Option<CashAccount38>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "XpctdValDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_val_dt: Option<String>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<Party40Choice>,
    #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt", skip_serializing_if = "Option::is_none")]
    pub intrmy_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Purpose2Choice>,
    #[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
    pub rltd_rmt_inf: Option<RemittanceLocation7>,
    #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation16>,
}

impl NotificationItem7 {
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
        if let Some(ref val) = self.end_to_end_id {
            if val.chars().count() < 1 {
                return Err(ValidationError::new(
                    1001,
                    "end_to_end_id is shorter than the minimum length of 1".to_string(),
                ));
            }
            if val.chars().count() > 35 {
                return Err(ValidationError::new(
                    1002,
                    "end_to_end_id exceeds the maximum length of 35".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.uetr {
            let pattern =
                Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")
                    .unwrap();
            if !pattern.is_match(val) {
                return Err(ValidationError::new(
                    1005,
                    "uetr does not match the required pattern".to_string(),
                ));
            }
        }
        if let Some(ref val) = self.acct {
            val.validate()?
        }
        if let Some(ref val) = self.acct_ownr {
            val.validate()?
        }
        if let Some(ref val) = self.acct_svcr {
            val.validate()?
        }
        if let Some(ref val) = self.rltd_acct {
            val.validate()?
        }
        self.amt.validate()?;
        if let Some(ref val) = self.dbtr {
            val.validate()?
        }
        if let Some(ref val) = self.dbtr_agt {
            val.validate()?
        }
        if let Some(ref val) = self.intrmy_agt {
            val.validate()?
        }
        if let Some(ref val) = self.purp {
            val.validate()?
        }
        if let Some(ref val) = self.rltd_rmt_inf {
            val.validate()?
        }
        if let Some(ref val) = self.rmt_inf {
            val.validate()?
        }
        Ok(())
    }
}

// NotificationToReceiveV06 ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NotificationToReceiveV06 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader77,
    #[serde(rename = "Ntfctn")]
    pub ntfctn: AccountNotification16,
    #[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
    pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl NotificationToReceiveV06 {
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.grp_hdr.validate()?;
        self.ntfctn.validate()?;
        if let Some(ref vec) = self.splmtry_data {
            for item in vec {
                item.validate()?
            }
        }
        Ok(())
    }
}
