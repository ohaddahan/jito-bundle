use crate::bundle::{BuildBundleInput, BundleBuilder};
use crate::config::JitoConfig;
use crate::error::JitoError;
use crate::send::SendHelper;
use crate::simulate::SimulateHelper;
use crate::status::StatusHelper;
use crate::tip::TipHelper;
use crate::types::{BundleResult, BundleStatus, SimulateBundleValue};
use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::VersionedTransaction;
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

    pub fn build_bundle(
        &self,
        input: BuildBundleOptions<'_>,
    ) -> Result<Vec<VersionedTransaction>, JitoError> {
        let BuildBundleOptions {
            payer,
            instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
        } = input;

        BundleBuilder::build(BuildBundleInput {
            payer,
            instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey: self.config.jitodontfront_pubkey.as_ref(),
            compute_unit_limit: self.config.compute_unit_limit,
        })
    }

    pub async fn simulate(&self, transactions: &[VersionedTransaction]) -> Result<(), JitoError> {
        SimulateHelper::simulate_per_transaction(transactions, &self.rpc_client).await
    }

    pub async fn simulate_helius(
        &self,
        transactions: &[VersionedTransaction],
    ) -> Result<SimulateBundleValue, JitoError> {
        let helius_url =
            self.config
                .helius_rpc_url
                .as_deref()
                .ok_or_else(|| JitoError::Network {
                    reason: "helius_rpc_url not configured".to_string(),
                })?;

        SimulateHelper::simulate_bundle_helius(&self.http_client, transactions, helius_url).await
    }

    pub async fn send(
        &self,
        transactions: &[VersionedTransaction],
    ) -> Result<BundleResult, JitoError> {
        let base_url = self.config.network.block_engine_url();
        SendHelper::send_bundle(&self.http_client, transactions, base_url).await
    }

    pub async fn send_and_confirm(
        &self,
        input: SendAndConfirmInput<'_>,
    ) -> Result<BundleResult, JitoError> {
        let SendAndConfirmInput {
            payer,
            instructions,
            lookup_tables,
            recent_blockhash,
        } = input;

        let tip_lamports = self.fetch_tip().await?;

        let transactions = self.build_bundle(BuildBundleOptions {
            payer,
            instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
        })?;

        if let Some(helius_url) = &self.config.helius_rpc_url
            && let Err(e) =
                SimulateHelper::simulate_bundle_helius(&self.http_client, &transactions, helius_url)
                    .await
        {
            tracing::warn!("Helius simulation failed: {e}");
            return Err(e);
        }

        let result = self.send(&transactions).await?;

        tracing::info!(
            "bundle submitted: bundle_id={:?}, signatures={:?}, explorer={:?}",
            result.bundle_id,
            result.signatures,
            result.explorer_url
        );

        let status = StatusHelper::wait_for_landing_on_chain(
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
    pub instructions: Vec<Instruction>,
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
}

pub struct SendAndConfirmInput<'a> {
    pub payer: &'a Keypair,
    pub instructions: Vec<Instruction>,
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
}
