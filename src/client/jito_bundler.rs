use crate::bundler::bundle::{Bundle, BundleBuilderInputs};
use crate::config::jito::JitoConfig;
use crate::error::JitoError;
use crate::tip::TipHelper;
use crate::types::{BundleResult, BundleStatus, SimulateBundleValue};
use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::signer::keypair::Keypair;
use std::time::Duration;

pub struct JitoBundler {
    pub config: JitoConfig,
    pub http_client: Client,
    pub rpc_client: RpcClient,
}

impl JitoBundler {
    pub fn new(config: JitoConfig) -> Result<Self, JitoError> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| JitoError::Network {
                reason: format!("failed to create HTTP client: {e}"),
            })?;
        let rpc_client = RpcClient::new(config.rpc_url.clone());
        Ok(Self {
            config,
            http_client,
            rpc_client,
        })
    }

    pub async fn fetch_tip(&self) -> Result<u64, JitoError> {
        let tip_floor_url = self.config.network.tip_floor_url();
        TipHelper::resolve_tip(&self.http_client, tip_floor_url, &self.config.tip_strategy).await
    }

    pub fn build_bundle<'a>(
        &'a self,
        input: BuildBundleOptions<'a>,
    ) -> Result<Bundle<'a>, JitoError> {
        let BuildBundleOptions {
            payer,
            transactions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
        } = input;
        let bundle = Bundle::new(BundleBuilderInputs {
            payer,
            transactions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey: self.config.jitodontfront_pubkey.as_ref(),
            compute_unit_limit: self.config.compute_unit_limit,
        });
        bundle.build()
    }

    pub async fn simulate_helius<'a>(
        &self,
        bundle: &'a Bundle<'a>,
    ) -> Result<SimulateBundleValue, JitoError> {
        let helius_url =
            self.config
                .helius_rpc_url
                .as_deref()
                .ok_or_else(|| JitoError::Network {
                    reason: "helius_rpc_url not configured".to_string(),
                })?;
        self.simulate_bundle_helius(bundle, helius_url).await
    }

    pub async fn send_and_confirm(&self, bundle: &Bundle<'_>) -> Result<BundleResult, JitoError> {
        if let Some(helius_url) = &self.config.helius_rpc_url
            && let Err(e) = self.simulate_bundle_helius(bundle, helius_url).await
        {
            tracing::warn!("Helius simulation failed: {e}");
            return Err(e);
        }
        let result = self.send_bundle(bundle).await?;
        tracing::info!(
            "bundle submitted: bundle_id={:?}, signatures={:?}, explorer={:?}",
            result.bundle_id,
            result.signatures,
            result.explorer_url
        );
        let status = JitoBundler::wait_for_landing_on_chain(
            &result.signatures,
            &self.rpc_client,
            self.config.confirm_policy.max_attempts,
            self.config.confirm_policy.interval_ms,
        )
        .await;
        match status {
            Ok(BundleStatus::Landed { .. }) => Ok(result),
            Ok(BundleStatus::Failed { error }) => {
                let reason = error.unwrap_or_else(|| "unknown error".to_string());
                Err(JitoError::OnChainFailure { reason })
            }
            Ok(_) => Err(JitoError::ConfirmationTimeout {
                attempts: self.config.confirm_policy.max_attempts,
            }),
            Err(e) => Err(e),
        }
    }
}

pub struct BuildBundleOptions<'a> {
    pub payer: &'a Keypair,
    pub transactions: Vec<Vec<Instruction>>,
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
}
