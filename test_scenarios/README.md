# MX (ISO20022) Message Test Scenarios

This directory contains comprehensive test scenarios for ISO20022 MX messages, designed to validate the MXMessage library's ability to generate, parse, serialize, and process various real-world financial messaging use cases.

## Purpose

The test scenarios serve multiple critical functions:
- **Validation**: Ensure accurate generation and structure validation of ISO20022 messages
- **Compliance**: Verify adherence to ISO20022 standards and CBPR+ requirements
- **Coverage**: Test edge cases, minimal configurations, and complex multi-party scenarios
- **Real-world Testing**: Simulate actual business use cases from retail payments to complex correspondent banking
- **Migration Support**: Facilitate MT to MX message migration with equivalent scenarios

## Organization

Test scenarios are organized by message type, with each directory containing:
```
test_scenarios/
├── pain001/            # Customer Credit Transfer Initiation
│   ├── index.json      # List of available scenarios
│   ├── standard.json   # Basic scenario
│   ├── bulk_payment.json
│   └── ...
├── pacs008/            # FI to FI Customer Credit Transfer
│   ├── index.json
│   ├── standard.json
│   ├── cbpr_*.json     # CBPR+ compliant scenarios
│   └── ...
└── ...
```

Each scenario file uses a consistent JSON structure with:
- `variables`: Dynamic values using fake data generation
- `schema`: Message structure following ISO20022 schema

## Test Scenarios Overview

### Payment Initiation Messages (pain)

| Message Type | Scenario Name | Purpose | Use Case | Key Features | Completion |
|--------------|---------------|---------|----------|--------------|------------|
| **pain.001** | standard | Basic corporate payment request | Standard accounts payable | Single payment, basic fields | ✅ |
| **pain.001** | minimal | Minimum required fields | Testing, validation | Minimal viable payment | ✅ |
| **pain.001** | bulk_payment | Large batch processing | Corporate AP batch runs | Multiple transactions | ✅ |
| **pain.001** | cbpr_corporate_bulk | CBPR+ corporate bulk | Cross-border corporate batch | CBPR+ compliant batch | ✅ |
| **pain.001** | cbpr_payroll | CBPR+ payroll batch | Cross-border payroll | CBPR+ salary batch | ✅ |
| **pain.001** | cbpr_supplier_batch | CBPR+ supplier batch | Cross-border supplier payments | CBPR+ vendor batch | ✅ |
| **pain.001** | multi_currency | FX payment processing | International treasury ops | Multiple currencies | ✅ |
| **pain.001** | salary_payment | Payroll processing | Monthly salary runs | Employee references | ✅ |
| **pain.001** | scheduled_payment | Future-dated payments | Recurring obligations | Future execution dates | ✅ |
| **pain.001** | urgent_payment | Time-critical transfers | Emergency payments | Urgent priority | ✅ |
| **pain.001** | vendor_payment | Supplier settlements | Accounts payable | Invoice references | ✅ |
| **pain.008** | authorized_bulk_collection | Authorized collections | Pre-approved bulk debits | Authorization refs | ✅ |
| **pain.008** | general_direct_debit_basic | Standard direct debit | Basic collection request | Simple authorization | ✅ |
| **pain.008** | return_processing | Direct debit returns | Failed/disputed collections | Return codes | ✅ |
| **pain.008** | unauthorized_debit_processing | Unauthorized debits | Dispute handling | Dispute refs | ✅ |

### Payment Clearing and Settlement (pacs)

| Message Type | Scenario Name | Purpose | Use Case | Key Features | Completion |
|--------------|---------------|---------|----------|--------------|------------|
| **pacs.008** | standard | Basic credit transfer | Retail/commercial payment | Standard processing | ✅ |
| **pacs.008** | high_value | Large amount payment | Corporate acquisitions | Priority processing | ✅ |
| **pacs.008** | minimal | Minimum required fields | Testing baseline | Required fields only | ✅ |
| **pacs.008** | cbpr_business_payment | CBPR+ B2B payment | Cross-border B2B | UETR, purpose codes | ✅ |
| **pacs.008** | cbpr_charity_donation | CBPR+ charity payment | Charitable donations | Purpose codes | ✅ |
| **pacs.008** | cbpr_commission_payment | CBPR+ commission | Sales commissions | Commission details | ✅ |
| **pacs.008** | cbpr_crypto_settlement | CBPR+ crypto settlement | Cryptocurrency trading | Crypto references | ✅ |
| **pacs.008** | cbpr_dividend_distribution | CBPR+ dividends | Shareholder dividends | Dividend details | ✅ |
| **pacs.008** | cbpr_dividend_payment | CBPR+ dividend payment | Investment dividends | Investment refs | ✅ |
| **pacs.008** | cbpr_ecommerce_b2c | CBPR+ e-commerce | Online retail | Order references | ✅ |
| **pacs.008** | cbpr_education_international | CBPR+ intl education | International tuition | Student ID | ✅ |
| **pacs.008** | cbpr_education_payment | CBPR+ education | Domestic tuition | Enrollment refs | ✅ |
| **pacs.008** | cbpr_fees_payment | CBPR+ fees | Professional fees | Service details | ✅ |
| **pacs.008** | cbpr_gig_economy | CBPR+ gig economy | Freelance payments | Contractor ID | ✅ |
| **pacs.008** | cbpr_government_disbursement | CBPR+ govt payment | Government benefits | Benefit type | ✅ |
| **pacs.008** | cbpr_healthcare_payment | CBPR+ healthcare | Medical services | Claim references | ✅ |
| **pacs.008** | cbpr_insurance_cross_border | CBPR+ intl insurance | Cross-border insurance | Policy number | ✅ |
| **pacs.008** | cbpr_insurance_payment | CBPR+ insurance | Insurance premiums | Policy details | ✅ |
| **pacs.008** | cbpr_interest_payment | CBPR+ interest | Loan interest | Loan references | ✅ |
| **pacs.008** | cbpr_investment_payment | CBPR+ investment | Securities investment | Investment ID | ✅ |
| **pacs.008** | cbpr_loan_disbursement | CBPR+ loan | Loan proceeds | Loan agreement | ✅ |
| **pacs.008** | cbpr_pension_payment | CBPR+ pension | Retirement benefits | Beneficiary ID | ✅ |
| **pacs.008** | cbpr_person_to_person | CBPR+ P2P | Individual remittances | Purpose codes | ✅ |
| **pacs.008** | cbpr_real_estate | CBPR+ property | Property purchases | Escrow refs | ✅ |
| **pacs.008** | cbpr_remittance_corridor | CBPR+ remittance | Worker remittances | Corridor details | ✅ |
| **pacs.008** | cbpr_rent_payment | CBPR+ rent | Property rent | Lease references | ✅ |
| **pacs.008** | cbpr_royalty_payment | CBPR+ royalty | IP royalties | IP details | ✅ |
| **pacs.008** | cbpr_salary_payment | CBPR+ salary | Employee wages | Employee ID | ✅ |
| **pacs.008** | cbpr_sanctions_failure | CBPR+ sanctions test | Sanctions screening | Sanctioned entity | ✅ |
| **pacs.008** | cbpr_social_security | CBPR+ social security | Social benefits | Benefit type | ✅ |
| **pacs.008** | cbpr_stp_compliant | CBPR+ STP | Full automation | All structured | ✅ |
| **pacs.008** | cbpr_stp_enhanced | CBPR+ STP enhanced | Enhanced automation | Extended data | ✅ |
| **pacs.008** | cbpr_subscription_saas | CBPR+ SaaS | Software subscriptions | Subscription ID | ✅ |
| **pacs.008** | cbpr_supplier_payment | CBPR+ supplier | B2B settlements | Invoice refs | ✅ |
| **pacs.008** | cbpr_tax_payment | CBPR+ tax | Tax obligations | Tax ID | ✅ |
| **pacs.008** | cbpr_trade_finance | CBPR+ trade | L/C settlements | Trade docs | ✅ |
| **pacs.008** | cbpr_treasury_intercompany | CBPR+ intercompany | Corporate treasury | Company refs | ✅ |
| **pacs.008** | cbpr_utility_cross_border | CBPR+ intl utility | Cross-border utilities | Account number | ✅ |
| **pacs.008** | cbpr_utility_payment | CBPR+ utility | Domestic utilities | Meter number | ✅ |
| **pacs.008** | cbpr_validation_failure | CBPR+ validation test | Validation testing | Invalid codes | ✅ |
| **pacs.008** | correspondent_banking | Correspondent banking | FI relationships | Intermediaries | ✅ |
| **pacs.008** | cover_payment | Cover payment | Correspondent banking | Full chain | ✅ |
| **pacs.008** | duplicate_uetr | Duplicate UETR test | UETR tracking | Duplicate detection | ✅ |
| **pacs.008** | fx_conversion | Cross-currency | International trade | Exchange rates | ✅ |
| **pacs.008** | missing_lei_entity | Missing LEI test | Compliance testing | LEI validation | ✅ |
| **pacs.008** | regulatory_compliant | Regulatory compliance | Compliance testing | Full regulatory | ✅ |
| **pacs.008** | rejection | Payment rejection | Rejection handling | Rejection codes | ✅ |
| **pacs.008** | remit_basic | Basic remittance | Simple remittance | Basic remit info | ✅ |
| **pacs.008** | remit_structured | Structured remittance | Complex remittance | Structured data | ✅ |
| **pacs.008** | remittance_enhanced | Enhanced remittance | Detailed remittance | Extended info | ✅ |
| **pacs.008** | return | Payment return | Return processing | Return reasons | ✅ |
| **pacs.008** | stp | Straight-through | Automated processing | All structured | ✅ |
| **pacs.008** | treasury_payment | Treasury payment | Corporate treasury | Treasury ops | ✅ |
| **pacs.008** | unresolved_intermediary | Unresolved routing | Routing testing | Missing intermediary | ✅ |
| **pacs.003** | fi_direct_debit_basic | Basic direct debit | Standard collections | Mandate reference | ✅ |
| **pacs.003** | fi_direct_debit_cbpr | CBPR+ direct debit | Cross-border collections | CBPR+ compliant | ✅ |
| **pacs.003** | fi_direct_debit_multiple | Batch collections | Multiple debtor processing | Multiple transactions | ✅ |
| **pacs.003** | fi_direct_debit_recurring | Recurring collections | Subscriptions, utilities | Standing orders | ✅ |
| **pacs.003** | fi_direct_debit_return | Collection returns | Failed collections | Return reasons | ✅ |
| **pacs.003** | cbpr_insurance_collection | CBPR+ insurance | Cross-border premiums | CBPR+ compliant | ✅ |
| **pacs.003** | cbpr_subscription_collection | CBPR+ subscription | Cross-border subscriptions | CBPR+ compliant | ✅ |
| **pacs.003** | cbpr_utility_collection | CBPR+ utility | Cross-border utilities | CBPR+ compliant | ✅ |
| **pacs.009** | standard | Basic FI transfer | Bank-to-bank payment | Standard fields | ✅ |
| **pacs.009** | bank_transfer_cover | Cover transfer | FI cover payment | Cover for customer | ✅ |
| **pacs.009** | bank_transfer_non_cover | Non-cover transfer | Direct FI transfer | No cover required | ✅ |
| **pacs.009** | cbpr_cov_complex_routing | CBPR+ complex routing | Complex correspondent chain | Multiple hops | ✅ |
| **pacs.009** | cbpr_cov_compliance_enhanced | CBPR+ compliance | High compliance cover | Enhanced data | ✅ |
| **pacs.009** | cbpr_cov_rejection | CBPR+ rejection | Cross-border rejection | CBPR+ compliant | ✅ |
| **pacs.009** | cbpr_cov_return | CBPR+ return | Cross-border return | CBPR+ compliant | ✅ |
| **pacs.009** | cbpr_cov_standard | CBPR+ standard | Standard cover payment | Basic CBPR+ | ✅ |
| **pacs.009** | cbpr_serial_payment | CBPR+ serial | Serial payment chain | Multi-hop CBPR+ | ✅ |
| **pacs.009** | cov_mismatch | Cover mismatch | Mismatch testing | Amount mismatch | ✅ |
| **pacs.009** | fi_to_fi_transparency | FI transparency | Transparent routing | Full transparency | ✅ |
| **pacs.009** | minimal_return | Minimal return | Minimal fields return | Minimum return | ✅ |
| **pacs.009** | regulatory_reporting | Regulatory reporting | Compliance reporting | Regulatory data | ✅ |
| **pacs.009** | rejection_payment | Payment rejection | FI rejection | Rejected payment | ✅ |
| **pacs.009** | return | Payment return | FI-to-FI return | Return processing | ✅ |
| **pacs.009** | return_payment | Return payment | FI return payment | Return of funds | ✅ |
| **pacs.009** | return_simple | Simple return | Basic return | Minimal fields | ✅ |
| **pacs.009** | urgent_liquidity_transfer | Urgent liquidity | Emergency funding | Urgent transfer | ✅ |
| **pacs.002** | cheque_collection_advice | Cheque collection | Batch cheque processing | Multiple cheques | ✅ |
| **pacs.002** | duplicate_cheque_stop | Duplicate stop | Duplicate prevention | Stop duplicate | ✅ |
| **pacs.002** | foreign_cheque_collection | Foreign cheque | International clearing | FX conversion | ✅ |
| **pacs.002** | fraud_prevention_stop | Fraud stop | Fraud detection | Urgent stop | ✅ |
| **pacs.002** | lost_cheque_stop | Lost cheque | Lost cheque handling | Stop payment | ✅ |
| **pacs.002** | returned_cheque_advice | Returned cheque | Cheque bounce | Return fees | ✅ |
| **pacs.002** | single_cheque_advice | Single cheque | Express clearance | Large value | ✅ |
| **pacs.002** | stop_payment_accepted | Stop accepted | Successful stop | Confirmation | ✅ |
| **pacs.002** | stop_payment_pending | Stop pending | Processing stop | Pending status | ✅ |
| **pacs.002** | stop_payment_rejected | Stop rejected | Failed stop | Rejection reason | ✅ |

### Cash Management Messages (camt)

| Message Type | Scenario Name | Purpose | Use Case | Key Features | Completion |
|--------------|---------------|---------|----------|--------------|------------|
| **camt.052** | daily_balance_report | Daily balance | End-of-day balance | Daily position | ✅ |
| **camt.052** | intraday_liquidity_report | Liquidity report | Intraday liquidity | Real-time | ✅ |
| **camt.052** | multi_currency_balance | Multi-currency | Multiple currencies | Multi-CCY | ✅ |
| **camt.052** | negative_balance_report | Negative balance | Overdraft position | Negative position | ✅ |
| **camt.052** | real_time_position_update | Real-time position | Live position update | Current position | ✅ |
| **camt.052** | treasury_cash_sweep | Cash sweep | Treasury operations | Automated sweep | ✅ |
| **camt.053** | correspondent_banking | Correspondent statement | Nostro account | Correspondent | ✅ |
| **camt.053** | daily_account_statement | Daily statement | End-of-day statement | All transactions | ✅ |
| **camt.053** | high_volume_batch | High volume | Large transaction count | Batch processing | ✅ |
| **camt.053** | interim_statement_intraday | Interim statement | Intraday position | Real-time FX | ✅ |
| **camt.053** | repeated_sequence_issues | Duplicate test | Testing edge cases | Repeated items | ✅ |
| **camt.053** | simplified_statement | Simplified | Basic statement | Simple format | ✅ |
| **camt.053** | year_end_statement | Annual statement | Year-end summary | Annual summary | ✅ |
| **camt.054** | basic_credit_confirmation | Credit confirm | Standard credit notice | Account credited | ✅ |
| **camt.054** | basic_debit_confirmation | Debit confirm | Standard debit notice | Account debited | ✅ |
| **camt.054** | cbpr_credit_confirmation | CBPR+ credit | Cross-border credit | CBPR+ compliant | ✅ |
| **camt.054** | cbpr_debit_confirmation | CBPR+ debit | Cross-border debit | CBPR+ compliant | ✅ |
| **camt.054** | direct_debit_confirmation | DD confirm | DD collection notice | DD executed | ✅ |
| **camt.054** | dividend_payment | Dividend credit | Dividend receipt | Dividend credited | ✅ |
| **camt.054** | fee_debit_confirmation | Fee debit | Fee charge notice | Fees debited | ✅ |
| **camt.054** | fx_transaction_debit | FX debit | FX trade debit | FX settlement | ✅ |
| **camt.054** | incoming_wire_transfer | Wire credit | Incoming wire | Wire received | ✅ |
| **camt.054** | interest_credit | Interest credit | Interest payment | Interest credited | ✅ |
| **camt.054** | refund_credit | Refund credit | Refund receipt | Refund credited | ✅ |
| **camt.054** | standing_order_debit | SO debit | Recurring debit | SO executed | ✅ |
| **camt.056** | cbpr_cancellation_request | CBPR+ cancel | Cross-border cancel | CBPR+ compliant | ✅ |
| **camt.056** | compliance_hold_cancellation | Compliance hold | Regulatory freeze | Enhanced DD | ✅ |
| **camt.056** | fi_cancellation_request | FI cancel | Bank-to-bank cancel | Agent error | ✅ |
| **camt.056** | fraud_prevention_cancellation | Fraud cancel | Security incident | Suspected fraud | ✅ |
| **camt.056** | regulatory_compliance_cancellation | Compliance cancel | Sanctions hit | AML alert | ✅ |
| **camt.056** | request_cancellation | Basic cancel | Payment cancellation | Customer request | ✅ |
| **camt.056** | system_error_cancellation | System error | Technical failure | Batch error | ✅ |
| **camt.056** | urgent_cancellation | Urgent cancel | Same day cancel | Duplicate payment | ✅ |
| **camt.056** | wrong_beneficiary_cancellation | Wrong beneficiary | Incorrect account | Account closed | ✅ |
| **camt.029** | answer_cancellation | Cancel accepted | Successful cancel | Cancel confirmed | ✅ |
| **camt.029** | answer_inquiry_response | Inquiry response | Payment status | Payment found | ✅ |
| **camt.029** | answer_pending_investigation | Investigation pending | Fraud investigation | Under review | ✅ |
| **camt.029** | answer_rejection | Cancel rejected | Failed cancel | Already executed | ✅ |
| **camt.029** | cancellation_accepted | Cancel accepted | Successful cancel | Confirmed | ✅ |
| **camt.029** | cancellation_rejected | Cancel rejected | Failed cancel | Denied | ✅ |
| **camt.029** | cbpr_cancellation_response | CBPR+ response | Cross-border response | CBPR+ compliant | ✅ |
| **camt.029** | inquiry_response | Inquiry response | Query answer | Information | ✅ |
| **camt.029** | no_payment_found | Not found | Not found response | Payment not located | ✅ |
| **camt.029** | partial_cancellation | Partial cancel | Partial amount | Partial cancellation | ✅ |
| **camt.027** | cbpr_cancellation | CBPR+ cancel | Cross-border cancel | CBPR+ compliant | ✅ |
| **camt.027** | cbpr_inquiry | CBPR+ inquiry | Payment inquiry | CBPR+ compliant | ✅ |
| **camt.028** | cbpr_payment_response | CBPR+ status | Transparency response | Full tracking | ✅ |
| **camt.028** | regulatory_notification | Regulatory notice | Compliance update | New requirements | ✅ |
| **camt.028** | settlement_instructions_update | SSI update | Nostro change | Account change | ✅ |
| **camt.028** | system_maintenance_notice | Maintenance | Scheduled downtime | Maintenance window | ✅ |
| **camt.025** | central_bank_rate_notification | CB rates | Policy rate change | CB notification | ✅ |
| **camt.025** | deposit_rate_change | Deposit rates | Savings rate update | Rate notification | ✅ |
| **camt.025** | fx_rate_update | FX rates | Exchange rate change | FX notification | ✅ |
| **camt.025** | loan_rate_adjustment | Loan rates | Lending rate change | Rate notification | ✅ |
| **camt.025** | multi_product_rate_change | Multi rates | Various rate changes | Multiple updates | ✅ |
| **camt.057** | expected_incoming_funds | Expected funds | Batch incoming | Multiple payments | ✅ |
| **camt.057** | fx_settlement_notice | FX settlement | Currency trade | Two-way FX | ✅ |
| **camt.057** | securities_settlement_notice | Securities | DVP settlement | Bond settlement | ✅ |
| **camt.057** | single_payment_notice | Single payment | Wire transfer | Large value | ✅ |
| **camt.060** | interim_report_request | Interim request | Intraday statement | Request interim | ✅ |
| **camt.060** | multi_account_request | Multi-account | Multiple accounts | Request multiple | ✅ |
| **camt.060** | statement_request_basic | Basic request | Account statement | Request statement | ✅ |

## Current Test Status

| Component | Status | Details |
|-----------|--------|---------|
| **Sample Generator** | ✅ | All scenarios generate valid samples |
| **Structure Validation** | ✅ | Message structure validation passes |
| **JSON Round Trip Test** | ✅ | Full JSON serialization/deserialization |
| **MT to MX Migration** | ✅ | 99.4% complete (168/169 scenarios) |

### Test Coverage
- **Total Scenarios**: 168
- **Message Types**: 16 (pain.001-camt.060)
- **CBPR+ Scenarios**: 58 (cross-border payment compliance)
- **Success Rate**: 100%

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Passed/Complete |
| ❌ | Failed |
| ⚠️ | Partial/Warning |
| ⏳ | In Progress |
| ❔ | Not Started |

## Running Tests

### Quick Start - Generate Sample Messages

```rust
use mx_message::sample::generate_sample;

// Generate standard scenario
let msg: serde_json::Value = generate_sample("pacs008", None)?;

// Generate specific scenario  
let msg: serde_json::Value = generate_sample("pacs008", Some("high_value"))?;
```

### Test All Scenarios
```bash
# Run all scenarios with default 10 samples each
cargo test round_trip_scenarios -- --nocapture

# Run with specific sample count
TEST_SAMPLE_COUNT=100 cargo test round_trip_scenarios -- --nocapture
```

### Test Specific Message Type
```bash
# Test all pacs.008 scenarios
TEST_MESSAGE_TYPE=pacs.008 cargo test round_trip_scenarios -- --nocapture

# Test all pain.001 scenarios
TEST_MESSAGE_TYPE=pain.001 cargo test round_trip_scenarios -- --nocapture
```

### Test Specific Scenario
```bash
# Test a specific scenario
TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=cbpr_business_payment cargo test round_trip_scenarios -- --nocapture
```

### Debug Mode
```bash
# Enable debug output for detailed error information
TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=high_value TEST_DEBUG=1 cargo test round_trip_scenarios -- --nocapture

# Debug with single sample for focused troubleshooting
TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=high_value TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test round_trip_scenarios -- --nocapture

# Stop on first failure
TEST_MESSAGE_TYPE=pacs.008 TEST_DEBUG=1 TEST_STOP_ON_FAILURE=1 cargo test round_trip_scenarios -- --nocapture
```

### Environment Variables

| Variable | Purpose | Default | Example |
|----------|---------|---------|---------|
| `TEST_MESSAGE_TYPE` | Test specific message type | All | `pacs.008` |
| `TEST_SCENARIO` | Test specific scenario | All | `cbpr_business_payment` |
| `TEST_DEBUG` | Enable debug output | Disabled | `1` |
| `TEST_SAMPLE_COUNT` | Number of samples per scenario | 10 | `100` |
| `TEST_STOP_ON_FAILURE` | Stop on first failure | Continue | `1` |

## Scenario Format

```json
{
    "variables": {
        "msg_id": {"fake": ["uuid"]},
        "amount": {"fake": ["f64", 1000.0, 50000.0]},
        "currency": {"pick": ["EUR", "USD", "GBP"]},
        "sender_bic": {"fake": ["bic"]},
        "reference": {"cat": ["REF", {"fake": ["i64", 1000, 9999]}]}
    },
    "schema": {
        "GrpHdr": {
            "MsgId": {"var": "msg_id"},
            "CreDtTm": {"fake": ["iso8601_datetime"]},
            "NbOfTxs": "1"
        },
        "CdtTrfTxInf": {
            "PmtId": {
                "InstrId": {"var": "reference"},
                "EndToEndId": {"cat": ["E2E-", {"var": "reference"}]}
            }
        }
    }
}
```

## Variable Generation Types

### Fake Data Generation
- `{"fake": ["bic"]}` - Generate BIC code (e.g., DEUTDEFF123)
- `{"fake": ["iban", "DE"]}` - Generate IBAN for country
- `{"fake": ["uuid"]}` - Generate UUID
- `{"fake": ["lei"]}` - Generate LEI (Legal Entity Identifier)
- `{"fake": ["company_name"]}` - Generate company name
- `{"fake": ["full_name"]}` - Generate person name
- `{"fake": ["i64", min, max]}` - Generate integer in range
- `{"fake": ["f64", min, max]}` - Generate float in range
- `{"fake": ["iso8601_datetime"]}` - Generate ISO datetime
- `{"fake": ["date", "%Y-%m-%d", "-5d", "+5d"]}` - Generate date with format and range
- `{"fake": ["enum", "val1", "val2", ...]}` - Pick random from list

### Concatenation
- `{"cat": ["PREFIX-", {"fake": ["i64", 1, 100]}]}` - Concatenate values
- `{"cat": ["REF-", {"var": "reference"}, "-END"]}` - Multiple concatenation

### Random Selection
- `{"pick": ["EUR", "USD", "GBP"]}` - Pick random value from array

### Variable Reference
- `{"var": "amount"}` - Reference a previously defined variable

## Field Naming Convention

MX messages use specific field naming patterns:
- **Group Headers**: `GrpHdr`
- **Amount fields**: Nested structure with `Amt.Value` and `Amt.Ccy`
- **Capitalized field names**: `PmtInf`, `CdtTrfTxInf`, etc.
- **Arrays**: Can be either single objects or arrays depending on multiplicity

## Test Process

For each scenario, the round-trip test performs:

1. **Generation**: Create sample messages using scenario configuration with fake data
2. **Parsing**: Generate JSON structure from scenario definition
3. **Structure Validation**: Verify required fields based on message type
4. **JSON Round Trip**: Serialize to JSON and back to ensure data integrity
5. **CBPR+ Validation**: Check CBPR+ specific requirements (UETR, purpose codes)
6. **Comparison**: Verify original and round-trip results match exactly

## Adding New Scenarios

1. Create a new JSON file in the appropriate message type directory:
   ```json
   {
     "variables": {
       "msg_id": {"fake": ["uuid"]},
       "amount": {"fake": ["f64", 1000.0, 50000.0]},
       "sender_bic": {"fake": ["bic"]}
     },
     "schema": {
       "GrpHdr": {
         "MsgId": {"var": "msg_id"},
         "CreDtTm": {"fake": ["iso8601_datetime"]},
         "NbOfTxs": "1"
       },
       "CdtTrfTxInf": {
         ...
       }
     }
   }
   ```

2. Add the scenario name to the `index.json` file

3. Update this README with the scenario details

4. Test the new scenario:
   ```bash
   TEST_MESSAGE_TYPE=pacs.008 TEST_SCENARIO=your_new_scenario cargo test round_trip_scenarios -- --nocapture
   ```

## Troubleshooting

### Common Issues

1. **Missing Fields**: Check ISO20022 schema requirements for the message type
2. **Structure Validation Failures**: Review expected fields in round_trip_test.rs
3. **Round Trip Mismatches**: Ensure proper JSON structure in scenario file
4. **CBPR+ Validation**: Ensure UETR is present in PmtId for CBPR+ scenarios

### Debug Process

1. Run with debug mode to see detailed error messages
2. Use single sample count to focus on specific failure
3. Check generated JSON structure for missing fields
4. Review validation logic in test for message type

## MT to MX Migration Status

The test scenarios support migration from SWIFT MT to ISO20022 MX messages:

| MT Type | MX Type | Scenarios | Status |
|---------|---------|-----------|--------|
| MT101 | pain.001 | 12 | ✅ Complete |
| MT103 | pacs.008 | 54 | ✅ Complete |
| MT104 | pacs.003 | 8 | ✅ Complete |
| MT107 | pain.008 | 4 | ✅ Complete |
| MT110/111/112 | pacs.002 | 10 | ✅ Complete |
| MT192/292 | camt.056 | 9 | ✅ Complete |
| MT196/296 | camt.029 | 10 | ✅ Complete |
| MT199/299 | camt.027/028 | 6 | ✅ Complete |
| MT202/205 | pacs.009 | 18 | ✅ Complete |
| MT210 | camt.057 | 4 | ✅ Complete |
| MT900/910 | camt.054 | 12 | ✅ Complete |
| MT920 | camt.060 | 3 | ✅ Complete |
| MT935 | camt.025 | 5 | ✅ Complete |
| MT940/950 | camt.053 | 7 | ✅ Complete |
| MT941/942 | camt.052 | 6 | ✅ Complete |

## Recent Updates

### January 2025
- Completed all remaining MT103 to pacs.008 scenarios (10 scenarios)
- Completed all remaining MT202 to pacs.009 scenarios (3 scenarios)
- Achieved 99.4% MT to MX migration completion (168/169 scenarios)
- Implemented comprehensive round-trip testing for all message types
- Added support for CBPR+ compliant scenarios with UETR tracking

## Contributing

When adding new scenarios:
1. Follow existing patterns and naming conventions
2. Include realistic data ranges
3. Add CBPR+ compliant variants where applicable
4. Update the tracker in `MT_TO_MX_SCENARIO_TRACKER.md`
5. Test that scenarios generate valid messages
6. Ensure all required fields are present for the message type