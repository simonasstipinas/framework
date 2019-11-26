pub use crate::primitives::Gwei;

pub const JUSTIFICATION_BITS_LENGTH: usize = 4;
pub const SECONDS_PER_DAY: u64 = 86400;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub const FAR_FUTURE_EPOCH: u64 = u64::max_value(); // prideta
pub const SHUFFLE_ROUND_COUNT: u64 = 90; // prideta
pub type DepositContractTreeDepth = typenum::U32;
pub type JustificationBitsLength = typenum::U4;
