use crate::bundler::builder::types::BundleBuilder;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::VersionedTransaction;

pub struct Bundle<'a> {
    pub versioned_transaction: Vec<VersionedTransaction>,
    pub payer: &'a Keypair,
    pub transactions_instructions: [Option<Vec<Instruction>>; 5],
    pub lookup_tables: &'a [AddressLookupTableAccount],
    pub recent_blockhash: Hash,
    pub tip_lamports: u64,
    pub jitodontfront_pubkey: Option<&'a Pubkey>,
    pub compute_unit_limit: u32,
    pub tip_account: Pubkey,
    pub last_txn_is_tip: bool,
}

impl<'a> From<BundleBuilder<'a>> for Bundle<'a> {
    fn from(value: BundleBuilder<'a>) -> Self {
        Self {
            versioned_transaction: value.versioned_transaction,
            payer: value.payer,
            transactions_instructions: value.transactions_instructions,
            lookup_tables: value.lookup_tables,
            recent_blockhash: value.recent_blockhash,
            tip_lamports: value.tip_lamports,
            jitodontfront_pubkey: value.jitodontfront_pubkey,
            compute_unit_limit: value.compute_unit_limit,
            tip_account: value.tip_account,
            last_txn_is_tip: value.last_txn_is_tip,
        }
    }
}
