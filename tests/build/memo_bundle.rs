use crate::common;
use jito_bundle::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use jito_bundle::constants::{DEFAULT_COMPUTE_UNIT_LIMIT, SOLANA_MAX_TX_SIZE};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL"]
async fn build_memo_bundle_succeeds() {
    let Some(env) = common::load_test_env() else {
        return;
    };

    let rpc = RpcClient::new(env.rpc_url);
    let blockhash = rpc.get_latest_blockhash().await.ok();
    let Some(blockhash) = blockhash else {
        return;
    };

    let payer_pubkey = env.keypair.pubkey();
    let slots = common::build_memo_slots(
        &payer_pubkey,
        &["jito-bundle test memo 1", "jito-bundle test memo 2"],
    );

    let inputs = BundleBuilderInputs {
        payer: &env.keypair,
        transactions_instructions: slots,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    };
    let result = BundleBuilder::build(inputs);
    assert!(result.is_ok(), "bundle build failed");
    let bundle = match result {
        Ok(b) => b,
        Err(_) => return,
    };
    common::print_bundle_info("build_memo_bundle", &bundle);
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
