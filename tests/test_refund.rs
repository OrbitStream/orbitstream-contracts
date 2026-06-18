use orbitstream_contracts::{escrow::EscrowStatus, OrbitStream, OrbitStreamClient};
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, Env};

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
fn test_refund_after_timeout() {
    let (env, client, _buyer, _seller, escrow_id) = setup_with_escrow();

    // Advance ledger timestamp past timeout
    env.ledger().with_mut(|l| {
        l.timestamp = l.timestamp + 3601;
    });

    client.refund(&escrow_id);

    let escrow = client.get_escrow(&escrow_id);
    assert_eq!(escrow.status, EscrowStatus::Refunded);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_refund_before_timeout() {
    let (_env, client, _buyer, _seller, escrow_id) = setup_with_escrow();

    // Try to refund without advancing time
    client.refund(&escrow_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_refund_already_released() {
    let (env, client, _buyer, _seller, escrow_id) = setup_with_escrow();

    // Release first
    client.release(&escrow_id);

    // Advance time
    env.ledger().with_mut(|l| {
        l.timestamp = l.timestamp + 3601;
    });

    // Try refund on released escrow
    client.refund(&escrow_id);
}
