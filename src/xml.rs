// XML Serialization and Deserialization utilities for MX Messages

use crate::mx_envelope::MxEnvelope;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use quick_xml::{de::from_str as xml_from_str, se::to_string as xml_to_string};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::Cursor;

/// Configuration for XML serialization
#[derive(Debug, Clone)]
pub struct XmlConfig {
    /// Whether to include XML declaration
    pub include_declaration: bool,
    /// Whether to format with indentation
    pub pretty_print: bool,
    /// Indentation string (e.g., "  " for 2 spaces)
    pub indent: String,
    /// Whether to include schema location
    pub include_schema_location: bool,
}

impl Default for XmlConfig {
    fn default() -> Self {
        Self {
            include_declaration: true,
            pretty_print: true,
            indent: "  ".to_string(),
            include_schema_location: false,
        }
    }
}

/// Error type for XML operations
#[derive(Debug)]
pub enum XmlError {
    SerializationError(String),
    DeserializationError(String),
    ValidationError(String),
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlError::SerializationError(msg) => write!(f, "XML Serialization Error: {msg}"),
            XmlError::DeserializationError(msg) => write!(f, "XML Deserialization Error: {msg}"),
            XmlError::ValidationError(msg) => write!(f, "XML Validation Error: {msg}"),
        }
    }
}

impl Error for XmlError {}

/// Serialize any MX message to complete XML with envelope
pub fn to_mx_xml<H, D>(
    message: D,
    header: H,
    message_type: &str,
    config: Option<XmlConfig>,
) -> Result<String, XmlError>
where
    H: Serialize,
    D: Serialize,
{
    let config = config.unwrap_or_default();

    // Determine the namespace based on message type
    let document_namespace = get_namespace_for_message_type(message_type);

    // Create the envelope
    let envelope = MxEnvelope::new(header, message, document_namespace);

    // Use custom XML writer for proper formatting
    if config.pretty_print {
        format_mx_xml(&envelope, &config)
    } else {
        // Use quick-xml for compact output
        xml_to_string(&envelope).map_err(|e| XmlError::SerializationError(e.to_string()))
    }
}

/// Parse complete MX XML with envelope
pub fn from_mx_xml<H, D>(xml: &str) -> Result<MxEnvelope<H, D>, XmlError>
where
    H: for<'de> Deserialize<'de>,
    D: for<'de> Deserialize<'de>,
{
    xml_from_str(xml).map_err(|e| XmlError::DeserializationError(e.to_string()))
}

/// Format MX XML with proper indentation and structure
fn format_mx_xml<H, D>(envelope: &MxEnvelope<H, D>, config: &XmlConfig) -> Result<String, XmlError>
where
    H: Serialize,
    D: Serialize,
{
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', config.indent.len());

    // Write XML declaration
    if config.include_declaration {
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| XmlError::SerializationError(e.to_string()))?;
    }

    // Start envelope element without namespace (just a wrapper)
    let envelope_elem = BytesStart::new("Envelope");
    writer
        .write_event(Event::Start(envelope_elem))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Write AppHdr with its namespace
    let mut app_hdr_elem = BytesStart::new("AppHdr");
    app_hdr_elem.push_attribute(("xmlns", "urn:iso:std:iso:20022:tech:xsd:head.001.001.02"));

    writer
        .write_event(Event::Start(app_hdr_elem))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Serialize the AppHdr content (without the wrapper element)
    let app_hdr_xml = xml_to_string(&envelope.app_hdr)
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Extract just the inner content (remove XML declaration and AppHdr wrapper tags)
    let app_hdr_xml = app_hdr_xml
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim();

    // Remove the opening and closing AppHdr tags to get just the content
    let app_hdr_inner = if app_hdr_xml.starts_with("<AppHdr>") {
        app_hdr_xml
            .trim_start_matches("<AppHdr>")
            .trim_end_matches("</AppHdr>")
    } else if app_hdr_xml.starts_with("<AppHdr") {
        // Handle case where AppHdr might have attributes
        if let Some(pos) = app_hdr_xml.find('>') {
            let content = &app_hdr_xml[pos + 1..];
            content.trim_end_matches("</AppHdr>")
        } else {
            app_hdr_xml
        }
    } else {
        app_hdr_xml
    };

    writer
        .write_event(Event::Text(BytesText::from_escaped(app_hdr_inner)))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Close AppHdr
    writer
        .write_event(Event::End(BytesEnd::new("AppHdr")))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Write Document with its namespace
    let mut doc_elem = BytesStart::new("Document");
    if let Some(ref xmlns) = envelope.document.xmlns {
        doc_elem.push_attribute(("xmlns", xmlns.as_str()));
    }

    writer
        .write_event(Event::Start(doc_elem))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Write the actual message content
    let message_xml = xml_to_string(&envelope.document.message)
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Remove the XML declaration from the inner serialization if present
    let message_xml = message_xml
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim();

    writer
        .write_event(Event::Text(BytesText::from_escaped(message_xml)))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Close Document
    writer
        .write_event(Event::End(BytesEnd::new("Document")))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    // Close Envelope
    writer
        .write_event(Event::End(BytesEnd::new("Envelope")))
        .map_err(|e| XmlError::SerializationError(e.to_string()))?;

    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| XmlError::SerializationError(e.to_string()))
}

/// Message type to namespace mapping
/// Format: (short_form, full_form, namespace)
static NAMESPACE_MAPPINGS: &[(&str, &str, &str)] = &[
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
        "camt.027",
        "camt.027.001.07",
        "urn:iso:std:iso:20022:tech:xsd:camt.027.001.07",
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
];

/// Get the appropriate namespace for a message type
fn get_namespace_for_message_type(message_type: &str) -> String {
    // Look up in the static mapping
    for (short_form, full_form, namespace) in NAMESPACE_MAPPINGS {
        if message_type == *short_form || message_type == *full_form {
            return namespace.to_string();
        }
    }

    // Default fallback: construct namespace from message type
    format!("urn:iso:std:iso:20022:tech:xsd:{}", message_type)
}

/// Helper function to create XML for a specific message type
pub fn create_pacs008_xml<D: Serialize>(
    message: D,
    from_bic: String,
    to_bic: String,
    business_msg_id: String,
) -> Result<String, XmlError> {
    use crate::header::bah_pacs_008_001_08::{
        BranchAndFinancialInstitutionIdentification62, BusinessApplicationHeaderV02,
        FinancialInstitutionIdentification182, Party44Choice1,
    };

    let header = BusinessApplicationHeaderV02 {
        char_set: None,
        fr: Party44Choice1 {
            fi_id: Some(BranchAndFinancialInstitutionIdentification62 {
                fin_instn_id: FinancialInstitutionIdentification182 {
                    bicfi: from_bic,
                    clr_sys_mmb_id: None,
                    lei: None,
                },
            }),
        },
        to: Party44Choice1 {
            fi_id: Some(BranchAndFinancialInstitutionIdentification62 {
                fin_instn_id: FinancialInstitutionIdentification182 {
                    bicfi: to_bic,
                    clr_sys_mmb_id: None,
                    lei: None,
                },
            }),
        },
        biz_msg_idr: business_msg_id,
        msg_def_idr: "pacs.008.001.08".to_string(),
        biz_svc: "swift.ug".to_string(),
        mkt_prctc: None,
        cre_dt: chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string(),
        cpy_dplct: None,
        pssbl_dplct: None,
        prty: None,
        rltd: None,
    };

    to_mx_xml(message, header, "pacs.008", None)
}

/// Helper function to create XML for pain.001 message
pub fn create_pain001_xml<D: Serialize>(
    message: D,
    from_bic: String,
    to_bic: String,
    business_msg_id: String,
) -> Result<String, XmlError> {
    use crate::header::bah_pain_001_001_09::{
        BranchAndFinancialInstitutionIdentification64, BusinessApplicationHeaderV02,
        FinancialInstitutionIdentification183, Party44Choice1,
    };

    let header = BusinessApplicationHeaderV02 {
        char_set: None,
        fr: Party44Choice1 {
            fi_id: Some(BranchAndFinancialInstitutionIdentification64 {
                fin_instn_id: FinancialInstitutionIdentification183 {
                    bicfi: from_bic,
                    clr_sys_mmb_id: None,
                    lei: None,
                },
            }),
        },
        to: Party44Choice1 {
            fi_id: Some(BranchAndFinancialInstitutionIdentification64 {
                fin_instn_id: FinancialInstitutionIdentification183 {
                    bicfi: to_bic,
                    clr_sys_mmb_id: None,
                    lei: None,
                },
            }),
        },
        biz_msg_idr: business_msg_id,
        msg_def_idr: "pain.001.001.09".to_string(),
        biz_svc: "swift.ug".to_string(),
        mkt_prctc: None,
        cre_dt: chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string(),
        cpy_dplct: None,
        pssbl_dplct: None,
        prty: None,
        rltd: None,
    };

    to_mx_xml(message, header, "pain.001", None)
}

/// Helper function to create XML for camt.053 message
pub fn create_camt053_xml<D: Serialize>(
    message: D,
    from_bic: String,
    to_bic: String,
    business_msg_id: String,
) -> Result<String, XmlError> {
    use crate::header::bah_camt_053_001_08::{
        BranchAndFinancialInstitutionIdentification63, BusinessApplicationHeaderV02,
        FinancialInstitutionIdentification182, Party44Choice1,
    };

    let header = BusinessApplicationHeaderV02 {
        char_set: None,
        fr: Party44Choice1 {
            fi_id: Some(BranchAndFinancialInstitutionIdentification63 {
                fin_instn_id: FinancialInstitutionIdentification182 {
                    bicfi: from_bic,
                    clr_sys_mmb_id: None,
                    lei: None,
                },
            }),
        },
        to: Party44Choice1 {
            fi_id: Some(BranchAndFinancialInstitutionIdentification63 {
                fin_instn_id: FinancialInstitutionIdentification182 {
                    bicfi: to_bic,
                    clr_sys_mmb_id: None,
                    lei: None,
                },
            }),
        },
        biz_msg_idr: business_msg_id,
        msg_def_idr: "camt.053.001.08".to_string(),
        biz_svc: "swift.ug".to_string(),
        mkt_prctc: None,
        cre_dt: chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string(),
        cpy_dplct: None,
        pssbl_dplct: None,
        prty: None,
        rltd: None,
    };

    to_mx_xml(message, header, "camt.053", None)
}

/// Convert JSON Value to MX XML using typed document structs
/// This function deserializes JSON into the appropriate typed struct,
/// then uses quick-xml's serializer which correctly handles arrays and attributes.
///
/// This replaces the old custom json_value_to_xml implementation which had issues with:
/// - Arrays being concatenated without separation
/// - Complex nested structures not handled properly
pub fn json_to_typed_xml(
    json_data: &serde_json::Value,
    message_type: &str,
) -> Result<String, XmlError> {
    use crate::document::*;

    // Extract the Document content from the JSON
    let document_json = json_data.get("Document").unwrap_or(json_data);

    // Get the inner message element (e.g., FIToFICstmrCdtTrf)
    let inner_json = if let Some(obj) = document_json.as_object() {
        // Get the first (and usually only) key from Document
        if let Some((_key, value)) = obj.iter().next() {
            value
        } else {
            document_json
        }
    } else {
        document_json
    };

    // Match on message type and deserialize into appropriate typed struct
    let xml = match message_type {
        "pacs.008" => {
            let mx_struct = serde_json::from_value::<
                pacs_008_001_08::FIToFICustomerCreditTransferV08,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.008: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.009" => {
            let mx_struct = serde_json::from_value::<
                pacs_009_001_08::FinancialInstitutionCreditTransferV08,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.009: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.003" => {
            let mx_struct =
                serde_json::from_value::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(
                    inner_json.clone(),
                )
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.003: {}", e))
                })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.004" => {
            let mx_struct =
                serde_json::from_value::<pacs_004_001_09::PaymentReturnV09>(inner_json.clone())
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse pacs.004: {}", e))
                    })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.010" => {
            let mx_struct = serde_json::from_value::<
                pacs_010_001_03::FinancialInstitutionDirectDebitV03,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.010: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.002" => {
            let mx_struct =
                serde_json::from_value::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(
                    inner_json.clone(),
                )
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.002: {}", e))
                })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pain.001" => {
            let mx_struct = serde_json::from_value::<
                pain_001_001_09::CustomerCreditTransferInitiationV09,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pain.001: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pain.008" => {
            let mx_struct = serde_json::from_value::<
                pain_008_001_08::CustomerDirectDebitInitiationV08,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pain.008: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.025" => {
            let mx_struct =
                serde_json::from_value::<camt_025_001_08::ReceiptV08>(inner_json.clone()).map_err(
                    |e| XmlError::DeserializationError(format!("Failed to parse camt.025: {}", e)),
                )?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.029" => {
            let mx_struct =
                serde_json::from_value::<camt_029_001_09::ResolutionOfInvestigationV09>(
                    inner_json.clone(),
                )
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.029: {}", e))
                })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.052" => {
            let mx_struct =
                serde_json::from_value::<camt_052_001_08::BankToCustomerAccountReportV08>(
                    inner_json.clone(),
                )
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.052: {}", e))
                })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.053" => {
            let mx_struct = serde_json::from_value::<camt_053_001_08::BankToCustomerStatementV08>(
                inner_json.clone(),
            )
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.053: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.054" => {
            let mx_struct = serde_json::from_value::<
                camt_054_001_08::BankToCustomerDebitCreditNotificationV08,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.054: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.056" => {
            let mx_struct = serde_json::from_value::<
                camt_056_001_08::FIToFIPaymentCancellationRequestV08,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.056: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.057" => {
            let mx_struct = serde_json::from_value::<camt_057_001_06::NotificationToReceiveV06>(
                inner_json.clone(),
            )
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.057: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.060" => {
            let mx_struct = serde_json::from_value::<camt_060_001_05::AccountReportingRequestV05>(
                inner_json.clone(),
            )
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.060: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.107" => {
            let mx_struct = serde_json::from_value::<
                camt_107_001_01::ChequePresentmentNotificationV01,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.107: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.108" => {
            let mx_struct = serde_json::from_value::<
                camt_108_001_01::ChequeCancellationOrStopRequestV01,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.108: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.109" => {
            let mx_struct = serde_json::from_value::<
                camt_109_001_01::ChequeCancellationOrStopReportV01,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.109: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "admi.024" => {
            let mx_struct = serde_json::from_value::<
                admi_024_001_01::NotificationOfCorrespondenceV01,
            >(inner_json.clone())
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse admi.024: {}", e))
            })?;
            xml_to_string(&mx_struct).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        _ => {
            return Err(XmlError::SerializationError(format!(
                "Unsupported message type: {}",
                message_type
            )));
        }
    };

    // Wrap in Document element and add XML declaration
    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Document>\n{}</Document>\n",
        xml.trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
            .trim()
    ))
}

/// Deserialize XML string directly to typed struct for validation (Document only)
/// This validates the XML structure and content by attempting deserialization
pub fn from_mx_xml_str(xml: &str, message_type: &str) -> Result<(), XmlError> {
    use crate::document::*;

    // Extract the inner XML content (strip Document wrapper and XML declaration)
    let inner_xml = xml
        .trim()
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim()
        .trim_start_matches("<Document>")
        .trim_end_matches("</Document>")
        .trim();

    // Try to deserialize to the appropriate typed struct
    match message_type {
        "pacs.008" => {
            xml_from_str::<pacs_008_001_08::FIToFICustomerCreditTransferV08>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse pacs.008: {}", e)),
            )?;
        }
        "pacs.002" => {
            xml_from_str::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse pacs.002: {}", e)),
            )?;
        }
        "pacs.009" => {
            xml_from_str::<pacs_009_001_08::FinancialInstitutionCreditTransferV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.009: {}", e))
                })?;
        }
        "pacs.004" => {
            xml_from_str::<pacs_004_001_09::PaymentReturnV09>(inner_xml).map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.004: {}", e))
            })?;
        }
        "pacs.003" => {
            xml_from_str::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse pacs.003: {}", e)),
            )?;
        }
        "pacs.010" => {
            xml_from_str::<pacs_010_001_03::FinancialInstitutionDirectDebitV03>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.010: {}", e))
                })?;
        }
        "camt.052" => {
            xml_from_str::<camt_052_001_08::BankToCustomerAccountReportV08>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse camt.052: {}", e)),
            )?;
        }
        "camt.053" => {
            xml_from_str::<camt_053_001_08::BankToCustomerStatementV08>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse camt.053: {}", e)),
            )?;
        }
        "camt.054" => {
            xml_from_str::<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.054: {}", e))
                })?;
        }
        "camt.060" => {
            xml_from_str::<camt_060_001_05::AccountReportingRequestV05>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse camt.060: {}", e)),
            )?;
        }
        "camt.029" => {
            xml_from_str::<camt_029_001_09::ResolutionOfInvestigationV09>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse camt.029: {}", e)),
            )?;
        }
        "camt.025" => {
            xml_from_str::<camt_025_001_08::ReceiptV08>(inner_xml).map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.025: {}", e))
            })?;
        }
        "camt.056" => {
            xml_from_str::<camt_056_001_08::FIToFIPaymentCancellationRequestV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.056: {}", e))
                })?;
        }
        "camt.057" => {
            xml_from_str::<camt_057_001_06::NotificationToReceiveV06>(inner_xml).map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.057: {}", e))
            })?;
        }
        "camt.107" => {
            xml_from_str::<camt_107_001_01::ChequePresentmentNotificationV01>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse camt.107: {}", e)),
            )?;
        }
        "camt.108" => {
            xml_from_str::<camt_108_001_01::ChequeCancellationOrStopRequestV01>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.108: {}", e))
                })?;
        }
        "camt.109" => {
            xml_from_str::<camt_109_001_01::ChequeCancellationOrStopReportV01>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse camt.109: {}", e)),
            )?;
        }
        "pain.001" => {
            xml_from_str::<pain_001_001_09::CustomerCreditTransferInitiationV09>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pain.001: {}", e))
                })?;
        }
        "pain.002" => {
            xml_from_str::<pain_002_001_10::CustomerPaymentStatusReportV10>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse pain.002: {}", e)),
            )?;
        }
        "pain.008" => {
            xml_from_str::<pain_008_001_08::CustomerDirectDebitInitiationV08>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse pain.008: {}", e)),
            )?;
        }
        "admi.024" => {
            xml_from_str::<admi_024_001_01::NotificationOfCorrespondenceV01>(inner_xml).map_err(
                |e| XmlError::DeserializationError(format!("Failed to parse admi.024: {}", e)),
            )?;
        }
        _ => {
            return Err(XmlError::DeserializationError(format!(
                "Unsupported message type for validation: {}",
                message_type
            )));
        }
    }

    Ok(())
}

/// Deserialize complete MX XML envelope (with AppHdr) to typed structs for validation
/// This validates both the header and document structure
pub fn from_mx_xml_envelope_str(xml: &str, message_type: &str) -> Result<(), XmlError> {
    use crate::document::*;
    use crate::header::bah_pacs_008_001_08::BusinessApplicationHeaderV02;

    // Check if XML contains AppHdr (full envelope) or just Document
    let has_envelope = xml.contains("<AppHdr") || xml.contains("<Envelope");

    if !has_envelope {
        // No envelope, just validate the Document part
        return from_mx_xml_str(xml, message_type);
    }

    // Parse as full envelope with AppHdr
    match message_type {
        "pacs.008" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pacs_008_001_08::FIToFICustomerCreditTransferV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.008 envelope: {}", e))
            })?;
        }
        "pacs.002" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pacs_002_001_10::FIToFIPaymentStatusReportV10,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.002 envelope: {}", e))
            })?;
        }
        "pacs.009" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pacs_009_001_08::FinancialInstitutionCreditTransferV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.009 envelope: {}", e))
            })?;
        }
        "pacs.004" => {
            from_mx_xml::<BusinessApplicationHeaderV02, pacs_004_001_09::PaymentReturnV09>(xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!(
                        "Failed to parse pacs.004 envelope: {}",
                        e
                    ))
                })?;
        }
        "pacs.003" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pacs_003_001_08::FIToFICustomerDirectDebitV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.003 envelope: {}", e))
            })?;
        }
        "pacs.010" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pacs_010_001_03::FinancialInstitutionDirectDebitV03,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.010 envelope: {}", e))
            })?;
        }
        "camt.052" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_052_001_08::BankToCustomerAccountReportV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.052 envelope: {}", e))
            })?;
        }
        "camt.053" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_053_001_08::BankToCustomerStatementV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!(
                    "Failed to parse camt.053 envelope: {}",
                    e
                ))
            })?;
        }
        "camt.054" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_054_001_08::BankToCustomerDebitCreditNotificationV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.054 envelope: {}", e))
            })?;
        }
        "camt.060" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_060_001_05::AccountReportingRequestV05,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!(
                    "Failed to parse camt.060 envelope: {}",
                    e
                ))
            })?;
        }
        "camt.029" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_029_001_09::ResolutionOfInvestigationV09,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.029 envelope: {}", e))
            })?;
        }
        "camt.025" => {
            from_mx_xml::<BusinessApplicationHeaderV02, camt_025_001_08::ReceiptV08>(xml).map_err(
                |e| {
                    XmlError::DeserializationError(format!(
                        "Failed to parse camt.025 envelope: {}",
                        e
                    ))
                },
            )?;
        }
        "camt.056" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_056_001_08::FIToFIPaymentCancellationRequestV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.056 envelope: {}", e))
            })?;
        }
        "camt.057" => {
            from_mx_xml::<BusinessApplicationHeaderV02, camt_057_001_06::NotificationToReceiveV06>(
                xml,
            )
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.057 envelope: {}", e))
            })?;
        }
        "camt.107" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_107_001_01::ChequePresentmentNotificationV01,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.107 envelope: {}", e))
            })?;
        }
        "camt.108" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_108_001_01::ChequeCancellationOrStopRequestV01,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.108 envelope: {}", e))
            })?;
        }
        "camt.109" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                camt_109_001_01::ChequeCancellationOrStopReportV01,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.109 envelope: {}", e))
            })?;
        }
        "pain.001" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pain_001_001_09::CustomerCreditTransferInitiationV09,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pain.001 envelope: {}", e))
            })?;
        }
        "pain.002" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pain_002_001_10::CustomerPaymentStatusReportV10,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pain.002 envelope: {}", e))
            })?;
        }
        "pain.008" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                pain_008_001_08::CustomerDirectDebitInitiationV08,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pain.008 envelope: {}", e))
            })?;
        }
        "admi.024" => {
            from_mx_xml::<
                BusinessApplicationHeaderV02,
                admi_024_001_01::NotificationOfCorrespondenceV01,
            >(xml)
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse admi.024 envelope: {}", e))
            })?;
        }
        _ => {
            return Err(XmlError::DeserializationError(format!(
                "Unsupported message type for envelope validation: {}",
                message_type
            )));
        }
    }

    Ok(())
}

/// Deprecated: Use json_to_typed_xml instead
/// This function is kept for backward compatibility but now delegates to json_to_typed_xml
#[deprecated(
    since = "3.2.0",
    note = "Use json_to_typed_xml instead for correct array and attribute handling"
)]
pub fn to_mx_xml_with_config(
    json_data: &serde_json::Value,
    _config: &XmlConfig,
) -> Result<String, XmlError> {
    // Try to extract message type from the JSON data
    let message_type = json_data
        .get("message_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            XmlError::SerializationError("Missing message_type field in JSON data".to_string())
        })?;

    json_to_typed_xml(json_data, message_type)
}

/// Helper function to get the Document element name for a message type
fn get_document_element_name(message_type: &str) -> &'static str {
    match message_type {
        "pacs.008" => "FIToFICstmrCdtTrf",
        "pacs.009" => "FIToFICstmrCdtTrf",
        "pacs.003" => "FIToFICstmrDrctDbt",
        "pacs.004" => "PmtRtr",
        "pacs.010" => "FIDrctDbt",
        "pacs.002" => "FIToFIPmtStsRpt",
        "pain.001" => "CstmrCdtTrfInitn",
        "pain.008" => "CstmrDrctDbtInitn",
        "camt.025" => "Rct",
        "camt.029" => "RsltnOfInvstgtn",
        "camt.052" => "BkToCstmrAcctRpt",
        "camt.053" => "BkToCstmrStmt",
        "camt.054" => "BkToCstmrDbtCdtNtfctn",
        "camt.056" => "FIToFIPmtCxlReq",
        "camt.057" => "NtfctnToRcv",
        "camt.060" => "AcctRptgReq",
        "camt.107" => "ChqPresntmntNtfctn",
        "camt.108" => "ChqCxlOrStopReq",
        "camt.109" => "ChqCxlOrStopRpt",
        "admi.024" => "NtfctnOfCrspdc",
        _ => "Unknown",
    }
}

/// Parse XML to JSON using typed ISO20022 document structures
/// This is the correct way to parse XML that preserves arrays and complex structures
pub fn xml_to_json_via_document(
    xml: &str,
    message_type: &str,
) -> Result<serde_json::Value, XmlError> {
    use crate::document::*;
    use serde_json::json;

    // Extract inner XML (strip Document wrapper and XML declaration)
    let inner_xml = xml
        .trim()
        .trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
        .trim();

    // Find the Document wrapper and extract inner content
    let inner_xml = if let Some(start_idx) = inner_xml.find("<Document") {
        if let Some(end_bracket) = inner_xml[start_idx..].find('>') {
            let content_start = start_idx + end_bracket + 1;
            if let Some(end_idx) = inner_xml.rfind("</Document>") {
                &inner_xml[content_start..end_idx]
            } else {
                inner_xml
            }
        } else {
            inner_xml
        }
    } else {
        inner_xml
    };

    let inner_xml = inner_xml.trim();

    // Deserialize XML to typed struct, then serialize to JSON
    let json_value = match message_type {
        "pacs.008" => {
            let doc = xml_from_str::<pacs_008_001_08::FIToFICustomerCreditTransferV08>(inner_xml)
                .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse pacs.008: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.009" => {
            let doc =
                xml_from_str::<pacs_009_001_08::FinancialInstitutionCreditTransferV08>(inner_xml)
                    .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.009: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.003" => {
            let doc = xml_from_str::<pacs_003_001_08::FIToFICustomerDirectDebitV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.003: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.004" => {
            let doc =
                xml_from_str::<pacs_004_001_09::PaymentReturnV09>(inner_xml).map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.004: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.010" => {
            let doc =
                xml_from_str::<pacs_010_001_03::FinancialInstitutionDirectDebitV03>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse pacs.010: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pacs.002" => {
            let doc = xml_from_str::<pacs_002_001_10::FIToFIPaymentStatusReportV10>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pacs.002: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pain.001" => {
            let doc =
                xml_from_str::<pain_001_001_09::CustomerCreditTransferInitiationV09>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse pain.001: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "pain.008" => {
            let doc = xml_from_str::<pain_008_001_08::CustomerDirectDebitInitiationV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse pain.008: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.025" => {
            let doc = xml_from_str::<camt_025_001_08::ReceiptV08>(inner_xml).map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.025: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.029" => {
            let doc = xml_from_str::<camt_029_001_09::ResolutionOfInvestigationV09>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.029: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.052" => {
            let doc = xml_from_str::<camt_052_001_08::BankToCustomerAccountReportV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.052: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.053" => {
            let doc = xml_from_str::<camt_053_001_08::BankToCustomerStatementV08>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.053: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.054" => {
            let doc = xml_from_str::<camt_054_001_08::BankToCustomerDebitCreditNotificationV08>(
                inner_xml,
            )
            .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse camt.054: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.056" => {
            let doc =
                xml_from_str::<camt_056_001_08::FIToFIPaymentCancellationRequestV08>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse camt.056: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.057" => {
            let doc = xml_from_str::<camt_057_001_06::NotificationToReceiveV06>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.057: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.060" => {
            let doc = xml_from_str::<camt_060_001_05::AccountReportingRequestV05>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.060: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.107" => {
            let doc = xml_from_str::<camt_107_001_01::ChequePresentmentNotificationV01>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.107: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.108" => {
            let doc =
                xml_from_str::<camt_108_001_01::ChequeCancellationOrStopRequestV01>(inner_xml)
                    .map_err(|e| {
                        XmlError::DeserializationError(format!("Failed to parse camt.108: {}", e))
                    })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "camt.109" => {
            let doc = xml_from_str::<camt_109_001_01::ChequeCancellationOrStopReportV01>(inner_xml)
                .map_err(|e| {
                    XmlError::DeserializationError(format!("Failed to parse camt.109: {}", e))
                })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        "admi.024" => {
            let doc = xml_from_str::<admi_024_001_01::NotificationOfCorrespondenceV01>(inner_xml)
                .map_err(|e| {
                XmlError::DeserializationError(format!("Failed to parse admi.024: {}", e))
            })?;
            serde_json::to_value(&doc).map_err(|e| XmlError::SerializationError(e.to_string()))?
        }
        _ => {
            return Err(XmlError::DeserializationError(format!(
                "Unsupported message type: {}",
                message_type
            )));
        }
    };

    // Wrap in Document structure with correct element name
    let element_name = get_document_element_name(message_type);
    Ok(json!({
        "Document": {
            element_name: json_value
        }
    }))
}

/// Helper function to parse XML to JSON Value (for dataflow plugins)
/// This function parses XML and converts it to a JSON structure
///
/// Note: This is a simplified XML parser for plugin use.
/// It converts XML elements to JSON objects preserving the structure.
///
/// DEPRECATED: Use xml_to_json_via_document() instead, which uses typed structs
/// and correctly handles arrays and complex structures.
pub fn from_mx_xml_to_json(xml: &str) -> Result<serde_json::Value, XmlError> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<(String, serde_json::Map<String, serde_json::Value>)> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut map = serde_json::Map::new();

                // Process attributes
                for attr in e.attributes().flatten() {
                    let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                    let value = String::from_utf8_lossy(&attr.value).to_string();
                    map.insert(key, serde_json::Value::String(value));
                }

                stack.push((name, map));
            }
            Ok(Event::Text(e)) => {
                if let Some((_, map)) = stack.last_mut() {
                    let text_bytes = e.as_ref();
                    let text_str = String::from_utf8_lossy(text_bytes).trim().to_string();
                    if !text_str.is_empty() {
                        map.insert("$value".to_string(), serde_json::Value::String(text_str));
                    }
                }
            }
            Ok(Event::End(_)) => {
                if let Some((name, map)) = stack.pop() {
                    let value = if map.len() == 1 && map.contains_key("$value") {
                        // Just text content, unwrap it
                        map.get("$value").unwrap().clone()
                    } else {
                        serde_json::Value::Object(map)
                    };

                    if let Some((_, parent_map)) = stack.last_mut() {
                        parent_map.insert(name, value);
                    } else {
                        // Root element
                        let mut root = serde_json::Map::new();
                        root.insert(name, value);
                        return Ok(serde_json::Value::Object(root));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => {
                return Err(XmlError::DeserializationError(format!(
                    "XML parsing error: {}",
                    e
                )));
            }
        }
        buf.clear();
    }

    Ok(serde_json::Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_lookup() {
        assert_eq!(
            get_namespace_for_message_type("pacs.008"),
            "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"
        );
        assert_eq!(
            get_namespace_for_message_type("pain.001"),
            "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09"
        );
        assert_eq!(
            get_namespace_for_message_type("camt.053"),
            "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08"
        );
    }

    #[test]
    fn test_xml_config_default() {
        let config = XmlConfig::default();
        assert!(config.include_declaration);
        assert!(config.pretty_print);
        assert_eq!(config.indent, "  ");
        assert!(!config.include_schema_location);
    }
}
