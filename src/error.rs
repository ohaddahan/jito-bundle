use thiserror::Error;

#[derive(Debug, Error)]
pub enum JitoError {
    #[error("bundle must contain at least 1 transaction, got {count}")]
    InvalidBundleSize { count: usize },
    #[error(
        "tip account {tip_account} found in address lookup table — this will cause a runtime failure"
    )]
    TipAccountInLut { tip_account: String },
    #[error("transaction {index} exceeds max size: {size}/{max} bytes")]
    TransactionOversized {
        index: usize,
        size: usize,
        max: usize,
    },
    #[error("failed to compile v0 message for transaction {index}: {reason}")]
    MessageCompileFailed { index: usize, reason: String },
    #[error("failed to create versioned transaction {index}: {reason}")]
    TransactionCreationFailed { index: usize, reason: String },
    #[error("simulation failed: {details}")]
    SimulationFailed { details: String },
    #[error("bundle rejected by Jito: {reason}")]
    BundleRejected { reason: String },
    #[error("all {count} endpoints failed, last error: {last_error}")]
    AllEndpointsFailed { count: usize, last_error: String },
    #[error("bundle confirmation timed out after {attempts} attempts")]
    ConfirmationTimeout { attempts: u32 },
    #[error("bundle failed on-chain: {reason}")]
    OnChainFailure { reason: String },
    #[error("failed to fetch tip floor: {reason}")]
    TipFloorFetchFailed { reason: String },
    #[error("serialization error: {reason}")]
    Serialization { reason: String },
    #[error("JSON-RPC error ({code}): {message}")]
    JsonRpc { code: i64, message: String },
    #[error("network error: {reason}")]
    Network { reason: String },
    #[error("invalid signature format: {reason}")]
    InvalidSignature { reason: String },
}
