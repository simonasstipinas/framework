use crate::beacon_state_accessors::get_current_epoch;
use crate::beacon_state_accessors::get_validator_churn_limit;
use crate::error::Error;
use crate::misc::compute_activation_exit_epoch;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::consts::FAR_FUTURE_EPOCH;
use types::consts::MIN_VALIDATOR_WITHDRAWABILITY_DELAY;
use types::primitives::Epoch;
use types::primitives::Gwei;
use types::primitives::ValidatorIndex;

pub fn increase_balance<C: Config>(state: &mut BeaconState<C>, index: ValidatorIndex, delta: Gwei) {
    state.balances[index as usize] += delta;
}

pub fn decrease_balance<C: Config>(state: &mut BeaconState<C>, index: ValidatorIndex, delta: Gwei) {
    if delta > state.balances[index as usize] {
        state.balances[index as usize] = 0;
    } else {
        state.balances[index as usize] -= delta;
    }
}

pub fn initiate_validator_exit<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
) -> Result<(), Error> {
    let mut validator = state.validators[index as usize].clone();
    if validator.exit_epoch != FAR_FUTURE_EPOCH {
        return Ok(());
    }
    let validators_number = state.validators.len();

    // get exit epochs of all validators
    let mut exit_epochs:Vec<Epoch> = Vec::with_capacity(validators_number);
    for i in 0..validators_number {
        if state.validators[i as usize].exit_epoch != FAR_FUTURE_EPOCH {
            exit_epochs.push(state.validators[i as usize].exit_epoch);
        }
    }
    
    // get the possible exit epoch - by limit of MAX_SEED_LOOK_AHEAD or the last validator in queue:
    let mut exit_queue_epoch:Epoch = compute_activation_exit_epoch(get_current_epoch(&state));
    let _len = exit_epochs.len() as usize;
    for i in 0.._len {
        if exit_epochs[i] > exit_queue_epoch {
            exit_queue_epoch = exit_epochs[i];
        }
    }
    
    // check if number of exiting validators does not exceed churn limit
    let mut exit_queue_churn = 0;
    for i in 0.._len {
        if exit_epochs[i] == exit_queue_epoch {
            exit_queue_churn += 1;
        }
    }
    if exit_queue_churn >= get_validator_churn_limit(state.clone()) {
        exit_queue_epoch += 1;
    }
    
    // change validator's exit epoch in the beacon chain 
    validator.exit_epoch = exit_queue_epoch;
    validator.withdrawable_epoch = (validator.exit_epoch + MIN_VALIDATOR_WITHDRAWABILITY_DELAY) as Epoch;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use ethereum_types::H256;
    use types::types::Validator;
    use bls::{PublicKey, SecretKey};

    fn test_validator_exit_init() {
        let val1:Validator = Validator{
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 24,
            exit_epoch: 4,
            pubkey: PublicKey::from_secret_key(&SecretKey::random()),
            slashed: false,
            withdrawable_epoch: 9999,
            withdrawal_credentials: H256([0; 32]),
        };
        let val2:Validator = Validator{
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 24,
            exit_epoch: FAR_FUTURE_EPOCH,
            pubkey: PublicKey::from_secret_key(&SecretKey::random()),
            slashed: false,
            withdrawable_epoch: 9999,
            withdrawal_credentials: H256([0; 32]),
        };
    }
    */
}
