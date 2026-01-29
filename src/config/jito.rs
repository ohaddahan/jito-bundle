use crate::config::confirm_policy::ConfirmPolicy;
use crate::config::network::Network;
use crate::config::tip_strategy::TipStrategy;
use crate::constants::DEFAULT_COMPUTE_UNIT_LIMIT;
use solana_pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct JitoConfig {
    pub network: Network,
    pub rpc_url: String,
    pub helius_rpc_url: Option<String>,
    pub uuid: Option<String>,
    pub tip_strategy: TipStrategy,
    pub confirm_policy: ConfirmPolicy,
    pub jitodontfront_pubkey: Option<Pubkey>,
    pub compute_unit_limit: u32,
}

impl JitoConfig {
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

    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    pub fn with_helius_rpc_url(mut self, url: impl Into<String>) -> Self {
        self.helius_rpc_url = Some(url.into());
        self
    }

    pub fn with_uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    pub fn with_tip_strategy(mut self, strategy: TipStrategy) -> Self {
        self.tip_strategy = strategy;
        self
    }

    pub fn with_confirm_policy(mut self, policy: ConfirmPolicy) -> Self {
        self.confirm_policy = policy;
        self
    }

    pub fn with_jitodontfront(mut self, pubkey: Pubkey) -> Self {
        self.jitodontfront_pubkey = Some(pubkey);
        self
    }

    pub fn with_compute_unit_limit(mut self, limit: u32) -> Self {
        self.compute_unit_limit = limit;
        self
    }
}
