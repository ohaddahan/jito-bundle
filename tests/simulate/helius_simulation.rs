use crate::common;
use jito_bundle::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use jito_bundle::client::jito_bundler::JitoBundler;
use jito_bundle::config::jito::JitoConfig;
use jito_bundle::constants::DEFAULT_COMPUTE_UNIT_LIMIT;
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL"]
async fn simulate_memo_bundle_on_helius() {
    let env =
        common::load_test_env().expect("missing required .env values: KEYPAIR_PATH and RPC_URL");
    let helius_url = env
        .helius_url
        .as_ref()
        .expect("missing HELIUS_RPC_URL for simulation test");
    let config = JitoConfig::new(&env.rpc_url).with_helius_rpc_url(helius_url);
    let bundler = JitoBundler::new(config).expect("failed to create JitoBundler");
    let blockhash = bundler
        .rpc_client
        .get_latest_blockhash()
        .await
        .expect("failed to fetch latest blockhash");
    let payer_pubkey = env.keypair.pubkey();
    let slots =
        common::build_memo_slots(&payer_pubkey, &["helius sim test 1", "helius sim test 2"]);
    let inputs = BundleBuilderInputs {
        payer: &env.keypair,
        transactions_instructions: slots,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    };
    let bundle = BundleBuilder::build(inputs).expect("bundle build failed");
    common::print_bundle_info("simulate_memo_bundle_on_helius", &bundle);
    let sim_result = bundler.simulate_helius(&bundle).await;
    assert!(
        sim_result.is_ok(),
        "helius simulation failed: {sim_result:?}"
    );
}
