use orbitstream_contracts::{escrow::EscrowStatus, OrbitStream, OrbitStreamClient};
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup() -> (
    Env,
    OrbitStreamClient<'static>,
    Address,
    Address,
    Address,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, OrbitStream);
    let client = OrbitStreamClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token = token_contract.address();

    (env, client, buyer, seller, token, admin, contract_id)
}

#[test]
fn test_create_escrow() {
    let (env, client, buyer, seller, token, _admin, contract_id) = setup();

    let stellar_client = token::StellarAssetClient::new(&env, &token);
    stellar_client.mint(&buyer, &1000);

    let token_client = token::Client::new(&env, &token);
    let escrow_id = client.create_escrow(&buyer, &seller, &token, &1000, &3600);
    assert_eq!(escrow_id, 1);

    let escrow = client.get_escrow(&escrow_id);
    assert_eq!(escrow.id, 1);
    assert_eq!(escrow.buyer, buyer);
    assert_eq!(escrow.seller, seller);
    assert_eq!(escrow.amount, 1000);
    assert_eq!(escrow.status, EscrowStatus::Active);

    assert_eq!(token_client.balance(&buyer), 0);
    assert_eq!(token_client.balance(&contract_id), 1000);
}

#[test]
fn test_create_escrow_increments_id() {
    let (env, client, buyer, seller, token, _admin, _contract_id) = setup();

    let stellar_client = token::StellarAssetClient::new(&env, &token);
    stellar_client.mint(&buyer, &300);

    let id1 = client.create_escrow(&buyer, &seller, &token, &100, &3600);
    let id2 = client.create_escrow(&buyer, &seller, &token, &200, &7200);
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_create_escrow_zero_amount() {
    let (_env, client, buyer, seller, token, _admin, _contract_id) = setup();
    client.create_escrow(&buyer, &seller, &token, &0, &3600);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_create_escrow_zero_timeout() {
    let (_env, client, buyer, seller, token, _admin, _contract_id) = setup();
    client.create_escrow(&buyer, &seller, &token, &1000, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_create_escrow_same_buyer_seller() {
    let (_env, client, buyer, _seller, token, _admin, _contract_id) = setup();
    client.create_escrow(&buyer, &buyer, &token, &1000, &3600);
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn test_create_escrow_insufficient_balance() {
    let (env, client, buyer, seller, token, _admin, _contract_id) = setup();

    let stellar_client = token::StellarAssetClient::new(&env, &token);
    stellar_client.mint(&buyer, &500);

    client.create_escrow(&buyer, &seller, &token, &1000, &3600);
}
