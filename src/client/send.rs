use crate::bundler::bundle::types::Bundle;
use crate::client::jito_bundler::JitoBundler;
use crate::constants::JITO_EXPLORER_URL;
use crate::error::JitoError;
use crate::types::{BundleResult, JsonRpcRequest, JsonRpcResponse};
use base64::Engine;
use serde::Serialize;
use solana_sdk::transaction::VersionedTransaction;

impl JitoBundler {
    pub fn get_jito_explorer_url(bundle_id: &str) -> String {
        format!("{JITO_EXPLORER_URL}/{bundle_id}")
    }

    pub fn encode_transactions(
        transactions: &[VersionedTransaction],
    ) -> Result<Vec<String>, JitoError> {
        transactions
            .iter()
            .map(|tx| {
                let serialized = bincode::serialize(tx).map_err(|e| JitoError::Serialization {
                    reason: e.to_string(),
                })?;
                Ok(base64::engine::general_purpose::STANDARD.encode(serialized))
            })
            .collect()
    }

    pub fn extract_signatures(transactions: &[VersionedTransaction]) -> Vec<String> {
        transactions
            .iter()
            .map(|tx| bs58::encode(&tx.signatures[0]).into_string())
            .collect()
    }

    pub async fn send_bundle(&self, bundle: &Bundle<'_>) -> Result<BundleResult, JitoError> {
        let encoded_txs = Self::encode_transactions(&bundle.versioned_transaction)?;
        let signatures = Self::extract_signatures(&bundle.versioned_transaction);
        let endpoints = self.config.network.send_endpoints();
        let mut last_error = String::from("no endpoints available");
        for endpoint in &endpoints {
            match self.send_bundle_to_endpoint(endpoint, &encoded_txs).await {
                Ok(bundle_id) => {
                    let explorer_url = Self::get_jito_explorer_url(&bundle_id);
                    return Ok(BundleResult {
                        success: true,
                        bundle_id: Some(bundle_id),
                        error: None,
                        signatures,
                        explorer_url: Some(explorer_url),
                    });
                }
                Err(e) => {
                    tracing::warn!("endpoint {endpoint} failed: {e}");
                    last_error = e.to_string();
                }
            }
        }

        Err(JitoError::AllEndpointsFailed {
            count: endpoints.len(),
            last_error,
        })
    }

    async fn send_bundle_to_endpoint(
        &self,
        endpoint: &str,
        encoded_txs: &[String],
    ) -> Result<String, JitoError> {
        #[derive(Serialize)]
        struct BundleParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            encoding: Option<&'a str>,
        }

        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "sendBundle",
            params: (
                encoded_txs,
                BundleParams {
                    encoding: Some("base64"),
                },
            ),
        };
        let response = self
            .jito_post(endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| JitoError::Network {
                reason: e.to_string(),
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| JitoError::Network {
            reason: format!("failed to read response: {e}"),
        })?;

        if !status.is_success() {
            return Err(JitoError::Network {
                reason: format!("HTTP {status}: {response_text}"),
            });
        }

        let parsed: JsonRpcResponse<String> =
            serde_json::from_str(&response_text).map_err(|e| JitoError::Serialization {
                reason: format!("failed to parse response: {e}, body: {response_text}"),
            })?;

        if let Some(error) = parsed.error {
            return Err(JitoError::JsonRpc {
                code: error.code,
                message: error.message,
            });
        }

        parsed.result.ok_or_else(|| JitoError::JsonRpc {
            code: -1,
            message: "no bundle_id in response".to_string(),
        })
    }
}
