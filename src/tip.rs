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

pub struct TipHelper;

impl TipHelper {
    pub fn get_random_tip_account() -> Pubkey {
        let index = rand::rng().random_range(0..JITO_TIP_ACCOUNTS.len());
        JITO_TIP_ACCOUNTS[index]
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
}
