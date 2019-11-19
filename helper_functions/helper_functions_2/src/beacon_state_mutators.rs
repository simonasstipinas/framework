use super::beacon_state_accessors as accessors;
use super::error::Error;
use std::cmp;
use std::convert::TryFrom;
use typenum::Unsigned;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::{Epoch, Gwei, ValidatorIndex};

pub fn increase_balance<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
    delta: Gwei,
) -> Result<(), Error> {
    // TODO: check if index is not out of bounds
    let v_index = usize::try_from(index)
        .expect("Conversion to usize for indexing would truncate the value of ValidatorIndex");
    state.balances[v_index] += delta;
    Ok(())
}

pub fn decrease_balance<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
    delta: Gwei,
) -> Result<(), Error> {
    // TODO: check if index not out of bounds
    let v_index = usize::try_from(index)
        .expect("Conversion to usize for indexing would truncate the value of ValidatorIndex");
    if delta > state.balances[v_index] {
        state.balances[v_index] = 0;
    } else {
        state.balances[v_index] -= delta;
    }
    Ok(())
}

pub fn initiate_validator_exit<C: Config>(
    _state: &mut BeaconState<C>,
    _index: ValidatorIndex,
) -> Result<(), Error> {
    // TODO:
    Ok(())
}

pub fn slash_validator<C: Config>(
    state: &mut BeaconState<C>,
    slashed_index: ValidatorIndex,
    whistleblower_index: Option<ValidatorIndex>,
) -> Result<(), Error> {
    let epoch: Epoch = accessors::get_current_epoch(state);
    initiate_validator_exit(state, slashed_index)?;
    let sl_index = usize::try_from(slashed_index)
        .expect("Conversion to usize for indexing would truncate the value of ValidatorIndex");
    let validator = &mut state.validators[sl_index];
    validator.slashed = true;
    let epochs_per_slashings = C::EpochsPerSlashingsVector::to_u64();
    validator.withdrawable_epoch =
        cmp::max(validator.withdrawable_epoch, epoch + epochs_per_slashings);
    let effective_balance = validator.effective_balance;
    let slashings_index = usize::try_from(epoch % epochs_per_slashings)
        .expect("Conversion to usize for indexing would truncate the value of ValidatorIndex");
    state.slashings[slashings_index] += effective_balance;
    let decr = validator.effective_balance / C::min_slashing_penalty_quotient();
    decrease_balance(state, slashed_index, decr)?;

    // Apply proposer and whistleblower rewards
    let proposer_index = accessors::get_beacon_proposer_index(state)?;
    let whistleblower_ind_val = match whistleblower_index {
        None => proposer_index,
        Some(i) => i,
    };
    let whistleblower_reward = effective_balance / C::whistleblower_reward_quotient();
    let proposer_reward = effective_balance / C::proposer_reward_quotient();
    increase_balance(state, proposer_index, proposer_reward)?;
    increase_balance(state, whistleblower_ind_val, whistleblower_reward)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls::{PublicKey, SecretKey};
    use types::config::MainnetConfig;
    use types::primitives::H256;
    use types::types::Validator;

    // fn mock_beaconstate() -> BeaconState {}

    const EPOCH_MAX: u64 = u64::max_value();

    fn default_validator() -> Validator {
        Validator {
            effective_balance: 0,
            slashed: false,
            activation_eligibility_epoch: EPOCH_MAX,
            activation_epoch: EPOCH_MAX,
            exit_epoch: EPOCH_MAX,
            withdrawable_epoch: EPOCH_MAX,
            withdrawal_credentials: H256([0; 32]),
            pubkey: PublicKey::from_secret_key(&SecretKey::random()),
        }
    }

    mod slash_validator_tests {
        use super::*;

        #[test]
        fn test_exit_epoch() {
            let mut state: BeaconState<MainnetConfig> = BeaconState::default();
            // Add validator and it's balance
            state
                .validators
                .push(default_validator())
                .expect("Expected successess");
            state.balances.push(100).expect("Expected success");

            let mut state_copy = state.clone();
            initiate_validator_exit(&mut state_copy, 0)
                .expect("Expected successful initiate_validator_exit");

            slash_validator(&mut state, 0, None).expect("slash_validator should succeed");

            assert_eq!(
                state_copy.validators[0].exit_epoch,
                state.validators[0].exit_epoch
            );
        }
    }
}
