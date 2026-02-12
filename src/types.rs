use serde::{Deserialize, Serialize};

/// Generic JSON-RPC request payload wrapper.
#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<T: Serialize> {
    /// JSON-RPC protocol version.
    pub jsonrpc: &'static str,
    /// Request id used for matching responses.
    pub id: u64,
    /// RPC method name.
    pub method: &'static str,
    /// Method-specific parameters.
    pub params: T,
}

/// Generic JSON-RPC response payload wrapper.
#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    /// JSON-RPC protocol version.
    pub jsonrpc: String,
    /// Response id corresponding to request id.
    pub id: u64,
    /// Success result payload when present.
    pub result: Option<T>,
    /// Error payload when present.
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error object.
#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    /// JSON-RPC error code.
    pub code: i64,
    /// Human-readable error message.
    pub message: String,
}

/// Jito tip-floor API response entry.
#[derive(Debug, Deserialize, Clone)]
pub struct JitoTipFloorResponse {
    /// Response timestamp.
    pub time: String,
    /// 25th percentile landed tip (SOL units).
    pub landed_tips_25th_percentile: f64,
    /// 50th percentile landed tip (SOL units).
    pub landed_tips_50th_percentile: f64,
    /// 75th percentile landed tip (SOL units).
    pub landed_tips_75th_percentile: f64,
    /// 95th percentile landed tip (SOL units).
    pub landed_tips_95th_percentile: f64,
    /// 99th percentile landed tip (SOL units).
    pub landed_tips_99th_percentile: f64,
    /// EMA 50th percentile landed tip (SOL units).
    pub ema_landed_tips_50th_percentile: f64,
}

/// High-level bundle landing status.
#[derive(Clone)]
pub enum BundleStatus {
    /// Bundle is still pending.
    Pending,
    /// Bundle landed with optional slot.
    Landed { slot: Option<u64> },
    /// Bundle failed with optional error details.
    Failed { error: Option<String> },
    /// Status could not be determined.
    Unknown,
}

impl std::fmt::Debug for BundleStatus {
    /// Formats bundle status in a compact human-readable form.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleStatus::Pending => write!(f, "Pending"),
            BundleStatus::Landed { slot } => write!(f, "Landed(slot: {slot:?})"),
            BundleStatus::Failed { error } => write!(f, "Failed(error: {error:?})"),
            BundleStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Result returned after bundle submission.
#[derive(Debug, Clone)]
pub struct BundleResult {
    /// Returned bundle id.
    pub bundle_id: String,
    /// Transaction signatures from submitted bundle.
    pub signatures: Vec<String>,
    /// Explorer URL for bundle status.
    pub explorer_url: String,
}

/// Parameters for Helius `simulateBundle` call.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleParams {
    /// Base64-encoded transactions in bundle order.
    pub encoded_transactions: Vec<String>,
}

/// Top-level Helius `simulateBundle` result.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleApiResult {
    /// Simulation context metadata.
    pub context: SimulateBundleContext,
    /// Simulation output payload.
    pub value: SimulateBundleValue,
}

/// Simulation context metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleContext {
    /// API version that handled simulation.
    pub api_version: String,
    /// Slot used for simulation context.
    pub slot: u64,
}

/// Full simulation output.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleValue {
    /// Overall simulation summary.
    pub summary: SimulateBundleSummary,
    /// Per-transaction simulation results.
    pub transaction_results: Vec<TransactionSimulationResult>,
}

/// Overall simulation summary state.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimulateBundleSummary {
    /// All transactions simulated successfully.
    Succeeded,
    /// Simulation failed with details.
    Failed(SimulateBundleFailure),
}

/// Failure payload returned by Helius simulation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleFailure {
    /// Raw simulation error object.
    pub error: serde_json::Value,
    /// Optional failing transaction signature.
    pub tx_signature: Option<String>,
}

impl SimulateBundleFailure {
    /// Extracts a readable error message from Helius failure payload.
    pub fn error_message(&self) -> String {
        if let Some(tx_failure) = self.error.get("TransactionFailure")
            && let Some(arr) = tx_failure.as_array()
            && let Some(msg) = arr.get(1).and_then(|v| v.as_str())
        {
            return msg.to_string();
        }
        self.error.to_string()
    }
}

/// Per-transaction simulation result entry.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSimulationResult {
    /// Transaction-level error when present.
    pub err: Option<serde_json::Value>,
    /// Program logs emitted during execution.
    pub logs: Option<Vec<String>>,
    /// Compute units consumed by transaction.
    pub units_consumed: Option<u64>,
    /// Optional program return data.
    pub return_data: Option<ReturnData>,
    /// Optional pre-execution account states.
    pub pre_execution_accounts: Option<Vec<AccountState>>,
    /// Optional post-execution account states.
    pub post_execution_accounts: Option<Vec<AccountState>>,
}

/// Account snapshot in simulation response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountState {
    /// Base58-encoded account pubkey.
    pub pubkey: String,
    /// Account lamports balance.
    pub lamports: u64,
    /// Account data payload.
    pub data: Vec<String>,
    /// Base58-encoded owner program id.
    pub owner: String,
    /// Whether account is executable.
    pub executable: bool,
    /// Account rent epoch.
    pub rent_epoch: u64,
}

/// Program return data from simulation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnData {
    /// Program id that returned the data.
    pub program_id: String,
    /// Encoded return data payload.
    pub data: Vec<String>,
}
