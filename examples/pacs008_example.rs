// Plasmatic MX Message Parsing Library - pacs.008 Example
// This example demonstrates how to create, serialize, deserialize, and validate
// a pacs.008 (FI to FI Customer Credit Transfer) message

use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;
use serde_json;
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
        println!("   Number of Transactions: {}", msg.grp_hdr.nb_of_txs);
        println!(
            "   Settlement Method: {:?}",
            msg.grp_hdr.sttlm_inf.sttlm_mtd
        );

        if let Some(ref total_amount) = msg.grp_hdr.ttl_intr_bk_sttlm_amt {
            println!(
                "   Total Settlement Amount: {} {}",
                total_amount.value, total_amount.ccy
            );
        }

        println!(
            "   Credit Transfer Transactions: {}",
            msg.cdt_trf_tx_inf.len()
        );

        for (i, tx) in msg.cdt_trf_tx_inf.iter().enumerate() {
            println!("   Transaction {}:", i + 1);
            println!("     End-to-End ID: {}", tx.pmt_id.end_to_end_id);
            println!(
                "     Settlement Amount: {} {}",
                tx.intr_bk_sttlm_amt.value, tx.intr_bk_sttlm_amt.ccy
            );
            println!("     Charge Bearer: {:?}", tx.chrg_br);
            if let Some(ref debtor_name) = tx.dbtr.nm {
                println!("     Debtor: {}", debtor_name);
            }
            if let Some(ref creditor_name) = tx.cdtr.nm {
                println!("     Creditor: {}", creditor_name);
            }
        }
    }

    println!("\n=== Example completed successfully ===");
    Ok(())
}

fn create_sample_pacs008_message() -> Result<Document, Box<dyn Error>> {
    // Create Group Header
    let group_header = GroupHeader93 {
        msg_id: "MSG123456789".to_string(),
        cre_dt_tm: "2024-01-15T10:30:00Z".to_string(),
        btch_bookg: Some(false),
        nb_of_txs: "1".to_string(),
        ctrl_sum: Some(1000.00),
        ttl_intr_bk_sttlm_amt: Some(ActiveCurrencyAndAmount {
            ccy: "EUR".to_string(),
            value: 1000.00,
        }),
        intr_bk_sttlm_dt: Some("2024-01-15".to_string()),
        sttlm_inf: SettlementInstruction7 {
            sttlm_mtd: SettlementMethod1Code::CodeCLRG,
            sttlm_acct: None,
            clr_sys: Some(ClearingSystemIdentification3Choice {
                cd: Some("T2".to_string()),
                prtry: None,
            }),
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

    // Create Financial Institution Identification for Debtor Agent
    let debtor_agent = BranchAndFinancialInstitutionIdentification6 {
        fin_instn_id: FinancialInstitutionIdentification18 {
            bicfi: Some("DEUTDEFF".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: Some("Deutsche Bank AG".to_string()),
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    };

    // Create Financial Institution Identification for Creditor Agent
    let creditor_agent = BranchAndFinancialInstitutionIdentification6 {
        fin_instn_id: FinancialInstitutionIdentification18 {
            bicfi: Some("BNPAFRPP".to_string()),
            clr_sys_mmb_id: None,
            lei: None,
            nm: Some("BNP Paribas".to_string()),
            pstl_adr: None,
            othr: None,
        },
        brnch_id: None,
    };

    // Create Debtor Party
    let debtor = PartyIdentification135 {
        nm: Some("ACME Corporation".to_string()),
        pstl_adr: Some(PostalAddress24 {
            adr_tp: None,
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
        ctct_dtls: None,
    };

    // Create Creditor Party
    let creditor = PartyIdentification135 {
        nm: Some("Global Suppliers Ltd".to_string()),
        pstl_adr: Some(PostalAddress24 {
            adr_tp: None,
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
        ctct_dtls: None,
    };

    // Create Payment Identification
    let payment_id = PaymentIdentification7 {
        instr_id: Some("INSTR123".to_string()),
        end_to_end_id: "E2E123456789".to_string(),
        tx_id: Some("TXN123456789".to_string()),
        uetr: Some("12345678-1234-4567-8901-123456789012".to_string()),
        clr_sys_ref: None,
    };

    // Create Credit Transfer Transaction
    let credit_transfer_tx = CreditTransferTransaction39 {
        pmt_id: payment_id,
        pmt_tp_inf: None,
        intr_bk_sttlm_amt: ActiveCurrencyAndAmount {
            ccy: "EUR".to_string(),
            value: 1000.00,
        },
        intr_bk_sttlm_dt: Some("2024-01-15".to_string()),
        sttlm_prty: Some(Priority3Code::CodeNORM),
        sttlm_tm_indctn: None,
        sttlm_tm_req: None,
        accptnc_dt_tm: None,
        poolg_adjstmnt_dt: None,
        instd_amt: Some(ActiveOrHistoricCurrencyAndAmount {
            ccy: "EUR".to_string(),
            value: 1000.00,
        }),
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
        dbtr_acct: Some(CashAccount38 {
            id: AccountIdentification4Choice {
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
        cdtr_acct: Some(CashAccount38 {
            id: AccountIdentification4Choice {
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
        purp: Some(Purpose2Choice {
            cd: Some("SUPP".to_string()),
            prtry: None,
        }),
        rgltry_rptg: None,
        tax: None,
        rltd_rmt_inf: None,
        rmt_inf: Some(RemittanceInformation16 {
            ustrd: Some(vec!["Payment for Invoice INV-2024-001".to_string()]),
            strd: None,
        }),
        splmtry_data: None,
    };

    // Create the main FI to FI Customer Credit Transfer message
    let fi_to_fi_msg = FIToFICustomerCreditTransferV08 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: vec![credit_transfer_tx],
        splmtry_data: None,
    };

    // Wrap in Document
    let document = Document::FIToFICustomerCreditTransferV08(Box::new(fi_to_fi_msg));

    Ok(document)
}
