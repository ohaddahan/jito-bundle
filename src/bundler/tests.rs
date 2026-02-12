#[cfg(test)]
mod bundle_builder_tests {
    use crate::JitoError;
    use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
    use crate::bundler::bundle::types::BuiltBundle;
    use crate::bundler::types::TipMode;
    use crate::constants::{JITO_TIP_ACCOUNTS, SOLANA_MAX_TX_SIZE, SYSTEM_PROGRAM_ID};
    use solana_instruction::{AccountMeta, Instruction};
    use solana_pubkey::Pubkey;
    use solana_sdk::address_lookup_table::AddressLookupTableAccount;
    use solana_sdk::hash::Hash;
    use solana_sdk::signature::{Keypair, Signer};

    fn assert_build_ok(result: Result<BuiltBundle, JitoError>) -> BuiltBundle {
        match result {
            Ok(bundle) => bundle,
            Err(e) => {
                assert!(e.to_string().is_empty(), "build failed: {e}");
                std::process::abort();
            }
        }
    }

    fn get_slot(bundle: &BuiltBundle, index: usize) -> &Vec<Instruction> {
        let slot = &bundle.instruction_slots()[index];
        if let Some(ixs) = slot {
            ixs
        } else {
            assert!(slot.is_some(), "expected Some at slot {index}, got None");
            std::process::abort();
        }
    }

    struct TestBundleParams<'a> {
        pub payer: &'a Keypair,
        pub tx_count: usize,
        pub blockhash: Hash,
        pub luts: &'a [AddressLookupTableAccount],
        pub jdf: Option<&'a Pubkey>,
        pub tip: u64,
    }

    fn make_noop_instruction(payer: &Pubkey) -> Instruction {
        let mut data = vec![2, 0, 0, 0];
        data.extend_from_slice(&0u64.to_le_bytes());
        Instruction {
            program_id: SYSTEM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new(*payer, false),
            ],
            data,
        }
    }

    fn make_custom_instruction(payer: &Pubkey, program_id: Pubkey) -> Instruction {
        Instruction {
            program_id,
            accounts: vec![AccountMeta::new(*payer, true)],
            data: vec![1, 2, 3],
        }
    }

    fn make_bundle_inputs(params: TestBundleParams<'_>) -> BundleBuilderInputs<'_> {
        let TestBundleParams {
            payer,
            tx_count,
            blockhash,
            luts,
            jdf,
            tip,
        } = params;
        let pubkey = payer.pubkey();
        let mut slots = [None, None, None, None, None];
        for slot in slots.iter_mut().take(tx_count) {
            *slot = Some(vec![make_noop_instruction(&pubkey)]);
        }
        BundleBuilderInputs {
            payer,
            transactions_instructions: slots,
            lookup_tables: luts,
            recent_blockhash: blockhash,
            tip_lamports: tip,
            jitodontfront_pubkey: jdf,
            compute_unit_limit: 200_000,
        }
    }

    #[test]
    fn jitodontfront_added_to_first_instruction() {
        let payer = Keypair::new();
        let jdf = Pubkey::new_unique();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: Some(&jdf),
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let first_tx_instructions = get_slot(&bundle, 0);
        let first_ix = &first_tx_instructions[0];
        let last_account = &first_ix.accounts[first_ix.accounts.len() - 1];
        assert_eq!(last_account.pubkey, jdf);
        assert!(!last_account.is_signer);
        assert!(!last_account.is_writable);
    }

    #[test]
    fn jitodontfront_none_means_no_extra_account() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let first_ix = &get_slot(&bundle, 0)[0];
        assert_eq!(first_ix.accounts.len(), 2);
    }

    #[test]
    fn one_tx_produces_two_versioned_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert_eq!(bundle.transactions.len(), 2);
        assert!(matches!(bundle.tip_mode, TipMode::SeparateTx));
    }

    #[test]
    fn four_txs_produce_five_versioned_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 4,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert_eq!(bundle.transactions.len(), 5);
        assert!(matches!(bundle.tip_mode, TipMode::SeparateTx));
    }

    #[test]
    fn five_txs_produce_five_versioned_txs_tip_inline() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert_eq!(bundle.transactions.len(), 5);
        assert!(matches!(bundle.tip_mode, TipMode::InlineLastTx));
    }

    #[test]
    fn zero_transactions_returns_invalid_bundle_size() {
        let payer = Keypair::new();
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [None, None, None, None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        };
        let result = BundleBuilder::build(inputs);
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::InvalidBundleSize { count: 0 })),
            "expected InvalidBundleSize {{ count: 0 }}, got {err:?}"
        );
    }

    #[test]
    fn one_to_five_transactions_all_succeed() {
        for tx_count in 1..=5 {
            let payer = Keypair::new();
            let inputs = make_bundle_inputs(TestBundleParams {
                payer: &payer,
                tx_count,
                blockhash: Hash::default(),
                luts: &[],
                jdf: None,
                tip: 100_000,
            });
            let result = BundleBuilder::build(inputs);
            assert!(result.is_ok(), "expected Ok for {tx_count} transactions");
        }
    }

    #[test]
    fn compiled_transactions_within_size_limit() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 2,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        for (i, tx) in bundle.transactions.iter().enumerate() {
            let serialized = bincode::serialize(tx).unwrap_or_default();
            assert!(
                serialized.len() <= SOLANA_MAX_TX_SIZE,
                "transaction {i} is {size} bytes, exceeds {SOLANA_MAX_TX_SIZE}",
                size = serialized.len()
            );
        }
    }

    #[test]
    fn oversized_transaction_returns_error() {
        let payer = Keypair::new();
        let pubkey = payer.pubkey();
        let big_data = vec![0u8; 1500];
        let big_ix = Instruction {
            program_id: SYSTEM_PROGRAM_ID,
            accounts: vec![AccountMeta::new(pubkey, true)],
            data: big_data,
        };
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [Some(vec![big_ix]), None, None, None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        };
        let result = BundleBuilder::build(inputs);
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::TransactionOversized { .. })),
            "expected TransactionOversized, got {err:?}"
        );
    }

    #[test]
    fn tip_separate_tx_when_under_five() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 2,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(matches!(bundle.tip_mode, TipMode::SeparateTx));
        assert_eq!(bundle.populated_count(), 3);
        let tip_tx = get_slot(&bundle, 2);
        assert_eq!(tip_tx.len(), 1);
        assert_eq!(tip_tx[0].program_id, SYSTEM_PROGRAM_ID);
    }

    #[test]
    fn tip_inline_when_five_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(matches!(bundle.tip_mode, TipMode::InlineLastTx));
        assert_eq!(bundle.populated_count(), 5);
        let last_tx = get_slot(&bundle, 4);
        let last_ix = &last_tx[last_tx.len() - 1];
        assert_eq!(last_ix.program_id, SYSTEM_PROGRAM_ID);
    }

    #[test]
    fn tip_account_is_valid_jito_account() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(
            JITO_TIP_ACCOUNTS.contains(&bundle.tip_account),
            "tip_account {} not in JITO_TIP_ACCOUNTS",
            bundle.tip_account
        );
    }

    #[test]
    fn tip_lamports_encoded_correctly() {
        let payer = Keypair::new();
        let tip_amount: u64 = 500_000;
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: tip_amount,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let last_idx = bundle.last_populated_index();
        assert!(last_idx.is_some(), "no populated slots found");
        let tip_tx = get_slot(&bundle, last_idx.unwrap_or(0));
        let tip_ix = if matches!(bundle.tip_mode, TipMode::SeparateTx) {
            &tip_tx[0]
        } else {
            &tip_tx[tip_tx.len() - 1]
        };
        let encoded_lamports = &tip_ix.data[4..12];
        assert_eq!(encoded_lamports, &tip_amount.to_le_bytes());
    }

    #[test]
    fn tip_account_in_lut_rejected() {
        let payer = Keypair::new();
        let lut_key = Pubkey::new_unique();
        let lut = AddressLookupTableAccount {
            key: lut_key,
            addresses: JITO_TIP_ACCOUNTS.to_vec(),
        };
        let luts = [lut];
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &luts,
            jdf: None,
            tip: 100_000,
        });
        let result = BundleBuilder::build(inputs);
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::TipAccountInLut { .. })),
            "expected TipAccountInLut, got {err:?}"
        );
    }

    #[test]
    fn tip_appended_to_last_populated_slot_even_with_gaps() {
        let payer = Keypair::new();
        let pubkey = payer.pubkey();
        let ix1 = make_custom_instruction(&pubkey, Pubkey::new_unique());
        let ix2 = make_custom_instruction(&pubkey, Pubkey::new_unique());
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [Some(vec![ix1]), None, Some(vec![ix2]), None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        };
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let last_idx = bundle.last_populated_index();
        assert!(last_idx.is_some(), "no populated slots found");
        let last_tx = get_slot(&bundle, last_idx.unwrap_or(0));
        let last_ix = &last_tx[last_tx.len() - 1];
        assert_eq!(
            last_ix.program_id, SYSTEM_PROGRAM_ID,
            "expected tip to be appended to the last populated slot"
        );
    }

    #[test]
    fn jitodontfront_not_duplicated_if_already_present() {
        let payer = Keypair::new();
        let jdf = Pubkey::from_str_const("jitodontfront111111111111111111111111111111");
        let mut ix = make_custom_instruction(&payer.pubkey(), Pubkey::new_unique());
        ix.accounts.push(AccountMeta::new_readonly(jdf, false));
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [Some(vec![ix]), None, None, None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: Some(&jdf),
            compute_unit_limit: 200_000,
        };
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let first_ix = &get_slot(&bundle, 0)[0];
        let count = first_ix
            .accounts
            .iter()
            .filter(|acct| acct.pubkey == jdf)
            .count();
        assert_eq!(
            count, 1,
            "expected jitodontfront account to appear exactly once"
        );
    }
}
