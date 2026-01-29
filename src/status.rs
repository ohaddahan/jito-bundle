use crate::constants::DEFAULT_INITIAL_CONFIRM_DELAY_SECS;
use crate::error::JitoError;
use crate::types::{BundleStatus, JsonRpcRequest, JsonRpcResponse};
use reqwest::Client;
use serde::Deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status_client_types::TransactionConfirmationStatus;
use std::str::FromStr;
use std::time::Duration;

pub struct StatusHelper;

impl StatusHelper {
    pub async fn get_bundle_status(
        client: &Client,
        endpoint: &str,
        bundle_id: &str,
    ) -> BundleStatus {
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "getBundleStatuses",
            params: [[bundle_id]],
        };

        let response = match client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => return BundleStatus::Unknown,
        };

        let status_code = response.status();
        if status_code.as_u16() == 429 {
            return BundleStatus::Pending;
        }
        if !status_code.is_success() {
            return BundleStatus::Unknown;
        }

        let response_text = match response.text().await {
            Ok(t) => t,
            Err(_) => return BundleStatus::Unknown,
        };

        #[derive(Debug, Deserialize)]
        struct BundleStatusValue {
            confirmation_status: Option<String>,
            slot: Option<u64>,
            err: Option<serde_json::Value>,
        }

        #[derive(Debug, Deserialize)]
        struct StatusResult {
            value: Vec<BundleStatusValue>,
        }

        let parsed: JsonRpcResponse<StatusResult> = match serde_json::from_str(&response_text) {
            Ok(p) => p,
            Err(_) => return BundleStatus::Unknown,
        };

        if let Some(result) = parsed.result
            && let Some(status) = result.value.first()
        {
            if let Some(err) = &status.err {
                return BundleStatus::Failed {
                    error: Some(err.to_string()),
                };
            }

            if let Some(confirmation_status) = &status.confirmation_status
                && (confirmation_status == "confirmed" || confirmation_status == "finalized")
            {
                return BundleStatus::Landed { slot: status.slot };
            }

            return BundleStatus::Pending;
        }

        BundleStatus::Unknown
    }

    pub async fn wait_for_landing_on_chain(
        signatures: &[String],
        rpc_client: &RpcClient,
        max_attempts: u32,
        interval_ms: u64,
    ) -> Result<BundleStatus, JitoError> {
        let parsed_signatures: Vec<Signature> = signatures
            .iter()
            .map(|s| Signature::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JitoError::InvalidSignature {
                reason: e.to_string(),
            })?;

        tokio::time::sleep(Duration::from_secs(DEFAULT_INITIAL_CONFIRM_DELAY_SECS)).await;

        for _attempt in 0..max_attempts {
            if let Ok(statuses) = rpc_client.get_signature_statuses(&parsed_signatures).await {
                for (j, status) in statuses.value.iter().enumerate() {
                    if let Some(s) = status
                        && let Some(err) = &s.err
                    {
                        return Ok(BundleStatus::Failed {
                            error: Some(format!("transaction {j} failed: {err:?}")),
                        });
                    }
                }
                let all_confirmed = statuses.value.iter().all(|status| {
                    status.as_ref().is_some_and(|s| {
                        s.confirmation_status.as_ref().is_some_and(|cs| {
                            cs == &TransactionConfirmationStatus::Confirmed
                                || cs == &TransactionConfirmationStatus::Finalized
                        })
                    })
                });
                if all_confirmed {
                    let slot = statuses
                        .value
                        .first()
                        .and_then(|s| s.as_ref().map(|s| s.slot));
                    return Ok(BundleStatus::Landed { slot });
                }
            }
            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }

        Err(JitoError::ConfirmationTimeout {
            attempts: max_attempts,
        })
    }

    pub async fn wait_for_landing_via_jito(
        client: &Client,
        endpoint_fn: impl Fn() -> String,
        bundle_id: &str,
        max_attempts: u32,
        interval_ms: u64,
    ) -> Result<BundleStatus, JitoError> {
        tokio::time::sleep(Duration::from_secs(DEFAULT_INITIAL_CONFIRM_DELAY_SECS)).await;

        for _attempt in 0..max_attempts {
            let endpoint = endpoint_fn();
            let status = Self::get_bundle_status(client, &endpoint, bundle_id).await;

            match &status {
                BundleStatus::Landed { .. } | BundleStatus::Failed { .. } => {
                    return Ok(status);
                }
                _ => {}
            }

            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }

        Err(JitoError::ConfirmationTimeout {
            attempts: max_attempts,
        })
    }
}
