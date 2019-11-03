fn process_proposer_slashing(state: &mut BeaconState<MainnetConfig>, proposer_slashing: ProposerSlashing){
    let proposer = state.validators[proposer_slashing.proposer_index];
    // Verify slots match
    assert_eq!(proposer_slashing.header_1.slot, proposer_slashing.header_2.slot);
    // But the headers are different
    assert_ne!(proposer_slashing.header_1, proposer_slashing.header_2);
    // Check proposer is slashable
    //!assert(is_slashable_validator(proposer, get_current_epoch(state))); 
    // Signatures are valid
    //?for header in (proposer_slashing.header_1, proposer_slashing.header_2_{
    //!    let domain = get_domain(state, DOMAIN_BEACON_PROPOSER, compute_epoch_at_slot(header.slot));
    //!    assert!(bls_verify(proposer.pubkey, signing_root(header), header.signature, domain)) ;
    }

    //!slash_validator(state, proposer_slashing.proposer_index);
}

fn process_attester_slashing(state: &mut BeaconState<MainnetConfig>, attester_slashing: AttesterSlashing){
    let attestation_1 = attester_slashing.attestation_1;
    assert!(is_slashable_attestation_data(attestation_1.data, attestation_2.data));
    let attestation_2 = attester_slashing.attestation_2;
    //!assert!(is_valid_indexed_attestation(state, attestation_1));
    //!assert!(is_valid_indexed_attestation(state, attestation_2)); 

    let mut slashed_any = false;
    let attesting_indices_1 = attestation_1.custody_bit_0_indices + attestation_1.custody_bit_1_indices;
    let attesting_indices_2 = attestation_2.custody_bit_0_indices + attestation_2.custody_bit_1_indices;
    //?for index in sorted(set(attesting_indices_1).intersection(attesting_indices_2)){
    //!    if is_slashable_validator(state.validators[index], get_current_epoch(state)){
    //!        slash_validator(state, index)
    //!        slashed_any = true;
    //!     }
    //! }
    assert!(slashed_any);
}

fn process_attestation(state: &mut BeaconState<MainnetConfig>, attestation: Attestation){
    let data = attestation.data;
    //!assert!(data.index < get_committee_count_at_slot(state, data.slot)); 
    //!assert!(data.target.epoch in (get_previous_epoch(state), get_current_epoch(state)))
    assert!(data.slot + MIN_ATTESTATION_INCLUSION_DELAY <= state.slot <= data.slot + SLOTS_PER_EPOCH);

    //!let committee = get_beacon_committee(state, data.slot, data.index);
    assert_eq!(attestation.aggregation_bits.len(), attestation.custody_bits.len());
    assert_eq!(attestation.custody_bits.len(), committee.len());

    //?let pending_attestation = PendingAttestation(
    //?    data=data,
    //?    aggregation_bits=attestation.aggregation_bits,
    //?    inclusion_delay=state.slot - data.slot,
    //?    proposer_index=get_beacon_proposer_index(state),
    //?)

//!    if data.target.epoch == get_current_epoch(state){
        assert_eq! (data.source, state.current_justified_checkpoint);
        state.current_epoch_attestations.append(pending_attestation);
    }
    else{
        assert_eq! (data.source, state.previous_justified_checkpoint);
        state.previous_epoch_attestations.append(pending_attestation);
    }

    # Check signature
    //!assert! (is_valid_indexed_attestation(state, get_indexed_attestation(state, attestation))_
}


fn process_eth1_data(state: &mut BeaconState<MainnetConfig>, body: BeaconBlockBody){
    //?state.eth1_data_votes.append(body.eth1_data);
    if state.eth1_data_votes.count(body.eth1_data) * 2 > SLOTS_PER_ETH1_VOTING_PERIOD{
        state.eth1_data = body.eth1_data;
    }
}

fn process_operations(state: &mut BeaconState<MainnetConfig>, body: BeaconBlockBody){
    # Verify that outstanding deposits are processed up to the maximum number of deposits
    assert_eq(body.deposits.len(), min(MAX_DEPOSITS, state.eth1_data.deposit_count - state.eth1_deposit_index)); 

    //?for operations, function in (
    //?    (body.proposer_slashings, process_proposer_slashing),
    //?    (body.attester_slashings, process_attester_slashing),
    //?    (body.attestations, process_attestation),
    //?    (body.deposits, process_deposit),
    //?    (body.voluntary_exits, process_voluntary_exit),
    //?    # @process_shard_receipt_proofs
    //?):
    //?    for operation in operations{
            //!function(state, operation);}
}
