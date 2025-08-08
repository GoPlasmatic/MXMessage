// Plasmatic MX Message Parsing Library - XML Serialization Example
// This example demonstrates XML serialization and deserialization of pacs.008 messages

use mx_message::document::pacs_008_001_08::*;
use mx_message::parse_result::{ErrorCollector, ParserConfig};
use mx_message::validation::Validate;
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== ISO20022 XML Serialization Example ===\n");

    // Create a sample message
    let document = create_minimal_pacs008_message()?;

    // Validate the message
    println!("1. Validating the message...");
    let config = ParserConfig::default();
    let mut collector = ErrorCollector::new();
    document.validate("", &config, &mut collector);

    if collector.has_errors() {
        println!(
            "✗ Message validation failed with {} errors:",
            collector.error_count()
        );
        for error in collector.errors() {
            println!("  - {} (code: {})", error.message, error.code);
        }
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
    println!("{xml_output}\n");

    // Deserialize from XML
    println!("4. XML Deserialization:");
    let deserialized_document: FIToFICustomerCreditTransferV08 = xml_from_str(&xml_output)?;
    println!("✓ Successfully deserialized from XML\n");

    // Validate the deserialized message
    println!("5. Validating deserialized message...");
    let mut collector2 = ErrorCollector::new();
    deserialized_document.validate("", &config, &mut collector2);

    if collector2.has_errors() {
        println!(
            "✗ Deserialized message validation failed with {} errors:",
            collector2.error_count()
        );
        for error in collector2.errors() {
            println!("  - {} (code: {})", error.message, error.code);
        }
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
        println!("{original_json}\n");
        println!("Deserialized JSON:");
        println!("{deserialized_json}\n");
    }

    println!("=== XML Serialization Example completed ===");
    Ok(())
}

fn create_minimal_pacs008_message() -> Result<FIToFICustomerCreditTransferV08, Box<dyn Error>> {
    // Create a minimal but valid pacs.008 message for XML testing

    let group_header = GroupHeader931 {
        msg_id: "XML123".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        nb_of_txs: Max15NumericTextfixed::Code1,
        sttlm_inf: SettlementInstruction71 {
            sttlm_mtd: SettlementMethod1Code1::CodeINDA,
            sttlm_acct: None,
            instg_rmbrsmnt_agt: None,
            instg_rmbrsmnt_agt_acct: None,
            instd_rmbrsmnt_agt: None,
            instd_rmbrsmnt_agt_acct: None,
            thrd_rmbrsmnt_agt: None,
            thrd_rmbrsmnt_agt_acct: None,
        },
    };

    let debtor_agent = BranchAndFinancialInstitutionIdentification61 {
        fin_instn_id: FinancialInstitutionIdentification181 {
            bicfi: Some("TESTBIC1".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: None,
            pstl_adr: None,
        },
    };

    let creditor_agent = BranchAndFinancialInstitutionIdentification63 {
        fin_instn_id: FinancialInstitutionIdentification181 {
            bicfi: Some("TESTBIC2".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: None,
            pstl_adr: None,
        },
        brnch_id: None,
    };

    let instructing_agent = BranchAndFinancialInstitutionIdentification62 {
        fin_instn_id: FinancialInstitutionIdentification182 {
            bicfi: "TESTBIC3".to_string(),
            clr_sys_mmb_id: None,
            lei: None,
        },
    };

    let instructed_agent = BranchAndFinancialInstitutionIdentification62 {
        fin_instn_id: FinancialInstitutionIdentification182 {
            bicfi: "TESTBIC4".to_string(),
            clr_sys_mmb_id: None,
            lei: None,
        },
    };

    let debtor = PartyIdentification1352 {
        nm: Some("Test Debtor".to_string()),
        pstl_adr: None,
        id: None,
        ctry_of_res: None,
    };

    let creditor = PartyIdentification1353 {
        nm: Some("Test Creditor".to_string()),
        pstl_adr: None,
        id: None,
        ctry_of_res: None,
    };

    let payment_id = PaymentIdentification71 {
        instr_id: "INSTR123".to_string(),
        end_to_end_id: "E2EXML123".to_string(),
        tx_id: None,
        uetr: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        clr_sys_ref: None,
    };

    let credit_transfer_tx = CreditTransferTransaction391 {
        pmt_id: payment_id,
        pmt_tp_inf: None,
        intr_bk_sttlm_amt: CBPRAmount1 {
            ccy: "USD".to_string(),
            value: 100.00,
        },
        intr_bk_sttlm_dt: "2024-01-16".to_string(),
        sttlm_prty: None,
        sttlm_tm_indctn: None,
        sttlm_tm_req: None,
        instd_amt: None,
        xchg_rate: None,
        chrg_br: ChargeBearerType1Code1::CodeSHAR,
        chrgs_inf: None,
        prvs_instg_agt1: None,
        prvs_instg_agt1_acct: None,
        prvs_instg_agt2: None,
        prvs_instg_agt2_acct: None,
        prvs_instg_agt3: None,
        prvs_instg_agt3_acct: None,
        instg_agt: instructing_agent,
        instd_agt: instructed_agent,
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
        rltd_rmt_inf: None,
        rmt_inf: None,
    };

    let fi_to_fi_msg = FIToFICustomerCreditTransferV08 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: credit_transfer_tx,
    };

    Ok(fi_to_fi_msg)
}
