use types::{
    beacon_state::BeaconState,
    config::Config,
    primitives::Epoch,
    types::{AttestationData, IndexedAttestation, Validator},
};

pub fn is_active_validator(_validator: &Validator, _epoch: Epoch) -> bool {
    true
}

pub fn is_slashable_validator(_validator: &Validator, _epoch: Epoch) -> bool {
    true
}

pub fn is_slashable_attestation_data(_data_1: &AttestationData, _data_2: &AttestationData) -> bool {
    true
}

pub fn is_valid_indexed_attestation<C: Config>(
    _state: &BeaconState<C>,
    _indexed_attestation: &IndexedAttestation<C>,
) -> bool {
    true
}

pub fn is_valid_merkle_branch<C: Config>(
    _leaf: &[u8],
    _branch: Vec<&[u8]>,
    _depth: &u64,
    _index: &u64,
    _root: &[u8],
) -> bool {
    true
}
