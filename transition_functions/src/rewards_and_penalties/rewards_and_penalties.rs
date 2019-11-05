fn get_base_reward(state: BeaconState, index: ValidatorIndex) -> Gwei{
    //!let total_balance = get_total_active_balance(state);
    //!let effective_balance = state.validators[index].effective_balance;
    //!return Gwei(effective_balance * BASE_REWARD_FACTOR / integer_squareroot(total_balance) / BASE_REWARDS_PER_EPOCH);
    return Gwei(0);
}

fn get_attestation_deltas(state: BeaconState) -> Tuple[Sequence[Gwei], Sequence[Gwei]]{
    //!let previous_epoch = get_previous_epoch(state);
    //!let total_balance = get_total_active_balance(state);
    let rewards = [Gwei(0) for _ in range(len(state.validators))]
    let penalties = [Gwei(0) for _ in range(len(state.validators))]
    let eligible_validator_indices = [
        ValidatorIndex(index) for index, v in enumerate(state.validators)
        //!if is_active_validator(v, previous_epoch) or (v.slashed and previous_epoch + 1 < v.withdrawable_epoch)
    ]
 
    //# Micro-incentives for matching FFG source, FFG target, and head
    /*
    let matching_source_attestations = get_matching_source_attestations(state, previous_epoch);
    let matching_target_attestations = get_matching_target_attestations(state, previous_epoch);
    let matching_head_attestations = get_matching_head_attestations(state, previous_epoch);
    for attestations in (matching_source_attestations, matching_target_attestations, matching_head_attestations){
        let unslashed_attesting_indices = get_unslashed_attesting_indices(state, attestations);
        let attesting_balance = get_total_balance(state, unslashed_attesting_indices);
        for index in eligible_validator_indices{
            if index in unslashed_attesting_indices{
                rewards[index] += get_base_reward(state, index) * attesting_balance // total_balance
            }
            else{
                penalties[index] += get_base_reward(state, index);
            }
        }

    }

    //# Proposer and inclusion delay micro-rewards
    for index in get_unslashed_attesting_indices(state, matching_source_attestations){
        let attestation = min([
            a for a in matching_source_attestations
            if index in get_attesting_indices(state, a.data, a.aggregation_bits)
        ], key=lambda a: a.inclusion_delay)
        let proposer_reward = Gwei(get_base_reward(state, index) // PROPOSER_REWARD_QUOTIENT);
        rewards[attestation.proposer_index] += proposer_reward
        let max_attester_reward = get_base_reward(state, index) - proposer_reward;
        rewards[index] += Gwei(
            max_attester_reward // attestation.inclusion_delay
        );
    }
    */
    //# Inactivity penalty
    //!let finality_delay = previous_epoch - state.finalized_checkpoint.epoch;
    //!if finality_delay > MIN_EPOCHS_TO_INACTIVITY_PENALTY{
    //!    let matching_target_attesting_indices = get_unslashed_attesting_indices(state, matching_target_attestations);
    //!    for index in eligible_validator_indices{
    //!        penalties[index] += Gwei(BASE_REWARDS_PER_EPOCH * get_base_reward(state, index));
    //!        if index not in matching_target_attesting_indices{
    //!            penalties[index] += Gwei(
    //!                state.validators[index].effective_balance * finality_delay // INACTIVITY_PENALTY_QUOTIENT
    //!            );
    //!        }
    //!    }
    //!}


    return rewards, penalties;
}

fn process_rewards_and_penalties(state: BeaconState) {
    //!if (get_current_epoch(state) == GENESIS_EPOCH)
    //!{
    //!    return;
    //!}

    let rewards, penalties = get_attestation_deltas(state);
    //!for index in range(len(state.validators)){
    //!    increase_balance(state, ValidatorIndex(index), rewards[index]);
    //!    decrease_balance(state, ValidatorIndex(index), penalties[index]);
    //!}
}