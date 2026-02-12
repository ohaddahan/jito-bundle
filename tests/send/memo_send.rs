use crate::common;
use jito_bundle::client::jito_bundler::{BuildBundleOptions, JitoBundler};
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL; sends real bundle to mainnet"]
async fn send_memo_bundle_succeeds() {
    let env =
        common::load_test_env().expect("missing required .env values: KEYPAIR_PATH and RPC_URL");
    let config = common::build_jito_config(&env);
    let bundler = JitoBundler::new(config).expect("failed to create JitoBundler");
    let payer_pubkey = env.keypair.pubkey();
    let slots = common::build_memo_slots(
        &payer_pubkey,
        &["jito-bundle send test 1", "jito-bundle send test 2"],
    );
    let bundle = match bundler
        .build_bundle(BuildBundleOptions {
            payer: &env.keypair,
            transactions_instructions: slots,
            lookup_tables: &[],
        })
        .await
    {
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
    assert!(
        !result.bundle_id.is_empty(),
        "bundle_id should not be empty on success"
    );
}
