use core::ExpConst;
use ssz_types::VariableList;
use types::{
    beacon_state::*,
    config::Config,
    primitives::{Epoch, Gwei},
    types::PendingAttestation,
};

pub mod attestations;
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
        let ret: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        ret
    }
    fn get_matching_target_attestations(
        &self,
        epoch: Epoch,
    ) -> VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> {
        let ret: VariableList<PendingAttestation<T>, T::MaxAttestationsPerEpoch> =
            VariableList::from(vec![]);
        ret
    }
    fn get_matching_head_attestations(&self, epoch: Epoch) {}
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
