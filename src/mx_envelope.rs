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

/// ISO 20022 Business Application Header V02
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessApplicationHeaderV02 {
    /// Sender of the message
    pub fr: Party44Choice,

    /// Receiver of the message
    pub to: Party44Choice,

    /// Business message identifier
    pub biz_msg_idr: String,

    /// Message definition identifier (e.g., pacs.008.001.08)
    pub msg_def_idr: String,

    /// Business service (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<String>,

    /// Creation date time
    pub cre_dt: String,

    /// Copy duplicate indicator (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_dplct: Option<CopyDuplicate1Code>,

    /// Possible duplicate flag (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<bool>,

    /// Priority (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prty: Option<String>,

    /// Signature (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<SignatureEnvelope>,
}

/// Choice between different party identification methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Party44Choice {
    /// Organization identification
    OrgId {
        #[serde(rename = "OrgId")]
        org_id: PartyIdentification135,
    },

    /// Financial institution identification
    FIId {
        #[serde(rename = "FIId")]
        fi_id: BranchAndFinancialInstitutionIdentification6,
    },
}

/// Party identification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PartyIdentification135 {
    /// Name of the party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,

    /// Postal address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,

    /// Identification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Party38Choice>,

    /// Country of residence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<String>,

    /// Contact details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
}

/// Financial institution identification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BranchAndFinancialInstitutionIdentification6 {
    /// Financial institution identification
    pub fin_instn_id: FinancialInstitutionIdentification18,

    /// Branch identification (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData3>,
}

/// Financial institution identification details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FinancialInstitutionIdentification18 {
    /// BICFI code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<String>,

    /// Clearing system member identification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,

    /// LEI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,

    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<String>,

    /// Postal address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,

    /// Other identification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}

/// Postal address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PostalAddress24 {
    /// Address type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType3Choice>,

    /// Department
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<String>,

    /// Sub department
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<String>,

    /// Street name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<String>,

    /// Building number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<String>,

    /// Building name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<String>,

    /// Floor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flr: Option<String>,

    /// Post box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<String>,

    /// Room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,

    /// Post code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<String>,

    /// Town name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<String>,

    /// Town location name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<String>,

    /// District name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<String>,

    /// Country sub division
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<String>,

    /// Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<String>,

    /// Address line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr_line: Option<Vec<String>>,
}

/// Copy duplicate code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CopyDuplicate1Code {
    #[serde(rename = "CODU")]
    CODU,
    #[serde(rename = "COPY")]
    COPY,
    #[serde(rename = "DUPL")]
    DUPL,
}

// Placeholder types for completeness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureEnvelope;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Party38Choice;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact4;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BranchData3;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClearingSystemMemberIdentification2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericFinancialIdentification1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddressType3Choice;

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

/// Builder for creating Business Application Headers
pub struct BusinessApplicationHeaderBuilder {
    header: BusinessApplicationHeaderV02,
}

impl BusinessApplicationHeaderBuilder {
    pub fn new(msg_def_idr: String) -> Self {
        Self {
            header: BusinessApplicationHeaderV02 {
                fr: Party44Choice::FIId {
                    fi_id: BranchAndFinancialInstitutionIdentification6 {
                        fin_instn_id: FinancialInstitutionIdentification18 {
                            bicfi: None,
                            clr_sys_mmb_id: None,
                            lei: None,
                            nm: None,
                            pstl_adr: None,
                            othr: None,
                        },
                        brnch_id: None,
                    },
                },
                to: Party44Choice::FIId {
                    fi_id: BranchAndFinancialInstitutionIdentification6 {
                        fin_instn_id: FinancialInstitutionIdentification18 {
                            bicfi: None,
                            clr_sys_mmb_id: None,
                            lei: None,
                            nm: None,
                            pstl_adr: None,
                            othr: None,
                        },
                        brnch_id: None,
                    },
                },
                biz_msg_idr: String::new(),
                msg_def_idr,
                biz_svc: None,
                cre_dt: chrono::Utc::now()
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    .to_string(),
                copy_dplct: None,
                pssbl_dplct: None,
                prty: None,
                sgntr: None,
            },
        }
    }

    pub fn from_bicfi(mut self, bicfi: String) -> Self {
        if let Party44Choice::FIId { ref mut fi_id } = self.header.fr {
            fi_id.fin_instn_id.bicfi = Some(bicfi);
        }
        self
    }

    pub fn to_bicfi(mut self, bicfi: String) -> Self {
        if let Party44Choice::FIId { ref mut fi_id } = self.header.to {
            fi_id.fin_instn_id.bicfi = Some(bicfi);
        }
        self
    }

    pub fn business_message_identifier(mut self, id: String) -> Self {
        self.header.biz_msg_idr = id;
        self
    }

    pub fn business_service(mut self, service: String) -> Self {
        self.header.biz_svc = Some(service);
        self
    }

    pub fn priority(mut self, priority: String) -> Self {
        self.header.prty = Some(priority);
        self
    }

    pub fn build(self) -> BusinessApplicationHeaderV02 {
        self.header
    }
}
