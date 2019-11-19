pub const JUSTIFICATION_BITS_LENGTH: usize = 4;
pub const SECONDS_PER_DAY: u64 = 86400;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub const SLOTS_PER_EPOCH: u64 = 32; // prideta
pub const MAX_SEED_LOOKAHEAD: u64 = 4; // prideta
pub type DepositContractTreeDepth = typenum::U32;
pub type JustificationBitsLength = typenum::U4;

pub const SLOTS_PER_HISTORICAL_ROOT: u64 = 8192;
pub const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 65536;
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 65536;
pub const MIN_SEED_LOOKAHEAD: u64 = 1;
pub const MAX_COMMITTEES_PER_SLOT: u64 = 64;

pub const DOMAIN_BEACON_ATTESTER: u32 = 1;
pub const DOMAIN_BEACON_PROPOSER: u32 = 0;