use crate::JitoError;
use crate::analysis::TransactionAnalysis;
use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use crate::bundler::bundle::types::Bundle;
use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use crate::tip::TipHelper;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::message::{VersionedMessage, v0};
use solana_sdk::signature::Signer;
use solana_sdk::transaction::VersionedTransaction;

impl<'a> BundleBuilder<'a> {
    fn new(inputs: BundleBuilderInputs<'a>) -> Self {
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

    pub fn build(inputs: BundleBuilderInputs<'a>) -> Result<Bundle<'a>, JitoError> {
        let mut builder = Self::new(inputs);
        builder.compact_transactions();
        let count = builder.populated_count();
        if count == 0 {
            return Err(JitoError::InvalidBundleSize { count: 0 });
        }

        if let Some(jitodontfront_pubkey) = builder.jitodontfront_pubkey {
            builder.apply_jitodont_front(jitodontfront_pubkey);
        }

        if count < MAX_BUNDLE_TRANSACTIONS {
            builder.append_tip_transaction()?;
        } else {
            builder.append_tip_instruction();
        }

        let total = builder.populated_count();
        let mut versioned = Vec::with_capacity(total);
        for (compiled_index, ixs) in builder
            .transactions_instructions
            .iter()
            .flatten()
            .enumerate()
        {
            let txn = builder.build_versioned_transaction(compiled_index, total, ixs)?;
            versioned.push(txn);
        }
        builder.versioned_transaction = versioned;
        if !builder.last_txn_is_tip {
            Self::validate_tip_not_in_luts(&builder.tip_account, builder.lookup_tables)?;
        }
        Ok(builder.into())
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
        for slot in &mut self.transactions_instructions {
            if let Some(ixs) = slot.take()
                && idx < new_slots.len()
            {
                new_slots[idx] = Some(ixs);
                idx += 1;
            }
        }
        self.transactions_instructions = new_slots;
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

    pub fn last_populated_index(&self) -> Option<usize> {
        self.transactions_instructions
            .iter()
            .rposition(|slot| slot.is_some())
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
