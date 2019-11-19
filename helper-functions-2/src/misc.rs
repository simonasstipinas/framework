use crate::crypto::hash;
use crate::error::Error;
use crate::math::bytes_to_int;
use crate::math::int_to_bytes;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::consts::MAX_SEED_LOOKAHEAD;
use types::consts::MAX_EFFECTIVE_BALANCE;
use types::consts::SHUFFLE_ROUND_COUNT;
use types::consts::SLOTS_PER_EPOCH;
use types::primitives::{Domain, DomainType, Epoch, Slot, ValidatorIndex, Version, H256};

pub fn compute_epoch_at_slot(slot: Slot) -> Epoch {
    let slot_f64 = slot as f64;
    (slot_f64 / SLOTS_PER_EPOCH as f64) as u64
}

pub fn compute_start_slot_at_epoch(epoch: Epoch) -> Slot {
    epoch * SLOTS_PER_EPOCH
}

pub fn compute_activation_exit_epoch(epoch: Epoch) -> Epoch {
    epoch + 1 + MAX_SEED_LOOKAHEAD
}

pub fn compute_domain(
    domain_type: DomainType,
    fork_version: Option<&Version>,
) -> Domain {
    let mut domain:Domain = [0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..4 {
        domain[i] = domain_type[i];
        match fork_version {
            Some(f) => {
                domain[i+4] = f[i];
            }
            None => return domain
        }
    }
    domain
}

pub fn compute_shuffled_index(
    index: ValidatorIndex,
    index_count: u64,
    seed: &H256,
) -> Result<ValidatorIndex, Error> {
    if index > index_count {
        return Err(Error::IndexOutOfRange);
    }
    
    let mut _index = index;
    for current_round in 0..SHUFFLE_ROUND_COUNT {
        // compute pivot
        let seed_bytes = seed.as_bytes();
        let round_bytes:Vec<u8> = int_to_bytes(current_round, 1).unwrap();
        let mut sum_vec:Vec<u8> = Vec::new();
        for i in 0..seed_bytes.len() {
            sum_vec.push(seed_bytes[i]);
        }
        sum_vec.push(round_bytes[0]);
        let hashed_value = hash(&sum_vec.as_mut_slice());
        let mut hash_8_bytes:Vec<u8> = Vec::new();
        for i in 0..8{
            hash_8_bytes.push(hashed_value[i]);
        }
        let pivot = bytes_to_int(hash_8_bytes.as_mut_slice()).unwrap() % index_count;
        // compute flip
        let flip = (pivot + index_count - _index) % index_count;
        // compute position
        let position;
        if index > flip {
            position = _index;
        } else {
            position = flip;
        }
        // compute source
        let addition_to_sum:Vec<u8> = int_to_bytes(position / 256, 4).unwrap();
        for i in 0..addition_to_sum.len(){
            sum_vec.push(addition_to_sum[i]);
        }
        let source = hash(&sum_vec.as_mut_slice());
        // compute byte
        let byte = source[((position % 256) / 8) as usize];
        let bit = (byte / ((2 * (position % 8)) as u8)) % 2;
        // flip or not?
        if bit == 1 {
            _index = flip;
        }
    }
    Ok(_index)
}

pub fn compute_proposer_index<C: Config>(
    state: &BeaconState<C>,
    indices: &[ValidatorIndex],
    seed: &H256,
) -> Result<ValidatorIndex, Error> {
    if indices.len() <= 0 {
        return Err(Error::ArrayIsEmpty);
    }
    let max_random_byte = 255;
    let mut i = 0;
    loop {
        let candidate_index = indices[compute_shuffled_index(i % indices.len() as u64, indices.len() as u64, seed).unwrap() as usize];
        let rand_bytes = int_to_bytes(i / 32, 8).unwrap();
        let mut seed_and_bytes:Vec<u8> = Vec::new();
        for i in 0..32 {
            seed_and_bytes.push(seed[i]);
        }
        for i in 0..8 {
            seed_and_bytes.push(rand_bytes[i]);
        }
        let hashed_seed_and_bytes = hash(seed_and_bytes.as_mut_slice());
        let random_byte = hashed_seed_and_bytes[(i % 32) as usize];
        let effective_balance = state.validators[candidate_index as usize].effective_balance;
        if effective_balance * max_random_byte >= MAX_EFFECTIVE_BALANCE * (random_byte as u64) {
            return Ok(candidate_index as ValidatorIndex);
        }
        i += 1;
    }
}

pub fn compute_committee<'a>(
    indices: &'a [ValidatorIndex],
    seed: &H256,
    index: u64,
    count: u64,
) -> Result<Vec<ValidatorIndex>, Error> {
    let start = ((indices.len() as u64) * index) / count;
    let end = ((indices.len() as u64) * (index + 1)) / count;
    let mut committee_vec:Vec<ValidatorIndex> = Vec::new();
    for i in start..end {
        committee_vec.push(indices[compute_shuffled_index(i as ValidatorIndex, indices.len() as u64, seed).unwrap() as usize]);
    }
    Ok(committee_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_at_slot() {
        assert_eq!(compute_epoch_at_slot(65), 2);
        assert_eq!(compute_epoch_at_slot(64), 2);
        assert_eq!(compute_epoch_at_slot(63), 1);
    }

    #[test]
    fn test_start_slot_at_epoch() {
        assert_eq!(compute_start_slot_at_epoch(2), 64);
        assert_ne!(compute_start_slot_at_epoch(2), 63);
        assert_ne!(compute_start_slot_at_epoch(2), 65);
    }

    #[test]
    fn test_activation_exit_epoch() {
        assert_eq!(compute_activation_exit_epoch(1), 6);
    }
}
