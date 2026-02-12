use crate::client::jito_bundler::JitoBundler;
use crate::error::JitoError;
use crate::types::{JsonRpcRequest, JsonRpcResponse};
use base64::Engine;
use reqwest::{RequestBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use solana_sdk::transaction::VersionedTransaction;

impl JitoBundler {
    // --- Shared RPC Utilities ---
    /// Sends a JSON-RPC POST request and returns status + raw response body.
    pub async fn send_json_rpc_request<Req: Serialize>(
        &self,
        request_builder: RequestBuilder,
        request: &JsonRpcRequest<Req>,
        context: &str,
    ) -> Result<(StatusCode, String), JitoError> {
        let response =
            request_builder
                .json(request)
                .send()
                .await
                .map_err(|e| JitoError::Network {
                    reason: format!("{context}: {e}"),
                })?;
        let status = response.status();
        let response_text = response.text().await.map_err(|e| JitoError::Network {
            reason: format!("{context} response read failed: {e}"),
        })?;
        Ok((status, response_text))
    }

    /// Parses a JSON-RPC response and extracts `result` or mapped error.
    pub fn parse_json_rpc_result<Res: DeserializeOwned>(
        response_text: &str,
        context: &str,
        missing_result_message: &str,
    ) -> Result<Res, JitoError> {
        let parsed: JsonRpcResponse<Res> =
            serde_json::from_str(response_text).map_err(|e| JitoError::Serialization {
                reason: format!("{context} parse failed: {e}, body: {response_text}"),
            })?;

        if let Some(error) = parsed.error {
            return Err(JitoError::JsonRpc {
                code: error.code,
                message: error.message,
            });
        }

        parsed.result.ok_or_else(|| JitoError::JsonRpc {
            code: -1,
            message: missing_result_message.to_string(),
        })
    }

    /// Serializes and base64-encodes all versioned transactions.
    pub fn encode_transactions_base64(
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

    /// Returns the first transaction signature as base58, or a typed error when absent.
    pub fn first_signature_base58(
        tx: &VersionedTransaction,
        tx_index: usize,
        context: &str,
    ) -> Result<String, JitoError> {
        let signature = tx
            .signatures
            .first()
            .ok_or_else(|| JitoError::InvalidSignature {
                reason: format!("{context}: transaction {tx_index} has no signatures"),
            })?;
        Ok(bs58::encode(signature).into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::message::{Message, VersionedMessage};
    use solana_sdk::signature::Signature;

    /// Creates a minimal versioned transaction with caller-provided signatures.
    fn make_tx(signatures: Vec<Signature>) -> VersionedTransaction {
        let message = Message::new(&[], None);
        VersionedTransaction {
            signatures,
            message: VersionedMessage::Legacy(message),
        }
    }

    #[test]
    fn first_signature_base58_returns_error_when_missing() {
        let tx = make_tx(vec![]);
        let result = JitoBundler::first_signature_base58(&tx, 2, "send_bundle");
        assert!(
            matches!(result, Err(JitoError::InvalidSignature { .. })),
            "expected InvalidSignature, got {result:?}"
        );
    }

    #[test]
    fn first_signature_base58_returns_encoded_signature() {
        let signature = Signature::default();
        let tx = make_tx(vec![signature]);
        let result = JitoBundler::first_signature_base58(&tx, 0, "simulate");
        assert!(result.is_ok(), "expected Ok signature, got {result:?}");
        let encoded = result.unwrap_or_default();
        assert_eq!(encoded, bs58::encode(signature).into_string());
    }
}
