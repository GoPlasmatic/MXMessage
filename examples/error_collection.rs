use mx_message::document::pacs_008_001_08::{
    FIToFICustomerCreditTransferV08, GroupHeader931, Max15NumericTextfixed,
    SettlementInstruction71, SettlementMethod1Code1,
};
use mx_message::parse_result::{ErrorCollector, ParserConfig};
use mx_message::validation::Validate;

fn main() {
    println!("Error Collection Example\n");

    // Create a message with multiple validation issues
    let payment = FIToFICustomerCreditTransferV08 {
        grp_hdr: GroupHeader931 {
            msg_id: "ID".to_string(),              // Too short (min 5 chars)
            cre_dt_tm: "invalid-date".to_string(), // Wrong format
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
        },
        cdt_trf_tx_inf: Default::default(), // Required field with complex type
    };

    // Collect all validation errors
    let config = ParserConfig::default();
    let mut collector = ErrorCollector::new();
    payment.validate("", &config, &mut collector);

    // Process all errors at once
    if collector.has_errors() {
        println!("Found {} validation errors:", collector.error_count());
        for error in collector.errors() {
            println!(
                "  - {}: {} (code: {})",
                error.path.as_ref().unwrap_or(&"root".to_string()),
                error.message,
                error.code
            );
        }
    } else {
        println!("✓ Message is valid");
    }

    println!("\n---\n");

    // Example with a valid message
    let valid_payment = FIToFICustomerCreditTransferV08 {
        grp_hdr: GroupHeader931 {
            msg_id: "MSGID123456".to_string(),
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
        },
        cdt_trf_tx_inf: Default::default(),
    };

    let mut collector2 = ErrorCollector::new();
    valid_payment.validate("", &config, &mut collector2);

    if collector2.has_errors() {
        println!("Found {} validation errors:", collector2.error_count());
        for error in collector2.errors() {
            println!("  - {}: {}", error.code, error.message);
        }
    } else {
        println!(
            "✓ Message is valid (except for empty transactions which may be business rule violation)"
        );
    }
}
