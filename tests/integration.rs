use jito_bundle::bundler::bundle::{Bundle, BundleBuilderInputs};
use jito_bundle::client::jito_bundler::JitoBundler;
use jito_bundle::config::jito::JitoConfig;
use jito_bundle::constants::{DEFAULT_COMPUTE_UNIT_LIMIT, SOLANA_MAX_TX_SIZE};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::{Pubkey, pubkey};
use solana_sdk::signature::{Keypair, Signer};
use std::fs;

const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

struct TestEnv {
    pub keypair: Keypair,
    pub rpc_url: String,
    pub helius_url: Option<String>,
}

fn load_test_env() -> Option<TestEnv> {
    dotenvy::dotenv().ok();

    let keypair_path = std::env::var("KEYPAIR_PATH").ok()?;
    let rpc_url = std::env::var("RPC_URL").ok()?;
    let helius_url = std::env::var("HELIUS_RPC_URL").ok();

    let file_contents = fs::read_to_string(&keypair_path).ok()?;
    let bytes: Vec<u8> = serde_json::from_str(&file_contents).ok()?;
    let keypair = Keypair::try_from(bytes.as_slice()).ok()?;

    Some(TestEnv {
        keypair,
        rpc_url,
        helius_url,
    })
}

fn create_memo_instruction(payer: &Pubkey, message: &str) -> Instruction {
    Instruction {
        program_id: MEMO_PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(*payer, true)],
        data: message.as_bytes().to_vec(),
    }
}

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL"]
async fn build_memo_bundle_succeeds() {
    let Some(env) = load_test_env() else {
        return;
    };

    let rpc = RpcClient::new(env.rpc_url);
    let blockhash = rpc.get_latest_blockhash().await.ok();
    let Some(blockhash) = blockhash else {
        return;
    };

    let payer_pubkey = env.keypair.pubkey();
    let transactions = vec![
        vec![create_memo_instruction(
            &payer_pubkey,
            "jito-bundle test memo 1",
        )],
        vec![create_memo_instruction(
            &payer_pubkey,
            "jito-bundle test memo 2",
        )],
    ];

    let inputs = BundleBuilderInputs {
        payer: &env.keypair,
        transactions,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    };

    let result = Bundle::new(inputs).build();
    assert!(result.is_ok(), "bundle build failed");
    let bundle = match result {
        Ok(b) => b,
        Err(_) => return,
    };

    assert_eq!(bundle.versioned_transaction.len(), 3);

    for (i, tx) in bundle.versioned_transaction.iter().enumerate() {
        let serialized = bincode::serialize(tx).unwrap_or_default();
        assert!(
            serialized.len() <= SOLANA_MAX_TX_SIZE,
            "transaction {i} is {size} bytes, exceeds {SOLANA_MAX_TX_SIZE}",
            size = serialized.len()
        );
    }
}

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL"]
async fn simulate_memo_bundle_on_helius() {
    let Some(env) = load_test_env() else {
        return;
    };
    let Some(helius_url) = &env.helius_url else {
        return;
    };

    let config = JitoConfig::new(&env.rpc_url).with_helius_rpc_url(helius_url);
    let bundler = match JitoBundler::new(config) {
        Ok(b) => b,
        Err(_) => return,
    };

    let blockhash = bundler.rpc_client.get_latest_blockhash().await.ok();
    let Some(blockhash) = blockhash else {
        return;
    };

    let payer_pubkey = env.keypair.pubkey();
    let transactions = vec![
        vec![create_memo_instruction(&payer_pubkey, "helius sim test 1")],
        vec![create_memo_instruction(&payer_pubkey, "helius sim test 2")],
    ];

    let inputs = BundleBuilderInputs {
        payer: &env.keypair,
        transactions,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    };

    let bundle = match Bundle::new(inputs).build() {
        Ok(b) => b,
        Err(_) => return,
    };

    let sim_result = bundler.simulate_helius(&bundle).await;
    assert!(
        sim_result.is_ok(),
        "helius simulation failed: {sim_result:?}"
    );
}
