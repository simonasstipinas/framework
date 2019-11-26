use types::beacon_state::BeaconState;
use types::config::MinimalConfig;
use types::primitives::Gwei;
use types::primitives::ValidatorIndex;

pub fn increase_balance(mut state: BeaconState<MinimalConfig>, index: ValidatorIndex, delta: Gwei) {
    state.balances[index as usize] += delta;
}

pub fn decrease_balance(mut state: BeaconState<MinimalConfig>, index: ValidatorIndex, delta: Gwei) {
    if delta > state.balances[index as usize] {
        state.balances[index as usize] = 0;
    } else {
        state.balances[index as usize] -= delta;
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    fn mock_beaconstate() -> BeaconState {}
    */
}
