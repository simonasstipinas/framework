use crate::primitives::*;

pub const JUSTIFICATION_BITS_LENGTH: usize = 4;
pub const SECONDS_PER_DAY: u64 = 86400;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub type DepositContractTreeDepth = typenum::U32;
pub type JustificationBitsLength = typenum::U4;

// Gwei values
pub const MIN_DEPOSIT_AMOUNT: Gwei = 1000000000;
pub const MAX_EFFECTIVE_BALANCE: Gwei = 32000000000;
pub const EJECTION_BALANCE: Gwei = 16000000000;
pub const EFFECTIVE_BALANCE_INCREMENT: Gwei = 1000000000;

// State list lengths
pub const EPOCHS_PER_HISTORICAL_VECTOR:u64 = 0x1_0000;
pub const EPOCHS_PER_SLASHINGS_VECTOR:u64 = 0x2000;
pub const HISTORICAL_ROOTS_LIMIT:u64 = 0x100_0000;
pub const VALIDATOR_REGISTRY_LIMIT:u64 = 0x100_0000_0000;

// more constants
pub const FAR_FUTURE_EPOCH:Epoch = 2^64 - 1;
pub const BASE_REWARDS_PER_EPOCH:u64 = 4;
