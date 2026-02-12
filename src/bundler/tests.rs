#[cfg(test)]
mod bundle_builder_tests {
    use crate::JitoError;
    use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
    use crate::bundler::bundle::types::BuiltBundle;
    use crate::bundler::types::{BundleSlotView, TipMode};
    use crate::constants::{JITO_TIP_ACCOUNTS, SOLANA_MAX_TX_SIZE, SYSTEM_PROGRAM_ID};
    use solana_instruction::{AccountMeta, Instruction};
    use solana_pubkey::Pubkey;
    use solana_sdk::address_lookup_table::AddressLookupTableAccount;
    use solana_sdk::hash::Hash;
    use solana_sdk::signature::{Keypair, Signer};

    /// Unwraps successful build results with test-friendly failure output.
    fn assert_build_ok(result: Result<BuiltBundle, JitoError>) -> BuiltBundle {
        match result {
            Ok(bundle) => bundle,
            Err(e) => {
                assert!(e.to_string().is_empty(), "build failed: {e}");
                std::process::abort();
            }
        }
    }

    /// Returns instructions stored in a specific post-build slot.
    fn get_slot(bundle: &BuiltBundle, index: usize) -> &Vec<Instruction> {
        let slot = &bundle.instruction_slots()[index];
        if let Some(ixs) = slot {
            ixs
        } else {
            assert!(slot.is_some(), "expected Some at slot {index}, got None");
            std::process::abort();
        }
    }

    /// Returns true when instruction matches the expected tip transfer format.
    fn is_tip_instruction(ix: &Instruction, tip_account: &Pubkey, tip_lamports: u64) -> bool {
        let lamports = tip_lamports.to_le_bytes();
        ix.program_id == SYSTEM_PROGRAM_ID
            && ix.accounts.len() >= 2
            && ix.accounts[1].pubkey == *tip_account
            && ix
                .data
                .get(0..4)
                .is_some_and(|op| op == [2, 0, 0, 0].as_slice())
            && ix
                .data
                .get(4..12)
                .is_some_and(|bytes| bytes == lamports.as_slice())
    }

    /// Counts tip instructions across all populated slots.
    fn count_tip_instructions(bundle: &BuiltBundle) -> usize {
        bundle
            .instruction_slots()
            .iter()
            .flatten()
            .flat_map(|ixs| ixs.iter())
            .filter(|ix| is_tip_instruction(ix, &bundle.tip_account, bundle.tip_lamports))
            .count()
    }

    struct TestBundleParams<'a> {
        payer: &'a Keypair,
        tx_count: usize,
        blockhash: Hash,
        luts: &'a [AddressLookupTableAccount],
        jdf: Option<&'a Pubkey>,
        tip: u64,
    }

    /// Creates a simple system instruction used by test transactions.
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

    /// Creates a custom single-account instruction for tests.
    fn make_custom_instruction(payer: &Pubkey, program_id: Pubkey) -> Instruction {
        Instruction {
            program_id,
            accounts: vec![AccountMeta::new(*payer, true)],
            data: vec![1, 2, 3],
        }
    }

    /// Builds standard test inputs with configurable tx count and options.
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

    /// Verifies jito-dont-front account gets injected into first instruction.
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

    /// Verifies no extra account is added when JDF is disabled.
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

    /// Verifies one input transaction produces one tx plus separate tip tx.
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

    /// Verifies four input transactions produce five total with separate tip.
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

    /// Verifies five input transactions keep tip inline in final transaction.
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

    /// Verifies empty bundles are rejected with size error.
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

    /// Verifies every bundle size from one through five builds successfully.
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

    /// Verifies built transactions respect Solana max serialized size.
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
        assert!(
            !bundle.transactions.is_empty(),
            "expected at least one compiled transaction"
        );
        for (i, tx) in bundle.transactions.iter().enumerate() {
            let serialized_result = bincode::serialize(tx);
            assert!(
                serialized_result.is_ok(),
                "failed to serialize transaction {i}"
            );
            let serialized = serialized_result.unwrap_or_default();
            assert!(
                !serialized.is_empty(),
                "transaction {i} serialized to empty bytes"
            );
            assert!(
                serialized.len() <= SOLANA_MAX_TX_SIZE,
                "transaction {i} is {size} bytes, exceeds {SOLANA_MAX_TX_SIZE}",
                size = serialized.len()
            );
        }
    }

    /// Verifies oversized instruction payloads fail at build time.
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

    /// Verifies tip is emitted as a separate transaction when bundle is not full.
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
        assert_eq!(count_tip_instructions(&bundle), 1);
        for idx in 0..2 {
            let has_tip_in_original_tx = get_slot(&bundle, idx)
                .iter()
                .any(|ix| is_tip_instruction(ix, &bundle.tip_account, bundle.tip_lamports));
            assert!(
                !has_tip_in_original_tx,
                "did not expect tip instruction inline in slot {idx}"
            );
        }
        let tip_tx = get_slot(&bundle, 2);
        assert_eq!(tip_tx.len(), 1);
        assert!(is_tip_instruction(
            &tip_tx[0],
            &bundle.tip_account,
            bundle.tip_lamports
        ));
    }

    /// Verifies tip is appended inline when bundle already has five txs.
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
        assert_eq!(bundle.transactions.len(), 5);
        assert_eq!(bundle.populated_count(), 5);
        assert_eq!(count_tip_instructions(&bundle), 1);
        for idx in 0..4 {
            let has_tip_in_non_last_tx = get_slot(&bundle, idx)
                .iter()
                .any(|ix| is_tip_instruction(ix, &bundle.tip_account, bundle.tip_lamports));
            assert!(
                !has_tip_in_non_last_tx,
                "did not expect tip instruction in non-last slot {idx}"
            );
        }
        let last_tx = get_slot(&bundle, 4);
        let last_ix = &last_tx[last_tx.len() - 1];
        assert!(is_tip_instruction(
            last_ix,
            &bundle.tip_account,
            bundle.tip_lamports
        ));
    }

    /// Verifies selected tip account is always from known Jito tip set.
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

    /// Verifies encoded transfer lamports in tip instruction are correct.
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

    /// Verifies inline tip mode rejects LUTs containing tip accounts.
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

    /// Verifies inline mode succeeds when LUTs do not contain tip account.
    #[test]
    fn tip_account_not_in_lut_is_accepted() {
        let payer = Keypair::new();
        let lut = AddressLookupTableAccount {
            key: Pubkey::new_unique(),
            addresses: vec![Pubkey::new_unique(), Pubkey::new_unique()],
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
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(matches!(bundle.tip_mode, TipMode::InlineLastTx));
    }

    /// Verifies compaction preserves last slot used for inline tip append.
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

    /// Verifies JDF account is not duplicated if already present.
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
