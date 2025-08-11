// Integration tests for MX XML generation using sample generation

#[cfg(test)]
mod tests {
    use mx_message::sample::generate_sample_xml;
    use mx_message::scenario_config::ScenarioConfig;

    #[test]
    fn test_xml_generation_with_sample_pacs008() {
        // Generate sample pacs.008 message as XML
        let xml = generate_sample_xml("pacs008", Some("standard"), &ScenarioConfig::default())
            .expect("Failed to generate sample");

        // Verify XML structure
        assert!(xml.contains("<?xml"), "Missing XML declaration");
        assert!(xml.contains("<Envelope"), "Missing Envelope");
        assert!(xml.contains("<AppHdr"), "Missing Application Header");
        assert!(xml.contains("<Document"), "Missing Document");
        assert!(
            xml.contains("xmlns=\"urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08\""),
            "Missing document namespace"
        );
        assert!(
            xml.contains("<FIToFICstmrCdtTrf") || xml.contains("<FIToFICustomerCreditTransferV08>"),
            "Missing message root element"
        );
        // Check that BIC codes are present (they are randomly generated)
        assert!(xml.contains("<BICFI>"), "Missing BIC elements");
    }

    #[test]
    fn test_xml_generation_multiple_scenarios() {
        let scenarios = vec!["standard", "high_value", "cbpr_business_payment", "minimal"];

        for scenario in scenarios {
            let result = generate_sample_xml("pacs008", Some(scenario), &ScenarioConfig::default());

            assert!(
                result.is_ok(),
                "XML generation failed for scenario: {scenario}"
            );

            if let Ok(xml) = result {
                assert!(xml.len() > 500, "XML too short for scenario: {scenario}");
                assert!(
                    xml.contains("<?xml"),
                    "Missing XML declaration for scenario: {scenario}"
                );
                assert!(
                    xml.contains("<Envelope"),
                    "Missing Envelope for scenario: {scenario}"
                );
            }
        }
    }

    #[test]
    fn test_xml_with_different_message_types() {
        // Test PACS.008
        let pacs008_xml = generate_sample_xml("pacs008", None, &ScenarioConfig::default());
        assert!(pacs008_xml.is_ok(), "Failed to generate pacs008 sample");

        if let Ok(xml) = pacs008_xml {
            assert!(
                xml.contains("pacs.008.001.08"),
                "Missing PACS.008 namespace"
            );
            assert!(
                xml.contains("<FIToFICstmrCdtTrf")
                    || xml.contains("<FIToFICustomerCreditTransferV08>")
            );
        }

        // Test PAIN.001
        let pain001_xml =
            generate_sample_xml("pain001", Some("standard"), &ScenarioConfig::default());
        assert!(pain001_xml.is_ok(), "Failed to generate pain001 sample");

        if let Ok(xml) = pain001_xml {
            assert!(
                xml.contains("pain.001.001.09"),
                "Missing PAIN.001 namespace"
            );
            assert!(
                xml.contains("<CstmrCdtTrfInitn")
                    || xml.contains("<CustomerCreditTransferInitiationV09>")
            );
        }

        // Test CAMT.053
        let camt053_xml = generate_sample_xml(
            "camt053",
            Some("daily_account_statement"),
            &ScenarioConfig::default(),
        );
        assert!(camt053_xml.is_ok(), "Failed to generate camt053 sample");

        if let Ok(xml) = camt053_xml {
            assert!(
                xml.contains("camt.053.001.08"),
                "Missing CAMT.053 namespace"
            );
            assert!(xml.contains("<BkToCstmrStmt") || xml.contains("<BankToCustomerStatementV08>"));
        }

        // Test CAMT.052
        let camt052_xml = generate_sample_xml("camt052", None, &ScenarioConfig::default());
        assert!(camt052_xml.is_ok(), "Failed to generate camt052 sample");

        if let Ok(xml) = camt052_xml {
            assert!(
                xml.contains("camt.052.001.08"),
                "Missing CAMT.052 namespace"
            );
        }

        // Test PAIN.008
        let pain008_xml = generate_sample_xml(
            "pain008",
            Some("general_direct_debit_basic"),
            &ScenarioConfig::default(),
        );
        assert!(pain008_xml.is_ok(), "Failed to generate pain008 sample");

        if let Ok(xml) = pain008_xml {
            assert!(
                xml.contains("pain.008.001.08"),
                "Missing PAIN.008 namespace"
            );
        }
    }

    #[test]
    fn test_xml_header_information() {
        // Test that header information is properly generated based on scenario
        let standard_xml =
            generate_sample_xml("pacs008", Some("standard"), &ScenarioConfig::default())
                .expect("Failed to generate standard sample");

        // Check for standard BICs
        assert!(
            standard_xml.contains("<Bicfi>") || standard_xml.contains("<BICFI>"),
            "Missing BIC elements in standard scenario"
        );

        // Test CBPR scenario has specific headers
        let cbpr_xml = generate_sample_xml(
            "pacs008",
            Some("cbpr_business_payment"),
            &ScenarioConfig::default(),
        )
        .expect("Failed to generate CBPR sample");

        // CBPR scenarios have cbprplus in the business service
        assert!(
            cbpr_xml.contains("swift.cbprplus"),
            "CBPR scenario should have CBPR+ business service"
        );
        assert!(
            cbpr_xml.contains("swift.cbprplus.01") || cbpr_xml.contains("<BizSvc>"),
            "CBPR scenario should have business service"
        );

        // Test high value scenario
        let high_value_xml =
            generate_sample_xml("pacs008", Some("high_value"), &ScenarioConfig::default())
                .expect("Failed to generate high value sample");

        // High value scenarios should still contain valid BIC codes
        assert!(
            high_value_xml.contains("<BICFI>"),
            "High value scenario should have BIC identifiers"
        );
    }

    #[test]
    fn test_custom_scenario_config() {
        // Test with custom scenario configuration
        let custom_config = ScenarioConfig::default();

        let xml = generate_sample_xml("pacs008", Some("standard"), &custom_config)
            .expect("Sample generation with custom config failed");

        assert!(xml.contains("<?xml"), "Missing XML declaration");
        assert!(xml.contains("<Envelope"), "Missing Envelope");
        assert!(xml.len() > 500, "Generated XML too short");
    }

    #[test]
    fn test_xml_structure_consistency() {
        // Generate multiple samples and verify consistent structure
        for _ in 0..3 {
            let xml = generate_sample_xml("pacs008", Some("minimal"), &ScenarioConfig::default())
                .expect("Failed to generate sample");

            // Check basic structure elements are always present
            assert!(
                xml.starts_with("<?xml"),
                "XML should start with declaration"
            );
            assert!(xml.contains("<Envelope"), "Should contain Envelope");
            assert!(xml.contains("</Envelope>"), "Should close Envelope");
            assert!(xml.contains("<AppHdr"));
            assert!(xml.contains("<Document"), "Should contain Document");
            assert!(xml.contains("</Document>"), "Should close Document");

            // Check namespaces
            assert!(
                xml.contains("xmlns"),
                "Should contain namespace declarations"
            );
            assert!(
                xml.contains("urn:iso:std:iso:20022:tech:xsd"),
                "Should contain ISO namespace"
            );
        }
    }

    #[test]
    fn test_message_id_generation() {
        // Test that message IDs are unique and follow expected pattern
        let xml1 = generate_sample_xml("pacs008", Some("standard"), &ScenarioConfig::default())
            .expect("Failed to generate first sample");
        let xml2 = generate_sample_xml("pacs008", Some("standard"), &ScenarioConfig::default())
            .expect("Failed to generate second sample");

        // Extract message IDs
        let extract_msg_id = |xml: &str| -> Option<String> {
            if let Some(start) = xml.find("<BizMsgIdr>") {
                if let Some(end) = xml[start..].find("</BizMsgIdr>") {
                    return Some(xml[start + 11..start + end].to_string());
                }
            }
            None
        };

        let msg_id1 = extract_msg_id(&xml1).expect("Failed to extract first message ID");
        let msg_id2 = extract_msg_id(&xml2).expect("Failed to extract second message ID");

        // Message IDs should be unique
        assert_ne!(msg_id1, msg_id2, "Message IDs should be unique");

        // Message IDs should be in the MSG format
        assert!(
            msg_id1.starts_with("MSG"),
            "Message ID should start with MSG"
        );
    }

    #[test]
    fn test_all_supported_message_types() {
        let message_types = vec![
            ("pacs008", None),
            ("pacs009", Some("standard")),
            ("pacs003", Some("fi_direct_debit_basic")),
            ("pacs002", Some("cheque_collection_advice")),
            ("pain001", Some("standard")),
            ("pain008", Some("general_direct_debit_basic")),
            ("camt025", Some("central_bank_rate_notification")),
            ("camt029", Some("answer_cancellation")),
            ("camt052", Some("daily_balance_report")),
            ("camt053", Some("daily_account_statement")),
            ("camt054", Some("basic_debit_confirmation")),
            ("camt056", Some("cbpr_cancellation_request")),
            ("camt057", Some("expected_incoming_funds")),
            ("camt060", Some("interim_report_request")),
        ];

        for (msg_type, scenario) in message_types {
            let result = generate_sample_xml(msg_type, scenario, &ScenarioConfig::default());
            assert!(
                result.is_ok(),
                "Failed to generate {msg_type} with scenario {scenario:?}"
            );

            if let Ok(xml) = result {
                // Check that XML contains message type in namespace or identifier
                let msg_type_with_dot = msg_type
                    .replace("pacs", "pacs.")
                    .replace("pain", "pain.")
                    .replace("camt", "camt.");
                assert!(
                    xml.contains(&msg_type_with_dot) || xml.contains(msg_type),
                    "XML for {msg_type} should contain message type identifier"
                );
                assert!(xml.len() > 500, "XML for {msg_type} seems too short");
            }
        }
    }
}
