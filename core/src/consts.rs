use types::config::MainnetConfig;

pub trait ExpConst {
    fn far_future_epoch() -> u64 {
        u64::max_value()
    }
    fn epochs_per_slashings_vector() -> u64 {
        8192
    }
    fn base_rewards_per_epoch() -> u64 {
        4
    }
    fn slots_per_epoch() -> u64 {
        32
    }
}

impl ExpConst for MainnetConfig {}
