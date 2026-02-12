use crate::bundler::types::{BundleInstructionSlots, TipMode};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::signature::Keypair;

/// Inputs required to build a Jito bundle.
pub struct BundleBuilderInputs<'a> {
    /// Signing payer used for all compiled transactions.
    pub payer: &'a Keypair,
    /// Fixed instruction slots (max 5).
    pub transactions_instructions: BundleInstructionSlots,
    /// Lookup tables used for v0 message compilation.
    pub lookup_tables: &'a [AddressLookupTableAccount],
    /// Recent blockhash used for all compiled messages.
    pub recent_blockhash: Hash,
    /// Tip amount in lamports.
    pub tip_lamports: u64,
    /// Optional `jitodontfront` account to inject.
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    /// Compute unit limit prepended to each transaction.
    pub compute_unit_limit: u32,
}

/// Mutable builder state used while constructing a bundle.
pub struct BundleBuilder<'a> {
    /// Signing payer used for all compiled transactions.
    pub payer: &'a Keypair,
    /// Fixed instruction slots (max 5).
    pub transactions_instructions: BundleInstructionSlots,
    /// Lookup tables used for v0 message compilation.
    pub lookup_tables: &'a [AddressLookupTableAccount],
    /// Recent blockhash used for all compiled messages.
    pub recent_blockhash: Hash,
    /// Tip amount in lamports.
    pub tip_lamports: u64,
    /// Optional `jitodontfront` account to inject.
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    /// Compute unit limit prepended to each transaction.
    pub compute_unit_limit: u32,
    /// Randomly chosen Jito tip account for this build.
    pub tip_account: Pubkey,
    /// Final tip placement mode selected during build.
    pub tip_mode: TipMode,
}
