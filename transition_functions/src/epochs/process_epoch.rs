use std::cmp;

fn process_final_updates<T>(state: &mut BeaconState<T>) {
    for (i, validator) in state.validators.iter().enumerate() {
        if validator.activation_eligibility_epoch == T::far_future_epoch
            && validator.effective_balance == T::max_effective_balance()
        {
            // validator.activation_eligibility_epoch = get_current_epoch(state); // ! missing helper function
        }

        // !missing helper functions
        // if is_active_validator(validator, get_current_epoch(state))
        //     && validator.effective_balance <= T::ejection_balance
        // {
        //     initiate_validator_exit(state, ValidatorIndex(i));
        // }
    }

    // TODO: Translate
    //     # Queue validators eligible for activation and not dequeued for activation prior to finalized epoch
    //     activation_queue = sorted([
    //         index for index, validator in enumerate(state.validators)
    //         if validator.activation_eligibility_epoch != FAR_FUTURE_EPOCH
    //         and validator.activation_epoch >= compute_activation_exit_epoch(state.finalized_checkpoint.epoch)
    //     ], key=lambda index: state.validators[index].activation_eligibility_epoch)
    //     # Dequeued validators for activation up to churn limit (without resetting activation epoch)
    //     for index in activation_queue[:get_validator_churn_limit(state)]:
    //         validator = state.validators[index]
    //         if validator.activation_epoch == FAR_FUTURE_EPOCH:
    //             validator.activation_epoch = compute_activation_exit_epoch(get_current_epoch(state))
}

fn process_slashings<T>(state: &mut BeaconState<T>) {
    // let epoch = get_current_epoch(state) //! missing helper function
    // let total_balance = get_total_active_balance(state) //!missing helper function
    let epoch = 1; // placeholder
    let total_balance:u64 = 1; // placeholder
    for (index, validator) in state.validators.iter().enumerate() {
        if validator.slashed && epoch + T::EpochsPerSlashingsVector / 2 == validator.withdrawable_epoch {
            let increment = T::effective_balance_increment();
            let slashings_sum = state.slashings.iter().sum::<u64>();
            let penalty_numerator = validator.effective_balance / increment * std::cmp::min(slashings_sum * 3, total_balance);
            let penalty = penalty_numerator / total_balance * increment;
            // decrease_balance(state, ValidatorIndex(index), penalty); //! missing helper function
        }
    }
}

fn process_registry_updates<T>(state: &mut BeaconState<T>) {
    for (index, validator) in state.validators.iter().enumerate() {
        if validator.activation_eligibility_epoch == T::far_future_epoch
            && validator.effective_balance == T::max_effective_balance()
        {
                // validator.activation_eligibility_epoch = get_current_epoch(state); //! missing helper function
        }
        //! missing helper functions
        // if is_active_validator(validator, get_current_epoch(state)) //! missing get_current_epoch
        //     && validator.effective_balance <= T::ejection_balance()
        // {
        //     initiate_validator_exit(state, index as ValidatorIndex); //! missing initiate_validator_exit
        // }
        //! missing helper functions
        // let activation_queue = state
        // .validators
        // .iter()
        // .enumerate()
        // .filter(|(_, validator)| {
        //     validator.activation_eligibility_epoch != T::far_future_epoch
        //         && validator.activation_epoch
        //             >= compute_activation_exit_epoch(state.finalized_checkpoint.epoch) //! missing compute_activation_exit_epoch
        // })
        // .sorted_by_key(|(_, validator)| validator.activation_eligibility_epoch)
        // .map(|(index, _)| index)
        // .collect_vec();

        // let churn_limit = get_validator_churn_limit(state); //! missing helper function get_validator_churn_limit
        for index in activation_queue.into_iter().take(churn_limit) {
            let validator = &mut state.validators[index];
            if validator.activation_epoch == T::far_future_epoch {
                // validator.activation_epoch = compute_activation_exit_epoch(get_current_epoch(state)); //! missing helper functions
            }
        }
    }
}

fn process_final_updates(state: BeaconState<T>) {
    //!current_epoch = get_current_epoch(state);
    let next_epoch = Epoch(current_epoch + 1);
    //# Reset eth1 data votes
    if (state.slot + 1) % SLOTS_PER_ETH1_VOTING_PERIOD == 0{
        state.eth1_data_votes = [];
    }
    //# Update effective balances with hysteresis
    for index, validator in enumerate(state.validators){
        let balance = state.balances[index];
        HALF_INCREMENT = EFFECTIVE_BALANCE_INCREMENT / 2;
        if balance < validator.effective_balance or validator.effective_balance + 3 * HALF_INCREMENT < balance{
            validator.effective_balance = min(balance - balance % EFFECTIVE_BALANCE_INCREMENT, MAX_EFFECTIVE_BALANCE);
        }
    }

    //# Reset slashings
    state.slashings[next_epoch % EPOCHS_PER_SLASHINGS_VECTOR] = Gwei(0);
    //# Set randao mix
    state.randao_mixes[next_epoch % EPOCHS_PER_HISTORICAL_VECTOR] = get_randao_mix(state, current_epoch);
    //# Set historical root accumulator
    if next_epoch % (SLOTS_PER_HISTORICAL_ROOT / SLOTS_PER_EPOCH) == 0{
        historical_batch = HistoricalBatch(block_roots=state.block_roots, state_roots=state.state_roots);
        state.historical_roots.append(hash_tree_root(historical_batch));
    }

    //# Rotate current/previous epoch attestations
    state.previous_epoch_attestations = state.current_epoch_attestations;
    state.current_epoch_attestations = [];
}