use crate::common;
use jito_bundle::client::jito_bundler::{BuildBundleOptions, JitoBundler};
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL; sends real bundle to mainnet"]
async fn send_memo_bundle_succeeds() {
    let Some(env) = common::load_test_env() else {
        return;
    };

    let config = common::build_jito_config(&env);
    let bundler = match JitoBundler::new(config) {
        Ok(b) => b,
        Err(e) => {
            println!("failed to create bundler: {e}");
            return;
        }
    };

    let tip_lamports = match bundler.fetch_tip().await {
        Ok(tip) => {
            println!("fetched tip floor: {tip} lamports");
            tip.max(env.tip_lamports)
        }
        Err(e) => {
            println!(
                "fetch_tip failed ({e}), falling back to env tip: {}",
                env.tip_lamports
            );
            env.tip_lamports
        }
    };

    let blockhash = match bundler.rpc_client.get_latest_blockhash().await {
        Ok(bh) => bh,
        Err(e) => {
            println!("failed to get blockhash: {e}");
            return;
        }
    };

    let payer_pubkey = env.keypair.pubkey();
    let slots = common::build_memo_slots(
        &payer_pubkey,
        &["jito-bundle send test 1", "jito-bundle send test 2"],
    );

    let bundle = match bundler.build_bundle(BuildBundleOptions {
        payer: &env.keypair,
        transactions_instructions: slots,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports,
    }) {
        Ok(b) => b,
        Err(e) => {
            println!("bundle build failed: {e}");
            panic!("bundle build failed: {e}");
        }
    };

    common::print_bundle_info("send_memo_bundle", &bundle);

    let result = match bundler.send_and_confirm(&bundle).await {
        Ok(r) => r,
        Err(e) => {
            println!("send_and_confirm failed: {e}");
            panic!("send_and_confirm failed: {e}");
        }
    };

    common::print_bundle_result("send_memo_bundle", &result);
    assert!(result.success, "bundle was not successful");
}
