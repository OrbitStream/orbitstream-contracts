use orbitstream_contracts::{escrow::EscrowStatus, OrbitStream, OrbitStreamClient};
use soroban_sdk::{testutils::Address as _, testutils::Ledger, token, Address, Env};

fn setup_with_escrow() -> (
    Env,
    OrbitStreamClient<'static>,
    Address,
    Address,
    Address,
    Address,
    u64,
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

    let stellar_client = token::StellarAssetClient::new(&env, &token);
    stellar_client.mint(&buyer, &1000);

    let escrow_id = client.create_escrow(&buyer, &seller, &token, &1000, &3600);
    (env, client, buyer, seller, token, contract_id, escrow_id)
}

#[test]
fn test_release_escrow() {
    let (env, client, _buyer, seller, token, contract_id, escrow_id) = setup_with_escrow();

    client.release(&escrow_id);

    let escrow = client.get_escrow(&escrow_id);
    assert_eq!(escrow.status, EscrowStatus::Released);

    let token_client = token::Client::new(&env, &token);
    assert_eq!(token_client.balance(&seller), 1000);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_release_already_released() {
    let (_env, client, _buyer, _seller, _token, _contract_id, escrow_id) = setup_with_escrow();

    client.release(&escrow_id);
    client.release(&escrow_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_release_nonexistent() {
    let (_env, client, _buyer, _seller, _token, _contract_id, _escrow_id) = setup_with_escrow();
    client.release(&999);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_release_after_refund() {
    let (env, client, _buyer, _seller, _token, _contract_id, escrow_id) = setup_with_escrow();

    env.ledger().with_mut(|l| {
        l.timestamp = l.timestamp + 3601;
    });

    client.refund(&escrow_id);
    client.release(&escrow_id);
}
