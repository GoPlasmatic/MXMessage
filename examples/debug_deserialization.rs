use datafake_rs::DataGenerator;
/// Debug example to test JSON and XML de/serialization with the new Document structure
///
/// This example tests:
/// 1. JSON serialization/deserialization
/// 2. XML serialization/deserialization
/// 3. JSON -> XML -> JSON round-trip
/// 4. XML -> JSON -> XML round-trip
///
/// Run with:
/// ```
/// cargo run --example debug_deserialization
/// ```
use mx_message::mx_envelope::MxMessage;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         MX Message Serialization/Deserialization Test        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Generate test message once
    let test_message = generate_test_message();

    if let Some((json_str, msg)) = test_message {
        println!("\n");
        test_json_serialization(&json_str);

        println!("\n");
        test_xml_serialization(&msg);

        println!("\n");
        test_json_to_xml_roundtrip(&json_str);

        println!("\n");
        test_xml_to_json_roundtrip(&msg);

        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                      ALL TESTS COMPLETE                      ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

fn generate_test_message() -> Option<(String, MxMessage)> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("SETUP: Generating Test Message");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let scenario_path = "test_scenarios/pacs008/minimal.json";
    println!("📖 Loading scenario: {}", scenario_path);

    match std::fs::read_to_string(scenario_path) {
        Ok(scenario_json) => match serde_json::from_str::<serde_json::Value>(&scenario_json) {
            Ok(scenario) => match DataGenerator::from_value(scenario) {
                Ok(generator) => match generator.generate() {
                    Ok(generated) => {
                        println!("✅ Generated test data");

                        let json_data = if let Some(schema) = generated.get("schema") {
                            schema
                        } else if let Some(json_data) = generated.get("json_data") {
                            json_data
                        } else {
                            &generated
                        };

                        let json_str = serde_json::to_string_pretty(json_data).unwrap();

                        match MxMessage::from_json(&json_str) {
                            Ok(msg) => {
                                println!("✅ Created MxMessage from JSON");
                                println!("📄 Generated JSON: {:?}", msg);
                                Some((json_str, msg))
                            }
                            Err(e) => {
                                println!("❌ Failed to create MxMessage: {}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Generation failed: {}", e);
                        None
                    }
                },
                Err(e) => {
                    println!("❌ Failed to create generator: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("❌ Failed to parse scenario JSON: {}", e);
                None
            }
        },
        Err(e) => {
            println!("❌ Failed to load scenario: {}", e);
            println!("   Make sure you run this from the MXMessage directory");
            None
        }
    }
}

fn test_json_serialization(original_json: &str) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 1: JSON Deserialization -> Serialization");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Deserialize from JSON
    print!("Step 1.1: Deserializing JSON... ");
    match MxMessage::from_json(original_json) {
        Ok(msg) => {
            println!("✅");

            // Serialize back to JSON
            print!("Step 1.2: Serializing to JSON... ");
            match msg.to_json() {
                Ok(json_output) => {
                    println!("✅");

                    // Compare
                    print!("Step 1.3: Comparing JSON structures... ");
                    let original: serde_json::Value = serde_json::from_str(original_json).unwrap();
                    let roundtrip: serde_json::Value = serde_json::from_str(&json_output).unwrap();

                    if original == roundtrip {
                        println!("✅ IDENTICAL");
                        println!("\n📊 Result: JSON serialization is lossless");
                    } else {
                        println!("⚠️  DIFFERENT");
                        println!("\n📊 Result: JSON structures differ (may be expected)");
                    }

                    // Save for inspection
                    std::fs::write("/tmp/test_json_original.json", original_json).ok();
                    std::fs::write("/tmp/test_json_roundtrip.json", &json_output).ok();
                    println!("💾 Files saved to /tmp/test_json_*.json");
                }
                Err(e) => {
                    println!("❌ FAILED");
                    println!("   Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED");
            println!("   Error: {}", e);
        }
    }
}

fn test_xml_serialization(msg: &MxMessage) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 2: XML Serialization -> Deserialization");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Serialize to XML
    print!("Step 2.1: Serializing to XML... ");
    match msg.to_xml() {
        Ok(xml) => {
            println!("✅");
            println!("\n📄 Generated XML (first 500 chars):");
            println!("{}", xml.chars().take(500).collect::<String>());
            if xml.len() > 500 {
                println!("... ({} more characters)", xml.len() - 500);
            }

            // Deserialize from XML
            print!("\nStep 2.2: Deserializing XML... ");
            match MxMessage::from_xml(&xml) {
                Ok(msg_from_xml) => {
                    println!("✅");

                    // Convert both to JSON for comparison
                    print!("Step 2.3: Comparing structures... ");
                    let json1 = msg.to_json().unwrap();
                    let json2 = msg_from_xml.to_json().unwrap();

                    let value1: serde_json::Value = serde_json::from_str(&json1).unwrap();
                    let value2: serde_json::Value = serde_json::from_str(&json2).unwrap();

                    if value1 == value2 {
                        println!("✅ IDENTICAL");
                        println!("\n📊 Result: XML serialization is lossless");
                    } else {
                        println!("⚠️  DIFFERENT");
                        println!("\n📊 Result: Structures differ (may be expected)");
                    }

                    // Save for inspection
                    std::fs::write("/tmp/test_xml.xml", &xml).ok();
                    println!("💾 File saved to /tmp/test_xml.xml");
                }
                Err(e) => {
                    println!("❌ FAILED");
                    println!("   Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED");
            println!("   Error: {}", e);
        }
    }
}

fn test_json_to_xml_roundtrip(original_json: &str) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 3: JSON -> XML -> JSON Round-Trip");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    print!("Step 3.1: JSON -> MxMessage... ");
    match MxMessage::from_json(original_json) {
        Ok(msg) => {
            println!("✅");

            print!("Step 3.2: MxMessage -> XML... ");
            match msg.to_xml() {
                Ok(xml) => {
                    println!("✅");

                    print!("Step 3.3: XML -> MxMessage... ");
                    match MxMessage::from_xml(&xml) {
                        Ok(msg2) => {
                            println!("✅");

                            print!("Step 3.4: MxMessage -> JSON... ");
                            match msg2.to_json() {
                                Ok(final_json) => {
                                    println!("✅");

                                    // Compare original and final JSON
                                    print!("Step 3.5: Comparing JSON... ");
                                    let original: serde_json::Value =
                                        serde_json::from_str(original_json).unwrap();
                                    let final_val: serde_json::Value =
                                        serde_json::from_str(&final_json).unwrap();

                                    if original == final_val {
                                        println!("✅ IDENTICAL");
                                        println!(
                                            "\n📊 Result: JSON -> XML -> JSON preserves all data!"
                                        );
                                    } else {
                                        println!("⚠️  DIFFERENT");
                                        println!(
                                            "\n📊 Result: Round-trip caused changes (may be expected)"
                                        );
                                    }

                                    std::fs::write(
                                        "/tmp/test_roundtrip_json_to_xml_to_json.json",
                                        &final_json,
                                    )
                                    .ok();
                                    std::fs::write("/tmp/test_roundtrip_json_to_xml.xml", &xml)
                                        .ok();
                                    println!("💾 Files saved to /tmp/test_roundtrip_json_to_*");
                                }
                                Err(e) => println!("❌ FAILED: {}", e),
                            }
                        }
                        Err(e) => println!("❌ FAILED: {}", e),
                    }
                }
                Err(e) => println!("❌ FAILED: {}", e),
            }
        }
        Err(e) => println!("❌ FAILED: {}", e),
    }
}

fn test_xml_to_json_roundtrip(msg: &MxMessage) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 4: XML -> JSON -> XML Round-Trip");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    print!("Step 4.1: MxMessage -> XML... ");
    match msg.to_xml() {
        Ok(original_xml) => {
            println!("✅");

            print!("Step 4.2: XML -> MxMessage... ");
            match MxMessage::from_xml(&original_xml) {
                Ok(msg) => {
                    println!("✅");

                    print!("Step 4.3: MxMessage -> JSON... ");
                    match msg.to_json() {
                        Ok(json) => {
                            println!("✅");

                            print!("Step 4.4: JSON -> MxMessage... ");
                            match MxMessage::from_json(&json) {
                                Ok(msg2) => {
                                    println!("✅");

                                    print!("Step 4.5: MxMessage -> XML... ");
                                    match msg2.to_xml() {
                                        Ok(final_xml) => {
                                            println!("✅");

                                            // Compare XMLs by converting to JSON (easier comparison)
                                            print!("Step 4.6: Comparing structures... ");
                                            let json1 = MxMessage::from_xml(&original_xml)
                                                .unwrap()
                                                .to_json()
                                                .unwrap();
                                            let json2 = MxMessage::from_xml(&final_xml)
                                                .unwrap()
                                                .to_json()
                                                .unwrap();

                                            let val1: serde_json::Value =
                                                serde_json::from_str(&json1).unwrap();
                                            let val2: serde_json::Value =
                                                serde_json::from_str(&json2).unwrap();

                                            if val1 == val2 {
                                                println!("✅ IDENTICAL");
                                                println!(
                                                    "\n📊 Result: XML -> JSON -> XML preserves all data!"
                                                );
                                            } else {
                                                println!("⚠️  DIFFERENT");
                                                println!(
                                                    "\n📊 Result: Round-trip caused changes (may be expected)"
                                                );
                                            }

                                            std::fs::write("/tmp/test_roundtrip_xml_to_json_to_xml_original.xml", &original_xml).ok();
                                            std::fs::write(
                                                "/tmp/test_roundtrip_xml_to_json_to_xml_final.xml",
                                                &final_xml,
                                            )
                                            .ok();
                                            std::fs::write(
                                                "/tmp/test_roundtrip_xml_to_json.json",
                                                &json,
                                            )
                                            .ok();
                                            println!(
                                                "💾 Files saved to /tmp/test_roundtrip_xml_to_*"
                                            );
                                        }
                                        Err(e) => println!("❌ FAILED: {}", e),
                                    }
                                }
                                Err(e) => println!("❌ FAILED: {}", e),
                            }
                        }
                        Err(e) => println!("❌ FAILED: {}", e),
                    }
                }
                Err(e) => println!("❌ FAILED: {}", e),
            }
        }
        Err(e) => println!("❌ FAILED: {}", e),
    }
}
