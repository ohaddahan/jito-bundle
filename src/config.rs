use crate::constants::{
    DEFAULT_COMPUTE_UNIT_LIMIT, DEFAULT_CONFIRM_INTERVAL_MS, DEFAULT_MAX_CONFIRM_ATTEMPTS,
    DEFAULT_TIP_LAMPORTS, JITO_BLOCK_ENGINE_URL, JITO_TIPS_FLOOR_URL, MAX_TIP_LAMPORTS,
};
use solana_pubkey::Pubkey;

#[derive(Debug, Clone)]
pub enum Network {
    Mainnet,
    Custom {
        block_engine_url: String,
        tip_floor_url: String,
    },
}

impl Network {
    pub fn block_engine_url(&self) -> &str {
        match self {
            Network::Mainnet => JITO_BLOCK_ENGINE_URL,
            Network::Custom {
                block_engine_url, ..
            } => block_engine_url,
        }
    }

    pub fn tip_floor_url(&self) -> &str {
        match self {
            Network::Mainnet => JITO_TIPS_FLOOR_URL,
            Network::Custom { tip_floor_url, .. } => tip_floor_url,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TipStrategy {
    Fixed(u64),
    FetchFloor,
    FetchFloorWithCap { min: u64, max: u64 },
}

impl Default for TipStrategy {
    fn default() -> Self {
        TipStrategy::FetchFloorWithCap {
            min: DEFAULT_TIP_LAMPORTS,
            max: MAX_TIP_LAMPORTS,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfirmPolicy {
    pub max_attempts: u32,
    pub interval_ms: u64,
}

impl Default for ConfirmPolicy {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_CONFIRM_ATTEMPTS,
            interval_ms: DEFAULT_CONFIRM_INTERVAL_MS,
        }
    }
}

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
