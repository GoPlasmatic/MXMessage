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

use crate::document::*;
use crate::error::ValidationError;
use serde::{Deserialize, Serialize};

/// Document represents the root container for all supported CBPR+ ISO20022 message types
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Document {
    // Administrative Messages
    /// admi.024.001.01 - Notification of Correspondence
    #[serde(rename = "NtfctnOfCrspdc")]
    NotificationOfCorrespondenceV01(Box<admi_024_001_01::NotificationOfCorrespondenceV01>),

    // Cash Management Messages (camt)
    /// camt.025.001.08 - Receipt
    #[serde(rename = "Rct")]
    ReceiptV08(Box<camt_025_001_08::ReceiptV08>),

    /// camt.029.001.09 - Resolution of Investigation
    #[serde(rename = "RsltnOfInvstgtn")]
    ResolutionOfInvestigationV09(Box<camt_029_001_09::ResolutionOfInvestigationV09>),

    /// camt.052.001.08 - Bank to Customer Account Report
    #[serde(rename = "BkToCstmrAcctRpt")]
    BankToCustomerAccountReportV08(Box<camt_052_001_08::BankToCustomerAccountReportV08>),

    /// camt.053.001.08 - Bank to Customer Statement
    #[serde(rename = "BkToCstmrStmt")]
    BankToCustomerStatementV08(Box<camt_053_001_08::BankToCustomerStatementV08>),

    /// camt.054.001.08 - Bank to Customer Debit Credit Notification
    #[serde(rename = "BkToCstmrDbtCdtNtfctn")]
    BankToCustomerDebitCreditNotificationV08(
        Box<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>,
    ),

    /// camt.055.001.08 - Customer Payment Cancellation Request
    #[serde(rename = "CstmrPmtCxlReq")]
    CustomerPaymentCancellationRequestV08(
        Box<camt_055_001_08::CustomerPaymentCancellationRequestV08>,
    ),

    /// camt.056.001.08 - FI to FI Payment Cancellation Request
    #[serde(rename = "FIToFIPmtCxlReq")]
    FIToFIPaymentCancellationRequestV08(Box<camt_056_001_08::FIToFIPaymentCancellationRequestV08>),

    /// camt.057.001.06 - Notification to Receive
    #[serde(rename = "NtfctnToRcv")]
    NotificationToReceiveV06(Box<camt_057_001_06::NotificationToReceiveV06>),

    /// camt.058.001.08 - Notification to Receive Cancellation Advice
    #[serde(rename = "NtfctnToRcvCxlAdvc")]
    NotificationToReceiveCancellationAdviceV08(
        Box<camt_058_001_08::NotificationToReceiveCancellationAdviceV08>,
    ),

    /// camt.060.001.05 - Account Reporting Request
    #[serde(rename = "AcctRptgReq")]
    AccountReportingRequestV05(Box<camt_060_001_05::AccountReportingRequestV05>),

    /// camt.105.001.02 - Charges Payment Notification
    #[serde(rename = "ChrgsInf")]
    ChargesPaymentNotificationV02(Box<camt_105_001_02::ChargesPaymentNotificationV02>),

    /// camt.105.001.02.mc - Charges Payment Notification (Multi-Currency)
    #[serde(rename = "ChrgsInfMC")]
    ChargesPaymentNotificationV02MC(Box<camt_105_001_02_mc::ChargesPaymentNotificationV02>),

    /// camt.106.001.02 - Charges Payment Request
    #[serde(rename = "ChrgsReq")]
    ChargesPaymentRequestV02(Box<camt_106_001_02::ChargesPaymentRequestV02>),

    /// camt.106.001.02.mc - Charges Payment Request (Multi-Currency)
    #[serde(rename = "ChrgsReqMC")]
    ChargesPaymentRequestV02MC(Box<camt_106_001_02_mc::ChargesPaymentRequestV02>),

    /// camt.107.001.01 - Cheque Presentment Notification
    #[serde(rename = "ChqPresntmntNtfctn")]
    ChequePresentmentNotificationV01(Box<camt_107_001_01::ChequePresentmentNotificationV01>),

    /// camt.108.001.01 - Cheque Cancellation or Stop Request
    #[serde(rename = "ChqCxlOrStopReq")]
    ChequeCancellationOrStopRequestV01(Box<camt_108_001_01::ChequeCancellationOrStopRequestV01>),

    /// camt.109.001.01 - Cheque Cancellation or Stop Report
    #[serde(rename = "ChqCxlOrStopRpt")]
    ChequeCancellationOrStopReportV01(Box<camt_109_001_01::ChequeCancellationOrStopReportV01>),

    // Payments Clearing and Settlement Messages (pacs)
    /// pacs.002.001.10 - FI to FI Payment Status Report
    #[serde(rename = "FIToFIPmtStsRpt")]
    FIToFIPaymentStatusReportV10(Box<pacs_002_001_10::FIToFIPaymentStatusReportV10>),

    /// pacs.003.001.08 - FI to FI Customer Direct Debit
    #[serde(rename = "FIToFICstmrDrctDbt")]
    FIToFICustomerDirectDebitV08(Box<pacs_003_001_08::FIToFICustomerDirectDebitV08>),

    /// pacs.004.001.09 - Payment Return
    #[serde(rename = "PmtRtr")]
    PaymentReturnV09(Box<pacs_004_001_09::PaymentReturnV09>),

    /// pacs.008.001.08 - FI to FI Customer Credit Transfer
    #[serde(rename = "FIToFICstmrCdtTrf")]
    FIToFICustomerCreditTransferV08(Box<pacs_008_001_08::FIToFICustomerCreditTransferV08>),

    /// pacs.008.001.08.stp - FI to FI Customer Credit Transfer (STP)
    #[serde(rename = "FIToFICstmrCdtTrfSTP")]
    FIToFICustomerCreditTransferV08STP(Box<pacs_008_001_08_stp::FIToFICustomerCreditTransferV08>),

    /// pacs.009.001.08 - Financial Institution Credit Transfer
    #[serde(rename = "FinInstnCdtTrf")]
    FinancialInstitutionCreditTransferV08(
        Box<pacs_009_001_08::FinancialInstitutionCreditTransferV08>,
    ),

    /// pacs.009.001.08.adv - Financial Institution Credit Transfer (Advice)
    #[serde(rename = "FinInstnCdtTrfAdv")]
    FinancialInstitutionCreditTransferV08ADV(
        Box<pacs_009_001_08_adv::FinancialInstitutionCreditTransferV08>,
    ),

    /// pacs.009.001.08.cov - Financial Institution Credit Transfer (Cover)
    #[serde(rename = "FinInstnCdtTrfCov")]
    FinancialInstitutionCreditTransferV08COV(
        Box<pacs_009_001_08_cov::FinancialInstitutionCreditTransferV08>,
    ),

    // Payment Initiation Messages (pain)
    /// pain.001.001.09 - Customer Credit Transfer Initiation
    #[serde(rename = "CstmrCdtTrfInitn")]
    CustomerCreditTransferInitiationV09(Box<pain_001_001_09::CustomerCreditTransferInitiationV09>),

    /// pain.002.001.10 - Customer Payment Status Report
    #[serde(rename = "CstmrPmtStsRpt")]
    CustomerPaymentStatusReportV10(Box<pain_002_001_10::CustomerPaymentStatusReportV10>),

    /// pain.008.001.08 - Customer Direct Debit Initiation
    #[serde(rename = "CstmrDrctDbtInitn")]
    CustomerDirectDebitInitiationV08(Box<pain_008_001_08::CustomerDirectDebitInitiationV08>),

    /// Unknown or unsupported document type
    #[default]
    UNKNOWN,
}

impl Document {
    /// Validates the document according to ISO20022 and CBPR+ specifications
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            // Administrative Messages
            Document::NotificationOfCorrespondenceV01(value) => value.validate(),

            // Cash Management Messages
            Document::ReceiptV08(value) => value.validate(),
            Document::ResolutionOfInvestigationV09(value) => value.validate(),
            Document::BankToCustomerAccountReportV08(value) => value.validate(),
            Document::BankToCustomerStatementV08(value) => value.validate(),
            Document::BankToCustomerDebitCreditNotificationV08(value) => value.validate(),
            Document::CustomerPaymentCancellationRequestV08(value) => value.validate(),
            Document::FIToFIPaymentCancellationRequestV08(value) => value.validate(),
            Document::NotificationToReceiveV06(value) => value.validate(),
            Document::NotificationToReceiveCancellationAdviceV08(value) => value.validate(),
            Document::AccountReportingRequestV05(value) => value.validate(),
            Document::ChargesPaymentNotificationV02(value) => value.validate(),
            Document::ChargesPaymentNotificationV02MC(value) => value.validate(),
            Document::ChargesPaymentRequestV02(value) => value.validate(),
            Document::ChargesPaymentRequestV02MC(value) => value.validate(),
            Document::ChequePresentmentNotificationV01(value) => value.validate(),
            Document::ChequeCancellationOrStopRequestV01(value) => value.validate(),
            Document::ChequeCancellationOrStopReportV01(value) => value.validate(),

            // Payments Clearing and Settlement Messages
            Document::FIToFIPaymentStatusReportV10(value) => value.validate(),
            Document::FIToFICustomerDirectDebitV08(value) => value.validate(),
            Document::PaymentReturnV09(value) => value.validate(),
            Document::FIToFICustomerCreditTransferV08(value) => value.validate(),
            Document::FIToFICustomerCreditTransferV08STP(value) => value.validate(),
            Document::FinancialInstitutionCreditTransferV08(value) => value.validate(),
            Document::FinancialInstitutionCreditTransferV08ADV(value) => value.validate(),
            Document::FinancialInstitutionCreditTransferV08COV(value) => value.validate(),

            // Payment Initiation Messages
            Document::CustomerCreditTransferInitiationV09(value) => value.validate(),
            Document::CustomerPaymentStatusReportV10(value) => value.validate(),
            Document::CustomerDirectDebitInitiationV08(value) => value.validate(),

            Document::UNKNOWN => Err(ValidationError::new(
                9999,
                "Unknown document type".to_string(),
            )),
        }
    }

    /// Returns the message type identifier for the document
    pub fn message_type(&self) -> &'static str {
        match self {
            // Administrative Messages
            Document::NotificationOfCorrespondenceV01(_) => "admi.024.001.01",

            // Cash Management Messages
            Document::ReceiptV08(_) => "camt.025.001.08",
            Document::ResolutionOfInvestigationV09(_) => "camt.029.001.09",
            Document::BankToCustomerAccountReportV08(_) => "camt.052.001.08",
            Document::BankToCustomerStatementV08(_) => "camt.053.001.08",
            Document::BankToCustomerDebitCreditNotificationV08(_) => "camt.054.001.08",
            Document::CustomerPaymentCancellationRequestV08(_) => "camt.055.001.08",
            Document::FIToFIPaymentCancellationRequestV08(_) => "camt.056.001.08",
            Document::NotificationToReceiveV06(_) => "camt.057.001.06",
            Document::NotificationToReceiveCancellationAdviceV08(_) => "camt.058.001.08",
            Document::AccountReportingRequestV05(_) => "camt.060.001.05",
            Document::ChargesPaymentNotificationV02(_) => "camt.105.001.02",
            Document::ChargesPaymentNotificationV02MC(_) => "camt.105.001.02.mc",
            Document::ChargesPaymentRequestV02(_) => "camt.106.001.02",
            Document::ChargesPaymentRequestV02MC(_) => "camt.106.001.02.mc",
            Document::ChequePresentmentNotificationV01(_) => "camt.107.001.01",
            Document::ChequeCancellationOrStopRequestV01(_) => "camt.108.001.01",
            Document::ChequeCancellationOrStopReportV01(_) => "camt.109.001.01",

            // Payments Clearing and Settlement Messages
            Document::FIToFIPaymentStatusReportV10(_) => "pacs.002.001.10",
            Document::FIToFICustomerDirectDebitV08(_) => "pacs.003.001.08",
            Document::PaymentReturnV09(_) => "pacs.004.001.09",
            Document::FIToFICustomerCreditTransferV08(_) => "pacs.008.001.08",
            Document::FIToFICustomerCreditTransferV08STP(_) => "pacs.008.001.08.stp",
            Document::FinancialInstitutionCreditTransferV08(_) => "pacs.009.001.08",
            Document::FinancialInstitutionCreditTransferV08ADV(_) => "pacs.009.001.08.adv",
            Document::FinancialInstitutionCreditTransferV08COV(_) => "pacs.009.001.08.cov",

            // Payment Initiation Messages
            Document::CustomerCreditTransferInitiationV09(_) => "pain.001.001.09",
            Document::CustomerPaymentStatusReportV10(_) => "pain.002.001.10",
            Document::CustomerDirectDebitInitiationV08(_) => "pain.008.001.08",

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
