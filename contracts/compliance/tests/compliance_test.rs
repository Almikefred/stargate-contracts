use compliance::{ComplianceContract, ComplianceContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn block_and_clear_address() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let id = env.register_contract(None, ComplianceContract);
    let client = ComplianceContractClient::new(&env, &id);
    client.initialize(&admin);
    client.allow_address(&admin, &payer);
    assert!(client.is_allowed(&payer));
    client.block_address(&admin, &payer);
    assert!(!client.is_allowed(&payer));
    client.clear_address(&admin, &payer);
    assert!(client.is_allowed(&payer));
}

#[test]
fn allow_address_mutation_succeeds_after_unpause() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let address1 = Address::generate(&env);
    let address2 = Address::generate(&env);
    let id = env.register_contract(None, ComplianceContract);
    let client = ComplianceContractClient::new(&env, &id);
    client.initialize(&admin);

    // Allow address1 before pausing
    client.allow_address(&admin, &address1);
    assert!(client.is_allowed(&address1));

    // Pause then unpause
    client.pause(&admin);
    client.unpause(&admin);

    // Allow address2 should now work
    client.allow_address(&admin, &address2);
    assert!(client.is_allowed(&address2));
}

#[test]
fn block_address_mutation_succeeds_after_unpause() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let address = Address::generate(&env);
    let id = env.register_contract(None, ComplianceContract);
    let client = ComplianceContractClient::new(&env, &id);
    client.initialize(&admin);

    // Allow address first
    client.allow_address(&admin, &address);
    assert!(client.is_allowed(&address));

    // Pause then unpause
    client.pause(&admin);
    client.unpause(&admin);

    // Block address should now work
    client.block_address(&admin, &address);
    assert!(!client.is_allowed(&address));
}

#[test]
fn clear_address_mutation_succeeds_after_unpause() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let address = Address::generate(&env);
    let id = env.register_contract(None, ComplianceContract);
    let client = ComplianceContractClient::new(&env, &id);
    client.initialize(&admin);

    // Allow and block address first
    client.allow_address(&admin, &address);
    client.block_address(&admin, &address);
    assert!(!client.is_allowed(&address));

    // Pause then unpause
    client.pause(&admin);
    client.unpause(&admin);

    // Clear address should now work
    client.clear_address(&admin, &address);
    assert!(client.is_allowed(&address));
}

#[test]
fn read_only_queries_not_blocked_by_pause() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let allowed_address = Address::generate(&env);
    let blocked_address = Address::generate(&env);
    let id = env.register_contract(None, ComplianceContract);
    let client = ComplianceContractClient::new(&env, &id);
    client.initialize(&admin);

    // Setup: allow one address, block another
    client.allow_address(&admin, &allowed_address);
    client.block_address(&admin, &blocked_address);

    // Pause the contract
    client.pause(&admin);

    // Read-only queries should still work
    assert!(client.is_allowed(&allowed_address));
    assert!(!client.is_allowed(&blocked_address));

    let unrelated_address = Address::generate(&env);
    assert!(!client.is_allowed(&unrelated_address));
}
