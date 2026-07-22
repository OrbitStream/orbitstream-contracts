#![no_std]

pub mod errors;
pub mod escrow;
pub mod events;
pub mod storage;

use errors::ContractError;
use escrow::{Escrow, EscrowStatus};
use events::*;
use soroban_sdk::{contract, contractimpl, token, Address, Env};
use storage::{get_escrow, get_next_escrow_id, increment_escrow_id, store_escrow};

#[contract]
pub struct OrbitStream;

#[contractimpl]
impl OrbitStream {
    /// Create a new escrow — buyer deposits funds locked until timeout or release.
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        token: Address,
        amount: u128,
        timeout_seconds: u64,
    ) -> Result<u64, ContractError> {
        buyer.require_auth();

        if amount == 0 {
            return Err(ContractError::InvalidAmount);
        }
        if timeout_seconds == 0 {
            return Err(ContractError::InvalidTimeout);
        }
        if buyer == seller {
            return Err(ContractError::InvalidAddresses);
        }

        let escrow_id = get_next_escrow_id(&env);
        increment_escrow_id(&env);

        let now = env.ledger().timestamp();
        let timeout_at = now
            .checked_add(timeout_seconds)
            .ok_or(ContractError::InvalidTimeout)?;
        let escrow = Escrow {
            id: escrow_id,
            buyer: buyer.clone(),
            seller: seller.clone(),
            token: token.clone(),
            amount,
            status: EscrowStatus::Active,
            created_at: now,
            timeout_at,
        };

        let token_client = token::Client::new(&env, &token);
        let amount_i128 = amount as i128;
        if token_client.balance(&buyer) < amount_i128 {
            return Err(ContractError::TransferFailed);
        }
        token_client.transfer(&buyer, &env.current_contract_address(), &amount_i128);

        store_escrow(&env, &escrow);

        emit_escrow_created(
            &env,
            EscrowCreatedEvent {
                escrow_id,
                buyer,
                seller,
                token,
                amount,
                timeout_at: escrow.timeout_at,
            },
        );

        Ok(escrow_id)
    }

    /// Release escrowed funds to seller. Requires seller auth.
    pub fn release(env: Env, escrow_id: u64) -> Result<(), ContractError> {
        let mut escrow = get_escrow(&env, escrow_id).ok_or(ContractError::EscrowNotFound)?;

        if escrow.status != EscrowStatus::Active {
            return Err(ContractError::EscrowAlreadySettled);
        }

        escrow.seller.require_auth();

        escrow.status = EscrowStatus::Released;

        let token_client = token::Client::new(&env, &escrow.token);
        let amount_i128 = escrow.amount as i128;
        if token_client.balance(&env.current_contract_address()) < amount_i128 {
            return Err(ContractError::TransferFailed);
        }
        token_client.transfer(
            &env.current_contract_address(),
            &escrow.seller,
            &amount_i128,
        );

        store_escrow(&env, &escrow);

        emit_escrow_released(
            &env,
            EscrowReleasedEvent {
                escrow_id,
                seller: escrow.seller.clone(),
                amount: escrow.amount,
            },
        );

        Ok(())
    }

    /// Refund escrowed funds to buyer. Only valid after timeout.
    pub fn refund(env: Env, escrow_id: u64) -> Result<(), ContractError> {
        let mut escrow = get_escrow(&env, escrow_id).ok_or(ContractError::EscrowNotFound)?;

        if escrow.status != EscrowStatus::Active {
            return Err(ContractError::EscrowAlreadySettled);
        }

        let now = env.ledger().timestamp();
        if now < escrow.timeout_at {
            return Err(ContractError::TimeoutNotReached);
        }

        escrow.buyer.require_auth();

        escrow.status = EscrowStatus::Refunded;

        let token_client = token::Client::new(&env, &escrow.token);
        let amount_i128 = escrow.amount as i128;
        if token_client.balance(&env.current_contract_address()) < amount_i128 {
            return Err(ContractError::TransferFailed);
        }
        token_client.transfer(&env.current_contract_address(), &escrow.buyer, &amount_i128);

        store_escrow(&env, &escrow);

        emit_escrow_refunded(
            &env,
            EscrowRefundedEvent {
                escrow_id,
                buyer: escrow.buyer.clone(),
                amount: escrow.amount,
            },
        );

        Ok(())
    }

    /// Read-only: get escrow details.
    pub fn get_escrow(env: Env, escrow_id: u64) -> Result<Escrow, ContractError> {
        get_escrow(&env, escrow_id).ok_or(ContractError::EscrowNotFound)
    }
}
