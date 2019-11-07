use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::{Epoch, Slot, ValidatorIndex, Version, H256};
use crate::error::Error;

//ok
pub fn compute_shuffled_index<C: Config>(
    _index: ValidatorIndex,
    _index_count: u64,
    _seed: &H256,
) -> Result<ValidatorIndex, Error> {
    Ok(0)
}

//ok
pub fn compute_proposer_index<C: Config>(
    _state: &BeaconState<C>,
    _indices: &[ValidatorIndex],
    _seed: &H256,
) -> Result<ValidatorIndex, Error> {
    Ok(0)
}

//ok
pub fn compute_committee<'a>(
    _indices: &'a [ValidatorIndex],
    _seed: &H256,
    _index: u64,
    _count: u64,
) -> Result<impl Iterator<Item=&'a ValidatorIndex>, Error> {
    Ok([].iter())
}

// ok
pub fn compute_epoch_at_slot<C: Config>(_slot: Slot) -> Epoch {
    0
}

// ok
pub fn compute_start_slot_of_epoch<C: Config>(_epoch: Epoch) -> Slot {
    0
}

// ok
pub fn compute_activation_exit_epoch<C: Config>(_epoch: Epoch) -> Epoch {
    0
}

//pub fn compute_domain<C: Config>(_domain_type: DomainType, _fork_version: Option<&Version>) -> u64 {
//    0
//}
