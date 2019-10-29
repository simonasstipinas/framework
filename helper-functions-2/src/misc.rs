use types::consts::MAX_SEED_LOOKAHEAD;
use types::consts::SLOTS_PER_EPOCH;
use types::primitives::Epoch;
use types::primitives::Slot;

pub fn compute_epoch_at_slot(slot: Slot) -> Epoch {
    let slot_f64 = slot as f64;
    (slot_f64 / SLOTS_PER_EPOCH as f64) as u64
}

pub fn compute_start_slot_at_epoch(epoch: Epoch) -> Slot {
    epoch * SLOTS_PER_EPOCH
}

pub fn compute_activation_exit_epoch(epoch: Epoch) -> Epoch {
    epoch + 1 + MAX_SEED_LOOKAHEAD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_at_slot() {
        assert_eq!(compute_epoch_at_slot(65), 2);
        assert_eq!(compute_epoch_at_slot(64), 2);
        assert_eq!(compute_epoch_at_slot(63), 1);
    }

    #[test]
    fn test_start_slot_at_epoch() {
        assert_eq!(compute_start_slot_at_epoch(2), 64);
        assert_ne!(compute_start_slot_at_epoch(2), 63);
        assert_ne!(compute_start_slot_at_epoch(2), 65);
    }

    #[test]
    fn test_activation_exit_epoch() {
        assert_eq!(compute_activation_exit_epoch(1), 6);
    }
}
