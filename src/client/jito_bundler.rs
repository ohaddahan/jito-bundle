use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use crate::bundler::bundle::types::BuiltBundle;
use crate::bundler::types::BundleInstructionSlots;
use crate::config::jito::JitoConfig;
use crate::error::JitoError;
use crate::tip::TipHelper;
use crate::types::{BundleResult, BundleStatus, SimulateBundleValue};
use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::signer::keypair::Keypair;
use std::time::Duration;

/// High-level facade for building, simulating, and sending Jito bundles.
pub struct JitoBundler {
    /// Runtime bundler configuration.
    pub config: JitoConfig,
    /// Shared HTTP client for Jito/Helius requests.
    pub http_client: Client,
    /// Solana RPC client for chain reads and simulations.
    pub rpc_client: RpcClient,
}

impl JitoBundler {
    // --- Construction ---
    /// Creates a bundler client from configuration.
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

    // --- HTTP Helpers ---
    /// Builds a JSON POST request with Jito auth headers when configured.
    pub fn jito_post(&self, url: &str) -> reqwest::RequestBuilder {
        let full_url = if let Some(uuid) = &self.config.uuid
            && !self.config.network.is_custom()
        {
            let separator = if url.contains('?') { "&" } else { "?" };
            format!("{url}{separator}uuid={uuid}")
        } else {
            url.to_string()
        };
        let mut builder = self
            .http_client
            .post(full_url)
            .header("Content-Type", "application/json");
        if let Some(uuid) = &self.config.uuid {
            builder = builder.header("x-jito-auth", uuid.as_str());
        }
        builder
    }

    // --- Bundle Lifecycle ---
    /// Resolves the tip amount according to configured strategy.
    pub async fn fetch_tip(&self) -> Result<u64, JitoError> {
        let tip_floor_url = self.config.network.tip_floor_url();
        TipHelper::resolve_tip(&self.http_client, tip_floor_url, &self.config.tip_strategy).await
    }

    /// Builds a signed `BuiltBundle` from fixed instruction slots.
    ///
    /// Steps:
    /// 1. Resolve the tip amount using configured `TipStrategy`.
    /// 2. Fetch a fresh recent blockhash from RPC.
    /// 3. Build and sign transactions via `BundleBuilder::build`.
    ///
    /// Returns `JitoError` when tip resolution, blockhash fetch, or compilation fails.
    pub async fn build_bundle(
        &self,
        input: BuildBundleOptions<'_>,
    ) -> Result<BuiltBundle, JitoError> {
        let BuildBundleOptions {
            payer,
            transactions_instructions,
            lookup_tables,
        } = input;
        let tip_lamports = self.fetch_tip().await?;
        let recent_blockhash = self.rpc_client.get_latest_blockhash().await.map_err(|e| {
            JitoError::GetLatestBlockhash {
                reason: e.to_string(),
            }
        })?;
        let bundle = BundleBuilder::build(BundleBuilderInputs {
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey: self.config.jitodontfront_pubkey.as_ref(),
            compute_unit_limit: self.config.compute_unit_limit,
        })?;
        Ok(bundle)
    }

    /// Simulates the built bundle against configured Helius RPC.
    pub async fn simulate_helius(
        &self,
        bundle: &BuiltBundle,
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

    /// Optionally simulates, sends, and confirms the bundle on-chain.
    ///
    /// Behavior:
    /// 1. If `helius_rpc_url` is configured, run atomic simulation first.
    /// 2. Submit bundle with endpoint retry.
    /// 3. Poll on-chain signatures until landed/failed/timeout.
    pub async fn send_and_confirm(&self, bundle: &BuiltBundle) -> Result<BundleResult, JitoError> {
        if let Some(helius_url) = &self.config.helius_rpc_url
            && let Err(e) = self.simulate_bundle_helius(bundle, helius_url).await
        {
            tracing::warn!("Helius simulation failed: {e}");
            return Err(e);
        }
        let result = self.send_bundle(bundle).await?;
        tracing::info!(
            "bundle submitted: bundle_id={}, signatures={:?}, explorer={}",
            result.bundle_id,
            result.signatures,
            result.explorer_url
        );
        let status = self.wait_for_landing_on_chain(&result.signatures).await;
        Self::interpret_landing_status(status, self.config.confirm_policy.max_attempts)?;
        Ok(result)
    }

    /// Maps raw landing status into final client-facing success/error.
    fn interpret_landing_status(
        status: Result<BundleStatus, JitoError>,
        max_attempts: u32,
    ) -> Result<(), JitoError> {
        match status {
            Ok(BundleStatus::Landed { .. }) => Ok(()),
            Ok(BundleStatus::Failed { error }) => {
                let reason = error.unwrap_or_else(|| "unknown error".to_string());
                Err(JitoError::OnChainFailure { reason })
            }
            Ok(_) => Err(JitoError::ConfirmationTimeout {
                attempts: max_attempts,
            }),
            Err(e) => Err(e),
        }
    }
}

/// Input arguments for `JitoBundler::build_bundle`.
pub struct BuildBundleOptions<'a> {
    /// Signing payer used for all transactions.
    pub payer: &'a Keypair,
    /// Fixed instruction slots (max 5) used for bundle creation.
    pub transactions_instructions: BundleInstructionSlots,
    /// Lookup tables used when compiling transactions.
    pub lookup_tables: &'a [AddressLookupTableAccount],
}
