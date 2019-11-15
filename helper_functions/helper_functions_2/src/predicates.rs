use crate::error::Error;
use types::{
    beacon_state::BeaconState,
    config::Config,
    primitives::{Epoch, H256},
    types::{AttestationData, AttestationDataAndCustodyBit, IndexedAttestation, Validator},
};
use typenum::Unsigned;
use itertools::Itertools;
use crate::{crypto, beacon_state_accessors as accessors};
use bls::{AggregatePublicKey, AggregateSignature};
use tree_hash::TreeHash;
use ssz_types::VariableList;
use std::convert::TryFrom;

type ValidatorIndexList<C: Config> = VariableList<u64, C::MaxValidatorsPerCommittee>;

// Check if validator is active
pub fn is_active_validator(validator: &Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

// Check if validator is slashable
pub fn is_slashable_validator(validator: &Validator, epoch: Epoch) -> bool {
    !validator.slashed
        && epoch < validator.withdrawable_epoch
        && validator.activation_epoch <= epoch
}

// Check if ``data_1`` and ``data_2`` are slashable according to Casper FFG rules.
pub fn is_slashable_attestation_data(data_1: &AttestationData, data_2: &AttestationData) -> bool {
    (data_1 != data_2 && data_1.target.epoch == data_2.target.epoch)
        || (data_1.source.epoch < data_2.source.epoch && data_2.target.epoch < data_1.target.epoch)
}

fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord + Clone,
{
    data.into_iter().tuple_windows().all(|(a, b)| a <= b)
}

fn has_common_elements<I>(data1: I, data2: I) -> bool
where
    I: IntoIterator,
    I::Item: Eq
{
    let mut data2_iter = data2.into_iter();
    data1.into_iter().any(|x| {
        data2_iter.any(|y| x == y)

    })
}

fn aggregate_validator_public_keys<C: Config>(
    indices: &ValidatorIndexList<C>,
    state: &BeaconState<C>,
 ) -> Result<AggregatePublicKey, Error> {
    let mut aggr_pkey = AggregatePublicKey::new();
    for i in indices.iter() {
        let ind = usize::try_from(*i).expect("Unable to convert ValidatorIndex to usize for indexing");
        if state.validators.len() >= ind {
            return Err(Error::IndexOutOfRange);
        }
        aggr_pkey.add(&state.validators[ind].pubkey);
    }
    Ok(aggr_pkey)
}

// ok
// In case of invalid attestatation return an error specifying why it's invalid
//  instead of just false. That's how lighthouse does it.
// TODO: add required error types to Error enum
// """
//     Check if ``indexed_attestation`` has valid indices and signature.
//     """
//     bit_0_indices = indexed_attestation.custody_bit_0_indices
//     bit_1_indices = indexed_attestation.custody_bit_1_indices

//     # Verify no index has custody bit equal to 1 [to be removed in phase 1]
//     if not len(bit_1_indices) == 0:  # [to be removed in phase 1]
//         return False                 # [to be removed in phase 1]
//     # Verify max number of indices
//     if not len(bit_0_indices) + len(bit_1_indices) <= MAX_VALIDATORS_PER_COMMITTEE:
//         return False
//     # Verify index sets are disjoint
//     if not len(set(bit_0_indices).intersection(bit_1_indices)) == 0:
//         return False
//     # Verify indices are sorted
//     if not (bit_0_indices == sorted(bit_0_indices) and bit_1_indices == sorted(bit_1_indices)):
//         return False
//     # Verify aggregate signature
//     if not bls_verify_multiple(
//         pubkeys=[
//             bls_aggregate_pubkeys([state.validators[i].pubkey for i in bit_0_indices]),
//             bls_aggregate_pubkeys([state.validators[i].pubkey for i in bit_1_indices]),
//         ],
//         message_hashes=[
//             hash_tree_root(AttestationDataAndCustodyBit(data=indexed_attestation.data, custody_bit=0b0)),
//             hash_tree_root(AttestationDataAndCustodyBit(data=indexed_attestation.data, custody_bit=0b1)),
//         ],
//         signature=indexed_attestation.signature,
//         domain=get_domain(state, DOMAIN_BEACON_ATTESTER, indexed_attestation.data.target.epoch),
//     ):
//         return False
//     return True

pub fn validate_index_attestation<C: Config>(
    state: &BeaconState<C>,
    indexed_attestation: &IndexedAttestation<C>,
) -> Result<(), Error> {
    let bit_0_indices = &indexed_attestation.custody_bit_0_indices;
    let bit_1_indices = &indexed_attestation.custody_bit_1_indices;

    if bit_1_indices.is_empty() {
        return Err(Error::CustodyBit1Set);
    }

    let max_validators = C::MaxValidatorsPerCommittee::to_usize();
    if bit_0_indices.len() + bit_1_indices.len() > max_validators {
        return Err(Error::IndicesExceedMaxValidators);
    } 
    
    if has_common_elements(bit_0_indices, bit_1_indices) {
        return Err(Error::CustodyBitIndicesIntersect);
    }

    if !is_sorted(bit_0_indices) || !is_sorted(bit_1_indices) {
        return Err(Error::CustodyBitIndicesNotSorted)
    }

    let aggr_pubkey1 = aggregate_validator_public_keys(bit_0_indices, state)?;
    let aggr_pubkey2 = aggregate_validator_public_keys(bit_1_indices, state)?;

    let hash_1 = AttestationDataAndCustodyBit{
        data: indexed_attestation.data.clone(),
        custody_bit: false,
    }.tree_hash_root();
    let hash_2 = AttestationDataAndCustodyBit{
        data: indexed_attestation.data.clone(),
        custody_bit: true
    }.tree_hash_root();
    //TODO:
    indexed_attestation.signature.verify_multiple(
        &[&hash_1, &hash_2],
        accessors::get_domain(state, ),
        &[&aggr_pubkey1, &aggr_pubkey2]
    );
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls::{PublicKey, SecretKey};
    //use std::u64::max_value() as epoch_max;
    const EPOCH_MAX: u64 = u64::max_value();
    use types::primitives::H256;
    use types::types::{Checkpoint, Crosslink};

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

    const fn default_crosslink() -> Crosslink {
        Crosslink {
            shard: 0,
            parent_root: H256([0; 32]),
            start_epoch: 0,
            end_epoch: 1,
            data_root: H256([0; 32]),
        }
    }

    const fn default_attestation_data() -> AttestationData {
        AttestationData {
            beacon_block_root: H256([0; 32]),
            source: Checkpoint {
                epoch: 0,
                root: H256([0; 32]),
            },
            target: Checkpoint {
                epoch: 0,
                root: H256([0; 32]),
            },
            crosslink: default_crosslink(),
        }
    }

    #[test]
    fn test_not_activated() {
        let validator = default_validator();
        let epoch: u64 = 10;

        assert!(!is_active_validator(&validator, epoch));
    }

    #[test]
    fn test_activated() {
        let mut validator = default_validator();
        validator.activation_epoch = 4;
        let epoch: u64 = 10;

        assert!(is_active_validator(&validator, epoch));
    }

    #[test]
    fn test_exited() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.exit_epoch = 10;
        let epoch: u64 = 10;

        assert!(!is_active_validator(&validator, epoch));
    }

    #[test]
    fn test_already_slashed() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.slashed = true;
        let epoch: u64 = 10;

        assert!(!is_slashable_validator(&validator, epoch));
    }

    #[test]
    fn test_not_slashable_not_active() {
        let validator = default_validator();
        let epoch: u64 = 10;

        assert!(!is_slashable_validator(&validator, epoch));
    }

    #[test]
    fn test_not_slashable_withdrawable() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.withdrawable_epoch = 9;
        let epoch: u64 = 10;

        assert!(!is_slashable_validator(&validator, epoch));
    }

    #[test]
    fn test_slashable() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.withdrawable_epoch = 11;
        let epoch: u64 = 10;

        assert!(is_slashable_validator(&validator, epoch));
    }

    #[test]
    fn test_double_vote_attestation_data() {
        let mut data_1 = default_attestation_data();
        let data_2 = default_attestation_data();
        data_1.target.root = H256([1; 32]);

        assert!(is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_equal_attestation_data() {
        let data_1 = default_attestation_data();
        let data_2 = default_attestation_data();

        assert!(!is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_surround_vote_attestation_data() {
        let mut data_1 = default_attestation_data();
        let mut data_2 = default_attestation_data();
        data_1.source.epoch = 0;
        data_2.source.epoch = 1;
        data_1.target.epoch = 4;
        data_2.target.epoch = 3;

        assert!(is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_not_slashable_attestation_data() {
        let mut data_1 = default_attestation_data();
        let mut data_2 = default_attestation_data();
        data_1.source.epoch = 0;
        data_1.target.epoch = 4;
        data_2.source.epoch = 4;
        data_2.target.epoch = 5;
        data_2.source.root = H256([1; 32]);
        data_2.target.root = H256([1; 32]);

        assert!(!is_slashable_attestation_data(&data_1, &data_2));
    }
}
