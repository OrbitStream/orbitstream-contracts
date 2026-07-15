use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    EscrowNotFound = 1,
    Unauthorized = 2,
    EscrowAlreadySettled = 3,
    TimeoutNotReached = 4,
    InvalidAmount = 5,
    InvalidTimeout = 6,
    InvalidAddresses = 7,
}
