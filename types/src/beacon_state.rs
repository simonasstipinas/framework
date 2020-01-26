use crate::{
    config::*, consts, helper_functions_types::Error as HelperError, primitives::*, types::*,
};
use ethereum_types::H256 as Hash256;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, Error as SzzError, FixedVector, VariableList};
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq)]
pub enum Error {
    EpochOutOfBounds,
    SlotOutOfBounds,
    ShardOutOfBounds,
    UnknownValidator,
    UnableToDetermineProducer,
    InvalidBitfield,
    ValidatorIsWithdrawable,
    UnableToShuffle,
    TooManyValidators,
    InsufficientValidators,
    InsufficientRandaoMixes,
    InsufficientBlockRoots,
    InsufficientIndexRoots,
    InsufficientAttestations,
    InsufficientCommittees,
    InsufficientStateRoots,
    NoCommitteeForShard,
    NoCommitteeForSlot,
    ZeroSlotsPerEpoch,
    PubkeyCacheInconsistent,
    PubkeyCacheIncomplete {
        cache_len: usize,
        registry_len: usize,
    },
    PreviousCommitteeCacheUninitialized,
    CurrentCommitteeCacheUninitialized,
    //RelativeEpochError(RelativeEpochError),
    //CommitteeCacheUninitialized(RelativeEpoch),
    SszTypesError(ssz_types::Error),
    HelperError(HelperError),
}

impl From<SzzError> for Error {
    fn from(error: SzzError) -> Self {
        Error::SszTypesError(error)
    }
}

impl From<HelperError> for Error {
    fn from(error: HelperError) -> Self {
        Error::HelperError(error)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash, Default)]
pub struct BeaconState<C: Config> {
    pub genesis_time: u64,
    pub slot: Slot,
    pub fork: Fork,

    // History
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
    pub state_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
    pub historical_roots: VariableList<H256, C::HistoricalRootsLimit>,

    // Eth1 Data
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: VariableList<Eth1Data, C::SlotsPerEth1VotingPeriod>,
    pub eth1_deposit_index: u64,

    // Registry
    pub validators: VariableList<Validator, C::ValidatorRegistryLimit>,
    pub balances: VariableList<u64, C::ValidatorRegistryLimit>,

    // Shuffling
    pub randao_mixes: FixedVector<H256, C::EpochsPerHistoricalVector>,

    // Slashings
    pub slashings: FixedVector<u64, C::EpochsPerSlashingsVector>,

    // Attestations
    pub previous_epoch_attestations:
        VariableList<PendingAttestation<C>, C::MaxAttestationsPerEpoch>,
    pub current_epoch_attestations: VariableList<PendingAttestation<C>, C::MaxAttestationsPerEpoch>,

    // Finality
    pub justification_bits: BitVector<consts::JustificationBitsLength>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
}

impl<C: Config> BeaconState<C> {
    pub fn canonical_root(&self) -> Hash256 {
        Hash256::from_slice(&self.tree_hash_root()[..])
    }

    pub fn update_tree_hash_cache(&mut self) -> Result<Hash256, Error> {
        Ok(Hash256::from_slice(&self.tree_hash_root()))
    }

    fn get_latest_block_roots_index(&self, slot: Slot) -> Result<usize, Error> {
        if (slot < self.slot) && (self.slot <= slot + self.block_roots.len() as u64) {
            let b = slot as usize;
            Ok(b % self.block_roots.len())
        } else {
            Err(Error::SlotOutOfBounds)
        }
    }

    fn get_latest_state_roots_index(&self, slot: Slot) -> Result<usize, Error> {
        if (slot < self.slot) && (self.slot <= slot + Slot::from(self.state_roots.len() as u64)) {
            let b = slot as usize;
            Ok(b % self.state_roots.len())
        } else {
            Err(Error::SlotOutOfBounds)
        }
    }

    pub fn set_state_root(&mut self, slot: Slot, state_root: Hash256) -> Result<(), Error> {
        let i = self.get_latest_state_roots_index(slot)?;
        self.state_roots[i] = state_root;
        Ok(())
    }

    pub fn set_block_root(&mut self, slot: Slot, block_root: Hash256) -> Result<(), Error> {
        let i = self.get_latest_block_roots_index(slot)?;
        self.block_roots[i] = block_root;
        Ok(())
    }
}
