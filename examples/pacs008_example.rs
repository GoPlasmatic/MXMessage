// Plasmatic MX Message Parsing Library - pacs.008 Example
// This example demonstrates how to create, serialize, deserialize, and validate
// a pacs.008 (FI to FI Customer Credit Transfer) message

use mx_message::app_document::Document;
use mx_message::document::pacs_008_001_08::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== ISO20022 pacs.008 Message Example ===\n");

    // Create a sample pacs.008 message
    let document = create_sample_pacs008_message()?;

    // Validate the message
    println!("1. Validating the message...");
    match document.validate() {
        Ok(()) => println!("✓ Message validation successful\n"),
        Err(e) => {
            println!(
                "✗ Message validation failed: {} (code: {})\n",
                e.message, e.code
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.message,
            )));
        }
    }

    // Serialize to JSON
    println!("2. Serializing to JSON...");
    let json_output = serde_json::to_string_pretty(&document)?;
    println!("JSON Output (first 500 chars):");
    println!(
        "{}\n",
        &json_output[..std::cmp::min(500, json_output.len())]
    );
    if json_output.len() > 500 {
        println!("... (truncated)\n");
    }

    // Deserialize from JSON
    println!("3. Deserializing from JSON...");
    let deserialized_document: Document = serde_json::from_str(&json_output)?;
    println!("✓ Successfully deserialized from JSON\n");

    // Validate the deserialized message
    println!("4. Validating deserialized message...");
    match deserialized_document.validate() {
        Ok(()) => println!("✓ Deserialized message validation successful\n"),
        Err(e) => {
            println!(
                "✗ Deserialized message validation failed: {} (code: {})\n",
                e.message, e.code
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.message,
            )));
        }
    }

    // Compare original and deserialized
    println!("5. Comparing original and deserialized messages...");
    if document == deserialized_document {
        println!("✓ Original and deserialized messages are identical\n");
    } else {
        println!("✗ Original and deserialized messages differ\n");
    }

    // Display message details
    if let Document::FIToFICustomerCreditTransferV08(ref msg) = document {
        println!("6. Message Details:");
        println!("   Message ID: {}", msg.grp_hdr.msg_id);
        println!("   Creation Date/Time: {}", msg.grp_hdr.cre_dt_tm);
        println!("   Number of Transactions: {:?}", msg.grp_hdr.nb_of_txs);
        println!(
            "   Settlement Method: {:?}",
            msg.grp_hdr.sttlm_inf.sttlm_mtd
        );

        println!("   Credit Transfer Transaction Details:");
        println!(
            "     End-to-End ID: {}",
            msg.cdt_trf_tx_inf.pmt_id.end_to_end_id
        );
        println!(
            "     Settlement Amount: {} {}",
            msg.cdt_trf_tx_inf.intr_bk_sttlm_amt.value, msg.cdt_trf_tx_inf.intr_bk_sttlm_amt.ccy
        );
        println!("     Charge Bearer: {:?}", msg.cdt_trf_tx_inf.chrg_br);
        if let Some(ref debtor_name) = msg.cdt_trf_tx_inf.dbtr.nm {
            println!("     Debtor: {}", debtor_name);
        }
        if let Some(ref creditor_name) = msg.cdt_trf_tx_inf.cdtr.nm {
            println!("     Creditor: {}", creditor_name);
        }
    }

    println!("\n=== Example completed successfully ===");
    Ok(())
}

fn create_sample_pacs008_message() -> Result<Document, Box<dyn Error>> {
    // Create Group Header
    let group_header = GroupHeader931 {
        msg_id: "MSG123456789".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00+00:00".to_string(),
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

    // Create Financial Institution Identification for Debtor Agent
    let debtor_agent = BranchAndFinancialInstitutionIdentification61 {
        fin_instn_id: FinancialInstitutionIdentification181 {
            bicfi: Some("DEUTDEFF".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: Some("Deutsche Bank AG".to_string()),
            pstl_adr: None,
        },
    };

    // Create Financial Institution Identification for Creditor Agent
    let creditor_agent = BranchAndFinancialInstitutionIdentification63 {
        fin_instn_id: FinancialInstitutionIdentification181 {
            bicfi: Some("BNPAFRPP".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: Some("BNP Paribas".to_string()),
            pstl_adr: None,
        },
        brnch_id: None,
    };

    // Create Instructing Agent
    let instructing_agent = BranchAndFinancialInstitutionIdentification62 {
        fin_instn_id: FinancialInstitutionIdentification182 {
            bicfi: "DEUTDEFF".to_string(),
            clr_sys_mmb_id: None,
            lei: None,
        },
    };

    // Create Instructed Agent
    let instructed_agent = BranchAndFinancialInstitutionIdentification62 {
        fin_instn_id: FinancialInstitutionIdentification182 {
            bicfi: "BNPAFRPP".to_string(),
            clr_sys_mmb_id: None,
            lei: None,
        },
    };

    // Create Debtor Party
    let debtor = PartyIdentification1352 {
        nm: Some("ACME Corporation".to_string()),
        pstl_adr: Some(PostalAddress241 {
            dept: None,
            sub_dept: None,
            strt_nm: Some("Main Street".to_string()),
            bldg_nb: Some("123".to_string()),
            bldg_nm: None,
            flr: None,
            pst_bx: None,
            room: None,
            pst_cd: Some("12345".to_string()),
            twn_nm: Some("Frankfurt".to_string()),
            twn_lctn_nm: None,
            dstrct_nm: None,
            ctry_sub_dvsn: None,
            ctry: Some("DE".to_string()),
            adr_line: None,
        }),
        id: None,
        ctry_of_res: Some("DE".to_string()),
    };

    // Create Creditor Party
    let creditor = PartyIdentification1353 {
        nm: Some("Global Suppliers Ltd".to_string()),
        pstl_adr: Some(PostalAddress241 {
            dept: None,
            sub_dept: None,
            strt_nm: Some("Rue de la Paix".to_string()),
            bldg_nb: Some("456".to_string()),
            bldg_nm: None,
            flr: None,
            pst_bx: None,
            room: None,
            pst_cd: Some("75001".to_string()),
            twn_nm: Some("Paris".to_string()),
            twn_lctn_nm: None,
            dstrct_nm: None,
            ctry_sub_dvsn: None,
            ctry: Some("FR".to_string()),
            adr_line: None,
        }),
        id: None,
        ctry_of_res: Some("FR".to_string()),
    };

    // Create Payment Identification
    let payment_id = PaymentIdentification71 {
        instr_id: "INSTR123".to_string(),
        end_to_end_id: "E2E123456789".to_string(),
        tx_id: Some("TXN123456789".to_string()),
        uetr: "12345678-1234-4567-8901-123456789012".to_string(),
        clr_sys_ref: None,
    };

    // Create Credit Transfer Transaction
    let credit_transfer_tx = CreditTransferTransaction391 {
        pmt_id: payment_id,
        pmt_tp_inf: None,
        intr_bk_sttlm_amt: CBPRAmount1 {
            ccy: "EUR".to_string(),
            value: 1000.00,
        },
        intr_bk_sttlm_dt: "2024-01-15".to_string(),
        sttlm_prty: Some(Priority3Code::CodeNORM),
        sttlm_tm_indctn: None,
        sttlm_tm_req: None,
        instd_amt: Some(CBPRAmount1 {
            ccy: "EUR".to_string(),
            value: 1000.00,
        }),
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
        dbtr_acct: Some(CashAccount381 {
            id: AccountIdentification4Choice1 {
                iban: Some("DE89370400440532013000".to_string()),
                othr: None,
            },
            tp: None,
            ccy: Some("EUR".to_string()),
            nm: Some("ACME Main Account".to_string()),
            prxy: None,
        }),
        dbtr_agt: debtor_agent,
        dbtr_agt_acct: None,
        cdtr_agt: creditor_agent,
        cdtr_agt_acct: None,
        cdtr: creditor,
        cdtr_acct: Some(CashAccount381 {
            id: AccountIdentification4Choice1 {
                iban: Some("FR1420041010050500013M02606".to_string()),
                othr: None,
            },
            tp: None,
            ccy: Some("EUR".to_string()),
            nm: Some("Supplier Account".to_string()),
            prxy: None,
        }),
        ultmt_cdtr: None,
        instr_for_cdtr_agt: None,
        instr_for_nxt_agt: None,
        purp: Some(Purpose2Choice1 {
            cd: Some("SUPP".to_string()),
            prtry: None,
        }),
        rgltry_rptg: None,
        rltd_rmt_inf: None,
        rmt_inf: Some(RemittanceInformation161 {
            ustrd: Some("Payment for Invoice INV-2024-001".to_string()),
            strd: None,
        }),
    };

    // Create the main FI to FI Customer Credit Transfer message
    let fi_to_fi_msg = FIToFICustomerCreditTransferV08 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: credit_transfer_tx,
    };

    // Wrap in Document
    let document = Document::FIToFICustomerCreditTransferV08(Box::new(fi_to_fi_msg));

    Ok(document)
}
