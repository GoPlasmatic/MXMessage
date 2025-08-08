// MX Message Envelope Structure for ISO 20022 compliant XML generation

use serde::{Deserialize, Serialize};

/// Complete MX message envelope containing Business Application Header and Document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Envelope")]
pub struct MxEnvelope<H, D> {
    /// XML namespace declarations
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,

    #[serde(rename = "@xmlns:xsi", skip_serializing_if = "Option::is_none")]
    pub xmlns_xsi: Option<String>,

    /// Business Application Header
    #[serde(rename = "AppHdr")]
    pub app_hdr: H,

    /// Document containing the actual message
    #[serde(rename = "Document")]
    pub document: MxDocument<D>,
}

/// Document wrapper for MX messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MxDocument<D> {
    /// Document namespace
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,

    /// The actual message content
    #[serde(flatten)]
    pub message: D,
}

impl<H, D> MxEnvelope<H, D> {
    /// Create a new MX envelope with default namespaces
    pub fn new(app_hdr: H, document: D, document_namespace: String) -> Self {
        Self {
            xmlns: Some("urn:iso:std:iso:20022:tech:xsd:head.001.001.02".to_string()),
            xmlns_xsi: Some("http://www.w3.org/2001/XMLSchema-instance".to_string()),
            app_hdr,
            document: MxDocument {
                xmlns: Some(document_namespace),
                message: document,
            },
        }
    }

    /// Create envelope for pacs.008 message
    pub fn new_pacs008(app_hdr: H, document: D) -> Self {
        Self::new(
            app_hdr,
            document,
            "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08".to_string(),
        )
    }

    /// Create envelope for pacs.009 message
    pub fn new_pacs009(app_hdr: H, document: D) -> Self {
        Self::new(
            app_hdr,
            document,
            "urn:iso:std:iso:20022:tech:xsd:pacs.009.001.08".to_string(),
        )
    }

    /// Create envelope for pain.001 message
    pub fn new_pain001(app_hdr: H, document: D) -> Self {
        Self::new(
            app_hdr,
            document,
            "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09".to_string(),
        )
    }

    /// Create envelope for camt.052 message
    pub fn new_camt052(app_hdr: H, document: D) -> Self {
        Self::new(
            app_hdr,
            document,
            "urn:iso:std:iso:20022:tech:xsd:camt.052.001.08".to_string(),
        )
    }

    /// Create envelope for camt.053 message
    pub fn new_camt053(app_hdr: H, document: D) -> Self {
        Self::new(
            app_hdr,
            document,
            "urn:iso:std:iso:20022:tech:xsd:camt.053.001.08".to_string(),
        )
    }
}
