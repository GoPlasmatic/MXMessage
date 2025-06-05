// Plasmatic MX Message Parsing Library - XML Serialization Example
// This example demonstrates XML serialization and deserialization of pacs.008 messages

use mx_message::common::*;
use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== ISO20022 XML Serialization Example ===\n");

    // Create a sample message
    let document = create_minimal_pacs008_message()?;

    // Validate the message
    println!("1. Validating the message...");
    if let Err(e) = document.validate() {
        println!("✗ Message validation failed: {:?}", e);
    } else {
        println!("✓ Message validation successful\n");
    }

    // Serialize to JSON first (for comparison)
    println!("2. JSON Serialization:");
    let json_output = serde_json::to_string_pretty(&document)?;
    println!("JSON (first 300 chars):");
    println!(
        "{}\n",
        &json_output[..std::cmp::min(300, json_output.len())]
    );

    // Serialize to XML
    println!("3. XML Serialization:");
    let xml_output = xml_to_string(&document)?;
    println!("XML Output:");
    println!("{}\n", xml_output);

    // Deserialize from XML
    println!("4. XML Deserialization:");
    let deserialized_document: Document = xml_from_str(&xml_output)?;
    println!("✓ Successfully deserialized from XML\n");

    // Validate the deserialized message
    println!("5. Validating deserialized message...");
    if let Err(e) = deserialized_document.validate() {
        println!("✗ Deserialized message validation failed: {:?}", e);
    } else {
        println!("✓ Deserialized message validation successful\n");
    }

    // Compare original and deserialized
    println!("6. Comparing original and deserialized messages...");
    if document == deserialized_document {
        println!("✓ Original and deserialized messages are identical\n");
    } else {
        println!("✗ Original and deserialized messages differ\n");

        // Show differences in JSON format for easier comparison
        let original_json = serde_json::to_string_pretty(&document)?;
        let deserialized_json = serde_json::to_string_pretty(&deserialized_document)?;

        println!("Original JSON:");
        println!("{}\n", original_json);
        println!("Deserialized JSON:");
        println!("{}\n", deserialized_json);
    }

    println!("=== XML Serialization Example completed ===");
    Ok(())
}

fn create_minimal_pacs008_message() -> Result<Document, Box<dyn Error>> {
    // Create a minimal but valid pacs.008 message for XML testing

    let group_header = GroupHeader93 {
        msg_id: "XML123".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        btch_bookg: None,
        nb_of_txs: "1".to_string(),
        ctrl_sum: None,
        ttl_intr_bk_sttlm_amt: None,
        intr_bk_sttlm_dt: None,
        sttlm_inf: SettlementInstruction7 {
            sttlm_mtd: SettlementMethod1Code::CodeINDA,
            sttlm_acct: None,
            clr_sys: None,
            instg_rmbrsmnt_agt: None,
            instg_rmbrsmnt_agt_acct: None,
            instd_rmbrsmnt_agt: None,
            instd_rmbrsmnt_agt_acct: None,
            thrd_rmbrsmnt_agt: None,
            thrd_rmbrsmnt_agt_acct: None,
        },
        pmt_tp_inf: None,
        instg_agt: None,
        instd_agt: None,
    };

    let debtor_agent = BranchAndFinancialInstitutionIdentification6 {
        fin_instn_id: FinancialInstitutionIdentification18 {
            bicfi: Some("TESTBIC1".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: None,
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    };

    let creditor_agent = BranchAndFinancialInstitutionIdentification6 {
        fin_instn_id: FinancialInstitutionIdentification18 {
            bicfi: Some("TESTBIC2".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: None,
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    };

    let debtor = PartyIdentification135 {
        nm: Some("Test Debtor".to_string()),
        pstl_adr: None,
        id: None,
        ctry_of_res: None,
        ctct_dtls: None,
    };

    let creditor = PartyIdentification135 {
        nm: Some("Test Creditor".to_string()),
        pstl_adr: None,
        id: None,
        ctry_of_res: None,
        ctct_dtls: None,
    };

    let payment_id = PaymentIdentification7 {
        instr_id: None,
        end_to_end_id: "E2EXML123".to_string(),
        tx_id: None,
        uetr: None,
        clr_sys_ref: None,
    };

    let credit_transfer_tx = CreditTransferTransaction39 {
        pmt_id: payment_id,
        pmt_tp_inf: None,
        intr_bk_sttlm_amt: ActiveCurrencyAndAmount {
            ccy: "USD".to_string(),
            value: 100.00,
        },
        intr_bk_sttlm_dt: None,
        sttlm_prty: None,
        sttlm_tm_indctn: None,
        sttlm_tm_req: None,
        accptnc_dt_tm: None,
        poolg_adjstmnt_dt: None,
        instd_amt: None,
        xchg_rate: None,
        chrg_br: ChargeBearerType1Code::CodeSHAR,
        chrgs_inf: None,
        prvs_instg_agt1: None,
        prvs_instg_agt1_acct: None,
        prvs_instg_agt2: None,
        prvs_instg_agt2_acct: None,
        prvs_instg_agt3: None,
        prvs_instg_agt3_acct: None,
        instg_agt: None,
        instd_agt: None,
        intrmy_agt1: None,
        intrmy_agt1_acct: None,
        intrmy_agt2: None,
        intrmy_agt2_acct: None,
        intrmy_agt3: None,
        intrmy_agt3_acct: None,
        ultmt_dbtr: None,
        initg_pty: None,
        dbtr: debtor,
        dbtr_acct: None,
        dbtr_agt: debtor_agent,
        dbtr_agt_acct: None,
        cdtr_agt: creditor_agent,
        cdtr_agt_acct: None,
        cdtr: creditor,
        cdtr_acct: None,
        ultmt_cdtr: None,
        instr_for_cdtr_agt: None,
        instr_for_nxt_agt: None,
        purp: None,
        rgltry_rptg: None,
        tax: None,
        rltd_rmt_inf: None,
        rmt_inf: None,
        splmtry_data: None,
    };

    let fi_to_fi_msg = FIToFICustomerCreditTransferV08 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: vec![credit_transfer_tx],
        splmtry_data: None,
    };

    let document = Document::FIToFICustomerCreditTransferV08(Box::new(fi_to_fi_msg));

    Ok(document)
}
