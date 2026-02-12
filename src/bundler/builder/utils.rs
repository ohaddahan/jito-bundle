use crate::JitoError;
use crate::analysis::TransactionAnalysis;
use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use crate::bundler::bundle::types::BuiltBundle;
use crate::bundler::types::{
    BundleInstructionSlots, BundleSlotView, TipMode, empty_instruction_slots,
};
use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use crate::tip::TipHelper;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::message::{VersionedMessage, v0};
use solana_sdk::signature::Signer;
use solana_sdk::transaction::VersionedTransaction;

impl BundleSlotView for BundleBuilder<'_> {
    /// Returns the mutable builder's current instruction slots view.
    fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.transactions_instructions
    }
}

impl<'a> BundleBuilder<'a> {
    // --- Construction ---
    /// Creates a builder from validated build inputs.
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
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
            tip_account,
            tip_mode: TipMode::InlineLastTx,
        }
    }

    // --- Build Pipeline ---
    /// Builds a final `BuiltBundle` from fixed instruction slots.
    ///
    /// Jito hard-limits bundles to `MAX_BUNDLE_TRANSACTIONS` (5), so this builder
    /// chooses tip placement based on remaining transaction capacity.
    ///
    /// Build flow:
    /// 1. Compact sparse slots while preserving transaction order.
    /// 2. Optionally apply `jitodontfront` account rewriting.
    /// 3. Insert tip as separate tx when count < 5, or inline when count == 5.
    /// 4. Validate tip account is not in LUTs for inline mode.
    /// 5. Compile, sign, and size-check each transaction.
    ///
    /// Returns `JitoError` for invalid bundle size, compile/sign failures,
    /// oversized transactions, and invalid LUT tip-account usage.
    pub fn build(inputs: BundleBuilderInputs<'a>) -> Result<BuiltBundle, JitoError> {
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
            builder.tip_mode = TipMode::SeparateTx;
        } else {
            builder.append_tip_instruction();
            builder.tip_mode = TipMode::InlineLastTx;
        }

        if matches!(builder.tip_mode, TipMode::InlineLastTx) {
            Self::validate_tip_not_in_luts(&builder.tip_account, builder.lookup_tables)?;
        }

        let total = builder.populated_count();
        let mut transactions = Vec::with_capacity(total);
        for (compiled_index, ixs) in builder
            .transactions_instructions
            .iter()
            .flatten()
            .enumerate()
        {
            let txn = builder.build_versioned_transaction(compiled_index, total, ixs)?;
            transactions.push(txn);
        }

        Ok(BuiltBundle::new(
            transactions,
            builder.tip_account,
            builder.tip_lamports,
            builder.tip_mode,
            builder.transactions_instructions,
        ))
    }

    /// Compacts sparse slots while preserving transaction order.
    fn compact_transactions(&mut self) {
        let mut new_slots = empty_instruction_slots();
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

    /// Appends the tip as a dedicated transaction when the bundle has room (< 5 txs).
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
        Ok(())
    }

    /// Appends the tip instruction to the last populated transaction when already at 5 txs.
    fn append_tip_instruction(&mut self) {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        if let Some(last_idx) = BundleSlotView::last_populated_index(self)
            && let Some(ixs) = &mut self.transactions_instructions[last_idx]
        {
            ixs.push(tip_ix);
        }
    }

    /// Rewrites `jitodontfront` account usage so it appears only in the first transaction.
    ///
    /// The expected pubkey prefix is `jitodontfront`, while the suffix can vary,
    /// so matching uses string prefix rather than exact full-string equality.
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

    /// Compiles and signs one versioned transaction from instruction list.
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

        let lut: &[AddressLookupTableAccount] =
            if index == total - 1 && matches!(self.tip_mode, TipMode::SeparateTx) {
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

    /// Ensures the chosen tip account is not present in provided LUTs.
    ///
    /// If the tip account appears in a LUT for inline-tip mode, Jito bundle execution
    /// will fail, so this is validated pre-send.
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
