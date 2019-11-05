use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::{Gwei, ValidatorIndex};

pub fn increase_balance<C: Config>(
    _state: &mut BeaconState<C>,
    _index: ValidatorIndex,
    _delta: Gwei,
) {
}

pub fn decrease_balance<C: Config>(
    _state: &mut BeaconState<C>,
    _index: ValidatorIndex,
    _delta: Gwei,
) {
}

pub fn initiate_validator_exit<C: Config>(_state: &mut BeaconState<C>, _index: ValidatorIndex) {}

pub fn slash_validator<C: Config>(_state: &mut BeaconState<C>, _index: ValidatorIndex) {}
