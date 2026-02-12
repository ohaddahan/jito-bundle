use crate::client::jito_bundler::JitoBundler;
use crate::client::types::StatusResult;
use crate::constants::DEFAULT_INITIAL_CONFIRM_DELAY_SECS;
use crate::error::JitoError;
use crate::types::{BundleStatus, JsonRpcRequest};
use solana_sdk::signature::Signature;
use solana_transaction_status_client_types::TransactionConfirmationStatus;
use std::str::FromStr;
use std::time::Duration;

impl JitoBundler {
    pub async fn get_bundle_status(&self, bundle_id: &str) -> BundleStatus {
        let endpoint = self.config.network.block_engine_url();
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "getBundleStatuses",
            params: [[bundle_id]],
        };
        let (status_code, response_text) = match self
            .send_json_rpc_request(self.jito_post(endpoint), &request, "getBundleStatuses")
            .await
        {
            Ok(result) => result,
            Err(_) => return BundleStatus::Unknown,
        };
        if status_code.as_u16() == 429 {
            return BundleStatus::Pending;
        }
        if !status_code.is_success() {
            return BundleStatus::Unknown;
        }
        let result: StatusResult = match Self::parse_json_rpc_result(
            &response_text,
            "getBundleStatuses",
            "no result in getBundleStatuses response",
        ) {
            Ok(r) => r,
            Err(_) => return BundleStatus::Unknown,
        };
        if let Some(status) = result.value.first() {
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
        &self,
        signatures: &[String],
    ) -> Result<BundleStatus, JitoError> {
        let parsed_signatures: Vec<Signature> = signatures
            .iter()
            .map(|s| Signature::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JitoError::InvalidSignature {
                reason: e.to_string(),
            })?;
        tokio::time::sleep(Duration::from_secs(DEFAULT_INITIAL_CONFIRM_DELAY_SECS)).await;
        let max_attempts = self.config.confirm_policy.max_attempts;
        let interval_ms = self.config.confirm_policy.interval_ms;
        for _attempt in 0..max_attempts {
            if let Ok(statuses) = self
                .rpc_client
                .get_signature_statuses(&parsed_signatures)
                .await
            {
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
        &self,
        bundle_id: &str,
    ) -> Result<BundleStatus, JitoError> {
        tokio::time::sleep(Duration::from_secs(DEFAULT_INITIAL_CONFIRM_DELAY_SECS)).await;
        let max_attempts = self.config.confirm_policy.max_attempts;
        let interval_ms = self.config.confirm_policy.interval_ms;
        for _attempt in 0..max_attempts {
            let status = self.get_bundle_status(bundle_id).await;
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
