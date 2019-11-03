fn process_voluntary_exit(state: &mut BeaconState<T>,exit: VoluntaryExit){
    let validator = state.validators[exit.validator_index];
    // Verify the validator is active
    //!assert! (is_active_validator(validator, get_current_epoch(state)))
    // Verify the validator has not yet exited
    assert! validator.exit_epoch == FAR_FUTURE_EPOCH
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