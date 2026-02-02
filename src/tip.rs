use crate::config::tip_strategy::TipStrategy;
use crate::constants::{
    JITO_TIP_ACCOUNTS, SYSTEM_PROGRAM_ID,
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

        let tip_in_lamports = Self::compute_tip_floor_lamports(tip_data);

        Ok((tip_in_lamports, tip_data.clone()))
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
                Ok(Self::apply_floor_strategy(tip, strategy))
            }
            TipStrategy::FetchFloorWithCap { min, max } => {
                let (tip, _) = Self::fetch_tip_floor(client, tip_floor_url).await?;
                Ok(Self::apply_floor_strategy(
                    tip,
                    &TipStrategy::FetchFloorWithCap { min: *min, max: *max },
                ))
            }
        }
    }

    fn compute_tip_floor_lamports(tip_data: &JitoTipFloorResponse) -> u64 {
        let tip_float = (tip_data.ema_landed_tips_50th_percentile * 1e9).ceil();
        if tip_float.is_sign_negative() || tip_float.is_nan() {
            0u64
        } else {
            tip_float as u64
        }
    }

    fn apply_floor_strategy(tip: u64, strategy: &TipStrategy) -> u64 {
        match strategy {
            TipStrategy::Fixed(lamports) => *lamports,
            TipStrategy::FetchFloor => tip,
            TipStrategy::FetchFloorWithCap { min, max } => tip.clamp(*min, *max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{DEFAULT_TIP_LAMPORTS, MAX_TIP_LAMPORTS};

    fn make_tip_floor(ema_50th: f64) -> JitoTipFloorResponse {
        JitoTipFloorResponse {
            time: "2024-01-01T00:00:00Z".to_string(),
            landed_tips_25th_percentile: 0.0,
            landed_tips_50th_percentile: 0.0,
            landed_tips_75th_percentile: 0.0,
            landed_tips_95th_percentile: 0.0,
            landed_tips_99th_percentile: 0.0,
            ema_landed_tips_50th_percentile: ema_50th,
        }
    }

    #[test]
    fn random_tip_account_is_valid() {
        for _ in 0..100 {
            let account = TipHelper::get_random_tip_account();
            assert!(JITO_TIP_ACCOUNTS.contains(&account));
        }
    }

    #[test]
    fn fetch_floor_does_not_clamp_by_default() {
        let tip_data = make_tip_floor(20.0);
        let tip = TipHelper::compute_tip_floor_lamports(&tip_data);
        assert_eq!(tip, 20_000_000_000);
    }

    #[test]
    fn fetch_floor_negative_or_nan_returns_zero() {
        let negative = make_tip_floor(-0.1);
        let tip = TipHelper::compute_tip_floor_lamports(&negative);
        assert_eq!(tip, 0);

        let nan = make_tip_floor(f64::NAN);
        let tip = TipHelper::compute_tip_floor_lamports(&nan);
        assert_eq!(tip, 0);
    }

    #[test]
    fn fetch_floor_with_cap_applies_min_max() {
        let tip = 20_000_000_000;
        let clamped = TipHelper::apply_floor_strategy(
            tip,
            &TipStrategy::FetchFloorWithCap {
                min: DEFAULT_TIP_LAMPORTS,
                max: MAX_TIP_LAMPORTS,
            },
        );
        assert_eq!(clamped, MAX_TIP_LAMPORTS);

        let small_tip = 50_000;
        let clamped = TipHelper::apply_floor_strategy(
            small_tip,
            &TipStrategy::FetchFloorWithCap {
                min: DEFAULT_TIP_LAMPORTS,
                max: MAX_TIP_LAMPORTS,
            },
        );
        assert_eq!(clamped, DEFAULT_TIP_LAMPORTS);
    }
}
