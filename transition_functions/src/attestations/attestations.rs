use core::ExpConst;
use helper_functions::beacon_state_accessors::BeaconStateAccessor;
use helper_functions::{
    beacon_state_accessors::{
        get_attesting_indices, get_block_root, get_current_epoch, get_previous_epoch,
        get_randao_mix, get_total_active_balance, get_total_balance, get_validator_churn_limit,
    },
    beacon_state_mutators::{decrease_balance, initiate_validator_exit},
    misc::compute_activation_exit_epoch,
    predicates::is_active_validator,
};
use ssz_types::VariableList;
use types::{
    beacon_state::*,
    config::Config,
    primitives::{Epoch, Gwei, ValidatorIndex},
    types::PendingAttestation,
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
    fn get_matching_head_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>;
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
        assert!(epoch == get_previous_epoch(&self) || epoch == get_current_epoch(&self));
        if epoch == get_current_epoch(&self) {
            return self.current_epoch_attestations.clone();
        } else {
            return self.previous_epoch_attestations.clone();
        }
    }
    fn get_matching_target_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let target_attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        for a in self
            .get_matching_source_attestations(get_current_epoch(&self))
            .iter()
        {
            if a.data.target.root == self.get_block_root(epoch).unwrap() {
                target_attestations.push(*a);
            }
        }
        return target_attestations;
    }
    fn get_matching_head_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let head_attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        for a in self
            .get_matching_source_attestations(self.get_current_epoch())
            .iter()
        {
            if a.data.beacon_block_root == self.get_block_root_at_slot(a.data.slot).unwrap() {
                head_attestations.push(*a);
            }
        }
        return head_attestations;
    }

    fn get_unslashed_attesting_indices(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> VariableList<ValidatorIndex, T::MaxAttestationsPerEpoch> {
        let output: VariableList<ValidatorIndex, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        for attestation in attestations.iter() {
            let indices =
                get_attesting_indices(&self, &attestation.data, &attestation.aggregation_bits)
                    .unwrap();
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
        return get_total_balance(&self, &self.get_unslashed_attesting_indices(attestations))
            .unwrap();
    }
}
