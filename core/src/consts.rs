use types::config::MainnetConfig;

pub trait ExpConst {
    fn far_future_epoch() -> u64 {
        u64::max_value()
    }
}

impl ExpConst for MainnetConfig {}
