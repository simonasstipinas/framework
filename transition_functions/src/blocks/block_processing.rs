fn process_proposer_slashing(state: &mut BeaconState<MainnetConfig>, proposer_slashing: ProposerSlashing){
    let proposer = state.validators[proposer_slashing.proposer_index];
    // Verify slots match
    assert_eq!(proposer_slashing.header_1.slot, proposer_slashing.header_2.slot);
    // But the headers are different
    assert_ne!(proposer_slashing.header_1, proposer_slashing.header_2);
    // Check proposer is slashable
    //!assert(is_slashable_validator(proposer, get_current_epoch(state))); 
    // Signatures are valid
    //!for header in (proposer_slashing.header_1, proposer_slashing.header_2){
    //!    domain = get_domain(state, DOMAIN_BEACON_PROPOSER, compute_epoch_at_slot(header.slot));
    //!    assert bls_verify(proposer.pubkey, signing_root(header), header.signature, domain);
    //!}

    //!slash_validator(state, proposer_slashing.proposer_index);
}