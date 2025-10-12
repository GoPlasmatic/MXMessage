// MX Message Envelope Structure for ISO 20022 compliant XML generation

use serde::{Deserialize, Serialize};

// Re-export AppHdr for convenience
use crate::error::MxError;
pub use crate::header::AppHdr;
use crate::message_registry;

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

    #[serde(rename = "CstmrPmtCxlReq")]
    Camt055(Box<crate::document::camt_055_001_08::CustomerPaymentCancellationRequestV08>),

    #[serde(rename = "FIToFIPmtCxlReq")]
    Camt056(Box<crate::document::camt_056_001_08::FIToFIPaymentCancellationRequestV08>),

    #[serde(rename = "NtfctnToRcvCxlAdvc")]
    Camt058(Box<crate::document::camt_058_001_08::NotificationToReceiveCancellationAdviceV08>),

    #[serde(rename = "NtfctnToRcv")]
    Camt057(Box<crate::document::camt_057_001_06::NotificationToReceiveV06>),

    #[serde(rename = "AcctRptgReq")]
    Camt060(Box<crate::document::camt_060_001_05::AccountReportingRequestV05>),

    #[serde(rename = "ChrgsPmtNtfctn")]
    Camt105(Box<crate::document::camt_105_001_02::ChargesPaymentNotificationV02>),

    #[serde(rename = "ChrgsPmtReq")]
    Camt106(Box<crate::document::camt_106_001_02::ChargesPaymentRequestV02>),

    #[serde(rename = "ChqPresntmntNtfctn")]
    Camt107(Box<crate::document::camt_107_001_01::ChequePresentmentNotificationV01>),

    #[serde(rename = "ChqCxlOrStopReq")]
    Camt108(Box<crate::document::camt_108_001_01::ChequeCancellationOrStopRequestV01>),

    #[serde(rename = "ChqCxlOrStopRpt")]
    Camt109(Box<crate::document::camt_109_001_01::ChequeCancellationOrStopReportV01>),

    // ADMI - Administration
    #[serde(rename = "NtfctnOfCrspdc")]
    Admi024(Box<crate::document::admi_024_001_01::NotificationOfCorrespondenceV01>),
}

impl Document {
    /// Get the namespace for this document based on its type
    pub fn namespace(&self) -> String {
        let msg_type = match self {
            Document::Pacs008(_) => "pacs.008",
            Document::Pacs009(_) => "pacs.009",
            Document::Pacs003(_) => "pacs.003",
            Document::Pacs004(_) => "pacs.004",
            Document::Pacs002(_) => "pacs.002",
            Document::Pacs010(_) => "pacs.010",
            Document::Pain001(_) => "pain.001",
            Document::Pain002(_) => "pain.002",
            Document::Pain008(_) => "pain.008",
            Document::Camt025(_) => "camt.025",
            Document::Camt029(_) => "camt.029",
            Document::Camt052(_) => "camt.052",
            Document::Camt053(_) => "camt.053",
            Document::Camt054(_) => "camt.054",
            Document::Camt055(_) => "camt.055",
            Document::Camt056(_) => "camt.056",
            Document::Camt057(_) => "camt.057",
            Document::Camt058(_) => "camt.058",
            Document::Camt060(_) => "camt.060",
            Document::Camt105(_) => "camt.105",
            Document::Camt106(_) => "camt.106",
            Document::Camt107(_) => "camt.107",
            Document::Camt108(_) => "camt.108",
            Document::Camt109(_) => "camt.109",
            Document::Admi024(_) => "admi.024",
        };
        message_registry::get_namespace(msg_type)
    }
}

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

/// Get the appropriate namespace for a message type
/// Delegates to message_registry module
pub fn get_namespace_for_message_type(message_type: &str) -> String {
    message_registry::get_namespace(message_type)
}

/// Get the short form of message type (e.g., "pacs.008")
/// Delegates to message_registry module
pub fn normalize_message_type(message_type: &str) -> String {
    message_registry::normalize_message_type(message_type)
}

/// Macro to reduce serialization boilerplate
macro_rules! serialize_doc {
    ($doc:expr, $rust_type:expr, $xml_elem:expr, $msg_type:expr) => {
        MxMessage::serialize_with_rename($doc.as_ref(), $rust_type, $xml_elem, $msg_type)
    };
}

/// Macro to reduce deserialization boilerplate
macro_rules! deserialize_doc {
    ($xml:expr, $path:path, $variant:ident, $msg_type:expr) => {{
        let doc = quick_xml::de::from_str::<$path>($xml).map_err(|e| {
            MxError::XmlDeserialization(format!("Failed to parse {}: {}", $msg_type, e))
        })?;
        Ok(Document::$variant(Box::new(doc)))
    }};
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

    /// Helper function to serialize a document with struct name replacement
    fn serialize_with_rename<T: Serialize>(
        value: &T,
        rust_type: &str,
        xml_element: &str,
        msg_type: &str,
    ) -> Result<String, MxError> {
        let xml = quick_xml::se::to_string(value).map_err(|e| {
            MxError::XmlSerialization(format!("Failed to serialize {}: {}", msg_type, e))
        })?;
        Ok(xml
            .replace(&format!("<{}>", rust_type), &format!("<{}>", xml_element))
            .replace(&format!("</{}>", rust_type), &format!("</{}>", xml_element)))
    }

    /// Serialize to XML string
    pub fn to_xml(&self) -> Result<String, MxError> {
        // Custom serialization to handle enum variants
        // Serialize AppHdr
        let app_hdr_xml = quick_xml::se::to_string(&self.app_hdr)
            .map_err(|e| MxError::XmlSerialization(format!("Failed to serialize AppHdr: {}", e)))?;

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
            Document::Pacs008(doc) => serialize_doc!(
                doc,
                "FIToFICustomerCreditTransferV08",
                "FIToFICstmrCdtTrf",
                "pacs.008"
            ),
            Document::Pacs002(doc) => serialize_doc!(
                doc,
                "FIToFIPaymentStatusReportV10",
                "FIToFIPmtStsRpt",
                "pacs.002"
            ),
            Document::Pacs003(doc) => serialize_doc!(
                doc,
                "FIToFICustomerDirectDebitV08",
                "FIToFICstmrDrctDbt",
                "pacs.003"
            ),
            Document::Pacs004(doc) => serialize_doc!(doc, "PaymentReturnV09", "PmtRtr", "pacs.004"),
            Document::Pacs009(doc) => serialize_doc!(
                doc,
                "FinancialInstitutionCreditTransferV08",
                "FICdtTrf",
                "pacs.009"
            ),
            Document::Pacs010(doc) => serialize_doc!(
                doc,
                "FinancialInstitutionDirectDebitV03",
                "FIDrctDbt",
                "pacs.010"
            ),
            Document::Pain001(doc) => serialize_doc!(
                doc,
                "CustomerCreditTransferInitiationV09",
                "CstmrCdtTrfInitn",
                "pain.001"
            ),
            Document::Pain002(doc) => serialize_doc!(
                doc,
                "CustomerPaymentStatusReportV10",
                "CstmrPmtStsRpt",
                "pain.002"
            ),
            Document::Pain008(doc) => serialize_doc!(
                doc,
                "CustomerDirectDebitInitiationV08",
                "CstmrDrctDbtInitn",
                "pain.008"
            ),
            Document::Camt025(doc) => serialize_doc!(doc, "ReceiptV08", "Rcpt", "camt.025"),
            Document::Camt029(doc) => serialize_doc!(
                doc,
                "ResolutionOfInvestigationV09",
                "RsltnOfInvstgtn",
                "camt.029"
            ),
            Document::Camt052(doc) => serialize_doc!(
                doc,
                "BankToCustomerAccountReportV08",
                "BkToCstmrAcctRpt",
                "camt.052"
            ),
            Document::Camt053(doc) => serialize_doc!(
                doc,
                "BankToCustomerStatementV08",
                "BkToCstmrStmt",
                "camt.053"
            ),
            Document::Camt054(doc) => serialize_doc!(
                doc,
                "BankToCustomerDebitCreditNotificationV08",
                "BkToCstmrDbtCdtNtfctn",
                "camt.054"
            ),
            Document::Camt055(doc) => serialize_doc!(
                doc,
                "CustomerPaymentCancellationRequestV08",
                "CstmrPmtCxlReq",
                "camt.055"
            ),
            Document::Camt056(doc) => serialize_doc!(
                doc,
                "FIToFIPaymentCancellationRequestV08",
                "FIToFIPmtCxlReq",
                "camt.056"
            ),
            Document::Camt058(doc) => serialize_doc!(
                doc,
                "NotificationToReceiveCancellationAdviceV08",
                "NtfctnToRcvCxlAdvc",
                "camt.058"
            ),
            Document::Camt057(doc) => {
                serialize_doc!(doc, "NotificationToReceiveV06", "NtfctnToRcv", "camt.057")
            }
            Document::Camt060(doc) => {
                serialize_doc!(doc, "AccountReportingRequestV05", "AcctRptgReq", "camt.060")
            }
            Document::Camt105(doc) => serialize_doc!(
                doc,
                "ChargesPaymentNotificationV02",
                "ChrgsPmtNtfctn",
                "camt.105"
            ),
            Document::Camt106(doc) => serialize_doc!(
                doc,
                "ChargesPaymentRequestV02",
                "ChrgsPmtReq",
                "camt.106"
            ),
            Document::Camt107(doc) => serialize_doc!(
                doc,
                "ChequePresentmentNotificationV01",
                "ChqPresntmntNtfctn",
                "camt.107"
            ),
            Document::Camt108(doc) => serialize_doc!(
                doc,
                "ChequeCancellationOrStopRequestV01",
                "ChqCxlOrStopReq",
                "camt.108"
            ),
            Document::Camt109(doc) => serialize_doc!(
                doc,
                "ChequeCancellationOrStopReportV01",
                "ChqCxlOrStopRpt",
                "camt.109"
            ),
            Document::Admi024(doc) => serialize_doc!(
                doc,
                "NotificationOfCorrespondenceV01",
                "NtfctnOfCrspdc",
                "admi.024"
            ),
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, MxError> {
        serde_json::to_string_pretty(self).map_err(|e| MxError::XmlSerialization(e.to_string()))
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
            .ok_or_else(|| MxError::XmlDeserialization("AppHdr not found in XML".to_string()))?;

        // Deserialize AppHdr using quick-xml
        let app_hdr: crate::header::AppHdr =
            quick_xml::de::from_str(&format!("<AppHdr>{}</AppHdr>", app_hdr_xml)).map_err(|e| {
                MxError::XmlDeserialization(format!("Failed to parse AppHdr: {}", e))
            })?;

        // Extract Document section
        let doc_xml = Self::extract_section(xml, "Document")
            .ok_or_else(|| MxError::XmlDeserialization("Document not found in XML".to_string()))?;

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
        Err(MxError::XmlDeserialization(
            "Document-only XML requires AppHdr information. Use full envelope format.".to_string(),
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
            return Err(MxError::XmlDeserialization(
                "Invalid document XML structure".to_string(),
            ));
        }

        let end_idx = trimmed[1..]
            .find(|c: char| c.is_whitespace() || c == '>')
            .map(|i| i + 1)
            .ok_or_else(|| {
                MxError::XmlDeserialization("Could not find document element".to_string())
            })?;

        let element_name = &trimmed[1..end_idx];

        // Map element name to message type using message registry
        let message_type =
            message_registry::element_to_message_type(element_name).ok_or_else(|| {
                MxError::XmlDeserialization(format!("Unknown document type: {}", element_name))
            })?;

        Ok(message_type.to_string())
    }

    /// Deserialize document based on message type
    fn deserialize_document(doc_xml: &str, message_type: &str) -> Result<Document, MxError> {
        use crate::document::*;

        match message_type {
            "pacs.008" => deserialize_doc!(
                doc_xml,
                pacs_008_001_08::FIToFICustomerCreditTransferV08,
                Pacs008,
                "pacs.008"
            ),
            "pacs.002" => deserialize_doc!(
                doc_xml,
                pacs_002_001_10::FIToFIPaymentStatusReportV10,
                Pacs002,
                "pacs.002"
            ),
            "pacs.003" => deserialize_doc!(
                doc_xml,
                pacs_003_001_08::FIToFICustomerDirectDebitV08,
                Pacs003,
                "pacs.003"
            ),
            "pacs.004" => deserialize_doc!(
                doc_xml,
                pacs_004_001_09::PaymentReturnV09,
                Pacs004,
                "pacs.004"
            ),
            "pacs.009" => deserialize_doc!(
                doc_xml,
                pacs_009_001_08::FinancialInstitutionCreditTransferV08,
                Pacs009,
                "pacs.009"
            ),
            "pacs.010" => deserialize_doc!(
                doc_xml,
                pacs_010_001_03::FinancialInstitutionDirectDebitV03,
                Pacs010,
                "pacs.010"
            ),
            "pain.001" => deserialize_doc!(
                doc_xml,
                pain_001_001_09::CustomerCreditTransferInitiationV09,
                Pain001,
                "pain.001"
            ),
            "pain.002" => deserialize_doc!(
                doc_xml,
                pain_002_001_10::CustomerPaymentStatusReportV10,
                Pain002,
                "pain.002"
            ),
            "pain.008" => deserialize_doc!(
                doc_xml,
                pain_008_001_08::CustomerDirectDebitInitiationV08,
                Pain008,
                "pain.008"
            ),
            "camt.025" => {
                deserialize_doc!(doc_xml, camt_025_001_08::ReceiptV08, Camt025, "camt.025")
            }
            "camt.029" => deserialize_doc!(
                doc_xml,
                camt_029_001_09::ResolutionOfInvestigationV09,
                Camt029,
                "camt.029"
            ),
            "camt.052" => deserialize_doc!(
                doc_xml,
                camt_052_001_08::BankToCustomerAccountReportV08,
                Camt052,
                "camt.052"
            ),
            "camt.053" => deserialize_doc!(
                doc_xml,
                camt_053_001_08::BankToCustomerStatementV08,
                Camt053,
                "camt.053"
            ),
            "camt.054" => deserialize_doc!(
                doc_xml,
                camt_054_001_08::BankToCustomerDebitCreditNotificationV08,
                Camt054,
                "camt.054"
            ),
            "camt.055" => deserialize_doc!(
                doc_xml,
                camt_055_001_08::CustomerPaymentCancellationRequestV08,
                Camt055,
                "camt.055"
            ),
            "camt.056" => deserialize_doc!(
                doc_xml,
                camt_056_001_08::FIToFIPaymentCancellationRequestV08,
                Camt056,
                "camt.056"
            ),
            "camt.058" => deserialize_doc!(
                doc_xml,
                camt_058_001_08::NotificationToReceiveCancellationAdviceV08,
                Camt058,
                "camt.058"
            ),
            "camt.057" => deserialize_doc!(
                doc_xml,
                camt_057_001_06::NotificationToReceiveV06,
                Camt057,
                "camt.057"
            ),
            "camt.060" => deserialize_doc!(
                doc_xml,
                camt_060_001_05::AccountReportingRequestV05,
                Camt060,
                "camt.060"
            ),
            "camt.105" => deserialize_doc!(
                doc_xml,
                camt_105_001_02::ChargesPaymentNotificationV02,
                Camt105,
                "camt.105"
            ),
            "camt.106" => deserialize_doc!(
                doc_xml,
                camt_106_001_02::ChargesPaymentRequestV02,
                Camt106,
                "camt.106"
            ),
            "camt.107" => deserialize_doc!(
                doc_xml,
                camt_107_001_01::ChequePresentmentNotificationV01,
                Camt107,
                "camt.107"
            ),
            "camt.108" => deserialize_doc!(
                doc_xml,
                camt_108_001_01::ChequeCancellationOrStopRequestV01,
                Camt108,
                "camt.108"
            ),
            "camt.109" => deserialize_doc!(
                doc_xml,
                camt_109_001_01::ChequeCancellationOrStopReportV01,
                Camt109,
                "camt.109"
            ),
            "admi.024" => deserialize_doc!(
                doc_xml,
                admi_024_001_01::NotificationOfCorrespondenceV01,
                Admi024,
                "admi.024"
            ),
            _ => Err(MxError::XmlDeserialization(format!(
                "Unsupported message type: {}",
                message_type
            ))),
        }
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, MxError> {
        let message: MxMessage = serde_json::from_str(json).map_err(|e| {
            MxError::XmlDeserialization(format!("JSON deserialization failed: {}", e))
        })?;

        Ok(message)
    }
}

/// Extract message type from XML without full deserialization
pub fn peek_message_type_from_xml(xml: &str) -> Result<String, MxError> {
    // Simple regex-based extraction for MsgDefIdr
    use regex::Regex;

    let re = Regex::new(r"<MsgDefIdr>([^<]+)</MsgDefIdr>")
        .map_err(|e| MxError::XmlDeserialization(format!("Regex error: {}", e)))?;

    if let Some(captures) = re.captures(xml)
        && let Some(msg_def_idr) = captures.get(1)
    {
        return Ok(normalize_message_type(msg_def_idr.as_str()));
    }

    Err(MxError::XmlDeserialization(
        "Could not find MsgDefIdr in XML".to_string(),
    ))
}

/// Extract message type from JSON without full deserialization
pub fn peek_message_type_from_json(json: &str) -> Result<String, MxError> {
    let value: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| MxError::XmlDeserialization(format!("JSON parsing error: {}", e)))?;

    // Try to extract MsgDefIdr from AppHdr
    if let Some(msg_def_idr) = value
        .get("AppHdr")
        .or_else(|| value.get("Envelope").and_then(|e| e.get("AppHdr")))
        .and_then(|hdr| hdr.get("MsgDefIdr"))
        .and_then(|v| v.as_str())
    {
        return Ok(normalize_message_type(msg_def_idr));
    }

    Err(MxError::XmlDeserialization(
        "Could not find MsgDefIdr in JSON".to_string(),
    ))
}
