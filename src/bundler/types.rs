use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use solana_instruction::Instruction;

/// Fixed-size instruction slots aligned with Jito's max bundle size.
pub type BundleInstructionSlots = [Option<Vec<Instruction>>; MAX_BUNDLE_TRANSACTIONS];

/// Indicates how the tip was placed in the built bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipMode {
    /// Tip is appended as a dedicated final transaction.
    SeparateTx,
    /// Tip is appended to the last existing transaction.
    InlineLastTx,
}

/// Shared view over fixed instruction slots.
pub trait BundleSlotView {
    /// Returns the underlying fixed instruction slots.
    fn instruction_slots(&self) -> &BundleInstructionSlots;

    /// Counts non-empty instruction slots.
    fn populated_count(&self) -> usize {
        self.instruction_slots()
            .iter()
            .filter(|slot| slot.is_some())
            .count()
    }

    /// Returns the index of the last non-empty slot.
    fn last_populated_index(&self) -> Option<usize> {
        self.instruction_slots()
            .iter()
            .rposition(|slot| slot.is_some())
    }
}

/// Creates an empty fixed-size slot array.
pub fn empty_instruction_slots() -> BundleInstructionSlots {
    std::array::from_fn(|_| None)
}
