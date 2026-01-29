use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<T: Serialize> {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: &'static str,
    pub params: T,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JitoTipFloorResponse {
    pub time: String,
    pub landed_tips_25th_percentile: f64,
    pub landed_tips_50th_percentile: f64,
    pub landed_tips_75th_percentile: f64,
    pub landed_tips_95th_percentile: f64,
    pub landed_tips_99th_percentile: f64,
    pub ema_landed_tips_50th_percentile: f64,
}

#[derive(Clone)]
pub enum BundleStatus {
    Pending,
    Landed { slot: Option<u64> },
    Failed { error: Option<String> },
    Unknown,
}

impl std::fmt::Debug for BundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleStatus::Pending => write!(f, "Pending"),
            BundleStatus::Landed { slot } => write!(f, "Landed(slot: {slot:?})"),
            BundleStatus::Failed { error } => write!(f, "Failed(error: {error:?})"),
            BundleStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct BundleResult {
    pub success: bool,
    pub bundle_id: Option<String>,
    pub error: Option<String>,
    pub signatures: Vec<String>,
    pub explorer_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleParams {
    pub encoded_transactions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleApiResult {
    pub context: SimulateBundleContext,
    pub value: SimulateBundleValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleContext {
    pub api_version: String,
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleValue {
    pub summary: SimulateBundleSummary,
    pub transaction_results: Vec<TransactionSimulationResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimulateBundleSummary {
    Succeeded,
    Failed(SimulateBundleFailure),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleFailure {
    pub error: serde_json::Value,
    pub tx_signature: Option<String>,
}

impl SimulateBundleFailure {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSimulationResult {
    pub err: Option<serde_json::Value>,
    pub logs: Option<Vec<String>>,
    pub units_consumed: Option<u64>,
    pub return_data: Option<ReturnData>,
    pub pre_execution_accounts: Option<Vec<AccountState>>,
    pub post_execution_accounts: Option<Vec<AccountState>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountState {
    pub pubkey: String,
    pub lamports: u64,
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnData {
    pub program_id: String,
    pub data: Vec<String>,
}
