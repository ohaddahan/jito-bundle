use crate::bundler::bundle::types::Bundle;

impl Bundle<'_> {
    pub fn last_populated_index(&self) -> Option<usize> {
        self.transactions_instructions
            .iter()
            .rposition(|slot| slot.is_some())
    }

    pub fn populated_count(&self) -> usize {
        self.transactions_instructions
            .iter()
            .filter(|slot| slot.is_some())
            .count()
    }
}
