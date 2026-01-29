use crate::constants::SOLANA_MAX_TX_SIZE;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::transaction::VersionedTransaction;
use std::collections::HashSet;

#[derive(Debug)]
pub struct TransactionSizeInfo {
    pub size: usize,
    pub max_size: usize,
    pub is_oversized: bool,
}

#[derive(Debug)]
pub struct AccountsNotInLuts {
    pub accounts: Vec<Pubkey>,
    pub total_accounts: usize,
    pub accounts_in_luts: usize,
}

pub struct TransactionAnalysis;

impl TransactionAnalysis {
    pub fn analyze_transaction_size(tx: &VersionedTransaction) -> TransactionSizeInfo {
        let size = bincode::serialize(tx).map_or(0, |s| s.len());
        TransactionSizeInfo {
            size,
            max_size: SOLANA_MAX_TX_SIZE,
            is_oversized: size > SOLANA_MAX_TX_SIZE,
        }
    }

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

    pub fn log_bundle_failure_analysis(
        transactions: &[VersionedTransaction],
        all_instructions: &[Vec<Instruction>],
        lookup_tables: &[AddressLookupTableAccount],
        error: &str,
    ) {
        tracing::error!("=== BUNDLE FAILURE ANALYSIS ===");
        tracing::error!("error: {error}");

        for (i, tx) in transactions.iter().enumerate() {
            tracing::error!("--- transaction {i} ---");
            Self::log_transaction_size_warning(tx, i);

            if let Some(ixs) = all_instructions.get(i) {
                Self::log_accounts_not_in_luts(ixs, lookup_tables, &format!("TX{i}"));
            }
        }

        tracing::error!("=== END ANALYSIS ===");
    }
}
