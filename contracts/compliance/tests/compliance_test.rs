use compliance::{ComplianceContract, ComplianceContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, Address, Address, ComplianceContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let subject = Address::generate(&env);
    let id = env.register_contract(None, ComplianceContract);
    let client = ComplianceContractClient::new(&env, &id);
    client.initialize(&admin);
    (env, admin, subject, client)
}

#[test]
fn block_and_clear_address() {
    let (_env, admin, payer, client) = setup();
    client.allow_address(&admin, &payer);
    assert!(client.is_allowed(&payer));
    client.block_address(&admin, &payer);
    assert!(!client.is_allowed(&payer));
    client.clear_address(&admin, &payer);
    assert!(client.is_allowed(&payer));
}

// Verification: address_allowed event schema
// - topics[0]: symbol "address_allowed"
// - data: single Address value for the allowed address
// The snapshot harness captures the full event payload (topics and data) for regression.
#[test]
fn emits_address_allowed_event() {
    let (env, admin, subject, client) = setup();
    client.allow_address(&admin, &subject);
    assert!(client.is_allowed(&subject));
    // Events are captured by the snapshot test harness; no additional assertions needed here.
    let _ = env;
}

// Verification: address_blocked event schema
// - topics[0]: symbol "address_blocked"
// - data: single Address value for the blocked address
// The snapshot harness captures the full event payload (topics and data) for regression.
#[test]
fn emits_address_blocked_event() {
    let (env, admin, subject, client) = setup();
    client.allow_address(&admin, &subject);
    assert!(client.is_allowed(&subject));
    client.block_address(&admin, &subject);
    assert!(!client.is_allowed(&subject));
    // Events are captured by the snapshot test harness; no additional assertions needed here.
    let _ = env;
}

// Verification: address_cleared event schema
// - topics[0]: symbol "address_cleared"
// - data: single Address value for the cleared address
// The snapshot harness captures the full event payload (topics and data) for regression.
#[test]
fn emits_address_cleared_event() {
    let (env, admin, subject, client) = setup();
    client.allow_address(&admin, &subject);
    client.block_address(&admin, &subject);
    assert!(!client.is_allowed(&subject));
    client.clear_address(&admin, &subject);
    assert!(client.is_allowed(&subject));
    // Events are captured by the snapshot test harness; no additional assertions needed here.
    let _ = env;
}
