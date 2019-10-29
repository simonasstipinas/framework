// use crate::{config::*, consts, error::Error, primitives::*, types::*};
// use ssz_types::{BitVector, FixedVector, VariableList};

#[cfg(test)]
mod process_slot {
    use types::{beacon_state::*, config::MainnetConfig};
    use types::{types::*};
    // use crate::{config::*};

    #[test]
    fn process_good_slot() {
        let mut bs: BeaconState<MainnetConfig> = BeaconState {
            ..BeaconState::default()
        };

        let mut bs: BeaconState<MainnetConfig> = BeaconState {
            ..BeaconState::default()
        };
        let bb: BeaconBlock<MainnetConfig> = BeaconBlock{
            ..BeaconBlock::default()
        };
        assert!(bs.slot == bb.slot);
       // assert_eq!(2 + 2, 4);
    }
} 

