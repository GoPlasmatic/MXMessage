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


use regex::Regex;
use crate::error::*;
use serde::{Deserialize, Serialize};

// document ...
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct document {
	#[serde(rename = "Document")]
	pub document: Document,
}

impl document {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.document.validate()?;
		Ok(())
	}
}


// AccountIdentification4Choice1: Unique identification of an account, as assigned by the account servicer, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountIdentification4Choice1 {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification11>,
}

impl AccountIdentification4Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.iban {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// AccountSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountSchemeName1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl AccountSchemeName1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount: A number of monetary units specified in an active or a historic currency where the unit of currency is explicit and compliant with ISO 4217.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "@Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification61: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification61 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification181,
}

impl BranchAndFinancialInstitutionIdentification61 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instn_id.validate()?;
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification62: Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification62 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification182,
}

impl BranchAndFinancialInstitutionIdentification62 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instn_id.validate()?;
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification63: Identifies a specific branch of a financial institution.
// 
// Usage: This component should be used in case the identification information in the financial institution component does not provide identification up to branch level.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification63 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification181,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData31>,
}

impl BranchAndFinancialInstitutionIdentification63 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instn_id.validate()?;
		if let Some(ref val) = self.brnch_id { val.validate()? }
		Ok(())
	}
}


// BranchData31: Unique and unambiguous identification of a branch of a financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct BranchData31 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

impl BranchData31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CBPRAmount1: CBPR_Amount__1
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CBPRAmount1 {
	#[serde(rename = "@Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl CBPRAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CashAccount381: Specifies an alternate assumed name for the identification of the account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccount381 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice1,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice1>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification11>,
}

impl CashAccount381 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prxy { val.validate()? }
		Ok(())
	}
}


// CashAccountType2Choice1: Nature or use of the account in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashAccountType2Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl CashAccountType2Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CategoryPurpose1Choice1: Category purpose, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CategoryPurpose1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl CategoryPurpose1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ChargeBearerType1Code__1: In a credit transfer context, means that transaction charges on the sender side are to be borne by the debtor, transaction charges on the receiver side are to be borne by the creditor. In a direct debit context, means that transaction charges on the sender side are to be borne by the creditor, transaction charges on the receiver side are to be borne by the debtor.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChargeBearerType1Code1 {
	#[default]
	#[serde(rename = "DEBT")]
	CodeDEBT,
	#[serde(rename = "CRED")]
	CodeCRED,
	#[serde(rename = "SHAR")]
	CodeSHAR,
}

impl ChargeBearerType1Code1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Charges71: Agent that takes the transaction charges or to which the transaction charges are due.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charges71 {
	#[serde(rename = "Amt")]
	pub amt: CBPRAmount1,
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification61,
}

impl Charges71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.agt.validate()?;
		Ok(())
	}
}


// ClearingChannel2Code: Payment through internal book transfer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClearingChannel2Code {
	#[default]
	#[serde(rename = "RTGS")]
	CodeRTGS,
	#[serde(rename = "RTNS")]
	CodeRTNS,
	#[serde(rename = "MPNS")]
	CodeMPNS,
	#[serde(rename = "BOOK")]
	CodeBOOK,
}

impl ClearingChannel2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingSystemIdentification2Choice1: Identification of a clearing system, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemIdentification2Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
}

impl ClearingSystemIdentification2Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 5 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 5".to_string()));
			}
		}
		Ok(())
	}
}


// ClearingSystemMemberIdentification21: Identification of a member of a clearing system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClearingSystemMemberIdentification21 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: ClearingSystemIdentification2Choice1,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}

impl ClearingSystemMemberIdentification21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.clr_sys_id.validate()?;
		if self.mmb_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mmb_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mmb_id.chars().count() > 28 {
			return Err(ValidationError::new(1002, "mmb_id exceeds the maximum length of 28".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.mmb_id) {
			return Err(ValidationError::new(1005, "mmb_id does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CreditDebitCode: Operation is a decrease.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum CreditDebitCode {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,
}

impl CreditDebitCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditTransferTransaction391: Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditTransferTransaction391 {
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification71,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation281>,
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: CBPRAmount1,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
	pub sttlm_prty: Option<Priority3Code>,
	#[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
	pub sttlm_tm_indctn: Option<SettlementDateTimeIndication11>,
	#[serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none")]
	pub sttlm_tm_req: Option<SettlementTimeRequest21>,
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<CBPRAmount1>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "ChrgBr")]
	pub chrg_br: ChargeBearerType1Code1,
	#[serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none")]
	pub chrgs_inf: Option<Vec<Charges71>>,
	#[serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt1_acct: Option<CashAccount381>,
	#[serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt2_acct: Option<CashAccount381>,
	#[serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt3_acct: Option<CashAccount381>,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification62,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: BranchAndFinancialInstitutionIdentification62,
	#[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1_acct: Option<CashAccount381>,
	#[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2_acct: Option<CashAccount381>,
	#[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3_acct: Option<CashAccount381>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<PartyIdentification1351>,
	#[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
	pub initg_pty: Option<PartyIdentification1351>,
	#[serde(rename = "Dbtr")]
	pub dbtr: PartyIdentification1352,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount381>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification61,
	#[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_acct: Option<CashAccount381>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification63,
	#[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_acct: Option<CashAccount381>,
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification1353,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount381>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<PartyIdentification1351>,
	#[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent11>>,
	#[serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent11>>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice1>,
	#[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting31>>,
	#[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
	pub rltd_rmt_inf: Option<RemittanceLocation71>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation161>,
}

impl CreditTransferTransaction391 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.intr_bk_sttlm_amt.validate()?;
		if let Some(ref val) = self.sttlm_prty { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_indctn { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_req { val.validate()? }
		if let Some(ref val) = self.instd_amt { val.validate()? }
		self.chrg_br.validate()?;
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		self.instg_agt.validate()?;
		self.instd_agt.validate()?;
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.initg_pty { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rltd_rmt_inf { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		Ok(())
	}
}


// CreditorReferenceInformation21: Unique reference, as assigned by the creditor, to unambiguously refer to the payment transaction.
// 
// Usage: If available, the initiating party should provide this reference in the structured remittance information, to enable reconciliation by the creditor upon receipt of the amount of money.
// 
// If the business context requires the use of a creditor reference or a payment remit identification, and only one identifier can be passed through the end-to-end chain, the creditor's reference or payment remittance identification should be quoted in the end-to-end transaction identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceInformation21 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType21>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ref_attr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ref_attr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType1Choice1: Creditor reference type, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DocumentType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl CreditorReferenceType1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType21: Entity that assigns the credit reference type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreditorReferenceType21 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice1,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl CreditorReferenceType21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DateAndPlaceOfBirth11: Country where a person was born.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DateAndPlaceOfBirth11 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prvc_of_birth does not match the required pattern".to_string()));
			}
		}
		if self.city_of_birth.chars().count() < 1 {
			return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
		}
		if self.city_of_birth.chars().count() > 35 {
			return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
		if !pattern.is_match(&self.city_of_birth) {
			return Err(ValidationError::new(1005, "city_of_birth does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_birth) {
			return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DatePeriod2: End date of the range.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DiscountAmountAndType11: Amount of money, which has been typed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountAndType11 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DiscountAmountType1Choice1>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DiscountAmountAndType11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// DiscountAmountType1Choice1: Specifies the amount type, in a free-text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscountAmountType1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl DiscountAmountType1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentAdjustment11: Provides further details on the document adjustment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentAdjustment11 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<String>,
}

impl DocumentAdjustment11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "rsn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "addtl_inf does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineIdentification11: Date associated with the referred document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineIdentification11 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentLineType11>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineInformation11: Provides details on the amounts of the document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineInformation11 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification11>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<RemittanceAmount31>,
}

impl DocumentLineInformation11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "desc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// DocumentLineType1Choice1: Proprietary identification of the type of the remittance document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl DocumentLineType1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineType11: Identification of the issuer of the reference document line identificationtype.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocumentLineType11 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice1,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl DocumentLineType11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentType3Code: Document is a structured communication reference provided by the creditor to identify the referred transaction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum DocumentType3Code {
	#[default]
	#[serde(rename = "RADM")]
	CodeRADM,
	#[serde(rename = "RPIN")]
	CodeRPIN,
	#[serde(rename = "FXDR")]
	CodeFXDR,
	#[serde(rename = "DISP")]
	CodeDISP,
	#[serde(rename = "PUOR")]
	CodePUOR,
	#[serde(rename = "SCOR")]
	CodeSCOR,
}

impl DocumentType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DocumentType6Code: Document is a purchase order.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum DocumentType6Code {
	#[default]
	#[serde(rename = "MSIN")]
	CodeMSIN,
	#[serde(rename = "CNFA")]
	CodeCNFA,
	#[serde(rename = "DNFA")]
	CodeDNFA,
	#[serde(rename = "CINV")]
	CodeCINV,
	#[serde(rename = "CREN")]
	CodeCREN,
	#[serde(rename = "DEBN")]
	CodeDEBN,
	#[serde(rename = "HIRI")]
	CodeHIRI,
	#[serde(rename = "SBIN")]
	CodeSBIN,
	#[serde(rename = "CMCN")]
	CodeCMCN,
	#[serde(rename = "SOAC")]
	CodeSOAC,
	#[serde(rename = "DISP")]
	CodeDISP,
	#[serde(rename = "BOLD")]
	CodeBOLD,
	#[serde(rename = "VCHR")]
	CodeVCHR,
	#[serde(rename = "AROI")]
	CodeAROI,
	#[serde(rename = "TSUT")]
	CodeTSUT,
	#[serde(rename = "PUOR")]
	CodePUOR,
}

impl DocumentType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FIToFICustomerCreditTransferV08: Set of elements providing information specific to the individual credit transfer(s).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FIToFICustomerCreditTransferV08 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader931,
	#[serde(rename = "CdtTrfTxInf")]
	pub cdt_trf_tx_inf: CreditTransferTransaction391,
}

impl FIToFICustomerCreditTransferV08 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.grp_hdr.validate()?;
		self.cdt_trf_tx_inf.validate()?;
		Ok(())
	}
}


// FinancialInstitutionIdentification181: Information that locates and identifies a specific address, as defined by postal services.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification181 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress241>,
}

impl FinancialInstitutionIdentification181 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_mmb_id { val.validate()? }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// FinancialInstitutionIdentification182: Legal entity identifier of the financial institution.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct FinancialInstitutionIdentification182 {
	#[serde(rename = "BICFI")]
	pub bicfi: String,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification21>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
}

impl FinancialInstitutionIdentification182 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.bicfi) {
			return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.clr_sys_mmb_id { val.validate()? }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Garnishment31: Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Garnishment31 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType11,
	#[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
	pub grnshee: Option<PartyIdentification1354>,
	#[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
	pub grnshmt_admstr: Option<PartyIdentification1354>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
	pub mplyee_termntn_ind: Option<bool>,
}

impl Garnishment31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.grnshee { val.validate()? }
		if let Some(ref val) = self.grnshmt_admstr { val.validate()? }
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ref_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// GarnishmentType1Choice1: Proprietary identification of the type of garnishment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl GarnishmentType1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GarnishmentType11: Identification of the issuer of the garnishment type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GarnishmentType11 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice1,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GarnishmentType11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericAccountIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericAccountIdentification11 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice1>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericAccountIdentification11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 34 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 34".to_string()));
		}
		let pattern = Regex::new("([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]([0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ]*(/[0-9a-zA-Z\\-\\?:\\(\\)\\.,'\\+ ])?)*)").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericOrganisationIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification11 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice1>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericOrganisationIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification12 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: OrganisationIdentificationSchemeName1Choice2,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		self.schme_nm.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericOrganisationIdentification13: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericOrganisationIdentification13 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice3>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericPersonIdentification11: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification11 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice1>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericPersonIdentification11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericPersonIdentification12: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification12 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: PersonIdentificationSchemeName1Choice2,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericPersonIdentification12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		self.schme_nm.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GenericPersonIdentification13: Entity that assigns the identification.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GenericPersonIdentification13 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice3>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl GenericPersonIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GroupHeader931: Specifies the details on how the settlement of the transaction(s) between the instructing agent and the instructed agent is completed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupHeader931 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: Max15NumericTextfixed,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction71,
}

impl GroupHeader931 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.msg_id) {
			return Err(ValidationError::new(1005, "msg_id does not match the required pattern".to_string()));
		}
		let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
		if !pattern.is_match(&self.cre_dt_tm) {
			return Err(ValidationError::new(1005, "cre_dt_tm does not match the required pattern".to_string()));
		}
		self.nb_of_txs.validate()?;
		self.sttlm_inf.validate()?;
		Ok(())
	}
}


// Instruction3Code: Please advise/contact (ultimate) creditor/claimant by the most efficient means of telecommunication.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Instruction3Code {
	#[default]
	#[serde(rename = "CHQB")]
	CodeCHQB,
	#[serde(rename = "HOLD")]
	CodeHOLD,
	#[serde(rename = "PHOB")]
	CodePHOB,
	#[serde(rename = "TELB")]
	CodeTELB,
}

impl Instruction3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InstructionForCreditorAgent11: Further information complementing the coded instruction or instruction to the creditor's agent that is bilaterally agreed or specific to a user community.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstructionForCreditorAgent11 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Instruction3Code>,
	#[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
	pub instr_inf: Option<String>,
}

impl InstructionForCreditorAgent11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "instr_inf does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// InstructionForNextAgent11: Further information complementing the coded instruction or instruction to the next agent that is bilaterally agreed or specific to a user community.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct InstructionForNextAgent11 {
	#[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
	pub instr_inf: Option<String>,
}

impl InstructionForNextAgent11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "instr_inf does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// LocalInstrument2Choice1: Specifies the local instrument, as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalInstrument2Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl LocalInstrument2Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Max15NumericText_fixed: 1
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Max15NumericTextfixed {
	#[default]
	#[serde(rename = "1")]
	Code1,
}

impl Max15NumericTextfixed {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NameAndAddress161: Postal address of a party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct NameAndAddress161 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress241,
}

impl NameAndAddress161 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
		if !pattern.is_match(&self.nm) {
			return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
		}
		self.adr.validate()?;
		Ok(())
	}
}


// OrganisationIdentification291: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification291 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification11>>,
}

impl OrganisationIdentification291 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationIdentification292: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification292 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification12>>,
}

impl OrganisationIdentification292 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationIdentification293: Unique identification of an organisation, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentification293 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification13>>,
}

impl OrganisationIdentification293 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice2: Name of the identification scheme, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice2 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice3: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice3 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Party38Choice1: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice1 {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification291>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification131>,
}

impl Party38Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// Party38Choice2: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice2 {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification292>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification132>,
}

impl Party38Choice2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// Party38Choice3: Unique and unambiguous identification of a person, for example a passport.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Party38Choice3 {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification293>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification133>,
}

impl Party38Choice3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// PartyIdentification1351: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1351 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress242>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice1>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
}

impl PartyIdentification1351 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification1352: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1352 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress241>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice2>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
}

impl PartyIdentification1352 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification1353: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1353 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress241>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice1>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
}

impl PartyIdentification1353 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification1354: Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartyIdentification1354 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress242>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice3>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
}

impl PartyIdentification1354 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentIdentification71: Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentIdentification71 {
	#[serde(rename = "InstrId")]
	pub instr_id: String,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: String,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR")]
	pub uetr: String,
	#[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
	pub clr_sys_ref: Option<String>,
}

impl PaymentIdentification71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.instr_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
		}
		if self.instr_id.chars().count() > 16 {
			return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 16".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.instr_id) {
			return Err(ValidationError::new(1005, "instr_id does not match the required pattern".to_string()));
		}
		if self.end_to_end_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
		}
		if self.end_to_end_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
		if !pattern.is_match(&self.end_to_end_id) {
			return Err(ValidationError::new(1005, "end_to_end_id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tx_id does not match the required pattern".to_string()));
			}
		}
		let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
		if !pattern.is_match(&self.uetr) {
			return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "clr_sys_ref does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentTypeInformation281: Specifies the high level purpose of the instruction based on a set of pre-defined categories.
// Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaymentTypeInformation281 {
	#[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
	pub instr_prty: Option<Priority2Code>,
	#[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice1>>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice1>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice1>,
}

impl PaymentTypeInformation281 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.clr_chanl { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PersonIdentification131: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification131 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification11>>,
}

impl PersonIdentification131 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentification132: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification132 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification12>>,
}

impl PersonIdentification132 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentification133: Unique identification of a person, as assigned by an institution, using an identification scheme.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentification133 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth11>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification13>>,
}

impl PersonIdentification133 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice2: Name of the identification scheme, in a coded form as published in an external list.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice2 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
}

impl PersonIdentificationSchemeName1Choice2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice3: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice3 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PostalAddress241: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress241 {
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<String>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<String>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<String>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
}

impl PostalAddress241 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dept does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sub_dept does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "strt_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bldg_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bldg_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "flr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pst_bx does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "room does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pst_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "twn_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "twn_lctn_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dstrct_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_sub_dvsn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
				let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "adr_line does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PostalAddress242: Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostalAddress242 {
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<String>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<String>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: String,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<String>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
}

impl PostalAddress242 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dept does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sub_dept does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "strt_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bldg_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bldg_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "flr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pst_bx does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "room does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pst_cd does not match the required pattern".to_string()));
			}
		}
		if self.twn_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.twn_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
		if !pattern.is_match(&self.twn_nm) {
			return Err(ValidationError::new(1005, "twn_nm does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "twn_lctn_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dstrct_nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_sub_dvsn does not match the required pattern".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
				let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "adr_line does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Priority2Code: Priority level is normal.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority2Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,
}

impl Priority2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority3Code: Priority level is normal.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority3Code {
	#[default]
	#[serde(rename = "URGT")]
	CodeURGT,
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,
}

impl Priority3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProxyAccountIdentification11: Identification used to indicate the account identification under another specified name.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountIdentification11 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice1>,
	#[serde(rename = "Id")]
	pub id: String,
}

impl ProxyAccountIdentification11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 320 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 320".to_string()));
		}
		let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ProxyAccountType1Choice1: Name of the identification scheme, in a free text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyAccountType1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl ProxyAccountType1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Purpose2Choice1: Purpose, in a proprietary form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Purpose2Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl Purpose2Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ReferredDocumentInformation71: Set of elements used to provide the content of the referred document line.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentInformation71 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ReferredDocumentType41>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
	#[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
	pub line_dtls: Option<Vec<DocumentLineInformation11>>,
}

impl ReferredDocumentInformation71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.line_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReferredDocumentType3Choice1: Proprietary identification of the type of the remittance document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType3Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DocumentType6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl ReferredDocumentType3Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ReferredDocumentType41: Identification of the issuer of the reference document type.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReferredDocumentType41 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice1,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}

impl ReferredDocumentType41 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryAuthority21: Country of the entity that requires the regulatory reporting information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryAuthority21 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
}

impl RegulatoryAuthority21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryReporting31: Set of elements used to provide details on the regulatory reporting information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegulatoryReporting31 {
	#[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
	pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
	#[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
	pub authrty: Option<RegulatoryAuthority21>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<StructuredRegulatoryReporting31>>,
}

impl RegulatoryReporting31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbt_cdt_rptg_ind { val.validate()? }
		if let Some(ref val) = self.authrty { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RegulatoryReportingType1Code: Regulatory information applies to both credit and debit sides.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum RegulatoryReportingType1Code {
	#[default]
	#[serde(rename = "CRED")]
	CodeCRED,
	#[serde(rename = "DEBT")]
	CodeDEBT,
	#[serde(rename = "BOTH")]
	CodeBOTH,
}

impl RegulatoryReportingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RemittanceAmount21: Amount of money remitted for the referred document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount21 {
	#[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType11>>,
	#[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<Vec<TaxAmountAndType11>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment11>>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.due_pybl_amt { val.validate()? }
		if let Some(ref vec) = self.dscnt_apld_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdt_note_amt { val.validate()? }
		if let Some(ref vec) = self.tax_amt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// RemittanceAmount31: Amount of money remitted.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceAmount31 {
	#[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType11>>,
	#[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<Vec<TaxAmountAndType11>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment11>>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.due_pybl_amt { val.validate()? }
		if let Some(ref vec) = self.dscnt_apld_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdt_note_amt { val.validate()? }
		if let Some(ref vec) = self.tax_amt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// RemittanceInformation161: Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in a structured form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceInformation161 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<String>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation161>>,
}

impl RemittanceInformation161 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ustrd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ustrd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ustrd exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ustrd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.strd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceLocation71: Set of elements used to provide information on the location and/or delivery of the remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceLocation71 {
	#[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
	pub rmt_id: Option<String>,
	#[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData11>>,
}

impl RemittanceLocation71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rmt_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "rmt_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rmt_lctn_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceLocationData11: Postal address to which an agent is to send the remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemittanceLocationData11 {
	#[serde(rename = "Mtd")]
	pub mtd: RemittanceLocationMethod2Code,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<NameAndAddress161>,
}

impl RemittanceLocationData11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mtd.validate()?;
		if let Some(ref val) = self.elctrnc_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "elctrnc_adr exceeds the maximum length of 2048".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "elctrnc_adr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// RemittanceLocationMethod2Code: Remittance advice information must be sent through by phone as a short message service (SMS).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum RemittanceLocationMethod2Code {
	#[default]
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "EDIC")]
	CodeEDIC,
	#[serde(rename = "URID")]
	CodeURID,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "SMSM")]
	CodeSMSM,
}

impl RemittanceLocationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ServiceLevel8Choice1: Specifies a pre-agreed service or level of service between the parties, as a proprietary code.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServiceLevel8Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl ServiceLevel8Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SettlementDateTimeIndication11: Date and time at which a payment has been credited at the transaction administrator. In the case of TARGET, the date and time at which the payment has been credited at the receiving central bank, expressed in Central European Time (CET).
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementDateTimeIndication11 {
	#[serde(rename = "DbtDtTm", skip_serializing_if = "Option::is_none")]
	pub dbt_dt_tm: Option<String>,
	#[serde(rename = "CdtDtTm", skip_serializing_if = "Option::is_none")]
	pub cdt_dt_tm: Option<String>,
}

impl SettlementDateTimeIndication11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbt_dt_tm {
			let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dbt_dt_tm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cdt_dt_tm {
			let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cdt_dt_tm does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SettlementInstruction71: Unambiguous identification of the account of the third reimbursement agent account at its servicing agent in the payment chain.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementInstruction71 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: SettlementMethod1Code1,
	#[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
	pub sttlm_acct: Option<CashAccount381>,
	#[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount381>,
	#[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount381>,
	#[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification61>,
	#[serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount381>,
}

impl SettlementInstruction71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sttlm_mtd.validate()?;
		if let Some(ref val) = self.sttlm_acct { val.validate()? }
		if let Some(ref val) = self.instg_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.instg_rmbrsmnt_agt_acct { val.validate()? }
		if let Some(ref val) = self.instd_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.instd_rmbrsmnt_agt_acct { val.validate()? }
		if let Some(ref val) = self.thrd_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct { val.validate()? }
		Ok(())
	}
}


// SettlementMethod1Code__1: Settlement is done through a cover payment.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SettlementMethod1Code1 {
	#[default]
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "INGA")]
	CodeINGA,
	#[serde(rename = "COVE")]
	CodeCOVE,
}

impl SettlementMethod1Code1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementTimeRequest21: Time by when the payment must be settled to avoid rejection.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementTimeRequest21 {
	#[serde(rename = "CLSTm", skip_serializing_if = "Option::is_none")]
	pub cls_tm: Option<String>,
	#[serde(rename = "TillTm", skip_serializing_if = "Option::is_none")]
	pub till_tm: Option<String>,
	#[serde(rename = "FrTm", skip_serializing_if = "Option::is_none")]
	pub fr_tm: Option<String>,
	#[serde(rename = "RjctTm", skip_serializing_if = "Option::is_none")]
	pub rjct_tm: Option<String>,
}

impl SettlementTimeRequest21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cls_tm {
			let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cls_tm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.till_tm {
			let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "till_tm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fr_tm {
			let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fr_tm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rjct_tm {
			let pattern = Regex::new(".*(\\+|-)((0[0-9])|(1[0-4])):[0-5][0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "rjct_tm does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// StructuredRegulatoryReporting31: Additional details that cater for specific domestic regulatory requirements.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRegulatoryReporting31 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<CBPRAmount1>,
	#[serde(rename = "Inf", skip_serializing_if = "Option::is_none")]
	pub inf: Option<Vec<String>>,
}

impl StructuredRegulatoryReporting31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 10".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "inf exceeds the maximum length of 35".to_string()));
				}
				let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ ]+").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "inf does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// StructuredRemittanceInformation161: Additional information, in free text form, to complement the structured remittance information.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuredRemittanceInformation161 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation71>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount21>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation21>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification1354>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification1354>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxInformation71>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment31>,
	#[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rmt_inf: Option<Vec<String>>,
}

impl StructuredRemittanceInformation161 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rfrd_doc_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rfrd_doc_amt { val.validate()? }
		if let Some(ref val) = self.cdtr_ref_inf { val.validate()? }
		if let Some(ref val) = self.invcr { val.validate()? }
		if let Some(ref val) = self.invcee { val.validate()? }
		if let Some(ref val) = self.tax_rmt { val.validate()? }
		if let Some(ref val) = self.grnshmt_rmt { val.validate()? }
		if let Some(ref vec) = self.addtl_rmt_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rmt_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_rmt_inf exceeds the maximum length of 140".to_string()));
				}
				let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "addtl_rmt_inf does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// TaxAmount2: Set of elements used to provide details on the tax period and amount.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmount2 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<TaxRecordDetails2>>,
}

impl TaxAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_amt { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxAmountAndType11: Amount of money, which has been typed.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmountAndType11 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxAmountType1Choice1>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxAmountAndType11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// TaxAmountType1Choice1: Specifies the amount type, in a free-text form.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAmountType1Choice1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}

impl TaxAmountType1Choice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// TaxAuthorisation11: Name of the debtor or the debtor's authorised representative.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxAuthorisation11 {
	#[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
	pub titl: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
}

impl TaxAuthorisation11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "titl exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "titl does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nm does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// TaxInformation71: Record of tax details.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxInformation71 {
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<TaxParty11>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<TaxParty21>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<TaxParty21>,
	#[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<String>,
	#[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<TaxRecord21>>,
}

impl TaxInformation71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.admstn_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "admstn_zone does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ref_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mtd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxParty11: Type of tax payer.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty11 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<String>,
}

impl TaxParty11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_tp does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// TaxParty21: Details of the authorised tax paying party.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxParty21 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<String>,
	#[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
	pub authstn: Option<TaxAuthorisation11>,
}

impl TaxParty21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.authstn { val.validate()? }
		Ok(())
	}
}


// TaxPeriod2: Range of time between a start date and an end date for which the tax report is provided.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxPeriod2 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		Ok(())
	}
}


// TaxRecord21: Further details of the tax record.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxRecord21 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<String>,
	#[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
	pub ctgy_dtls: Option<String>,
	#[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
	pub dbtr_sts: Option<String>,
	#[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
	pub cert_id: Option<String>,
	#[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
	pub frms_cd: Option<String>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount2>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<String>,
}

impl TaxRecord21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctgy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy_dtls {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy_dtls is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy_dtls exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctgy_dtls does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.dbtr_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_sts exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dbtr_sts does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cert_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_id exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cert_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "frms_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.tax_amt { val.validate()? }
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
			let pattern = Regex::new("[0-9a-zA-Z/\\-\\?:\\(\\)\\.,'\\+ !#$%&\\*=^_`\\{\\|\\}~\";<>@\\[\\\\\\]]+").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "addtl_inf does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// TaxRecordDetails2: Underlying tax amount related to the specified period.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaxRecordDetails2 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prd { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// TaxRecordPeriod1Code: Tax is related to the second half of the period.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaxRecordPeriod1Code {
	#[default]
	#[serde(rename = "MM01")]
	CodeMM01,
	#[serde(rename = "MM02")]
	CodeMM02,
	#[serde(rename = "MM03")]
	CodeMM03,
	#[serde(rename = "MM04")]
	CodeMM04,
	#[serde(rename = "MM05")]
	CodeMM05,
	#[serde(rename = "MM06")]
	CodeMM06,
	#[serde(rename = "MM07")]
	CodeMM07,
	#[serde(rename = "MM08")]
	CodeMM08,
	#[serde(rename = "MM09")]
	CodeMM09,
	#[serde(rename = "MM10")]
	CodeMM10,
	#[serde(rename = "MM11")]
	CodeMM11,
	#[serde(rename = "MM12")]
	CodeMM12,
	#[serde(rename = "QTR1")]
	CodeQTR1,
	#[serde(rename = "QTR2")]
	CodeQTR2,
	#[serde(rename = "QTR3")]
	CodeQTR3,
	#[serde(rename = "QTR4")]
	CodeQTR4,
	#[serde(rename = "HLF1")]
	CodeHLF1,
	#[serde(rename = "HLF2")]
	CodeHLF2,
}

impl TaxRecordPeriod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
