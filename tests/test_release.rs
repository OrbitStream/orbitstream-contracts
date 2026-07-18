use orbitstream_contracts::{escrow::EscrowStatus, OrbitStream, OrbitStreamClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup_with_escrow() -> (Env, OrbitStreamClient<'static>, Address, Address, u64) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, OrbitStream);
    let client = OrbitStreamClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let token = Address::generate(&env);

    let escrow_id = client.create_escrow(&buyer, &seller, &token, &1000, &3600);
    (env, client, buyer, seller, escrow_id)
}

#[test]
fn test_release_escrow() {
    let (_env, client, _buyer, _seller, escrow_id) = setup_with_escrow();

    client.release(&escrow_id);

    let escrow = client.get_escrow(&escrow_id);
    assert_eq!(escrow.status, EscrowStatus::Released);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_release_already_released() {
    let (_env, client, _buyer, _seller, escrow_id) = setup_with_escrow();

    client.release(&escrow_id);
    client.release(&escrow_id); // should fail
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_release_nonexistent() {
    let (_env, client, _buyer, _seller, _escrow_id) = setup_with_escrow();
    client.release(&999);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_release_after_refund() {
    let (env, client, _buyer, _seller, escrow_id) = setup_with_escrow();

    // Advance time past timeout
    env.ledger().with_mut(|l| {
        l.timestamp = l.timestamp + 3601;
    });

    // Refund first
    client.refund(&escrow_id);

    // Try to release refunded escrow
    client.release(&escrow_id);
}
