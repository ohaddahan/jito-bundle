use crate::config::tip_strategy::TipStrategy;
use crate::constants::{
    DEFAULT_TIP_LAMPORTS, JITO_TIP_ACCOUNTS, MAX_TIP_LAMPORTS, SYSTEM_PROGRAM_ID,
};
use crate::error::JitoError;
use crate::types::JitoTipFloorResponse;
use rand::Rng;
use reqwest::Client;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::hash::Hash;
use solana_sdk::message::{VersionedMessage, v0};
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::VersionedTransaction;

pub struct TipHelper;

impl TipHelper {
    pub fn get_random_tip_account() -> Pubkey {
        let index = rand::rng().random_range(0..JITO_TIP_ACCOUNTS.len());
        JITO_TIP_ACCOUNTS[index]
    }

    pub fn create_tip_instruction(payer: &Pubkey, tip_lamports: u64) -> Instruction {
        let tip_account = Self::get_random_tip_account();
        Self::create_tip_instruction_to(payer, &tip_account, tip_lamports)
    }

    pub fn create_tip_instruction_to(
        payer: &Pubkey,
        tip_account: &Pubkey,
        tip_lamports: u64,
    ) -> Instruction {
        let mut data = vec![2, 0, 0, 0];
        data.extend_from_slice(&tip_lamports.to_le_bytes());
        Instruction {
            program_id: SYSTEM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new(*tip_account, false),
            ],
            data,
        }
    }

    pub fn compile_tip_transaction(
        input: CompileTipTransactionInput<'_>,
    ) -> Result<VersionedTransaction, JitoError> {
        let CompileTipTransactionInput {
            keypair,
            tip_lamports,
            recent_blockhash,
            tip_account,
        } = input;

        let tip_ix = Self::create_tip_instruction_to(&keypair.pubkey(), tip_account, tip_lamports);

        let message = v0::Message::try_compile(&keypair.pubkey(), &[tip_ix], &[], recent_blockhash)
            .map_err(|e| JitoError::MessageCompileFailed {
                index: 0,
                reason: e.to_string(),
            })?;

        VersionedTransaction::try_new(VersionedMessage::V0(message), &[keypair]).map_err(|e| {
            JitoError::TransactionCreationFailed {
                index: 0,
                reason: e.to_string(),
            }
        })
    }

    pub async fn fetch_tip_floor(
        client: &Client,
        tip_floor_url: &str,
    ) -> Result<(u64, JitoTipFloorResponse), JitoError> {
        let response = client
            .get(tip_floor_url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| JitoError::TipFloorFetchFailed {
                reason: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(JitoError::TipFloorFetchFailed {
                reason: format!("HTTP {}", response.status()),
            });
        }

        let data: Vec<JitoTipFloorResponse> =
            response
                .json()
                .await
                .map_err(|e| JitoError::TipFloorFetchFailed {
                    reason: format!("failed to parse response: {e}"),
                })?;

        let tip_data = data.first().ok_or_else(|| JitoError::TipFloorFetchFailed {
            reason: "tip_floor returned an empty array".to_string(),
        })?;

        let tip_float = (tip_data.ema_landed_tips_50th_percentile * 1e9).ceil();
        let tip_in_lamports = if tip_float.is_sign_negative() || tip_float.is_nan() {
            0u64
        } else {
            tip_float as u64
        };
        let final_tip = tip_in_lamports.clamp(DEFAULT_TIP_LAMPORTS, MAX_TIP_LAMPORTS);

        Ok((final_tip, tip_data.clone()))
    }

    pub async fn resolve_tip(
        client: &Client,
        tip_floor_url: &str,
        strategy: &TipStrategy,
    ) -> Result<u64, JitoError> {
        match strategy {
            TipStrategy::Fixed(lamports) => Ok(*lamports),
            TipStrategy::FetchFloor => {
                let (tip, _) = Self::fetch_tip_floor(client, tip_floor_url).await?;
                Ok(tip)
            }
            TipStrategy::FetchFloorWithCap { min, max } => {
                let (tip, _) = Self::fetch_tip_floor(client, tip_floor_url).await?;
                Ok(tip.clamp(*min, *max))
            }
        }
    }
}

pub struct CompileTipTransactionInput<'a> {
    pub keypair: &'a Keypair,
    pub tip_lamports: u64,
    pub recent_blockhash: Hash,
    pub tip_account: &'a Pubkey,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_tip_account_is_valid() {
        for _ in 0..100 {
            let account = TipHelper::get_random_tip_account();
            assert!(JITO_TIP_ACCOUNTS.contains(&account));
        }
    }

    #[test]
    fn tip_instruction_has_correct_format() {
        let payer = Pubkey::new_unique();
        let ix = TipHelper::create_tip_instruction(&payer, 100_000);
        assert_eq!(ix.program_id, SYSTEM_PROGRAM_ID);
        assert_eq!(ix.accounts.len(), 2);
        assert_eq!(ix.accounts[0].pubkey, payer);
        assert!(ix.accounts[0].is_signer);
        assert!(ix.accounts[0].is_writable);
        assert!(!ix.accounts[1].is_signer);
        assert!(ix.accounts[1].is_writable);
        assert_eq!(ix.data[..4], [2, 0, 0, 0]);
        let lamports = u64::from_le_bytes(ix.data[4..12].try_into().unwrap_or_default());
        assert_eq!(lamports, 100_000);
    }
}
