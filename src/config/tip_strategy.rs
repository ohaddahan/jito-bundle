use crate::constants::{DEFAULT_TIP_LAMPORTS, MAX_TIP_LAMPORTS};

#[derive(Debug, Clone)]
pub enum TipStrategy {
    Fixed(u64),
    FetchFloor,
    FetchFloorWithCap { min: u64, max: u64 },
}

impl Default for TipStrategy {
    fn default() -> Self {
        TipStrategy::FetchFloorWithCap {
            min: DEFAULT_TIP_LAMPORTS,
            max: MAX_TIP_LAMPORTS,
        }
    }
}
