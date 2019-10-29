use crate::*;
use types::{ beacon_state::*, config::MainnetConfig};
// use types::*;
use ethereum_types::{H256 as Hash256};
use types::{types::*};
#[derive(Debug, PartialEq)]
pub enum Error {
    
}

pub fn process_slot(
    state: &mut BeaconState<MainnetConfig>,
    genesis_slot: u64,
) -> Result<(), Error> {

    cache_state(state)?;

    if state.slot > genesis_slot /*&& (state.slot + 1) % T::slots_per_epoch() == 0*/ {
        // ! implement per_epoch_processing
        // per_epoch_processing(state, spec)?;
    }

    state.slot += 1;

    Ok(())
}

fn cache_state(state: &mut BeaconState<MainnetConfig>) -> Result<(), Error> {
    //?let previous_state_root = state.update_tree_hash_cache()?;

    // ! FIX THIS :( @pikaciu22x
    // Note: increment the state slot here to allow use of our `state_root` and `block_root`
    // getter/setter functions.
    //
    // This is a bit hacky, however it gets the job safely without lots of code.
    let previous_slot = state.slot;
    state.slot += 1;

    // Store the previous slot's post state transition root.
    //?state.set_state_root(previous_slot, previous_state_root)?;

    // Cache latest block header state root
    if state.latest_block_header.state_root == Hash256::zero() {
        // state.latest_block_header.state_root = previous_state_root;
    }

    // Cache block root
    //? let latest_block_root = state.latest_block_header.canonical_root();
    //? state.set_block_root(previous_slot, latest_block_root)?;

    // Set the state slot back to what it should be.
    state.slot -= 1;

    Ok(())
}


#[cfg(test)]
mod process_slot_tests {
    use types::{beacon_state::*, config::MainnetConfig};
    // use crate::{config::*};
    use super::*;

    #[test]
    fn process_good_slot() {
        let mut bs: BeaconState<MainnetConfig> = BeaconState {
            ..BeaconState::default()
        };

        process_slot(&mut bs, 0);

        assert_eq!(bs.slot, 1);
    }
    #[test]
    fn process_good_slot_2() {
        let mut bs: BeaconState<MainnetConfig> = BeaconState {
            slot: 3,
            ..BeaconState::default()
        };

        process_slot(&mut bs, 0);

        assert_eq!(bs.slot, 4);
    }
} 

