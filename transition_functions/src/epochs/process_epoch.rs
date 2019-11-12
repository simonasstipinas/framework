use std::cmp;

fn process_final_updates<T>(state: &mut BeaconState<T>) {
    for (i, validator) in state.validators.iter().enumerate() {
        if validator.activation_eligibility_epoch == T::far_future_epoch
            && validator.effective_balance == T::max_effective_balance
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

fn process_slashings<T>(state: &mut BeaconState, spec: &ChainSpec) {
    // ! let epoch = get_current_epoch(state)
        // ! let total_balance = get_total_active_balance(state)
        for (i, validator) in state.validators.iter().enumerate() {
            if validator.slashed && epoch + T::EpochsPerSlashingsVector::to_u64() / 2 == validator.withdrawable_epoch {
                let increment = spec.effective_balance_increment; // Factored out from penalty numerator to avoid uint64 overflow
                // TODO: sum fixedvector
                let penalty_numerator = validator.effective_balance / increment * cmp::min(sum(state.slashings) * 3, total_balance);
                let penalty = penalty_numerator // total_balance * increment
                decrease_balance(state, ValidatorIndex(index), penalty)
            }
        }
}