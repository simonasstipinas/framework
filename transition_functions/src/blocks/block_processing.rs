fn process_voluntary_exit(state: &mut BeaconState<T>,exit: VoluntaryExit){
    let validator = state.validators[exit.validator_index];
    // Verify the validator is active
    //!assert! (is_active_validator(validator, get_current_epoch(state)))
    // Verify the validator has not yet exited
    assert! (validator.exit_epoch == FAR_FUTURE_EPOCH);
    // Exits must specify an epoch when they become valid; they are not valid before then
    //!assert! (get_current_epoch(state) >= exit.epoch)
    // Verify the validator has been active long enough
    //!assert! (get_current_epoch(state) >= validator.activation_epoch + PERSISTENT_COMMITTEE_PERIOD)
    // Verify signature
    //!domain = get_domain(state, DOMAIN_VOLUNTARY_EXIT, exit.epoch)
    //!assert! (bls_verify(validator.pubkey, signing_root(exit), exit.signature, domain))
    // Initiate exit
    //!initiate_validator_exit(state, exit.validator_index)
}

fn process_deposit(state: &mut BeaconState<T>, deposit: Deposit) { 
    //# Verify the Merkle branch  is_valid_merkle_branch
    //! what it do
    assert!(
       //? leaf=hash_tree_root(deposit.data),
        branch=deposit.proof &&
        depth=DEPOSIT_CONTRACT_TREE_DEPTH + 1 &&  //# Add 1 for the `List` length mix-in
        index=state.eth1_deposit_index &&
        root=state.eth1_data.deposit_root
    )

    //# Deposits must be processed in order
    state.eth1_deposit_index += 1

    let pubkey = deposit.data.pubkey;
    let amount = deposit.data.amount;
    let validator_pubkeys = [v.pubkey for v in state.validators];
    if pubkey not in validator_pubkeys:
        //# Verify the deposit signature (proof of possession) for new validators.
        //# Note: The deposit contract does not check signatures.
        //# Note: Deposits are valid across forks, thus the deposit domain is retrieved directly from `compute_domain`.
        //!let domain = compute_domain(DOMAIN_DEPOSIT)
        //!if not bls_verify(pubkey, signing_root(deposit.data), deposit.data.signature, domain):
          //!  return

        //# Add validator and balance entries
        state.validators.append(Validator(
            pubkey=pubkey,
            withdrawal_credentials=deposit.data.withdrawal_credentials,
            activation_eligibility_epoch=FAR_FUTURE_EPOCH,
            activation_epoch=FAR_FUTURE_EPOCH,
            exit_epoch=FAR_FUTURE_EPOCH,
            withdrawable_epoch=FAR_FUTURE_EPOCH,
            effective_balance=min(amount - amount % EFFECTIVE_BALANCE_INCREMENT, MAX_EFFECTIVE_BALANCE),
        ))
        state.balances.append(amount)
    else{
        # Increase balance by deposit amount
        index = ValidatorIndex(validator_pubkeys.index(pubkey))
        //!increase_balance(state, index, amount)
    }
}

fn process_block_header(state: BeaconState<T>, block: BeaconBlock) {
    //# Verify that the slots match
    assert! (block.slot == state.slot)
    //# Verify that the parent matches
    //!assert! (block.parent_root == signing_root(state.latest_block_header));
    //# Save current block as the new latest block
    //? check if its ok in rust
    state.latest_block_header = BeaconBlockHeader(
        slot=block.slot,
        parent_root=block.parent_root,
        //# `state_root` is zeroed and overwritten in the next `process_slot` call
        body_root=hash_tree_root(block.body),
        //# `signature` is zeroed
    )
    //# Verify proposer is not slashed
    //!proposer = state.validators[get_beacon_proposer_index(state)];
    assert! (not proposer.slashed);
    //# Verify proposer signature
    //!assert! (bls_verify(proposer.pubkey, signing_root(block), block.signature, get_domain(state, DOMAIN_BEACON_PROPOSER)));
}

fn process_randao(state: BeaconState, body: BeaconBlockBody) {
    //!epoch = get_current_epoch(state)
    //# Verify RANDAO reveal
    //!proposer = state.validators[get_beacon_proposer_index(state)]
    //!assert bls_verify(proposer.pubkey, hash_tree_root(epoch), body.randao_reveal, get_domain(state, DOMAIN_RANDAO))
    //# Mix in RANDAO reveal
    //!mix = xor(get_randao_mix(state, epoch), hash(body.randao_reveal))
    state.randao_mixes[epoch % EPOCHS_PER_HISTORICAL_VECTOR] = mix;
}

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

    let pending_attestation = PendingAttestation(
        attestation.aggregation_bits,
        data,
        state.slot - data.slot,
        get_beacon_proposer_index(state)
    )

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
    //# Verify that outstanding deposits are processed up to the maximum number of deposits
    assert_eq(body.deposits.len(), min(MAX_DEPOSITS, state.eth1_data.deposit_count - state.eth1_deposit_index)); 

    for operations, function in (
        (body.proposer_slashings, process_proposer_slashing),
        (body.attester_slashings, process_attester_slashing),
        (body.attestations, process_attestation),
        (body.deposits, process_deposit),
        (body.voluntary_exits, process_voluntary_exit),
        //?process_shard_receipt_proofs
    ){
        for operation in operations{
            //!function(state, operation);
    }
       
}
