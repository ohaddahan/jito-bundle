use crate::constants::{JITO_BLOCK_ENGINE_URL, JITO_MAINNET_ENDPOINTS, JITO_TIPS_FLOOR_URL};

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

    pub fn is_custom(&self) -> bool {
        matches!(self, Network::Custom { .. })
    }

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
