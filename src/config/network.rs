use crate::constants::{JITO_BLOCK_ENGINE_URL, JITO_MAINNET_ENDPOINTS, JITO_TIPS_FLOOR_URL};

/// Supported block-engine network configurations.
#[derive(Debug, Clone)]
pub enum Network {
    /// Use default Jito mainnet endpoints.
    Mainnet,
    /// Use custom block-engine and tip-floor URLs.
    Custom {
        /// Custom block-engine base URL.
        block_engine_url: String,
        /// Custom tip-floor API URL.
        tip_floor_url: String,
    },
}

impl Network {
    // --- Endpoint Accessors ---
    /// Returns the block-engine URL for status/send JSON-RPC calls.
    pub fn block_engine_url(&self) -> &str {
        match self {
            Network::Mainnet => JITO_BLOCK_ENGINE_URL,
            Network::Custom {
                block_engine_url, ..
            } => block_engine_url,
        }
    }

    /// Returns the tip-floor API URL for tip resolution.
    pub fn tip_floor_url(&self) -> &str {
        match self {
            Network::Mainnet => JITO_TIPS_FLOOR_URL,
            Network::Custom { tip_floor_url, .. } => tip_floor_url,
        }
    }

    /// Returns true when using custom network endpoints.
    pub fn is_custom(&self) -> bool {
        matches!(self, Network::Custom { .. })
    }

    /// Returns ordered send endpoints used for retry strategy.
    pub fn send_endpoints(&self) -> Vec<String> {
        match self {
            Network::Mainnet => JITO_MAINNET_ENDPOINTS
                .iter()
                .map(|ep| format!("{JITO_BLOCK_ENGINE_URL}?endpoint={ep}"))
                .collect(),
            Network::Custom {
                block_engine_url, ..
            } => vec![block_engine_url.clone()],
        }
    }
}
