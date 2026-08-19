//! ISO 20022 allows an amount 18 significant digits. f64 carries 15.

use mx_message::document::camt_053_001_08::ActiveOrHistoricCurrencyAndAmount;

#[test]
fn an_eighteen_digit_amount_is_not_rounded() {
    let xml = r#"<Amt Ccy="KZT">123456789012345678</Amt>"#;
    let amt: ActiveOrHistoricCurrencyAndAmount =
        quick_xml::de::from_str(xml).expect("the amount parses");

    // Nothing here is exotic: 18 digits is what the standard permits, and the
    // nearest f64 to this one is 123456789012345680.
    assert_eq!(amt.value.to_string(), "123456789012345678");
}
