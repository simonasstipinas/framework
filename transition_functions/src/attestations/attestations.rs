fn get_matching_source_attestations(state: BeaconState<T>, epoch: Epoch) /*-> Sequence[PendingAttestation]*/{
    //!assert epoch in (get_previous_epoch(state), get_current_epoch(state))
    //!return state.current_epoch_attestations if epoch == get_current_epoch(state) else state.previous_epoch_attestations
}

fn get_matching_target_attestations(state: BeaconState<T>, epoch: Epoch) /*-> Sequence[PendingAttestation]:*/{
    /*return [
        a for a in get_matching_source_attestations(state, epoch)
        if a.data.target.root == get_block_root(state, epoch)
    ]*/
}

fn get_matching_head_attestations(state: BeaconState<T>, epoch: Epoch) /*-> Sequence[PendingAttestation] */{
    /*return [
        a for a in get_matching_source_attestations(state, epoch)
        if a.data.beacon_block_root == get_block_root_at_slot(state, a.data.slot)
    ]*/
}

fn get_unslashed_attesting_indices(state: BeaconState<T>,
                                    attestations: Sequence[PendingAttestation]) /*-> Set[ValidatorIndex]*/{
    /*let mut output = set();  //# type: Set[ValidatorIndex]
    for a in attestations:
        output = output.union(get_attesting_indices(state, a.data, a.aggregation_bits))
    return set(filter(lambda index: not state.validators[index].slashed, output))*/
}

fn get_attesting_balance(state: BeaconState<T>, attestations: Sequence[PendingAttestation]) -> Gwei{
    //!return get_total_balance(state, get_unslashed_attesting_indices(state, attestations));
}
