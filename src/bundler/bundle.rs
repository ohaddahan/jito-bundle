use crate::JitoError;
use crate::analysis::TransactionAnalysis;
use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use crate::tip::TipHelper;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::message::{VersionedMessage, v0};
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::VersionedTransaction;

pub struct Bundle<'a> {
    pub versioned_transaction: Vec<VersionedTransaction>,
    pub payer: &'a Keypair,
    pub transactions_instructions: [Option<Vec<Instruction>>; 5],
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    pub compute_unit_limit: u32,
    pub tip_account: Pubkey,
    pub last_txn_is_tip: bool,
}

pub struct BundleBuilderInputs<'a> {
    pub payer: &'a Keypair,
    pub transactions_instructions: [Option<Vec<Instruction>>; 5],
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    pub compute_unit_limit: u32,
}

impl<'a> Bundle<'a> {
    pub fn new(inputs: BundleBuilderInputs<'a>) -> Self {
        let BundleBuilderInputs {
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
        } = inputs;
        let tip_account = TipHelper::get_random_tip_account();
        Self {
            versioned_transaction: vec![],
            tip_account,
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
            last_txn_is_tip: false,
        }
    }

    fn populated_count(&self) -> usize {
        self.transactions_instructions
            .iter()
            .filter(|slot| slot.is_some())
            .count()
    }

    fn compact_transactions(&mut self) {
        let mut new_slots: [Option<Vec<Instruction>>; 5] = std::array::from_fn(|_| None);
        let mut idx = 0;
        for slot in self.transactions_instructions.iter_mut() {
            if let Some(ixs) = slot.take() {
                if idx < new_slots.len() {
                    new_slots[idx] = Some(ixs);
                    idx += 1;
                }
            }
        }
        self.transactions_instructions = new_slots;
    }

    fn last_populated_index(&self) -> Option<usize> {
        self.transactions_instructions
            .iter()
            .rposition(|slot| slot.is_some())
    }

    fn append_tip_transaction(&mut self) -> Result<(), JitoError> {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        let first_none = self
            .transactions_instructions
            .iter()
            .position(|slot| slot.is_none())
            .ok_or(JitoError::InvalidBundleSize {
                count: MAX_BUNDLE_TRANSACTIONS,
            })?;
        self.transactions_instructions[first_none] = Some(vec![tip_ix]);
        self.last_txn_is_tip = true;
        Ok(())
    }

    fn append_tip_instruction(&mut self) {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        if let Some(last_idx) = self.last_populated_index()
            && let Some(ixs) = &mut self.transactions_instructions[last_idx]
        {
            ixs.push(tip_ix);
        }
    }

    fn apply_jitodont_front(&mut self, jitodontfront_pubkey: &Pubkey) {
        for ixs in self.transactions_instructions.iter_mut().flatten() {
            for instruction in ixs.iter_mut() {
                instruction
                    .accounts
                    .retain(|acct| !acct.pubkey.to_string().starts_with("jitodontfront"));
            }
        }
        if let Some(Some(ixs)) = self.transactions_instructions.first_mut()
            && let Some(instruction) = ixs.first_mut()
        {
            instruction
                .accounts
                .push(AccountMeta::new_readonly(*jitodontfront_pubkey, false));
        }
    }

    fn build_versioned_transaction(
        &self,
        index: usize,
        total: usize,
        tx_instructions: &[Instruction],
    ) -> Result<VersionedTransaction, JitoError> {
        let compute_budget =
            ComputeBudgetInstruction::set_compute_unit_limit(self.compute_unit_limit);
        let mut instructions = vec![compute_budget];
        instructions.extend_from_slice(tx_instructions);

        let lut: &[AddressLookupTableAccount] = if index == total - 1 && self.last_txn_is_tip {
            &[]
        } else {
            self.lookup_tables
        };

        let message = v0::Message::try_compile(
            &self.payer.pubkey(),
            &instructions,
            lut,
            self.recent_blockhash,
        )
        .map_err(|e| {
            TransactionAnalysis::log_accounts_not_in_luts(
                &instructions,
                lut,
                &format!("TX: {index} COMPILE_FAIL"),
            );
            JitoError::MessageCompileFailed {
                index,
                reason: e.to_string(),
            }
        })?;
        let txn = VersionedTransaction::try_new(VersionedMessage::V0(message), &[self.payer])
            .map_err(|e| JitoError::TransactionCreationFailed {
                index,
                reason: e.to_string(),
            })?;
        let size_info = TransactionAnalysis::analyze_transaction_size(&txn);
        if size_info.is_oversized {
            return Err(JitoError::TransactionOversized {
                index,
                size: size_info.size,
                max: size_info.max_size,
            });
        }
        Ok(txn)
    }

    pub fn build(mut self) -> Result<Self, JitoError> {
        self.compact_transactions();
        let count = self.populated_count();
        if count == 0 {
            return Err(JitoError::InvalidBundleSize { count: 0 });
        }

        if let Some(jitodontfront_pubkey) = self.jitodontfront_pubkey {
            self.apply_jitodont_front(jitodontfront_pubkey);
        }

        if count < MAX_BUNDLE_TRANSACTIONS {
            self.append_tip_transaction()?;
        } else {
            self.append_tip_instruction();
        }

        let total = self.populated_count();
        let mut versioned = Vec::with_capacity(total);
        for (compiled_index, ixs) in self.transactions_instructions.iter().flatten().enumerate() {
            let txn = self.build_versioned_transaction(compiled_index, total, ixs)?;
            versioned.push(txn);
        }
        self.versioned_transaction = versioned;

        if !self.last_txn_is_tip {
            Self::validate_tip_not_in_luts(&self.tip_account, self.lookup_tables)?;
        }

        Ok(self)
    }

    fn validate_tip_not_in_luts(
        tip_account: &Pubkey,
        lookup_tables: &[AddressLookupTableAccount],
    ) -> Result<(), JitoError> {
        for lut in lookup_tables {
            if lut.addresses.contains(tip_account) {
                return Err(JitoError::TipAccountInLut {
                    tip_account: tip_account.to_string(),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{JITO_TIP_ACCOUNTS, SOLANA_MAX_TX_SIZE, SYSTEM_PROGRAM_ID};
    use solana_sdk::signature::Keypair;

    fn assert_build_ok(result: Result<Bundle<'_>, JitoError>) -> Bundle<'_> {
        match result {
            Ok(b) => b,
            Err(e) => {
                assert!(e.to_string().is_empty(), "build failed: {e}");
                std::process::abort();
            }
        }
    }

    fn get_slot<'a>(bundle: &'a Bundle<'_>, index: usize) -> &'a Vec<Instruction> {
        match &bundle.transactions_instructions[index] {
            Some(ixs) => ixs,
            None => {
                assert!(false, "expected Some at slot {index}, got None");
                std::process::abort();
            }
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
        let mut slots: [Option<Vec<Instruction>>; 5] = [None, None, None, None, None];
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert_eq!(bundle.versioned_transaction.len(), 2);
        assert!(bundle.last_txn_is_tip);
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert_eq!(bundle.versioned_transaction.len(), 5);
        assert!(bundle.last_txn_is_tip);
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert_eq!(bundle.versioned_transaction.len(), 5);
        assert!(!bundle.last_txn_is_tip);
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
        let result = Bundle::new(inputs).build();
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
            let result = Bundle::new(inputs).build();
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        for (i, tx) in bundle.versioned_transaction.iter().enumerate() {
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
        let result = Bundle::new(inputs).build();
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert!(bundle.last_txn_is_tip);
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert!(!bundle.last_txn_is_tip);
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        let last_idx = bundle.last_populated_index();
        assert!(last_idx.is_some(), "no populated slots found");
        let tip_tx = get_slot(&bundle, last_idx.unwrap_or(0));
        let tip_ix = if bundle.last_txn_is_tip {
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
        let result = Bundle::new(inputs).build();
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        let last_idx = match bundle.last_populated_index() {
            Some(idx) => idx,
            None => {
                assert!(false, "no populated slots found");
                std::process::abort();
            }
        };
        let last_tx = get_slot(&bundle, last_idx);
        let last_ix = &last_tx[last_tx.len() - 1];
        assert_eq!(
            last_ix.program_id, SYSTEM_PROGRAM_ID,
            "expected tip to be appended to the last populated slot"
        );
    }

    #[test]
    fn jitodontfront_not_duplicated_if_already_present() {
        let payer = Keypair::new();
        let jdf = Pubkey::new_unique();
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
        let bundle = assert_build_ok(Bundle::new(inputs).build());
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
