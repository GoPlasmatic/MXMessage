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

use crate::common::ValidationError;

use crate::camt_027_001_07::ClaimNonReceiptV07;
use crate::camt_028_001_09::AdditionalPaymentInformationV09;
use crate::camt_029_001_09::ResolutionOfInvestigationV09;
use crate::camt_052_001_08::BankToCustomerAccountReportV08;
use crate::camt_053_001_08::BankToCustomerStatementV08;
use crate::camt_056_001_08::FIToFIPaymentCancellationRequestV08;
use crate::camt_057_001_06::NotificationToReceiveV06;
use crate::camt_998_001_03::CashManagementProprietaryMessageV03;
use crate::pacs_008_001_08::FIToFICustomerCreditTransferV08;
use crate::pacs_009_001_08::FinancialInstitutionCreditTransferV08;
use serde::{Deserialize, Serialize};

/// Document represents the root container for all supported CBPR+ ISO20022 message types
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Document {
    /// pacs.008.001.08 - FI to FI Customer Credit Transfer
    #[serde(rename = "FIToFICstmrCdtTrf")]
    FIToFICustomerCreditTransferV08(Box<FIToFICustomerCreditTransferV08>),

    /// pacs.009.001.08 - Financial Institution Credit Transfer  
    #[serde(rename = "FinInstnCdtTrf")]
    FinancialInstitutionCreditTransferV08(Box<FinancialInstitutionCreditTransferV08>),

    /// camt.027.001.07 - Claim Non Receipt
    #[serde(rename = "ClmNonRct")]
    ClaimNonReceiptV07(Box<ClaimNonReceiptV07>),

    /// camt.028.001.09 - Additional Payment Information
    #[serde(rename = "AddtlPmtInf")]
    AdditionalPaymentInformationV09(Box<AdditionalPaymentInformationV09>),

    /// camt.029.001.09 - Resolution of Investigation
    #[serde(rename = "RsltnOfInvstgtn")]
    ResolutionOfInvestigationV09(Box<ResolutionOfInvestigationV09>),

    /// camt.052.001.08 - Bank to Customer Account Report
    #[serde(rename = "BkToCstmrAcctRpt")]
    BankToCustomerAccountReportV08(Box<BankToCustomerAccountReportV08>),

    /// camt.053.001.08 - Bank to Customer Statement
    #[serde(rename = "BkToCstmrStmt")]
    BankToCustomerStatementV08(Box<BankToCustomerStatementV08>),

    /// camt.056.001.08 - FI to FI Payment Cancellation Request
    #[serde(rename = "FIToFIPmtCxlReq")]
    FIToFIPaymentCancellationRequestV08(Box<FIToFIPaymentCancellationRequestV08>),

    /// camt.057.001.06 - Notification to Receive
    #[serde(rename = "NtfctnToRcv")]
    NotificationToReceiveV06(Box<NotificationToReceiveV06>),

    /// camt.998.001.03 - Cash Management Proprietary Message
    #[serde(rename = "CshMgmtPrtryMsg")]
    CashManagementProprietaryMessageV03(Box<CashManagementProprietaryMessageV03>),

    /// Unknown or unsupported document type
    #[default]
    UNKNOWN,
}

impl Document {
    /// Validates the document according to ISO20022 and CBPR+ specifications
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Document::FIToFICustomerCreditTransferV08(value) => value.validate(),
            Document::FinancialInstitutionCreditTransferV08(value) => value.validate(),
            Document::ClaimNonReceiptV07(value) => value.validate(),
            Document::AdditionalPaymentInformationV09(value) => value.validate(),
            Document::ResolutionOfInvestigationV09(value) => value.validate(),
            Document::BankToCustomerAccountReportV08(value) => value.validate(),
            Document::BankToCustomerStatementV08(value) => value.validate(),
            Document::FIToFIPaymentCancellationRequestV08(value) => value.validate(),
            Document::NotificationToReceiveV06(value) => value.validate(),
            Document::CashManagementProprietaryMessageV03(value) => value.validate(),
            Document::UNKNOWN => {
                // Return an error for the UNKNOWN case
                Err(ValidationError::new(
                    9999,
                    "Unknown document type".to_string(),
                ))
            }
        }
    }

    /// Returns the message type identifier for the document
    pub fn message_type(&self) -> &'static str {
        match self {
            Document::FIToFICustomerCreditTransferV08(_) => "pacs.008.001.08",
            Document::FinancialInstitutionCreditTransferV08(_) => "pacs.009.001.08",
            Document::ClaimNonReceiptV07(_) => "camt.027.001.07",
            Document::AdditionalPaymentInformationV09(_) => "camt.028.001.09",
            Document::ResolutionOfInvestigationV09(_) => "camt.029.001.09",
            Document::BankToCustomerAccountReportV08(_) => "camt.052.001.08",
            Document::BankToCustomerStatementV08(_) => "camt.053.001.08",
            Document::FIToFIPaymentCancellationRequestV08(_) => "camt.056.001.08",
            Document::NotificationToReceiveV06(_) => "camt.057.001.06",
            Document::CashManagementProprietaryMessageV03(_) => "camt.998.001.03",
            Document::UNKNOWN => "unknown",
        }
    }

    /// Returns whether the document is CBPR+ compliant
    pub fn is_cbpr_plus_compliant(&self) -> bool {
        match self {
            Document::UNKNOWN => false,
            _ => true, // All implemented message types are CBPR+ compliant
        }
    }
}
