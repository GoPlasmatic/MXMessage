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

use crate::error::ValidationError;
use crate::header::*;
use serde::{Deserialize, Serialize};

/// AppHeader represents the application header container for all supported CBPR+ ISO20022 message types
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum AppHeader {
    // Administrative Messages Headers
    /// admi.024.001.01 - Notification of Correspondence Header
    #[serde(rename = "BizAppHdr_ADMI_024_001_01")]
    BusinessApplicationHeaderV02Admi024_001_01(
        Box<bah_admi_024_001_01::BusinessApplicationHeaderV02>,
    ),

    // Cash Management Messages Headers (camt)
    /// camt.025.001.08 - Receipt Header
    #[serde(rename = "BizAppHdr_CAMT_025_001_08")]
    BusinessApplicationHeaderV02Camt025_001_08(
        Box<bah_camt_025_001_08::BusinessApplicationHeaderV02>,
    ),

    /// camt.029.001 - Resolution of Investigation Header
    #[serde(rename = "BizAppHdr_CAMT_029_001")]
    BusinessApplicationHeaderV02Camt029_001(Box<bah_camt_029_001::BusinessApplicationHeaderV02>),

    /// camt.052.001.08 - Bank to Customer Account Report Header
    #[serde(rename = "BizAppHdr_CAMT_052_001_08")]
    BusinessApplicationHeaderV02Camt052_001_08(
        Box<bah_camt_052_001_08::BusinessApplicationHeaderV02>,
    ),

    /// camt.053.001.08 - Bank to Customer Statement Header
    #[serde(rename = "BizAppHdr_CAMT_053_001_08")]
    BusinessApplicationHeaderV02Camt053_001_08(
        Box<bah_camt_053_001_08::BusinessApplicationHeaderV02>,
    ),

    /// camt.054.001 - Bank to Customer Debit Credit Notification Header
    #[serde(rename = "BizAppHdr_CAMT_054_001")]
    BusinessApplicationHeaderV02Camt054_001(Box<bah_camt_054_001::BusinessApplicationHeaderV02>),

    /// camt.055.001.08 - Customer Payment Cancellation Request Header
    #[serde(rename = "BizAppHdr_CAMT_055_001_08")]
    BusinessApplicationHeaderV02Camt055_001_08(
        Box<bah_camt_055_001_08::BusinessApplicationHeaderV02>,
    ),

    /// camt.056.001.08 - FI to FI Payment Cancellation Request Header
    #[serde(rename = "BizAppHdr_CAMT_056_001_08")]
    BusinessApplicationHeaderV02Camt056_001_08(
        Box<bah_camt_056_001_08::BusinessApplicationHeaderV02>,
    ),

    /// camt.057.001.06 - Notification to Receive Header
    #[serde(rename = "BizAppHdr_CAMT_057_001_06")]
    BusinessApplicationHeaderV02Camt057_001_06(
        Box<bah_camt_057_001_06::BusinessApplicationHeaderV02>,
    ),

    /// camt.058.001.08 - Notification to Receive Cancellation Advice Header
    #[serde(rename = "BizAppHdr_CAMT_058_001_08")]
    BusinessApplicationHeaderV02Camt058_001_08(
        Box<bah_camt_058_001_08::BusinessApplicationHeaderV02>,
    ),

    /// camt.060.001.05 - Account Reporting Request Header
    #[serde(rename = "BizAppHdr_CAMT_060_001_05")]
    BusinessApplicationHeaderV02Camt060_001_05(
        Box<bah_camt_060_001_05::BusinessApplicationHeaderV02>,
    ),

    /// camt.105.001.02 - Charges Payment Notification Header
    #[serde(rename = "BizAppHdr_CAMT_105_001_02")]
    BusinessApplicationHeaderV02Camt105_001_02(
        Box<bah_camt_105_001_02::BusinessApplicationHeaderV02>,
    ),

    /// camt.105.001.02.mc - Charges Payment Notification Header (Multi-Currency)
    #[serde(rename = "BizAppHdr_CAMT_105_001_02_MC")]
    BusinessApplicationHeaderV02Camt105_001_02mc(
        Box<bah_camt_105_001_02_mc::BusinessApplicationHeaderV02>,
    ),

    /// camt.106.001.02 - Charges Payment Request Header
    #[serde(rename = "BizAppHdr_CAMT_106_001_02")]
    BusinessApplicationHeaderV02Camt106_001_02(
        Box<bah_camt_106_001_02::BusinessApplicationHeaderV02>,
    ),

    /// camt.106.001.02.mc - Charges Payment Request Header (Multi-Currency)
    #[serde(rename = "BizAppHdr_CAMT_106_001_02_MC")]
    BusinessApplicationHeaderV02Camt106_001_02mc(
        Box<bah_camt_106_001_02_mc::BusinessApplicationHeaderV02>,
    ),

    /// camt.107.001.01 - Cheque Presentment Notification Header
    #[serde(rename = "BizAppHdr_CAMT_107_001_01")]
    BusinessApplicationHeaderV02Camt107_001_01(
        Box<bah_camt_107_001_01::BusinessApplicationHeaderV02>,
    ),

    /// camt.108.001.01 - Cheque Cancellation or Stop Request Header
    #[serde(rename = "BizAppHdr_CAMT_108_001_01")]
    BusinessApplicationHeaderV02Camt108_001_01(
        Box<bah_camt_108_001_01::BusinessApplicationHeaderV02>,
    ),

    /// camt.109.001.01 - Cheque Cancellation or Stop Report Header
    #[serde(rename = "BizAppHdr_CAMT_109_001_01")]
    BusinessApplicationHeaderV02Camt109_001_01(
        Box<bah_camt_109_001_01::BusinessApplicationHeaderV02>,
    ),

    // Payments Clearing and Settlement Messages Headers (pacs)
    /// pacs.002.001.10 - FI to FI Payment Status Report Header
    #[serde(rename = "BizAppHdr_PACS_002_001_10")]
    BusinessApplicationHeaderV02Pacs002_001_10(
        Box<bah_pacs_002_001_10::BusinessApplicationHeaderV02>,
    ),

    /// pacs.003.001.08 - FI to FI Customer Direct Debit Header
    #[serde(rename = "BizAppHdr_PACS_003_001_08")]
    BusinessApplicationHeaderV02Pacs003_001_08(
        Box<bah_pacs_003_001_08::BusinessApplicationHeaderV02>,
    ),

    /// pacs.004.001.09 - Payment Return Header
    #[serde(rename = "BizAppHdr_PACS_004_001_09")]
    BusinessApplicationHeaderV02Pacs004_001_09(
        Box<bah_pacs_004_001_09::BusinessApplicationHeaderV02>,
    ),

    /// pacs.008.001.08 - FI to FI Customer Credit Transfer Header
    #[serde(rename = "BizAppHdr_PACS_008_001_08")]
    BusinessApplicationHeaderV02Pacs008_001_08(
        Box<bah_pacs_008_001_08::BusinessApplicationHeaderV02>,
    ),

    /// pacs.008.001.08.stp - FI to FI Customer Credit Transfer Header (STP)
    #[serde(rename = "BizAppHdr_PACS_008_001_08_STP")]
    BusinessApplicationHeaderV02Pacs008_001_08stp(
        Box<bah_pacs_008_001_08_stp::BusinessApplicationHeaderV02>,
    ),

    /// pacs.009.001.08 - Financial Institution Credit Transfer Header
    #[serde(rename = "BizAppHdr_PACS_009_001_08")]
    BusinessApplicationHeaderV02Pacs009_001_08(
        Box<bah_pacs_009_001_08::BusinessApplicationHeaderV02>,
    ),

    /// pacs.009.001.08.adv - Financial Institution Credit Transfer Header (Advice)
    #[serde(rename = "BizAppHdr_PACS_009_001_08_ADV")]
    BusinessApplicationHeaderV02Pacs009_001_08adv(
        Box<bah_pacs_009_001_08_adv::BusinessApplicationHeaderV02>,
    ),

    /// pacs.009.001.08.cov - Financial Institution Credit Transfer Header (Cover)
    #[serde(rename = "BizAppHdr_PACS_009_001_08_COV")]
    BusinessApplicationHeaderV02Pacs009_001_08cov(
        Box<bah_pacs_009_001_08_cov::BusinessApplicationHeaderV02>,
    ),

    // Payment Initiation Messages Headers (pain)
    /// pain.001.001.09 - Customer Credit Transfer Initiation Header
    #[serde(rename = "BizAppHdr_PAIN_001_001_09")]
    BusinessApplicationHeaderV02Pain001_001_09(
        Box<bah_pain_001_001_09::BusinessApplicationHeaderV02>,
    ),

    /// pain.002.001.10 - Customer Payment Status Report Header
    #[serde(rename = "BizAppHdr_PAIN_002_001_10")]
    BusinessApplicationHeaderV02Pain002_001_10(
        Box<bah_pain_002_001_10::BusinessApplicationHeaderV02>,
    ),

    /// pain.008.001.08 - Customer Direct Debit Initiation Header
    #[serde(rename = "BizAppHdr_PAIN_008_001_08")]
    BusinessApplicationHeaderV02Pain008_001_08(
        Box<bah_pain_008_001_08::BusinessApplicationHeaderV02>,
    ),

    /// Unknown or unsupported application header type
    #[default]
    UNKNOWN,
}

impl AppHeader {
    /// Validates the application header according to ISO20022 and CBPR+ specifications
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            // Administrative Messages Headers
            AppHeader::BusinessApplicationHeaderV02Admi024_001_01(value) => value.validate(),

            // Cash Management Messages Headers
            AppHeader::BusinessApplicationHeaderV02Camt025_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt029_001(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt052_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt053_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt054_001(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt055_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt056_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt057_001_06(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt058_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt060_001_05(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt105_001_02(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt105_001_02mc(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt106_001_02(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt106_001_02mc(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt107_001_01(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt108_001_01(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Camt109_001_01(value) => value.validate(),

            // Payments Clearing and Settlement Messages Headers
            AppHeader::BusinessApplicationHeaderV02Pacs002_001_10(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs003_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs004_001_09(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs008_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs008_001_08stp(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs009_001_08(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs009_001_08adv(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pacs009_001_08cov(value) => value.validate(),

            // Payment Initiation Messages Headers
            AppHeader::BusinessApplicationHeaderV02Pain001_001_09(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pain002_001_10(value) => value.validate(),
            AppHeader::BusinessApplicationHeaderV02Pain008_001_08(value) => value.validate(),

            AppHeader::UNKNOWN => Err(ValidationError::new(
                9999,
                "Unknown application header type".to_string(),
            )),
        }
    }

    /// Returns the message type identifier for the application header
    pub fn message_type(&self) -> &'static str {
        match self {
            // Administrative Messages Headers
            AppHeader::BusinessApplicationHeaderV02Admi024_001_01(_) => "admi.024.001.01",

            // Cash Management Messages Headers
            AppHeader::BusinessApplicationHeaderV02Camt025_001_08(_) => "camt.025.001.08",
            AppHeader::BusinessApplicationHeaderV02Camt029_001(_) => "camt.029.001",
            AppHeader::BusinessApplicationHeaderV02Camt052_001_08(_) => "camt.052.001.08",
            AppHeader::BusinessApplicationHeaderV02Camt053_001_08(_) => "camt.053.001.08",
            AppHeader::BusinessApplicationHeaderV02Camt054_001(_) => "camt.054.001",
            AppHeader::BusinessApplicationHeaderV02Camt055_001_08(_) => "camt.055.001.08",
            AppHeader::BusinessApplicationHeaderV02Camt056_001_08(_) => "camt.056.001.08",
            AppHeader::BusinessApplicationHeaderV02Camt057_001_06(_) => "camt.057.001.06",
            AppHeader::BusinessApplicationHeaderV02Camt058_001_08(_) => "camt.058.001.08",
            AppHeader::BusinessApplicationHeaderV02Camt060_001_05(_) => "camt.060.001.05",
            AppHeader::BusinessApplicationHeaderV02Camt105_001_02(_) => "camt.105.001.02",
            AppHeader::BusinessApplicationHeaderV02Camt105_001_02mc(_) => "camt.105.001.02.mc",
            AppHeader::BusinessApplicationHeaderV02Camt106_001_02(_) => "camt.106.001.02",
            AppHeader::BusinessApplicationHeaderV02Camt106_001_02mc(_) => "camt.106.001.02.mc",
            AppHeader::BusinessApplicationHeaderV02Camt107_001_01(_) => "camt.107.001.01",
            AppHeader::BusinessApplicationHeaderV02Camt108_001_01(_) => "camt.108.001.01",
            AppHeader::BusinessApplicationHeaderV02Camt109_001_01(_) => "camt.109.001.01",

            // Payments Clearing and Settlement Messages Headers
            AppHeader::BusinessApplicationHeaderV02Pacs002_001_10(_) => "pacs.002.001.10",
            AppHeader::BusinessApplicationHeaderV02Pacs003_001_08(_) => "pacs.003.001.08",
            AppHeader::BusinessApplicationHeaderV02Pacs004_001_09(_) => "pacs.004.001.09",
            AppHeader::BusinessApplicationHeaderV02Pacs008_001_08(_) => "pacs.008.001.08",
            AppHeader::BusinessApplicationHeaderV02Pacs008_001_08stp(_) => "pacs.008.001.08.stp",
            AppHeader::BusinessApplicationHeaderV02Pacs009_001_08(_) => "pacs.009.001.08",
            AppHeader::BusinessApplicationHeaderV02Pacs009_001_08adv(_) => "pacs.009.001.08.adv",
            AppHeader::BusinessApplicationHeaderV02Pacs009_001_08cov(_) => "pacs.009.001.08.cov",

            // Payment Initiation Messages Headers
            AppHeader::BusinessApplicationHeaderV02Pain001_001_09(_) => "pain.001.001.09",
            AppHeader::BusinessApplicationHeaderV02Pain002_001_10(_) => "pain.002.001.10",
            AppHeader::BusinessApplicationHeaderV02Pain008_001_08(_) => "pain.008.001.08",

            AppHeader::UNKNOWN => "unknown",
        }
    }

    /// Returns whether the application header is CBPR+ compliant
    pub fn is_cbpr_plus_compliant(&self) -> bool {
        match self {
            AppHeader::UNKNOWN => false,
            _ => true, // All implemented application header types are CBPR+ compliant
        }
    }
}
