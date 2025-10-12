// MX Message Envelope Structure for ISO 20022 compliant XML generation

use serde::{Deserialize, Serialize};
use std::fmt;

// Re-export AppHdr for convenience
pub use crate::header::AppHdr;

/// Document enum - represents the Document element in MX messages
/// Each variant uses serde rename to match the XML element name
/// Box wrappers are used to prevent stack overflow from large variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Document {
    // PACS - Payment Clearing and Settlement
    #[serde(rename = "FIToFICstmrCdtTrf")]
    Pacs008(Box<crate::document::pacs_008_001_08::FIToFICustomerCreditTransferV08>),

    #[serde(rename = "FIToFIPmtStsRpt")]
    Pacs002(Box<crate::document::pacs_002_001_10::FIToFIPaymentStatusReportV10>),

    #[serde(rename = "FIToFICstmrDrctDbt")]
    Pacs003(Box<crate::document::pacs_003_001_08::FIToFICustomerDirectDebitV08>),

    #[serde(rename = "PmtRtr")]
    Pacs004(Box<crate::document::pacs_004_001_09::PaymentReturnV09>),

    #[serde(rename = "FICdtTrf")]
    Pacs009(Box<crate::document::pacs_009_001_08::FinancialInstitutionCreditTransferV08>),

    #[serde(rename = "FIDrctDbt")]
    Pacs010(Box<crate::document::pacs_010_001_03::FinancialInstitutionDirectDebitV03>),

    // PAIN - Payment Initiation
    #[serde(rename = "CstmrCdtTrfInitn")]
    Pain001(Box<crate::document::pain_001_001_09::CustomerCreditTransferInitiationV09>),

    #[serde(rename = "CstmrPmtStsRpt")]
    Pain002(Box<crate::document::pain_002_001_10::CustomerPaymentStatusReportV10>),

    #[serde(rename = "CstmrDrctDbtInitn")]
    Pain008(Box<crate::document::pain_008_001_08::CustomerDirectDebitInitiationV08>),

    // CAMT - Cash Management
    #[serde(rename = "Rcpt")]
    Camt025(Box<crate::document::camt_025_001_08::ReceiptV08>),

    #[serde(rename = "RsltnOfInvstgtn")]
    Camt029(Box<crate::document::camt_029_001_09::ResolutionOfInvestigationV09>),

    #[serde(rename = "BkToCstmrAcctRpt")]
    Camt052(Box<crate::document::camt_052_001_08::BankToCustomerAccountReportV08>),

    #[serde(rename = "BkToCstmrStmt")]
    Camt053(Box<crate::document::camt_053_001_08::BankToCustomerStatementV08>),

    #[serde(rename = "BkToCstmrDbtCdtNtfctn")]
    Camt054(Box<crate::document::camt_054_001_08::BankToCustomerDebitCreditNotificationV08>),

    #[serde(rename = "FIToFIPmtCxlReq")]
    Camt056(Box<crate::document::camt_056_001_08::FIToFIPaymentCancellationRequestV08>),

    #[serde(rename = "NtfctnToRcv")]
    Camt057(Box<crate::document::camt_057_001_06::NotificationToReceiveV06>),

    #[serde(rename = "AcctRptgReq")]
    Camt060(Box<crate::document::camt_060_001_05::AccountReportingRequestV05>),

    #[serde(rename = "ChqPresntmntNtfctn")]
    Camt107(Box<crate::document::camt_107_001_01::ChequePresentmentNotificationV01>),

    #[serde(rename = "ChqCxlOrStopReq")]
    Camt108(Box<crate::document::camt_108_001_01::ChequeCancellationOrStopRequestV01>),

    #[serde(rename = "ChqCxlOrStopRpt")]
    Camt109(Box<crate::document::camt_109_001_01::ChequeCancellationOrStopReportV01>),

    // ADMI - Administration
    #[serde(rename = "NtfctnOfCrrspndnc")]
    Admi024(Box<crate::document::admi_024_001_01::NotificationOfCorrespondenceV01>),
}

impl Document {
    /// Get the namespace for this document based on its type
    pub fn namespace(&self) -> String {
        match self {
            Document::Pacs008(_) => "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08".to_string(),
            Document::Pacs009(_) => "urn:iso:std:iso:20022:tech:xsd:pacs.009.001.08".to_string(),
            Document::Pacs003(_) => "urn:iso:std:iso:20022:tech:xsd:pacs.003.001.08".to_string(),
            Document::Pacs004(_) => "urn:iso:std:iso:20022:tech:xsd:pacs.004.001.09".to_string(),
            Document::Pacs002(_) => "urn:iso:std:iso:20022:tech:xsd:pacs.002.001.10".to_string(),
            Document::Pacs010(_) => "urn:iso:std:iso:20022:tech:xsd:pacs.010.001.03".to_string(),
            Document::Pain001(_) => "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09".to_string(),
            Document::Pain002(_) => "urn:iso:std:iso:20022:tech:xsd:pain.002.001.10".to_string(),
            Document::Pain008(_) => "urn:iso:std:iso:20022:tech:xsd:pain.008.001.08".to_string(),
            Document::Camt025(_) => "urn:iso:std:iso:20022:tech:xsd:camt.025.001.08".to_string(),
            Document::Camt029(_) => "urn:iso:std:iso:20022:tech:xsd:camt.029.001.09".to_string(),
            Document::Camt052(_) => "urn:iso:std:iso:20022:tech:xsd:camt.052.001.08".to_string(),
            Document::Camt053(_) => "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08".to_string(),
            Document::Camt054(_) => "urn:iso:std:iso:20022:tech:xsd:camt.054.001.08".to_string(),
            Document::Camt056(_) => "urn:iso:std:iso:20022:tech:xsd:camt.056.001.08".to_string(),
            Document::Camt057(_) => "urn:iso:std:iso:20022:tech:xsd:camt.057.001.06".to_string(),
            Document::Camt060(_) => "urn:iso:std:iso:20022:tech:xsd:camt.060.001.05".to_string(),
            Document::Camt107(_) => "urn:iso:std:iso:20022:tech:xsd:camt.107.001.01".to_string(),
            Document::Camt108(_) => "urn:iso:std:iso:20022:tech:xsd:camt.108.001.01".to_string(),
            Document::Camt109(_) => "urn:iso:std:iso:20022:tech:xsd:camt.109.001.01".to_string(),
            Document::Admi024(_) => "urn:iso:std:iso:20022:tech:xsd:admi.024.001.01".to_string(),
        }
    }
}

/// Error type for MX envelope operations
#[derive(Debug)]
pub enum MxError {
    SerializationError(String),
    DeserializationError(String),
    ValidationError(String),
}

impl fmt::Display for MxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MxError::SerializationError(msg) => write!(f, "MX Serialization Error: {}", msg),
            MxError::DeserializationError(msg) => write!(f, "MX Deserialization Error: {}", msg),
            MxError::ValidationError(msg) => write!(f, "MX Validation Error: {}", msg),
        }
    }
}

impl std::error::Error for MxError {}

/// Complete MX message containing Business Application Header and Document
/// This is the unified structure for all ISO20022 message types
/// The message type is determined from the AppHdr.MsgDefIdr field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Envelope")]
pub struct MxMessage {
    /// XML namespace declarations
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,

    #[serde(rename = "@xmlns:xsi", skip_serializing_if = "Option::is_none")]
    pub xmlns_xsi: Option<String>,

    /// Business Application Header
    #[serde(rename = "AppHdr")]
    pub app_hdr: crate::header::AppHdr,

    /// Document containing the actual message
    #[serde(rename = "Document")]
    pub document: Document,
}

impl MxMessage {
    /// Create a new MX message with default namespaces
    pub fn new(app_hdr: crate::header::AppHdr, document: Document) -> Self {
        Self {
            xmlns: Some("urn:iso:std:iso:20022:tech:xsd:head.001.001.02".to_string()),
            xmlns_xsi: Some("http://www.w3.org/2001/XMLSchema-instance".to_string()),
            app_hdr,
            document,
        }
    }
}

/// Message type to namespace mapping
/// Format: (short_form, full_form, namespace)
const NAMESPACE_MAPPINGS: &[(&str, &str, &str)] = &[
    (
        "pacs.008",
        "pacs.008.001.08",
        "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
    ),
    (
        "pacs.009",
        "pacs.009.001.08",
        "urn:iso:std:iso:20022:tech:xsd:pacs.009.001.08",
    ),
    (
        "pacs.003",
        "pacs.003.001.08",
        "urn:iso:std:iso:20022:tech:xsd:pacs.003.001.08",
    ),
    (
        "pacs.004",
        "pacs.004.001.09",
        "urn:iso:std:iso:20022:tech:xsd:pacs.004.001.09",
    ),
    (
        "pacs.002",
        "pacs.002.001.10",
        "urn:iso:std:iso:20022:tech:xsd:pacs.002.001.10",
    ),
    (
        "pacs.010",
        "pacs.010.001.03",
        "urn:iso:std:iso:20022:tech:xsd:pacs.010.001.03",
    ),
    (
        "pain.001",
        "pain.001.001.09",
        "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09",
    ),
    (
        "pain.002",
        "pain.002.001.10",
        "urn:iso:std:iso:20022:tech:xsd:pain.002.001.10",
    ),
    (
        "pain.008",
        "pain.008.001.08",
        "urn:iso:std:iso:20022:tech:xsd:pain.008.001.08",
    ),
    (
        "camt.025",
        "camt.025.001.08",
        "urn:iso:std:iso:20022:tech:xsd:camt.025.001.08",
    ),
    (
        "camt.029",
        "camt.029.001.09",
        "urn:iso:std:iso:20022:tech:xsd:camt.029.001.09",
    ),
    (
        "camt.052",
        "camt.052.001.08",
        "urn:iso:std:iso:20022:tech:xsd:camt.052.001.08",
    ),
    (
        "camt.053",
        "camt.053.001.08",
        "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08",
    ),
    (
        "camt.054",
        "camt.054.001.08",
        "urn:iso:std:iso:20022:tech:xsd:camt.054.001.08",
    ),
    (
        "camt.056",
        "camt.056.001.08",
        "urn:iso:std:iso:20022:tech:xsd:camt.056.001.08",
    ),
    (
        "camt.057",
        "camt.057.001.06",
        "urn:iso:std:iso:20022:tech:xsd:camt.057.001.06",
    ),
    (
        "camt.060",
        "camt.060.001.05",
        "urn:iso:std:iso:20022:tech:xsd:camt.060.001.05",
    ),
    (
        "camt.107",
        "camt.107.001.01",
        "urn:iso:std:iso:20022:tech:xsd:camt.107.001.01",
    ),
    (
        "camt.108",
        "camt.108.001.01",
        "urn:iso:std:iso:20022:tech:xsd:camt.108.001.01",
    ),
    (
        "camt.109",
        "camt.109.001.01",
        "urn:iso:std:iso:20022:tech:xsd:camt.109.001.01",
    ),
    (
        "admi.024",
        "admi.024.001.01",
        "urn:iso:std:iso:20022:tech:xsd:admi.024.001.01",
    ),
];

/// Get the appropriate namespace for a message type
pub fn get_namespace_for_message_type(message_type: &str) -> String {
    // Look up in the static mapping
    for (short_form, full_form, namespace) in NAMESPACE_MAPPINGS {
        if message_type == *short_form || message_type == *full_form {
            return namespace.to_string();
        }
    }

    // Default fallback: construct namespace from message type
    format!("urn:iso:std:iso:20022:tech:xsd:{}", message_type)
}

/// Get the short form of message type (e.g., "pacs.008")
pub fn normalize_message_type(message_type: &str) -> String {
    for (short_form, full_form, _) in NAMESPACE_MAPPINGS {
        if message_type == *short_form || message_type == *full_form {
            return short_form.to_string();
        }
    }
    message_type.to_string()
}

impl MxMessage {
    /// Get the message type identifier from AppHdr (e.g., "pacs.008.001.08")
    pub fn message_type(&self) -> Result<&str, MxError> {
        Ok(&self.app_hdr.msg_def_idr)
    }

    /// Get the namespace for this message
    pub fn namespace(&self) -> Result<String, MxError> {
        Ok(get_namespace_for_message_type(self.message_type()?))
    }

    /// Serialize to XML string
    pub fn to_xml(&self) -> Result<String, MxError> {
        // Custom serialization to handle enum variants
        // Serialize AppHdr
        let app_hdr_xml = quick_xml::se::to_string(&self.app_hdr)
            .map_err(|e| MxError::SerializationError(format!("Failed to serialize AppHdr: {}", e)))?;

        // AppHdr is serialized directly without a wrapper tag, use it as-is
        let app_hdr_inner = app_hdr_xml;

        // Serialize Document based on its variant
        let doc_xml = self.serialize_document()?;

        // Build complete envelope
        // Note: quick-xml adds struct name as wrapper, so app_hdr_inner already contains <BusinessApplicationHeaderV02>
        // We need to rename it to <AppHdr>
        let app_hdr_wrapped = app_hdr_inner
            .replace("<BusinessApplicationHeaderV02>", "<AppHdr>")
            .replace("</BusinessApplicationHeaderV02>", "</AppHdr>");

        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        xml.push_str("<Envelope>");
        xml.push_str(&app_hdr_wrapped);
        xml.push_str("<Document>");
        xml.push_str(&doc_xml);
        xml.push_str("</Document>");
        xml.push_str("</Envelope>");

        Ok(xml)
    }

    /// Serialize document based on its variant
    fn serialize_document(&self) -> Result<String, MxError> {
        match &self.document {
            Document::Pacs008(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pacs.008: {}", e)))?;
                // Replace the struct name wrapper with the ISO20022 element name
                let renamed = xml
                    .replace("<FIToFICustomerCreditTransferV08>", "<FIToFICstmrCdtTrf>")
                    .replace("</FIToFICustomerCreditTransferV08>", "</FIToFICstmrCdtTrf>");
                Ok(renamed)
            }
            Document::Pacs002(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pacs.002: {}", e)))?;
                Ok(xml.replace("<FIToFIPaymentStatusReportV10>", "<FIToFIPmtStsRpt>")
                    .replace("</FIToFIPaymentStatusReportV10>", "</FIToFIPmtStsRpt>"))
            }
            Document::Pacs003(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pacs.003: {}", e)))?;
                Ok(xml.replace("<FIToFICustomerDirectDebitV08>", "<FIToFICstmrDrctDbt>")
                    .replace("</FIToFICustomerDirectDebitV08>", "</FIToFICstmrDrctDbt>"))
            }
            Document::Pacs004(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pacs.004: {}", e)))?;
                Ok(xml.replace("<PaymentReturnV09>", "<PmtRtr>")
                    .replace("</PaymentReturnV09>", "</PmtRtr>"))
            }
            Document::Pacs009(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pacs.009: {}", e)))?;
                Ok(xml.replace("<FinancialInstitutionCreditTransferV08>", "<FICdtTrf>")
                    .replace("</FinancialInstitutionCreditTransferV08>", "</FICdtTrf>"))
            }
            Document::Pacs010(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pacs.010: {}", e)))?;
                Ok(xml.replace("<FinancialInstitutionDirectDebitV03>", "<FIDrctDbt>")
                    .replace("</FinancialInstitutionDirectDebitV03>", "</FIDrctDbt>"))
            }
            Document::Pain001(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pain.001: {}", e)))?;
                Ok(xml.replace("<CustomerCreditTransferInitiationV09>", "<CstmrCdtTrfInitn>")
                    .replace("</CustomerCreditTransferInitiationV09>", "</CstmrCdtTrfInitn>"))
            }
            Document::Pain002(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pain.002: {}", e)))?;
                Ok(xml.replace("<CustomerPaymentStatusReportV10>", "<CstmrPmtStsRpt>")
                    .replace("</CustomerPaymentStatusReportV10>", "</CstmrPmtStsRpt>"))
            }
            Document::Pain008(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize pain.008: {}", e)))?;
                Ok(xml.replace("<CustomerDirectDebitInitiationV08>", "<CstmrDrctDbtInitn>")
                    .replace("</CustomerDirectDebitInitiationV08>", "</CstmrDrctDbtInitn>"))
            }
            Document::Camt025(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.025: {}", e)))?;
                Ok(xml.replace("<ReceiptV08>", "<Rcpt>")
                    .replace("</ReceiptV08>", "</Rcpt>"))
            }
            Document::Camt029(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.029: {}", e)))?;
                Ok(xml.replace("<ResolutionOfInvestigationV09>", "<RsltnOfInvstgtn>")
                    .replace("</ResolutionOfInvestigationV09>", "</RsltnOfInvstgtn>"))
            }
            Document::Camt052(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.052: {}", e)))?;
                Ok(xml.replace("<BankToCustomerAccountReportV08>", "<BkToCstmrAcctRpt>")
                    .replace("</BankToCustomerAccountReportV08>", "</BkToCstmrAcctRpt>"))
            }
            Document::Camt053(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.053: {}", e)))?;
                Ok(xml.replace("<BankToCustomerStatementV08>", "<BkToCstmrStmt>")
                    .replace("</BankToCustomerStatementV08>", "</BkToCstmrStmt>"))
            }
            Document::Camt054(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.054: {}", e)))?;
                Ok(xml.replace("<BankToCustomerDebitCreditNotificationV08>", "<BkToCstmrDbtCdtNtfctn>")
                    .replace("</BankToCustomerDebitCreditNotificationV08>", "</BkToCstmrDbtCdtNtfctn>"))
            }
            Document::Camt056(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.056: {}", e)))?;
                Ok(xml.replace("<FIToFIPaymentCancellationRequestV08>", "<FIToFIPmtCxlReq>")
                    .replace("</FIToFIPaymentCancellationRequestV08>", "</FIToFIPmtCxlReq>"))
            }
            Document::Camt057(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.057: {}", e)))?;
                Ok(xml.replace("<NotificationToReceiveV06>", "<NtfctnToRcv>")
                    .replace("</NotificationToReceiveV06>", "</NtfctnToRcv>"))
            }
            Document::Camt060(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.060: {}", e)))?;
                Ok(xml.replace("<AccountReportingRequestV05>", "<AcctRptgReq>")
                    .replace("</AccountReportingRequestV05>", "</AcctRptgReq>"))
            }
            Document::Camt107(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.107: {}", e)))?;
                Ok(xml.replace("<ChequePresentmentNotificationV01>", "<ChqPresntmntNtfctn>")
                    .replace("</ChequePresentmentNotificationV01>", "</ChqPresntmntNtfctn>"))
            }
            Document::Camt108(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.108: {}", e)))?;
                Ok(xml.replace("<ChequeCancellationOrStopRequestV01>", "<ChqCxlOrStopReq>")
                    .replace("</ChequeCancellationOrStopRequestV01>", "</ChqCxlOrStopReq>"))
            }
            Document::Camt109(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize camt.109: {}", e)))?;
                Ok(xml.replace("<ChequeCancellationOrStopReportV01>", "<ChqCxlOrStopRpt>")
                    .replace("</ChequeCancellationOrStopReportV01>", "</ChqCxlOrStopRpt>"))
            }
            Document::Admi024(doc) => {
                let xml = quick_xml::se::to_string(doc.as_ref())
                    .map_err(|e| MxError::SerializationError(format!("Failed to serialize admi.024: {}", e)))?;
                Ok(xml.replace("<NotificationOfCorrespondenceV01>", "<NtfctnOfCrrspndnc>")
                    .replace("</NotificationOfCorrespondenceV01>", "</NtfctnOfCrrspndnc>"))
            }
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, MxError> {
        serde_json::to_string_pretty(self).map_err(|e| MxError::SerializationError(e.to_string()))
    }

    /// Deserialize from XML string using quick-xml with custom enum handling
    pub fn from_xml(xml: &str) -> Result<Self, MxError> {
        // Check if XML contains full envelope or just Document
        let has_envelope = xml.contains("<AppHdr") || xml.contains("<Envelope");

        if has_envelope {
            Self::from_xml_with_envelope(xml)
        } else {
            Self::from_xml_document_only(xml)
        }
    }

    /// Deserialize XML with full envelope (AppHdr + Document)
    fn from_xml_with_envelope(xml: &str) -> Result<Self, MxError> {
        // Extract AppHdr section
        let app_hdr_xml = Self::extract_section(xml, "AppHdr")
            .ok_or_else(|| MxError::DeserializationError("AppHdr not found in XML".to_string()))?;

        // Deserialize AppHdr using quick-xml
        let app_hdr: crate::header::AppHdr = quick_xml::de::from_str(&format!("<AppHdr>{}</AppHdr>", app_hdr_xml))
            .map_err(|e| MxError::DeserializationError(format!("Failed to parse AppHdr: {}", e)))?;

        // Extract Document section
        let doc_xml = Self::extract_section(xml, "Document")
            .ok_or_else(|| MxError::DeserializationError("Document not found in XML".to_string()))?;

        // Determine document type from the first element inside Document
        let doc_type = Self::detect_document_type(&doc_xml)?;

        // Deserialize the document based on its type
        let document = Self::deserialize_document(&doc_xml, &doc_type)?;

        // Extract namespace attributes if present
        let xmlns = Self::extract_attribute(xml, "xmlns");
        let xmlns_xsi = Self::extract_attribute(xml, "xmlns:xsi");

        Ok(MxMessage {
            xmlns,
            xmlns_xsi,
            app_hdr,
            document,
        })
    }

    /// Deserialize document-only XML (no envelope)
    fn from_xml_document_only(_xml: &str) -> Result<Self, MxError> {
        // For document-only XML, we need to create a minimal AppHdr
        // This is a fallback case - typically we expect full envelopes
        Err(MxError::DeserializationError(
            "Document-only XML requires AppHdr information. Use full envelope format.".to_string()
        ))
    }

    /// Extract content between XML tags
    fn extract_section(xml: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}", tag);
        let end_tag = format!("</{}>", tag);

        let start_idx = xml.find(&start_tag)?;
        let content_start = xml[start_idx..].find('>')? + start_idx + 1;
        let end_idx = xml.find(&end_tag)?;

        if content_start < end_idx {
            Some(xml[content_start..end_idx].to_string())
        } else {
            None
        }
    }

    /// Extract XML attribute value
    fn extract_attribute(xml: &str, attr: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr);
        let start_idx = xml.find(&pattern)? + pattern.len();
        let end_idx = xml[start_idx..].find('"')? + start_idx;
        Some(xml[start_idx..end_idx].to_string())
    }

    /// Detect document type from XML content
    fn detect_document_type(doc_xml: &str) -> Result<String, MxError> {
        // Find the first element tag after any whitespace
        let trimmed = doc_xml.trim();
        if !trimmed.starts_with('<') {
            return Err(MxError::DeserializationError(
                "Invalid document XML structure".to_string()
            ));
        }

        let end_idx = trimmed[1..]
            .find(|c: char| c.is_whitespace() || c == '>')
            .map(|i| i + 1)
            .ok_or_else(|| MxError::DeserializationError("Could not find document element".to_string()))?;

        let element_name = &trimmed[1..end_idx];

        // Map element name to message type
        let message_type = match element_name {
            "FIToFICstmrCdtTrf" => "pacs.008",
            "FIToFIPmtStsRpt" => "pacs.002",
            "FIToFICstmrDrctDbt" => "pacs.003",
            "PmtRtr" => "pacs.004",
            "FICdtTrf" => "pacs.009",
            "FIDrctDbt" => "pacs.010",
            "CstmrCdtTrfInitn" => "pain.001",
            "CstmrPmtStsRpt" => "pain.002",
            "CstmrDrctDbtInitn" => "pain.008",
            "Rcpt" => "camt.025",
            "RsltnOfInvstgtn" => "camt.029",
            "BkToCstmrAcctRpt" => "camt.052",
            "BkToCstmrStmt" => "camt.053",
            "BkToCstmrDbtCdtNtfctn" => "camt.054",
            "FIToFIPmtCxlReq" => "camt.056",
            "NtfctnToRcv" => "camt.057",
            "AcctRptgReq" => "camt.060",
            "ChqPresntmntNtfctn" => "camt.107",
            "ChqCxlOrStopReq" => "camt.108",
            "ChqCxlOrStopRpt" => "camt.109",
            "NtfctnOfCrrspndnc" => "admi.024",
            _ => return Err(MxError::DeserializationError(
                format!("Unknown document type: {}", element_name)
            )),
        };

        Ok(message_type.to_string())
    }

    /// Deserialize document based on message type
    fn deserialize_document(doc_xml: &str, message_type: &str) -> Result<Document, MxError> {
        use crate::document::*;

        match message_type {
            "pacs.008" => {
                let doc = quick_xml::de::from_str::<pacs_008_001_08::FIToFICustomerCreditTransferV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pacs.008: {}", e)))?;
                Ok(Document::Pacs008(Box::new(doc)))
            }
            "pacs.002" => {
                let doc = quick_xml::de::from_str::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pacs.002: {}", e)))?;
                Ok(Document::Pacs002(Box::new(doc)))
            }
            "pacs.003" => {
                let doc = quick_xml::de::from_str::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pacs.003: {}", e)))?;
                Ok(Document::Pacs003(Box::new(doc)))
            }
            "pacs.004" => {
                let doc = quick_xml::de::from_str::<pacs_004_001_09::PaymentReturnV09>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pacs.004: {}", e)))?;
                Ok(Document::Pacs004(Box::new(doc)))
            }
            "pacs.009" => {
                let doc = quick_xml::de::from_str::<pacs_009_001_08::FinancialInstitutionCreditTransferV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pacs.009: {}", e)))?;
                Ok(Document::Pacs009(Box::new(doc)))
            }
            "pacs.010" => {
                let doc = quick_xml::de::from_str::<pacs_010_001_03::FinancialInstitutionDirectDebitV03>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pacs.010: {}", e)))?;
                Ok(Document::Pacs010(Box::new(doc)))
            }
            "pain.001" => {
                let doc = quick_xml::de::from_str::<pain_001_001_09::CustomerCreditTransferInitiationV09>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pain.001: {}", e)))?;
                Ok(Document::Pain001(Box::new(doc)))
            }
            "pain.002" => {
                let doc = quick_xml::de::from_str::<pain_002_001_10::CustomerPaymentStatusReportV10>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pain.002: {}", e)))?;
                Ok(Document::Pain002(Box::new(doc)))
            }
            "pain.008" => {
                let doc = quick_xml::de::from_str::<pain_008_001_08::CustomerDirectDebitInitiationV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse pain.008: {}", e)))?;
                Ok(Document::Pain008(Box::new(doc)))
            }
            "camt.025" => {
                let doc = quick_xml::de::from_str::<camt_025_001_08::ReceiptV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.025: {}", e)))?;
                Ok(Document::Camt025(Box::new(doc)))
            }
            "camt.029" => {
                let doc = quick_xml::de::from_str::<camt_029_001_09::ResolutionOfInvestigationV09>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.029: {}", e)))?;
                Ok(Document::Camt029(Box::new(doc)))
            }
            "camt.052" => {
                let doc = quick_xml::de::from_str::<camt_052_001_08::BankToCustomerAccountReportV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.052: {}", e)))?;
                Ok(Document::Camt052(Box::new(doc)))
            }
            "camt.053" => {
                let doc = quick_xml::de::from_str::<camt_053_001_08::BankToCustomerStatementV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.053: {}", e)))?;
                Ok(Document::Camt053(Box::new(doc)))
            }
            "camt.054" => {
                let doc = quick_xml::de::from_str::<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.054: {}", e)))?;
                Ok(Document::Camt054(Box::new(doc)))
            }
            "camt.056" => {
                let doc = quick_xml::de::from_str::<camt_056_001_08::FIToFIPaymentCancellationRequestV08>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.056: {}", e)))?;
                Ok(Document::Camt056(Box::new(doc)))
            }
            "camt.057" => {
                let doc = quick_xml::de::from_str::<camt_057_001_06::NotificationToReceiveV06>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.057: {}", e)))?;
                Ok(Document::Camt057(Box::new(doc)))
            }
            "camt.060" => {
                let doc = quick_xml::de::from_str::<camt_060_001_05::AccountReportingRequestV05>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.060: {}", e)))?;
                Ok(Document::Camt060(Box::new(doc)))
            }
            "camt.107" => {
                let doc = quick_xml::de::from_str::<camt_107_001_01::ChequePresentmentNotificationV01>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.107: {}", e)))?;
                Ok(Document::Camt107(Box::new(doc)))
            }
            "camt.108" => {
                let doc = quick_xml::de::from_str::<camt_108_001_01::ChequeCancellationOrStopRequestV01>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.108: {}", e)))?;
                Ok(Document::Camt108(Box::new(doc)))
            }
            "camt.109" => {
                let doc = quick_xml::de::from_str::<camt_109_001_01::ChequeCancellationOrStopReportV01>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse camt.109: {}", e)))?;
                Ok(Document::Camt109(Box::new(doc)))
            }
            "admi.024" => {
                let doc = quick_xml::de::from_str::<admi_024_001_01::NotificationOfCorrespondenceV01>(doc_xml)
                    .map_err(|e| MxError::DeserializationError(format!("Failed to parse admi.024: {}", e)))?;
                Ok(Document::Admi024(Box::new(doc)))
            }
            _ => Err(MxError::DeserializationError(
                format!("Unsupported message type: {}", message_type)
            )),
        }
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, MxError> {
        let message: MxMessage = serde_json::from_str(json).map_err(|e| {
            MxError::DeserializationError(format!("JSON deserialization failed: {}", e))
        })?;

        Ok(message)
    }
}

/// Extract message type from XML without full deserialization
pub fn peek_message_type_from_xml(xml: &str) -> Result<String, MxError> {
    // Simple regex-based extraction for MsgDefIdr
    use regex::Regex;

    let re = Regex::new(r"<MsgDefIdr>([^<]+)</MsgDefIdr>")
        .map_err(|e| MxError::DeserializationError(format!("Regex error: {}", e)))?;

    if let Some(captures) = re.captures(xml)
        && let Some(msg_def_idr) = captures.get(1)
    {
        return Ok(normalize_message_type(msg_def_idr.as_str()));
    }

    Err(MxError::DeserializationError(
        "Could not find MsgDefIdr in XML".to_string(),
    ))
}

/// Extract message type from JSON without full deserialization
pub fn peek_message_type_from_json(json: &str) -> Result<String, MxError> {
    let value: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| MxError::DeserializationError(format!("JSON parsing error: {}", e)))?;

    // Try to extract MsgDefIdr from AppHdr
    if let Some(msg_def_idr) = value
        .get("AppHdr")
        .or_else(|| value.get("Envelope").and_then(|e| e.get("AppHdr")))
        .and_then(|hdr| hdr.get("MsgDefIdr"))
        .and_then(|v| v.as_str())
    {
        return Ok(normalize_message_type(msg_def_idr));
    }

    Err(MxError::DeserializationError(
        "Could not find MsgDefIdr in JSON".to_string(),
    ))
}
