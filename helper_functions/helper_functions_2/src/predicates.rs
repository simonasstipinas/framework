use crate::{beacon_state_accessors as accessors, crypto};
use bls::AggregatePublicKey;
use itertools::Itertools;
use ssz_types::VariableList;
use std::convert::TryFrom;
use tree_hash::TreeHash;
use typenum::Unsigned;
use types::{
    beacon_state::BeaconState,
    config::Config,
    helper_functions_types::Error,
    primitives::{Epoch, H256},
    types::{AttestationData, IndexedAttestation, Validator},
};

type ValidatorIndexList<C> = VariableList<u64, <C as Config>::MaxValidatorsPerCommittee>;

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

fn aggregate_validator_public_keys<C: Config>(
    indices: &ValidatorIndexList<C>,
    state: &BeaconState<C>,
) -> Result<AggregatePublicKey, Error> {
    let mut aggr_pkey = AggregatePublicKey::new();
    for i in indices.iter() {
        let ind =
            usize::try_from(*i).expect("Unable to convert ValidatorIndex to usize for indexing");
        if state.validators.len() <= ind {
            return Err(Error::IndexOutOfRange);
        }
        aggr_pkey.add(&state.validators[ind].pubkey);
    }
    Ok(aggr_pkey)
}

pub fn validate_indexed_attestation<C: Config>(
    state: &BeaconState<C>,
    indexed_attestation: &IndexedAttestation<C>,
) -> Result<(), Error> {
    let indices = &indexed_attestation.attesting_indices;

    let max_validators = C::MaxValidatorsPerCommittee::to_usize();
    if indices.len() > max_validators {
        return Err(Error::IndicesExceedMaxValidators);
    }

    if !is_sorted(indices) {
        return Err(Error::IndicesNotSorted);
    }

    let aggr_pubkey = aggregate_validator_public_keys(indices, state)?;

    let hash = indexed_attestation.data.tree_hash_root();

    if indexed_attestation.signature.verify_multiple(
        &[hash.as_slice()],
        accessors::get_domain(
            state,
            C::domain_attestation(),
            Some(indexed_attestation.data.target.epoch),
        ),
        &[&aggr_pubkey],
    ) {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }

    // Check signature
    // Since bit_1_indices is empty (because of the first `if`) we only check that
    // let aggr_pubkey1 = aggregate_validator_public_keys(bit_0_indices, state)?;

    // let hash_1 = AttestationDataAndCustodyBit {
    //     data: indexed_attestation.data.clone(),
    //     custody_bit: false,
    // }
    // .tree_hash_root();

    // if indexed_attestation.signature.verify(
    //     &hash_1,
    //     accessors::get_domain(state, 0, Some(indexed_attestation.data.target.epoch)),
    //     &aggr_pubkey1
    // ) {
    //     Ok(())
    // } else {
    //     Err(Error::InvalidSignature)
    // }
}

pub fn is_valid_merkle_branch(
    leaf: &H256,
    branch: &[H256],
    depth: u64,
    index: u64,
    root: &H256,
) -> Result<bool, Error> {
    let mut value_bytes = leaf.as_bytes().to_vec();
    let depth_s = usize::try_from(depth).expect("Error converting to usize for indexing");
    let index_s = usize::try_from(index).expect("Error converting to usize for indexing");

    if branch.len() < depth_s {
        return Err(Error::IndexOutOfRange);
    }

    let mut branch_bytes: Vec<u8>;
    for (i, node) in branch.iter().enumerate().take(depth_s) {
        let ith_bit = (index_s >> i) & 0x01;
        branch_bytes = node.as_bytes().to_vec();
        if ith_bit == 1 {
            branch_bytes.append(&mut value_bytes);
            value_bytes = crypto::hash(branch_bytes.as_slice());
        } else {
            value_bytes.append(&mut branch_bytes);
            value_bytes = crypto::hash(value_bytes.as_slice());
        }
    }

    Ok(H256::from_slice(&value_bytes) == *root)
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
            index: 0,
            slot: 0,
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

    fn hash_concat(v1: H256, v2: H256) -> H256 {
        let mut val = v1.as_bytes().to_vec();
        val.append(&mut v2.as_bytes().to_vec());
        H256::from_slice(crypto::hash(val.as_slice()).as_slice())
    }

    #[test]
    fn test_valid_merkle_branch() {
        let leaf_b00 = H256::from([0xAA; 32]);
        let leaf_b01 = H256::from([0xBB; 32]);
        let leaf_b10 = H256::from([0xCC; 32]);
        let leaf_b11 = H256::from([0xDD; 32]);

        let node_b0x = hash_concat(leaf_b00, leaf_b01);
        let node_b1x = hash_concat(leaf_b10, leaf_b11);

        let root = hash_concat(node_b0x, node_b1x);

        assert!(
            is_valid_merkle_branch(&leaf_b00, &[leaf_b01, node_b1x], 2, 0, &root)
                .expect("Unexpected error")
        );

        assert!(
            is_valid_merkle_branch(&leaf_b01, &[leaf_b00, node_b1x], 2, 1, &root)
                .expect("Unexpected error")
        );

        assert!(
            is_valid_merkle_branch(&leaf_b10, &[leaf_b11, node_b0x], 2, 2, &root)
                .expect("Unexpected error")
        );

        assert!(
            is_valid_merkle_branch(&leaf_b11, &[leaf_b10, node_b0x], 2, 3, &root)
                .expect("Unexpected error")
        );
    }

    #[test]
    fn test_merkle_branch_depth() {
        let leaf_b00 = H256::from([0xAF; 32]);
        let leaf_b01 = H256::from([0xBB; 32]);
        let leaf_b10 = H256::from([0xCE; 32]);
        let leaf_b11 = H256::from([0xDB; 32]);

        let node_b0x = hash_concat(leaf_b00, leaf_b01);
        let node_b1x = hash_concat(leaf_b10, leaf_b11);

        let root = hash_concat(node_b0x, node_b1x);

        assert!(
            is_valid_merkle_branch(&leaf_b00, &[leaf_b01], 1, 0, &node_b0x)
                .expect("Unexpected error")
        );

        assert_eq!(
            is_valid_merkle_branch(&leaf_b00, &[leaf_b01], 3, 0, &root),
            Err(Error::IndexOutOfRange)
        );
    }

    #[test]
    fn test_invalid_merkle_branch() {
        let leaf_b00 = H256::from([0xFF; 32]);
        let leaf_b01 = H256::from([0xAB; 32]);
        let leaf_b10 = H256::from([0xCE; 32]);
        let leaf_b11 = H256::from([0xDB; 32]);

        let node_b0x = hash_concat(leaf_b00, leaf_b01);
        let node_b1x = hash_concat(leaf_b10, leaf_b11);

        let root = hash_concat(node_b0x, node_b1x);

        assert!(!is_valid_merkle_branch(
            &leaf_b00,
            &[leaf_b01, node_b0x], // should be node_b1x
            2,
            0,
            &root
        )
        .expect("Unexpected error"));

        assert!(!is_valid_merkle_branch(
            &leaf_b11,
            &[leaf_b10, node_b0x],
            2,
            3,
            &H256::from([0xFF; 32])
        ) // Wrong root
        .expect("Unexpected error"));

        assert!(!is_valid_merkle_branch(
            &leaf_b11,
            &[leaf_b10, node_b0x],
            2,
            0, // Wrong index
            &root
        )
        .expect("Unexpected error"));
    }

    mod validate_indexed_attestation_tests {
        use super::*;
        use bls::{AggregateSignature, SecretKey, Signature};
        use types::config::MainnetConfig;

        #[test]
        fn index_set_not_sorted() {
            let state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(2)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(1)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(3)
                .expect("Unable to add attesting index");

            assert_eq!(
                validate_indexed_attestation(&state, &attestation),
                Err(Error::IndicesNotSorted)
            );
        }

        #[test]
        fn non_existent_validators() {
            let state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(0)
                .expect("Unable to add attesting index");

            assert_eq!(
                validate_indexed_attestation(&state, &attestation),
                Err(Error::IndexOutOfRange)
            );
        }

        #[test]
        fn invalid_signature() {
            let mut state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(0)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(1)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(2)
                .expect("Unable to add attesting index");

            // default_validator() generates randome public key
            state
                .validators
                .push(default_validator())
                .expect("Expected successfull push to validator collection");
            state
                .validators
                .push(default_validator())
                .expect("Expected successfull push to validator collection");
            state
                .validators
                .push(default_validator())
                .expect("Expected successfull push to validator collection");

            assert_eq!(
                validate_indexed_attestation(&state, &attestation),
                Err(Error::InvalidSignature)
            );
        }

        #[test]
        fn valid_signature() {
            let mut state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(0)
                .expect("Unable to add attesting index");

            let skey1 = SecretKey::random();
            let pkey1 = PublicKey::from_secret_key(&skey1);
            let v1 = Validator {
                pubkey: pkey1,
                ..default_validator()
            };

            let skey2 = SecretKey::random();
            let pkey2 = PublicKey::from_secret_key(&skey2);
            let v2 = Validator {
                pubkey: pkey2,
                ..default_validator()
            };

            state
                .validators
                .push(v1)
                .expect("Expectected successfull push");
            state
                .validators
                .push(v2)
                .expect("Expectected successfull push");

            attestation.data.beacon_block_root = H256([0xFF; 32]);

            let digest1 = attestation.data.tree_hash_root();

            let sig1 = Signature::new(
                digest1.as_slice(),
                accessors::get_domain(
                    &state,
                    MainnetConfig::domain_attestation(),
                    Some(attestation.data.target.epoch),
                ),
                &skey1,
            );
            let sig2 = Signature::new(
                digest1.as_slice(),
                accessors::get_domain(
                    &state,
                    MainnetConfig::domain_attestation(),
                    Some(attestation.data.target.epoch),
                ),
                &skey2,
            );

            let mut asig = AggregateSignature::new();
            asig.add(&sig1);
            asig.add(&sig2);

            attestation.signature = asig;

            let aggr_pubkey =
                aggregate_validator_public_keys(&attestation.attesting_indices, &state)
                    .expect("Success");
            assert!(attestation.signature.verify(
                &digest1,
                accessors::get_domain(
                    &state,
                    MainnetConfig::domain_attestation(),
                    Some(attestation.data.target.epoch)
                ),
                &aggr_pubkey,
            ));

            assert_eq!(validate_indexed_attestation(&state, &attestation), Ok(()));
        }
    }
}
