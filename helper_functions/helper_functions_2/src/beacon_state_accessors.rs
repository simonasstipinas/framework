use crate::error::Error;
use crate::predicates::is_active_validator;
use ethereum_types::H256;
use std::cmp::max;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::{Epoch, Gwei, Slot, ValidatorIndex, Domain, DomainType};

const SLOTS_PER_HISTORICAL_ROOT: u64 = 2 ^ 13;
const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 2 ^ 16;
const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 2 ^ 16;

pub fn get_block_root_at_slot<C: Config>(state: BeaconState<C>, slot: Slot) -> H256 {
    assert!(slot < state.slot && state.slot <= slot + SLOTS_PER_HISTORICAL_ROOT);
    let index = (slot % SLOTS_PER_HISTORICAL_ROOT) as usize;
    state.block_roots[index]
}

pub fn get_randao_mix<C: Config>(state: BeaconState<C>, epoch: Epoch) -> H256 {
    let index = (epoch % EPOCHS_PER_HISTORICAL_VECTOR) as usize;
    state.randao_mixes[index]
}

pub fn get_active_validator_indices<C: Config>(
    state: &BeaconState<C>,
    epoch: Epoch,
) -> Vec<ValidatorIndex> {
    let mut validators = Vec::<ValidatorIndex>::new();
    for (i, v) in state.validators.iter().enumerate() {
        if is_active_validator(v, epoch) {
            validators.push(i as ValidatorIndex)
        }
    }
    validators
}

pub fn get_current_epoch<C: Config>(state: &BeaconState<C>) -> Epoch {
    crate::misc::compute_epoch_at_slot(state.slot)
}

pub fn get_validator_churn_limit<C: Config>(state: BeaconState<C>) -> u64 {
    let active_validator_indices = get_active_validator_indices(&state, 8); // get_current_epoch
    let active_validator_count = active_validator_indices.len() as u64;
    max(MIN_PER_EPOCH_CHURN_LIMIT, active_validator_count) // CHURN_LIMIT_QUOTIENT
}

pub fn get_total_balance<C: Config>(state: BeaconState<C>, indices: Vec<ValidatorIndex>) -> Gwei {
    let mut balance: Gwei = 0;
    for (i, v) in state.validators.iter().enumerate() {
        if indices.contains(&(i as u64)) {
            balance += v.effective_balance;
        }
    }
    if balance > 1 {
        balance
    } else {
        1
    }
}

pub fn get_total_active_balance<C: Config>(state: BeaconState<C>) -> Gwei {
    let validators = get_active_validator_indices(&state, 8); // get_current_epoch
    get_total_balance(state, validators)
}

// TODO:
pub fn get_beacon_proposer_index<C: Config>(
    _state: &BeaconState<C>,
) -> Result<ValidatorIndex, Error> {
    Ok(0)
}

// TODO:
pub fn get_domain<C: Config>(
    _state: &BeaconState<C>,
    _domain_type: DomainType,
    _message_epoch: Option<Epoch>,
) -> Domain {
    0
}
