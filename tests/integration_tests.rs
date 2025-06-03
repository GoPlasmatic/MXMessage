// Integration tests for the MX Message library

use mx_message::document::Document;
use mx_message::pacs_008_001_08::*;
use serde_json;

#[test]
fn test_create_and_validate_pacs008_message() {
    let document = create_test_pacs008_message();

    // Test validation
    match document.validate() {
        Ok(()) => println!("✓ Message is valid"),
        Err(e) => {
            println!("✗ Validation failed: {} (code: {})", e.message, e.code);
            panic!(
                "Message should be valid, but got error: {} (code: {})",
                e.message, e.code
            );
        }
    }
}

#[test]
fn test_json_serialization_roundtrip() {
    let original_document = create_test_pacs008_message();

    // Serialize to JSON
    let json_str = serde_json::to_string(&original_document).expect("Should serialize to JSON");

    // Deserialize from JSON
    let deserialized_document: Document =
        serde_json::from_str(&json_str).expect("Should deserialize from JSON");

    // Validate deserialized message
    assert!(
        deserialized_document.validate().is_ok(),
        "Deserialized message should be valid"
    );

    // Compare original and deserialized
    assert_eq!(
        original_document, deserialized_document,
        "Original and deserialized should be equal"
    );
}

#[test]
fn test_json_pretty_serialization() {
    let document = create_test_pacs008_message();

    let json_str =
        serde_json::to_string_pretty(&document).expect("Should serialize to pretty JSON");

    // Check that it contains expected fields
    assert!(json_str.contains("FIToFICstmrCdtTrf"));
    assert!(json_str.contains("GrpHdr"));
    assert!(json_str.contains("CdtTrfTxInf"));
    assert!(json_str.contains("MsgId"));
}

#[test]
fn test_validation_errors() {
    // Create an invalid message with empty message ID
    let mut document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref mut msg) = document {
        msg.grp_hdr.msg_id = "".to_string(); // Invalid: empty message ID
    }

    let validation_result = document.validate();
    assert!(
        validation_result.is_err(),
        "Should fail validation with empty message ID"
    );

    if let Err(error) = validation_result {
        assert_eq!(error.code, 1001); // Should be the "shorter than minimum length" error
        assert!(error.message.contains("msg_id"));
    }
}

#[test]
fn test_validation_iban_format() {
    let mut document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref mut msg) = document {
        // Set invalid IBAN
        if let Some(ref mut debtor_account) = msg.cdt_trf_tx_inf[0].dbtr_acct {
            debtor_account.id.iban = Some("INVALID_IBAN".to_string());
        }
    }

    let validation_result = document.validate();
    assert!(
        validation_result.is_err(),
        "Should fail validation with invalid IBAN"
    );

    if let Err(error) = validation_result {
        assert_eq!(error.code, 1005); // Should be the pattern mismatch error
        assert!(error.message.contains("iban"));
    }
}

#[test]
fn test_message_id_length_validation() {
    let mut document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref mut msg) = document {
        // Set message ID that's too long (over 35 characters)
        msg.grp_hdr.msg_id = "A".repeat(36);
    }

    let validation_result = document.validate();
    assert!(
        validation_result.is_err(),
        "Should fail validation with too long message ID"
    );

    if let Err(error) = validation_result {
        assert_eq!(error.code, 1002); // Should be the "exceeds maximum length" error
    }
}

#[test]
fn test_number_of_transactions_pattern() {
    let mut document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref mut msg) = document {
        // Set invalid number of transactions (should be numeric)
        msg.grp_hdr.nb_of_txs = "ABC".to_string();
    }

    let validation_result = document.validate();
    assert!(
        validation_result.is_err(),
        "Should fail validation with non-numeric nb_of_txs"
    );

    if let Err(error) = validation_result {
        assert_eq!(error.code, 1005); // Should be the pattern mismatch error
        assert!(error.message.contains("nb_of_txs"));
    }
}

#[test]
fn test_unknown_document_type() {
    let document = Document::UNKNOWN;

    let validation_result = document.validate();
    assert!(
        validation_result.is_err(),
        "Unknown document type should fail validation"
    );

    if let Err(error) = validation_result {
        assert_eq!(error.code, 9999); // Should be the unknown document type error
        assert!(error.message.contains("Unknown document type"));
    }
}

#[test]
fn test_minimal_valid_message() {
    let document = create_minimal_pacs008_message();

    // Should validate successfully
    assert!(
        document.validate().is_ok(),
        "Minimal message should be valid"
    );

    // Should serialize/deserialize successfully
    let json_str = serde_json::to_string(&document).expect("Should serialize minimal message");

    let deserialized: Document =
        serde_json::from_str(&json_str).expect("Should deserialize minimal message");

    assert_eq!(
        document, deserialized,
        "Minimal message roundtrip should work"
    );
}

#[test]
fn test_currency_and_amount_structure() {
    let document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref msg) = document {
        let tx = &msg.cdt_trf_tx_inf[0];

        // Test settlement amount
        assert_eq!(tx.intr_bk_sttlm_amt.ccy, "EUR");
        assert_eq!(tx.intr_bk_sttlm_amt.value, 1000.00);

        // Test instructed amount if present
        if let Some(ref instructed_amt) = tx.instd_amt {
            assert_eq!(instructed_amt.ccy, "EUR");
            assert_eq!(instructed_amt.value, 1000.00);
        }
    }
}

#[test]
fn test_party_identification_structure() {
    let document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref msg) = document {
        let tx = &msg.cdt_trf_tx_inf[0];

        // Test debtor information
        assert_eq!(tx.dbtr.nm, Some("ACME Corporation".to_string()));
        assert_eq!(tx.dbtr.ctry_of_res, Some("DE".to_string()));

        // Test creditor information
        assert_eq!(tx.cdtr.nm, Some("Global Suppliers Ltd".to_string()));
        assert_eq!(tx.cdtr.ctry_of_res, Some("FR".to_string()));

        // Test financial institution information
        assert_eq!(tx.dbtr_agt.fin_instn_id.bicfi, Some("DEUTDEFF".to_string()));
        assert_eq!(tx.cdtr_agt.fin_instn_id.bicfi, Some("BNPAFRPP".to_string()));
    }
}

#[test]
fn test_remittance_information() {
    let document = create_test_pacs008_message();

    if let Document::FIToFICustomerCreditTransferV08(ref msg) = document {
        let tx = &msg.cdt_trf_tx_inf[0];

        if let Some(ref rmt_inf) = tx.rmt_inf {
            if let Some(ref unstructured) = rmt_inf.ustrd {
                assert_eq!(unstructured[0], "Payment for Invoice INV-2024-001");
            }
        }
    }
}

#[test]
fn test_document_message_type_identification() {
    let document = create_test_pacs008_message();

    // Test message type identification
    assert_eq!(document.message_type(), "pacs.008.001.08");

    // Test CBPR+ compliance
    assert!(document.is_cbpr_plus_compliant());
}

#[test]
fn test_unknown_document_cbpr_compliance() {
    let document = Document::UNKNOWN;

    // Test message type identification
    assert_eq!(document.message_type(), "unknown");

    // Test CBPR+ compliance
    assert!(!document.is_cbpr_plus_compliant());
}

// Helper function to create a test pacs.008 message
fn create_test_pacs008_message() -> Document {
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

    let payment_id = PaymentIdentification7 {
        instr_id: Some("INSTR123".to_string()),
        end_to_end_id: "E2E123456789".to_string(),
        tx_id: Some("TXN123456789".to_string()),
        uetr: Some("12345678-1234-4567-8901-123456789012".to_string()),
        clr_sys_ref: None,
    };

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

    let fi_to_fi_msg = FIToFICustomerCreditTransferV08 {
        grp_hdr: group_header,
        cdt_trf_tx_inf: vec![credit_transfer_tx],
        splmtry_data: None,
    };

    Document::FIToFICustomerCreditTransferV08(Box::new(fi_to_fi_msg))
}

// Helper function to create a minimal valid pacs.008 message
fn create_minimal_pacs008_message() -> Document {
    let group_header = GroupHeader93 {
        msg_id: "MIN123".to_string(),
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
        end_to_end_id: "E2EMIN123".to_string(),
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

    Document::FIToFICustomerCreditTransferV08(Box::new(fi_to_fi_msg))
}
