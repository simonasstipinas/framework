use core::ExpConst;
use ssz_types::VariableList;
use types::{
    beacon_state::*,
    config::Config,
    primitives::{Epoch, Gwei, ValidatorIndex},
    types::PendingAttestation,
};
use helper_functions::{
    beacon_state_accessors::{get_current_epoch, get_previous_epoch, get_validator_churn_limit, get_total_balance, get_total_active_balance, get_randao_mix, get_randao_mix, get_block_root, get_attesting_indices},
    beacon_state_mutators::{initiate_validator_exit, decrease_balance},
    misc::compute_activation_exit_epoch,
    predicates::is_active_validator,
};


pub trait AttestableBlock<T>
where
    T: Config + ExpConst,
{
    fn get_matching_source_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>;
    fn get_matching_target_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>;
    fn get_matching_head_attestations(&self, epoch: Epoch);
    fn get_unslashed_attesting_indices(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> VariableList<ValidatorIndex, T::MaxAttestationsPerEpoch>;
    fn get_attesting_balance(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> Gwei;
}

impl<T> AttestableBlock<T> for BeaconState<T>
where
    T: Config + ExpConst,
{
    fn get_matching_source_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        assert!(epoch == get_previous_epoch(&state) || epoch == get_current_epoch(&state));
        if epoch == get_current_epoch(&state) {
            return state.current_epoch_attestations;
        }
        else {
            return state.previous_epoch_attestations;
        }
    }
    fn get_matching_target_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let target_attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> = VariableList::from(vec![]);
        for a in self.get_matching_source_attestations(get_current_epoch(&self)).iter() {
            if a.data.target.root == get_block_root(&state, epoch).unwrap() {
                target_attestations.push(*a);
            }
        }
        return target_attestations;
    }
    fn get_matching_head_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let head_attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> = VariableList::from(vec![]);
        for a in self.get_matching_source_attestations().iter(){
            if(a.data.beacon_block_root == get_block_root_at_slot(state, a.data.slot).unwrap()){
                head_attestations.push(*a);
            }
        }
        
        return head_attestations;
        return [
            a for a in get_matching_source_attestations(state, epoch)
            if a.data.beacon_block_root == get_block_root_at_slot(state, a.data.slot)
        ]
    }
    fn get_unslashed_attesting_indices(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> VariableList<ValidatorIndex, T::MaxAttestationsPerEpoch> {
        let output: VariableList<ValidatorIndex, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        for attestation in attestations.iter() {
            let indices = get_attesting_indices(&self, &attestation.data, &attestation.aggregation_bits).unwrap();
            for index in indices {
                if self.validators[*index as usize].slashed {
                    output.push(*index);
                }
            }
        }
        return output;
    }
    fn get_attesting_balance(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> Gwei {
        return get_total_balance(&self, &self.get_unslashed_attesting_indices(attestations)).unwrap();
    }
}



// fn get_matching_head_attestations(state: BeaconState<T>, epoch: Epoch)
//  -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
//     return [
//         a for a in get_matching_source_attestations(state, epoch)
//         if a.data.beacon_block_root == get_block_root_at_slot(state, a.data.slot)
//     ]
// }

// fn get_unslashed_attesting_indices(state: BeaconState<T>,
//                                     attestations: Sequence[PendingAttestation]) /*-> Set[ValidatorIndex]*/{
//     let mut output = set();  //# type: Set[ValidatorIndex]
//     for a in attestations:
//         output = output.union(get_attesting_indices(state, a.data, a.aggregation_bits))
//     return set(filter(lambda index: not state.validators[index].slashed, output))
// }
