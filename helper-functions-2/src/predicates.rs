use types::types::Validator;
use types::primitives::Epoch;

// Check if validator is active
pub fn is_active_validator(validator: &Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u64::MAX as epoch_max;
    use bls::{PublicKey, SecretKey};
    use types::primitives::H256;

    fn default_validator() -> Validator {
        Validator{
            effective_balance: 0,
            slashed: false,
            activation_eligibility_epoch: epoch_max,
            activation_epoch: epoch_max,
            exit_epoch: epoch_max,
            withdrawable_epoch: epoch_max,
            withdrawal_credentials: H256([0; 32]),
            pubkey: PublicKey::from_secret_key(&SecretKey::random())
        }
    }

    #[test]
    fn test_not_activated() {
        let validator = default_validator();
        let epoch: u64 = 10;

        assert!(is_active_validator(&validator, epoch) == false);
    }

    #[test]
    fn test_activated() {
        let mut validator = default_validator();
        validator.activation_epoch = 4;
        let epoch: u64 = 10;

        assert!(is_active_validator(&validator, epoch));
    }

    #[test]
    fn test_exited() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.exit_epoch = 10;
        let epoch: u64 = 10;

        assert!(!is_active_validator(&validator, epoch));
    }


}