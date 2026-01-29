use solana_pubkey::{Pubkey, pubkey};

pub const JITO_BLOCK_ENGINE_URL: &str = "https://mainnet.block-engine.jito.wtf/api/v1/bundles";

pub const JITO_MAINNET_ENDPOINTS: [&str; 5] = ["mainnet", "amsterdam", "frankfurt", "ny", "tokyo"];

pub const JITO_TIPS_FLOOR_URL: &str = "https://bundles.jito.wtf/api/v1/bundles/tip_floor";

pub const JITO_EXPLORER_URL: &str = "https://explorer.jito.wtf/events";

pub const SOLANA_EXPLORER_URL: &str = "https://explorer.solana.com/tx";

pub const JITO_TIP_ACCOUNTS: [Pubkey; 8] = [
    pubkey!("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"),
    pubkey!("HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe"),
    pubkey!("Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY"),
    pubkey!("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49"),
    pubkey!("DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh"),
    pubkey!("ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt"),
    pubkey!("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL"),
    pubkey!("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"),
];

pub const DEFAULT_TIP_LAMPORTS: u64 = 100_000;
pub const MAX_TIP_LAMPORTS: u64 = 10_000_000;

pub const MAX_BUNDLE_TRANSACTIONS: usize = 5;

pub const SOLANA_MAX_TX_SIZE: usize = 1232;

pub const DEFAULT_COMPUTE_UNIT_LIMIT: u32 = 3_000_000;

pub const DEFAULT_MAX_CONFIRM_ATTEMPTS: u32 = 30;
pub const DEFAULT_CONFIRM_INTERVAL_MS: u64 = 2_000;
pub const DEFAULT_INITIAL_CONFIRM_DELAY_SECS: u64 = 5;

pub const SYSTEM_PROGRAM_ID: Pubkey = pubkey!("11111111111111111111111111111111");
