use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BundleStatusValue {
    pub confirmation_status: Option<String>,
    pub slot: Option<u64>,
    pub err: Option<serde_json::Value>,
}
#[derive(Debug, Deserialize)]
pub struct StatusResult {
    pub value: Vec<BundleStatusValue>,
}
