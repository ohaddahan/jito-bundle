use crate::bundler::types::{BundleInstructionSlots, BundleSlotView, TipMode};
use solana_pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;

/// Final bundle artifact ready for simulation and submission.
pub struct BuiltBundle {
    /// Fully compiled and signed versioned transactions.
    pub transactions: Vec<VersionedTransaction>,
    /// Chosen Jito tip account.
    pub tip_account: Pubkey,
    /// Effective tip amount in lamports.
    pub tip_lamports: u64,
    /// How the tip instruction was inserted.
    pub tip_mode: TipMode,
    /// Post-compaction instruction slots used to build the transactions.
    pub instruction_slots: BundleInstructionSlots,
}

impl BuiltBundle {
    // --- Construction ---
    /// Creates a new built bundle value.
    pub fn new(
        transactions: Vec<VersionedTransaction>,
        tip_account: Pubkey,
        tip_lamports: u64,
        tip_mode: TipMode,
        instruction_slots: BundleInstructionSlots,
    ) -> Self {
        Self {
            transactions,
            tip_account,
            tip_lamports,
            tip_mode,
            instruction_slots,
        }
    }

    // --- Accessors ---
    /// Returns the post-compaction instruction slots.
    pub fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.instruction_slots
    }
}

impl BundleSlotView for BuiltBundle {
    /// Returns the post-compaction instruction slots.
    fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.instruction_slots
    }

    /// Counts non-empty instruction slots.
    fn populated_count(&self) -> usize {
        self.instruction_slots.iter().flatten().count()
    }

    /// Returns the index of the last non-empty slot.
    fn last_populated_index(&self) -> Option<usize> {
        self.instruction_slots
            .iter()
            .rposition(|slot| slot.is_some())
    }
}
