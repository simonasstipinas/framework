use core::consts::ExpConst;
use helper_functions::{
    beacon_state_accessors::{get_current_epoch, get_previous_epoch, get_validator_churn_limit, get_total_active_balance, get_randao_mix, get_randao_mix, get_block_root},
    beacon_state_mutators::{initiate_validator_exit, decrease_balance},
    misc::compute_activation_exit_epoch,
    predicates::is_active_validator,
};
use types::{
    beacon_state::*,
    config::{Config, MainnetConfig},
    types::{Validator, PendingAttestation},
    primitives::{Epoch},
};
use ssz_types::VariableList;

fn get_matching_source_attestations<T: Config + ExpConst>(state: BeaconState<T>, epoch: Epoch) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
    assert!(epoch == get_previous_epoch(&state) || epoch == get_current_epoch(&state));
    if epoch == get_current_epoch(&state) {
        return state.current_epoch_attestations;
    }
    else {
        return state.previous_epoch_attestations;
    }
}

fn get_matching_target_attestations<T: Config + ExpConst>(state: BeaconState<T>, epoch: Epoch) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
    let target_attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> = VariableList::from(vec![]);
    for a in get_matching_source_attestations(state, epoch).iter() {
        if a.data.target.root == get_block_root(&state, epoch).unwrap() {
            target_attestations.push(*a);
        }
    }
    return target_attestations;
}

fn get_matching_head_attestations(state: BeaconState<T>, epoch: Epoch) /*-> Sequence[PendingAttestation] */{
    /*return [
        a for a in get_matching_source_attestations(state, epoch)
        if a.data.beacon_block_root == get_block_root_at_slot(state, a.data.slot)
    ]*/
}

fn get_unslashed_attesting_indices(state: BeaconState<T>,
                                    attestations: Sequence[PendingAttestation]) /*-> Set[ValidatorIndex]*/{
    /*let mut output = set();  //# type: Set[ValidatorIndex]
    for a in attestations:
        output = output.union(get_attesting_indices(state, a.data, a.aggregation_bits))
    return set(filter(lambda index: not state.validators[index].slashed, output))*/
}

fn get_attesting_balance(state: BeaconState<T>, attestations: Sequence[PendingAttestation]) -> Gwei{
    //!return get_total_balance(state, get_unslashed_attesting_indices(state, attestations));
}
