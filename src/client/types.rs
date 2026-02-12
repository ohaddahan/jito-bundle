use serde::Deserialize;

/// One bundle status entry returned by `getBundleStatuses`.
#[derive(Debug, Deserialize)]
pub struct BundleStatusValue {
    /// Confirmation status string from block engine.
    pub confirmation_status: Option<String>,
    /// Slot where bundle landed, when available.
    pub slot: Option<u64>,
    /// Raw error object when status is failed.
    pub err: Option<serde_json::Value>,
}
/// Parsed payload wrapper for `getBundleStatuses`.
#[derive(Debug, Deserialize)]
pub struct StatusResult {
    /// Returned status entries.
    pub value: Vec<BundleStatusValue>,
}
