use crate::config::confirm_policy::ConfirmPolicy;
use crate::config::network::Network;
use crate::config::tip_strategy::TipStrategy;
use crate::constants::DEFAULT_COMPUTE_UNIT_LIMIT;
use solana_pubkey::Pubkey;

/// User-facing runtime configuration for `JitoBundler`.
#[derive(Debug, Clone)]
pub struct JitoConfig {
    /// Target network and block-engine endpoints.
    pub network: Network,
    /// Solana RPC URL used for chain reads.
    pub rpc_url: String,
    /// Optional Helius RPC URL for atomic simulation.
    pub helius_rpc_url: Option<String>,
    /// Optional Jito UUID for authenticated requests.
    pub uuid: Option<String>,
    /// Strategy used to resolve bundle tip amount.
    pub tip_strategy: TipStrategy,
    /// Confirmation polling behavior after send.
    pub confirm_policy: ConfirmPolicy,
    /// Optional `jitodontfront` pubkey to inject.
    pub jitodontfront_pubkey: Option<Pubkey>,
    /// Compute unit limit prepended to each transaction.
    pub compute_unit_limit: u32,
}

impl JitoConfig {
    // --- Construction ---
    /// Creates a config with sensible defaults for mainnet usage.
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            network: Network::Mainnet,
            rpc_url: rpc_url.into(),
            helius_rpc_url: None,
            uuid: None,
            tip_strategy: TipStrategy::default(),
            confirm_policy: ConfirmPolicy::default(),
            jitodontfront_pubkey: None,
            compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
        }
    }

    // --- Builder Setters ---
    /// Sets target Jito network configuration.
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Sets optional Helius RPC URL for atomic simulation.
    pub fn with_helius_rpc_url(mut self, url: impl Into<String>) -> Self {
        self.helius_rpc_url = Some(url.into());
        self
    }

    /// Sets optional Jito UUID for authenticated requests.
    pub fn with_uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Sets tip strategy used during bundle build.
    pub fn with_tip_strategy(mut self, strategy: TipStrategy) -> Self {
        self.tip_strategy = strategy;
        self
    }

    /// Sets confirmation polling behavior.
    pub fn with_confirm_policy(mut self, policy: ConfirmPolicy) -> Self {
        self.confirm_policy = policy;
        self
    }

    /// Enables `jitodontfront` account injection.
    pub fn with_jitodontfront(mut self, pubkey: Pubkey) -> Self {
        self.jitodontfront_pubkey = Some(pubkey);
        self
    }

    /// Sets compute unit limit prepended to each transaction.
    pub fn with_compute_unit_limit(mut self, limit: u32) -> Self {
        self.compute_unit_limit = limit;
        self
    }
}
