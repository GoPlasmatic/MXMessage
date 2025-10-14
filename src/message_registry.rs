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

//! Message Type Registry
//!
//! Single source of truth for ISO20022 message type mappings.
//! This module provides mappings between:
//! - Short form message types (e.g., "pacs.008")
//! - Full form message types (e.g., "pacs.008.001.08")
//! - Rust struct names (e.g., "FIToFICustomerCreditTransferV08")
//! - XML element names (e.g., "FIToFICstmrCdtTrf")
//! - XML namespaces

/// Message type registry entry
/// Format: (short_form, full_form, rust_type_name, xml_element_name)
pub struct MessageTypeInfo {
    pub short_form: &'static str,
    pub full_form: &'static str,
    pub rust_type_name: &'static str,
    pub xml_element_name: &'static str,
}

/// Complete registry of all supported ISO20022 message types
pub const MESSAGE_REGISTRY: &[MessageTypeInfo] = &[
    // PACS - Payment Clearing and Settlement
    MessageTypeInfo {
        short_form: "pacs.008",
        full_form: "pacs.008.001.08",
        rust_type_name: "FIToFICustomerCreditTransferV08",
        xml_element_name: "FIToFICstmrCdtTrf",
    },
    MessageTypeInfo {
        short_form: "pacs.002",
        full_form: "pacs.002.001.10",
        rust_type_name: "FIToFIPaymentStatusReportV10",
        xml_element_name: "FIToFIPmtStsRpt",
    },
    MessageTypeInfo {
        short_form: "pacs.003",
        full_form: "pacs.003.001.08",
        rust_type_name: "FIToFICustomerDirectDebitV08",
        xml_element_name: "FIToFICstmrDrctDbt",
    },
    MessageTypeInfo {
        short_form: "pacs.004",
        full_form: "pacs.004.001.09",
        rust_type_name: "PaymentReturnV09",
        xml_element_name: "PmtRtr",
    },
    MessageTypeInfo {
        short_form: "pacs.009",
        full_form: "pacs.009.001.08",
        rust_type_name: "FinancialInstitutionCreditTransferV08",
        xml_element_name: "FICdtTrf",
    },
    MessageTypeInfo {
        short_form: "pacs.010",
        full_form: "pacs.010.001.03",
        rust_type_name: "FinancialInstitutionDirectDebitV03",
        xml_element_name: "FIDrctDbt",
    },
    // PAIN - Payment Initiation
    MessageTypeInfo {
        short_form: "pain.001",
        full_form: "pain.001.001.09",
        rust_type_name: "CustomerCreditTransferInitiationV09",
        xml_element_name: "CstmrCdtTrfInitn",
    },
    MessageTypeInfo {
        short_form: "pain.002",
        full_form: "pain.002.001.10",
        rust_type_name: "CustomerPaymentStatusReportV10",
        xml_element_name: "CstmrPmtStsRpt",
    },
    MessageTypeInfo {
        short_form: "pain.008",
        full_form: "pain.008.001.08",
        rust_type_name: "CustomerDirectDebitInitiationV08",
        xml_element_name: "CstmrDrctDbtInitn",
    },
    // CAMT - Cash Management
    MessageTypeInfo {
        short_form: "camt.025",
        full_form: "camt.025.001.08",
        rust_type_name: "ReceiptV08",
        xml_element_name: "Rcpt",
    },
    MessageTypeInfo {
        short_form: "camt.029",
        full_form: "camt.029.001.09",
        rust_type_name: "ResolutionOfInvestigationV09",
        xml_element_name: "RsltnOfInvstgtn",
    },
    MessageTypeInfo {
        short_form: "camt.052",
        full_form: "camt.052.001.08",
        rust_type_name: "BankToCustomerAccountReportV08",
        xml_element_name: "BkToCstmrAcctRpt",
    },
    MessageTypeInfo {
        short_form: "camt.053",
        full_form: "camt.053.001.08",
        rust_type_name: "BankToCustomerStatementV08",
        xml_element_name: "BkToCstmrStmt",
    },
    MessageTypeInfo {
        short_form: "camt.054",
        full_form: "camt.054.001.08",
        rust_type_name: "BankToCustomerDebitCreditNotificationV08",
        xml_element_name: "BkToCstmrDbtCdtNtfctn",
    },
    MessageTypeInfo {
        short_form: "camt.055",
        full_form: "camt.055.001.08",
        rust_type_name: "CustomerPaymentCancellationRequestV08",
        xml_element_name: "CstmrPmtCxlReq",
    },
    MessageTypeInfo {
        short_form: "camt.056",
        full_form: "camt.056.001.08",
        rust_type_name: "FIToFIPaymentCancellationRequestV08",
        xml_element_name: "FIToFIPmtCxlReq",
    },
    MessageTypeInfo {
        short_form: "camt.058",
        full_form: "camt.058.001.08",
        rust_type_name: "NotificationToReceiveCancellationAdviceV08",
        xml_element_name: "NtfctnToRcvCxlAdvc",
    },
    MessageTypeInfo {
        short_form: "camt.057",
        full_form: "camt.057.001.06",
        rust_type_name: "NotificationToReceiveV06",
        xml_element_name: "NtfctnToRcv",
    },
    MessageTypeInfo {
        short_form: "camt.060",
        full_form: "camt.060.001.05",
        rust_type_name: "AccountReportingRequestV05",
        xml_element_name: "AcctRptgReq",
    },
    MessageTypeInfo {
        short_form: "camt.105",
        full_form: "camt.105.001.02",
        rust_type_name: "ChargesPaymentNotificationV02",
        xml_element_name: "ChrgsPmtNtfctn",
    },
    MessageTypeInfo {
        short_form: "camt.106",
        full_form: "camt.106.001.02",
        rust_type_name: "ChargesPaymentRequestV02",
        xml_element_name: "ChrgsPmtReq",
    },
    MessageTypeInfo {
        short_form: "camt.107",
        full_form: "camt.107.001.01",
        rust_type_name: "ChequePresentmentNotificationV01",
        xml_element_name: "ChqPresntmntNtfctn",
    },
    MessageTypeInfo {
        short_form: "camt.108",
        full_form: "camt.108.001.01",
        rust_type_name: "ChequeCancellationOrStopRequestV01",
        xml_element_name: "ChqCxlOrStopReq",
    },
    MessageTypeInfo {
        short_form: "camt.109",
        full_form: "camt.109.001.01",
        rust_type_name: "ChequeCancellationOrStopReportV01",
        xml_element_name: "ChqCxlOrStopRpt",
    },
    // ADMI - Administration
    MessageTypeInfo {
        short_form: "admi.024",
        full_form: "admi.024.001.01",
        rust_type_name: "NotificationOfCorrespondenceV01",
        xml_element_name: "NtfctnOfCrspdc",
    },
];

/// Get namespace URI for a message type
pub fn get_namespace(message_type: &str) -> String {
    // Look up in the registry
    for info in MESSAGE_REGISTRY {
        if message_type == info.short_form || message_type == info.full_form {
            return format!("urn:iso:std:iso:20022:tech:xsd:{}", info.full_form);
        }
    }

    // Default fallback: construct namespace from message type
    format!("urn:iso:std:iso:20022:tech:xsd:{}", message_type)
}

/// Convert message type to short form (e.g., "pacs.008.001.08" -> "pacs.008")
pub fn normalize_message_type(message_type: &str) -> String {
    for info in MESSAGE_REGISTRY {
        if message_type == info.short_form || message_type == info.full_form {
            return info.short_form.to_string();
        }
    }
    message_type.to_string()
}

/// Map XML element name to message type short form
pub fn element_to_message_type(element_name: &str) -> Option<&'static str> {
    MESSAGE_REGISTRY
        .iter()
        .find(|info| info.xml_element_name == element_name || info.rust_type_name == element_name)
        .map(|info| info.short_form)
}

/// Map message type to XML element name
pub fn message_type_to_element(message_type: &str) -> Option<&'static str> {
    MESSAGE_REGISTRY
        .iter()
        .find(|info| info.short_form == message_type || info.full_form == message_type)
        .map(|info| info.xml_element_name)
}

/// Map message type to Rust type name
pub fn message_type_to_rust_type(message_type: &str) -> Option<&'static str> {
    MESSAGE_REGISTRY
        .iter()
        .find(|info| info.short_form == message_type || info.full_form == message_type)
        .map(|info| info.rust_type_name)
}

/// Get full form of message type (e.g., "pacs.008" -> "pacs.008.001.08")
pub fn get_full_form(message_type: &str) -> Option<&'static str> {
    MESSAGE_REGISTRY
        .iter()
        .find(|info| info.short_form == message_type || info.full_form == message_type)
        .map(|info| info.full_form)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_to_message_type() {
        assert_eq!(
            element_to_message_type("FIToFICstmrCdtTrf"),
            Some("pacs.008")
        );
        assert_eq!(element_to_message_type("BkToCstmrStmt"), Some("camt.053"));
        assert_eq!(
            element_to_message_type("CstmrCdtTrfInitn"),
            Some("pain.001")
        );
        assert_eq!(element_to_message_type("UnknownElement"), None);
    }

    #[test]
    fn test_message_type_to_element() {
        assert_eq!(
            message_type_to_element("pacs.008"),
            Some("FIToFICstmrCdtTrf")
        );
        assert_eq!(
            message_type_to_element("pacs.008.001.08"),
            Some("FIToFICstmrCdtTrf")
        );
        assert_eq!(message_type_to_element("unknown"), None);
    }

    #[test]
    fn test_normalize_message_type() {
        assert_eq!(normalize_message_type("pacs.008.001.08"), "pacs.008");
        assert_eq!(normalize_message_type("pacs.008"), "pacs.008");
        assert_eq!(normalize_message_type("unknown.type"), "unknown.type");
    }

    #[test]
    fn test_get_namespace() {
        assert_eq!(
            get_namespace("pacs.008"),
            "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"
        );
        assert_eq!(
            get_namespace("camt.053"),
            "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08"
        );
    }

    #[test]
    fn test_get_full_form() {
        assert_eq!(get_full_form("pacs.008"), Some("pacs.008.001.08"));
        assert_eq!(get_full_form("pacs.008.001.08"), Some("pacs.008.001.08"));
        assert_eq!(get_full_form("unknown"), None);
    }
}
