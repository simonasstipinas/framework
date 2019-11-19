pub use crate::primitives::Gwei;

pub const JUSTIFICATION_BITS_LENGTH: usize = 4;
pub const SECONDS_PER_DAY: u64 = 86400;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub const SLOTS_PER_EPOCH: u64 = 32; // prideta
pub const MAX_SEED_LOOKAHEAD: u64 = 4; // prideta
pub const FAR_FUTURE_EPOCH: u64 = 2^64-1; // prideta
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: u64 = 256; // prideta
pub const MAX_EFFECTIVE_BALANCE: Gwei = 32;
pub const SHUFFLE_ROUND_COUNT: u64 = 90;
pub type DepositContractTreeDepth = typenum::U32;
pub type JustificationBitsLength = typenum::U4;
