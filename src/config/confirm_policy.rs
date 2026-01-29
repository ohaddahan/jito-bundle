use crate::constants::{DEFAULT_CONFIRM_INTERVAL_MS, DEFAULT_MAX_CONFIRM_ATTEMPTS};

#[derive(Debug, Clone)]
pub struct ConfirmPolicy {
    pub max_attempts: u32,
    pub interval_ms: u64,
}

impl Default for ConfirmPolicy {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_CONFIRM_ATTEMPTS,
            interval_ms: DEFAULT_CONFIRM_INTERVAL_MS,
        }
    }
}
