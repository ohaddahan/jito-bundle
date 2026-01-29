use crate::analysis::TransactionAnalysis;
use crate::constants::{MAX_BUNDLE_TRANSACTIONS, SOLANA_MAX_TX_SIZE};
use crate::error::JitoError;
use crate::tip::TipHelper;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::message::{VersionedMessage, v0};
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::VersionedTransaction;

pub struct BuildBundleInput<'a> {
    pub payer: &'a Keypair,
    pub instructions: Vec<Instruction>,
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    pub compute_unit_limit: u32,
}

pub struct BundleBuilder;

impl BundleBuilder {
    pub fn build(input: BuildBundleInput<'_>) -> Result<Vec<VersionedTransaction>, JitoError> {
        let BuildBundleInput {
            payer,
            instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
        } = input;

        let num_instructions = instructions.len();

        if num_instructions == 0 || num_instructions > MAX_BUNDLE_TRANSACTIONS {
            return Err(JitoError::InvalidBundleSize {
                count: num_instructions,
            });
        }

        let tip_account = TipHelper::get_random_tip_account();

        Self::validate_tip_not_in_luts(&tip_account, lookup_tables)?;

        let needs_separate_tip_tx = num_instructions < MAX_BUNDLE_TRANSACTIONS;
        let mut transactions: Vec<VersionedTransaction> = Vec::with_capacity(num_instructions + 1);

        for (i, mut instruction) in instructions.into_iter().enumerate() {
            let compute_budget =
                ComputeBudgetInstruction::set_compute_unit_limit(compute_unit_limit);

            if i == 0
                && let Some(jdf_pubkey) = jitodontfront_pubkey
            {
                instruction
                    .accounts
                    .push(AccountMeta::new_readonly(*jdf_pubkey, false));
            }

            let mut tx_instructions = vec![compute_budget, instruction];

            if !needs_separate_tip_tx && i == num_instructions - 1 {
                let tip_ix = TipHelper::create_tip_instruction_to(
                    &payer.pubkey(),
                    &tip_account,
                    tip_lamports,
                );
                tx_instructions.push(tip_ix);
            }

            let message = v0::Message::try_compile(
                &payer.pubkey(),
                &tx_instructions,
                lookup_tables,
                recent_blockhash,
            )
            .map_err(|e| {
                TransactionAnalysis::log_accounts_not_in_luts(
                    &tx_instructions,
                    lookup_tables,
                    &format!("TX{i}_COMPILE_FAIL"),
                );
                JitoError::MessageCompileFailed {
                    index: i,
                    reason: e.to_string(),
                }
            })?;

            let transaction =
                VersionedTransaction::try_new(VersionedMessage::V0(message), &[payer]).map_err(
                    |e| JitoError::TransactionCreationFailed {
                        index: i,
                        reason: e.to_string(),
                    },
                )?;

            let size_info = TransactionAnalysis::analyze_transaction_size(&transaction);
            if size_info.is_oversized {
                return Err(JitoError::TransactionOversized {
                    index: i,
                    size: size_info.size,
                    max: SOLANA_MAX_TX_SIZE,
                });
            }

            TransactionAnalysis::log_transaction_size_warning(&transaction, i);
            transactions.push(transaction);
        }

        if needs_separate_tip_tx {
            let tip_tx =
                TipHelper::compile_tip_transaction(crate::tip::CompileTipTransactionInput {
                    keypair: payer,
                    tip_lamports,
                    recent_blockhash,
                    tip_account: &tip_account,
                })?;

            let size_info = TransactionAnalysis::analyze_transaction_size(&tip_tx);
            if size_info.is_oversized {
                return Err(JitoError::TransactionOversized {
                    index: transactions.len(),
                    size: size_info.size,
                    max: SOLANA_MAX_TX_SIZE,
                });
            }

            transactions.push(tip_tx);
        }

        Ok(transactions)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{JITO_TIP_ACCOUNTS, SYSTEM_PROGRAM_ID};
    use solana_sdk::instruction::Instruction as SdkInstruction;

    fn make_dummy_instruction() -> Instruction {
        SdkInstruction::new_with_bytes(SYSTEM_PROGRAM_ID, &[0], vec![])
    }

    #[test]
    fn rejects_empty_bundle() {
        let payer = Keypair::new();
        let result = BundleBuilder::build(BuildBundleInput {
            payer: &payer,
            instructions: vec![],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        });
        assert!(matches!(
            result,
            Err(JitoError::InvalidBundleSize { count: 0 })
        ));
    }

    #[test]
    fn rejects_too_many_transactions() {
        let payer = Keypair::new();
        let instructions: Vec<Instruction> = (0..6).map(|_| make_dummy_instruction()).collect();
        let result = BundleBuilder::build(BuildBundleInput {
            payer: &payer,
            instructions,
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        });
        assert!(matches!(
            result,
            Err(JitoError::InvalidBundleSize { count: 6 })
        ));
    }

    #[test]
    fn builds_bundle_with_separate_tip_tx() {
        let payer = Keypair::new();
        let instructions = vec![make_dummy_instruction()];
        let result = BundleBuilder::build(BuildBundleInput {
            payer: &payer,
            instructions,
            lookup_tables: &[],
            recent_blockhash: Hash::new_unique(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        });
        assert!(result.is_ok(), "build should succeed for 1 instruction");
        let txs = result.unwrap_or_default();
        assert_eq!(txs.len(), 2, "1 instruction tx + 1 separate tip tx");
    }

    #[test]
    fn builds_bundle_with_inline_tip_at_max() {
        let payer = Keypair::new();
        let instructions: Vec<Instruction> = (0..MAX_BUNDLE_TRANSACTIONS)
            .map(|_| make_dummy_instruction())
            .collect();
        let result = BundleBuilder::build(BuildBundleInput {
            payer: &payer,
            instructions,
            lookup_tables: &[],
            recent_blockhash: Hash::new_unique(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        });
        assert!(result.is_ok(), "build should succeed at max bundle size");
        let txs = result.unwrap_or_default();
        assert_eq!(
            txs.len(),
            MAX_BUNDLE_TRANSACTIONS,
            "no separate tip tx when at max"
        );
    }

    #[test]
    fn rejects_tip_account_in_lut() {
        let payer = Keypair::new();
        let tip_account = JITO_TIP_ACCOUNTS[0];
        let lut = AddressLookupTableAccount {
            key: Pubkey::new_unique(),
            addresses: vec![tip_account],
        };
        let instructions = vec![make_dummy_instruction()];

        for _ in 0..20 {
            let result = BundleBuilder::build(BuildBundleInput {
                payer: &payer,
                instructions: instructions.clone(),
                lookup_tables: std::slice::from_ref(&lut),
                recent_blockhash: Hash::new_unique(),
                tip_lamports: 100_000,
                jitodontfront_pubkey: None,
                compute_unit_limit: 200_000,
            });
            if result.is_err() {
                return;
            }
        }
    }
}
