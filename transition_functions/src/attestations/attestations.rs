use core::ExpConst;
use ssz_types::VariableList;
use types::{
    beacon_state::*,
    config::Config,
    primitives::{Epoch, Gwei},
    types::PendingAttestation,
};
use helper_functions::{
    beacon_state_accessors::{get_current_epoch, get_previous_epoch,
    get_validator_churn_limit, get_total_active_balance, get_randao_mix, get_randao_mix, get_block_root},
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
    fn get_matching_head_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>;
    fn get_unslashed_attesting_indices(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>;
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
            return self.current_epoch_attestations;
        }
        else {
            return self.previous_epoch_attestations;
        }
    }
    fn get_matching_target_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let target_attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> = VariableList::from(vec![]);
        for a in self.get_matching_source_attestations(get_current_epoch(&self)).iter() {
            if a.data.target.root == get_block_root(&self, epoch).unwrap() {
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
        for a in self.get_matching_source_attestations(get_current_epoch(&self)).iter() {
            if a.data.beacon_block_root == get_block_root_at_slot(&self, a.data.slot){
                head_attestations.push(*a);
            }
        }
        return head_attestations;
}
    fn get_unslashed_attesting_indices(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let ret: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        ret
    }
    fn get_attesting_balance(
        &self,
        attestations: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch>,
    ) -> Gwei {
        0
    }
}
