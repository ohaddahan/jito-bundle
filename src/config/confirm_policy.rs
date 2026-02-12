use crate::constants::{DEFAULT_CONFIRM_INTERVAL_MS, DEFAULT_MAX_CONFIRM_ATTEMPTS};

/// Polling policy used when waiting for landing confirmation.
#[derive(Debug, Clone)]
pub struct ConfirmPolicy {
    /// Maximum number of polling attempts.
    pub max_attempts: u32,
    /// Delay between polling attempts in milliseconds.
    pub interval_ms: u64,
}

impl Default for ConfirmPolicy {
    /// Returns default confirmation polling settings.
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_CONFIRM_ATTEMPTS,
            interval_ms: DEFAULT_CONFIRM_INTERVAL_MS,
        }
    }
}
