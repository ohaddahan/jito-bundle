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
