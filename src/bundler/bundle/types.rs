use crate::bundler::types::{BundleInstructionSlots, BundleSlotView, TipMode};
use solana_pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;

pub struct BuiltBundle {
    pub transactions: Vec<VersionedTransaction>,
    pub tip_account: Pubkey,
    pub tip_lamports: u64,
    pub tip_mode: TipMode,
    pub instruction_slots: BundleInstructionSlots,
}

impl BuiltBundle {
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

    pub fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.instruction_slots
    }
}

impl BundleSlotView for BuiltBundle {
    fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.instruction_slots
    }

    fn populated_count(&self) -> usize {
        self.instruction_slots.iter().flatten().count()
    }

    fn last_populated_index(&self) -> Option<usize> {
        self.instruction_slots
            .iter()
            .rposition(|slot| slot.is_some())
    }
}
