use crate::bundler::types::{BundleInstructionSlots, TipMode};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::signature::Keypair;

pub struct BundleBuilderInputs<'a> {
    pub payer: &'a Keypair,
    pub transactions_instructions: BundleInstructionSlots,
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    pub compute_unit_limit: u32,
}

pub struct BundleBuilder<'a> {
    pub payer: &'a Keypair,
    pub transactions_instructions: BundleInstructionSlots,
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    pub compute_unit_limit: u32,
    pub tip_account: Pubkey,
    pub tip_mode: TipMode,
}
