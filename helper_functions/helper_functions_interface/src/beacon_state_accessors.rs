use types::{beacon_state::BeaconState, config::Config, primitives::*};

use crate::error::Error;

pub fn get_current_epoch<C: Config>(_state: &BeaconState<C>) -> Epoch {
    0
}

pub fn get_previous_epoch<C: Config>(_state: &BeaconState<C>) -> Epoch {
    0
}

pub fn get_block_root<C: Config>(_state: &BeaconState<C>, _epoch: Epoch) -> Result<H256, Error> {
    Err(Error::IndexOutOfRange)
}

pub fn get_block_root_at_slot<C: Config>(
    _state: &BeaconState<C>,
    _slot: Slot,
) -> Result<H256, Error> {
    Err(Error::IndexOutOfRange)
}

pub fn get_randao_mix<C: Config>(_state: &BeaconState<C>, _epoch: Epoch) -> Result<H256, Error> {
    Err(Error::IndexOutOfRange)
}

pub fn get_active_validator_indices<C: Config>(
    _state: &BeaconState<C>,
    _epoch: Epoch,
) -> Vec<ValidatorIndex> {
    [].to_vec()
}

pub fn get_validator_churn_limit<C: Config>(_state: &BeaconState<C>) -> Result<u64, Error> {
    Ok(1)
}

pub fn get_seed<C: Config>(_state: &BeaconState<C>, _epoch: &Epoch, _domain_type: &u64) -> H256 {
    H256::from([0; 32])
}

pub fn get_committee_count_at_slot<C: Config>(
    _state: &BeaconState<C>,
    _slot: &Slot,
) -> Result<u64, Error> {
    Ok(1)
}

pub fn get_beacon_committee<C: Config>(
    _state: &BeaconState<C>,
    _slot: &Slot,
    _index: u64,
) -> Vec<ValidatorIndex> {
    [].to_vec()
}

pub fn get_beacon_proposer_index<C: Config>(_state: &BeaconState<C>) -> ValidatorIndex {
    0
}

pub fn get_total_balance<C: Config>(
    _state: &BeaconState<C>,
    _indices: &[ValidatorIndex],
) -> Result<u64, Error> {
    Ok(1)
}

pub fn get_total_active_balance<C: Config>(_state: &BeaconState<C>) -> Result<u64, Error> {
    Ok(1)
}

pub fn get_domain<C: Config>(
    _state: &BeaconState<C>,
    _domain_type: &u64,
    _message_epoch: Option<&Epoch>,
) -> u64 {
    0
}

//pub fn get_indexed_attestation<C: Config>(_state: &BeaconState<C>, attestation: &Attestation<C>) -> IndexedAttestation<C> {
//}

//get_attesting_indices
