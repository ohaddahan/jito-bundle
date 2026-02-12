use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use solana_instruction::Instruction;

pub type BundleInstructionSlots = [Option<Vec<Instruction>>; MAX_BUNDLE_TRANSACTIONS];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipMode {
    SeparateTx,
    InlineLastTx,
}

pub trait BundleSlotView {
    fn instruction_slots(&self) -> &BundleInstructionSlots;

    fn populated_count(&self) -> usize {
        self.instruction_slots()
            .iter()
            .filter(|slot| slot.is_some())
            .count()
    }

    fn last_populated_index(&self) -> Option<usize> {
        self.instruction_slots()
            .iter()
            .rposition(|slot| slot.is_some())
    }
}

pub fn empty_instruction_slots() -> BundleInstructionSlots {
    std::array::from_fn(|_| None)
}
