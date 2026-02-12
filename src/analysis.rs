use crate::constants::SOLANA_MAX_TX_SIZE;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::transaction::VersionedTransaction;
use std::collections::HashSet;

/// Serialized-size details for a compiled transaction.
#[derive(Debug)]
pub struct TransactionSizeInfo {
    /// Serialized transaction size in bytes.
    pub size: usize,
    /// Maximum allowed Solana transaction size in bytes.
    pub max_size: usize,
    /// Whether serialized size exceeds `max_size`.
    pub is_oversized: bool,
}

/// Summary of accounts missing from provided lookup tables.
#[derive(Debug)]
pub struct AccountsNotInLuts {
    /// Accounts referenced by instructions but absent from LUTs.
    pub accounts: Vec<Pubkey>,
    /// Total unique accounts referenced by instructions.
    pub total_accounts: usize,
    /// Number of referenced accounts found in LUTs.
    pub accounts_in_luts: usize,
}

/// Helpers for transaction size and LUT diagnostics.
pub struct TransactionAnalysis;

impl TransactionAnalysis {
    // --- Size Analysis ---
    /// Computes serialized transaction size and max-size compliance.
    pub fn analyze_transaction_size(tx: &VersionedTransaction) -> TransactionSizeInfo {
        let size = bincode::serialize(tx).map_or(0, |s| s.len());
        TransactionSizeInfo {
            size,
            max_size: SOLANA_MAX_TX_SIZE,
            is_oversized: size > SOLANA_MAX_TX_SIZE,
        }
    }

    // --- LUT Analysis ---
    /// Returns accounts referenced by instructions but missing from LUTs.
    pub fn get_accounts_not_in_luts(
        instructions: &[Instruction],
        lookup_tables: &[AddressLookupTableAccount],
    ) -> AccountsNotInLuts {
        let all_accounts: HashSet<Pubkey> = instructions
            .iter()
            .flat_map(|ix| {
                std::iter::once(ix.program_id).chain(ix.accounts.iter().map(|m| m.pubkey))
            })
            .collect();

        let lut_accounts: HashSet<Pubkey> = lookup_tables
            .iter()
            .flat_map(|lut| lut.addresses.iter().copied())
            .collect();

        let accounts: Vec<Pubkey> = all_accounts.difference(&lut_accounts).copied().collect();
        let total_accounts = all_accounts.len();
        let accounts_in_luts = total_accounts.saturating_sub(accounts.len());

        AccountsNotInLuts {
            accounts,
            total_accounts,
            accounts_in_luts,
        }
    }

    // --- Logging Helpers ---
    /// Logs serialized transaction size diagnostics.
    pub fn log_transaction_size_warning(tx: &VersionedTransaction, tx_index: usize) {
        let size_info = Self::analyze_transaction_size(tx);
        if size_info.is_oversized {
            tracing::error!(
                "transaction {tx_index} is OVERSIZED: {size}/{max} bytes (exceeds by {excess} bytes)",
                size = size_info.size,
                max = size_info.max_size,
                excess = size_info.size - size_info.max_size
            );
        } else {
            tracing::info!(
                "transaction {tx_index} size: {size}/{max} bytes ({remaining} bytes remaining)",
                size = size_info.size,
                max = size_info.max_size,
                remaining = size_info.max_size - size_info.size
            );
        }
    }

    /// Logs which accounts are present or missing in provided LUTs.
    pub fn log_accounts_not_in_luts(
        instructions: &[Instruction],
        lookup_tables: &[AddressLookupTableAccount],
        context: &str,
    ) {
        let analysis = Self::get_accounts_not_in_luts(instructions, lookup_tables);

        if analysis.accounts.is_empty() {
            tracing::info!(
                "[{context}] all {total} accounts are in LUTs",
                total = analysis.total_accounts
            );
            return;
        }

        tracing::warn!(
            "[{context}] {missing_count}/{total} accounts NOT in LUTs ({in_luts} in LUTs)",
            missing_count = analysis.accounts.len(),
            total = analysis.total_accounts,
            in_luts = analysis.accounts_in_luts
        );

        for (i, account) in analysis.accounts.iter().enumerate() {
            tracing::warn!("[{context}] missing LUT account [{i}]: {account}");
        }
    }

    /// Logs full bundle diagnostics for post-failure debugging.
    pub fn log_bundle_failure_analysis(
        transactions: &[VersionedTransaction],
        all_instructions: &[Option<Vec<Instruction>>],
        lookup_tables: &[AddressLookupTableAccount],
        error: &str,
    ) {
        tracing::error!("=== BUNDLE FAILURE ANALYSIS ===");
        tracing::error!("error: {error}");

        for (i, tx) in transactions.iter().enumerate() {
            tracing::error!("--- transaction {i} ---");
            Self::log_transaction_size_warning(tx, i);

            if let Some(Some(ixs)) = all_instructions.get(i) {
                Self::log_accounts_not_in_luts(ixs, lookup_tables, &format!("TX{i}"));
            }
        }

        tracing::error!("=== END ANALYSIS ===");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_instruction::AccountMeta;
    use solana_pubkey::Pubkey;

    /// Verifies failure analysis handles sparse optional instruction slots.
    #[test]
    fn log_bundle_failure_analysis_handles_option_slots() {
        let ix = Instruction {
            program_id: Pubkey::new_unique(),
            accounts: vec![AccountMeta::new(Pubkey::new_unique(), true)],
            data: vec![1, 2, 3],
        };
        let slots: [Option<Vec<Instruction>>; 5] = [Some(vec![ix]), None, None, None, None];
        TransactionAnalysis::log_bundle_failure_analysis(&[], &slots, &[], "test error");
    }
}
