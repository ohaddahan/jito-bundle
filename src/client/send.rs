use crate::bundler::bundle::types::BuiltBundle;
use crate::client::jito_bundler::JitoBundler;
use crate::constants::JITO_EXPLORER_URL;
use crate::error::JitoError;
use crate::types::{BundleResult, JsonRpcRequest};
use serde::Serialize;
use solana_sdk::transaction::VersionedTransaction;

impl JitoBundler {
    // --- Bundle Sending ---
    /// Builds the Jito explorer URL for a bundle id.
    pub fn get_jito_explorer_url(bundle_id: &str) -> String {
        format!("{JITO_EXPLORER_URL}/{bundle_id}")
    }

    /// Extracts base58 signatures from compiled transactions.
    pub fn extract_signatures(
        transactions: &[VersionedTransaction],
    ) -> Result<Vec<String>, JitoError> {
        transactions
            .iter()
            .enumerate()
            .map(|(i, tx)| Self::first_signature_base58(tx, i, "send_bundle"))
            .collect()
    }

    /// Sends a built bundle with endpoint retry fallback.
    pub async fn send_bundle(&self, bundle: &BuiltBundle) -> Result<BundleResult, JitoError> {
        let encoded_txs = Self::encode_transactions_base64(&bundle.transactions)?;
        let signatures = Self::extract_signatures(&bundle.transactions)?;
        let endpoints = self.config.network.send_endpoints();
        let mut last_error = String::from("no endpoints available");
        for endpoint in &endpoints {
            match self.send_bundle_to_endpoint(endpoint, &encoded_txs).await {
                Ok(bundle_id) => {
                    let explorer_url = Self::get_jito_explorer_url(&bundle_id);
                    return Ok(BundleResult {
                        bundle_id,
                        signatures,
                        explorer_url,
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

    /// Sends a bundle request to a single block-engine endpoint.
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
        let (status, response_text) = self
            .send_json_rpc_request(self.jito_post(endpoint), &request, "sendBundle")
            .await?;
        if !status.is_success() {
            return Err(JitoError::Network {
                reason: format!("sendBundle HTTP {status}: {response_text}"),
            });
        }
        Self::parse_json_rpc_result(&response_text, "sendBundle", "no bundle_id in response")
    }
}
