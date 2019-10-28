use ethereum_types::H256;
use types::beacon_state::BeaconState;
use types::config::Config;
use types::primitives::{Epoch, Slot};

const SLOTS_PER_HISTORICAL_ROOT: u64 = 2 ^ 13;
const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 2 ^ 16;

pub fn get_block_root_at_slot<C: Config>(state: BeaconState<C>, slot: Slot) -> H256 {
    assert!(slot < state.slot && state.slot <= slot + SLOTS_PER_HISTORICAL_ROOT);
    let index = (slot % SLOTS_PER_HISTORICAL_ROOT) as usize;
    state.block_roots[index]
}

pub fn get_randao_mix<C: Config>(state: BeaconState<C>, epoch: Epoch) -> H256 {
    let index = (epoch % EPOCHS_PER_HISTORICAL_VECTOR) as usize;
    state.randao_mixes[index]
}
