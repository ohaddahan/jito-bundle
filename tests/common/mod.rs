use jito_bundle::bundler::bundle::types::BuiltBundle;
use jito_bundle::bundler::types::{BundleInstructionSlots, BundleSlotView, TipMode};
use jito_bundle::config::jito::JitoConfig;
use jito_bundle::config::network::Network;
use jito_bundle::constants::DEFAULT_TIP_LAMPORTS;
use jito_bundle::types::BundleResult;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::{Pubkey, pubkey};
use solana_sdk::signature::Keypair;
use std::fs;
use std::str::FromStr;

const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
const BUNDLES_PATH: &str = "/api/v1/bundles";

pub struct TestEnv {
    pub keypair: Keypair,
    pub rpc_url: String,
    pub helius_url: Option<String>,
    pub jito_block_engine_url: Option<String>,
    pub jito_uuid: Option<String>,
    pub jitodontfront_pubkey: Option<Pubkey>,
    pub tip_lamports: u64,
}

pub fn load_test_env() -> Option<TestEnv> {
    dotenvy::dotenv().ok();

    let keypair_path = std::env::var("KEYPAIR_PATH").ok()?;
    let rpc_url = std::env::var("RPC_URL").ok()?;
    let helius_url = std::env::var("HELIUS_RPC_URL").ok();
    let jito_block_engine_url = std::env::var("JITO_BLOCK_ENGINE_URL").ok();
    let jito_uuid = std::env::var("JITO_UUID").ok();
    let jitodontfront_pubkey = std::env::var("JITODONTFRONT_PUBKEY")
        .ok()
        .and_then(|s| Pubkey::from_str(&s).ok());
    let tip_lamports = std::env::var("TIP_LAMPORTS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TIP_LAMPORTS);

    let file_contents = fs::read_to_string(&keypair_path).ok()?;
    let bytes: Vec<u8> = serde_json::from_str(&file_contents).ok()?;
    let keypair = Keypair::try_from(bytes.as_slice()).ok()?;

    Some(TestEnv {
        keypair,
        rpc_url,
        helius_url,
        jito_block_engine_url,
        jito_uuid,
        jitodontfront_pubkey,
        tip_lamports,
    })
}

fn build_custom_urls(base_url: &str) -> (String, String) {
    let trimmed = base_url.trim_end_matches('/');
    let block_engine_url = if trimmed.ends_with(BUNDLES_PATH) {
        trimmed.to_string()
    } else {
        format!("{trimmed}{BUNDLES_PATH}")
    };
    let tip_floor_url = format!("{trimmed}/tip_floor");
    (block_engine_url, tip_floor_url)
}

pub fn build_jito_config(env: &TestEnv) -> JitoConfig {
    let network = match &env.jito_block_engine_url {
        Some(base_url) => {
            let (block_engine_url, tip_floor_url) = build_custom_urls(base_url);
            println!("  block_engine_url: {block_engine_url}");
            println!("  tip_floor_url:    {tip_floor_url}");
            Network::Custom {
                block_engine_url,
                tip_floor_url,
            }
        }
        None => Network::Mainnet,
    };

    let mut config = JitoConfig::new(&env.rpc_url).with_network(network);

    if let Some(uuid) = &env.jito_uuid {
        config = config.with_uuid(uuid);
    }
    if let Some(helius_url) = &env.helius_url {
        config = config.with_helius_rpc_url(helius_url);
    }
    if let Some(jdf) = env.jitodontfront_pubkey {
        config = config.with_jitodontfront(jdf);
    }

    config
}

pub fn print_bundle_result(test_name: &str, result: &BundleResult) {
    let bar = "━".repeat(56);
    println!("\n{bar}");
    println!("  {test_name} — result");
    println!("{bar}");

    println!("  bundle_id: {}", result.bundle_id);
    println!("  explorer:  {}", result.explorer_url);

    let sigs = result.signatures.join(" | ");
    println!("  signatures: {sigs}");
    println!("{bar}\n");
}

pub fn create_memo_instruction(payer: &Pubkey, message: &str) -> Instruction {
    Instruction {
        program_id: MEMO_PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(*payer, true)],
        data: message.as_bytes().to_vec(),
    }
}

pub fn build_memo_slots(payer: &Pubkey, messages: &[&str]) -> BundleInstructionSlots {
    let mut slots: BundleInstructionSlots = [None, None, None, None, None];
    for (i, msg) in messages.iter().enumerate().take(5) {
        slots[i] = Some(vec![create_memo_instruction(payer, msg)]);
    }
    slots
}

fn short_pubkey(pk: &Pubkey) -> String {
    let s = pk.to_string();
    if s.len() > 8 {
        format!("{}..{}", &s[..4], &s[s.len() - 4..])
    } else {
        s
    }
}

pub fn print_bundle_info(test_name: &str, bundle: &BuiltBundle) {
    let bar = "━".repeat(56);
    println!("\n{bar}");
    println!("  {test_name}");
    println!("{bar}");

    let tx_count = bundle.transactions.len();
    let populated = bundle.populated_count();

    let bundle_id = bundle.transactions.first().map_or_else(
        || "n/a".to_string(),
        |tx| bs58::encode(&tx.signatures[0]).into_string(),
    );

    println!("  bundle_id: {bundle_id}");
    println!("  transactions: {tx_count} versioned · {populated} instruction slots");
    println!("  tip_account: {}", short_pubkey(&bundle.tip_account));
    println!(
        "  tip_mode: {}",
        if matches!(bundle.tip_mode, TipMode::SeparateTx) {
            "separate"
        } else {
            "inline"
        }
    );

    for (i, tx) in bundle.transactions.iter().enumerate() {
        let size = bincode::serialize(tx).map_or(0, |s| s.len());
        let sig = bs58::encode(&tx.signatures[0]).into_string();
        println!("  tx[{i}]: {sig} ({size} bytes)");
    }

    println!("  tip_lamports: {}", bundle.tip_lamports);
    println!("{bar}");
    println!("  {test_name}: BUILT (not yet sent)\n");
}
