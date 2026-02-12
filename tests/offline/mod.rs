use crate::common;
use jito_bundle::config::network::Network;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

#[test]
fn build_memo_slots_caps_at_five() {
    let payer = Keypair::new().pubkey();
    let messages = ["m1", "m2", "m3", "m4", "m5", "m6"];
    let slots = common::build_memo_slots(&payer, &messages);

    let populated_count = slots.iter().filter(|slot| slot.is_some()).count();
    assert_eq!(populated_count, 5, "expected at most 5 populated slots");
}

#[test]
fn build_jito_config_derives_custom_urls() {
    let env = common::TestEnv {
        keypair: Keypair::new(),
        rpc_url: "https://rpc.example.com".to_string(),
        helius_url: None,
        jito_block_engine_url: Some("https://proxy.example.com".to_string()),
        jito_uuid: None,
        jitodontfront_pubkey: None,
        tip_lamports: 100_000,
    };

    let config = common::build_jito_config(&env);
    match config.network {
        Network::Custom {
            block_engine_url,
            tip_floor_url,
        } => {
            assert_eq!(block_engine_url, "https://proxy.example.com/api/v1/bundles");
            assert_eq!(tip_floor_url, "https://proxy.example.com/tip_floor");
        }
        Network::Mainnet => panic!("expected custom network"),
    }
}
