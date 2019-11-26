use crate::beacon_state_accessors::get_current_epoch;
use crate::beacon_state_accessors::get_validator_churn_limit;
use crate::error::Error;
use crate::misc::compute_activation_exit_epoch;
use std::convert::TryFrom;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::config::MainnetConfig;
use types::consts::FAR_FUTURE_EPOCH;
use types::primitives::Epoch;
use types::primitives::Gwei;
use types::primitives::ValidatorIndex;

pub fn increase_balance<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
    delta: Gwei,
) -> Result<(), Error> {
    let balances_size = state.balances.len();
    if usize::try_from(index).expect("") >= balances_size {
        return Err(Error::IndexOutOfRange);
    }
    state.balances[usize::try_from(index).expect("")] += delta;
    Ok(())
}

pub fn decrease_balance<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
    delta: Gwei,
) -> Result<(), Error> {
    let balances_size = state.balances.len();
    if usize::try_from(index).expect("") >= balances_size {
        return Err(Error::IndexOutOfRange);
    }
    if delta > state.balances[usize::try_from(index).expect("")] {
        state.balances[usize::try_from(index).expect("")] = 0;
    } else {
        state.balances[usize::try_from(index).expect("")] -= delta;
    }
    Ok(())
}

// function uses Mainnetconfig implementation to access static Config function - it seems that there is no workaround
pub fn initiate_validator_exit<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
) -> Result<(), Error> {
    let mut validator = state.validators[usize::try_from(index).expect("")].clone();
    if validator.exit_epoch != FAR_FUTURE_EPOCH {
        return Ok(());
    }
    let validators_number = state.validators.len();

    // get exit epochs of all validators
    let mut exit_epochs: Vec<Epoch> = Vec::with_capacity(validators_number);
    for i in 0..validators_number {
        if state.validators[i].exit_epoch != FAR_FUTURE_EPOCH {
            exit_epochs.push(state.validators[i].exit_epoch);
        }
    }

    // get the possible exit epoch - by MIN_SEED_LOOK_AHEAD or the last validator in queue:
    let current_epoch: Epoch = get_current_epoch(state);
    let mut exit_queue_epoch: Epoch = compute_activation_exit_epoch::<C>(current_epoch);
    let iter = exit_epochs.iter();
    for i in iter {
        if *i > exit_queue_epoch {
            exit_queue_epoch = *i;
        }
    }

    // check if number of exiting validators does not exceed churn limit
    let mut exit_queue_churn = 0;
    let iter = exit_epochs.iter();
    for i in iter {
        if *i == exit_queue_epoch {
            exit_queue_churn += 1;
        }
    }
    if exit_queue_churn >= get_validator_churn_limit(state) {
        exit_queue_epoch += 1;
    }

    // change validator's exit epoch in the beacon chain
    validator.exit_epoch = exit_queue_epoch;
    validator.withdrawable_epoch =
        validator.exit_epoch + MainnetConfig::min_validator_withdrawability_delay();
    state.validators[usize::try_from(index).expect("")] = validator;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls::{PublicKey, SecretKey};
    use ethereum_types::H256;
    use types::config::MinimalConfig;
    use types::types::Validator;

    #[test]
    fn test_validator_exit_init() {
        let mut state = BeaconState::<MinimalConfig>::default();

        let val1: Validator = Validator {
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 24,
            exit_epoch: 4,
            pubkey: PublicKey::from_secret_key(&SecretKey::random()),
            slashed: false,
            withdrawable_epoch: 9999,
            withdrawal_credentials: H256([0; 32]),
        };

        let val2: Validator = Validator {
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 24,
            exit_epoch: FAR_FUTURE_EPOCH,
            pubkey: PublicKey::from_secret_key(&SecretKey::random()),
            slashed: false,
            withdrawable_epoch: 9999,
            withdrawal_credentials: H256([0; 32]),
        };

        state.validators.push(val1).expect("");
        state.validators.push(val2).expect("");
        // 1 - exit epoch is already set and should remain the same
        let expected_exit_epoch: Epoch = 4;
        initiate_validator_exit(&mut state, 0).expect("");
        assert_eq!(expected_exit_epoch, state.validators[0].exit_epoch);
        assert_ne!(5, state.validators[0].exit_epoch);
        // 2 - exit epoch is FAR_FUTURE epoch and should be set to the lowest possible value
        initiate_validator_exit(&mut state, 1).expect("");
        assert_ne!(FAR_FUTURE_EPOCH, state.validators[1].exit_epoch);
        assert_eq!(4, state.validators[1].exit_epoch);
        // same exit epoch as val1, because churn is not exceeded
    }

    #[test]
    fn test_increase_balance() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.balances.push(5).expect("");
        increase_balance(&mut state, 0, 10).expect("");
        assert_eq!(state.balances[0], 15);
    }

    #[test]
    fn test_decrease_balance() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.balances.push(5).expect("");
        decrease_balance(&mut state, 0, 10).expect("");
        assert_eq!(state.balances[0], 0);
        state.balances.push(10).expect("");
        decrease_balance(&mut state, 1, 5).expect("");
        assert_eq!(state.balances[1], 5);
    }
}
