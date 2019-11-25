use helper_functions;
use types::{ beacon_state::*, config::{ Config, MainnetConfig }};
use types::consts::*;
use types::types::*;
use core::consts::ExpConst;
use helper_functions::math::*;
use types::primitives::*;
use helper_functions::beacon_state_accessors::*;
use helper_functions::beacon_state_mutators::*;
use helper_functions::predicates::*;
use crate::attestations::attestations::AttestableBlock;

pub trait StakeholderBlock<T>
where
    T: Config + ExpConst,
{
    fn get_base_reward(
        self,
        index: ValidatorIndex
    ) -> Gwei;
    fn get_attestation_deltas(
        self
    ) -> (Vec<Gwei>, Vec<Gwei>);
    fn process_rewards_and_penalties(
        self
    );
}

impl<T> StakeholderBlock<T> for BeaconState<T>
where
    T: Config + ExpConst,
{
    fn get_base_reward(
        self,
        index: ValidatorIndex
    ) -> Gwei {
        let total_balance = get_total_active_balance(&self).unwrap();
        let effective_balance = self.validators[index as usize].effective_balance;
        return (effective_balance * T::base_reward_factor() / integer_squareroot(total_balance) / T::base_rewards_per_epoch()) as Gwei;
    }

    fn get_attestation_deltas(
        self
    ) -> (Vec<Gwei>, Vec<Gwei>) {
        let previous_epoch = get_previous_epoch(&self);
        let total_balance = get_total_active_balance(&self);
        let rewards = Vec::new();
        let penalties = Vec::new();
        for i in 0..(self.validators.len()) {
            rewards.push(0 as Gwei);
            penalties.push(0 as Gwei);
        }
        let eligible_validator_indices: Vec<Validator> = Vec::new();

        for (index, v) in self.validators.iter().enumerate() {
            if is_active_validator(v, previous_epoch) || (v.slashed && previous_epoch + 1 < v.withdrawable_epoch) {
                eligible_validator_indices.push(index as ValidatorIndex);
            }
        }
        
        //# Micro-incentives for matching FFG source, FFG target, and head
        
        let matching_source_attestations = self.get_matching_source_attestations(previous_epoch);
        let matching_target_attestations = self.get_matching_target_attestations(previous_epoch);
        let matching_head_attestations = self.get_matching_head_attestations(previous_epoch);

        for attestations in (matching_source_attestations, matching_target_attestations, matching_head_attestations).iter() {
            let unslashed_attesting_indices = self.get_unslashed_attesting_indices(attestations);
            let attesting_balance = get_total_balance(self, unslashed_attesting_indices);

            for index in eligible_validator_indices.iter() {
                if unslashed_attesting_indices.contains(&index) {
                    rewards[index] += get_base_reward(self, index) * attesting_balance // total_balance
                }
                else {
                    penalties[index] += get_base_reward(self, index);
                }
            }
        }
        let minAttestation = vec::<>;
        for a in matching_head_attestations.iter() {
            if get_unslashed_attesting_indices(self, matching_source_attestations).contains(&a) {
                minAttestation.push(a);
            }
        }
        //# Proposer and inclusion delay micro-rewards
        for index in get_unslashed_attesting_indices(self, matching_source_attestations).iter() {
            let attestation = matching_source_attestations.iter().fold(None, |min, x| match min {
                None => Some(x),
                Some(y) => Some(
                    if get_attesting_indices(self, a.data, a.aggregation_bits).contains(&index)
                    && x.inclusion_delay < y.inclusion_delay { x } else { y }),
            }).unwrap();

            let proposer_reward = Gwei(get_base_reward(self, index)); // PROPOSER_REWARD_QUOTIENT;
            rewards[attestation.proposer_index] += proposer_reward
            let max_attester_reward = get_base_reward(state, index) - proposer_reward;
            rewards[index] += Gwei(
                max_attester_reward // attestation.inclusion_delay
            );
        }
        */
        //# Inactivity penalty
        let finality_delay = previous_epoch - state.finalized_checkpoint.epoch;
        if finality_delay > MIN_EPOCHS_TO_INACTIVITY_PENALTY{
            let matching_target_attesting_indices = get_unslashed_attesting_indices(state, matching_target_attestations);
            for index in eligible_validator_indices{
                penalties[index] += Gwei(BASE_REWARDS_PER_EPOCH * get_base_reward(state, index));
                if index not in matching_target_attesting_indices{
                    penalties[index] += Gwei(
                        state.validators[index].effective_balance * finality_delay // INACTIVITY_PENALTY_QUOTIENT
                    );
                }
            }
        }
        return (rewards, penalties);
    }

    fn process_rewards_and_penalties(
        self
    ) {
        if get_current_epoch(&self) == T::genesis_epoch() {
            return;
        }
        let (rewards, penalties) = get_attestation_deltas(self);
        for index in 0..self.validators.len() {
            increase_balance(&mut self, index as u64, rewards[index]);
            decrease_balance(&mut self, index as u64, penalties[index]);
        }
    }
}

#[test]
fn test_base_reward() {
    assert_eq!(1,1);
    let mut bs: BeaconState<MainnetConfig> = BeaconState {
        ..BeaconState::default()
    };
    let mut index = 0;
    
}