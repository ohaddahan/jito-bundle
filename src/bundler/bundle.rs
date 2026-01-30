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
    pub transactions: Vec<Vec<Instruction>>,
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
    pub transactions: Vec<Vec<Instruction>>,
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
            transactions,
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
            transactions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
            last_txn_is_tip: false,
        }
    }

    fn append_tip_transaction(&mut self) {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        self.transactions.push(vec![tip_ix]);
        self.last_txn_is_tip = true;
    }

    fn append_tip_instruction(&mut self) {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        if let Some(transaction) = self.transactions.last_mut() {
            transaction.push(tip_ix);
        }
    }

    fn apply_jitodont_front(&mut self, jitodontfront_pubkey: &Pubkey) {
        for transaction in &mut self.transactions {
            for instruction in transaction.iter_mut() {
                instruction
                    .accounts
                    .retain(|acct| !acct.pubkey.to_string().starts_with("jitodontfront"));
            }
        }
        if let Some(transaction) = self.transactions.first_mut()
            && let Some(instruction) = transaction.first_mut()
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
        if self.transactions.is_empty() || self.transactions.len() > MAX_BUNDLE_TRANSACTIONS {
            return Err(JitoError::InvalidBundleSize {
                count: self.transactions.len(),
            });
        }

        if let Some(jitodontfront_pubkey) = self.jitodontfront_pubkey {
            self.apply_jitodont_front(jitodontfront_pubkey);
        }

        if self.transactions.len() < MAX_BUNDLE_TRANSACTIONS {
            self.append_tip_transaction();
        } else {
            self.append_tip_instruction();
        }

        let total = self.transactions.len();
        let mut versioned = Vec::with_capacity(total);
        for index in 0..total {
            let txn = self.build_versioned_transaction(index, total, &self.transactions[index])?;
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

    fn make_bundle_inputs<'a>(
        payer: &'a Keypair,
        tx_count: usize,
        blockhash: Hash,
        luts: &'a [AddressLookupTableAccount],
        jdf: Option<&'a Pubkey>,
        tip: u64,
    ) -> BundleBuilderInputs<'a> {
        let pubkey = payer.pubkey();
        let transactions = (0..tx_count)
            .map(|_| vec![make_noop_instruction(&pubkey)])
            .collect();
        BundleBuilderInputs {
            payer,
            transactions,
            lookup_tables: luts,
            recent_blockhash: blockhash,
            tip_lamports: tip,
            jitodontfront_pubkey: jdf,
            compute_unit_limit: 200_000,
        }
    }

    // --- Item 1: jitodontfront ---

    #[test]
    fn jitodontfront_added_to_first_instruction() {
        let payer = Keypair::new();
        let jdf = Pubkey::new_unique();
        let inputs = make_bundle_inputs(&payer, 1, Hash::default(), &[], Some(&jdf), 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        let first_tx_instructions = &bundle.transactions[0];
        let first_ix = &first_tx_instructions[0];
        let last_account = &first_ix.accounts[first_ix.accounts.len() - 1];
        assert_eq!(last_account.pubkey, jdf);
        assert!(!last_account.is_signer);
        assert!(!last_account.is_writable);
    }

    #[test]
    fn jitodontfront_none_means_no_extra_account() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 1, Hash::default(), &[], None, 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        let first_ix = &bundle.transactions[0][0];
        assert_eq!(first_ix.accounts.len(), 2);
    }

    // --- Item 2: Coverage / tx count ---

    #[test]
    fn one_tx_produces_two_versioned_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 1, Hash::default(), &[], None, 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert_eq!(bundle.versioned_transaction.len(), 2);
        assert!(bundle.last_txn_is_tip);
    }

    #[test]
    fn four_txs_produce_five_versioned_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 4, Hash::default(), &[], None, 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert_eq!(bundle.versioned_transaction.len(), 5);
        assert!(bundle.last_txn_is_tip);
    }

    #[test]
    fn five_txs_produce_five_versioned_txs_tip_inline() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 5, Hash::default(), &[], None, 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert_eq!(bundle.versioned_transaction.len(), 5);
        assert!(!bundle.last_txn_is_tip);
    }

    // --- Item 3: Bundle size validation ---

    #[test]
    fn zero_transactions_returns_invalid_bundle_size() {
        let payer = Keypair::new();
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions: vec![],
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
    fn six_transactions_returns_invalid_bundle_size() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 6, Hash::default(), &[], None, 100_000);
        let result = Bundle::new(inputs).build();
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::InvalidBundleSize { count: 6 })),
            "expected InvalidBundleSize {{ count: 6 }}, got {err:?}"
        );
    }

    #[test]
    fn one_to_five_transactions_all_succeed() {
        for tx_count in 1..=5 {
            let payer = Keypair::new();
            let inputs = make_bundle_inputs(&payer, tx_count, Hash::default(), &[], None, 100_000);
            let result = Bundle::new(inputs).build();
            assert!(result.is_ok(), "expected Ok for {tx_count} transactions");
        }
    }

    // --- Item 4: Transaction size ---

    #[test]
    fn compiled_transactions_within_size_limit() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 2, Hash::default(), &[], None, 100_000);
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
            transactions: vec![vec![big_ix]],
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

    // --- Item 5: Tip scenarios ---

    #[test]
    fn tip_separate_tx_when_under_five() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 2, Hash::default(), &[], None, 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert!(bundle.last_txn_is_tip);
        assert_eq!(bundle.transactions.len(), 3);
        let tip_tx = &bundle.transactions[2];
        assert_eq!(tip_tx.len(), 1);
        assert_eq!(tip_tx[0].program_id, SYSTEM_PROGRAM_ID);
    }

    #[test]
    fn tip_inline_when_five_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 5, Hash::default(), &[], None, 100_000);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        assert!(!bundle.last_txn_is_tip);
        assert_eq!(bundle.transactions.len(), 5);
        let last_tx = &bundle.transactions[4];
        let last_ix = &last_tx[last_tx.len() - 1];
        assert_eq!(last_ix.program_id, SYSTEM_PROGRAM_ID);
    }

    #[test]
    fn tip_account_is_valid_jito_account() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(&payer, 1, Hash::default(), &[], None, 100_000);
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
        let inputs = make_bundle_inputs(&payer, 1, Hash::default(), &[], None, tip_amount);
        let bundle = assert_build_ok(Bundle::new(inputs).build());
        let tip_tx = &bundle.transactions[bundle.transactions.len() - 1];
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
        let inputs = make_bundle_inputs(&payer, 5, Hash::default(), &luts, None, 100_000);
        let result = Bundle::new(inputs).build();
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::TipAccountInLut { .. })),
            "expected TipAccountInLut, got {err:?}"
        );
    }
}
