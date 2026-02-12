use crate::constants::{DEFAULT_TIP_LAMPORTS, MAX_TIP_LAMPORTS};

/// Strategy for selecting bundle tip lamports.
#[derive(Debug, Clone)]
pub enum TipStrategy {
    /// Uses a fixed tip amount.
    Fixed(u64),
    /// Uses raw fetched tip floor value.
    FetchFloor,
    /// Uses fetched floor clamped to configured min/max.
    FetchFloorWithCap {
        /// Minimum tip in lamports.
        min: u64,
        /// Maximum tip in lamports.
        max: u64,
    },
}

impl Default for TipStrategy {
    /// Returns default capped floor strategy.
    fn default() -> Self {
        TipStrategy::FetchFloorWithCap {
            min: DEFAULT_TIP_LAMPORTS,
            max: MAX_TIP_LAMPORTS,
        }
    }
}
