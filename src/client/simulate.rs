use crate::bundler::bundle::Bundle;
use crate::client::jito_bundler::JitoBundler;
use crate::error::JitoError;
use crate::types::{
    JsonRpcRequest, JsonRpcResponse, SimulateBundleApiResult, SimulateBundleParams,
    SimulateBundleSummary, SimulateBundleValue,
};
use base64::Engine;
use solana_client::rpc_config::RpcSimulateTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;

impl JitoBundler {
    pub async fn simulate_per_transaction(&self, bundle: &Bundle<'_>) -> Result<(), JitoError> {
        for (i, tx) in bundle.versioned_transaction.iter().enumerate() {
            let sig = bs58::encode(&tx.signatures[0]).into_string();
            let config = RpcSimulateTransactionConfig {
                sig_verify: true,
                replace_recent_blockhash: false,
                commitment: Some(CommitmentConfig::confirmed()),
                accounts: None,
                min_context_slot: None,
                inner_instructions: false,
                encoding: None,
            };

            match self
                .rpc_client
                .simulate_transaction_with_config(tx, config)
                .await
            {
                Ok(result) => {
                    if let Some(err) = result.value.err {
                        let logs = result.value.logs.unwrap_or_default();
                        return Err(JitoError::SimulationFailed {
                            details: format!(
                                "transaction {i} simulation failed: {err:?}\nsignature: {sig}\nlogs: {logs:?}"
                            ),
                        });
                    }
                }
                Err(e) => {
                    return Err(JitoError::SimulationFailed {
                        details: format!(
                            "transaction {i} RPC simulation error: {e}\nsignature: {sig}"
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    pub async fn simulate_bundle_helius(
        &self,
        bundle: &Bundle<'_>,
        helius_rpc_url: &str,
    ) -> Result<SimulateBundleValue, JitoError> {
        let encoded_transactions: Vec<String> = bundle
            .versioned_transaction
            .iter()
            .map(|tx| {
                let serialized = bincode::serialize(tx).map_err(|e| JitoError::Serialization {
                    reason: e.to_string(),
                })?;
                Ok(base64::engine::general_purpose::STANDARD.encode(serialized))
            })
            .collect::<Result<Vec<String>, JitoError>>()?;
        let params = SimulateBundleParams {
            encoded_transactions,
        };
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "simulateBundle",
            params: [params],
        };
        let response = self
            .http_client
            .post(helius_rpc_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| JitoError::Network {
                reason: format!("Helius simulateBundle: {e}"),
            })?;
        let status = response.status();
        let response_text = response.text().await.map_err(|e| JitoError::Network {
            reason: format!("failed to read Helius response: {e}"),
        })?;
        if !status.is_success() {
            return Err(JitoError::Network {
                reason: format!("Helius simulateBundle HTTP {status}: {response_text}"),
            });
        }
        let parsed: JsonRpcResponse<SimulateBundleApiResult> = serde_json::from_str(&response_text)
            .map_err(|e| JitoError::Serialization {
                reason: format!(
                    "failed to parse Helius simulateBundle response: {e}, body: {response_text}"
                ),
            })?;
        if let Some(error) = parsed.error {
            return Err(JitoError::JsonRpc {
                code: error.code,
                message: error.message,
            });
        }

        let result = parsed.result.ok_or_else(|| JitoError::JsonRpc {
            code: -1,
            message: "no result in Helius simulateBundle response".to_string(),
        })?;

        if let SimulateBundleSummary::Failed(failure) = &result.value.summary {
            let tx_count = bundle.versioned_transaction.len();
            let results_count = result.value.transaction_results.len();
            let failed_tx_index = if results_count < tx_count {
                results_count
            } else {
                result
                    .value
                    .transaction_results
                    .iter()
                    .position(|r| r.err.is_some())
                    .unwrap_or(0)
            };
            let mut error_details = format!(
                "bundle simulation failed at transaction {failed_tx_index}: {}\n\
                 failing tx signature: {:?}\n\
                 bundle size: {tx_count} transactions, results returned: {results_count}",
                failure.error_message(),
                failure.tx_signature
            );

            if results_count < tx_count {
                error_details.push_str(&format!(
                    "\nHelius stopped after tx {failed_tx_index} failed — no logs for subsequent transactions"
                ));
            }

            for (idx, tx_result) in result.value.transaction_results.iter().enumerate() {
                let status_str = if tx_result.err.is_some() {
                    "FAILED"
                } else {
                    "OK"
                };
                let units = tx_result
                    .units_consumed
                    .map_or_else(|| "N/A".to_string(), |u| u.to_string());

                error_details.push_str(&format!("\n\n=== transaction {idx} [{status_str}] ==="));
                error_details.push_str(&format!("\ncompute units: {units}"));

                if let Some(err) = &tx_result.err {
                    error_details.push_str(&format!("\nerror: {err}"));
                }

                if let Some(logs) = &tx_result.logs {
                    error_details.push_str("\nlogs:");
                    for log in logs {
                        error_details.push_str(&format!("\n  {log}"));
                    }
                } else {
                    error_details.push_str("\nlogs: none");
                }
            }

            return Err(JitoError::SimulationFailed {
                details: error_details,
            });
        }

        Ok(result.value)
    }
}
