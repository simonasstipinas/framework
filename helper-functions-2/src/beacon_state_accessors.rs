use crate::predicates::is_active_validator;
use ethereum_types::H256;
use std::cmp::max;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::*;
use crate::error::Error;
use ssz_types::BitList;
// use types::types::{Attestation, AttestationData, IndexedAttestation};
use types::types::AttestationData;
use types::consts::*;
use crate::misc::*;
// use crate::crypto::*;

pub fn get_current_epoch<C: Config>(_state: &BeaconState<C>) -> Epoch {
    compute_epoch_at_slot(_state.slot)
}

pub fn get_previous_epoch<C: Config>(_state: &BeaconState<C>) -> Epoch {
    let current_epoch = get_current_epoch(_state);
    if current_epoch == 0 {
        current_epoch
    } else {
        current_epoch - 1
    }
}

pub fn get_block_root<C: Config>(
    _state: &BeaconState<C>,
    _epoch: Epoch,
) -> Result<H256, Error> {
    return get_block_root_at_slot(_state, crate::misc::compute_start_slot_at_epoch(_epoch))
}

pub fn get_block_root_at_slot<C: Config>(
    _state: &BeaconState<C>,
    _slot: Slot,
) -> Result<H256, Error> {
    if !(_slot < _state.slot && _state.slot <= _slot + SLOTS_PER_HISTORICAL_ROOT) {
        return Err(Error::SlotOutOfRange);
    }
    let index = (_slot % SLOTS_PER_HISTORICAL_ROOT) as usize;
    Ok(_state.block_roots[index])
}

pub fn get_randao_mix<C: Config>(
    _state: &BeaconState<C>,
    _epoch: Epoch,
) -> Result<H256, Error> {
    let index = (_epoch % EPOCHS_PER_HISTORICAL_VECTOR) as usize;
    if index >= _state.randao_mixes.len() {
        return Err(Error::IndexOutOfRange);
    }
    Ok(_state.randao_mixes[index])
}

// pub fn get_active_validator_indices<C: Config>(
//     _state: &BeaconState<C>,
//     _epoch: Epoch,
// ) -> impl Iterator<Item = &ValidatorIndex> {
//     // let mut validators = Vec::<ValidatorIndex>::new();
//     // let mut vals = iter::<&ValidatorIndex>();
//     let mut vals = [].iter();
//     for (i, v) in _state.validators.iter().enumerate() {
//         if is_active_validator(v, _epoch) {
//             // validators.push(i as ValidatorIndex);
//             vals.chain(&[i as ValidatorIndex]);
//         }
//     }
//     // validators.iter()
//     vals
// }
pub fn get_active_validator_indices<C: Config>(
    _state: &BeaconState<C>,
    _epoch: Epoch,
) -> Vec<ValidatorIndex> {
    let mut validators = Vec::<ValidatorIndex>::new();
    for (i, v) in _state.validators.iter().enumerate() {
        if is_active_validator(v, _epoch) {
            validators.push(i as ValidatorIndex)
        }
    }
    validators
}

pub fn get_validator_churn_limit<C: Config>(_state: BeaconState<C>) -> u64 {
    let active_validator_indices = get_active_validator_indices(&_state, get_current_epoch(&_state));
    let active_validator_count = active_validator_indices.len() as u64;
    max(MIN_PER_EPOCH_CHURN_LIMIT, active_validator_count)
}

pub fn get_seed<C: Config>(
    _state: &BeaconState<C>,
    _epoch: Epoch,
    _domain_type: DomainType,
) -> Result<H256, Error> {
    Ok(H256::from([0; 32]))
}
// fn int_to_bytes(_int: u64, _length: usize) -> Result<Vec<u8>, Error> {
//     Ok([].to_vec())
// }
// pub fn get_seed<C: Config>(
//     _state: BeaconState<C>,
//     _epoch: Epoch,
//     _domain_type: DomainType,
// ) -> Result<H256, Error> {
//     let mix = get_randao_mix(&_state, _epoch + EPOCHS_PER_HISTORICAL_VECTOR - MIN_SEED_LOOKAHEAD - 1); //  # Avoid underflow
//     // // crate::crypto::hash(domain_type + crate::math::int_to_bytes(epoch) + mix)

//     let epoch_bytes = int_to_bytes(_epoch, 32).unwrap();

//     let mut preimage = [0; 32 * 3];
//     preimage[0..1].copy_from_slice(&[_domain_type]);
//     // preimage[0..32].copy_from_slice(&mix[..]);
//     preimage[1..33].copy_from_slice(&mix[..]);
//     // preimage[33..64].copy_from_slice(&active_index_root[..]);
//     preimage[33..].copy_from_slice(&epoch_bytes[..]);

//     // Ok(Hash256::from_slice(&hash(&preimage)))
    
//     // crate::crypto::hash(domain_type + int_to_bytes(epoch, 8) + mix)
//     // // crate::crypto::hash(b"Hello World!!!".as_ref().into())
//     Ok(H256::from(preimage))
// }

pub fn get_committee_count_at_slot<C: Config>(
    _state: &BeaconState<C>,
    _slot: Slot,
) -> Result<u64, Error> {
    let epoch = crate::misc::compute_epoch_at_slot(_slot);
    let active_count = get_active_validator_indices(_state, epoch).len() as u64;
    let mut count = if MAX_COMMITTEES_PER_SLOT < active_count {
        MAX_COMMITTEES_PER_SLOT
    } else {
        active_count
    };

    count = if 1 > count {
        1
    } else {
        count
    };

    Ok(count)
}

fn compute_committee<'a>(
    _indices: &'a [ValidatorIndex],
    _seed: &H256,
    _index: u64,
    _count: u64,
) -> Result<impl Iterator<Item = &'a ValidatorIndex>, Error> {
    Ok([].iter())
}
pub fn get_beacon_committee<'a, C: Config>(
    _state: &BeaconState<C>,
    _slot: Slot,
    _index: u64,
) -> Result<impl Iterator<Item = &ValidatorIndex>, Error> {
    let epoch = compute_epoch_at_slot(_slot);
    let committees_per_slot = get_committee_count_at_slot(_state, _slot).unwrap();
    // let indices = get_active_validator_indices(_state, epoch).collect();
    let indices = &[];
    let seed = get_seed(_state, epoch, DOMAIN_BEACON_ATTESTER);//.unwrap();
    if seed.is_err() {
        return Err(seed.err().unwrap());
    }
    let index = (_slot % SLOTS_PER_EPOCH) * committees_per_slot + _index;
    let count = committees_per_slot * SLOTS_PER_EPOCH;
    compute_committee(
        indices,
        &seed.unwrap(),
        index,
        count,
    )
}

fn compute_proposer_index<C: Config>(
    _state: &BeaconState<C>,
    _indices: &[ValidatorIndex],
    _seed: &H256,
) -> Result<ValidatorIndex, Error> {
    Ok(0)
}
pub fn get_beacon_proposer_index<C: Config>(
    _state: &BeaconState<C>,
) -> Result<ValidatorIndex, Error> {
    let epoch = get_current_epoch(_state);
    let seed = get_seed(_state, epoch, DOMAIN_BEACON_PROPOSER);
    let indices = get_active_validator_indices(_state, epoch);
    compute_proposer_index(_state, &indices, &seed.unwrap())
}

pub fn get_total_balance<C: Config>(
    _state: &BeaconState<C>,
    _indices: &[ValidatorIndex],
) -> Result<u64, Error> {
    let mut balance: Gwei = 0;
    for (i, v) in _state.validators.iter().enumerate() {
        if _indices.contains(&(i as u64)) {
            balance += v.effective_balance;
        }
    }
    if balance > 1 {
        Ok(balance)
    } else {
        Ok(1)
    }
}

pub fn get_total_active_balance<C: Config>(_state: &BeaconState<C>) -> Result<u64, Error> {
    let current_epoch = get_current_epoch(_state);
    get_total_balance(_state, &get_active_validator_indices(_state, current_epoch))
}

fn compute_domain(
    _domain_type: DomainType,
    _fork_version: Option<&Version>,
) -> Domain {
    0
}
pub fn get_domain<C: Config>(
    _state: &BeaconState<C>,
    _domain_type: DomainType,
    _message_epoch: Option<Epoch>,
) -> Domain {
    let epoch = if _message_epoch == None {
        get_current_epoch(_state)
    } else {
        _message_epoch.unwrap()
    };
    let fork_version = if epoch < _state.fork.epoch {_state.fork.previous_version} else {_state.fork.current_version};
    compute_domain(_domain_type, Some(&fork_version))
}

// pub fn get_indexed_attestation<C: Config>(
//     _state: &BeaconState<C>,
//     _attestation: &Attestation<C>,
// ) -> Result<IndexedAttestation<C>, Error> {
//     let attesting_indices = get_attesting_indices(_state, &(_attestation.data), &(_attestation.aggregation_bits));

//     let attestation = IndexedAttestation(
//         attesting_indices=attesting_indices,
//         data=_attestation.data,
//         signature=_attestation.signature,
//     );
//     Ok(attestation)
//     // Err(Error::IndexOutOfRange)
// }

pub fn get_attesting_indices<'a, C: Config>(
    _state: &'a BeaconState<C>,
    _attestation_data: &AttestationData,
    _bitlist: &BitList<C::MaxValidatorsPerCommittee>,
) -> Result<impl Iterator<Item = &'a ValidatorIndex>, Error> {
    let comittee = get_beacon_committee(_state, _attestation_data.slot, _attestation_data.index);//.unwrap();
    // let mut validators = Vec::<ValidatorIndex>::new();
    // for (i, v) in comittee.enumerate() {
    //     if (*_bitlist.cloned()).into_bytes().contains(&(*v as u8)) {
    //         validators.push(*v);
    //     }
    // }
    // Ok(validators.iter())
    // set(index for i, index in enumerate(committee) if bits[i])
    // Ok([].iter())
    comittee
}



#[cfg(test)]
mod tests {
    use super::*;
    use types::config::MinimalConfig;
    use ssz_types::{FixedVector, typenum};

    #[test]
    fn test_get_current_epoch() {
        let state = BeaconState::<MinimalConfig>::default();
        assert_eq!(get_current_epoch::<MinimalConfig>(&state), 0);
    }

    #[test]
    fn test_get_previous_epoch() {
        let state = BeaconState::<MinimalConfig>::default();
        assert_eq!(get_previous_epoch::<MinimalConfig>(&state), 0);
    }

    #[test]
    fn test_get_block_root() {
        let mut state = BeaconState::<MinimalConfig>::default();

        let base: Vec<H256> = vec![H256::from([0; 32])];
        let roots: FixedVector<_, typenum::U64> = FixedVector::from(base.clone());
        state.block_roots = roots;
        let result = get_block_root::<MinimalConfig>(&state, 0);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_get_block_root_at_slot() {
        let mut state = BeaconState::<MinimalConfig>::default();

        let base: Vec<H256> = vec![H256::from([0; 32])];
        let roots: FixedVector<_, typenum::U64> = FixedVector::from(base.clone());
        state.block_roots = roots;
        let result = get_block_root_at_slot::<MinimalConfig>(&state, 0);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_get_randao_mix() {
        let state = BeaconState::<MinimalConfig>::default();
        let result = get_randao_mix::<MinimalConfig>(&state, 0);
        assert_eq!(result.is_ok(), false);
    }
}
