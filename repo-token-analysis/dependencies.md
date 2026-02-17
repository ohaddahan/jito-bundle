This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

## Additional Info

# Directory Structure
```
.config/
  nextest.toml
.github/
  workflows/
    ci.yml
    token-analysis.yml
repo-token-analysis/
  .tokenignore.default
scripts/
  tests.sh
src/
  bundler/
    builder/
      mod.rs
      types.rs
      utils.rs
    bundle/
      mod.rs
      types.rs
    mod.rs
    tests.rs
    types.rs
  client/
    jito_bundler.rs
    mod.rs
    rpc.rs
    send.rs
    simulate.rs
    status.rs
    types.rs
  config/
    confirm_policy.rs
    jito.rs
    mod.rs
    network.rs
    tip_strategy.rs
  analysis.rs
  constants.rs
  error.rs
  lib.rs
  tip.rs
  types.rs
tests/
  build/
    memo_bundle.rs
    mod.rs
  common/
    mod.rs
  offline/
    mod.rs
  send/
    memo_send.rs
    mod.rs
  simulate/
    helius_simulation.rs
    mod.rs
  main.rs
.gitignore
.tokenignore
AGENTS.md
Cargo.toml
CLAUDE.md
FOR_USER.md
README.md
```

# Files

## File: repo-token-analysis/.tokenignore.default
````
# Generated analysis output (prevents circular dependency)
repo-token-analysis/

# Dependencies & build artifacts
node_modules/
target/
dist/
build/
.next/
__pycache__/
*.pyc
vendor/

# Lock files
package-lock.json
yarn.lock
pnpm-lock.yaml
Cargo.lock
Gemfile.lock
poetry.lock
go.sum

# Binary & media files
*.png
*.jpg
*.jpeg
*.gif
*.ico
*.svg
*.woff
*.woff2
*.ttf
*.eot
*.mp3
*.mp4
*.zip
*.tar.gz
*.pdf

# IDE & OS
.idea/
.vscode/
.DS_Store
````

## File: .tokenignore
````
# Generated analysis output (prevents circular dependency)
repo-token-analysis/

# Dependencies & build artifacts
node_modules/
target/
dist/
build/
.next/
__pycache__/
*.pyc
vendor/

# Lock files
package-lock.json
yarn.lock
pnpm-lock.yaml
Cargo.lock
Gemfile.lock
poetry.lock
go.sum

# Binary & media files
*.png
*.jpg
*.jpeg
*.gif
*.ico
*.svg
*.woff
*.woff2
*.ttf
*.eot
*.mp3
*.mp4
*.zip
*.tar.gz
*.pdf

# IDE & OS
.idea/
.vscode/
.DS_Store
````

## File: .config/nextest.toml
````toml
[profile.default]
success-output = "immediate"
failure-output = "immediate"
status-level = "pass"
final-status-level = "all"
slow-timeout = { period = "60s", terminate-after = 3 }
````

## File: .github/workflows/ci.yml
````yaml
name: CI

on:
  push:
    branches: ["**"]
  pull_request:

jobs:
  checks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cargo Fmt
        run: cargo fmt --all -- --check

      - name: Cargo Clippy
        run: cargo clippy --all-targets

      - name: Cargo Test
        run: cargo test
````

## File: .github/workflows/token-analysis.yml
````yaml
name: Token Analysis
on: [push]

jobs:
  analyze:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      - uses: ohaddahan/repo-token-analysis@v1.0.1
        with:
          threshold_percent: 75
          top_n_offenders: 10
          history_max_entries: 100
````

## File: scripts/tests.sh
````bash
#!/bin/bash
cargo nextest run --features live-tests --test main --run-ignored all
````

## File: src/bundler/builder/mod.rs
````rust
pub mod types;
pub mod utils;
````

## File: src/bundler/builder/types.rs
````rust
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
````

## File: src/bundler/builder/utils.rs
````rust
use crate::JitoError;
use crate::analysis::TransactionAnalysis;
use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use crate::bundler::bundle::types::BuiltBundle;
use crate::bundler::types::{
    BundleInstructionSlots, BundleSlotView, TipMode, empty_instruction_slots,
};
use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use crate::tip::TipHelper;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::message::{VersionedMessage, v0};
use solana_sdk::signature::Signer;
use solana_sdk::transaction::VersionedTransaction;

impl BundleSlotView for BundleBuilder<'_> {
    /// Returns the mutable builder's current instruction slots view.
    fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.transactions_instructions
    }
}

impl<'a> BundleBuilder<'a> {
    // --- Construction ---
    /// Creates a builder from validated build inputs.
    fn new(inputs: BundleBuilderInputs<'a>) -> Self {
        let BundleBuilderInputs {
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
        } = inputs;
        let tip_account = TipHelper::get_random_tip_account();
        Self {
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey,
            compute_unit_limit,
            tip_account,
            tip_mode: TipMode::InlineLastTx,
        }
    }

    // --- Build Pipeline ---
    /// Builds a final `BuiltBundle` from fixed instruction slots.
    ///
    /// Jito hard-limits bundles to `MAX_BUNDLE_TRANSACTIONS` (5), so this builder
    /// chooses tip placement based on remaining transaction capacity.
    ///
    /// Build flow:
    /// 1. Compact sparse slots while preserving transaction order.
    /// 2. Optionally apply `jitodontfront` account rewriting.
    /// 3. Insert tip as separate tx when count < 5, or inline when count == 5.
    /// 4. Validate tip account is not in LUTs for inline mode.
    /// 5. Compile, sign, and size-check each transaction.
    ///
    /// Returns `JitoError` for invalid bundle size, compile/sign failures,
    /// oversized transactions, and invalid LUT tip-account usage.
    pub fn build(inputs: BundleBuilderInputs<'a>) -> Result<BuiltBundle, JitoError> {
        let mut builder = Self::new(inputs);
        builder.compact_transactions();
        let count = builder.populated_count();
        if count == 0 {
            return Err(JitoError::InvalidBundleSize { count: 0 });
        }

        if let Some(jitodontfront_pubkey) = builder.jitodontfront_pubkey {
            builder.apply_jitodont_front(jitodontfront_pubkey);
        }

        if count < MAX_BUNDLE_TRANSACTIONS {
            builder.append_tip_transaction()?;
            builder.tip_mode = TipMode::SeparateTx;
        } else {
            builder.append_tip_instruction();
            builder.tip_mode = TipMode::InlineLastTx;
        }

        if matches!(builder.tip_mode, TipMode::InlineLastTx) {
            Self::validate_tip_not_in_luts(&builder.tip_account, builder.lookup_tables)?;
        }

        let total = builder.populated_count();
        let mut transactions = Vec::with_capacity(total);
        for (compiled_index, ixs) in builder
            .transactions_instructions
            .iter()
            .flatten()
            .enumerate()
        {
            let txn = builder.build_versioned_transaction(compiled_index, total, ixs)?;
            transactions.push(txn);
        }

        Ok(BuiltBundle::new(
            transactions,
            builder.tip_account,
            builder.tip_lamports,
            builder.tip_mode,
            builder.transactions_instructions,
        ))
    }

    /// Compacts sparse slots while preserving transaction order.
    fn compact_transactions(&mut self) {
        let mut new_slots = empty_instruction_slots();
        let mut idx = 0;
        for slot in &mut self.transactions_instructions {
            if let Some(ixs) = slot.take()
                && idx < new_slots.len()
            {
                new_slots[idx] = Some(ixs);
                idx += 1;
            }
        }
        self.transactions_instructions = new_slots;
    }

    /// Appends the tip as a dedicated transaction when the bundle has room (< 5 txs).
    fn append_tip_transaction(&mut self) -> Result<(), JitoError> {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        let first_none = self
            .transactions_instructions
            .iter()
            .position(|slot| slot.is_none())
            .ok_or(JitoError::InvalidBundleSize {
                count: MAX_BUNDLE_TRANSACTIONS,
            })?;
        self.transactions_instructions[first_none] = Some(vec![tip_ix]);
        Ok(())
    }

    /// Appends the tip instruction to the last populated transaction when already at 5 txs.
    fn append_tip_instruction(&mut self) {
        let tip_ix = TipHelper::create_tip_instruction_to(
            &self.payer.pubkey(),
            &self.tip_account,
            self.tip_lamports,
        );
        if let Some(last_idx) = self.last_populated_index()
            && let Some(ixs) = &mut self.transactions_instructions[last_idx]
        {
            ixs.push(tip_ix);
        }
    }

    /// Rewrites `jitodontfront` account usage so it appears only in the first transaction.
    ///
    /// The expected pubkey prefix is `jitodontfront`, while the suffix can vary,
    /// so matching uses string prefix rather than exact full-string equality.
    fn apply_jitodont_front(&mut self, jitodontfront_pubkey: &Pubkey) {
        for ixs in self.transactions_instructions.iter_mut().flatten() {
            for instruction in ixs.iter_mut() {
                instruction
                    .accounts
                    .retain(|acct| !acct.pubkey.to_string().starts_with("jitodontfront"));
            }
        }
        if let Some(Some(ixs)) = self.transactions_instructions.first_mut()
            && let Some(instruction) = ixs.first_mut()
        {
            instruction
                .accounts
                .push(AccountMeta::new_readonly(*jitodontfront_pubkey, false));
        }
    }

    /// Compiles and signs one versioned transaction from instruction list.
    fn build_versioned_transaction(
        &self,
        index: usize,
        total: usize,
        tx_instructions: &[Instruction],
    ) -> Result<VersionedTransaction, JitoError> {
        let compute_budget =
            ComputeBudgetInstruction::set_compute_unit_limit(self.compute_unit_limit);
        let mut instructions = vec![compute_budget];
        instructions.extend_from_slice(tx_instructions);

        let lut: &[AddressLookupTableAccount] =
            if index == total - 1 && matches!(self.tip_mode, TipMode::SeparateTx) {
                &[]
            } else {
                self.lookup_tables
            };

        let message = v0::Message::try_compile(
            &self.payer.pubkey(),
            &instructions,
            lut,
            self.recent_blockhash,
        )
        .map_err(|e| {
            TransactionAnalysis::log_accounts_not_in_luts(
                &instructions,
                lut,
                &format!("TX: {index} COMPILE_FAIL"),
            );
            JitoError::MessageCompileFailed {
                index,
                reason: e.to_string(),
            }
        })?;
        let txn = VersionedTransaction::try_new(VersionedMessage::V0(message), &[self.payer])
            .map_err(|e| JitoError::TransactionCreationFailed {
                index,
                reason: e.to_string(),
            })?;
        let size_info = TransactionAnalysis::analyze_transaction_size(&txn);
        if size_info.is_oversized {
            return Err(JitoError::TransactionOversized {
                index,
                size: size_info.size,
                max: size_info.max_size,
            });
        }
        Ok(txn)
    }

    /// Ensures the chosen tip account is not present in provided LUTs.
    ///
    /// If the tip account appears in a LUT for inline-tip mode, Jito bundle execution
    /// will fail, so this is validated pre-send.
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
````

## File: src/bundler/bundle/mod.rs
````rust
pub mod types;
````

## File: src/bundler/bundle/types.rs
````rust
use crate::bundler::types::{BundleInstructionSlots, BundleSlotView, TipMode};
use solana_pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;

/// Final bundle artifact ready for simulation and submission.
pub struct BuiltBundle {
    /// Fully compiled and signed versioned transactions.
    pub transactions: Vec<VersionedTransaction>,
    /// Chosen Jito tip account.
    pub tip_account: Pubkey,
    /// Effective tip amount in lamports.
    pub tip_lamports: u64,
    /// How the tip instruction was inserted.
    pub tip_mode: TipMode,
    /// Post-compaction instruction slots used to build the transactions.
    pub instruction_slots: BundleInstructionSlots,
}

impl BuiltBundle {
    // --- Construction ---
    /// Creates a new built bundle value.
    pub fn new(
        transactions: Vec<VersionedTransaction>,
        tip_account: Pubkey,
        tip_lamports: u64,
        tip_mode: TipMode,
        instruction_slots: BundleInstructionSlots,
    ) -> Self {
        Self {
            transactions,
            tip_account,
            tip_lamports,
            tip_mode,
            instruction_slots,
        }
    }

    // --- Accessors ---
    /// Returns the post-compaction instruction slots.
    pub fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.instruction_slots
    }
}

impl BundleSlotView for BuiltBundle {
    /// Returns the post-compaction instruction slots.
    fn instruction_slots(&self) -> &BundleInstructionSlots {
        &self.instruction_slots
    }

    /// Counts non-empty instruction slots.
    fn populated_count(&self) -> usize {
        self.instruction_slots.iter().flatten().count()
    }

    /// Returns the index of the last non-empty slot.
    fn last_populated_index(&self) -> Option<usize> {
        self.instruction_slots
            .iter()
            .rposition(|slot| slot.is_some())
    }
}
````

## File: src/bundler/mod.rs
````rust
pub mod builder;
pub mod bundle;
pub mod tests;
pub mod types;
````

## File: src/bundler/tests.rs
````rust
#[cfg(test)]
mod bundle_builder_tests {
    use crate::JitoError;
    use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
    use crate::bundler::bundle::types::BuiltBundle;
    use crate::bundler::types::{BundleSlotView, TipMode};
    use crate::constants::{JITO_TIP_ACCOUNTS, SOLANA_MAX_TX_SIZE, SYSTEM_PROGRAM_ID};
    use solana_instruction::{AccountMeta, Instruction};
    use solana_pubkey::Pubkey;
    use solana_sdk::address_lookup_table::AddressLookupTableAccount;
    use solana_sdk::hash::Hash;
    use solana_sdk::signature::{Keypair, Signer};

    /// Unwraps successful build results with test-friendly failure output.
    fn assert_build_ok(result: Result<BuiltBundle, JitoError>) -> BuiltBundle {
        match result {
            Ok(bundle) => bundle,
            Err(e) => {
                assert!(e.to_string().is_empty(), "build failed: {e}");
                std::process::abort();
            }
        }
    }

    /// Returns instructions stored in a specific post-build slot.
    fn get_slot(bundle: &BuiltBundle, index: usize) -> &Vec<Instruction> {
        let slot = &bundle.instruction_slots()[index];
        if let Some(ixs) = slot {
            ixs
        } else {
            assert!(slot.is_some(), "expected Some at slot {index}, got None");
            std::process::abort();
        }
    }

    /// Returns true when instruction matches the expected tip transfer format.
    fn is_tip_instruction(ix: &Instruction, tip_account: &Pubkey, tip_lamports: u64) -> bool {
        let lamports = tip_lamports.to_le_bytes();
        ix.program_id == SYSTEM_PROGRAM_ID
            && ix.accounts.len() >= 2
            && ix.accounts[1].pubkey == *tip_account
            && ix
                .data
                .get(0..4)
                .is_some_and(|op| op == [2, 0, 0, 0].as_slice())
            && ix
                .data
                .get(4..12)
                .is_some_and(|bytes| bytes == lamports.as_slice())
    }

    /// Counts tip instructions across all populated slots.
    fn count_tip_instructions(bundle: &BuiltBundle) -> usize {
        bundle
            .instruction_slots()
            .iter()
            .flatten()
            .flat_map(|ixs| ixs.iter())
            .filter(|ix| is_tip_instruction(ix, &bundle.tip_account, bundle.tip_lamports))
            .count()
    }

    struct TestBundleParams<'a> {
        payer: &'a Keypair,
        tx_count: usize,
        blockhash: Hash,
        luts: &'a [AddressLookupTableAccount],
        jdf: Option<&'a Pubkey>,
        tip: u64,
    }

    /// Creates a simple system instruction used by test transactions.
    fn make_noop_instruction(payer: &Pubkey) -> Instruction {
        let mut data = vec![2, 0, 0, 0];
        data.extend_from_slice(&0u64.to_le_bytes());
        Instruction {
            program_id: SYSTEM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new(*payer, false),
            ],
            data,
        }
    }

    /// Creates a custom single-account instruction for tests.
    fn make_custom_instruction(payer: &Pubkey, program_id: Pubkey) -> Instruction {
        Instruction {
            program_id,
            accounts: vec![AccountMeta::new(*payer, true)],
            data: vec![1, 2, 3],
        }
    }

    /// Builds standard test inputs with configurable tx count and options.
    fn make_bundle_inputs(params: TestBundleParams<'_>) -> BundleBuilderInputs<'_> {
        let TestBundleParams {
            payer,
            tx_count,
            blockhash,
            luts,
            jdf,
            tip,
        } = params;
        let pubkey = payer.pubkey();
        let mut slots = [None, None, None, None, None];
        for slot in slots.iter_mut().take(tx_count) {
            *slot = Some(vec![make_noop_instruction(&pubkey)]);
        }
        BundleBuilderInputs {
            payer,
            transactions_instructions: slots,
            lookup_tables: luts,
            recent_blockhash: blockhash,
            tip_lamports: tip,
            jitodontfront_pubkey: jdf,
            compute_unit_limit: 200_000,
        }
    }

    /// Verifies jito-dont-front account gets injected into first instruction.
    #[test]
    fn jitodontfront_added_to_first_instruction() {
        let payer = Keypair::new();
        let jdf = Pubkey::new_unique();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: Some(&jdf),
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let first_tx_instructions = get_slot(&bundle, 0);
        let first_ix = &first_tx_instructions[0];
        let last_account = &first_ix.accounts[first_ix.accounts.len() - 1];
        assert_eq!(last_account.pubkey, jdf);
        assert!(!last_account.is_signer);
        assert!(!last_account.is_writable);
    }

    /// Verifies no extra account is added when JDF is disabled.
    #[test]
    fn jitodontfront_none_means_no_extra_account() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let first_ix = &get_slot(&bundle, 0)[0];
        assert_eq!(first_ix.accounts.len(), 2);
    }

    /// Verifies one input transaction produces one tx plus separate tip tx.
    #[test]
    fn one_tx_produces_two_versioned_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert_eq!(bundle.transactions.len(), 2);
        assert!(matches!(bundle.tip_mode, TipMode::SeparateTx));
    }

    /// Verifies four input transactions produce five total with separate tip.
    #[test]
    fn four_txs_produce_five_versioned_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 4,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert_eq!(bundle.transactions.len(), 5);
        assert!(matches!(bundle.tip_mode, TipMode::SeparateTx));
    }

    /// Verifies five input transactions keep tip inline in final transaction.
    #[test]
    fn five_txs_produce_five_versioned_txs_tip_inline() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert_eq!(bundle.transactions.len(), 5);
        assert!(matches!(bundle.tip_mode, TipMode::InlineLastTx));
    }

    /// Verifies empty bundles are rejected with size error.
    #[test]
    fn zero_transactions_returns_invalid_bundle_size() {
        let payer = Keypair::new();
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [None, None, None, None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        };
        let result = BundleBuilder::build(inputs);
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::InvalidBundleSize { count: 0 })),
            "expected InvalidBundleSize {{ count: 0 }}, got {err:?}"
        );
    }

    /// Verifies every bundle size from one through five builds successfully.
    #[test]
    fn one_to_five_transactions_all_succeed() {
        for tx_count in 1..=5 {
            let payer = Keypair::new();
            let inputs = make_bundle_inputs(TestBundleParams {
                payer: &payer,
                tx_count,
                blockhash: Hash::default(),
                luts: &[],
                jdf: None,
                tip: 100_000,
            });
            let result = BundleBuilder::build(inputs);
            assert!(result.is_ok(), "expected Ok for {tx_count} transactions");
        }
    }

    /// Verifies built transactions respect Solana max serialized size.
    #[test]
    fn compiled_transactions_within_size_limit() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 2,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(
            !bundle.transactions.is_empty(),
            "expected at least one compiled transaction"
        );
        for (i, tx) in bundle.transactions.iter().enumerate() {
            let serialized_result = bincode::serialize(tx);
            assert!(
                serialized_result.is_ok(),
                "failed to serialize transaction {i}"
            );
            let serialized = serialized_result.unwrap_or_default();
            assert!(
                !serialized.is_empty(),
                "transaction {i} serialized to empty bytes"
            );
            assert!(
                serialized.len() <= SOLANA_MAX_TX_SIZE,
                "transaction {i} is {size} bytes, exceeds {SOLANA_MAX_TX_SIZE}",
                size = serialized.len()
            );
        }
    }

    /// Verifies oversized instruction payloads fail at build time.
    #[test]
    fn oversized_transaction_returns_error() {
        let payer = Keypair::new();
        let pubkey = payer.pubkey();
        let big_data = vec![0u8; 1500];
        let big_ix = Instruction {
            program_id: SYSTEM_PROGRAM_ID,
            accounts: vec![AccountMeta::new(pubkey, true)],
            data: big_data,
        };
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [Some(vec![big_ix]), None, None, None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        };
        let result = BundleBuilder::build(inputs);
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::TransactionOversized { .. })),
            "expected TransactionOversized, got {err:?}"
        );
    }

    /// Verifies tip is emitted as a separate transaction when bundle is not full.
    #[test]
    fn tip_separate_tx_when_under_five() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 2,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(matches!(bundle.tip_mode, TipMode::SeparateTx));
        assert_eq!(bundle.populated_count(), 3);
        assert_eq!(count_tip_instructions(&bundle), 1);
        for idx in 0..2 {
            let has_tip_in_original_tx = get_slot(&bundle, idx)
                .iter()
                .any(|ix| is_tip_instruction(ix, &bundle.tip_account, bundle.tip_lamports));
            assert!(
                !has_tip_in_original_tx,
                "did not expect tip instruction inline in slot {idx}"
            );
        }
        let tip_tx = get_slot(&bundle, 2);
        assert_eq!(tip_tx.len(), 1);
        assert!(is_tip_instruction(
            &tip_tx[0],
            &bundle.tip_account,
            bundle.tip_lamports
        ));
    }

    /// Verifies tip is appended inline when bundle already has five txs.
    #[test]
    fn tip_inline_when_five_txs() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(matches!(bundle.tip_mode, TipMode::InlineLastTx));
        assert_eq!(bundle.transactions.len(), 5);
        assert_eq!(bundle.populated_count(), 5);
        assert_eq!(count_tip_instructions(&bundle), 1);
        for idx in 0..4 {
            let has_tip_in_non_last_tx = get_slot(&bundle, idx)
                .iter()
                .any(|ix| is_tip_instruction(ix, &bundle.tip_account, bundle.tip_lamports));
            assert!(
                !has_tip_in_non_last_tx,
                "did not expect tip instruction in non-last slot {idx}"
            );
        }
        let last_tx = get_slot(&bundle, 4);
        let last_ix = &last_tx[last_tx.len() - 1];
        assert!(is_tip_instruction(
            last_ix,
            &bundle.tip_account,
            bundle.tip_lamports
        ));
    }

    /// Verifies selected tip account is always from known Jito tip set.
    #[test]
    fn tip_account_is_valid_jito_account() {
        let payer = Keypair::new();
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(
            JITO_TIP_ACCOUNTS.contains(&bundle.tip_account),
            "tip_account {} not in JITO_TIP_ACCOUNTS",
            bundle.tip_account
        );
    }

    /// Verifies encoded transfer lamports in tip instruction are correct.
    #[test]
    fn tip_lamports_encoded_correctly() {
        let payer = Keypair::new();
        let tip_amount: u64 = 500_000;
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 1,
            blockhash: Hash::default(),
            luts: &[],
            jdf: None,
            tip: tip_amount,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let last_idx = bundle.last_populated_index();
        assert!(last_idx.is_some(), "no populated slots found");
        let tip_tx = get_slot(&bundle, last_idx.unwrap_or(0));
        let tip_ix = if matches!(bundle.tip_mode, TipMode::SeparateTx) {
            &tip_tx[0]
        } else {
            &tip_tx[tip_tx.len() - 1]
        };
        let encoded_lamports = &tip_ix.data[4..12];
        assert_eq!(encoded_lamports, &tip_amount.to_le_bytes());
    }

    /// Verifies inline tip mode rejects LUTs containing tip accounts.
    #[test]
    fn tip_account_in_lut_rejected() {
        let payer = Keypair::new();
        let lut_key = Pubkey::new_unique();
        let lut = AddressLookupTableAccount {
            key: lut_key,
            addresses: JITO_TIP_ACCOUNTS.to_vec(),
        };
        let luts = [lut];
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &luts,
            jdf: None,
            tip: 100_000,
        });
        let result = BundleBuilder::build(inputs);
        assert!(result.is_err());
        let err = result.err();
        assert!(
            matches!(err, Some(JitoError::TipAccountInLut { .. })),
            "expected TipAccountInLut, got {err:?}"
        );
    }

    /// Verifies inline mode succeeds when LUTs do not contain tip account.
    #[test]
    fn tip_account_not_in_lut_is_accepted() {
        let payer = Keypair::new();
        let lut = AddressLookupTableAccount {
            key: Pubkey::new_unique(),
            addresses: vec![Pubkey::new_unique(), Pubkey::new_unique()],
        };
        let luts = [lut];
        let inputs = make_bundle_inputs(TestBundleParams {
            payer: &payer,
            tx_count: 5,
            blockhash: Hash::default(),
            luts: &luts,
            jdf: None,
            tip: 100_000,
        });
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        assert!(matches!(bundle.tip_mode, TipMode::InlineLastTx));
    }

    /// Verifies compaction preserves last slot used for inline tip append.
    #[test]
    fn tip_appended_to_last_populated_slot_even_with_gaps() {
        let payer = Keypair::new();
        let pubkey = payer.pubkey();
        let ix1 = make_custom_instruction(&pubkey, Pubkey::new_unique());
        let ix2 = make_custom_instruction(&pubkey, Pubkey::new_unique());
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [Some(vec![ix1]), None, Some(vec![ix2]), None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: None,
            compute_unit_limit: 200_000,
        };
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let last_idx = bundle.last_populated_index();
        assert!(last_idx.is_some(), "no populated slots found");
        let last_tx = get_slot(&bundle, last_idx.unwrap_or(0));
        let last_ix = &last_tx[last_tx.len() - 1];
        assert_eq!(
            last_ix.program_id, SYSTEM_PROGRAM_ID,
            "expected tip to be appended to the last populated slot"
        );
    }

    /// Verifies JDF account is not duplicated if already present.
    #[test]
    fn jitodontfront_not_duplicated_if_already_present() {
        let payer = Keypair::new();
        let jdf = Pubkey::from_str_const("jitodontfront111111111111111111111111111111");
        let mut ix = make_custom_instruction(&payer.pubkey(), Pubkey::new_unique());
        ix.accounts.push(AccountMeta::new_readonly(jdf, false));
        let inputs = BundleBuilderInputs {
            payer: &payer,
            transactions_instructions: [Some(vec![ix]), None, None, None, None],
            lookup_tables: &[],
            recent_blockhash: Hash::default(),
            tip_lamports: 100_000,
            jitodontfront_pubkey: Some(&jdf),
            compute_unit_limit: 200_000,
        };
        let bundle = assert_build_ok(BundleBuilder::build(inputs));
        let first_ix = &get_slot(&bundle, 0)[0];
        let count = first_ix
            .accounts
            .iter()
            .filter(|acct| acct.pubkey == jdf)
            .count();
        assert_eq!(
            count, 1,
            "expected jitodontfront account to appear exactly once"
        );
    }
}
````

## File: src/bundler/types.rs
````rust
use crate::constants::MAX_BUNDLE_TRANSACTIONS;
use solana_instruction::Instruction;

/// Fixed-size instruction slots aligned with Jito's max bundle size.
pub type BundleInstructionSlots = [Option<Vec<Instruction>>; MAX_BUNDLE_TRANSACTIONS];

/// Indicates how the tip was placed in the built bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipMode {
    /// Tip is appended as a dedicated final transaction.
    SeparateTx,
    /// Tip is appended to the last existing transaction.
    InlineLastTx,
}

/// Shared view over fixed instruction slots.
pub trait BundleSlotView {
    /// Returns the underlying fixed instruction slots.
    fn instruction_slots(&self) -> &BundleInstructionSlots;

    /// Counts non-empty instruction slots.
    fn populated_count(&self) -> usize {
        self.instruction_slots()
            .iter()
            .filter(|slot| slot.is_some())
            .count()
    }

    /// Returns the index of the last non-empty slot.
    fn last_populated_index(&self) -> Option<usize> {
        self.instruction_slots()
            .iter()
            .rposition(|slot| slot.is_some())
    }
}

/// Creates an empty fixed-size slot array.
pub fn empty_instruction_slots() -> BundleInstructionSlots {
    std::array::from_fn(|_| None)
}
````

## File: src/client/jito_bundler.rs
````rust
use crate::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use crate::bundler::bundle::types::BuiltBundle;
use crate::bundler::types::BundleInstructionSlots;
use crate::config::jito::JitoConfig;
use crate::error::JitoError;
use crate::tip::TipHelper;
use crate::types::{BundleResult, BundleStatus, SimulateBundleValue};
use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::signer::keypair::Keypair;
use std::time::Duration;

/// High-level facade for building, simulating, and sending Jito bundles.
pub struct JitoBundler {
    /// Runtime bundler configuration.
    pub config: JitoConfig,
    /// Shared HTTP client for Jito/Helius requests.
    pub http_client: Client,
    /// Solana RPC client for chain reads and simulations.
    pub rpc_client: RpcClient,
}

impl JitoBundler {
    // --- Construction ---
    /// Creates a bundler client from configuration.
    pub fn new(config: JitoConfig) -> Result<Self, JitoError> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| JitoError::Network {
                reason: format!("failed to create HTTP client: {e}"),
            })?;
        let rpc_client = RpcClient::new(config.rpc_url.clone());
        Ok(Self {
            config,
            http_client,
            rpc_client,
        })
    }

    // --- HTTP Helpers ---
    /// Builds a JSON POST request with Jito auth headers when configured.
    pub fn jito_post(&self, url: &str) -> reqwest::RequestBuilder {
        let full_url = if let Some(uuid) = &self.config.uuid
            && !self.config.network.is_custom()
        {
            let separator = if url.contains('?') { "&" } else { "?" };
            format!("{url}{separator}uuid={uuid}")
        } else {
            url.to_string()
        };
        let mut builder = self
            .http_client
            .post(full_url)
            .header("Content-Type", "application/json");
        if let Some(uuid) = &self.config.uuid {
            builder = builder.header("x-jito-auth", uuid.as_str());
        }
        builder
    }

    // --- Bundle Lifecycle ---
    /// Resolves the tip amount according to configured strategy.
    pub async fn fetch_tip(&self) -> Result<u64, JitoError> {
        let tip_floor_url = self.config.network.tip_floor_url();
        TipHelper::resolve_tip(&self.http_client, tip_floor_url, &self.config.tip_strategy).await
    }

    /// Builds a signed `BuiltBundle` from fixed instruction slots.
    ///
    /// Steps:
    /// 1. Resolve the tip amount using configured `TipStrategy`.
    /// 2. Fetch a fresh recent blockhash from RPC.
    /// 3. Build and sign transactions via `BundleBuilder::build`.
    ///
    /// Returns `JitoError` when tip resolution, blockhash fetch, or compilation fails.
    pub async fn build_bundle(
        &self,
        input: BuildBundleOptions<'_>,
    ) -> Result<BuiltBundle, JitoError> {
        let BuildBundleOptions {
            payer,
            transactions_instructions,
            lookup_tables,
        } = input;
        let tip_lamports = self.fetch_tip().await?;
        let recent_blockhash = self.rpc_client.get_latest_blockhash().await.map_err(|e| {
            JitoError::GetLatestBlockhash {
                reason: e.to_string(),
            }
        })?;
        let bundle = BundleBuilder::build(BundleBuilderInputs {
            payer,
            transactions_instructions,
            lookup_tables,
            recent_blockhash,
            tip_lamports,
            jitodontfront_pubkey: self.config.jitodontfront_pubkey.as_ref(),
            compute_unit_limit: self.config.compute_unit_limit,
        })?;
        Ok(bundle)
    }

    /// Simulates the built bundle against configured Helius RPC.
    pub async fn simulate_helius(
        &self,
        bundle: &BuiltBundle,
    ) -> Result<SimulateBundleValue, JitoError> {
        let helius_url =
            self.config
                .helius_rpc_url
                .as_deref()
                .ok_or_else(|| JitoError::Network {
                    reason: "helius_rpc_url not configured".to_string(),
                })?;
        self.simulate_bundle_helius(bundle, helius_url).await
    }

    /// Optionally simulates, sends, and confirms the bundle on-chain.
    ///
    /// Behavior:
    /// 1. If `helius_rpc_url` is configured, run atomic simulation first.
    /// 2. Submit bundle with endpoint retry.
    /// 3. Poll on-chain signatures until landed/failed/timeout.
    pub async fn send_and_confirm(&self, bundle: &BuiltBundle) -> Result<BundleResult, JitoError> {
        if let Some(helius_url) = &self.config.helius_rpc_url
            && let Err(e) = self.simulate_bundle_helius(bundle, helius_url).await
        {
            tracing::warn!("Helius simulation failed: {e}");
            return Err(e);
        }
        let result = self.send_bundle(bundle).await?;
        tracing::info!(
            "bundle submitted: bundle_id={}, signatures={:?}, explorer={}",
            result.bundle_id,
            result.signatures,
            result.explorer_url
        );
        let status = self.wait_for_landing_on_chain(&result.signatures).await;
        Self::interpret_landing_status(status, self.config.confirm_policy.max_attempts)?;
        Ok(result)
    }

    /// Maps raw landing status into final client-facing success/error.
    fn interpret_landing_status(
        status: Result<BundleStatus, JitoError>,
        max_attempts: u32,
    ) -> Result<(), JitoError> {
        match status {
            Ok(BundleStatus::Landed { .. }) => Ok(()),
            Ok(BundleStatus::Failed { error }) => {
                let reason = error.unwrap_or_else(|| "unknown error".to_string());
                Err(JitoError::OnChainFailure { reason })
            }
            Ok(_) => Err(JitoError::ConfirmationTimeout {
                attempts: max_attempts,
            }),
            Err(e) => Err(e),
        }
    }
}

/// Input arguments for `JitoBundler::build_bundle`.
pub struct BuildBundleOptions<'a> {
    /// Signing payer used for all transactions.
    pub payer: &'a Keypair,
    /// Fixed instruction slots (max 5) used for bundle creation.
    pub transactions_instructions: BundleInstructionSlots,
    /// Lookup tables used when compiling transactions.
    pub lookup_tables: &'a [AddressLookupTableAccount],
}
````

## File: src/client/mod.rs
````rust
pub mod jito_bundler;
pub mod rpc;
pub mod send;
pub mod simulate;
pub mod status;
mod types;
````

## File: src/client/rpc.rs
````rust
use crate::client::jito_bundler::JitoBundler;
use crate::error::JitoError;
use crate::types::{JsonRpcRequest, JsonRpcResponse};
use base64::Engine;
use reqwest::{RequestBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use solana_sdk::transaction::VersionedTransaction;

impl JitoBundler {
    // --- Shared RPC Utilities ---
    /// Sends a JSON-RPC POST request and returns status + raw response body.
    pub async fn send_json_rpc_request<Req: Serialize>(
        &self,
        request_builder: RequestBuilder,
        request: &JsonRpcRequest<Req>,
        context: &str,
    ) -> Result<(StatusCode, String), JitoError> {
        let response =
            request_builder
                .json(request)
                .send()
                .await
                .map_err(|e| JitoError::Network {
                    reason: format!("{context}: {e}"),
                })?;
        let status = response.status();
        let response_text = response.text().await.map_err(|e| JitoError::Network {
            reason: format!("{context} response read failed: {e}"),
        })?;
        Ok((status, response_text))
    }

    /// Parses a JSON-RPC response and extracts `result` or mapped error.
    pub fn parse_json_rpc_result<Res: DeserializeOwned>(
        response_text: &str,
        context: &str,
        missing_result_message: &str,
    ) -> Result<Res, JitoError> {
        let parsed: JsonRpcResponse<Res> =
            serde_json::from_str(response_text).map_err(|e| JitoError::Serialization {
                reason: format!("{context} parse failed: {e}, body: {response_text}"),
            })?;

        if let Some(error) = parsed.error {
            return Err(JitoError::JsonRpc {
                code: error.code,
                message: error.message,
            });
        }

        parsed.result.ok_or_else(|| JitoError::JsonRpc {
            code: -1,
            message: missing_result_message.to_string(),
        })
    }

    /// Serializes and base64-encodes all versioned transactions.
    pub fn encode_transactions_base64(
        transactions: &[VersionedTransaction],
    ) -> Result<Vec<String>, JitoError> {
        transactions
            .iter()
            .map(|tx| {
                let serialized = bincode::serialize(tx).map_err(|e| JitoError::Serialization {
                    reason: e.to_string(),
                })?;
                Ok(base64::engine::general_purpose::STANDARD.encode(serialized))
            })
            .collect()
    }

    /// Returns the first transaction signature as base58, or a typed error when absent.
    pub fn first_signature_base58(
        tx: &VersionedTransaction,
        tx_index: usize,
        context: &str,
    ) -> Result<String, JitoError> {
        let signature = tx
            .signatures
            .first()
            .ok_or_else(|| JitoError::InvalidSignature {
                reason: format!("{context}: transaction {tx_index} has no signatures"),
            })?;
        Ok(bs58::encode(signature).into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::message::{Message, VersionedMessage};
    use solana_sdk::signature::Signature;

    /// Creates a minimal versioned transaction with caller-provided signatures.
    fn make_tx(signatures: Vec<Signature>) -> VersionedTransaction {
        let message = Message::new(&[], None);
        VersionedTransaction {
            signatures,
            message: VersionedMessage::Legacy(message),
        }
    }

    #[test]
    fn first_signature_base58_returns_error_when_missing() {
        let tx = make_tx(vec![]);
        let result = JitoBundler::first_signature_base58(&tx, 2, "send_bundle");
        assert!(
            matches!(result, Err(JitoError::InvalidSignature { .. })),
            "expected InvalidSignature, got {result:?}"
        );
    }

    #[test]
    fn first_signature_base58_returns_encoded_signature() {
        let signature = Signature::default();
        let tx = make_tx(vec![signature]);
        let result = JitoBundler::first_signature_base58(&tx, 0, "simulate");
        assert!(result.is_ok(), "expected Ok signature, got {result:?}");
        let encoded = result.unwrap_or_default();
        assert_eq!(encoded, bs58::encode(signature).into_string());
    }
}
````

## File: src/client/send.rs
````rust
use crate::bundler::bundle::types::BuiltBundle;
use crate::client::jito_bundler::JitoBundler;
use crate::constants::JITO_EXPLORER_URL;
use crate::error::JitoError;
use crate::types::{BundleResult, JsonRpcRequest};
use serde::Serialize;
use solana_sdk::transaction::VersionedTransaction;

impl JitoBundler {
    // --- Bundle Sending ---
    /// Builds the Jito explorer URL for a bundle id.
    pub fn get_jito_explorer_url(bundle_id: &str) -> String {
        format!("{JITO_EXPLORER_URL}/{bundle_id}")
    }

    /// Extracts base58 signatures from compiled transactions.
    pub fn extract_signatures(
        transactions: &[VersionedTransaction],
    ) -> Result<Vec<String>, JitoError> {
        transactions
            .iter()
            .enumerate()
            .map(|(i, tx)| Self::first_signature_base58(tx, i, "send_bundle"))
            .collect()
    }

    /// Sends a built bundle with endpoint retry fallback.
    pub async fn send_bundle(&self, bundle: &BuiltBundle) -> Result<BundleResult, JitoError> {
        let encoded_txs = Self::encode_transactions_base64(&bundle.transactions)?;
        let signatures = Self::extract_signatures(&bundle.transactions)?;
        let endpoints = self.config.network.send_endpoints();
        let mut last_error = String::from("no endpoints available");
        for endpoint in &endpoints {
            match self.send_bundle_to_endpoint(endpoint, &encoded_txs).await {
                Ok(bundle_id) => {
                    let explorer_url = Self::get_jito_explorer_url(&bundle_id);
                    return Ok(BundleResult {
                        bundle_id,
                        signatures,
                        explorer_url,
                    });
                }
                Err(e) => {
                    tracing::warn!("endpoint {endpoint} failed: {e}");
                    last_error = e.to_string();
                }
            }
        }

        Err(JitoError::AllEndpointsFailed {
            count: endpoints.len(),
            last_error,
        })
    }

    /// Sends a bundle request to a single block-engine endpoint.
    async fn send_bundle_to_endpoint(
        &self,
        endpoint: &str,
        encoded_txs: &[String],
    ) -> Result<String, JitoError> {
        #[derive(Serialize)]
        struct BundleParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            encoding: Option<&'a str>,
        }
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "sendBundle",
            params: (
                encoded_txs,
                BundleParams {
                    encoding: Some("base64"),
                },
            ),
        };
        let (status, response_text) = self
            .send_json_rpc_request(self.jito_post(endpoint), &request, "sendBundle")
            .await?;
        if !status.is_success() {
            return Err(JitoError::Network {
                reason: format!("sendBundle HTTP {status}: {response_text}"),
            });
        }
        Self::parse_json_rpc_result(&response_text, "sendBundle", "no bundle_id in response")
    }
}
````

## File: src/client/simulate.rs
````rust
use crate::bundler::bundle::types::BuiltBundle;
use crate::client::jito_bundler::JitoBundler;
use crate::error::JitoError;
use crate::types::{
    JsonRpcRequest, SimulateBundleApiResult, SimulateBundleParams, SimulateBundleSummary,
    SimulateBundleValue,
};
use solana_client::rpc_config::RpcSimulateTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;

impl JitoBundler {
    // --- Simulation ---
    /// Simulates each transaction individually via standard Solana RPC.
    pub async fn simulate_per_transaction(&self, bundle: &BuiltBundle) -> Result<(), JitoError> {
        for (i, tx) in bundle.transactions.iter().enumerate() {
            let sig = Self::first_signature_base58(tx, i, "simulate_per_transaction")?;
            let config = RpcSimulateTransactionConfig {
                sig_verify: true,
                replace_recent_blockhash: false,
                commitment: Some(CommitmentConfig::confirmed()),
                accounts: None,
                min_context_slot: None,
                inner_instructions: false,
                encoding: None,
            };

            match self
                .rpc_client
                .simulate_transaction_with_config(tx, config)
                .await
            {
                Ok(result) => {
                    if let Some(err) = result.value.err {
                        let logs = result.value.logs.unwrap_or_default();
                        return Err(JitoError::SimulationFailed {
                            details: format!(
                                "transaction {i} simulation failed: {err:?}\nsignature: {sig}\nlogs: {logs:?}"
                            ),
                        });
                    }
                }
                Err(e) => {
                    return Err(JitoError::SimulationFailed {
                        details: format!(
                            "transaction {i} RPC simulation error: {e}\nsignature: {sig}"
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Simulates the full atomic bundle through Helius `simulateBundle`.
    pub async fn simulate_bundle_helius(
        &self,
        bundle: &BuiltBundle,
        helius_rpc_url: &str,
    ) -> Result<SimulateBundleValue, JitoError> {
        let encoded_transactions = Self::encode_transactions_base64(&bundle.transactions)?;
        let params = SimulateBundleParams {
            encoded_transactions,
        };
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "simulateBundle",
            params: [params],
        };
        let (status, response_text) = self
            .send_json_rpc_request(
                self.http_client
                    .post(helius_rpc_url)
                    .header("Content-Type", "application/json"),
                &request,
                "Helius simulateBundle",
            )
            .await?;
        if !status.is_success() {
            return Err(JitoError::Network {
                reason: format!("Helius simulateBundle HTTP {status}: {response_text}"),
            });
        }
        let result: SimulateBundleApiResult = Self::parse_json_rpc_result(
            &response_text,
            "Helius simulateBundle",
            "no result in Helius simulateBundle response",
        )?;

        if let SimulateBundleSummary::Failed(failure) = &result.value.summary {
            let tx_count = bundle.transactions.len();
            let results_count = result.value.transaction_results.len();
            let failed_tx_index = if results_count < tx_count {
                results_count
            } else {
                result
                    .value
                    .transaction_results
                    .iter()
                    .position(|r| r.err.is_some())
                    .unwrap_or(0)
            };
            let mut error_details = format!(
                "bundle simulation failed at transaction {failed_tx_index}: {}\n\
                 failing tx signature: {:?}\n\
                 bundle size: {tx_count} transactions, results returned: {results_count}",
                failure.error_message(),
                failure.tx_signature
            );

            if results_count < tx_count {
                error_details.push_str(&format!(
                    "\nHelius stopped after tx {failed_tx_index} failed - no logs for subsequent transactions"
                ));
            }
            for (idx, tx_result) in result.value.transaction_results.iter().enumerate() {
                let status_str = if tx_result.err.is_some() {
                    "FAILED"
                } else {
                    "OK"
                };
                let units = tx_result
                    .units_consumed
                    .map_or_else(|| "N/A".to_string(), |u| u.to_string());
                error_details.push_str(&format!("\n\n=== transaction {idx} [{status_str}] ==="));
                error_details.push_str(&format!("\ncompute units: {units}"));
                if let Some(err) = &tx_result.err {
                    error_details.push_str(&format!("\nerror: {err}"));
                }
                if let Some(logs) = &tx_result.logs {
                    error_details.push_str("\nlogs:");
                    for log in logs {
                        error_details.push_str(&format!("\n  {log}"));
                    }
                } else {
                    error_details.push_str("\nlogs: none");
                }
            }
            return Err(JitoError::SimulationFailed {
                details: error_details,
            });
        }
        Ok(result.value)
    }
}
````

## File: src/client/status.rs
````rust
use crate::client::jito_bundler::JitoBundler;
use crate::client::types::StatusResult;
use crate::constants::DEFAULT_INITIAL_CONFIRM_DELAY_SECS;
use crate::error::JitoError;
use crate::types::{BundleStatus, JsonRpcRequest};
use solana_sdk::signature::Signature;
use solana_transaction_status_client_types::TransactionConfirmationStatus;
use std::str::FromStr;
use std::time::Duration;

impl JitoBundler {
    // --- Status Polling ---
    /// Fetches current bundle status from Jito block engine.
    pub async fn get_bundle_status(&self, bundle_id: &str) -> BundleStatus {
        let endpoint = self.config.network.block_engine_url();
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: "getBundleStatuses",
            params: [[bundle_id]],
        };
        let (status_code, response_text) = match self
            .send_json_rpc_request(self.jito_post(endpoint), &request, "getBundleStatuses")
            .await
        {
            Ok(result) => result,
            Err(_) => return BundleStatus::Unknown,
        };
        if status_code.as_u16() == 429 {
            return BundleStatus::Pending;
        }
        if !status_code.is_success() {
            return BundleStatus::Unknown;
        }
        let result: StatusResult = match Self::parse_json_rpc_result(
            &response_text,
            "getBundleStatuses",
            "no result in getBundleStatuses response",
        ) {
            Ok(r) => r,
            Err(_) => return BundleStatus::Unknown,
        };
        if let Some(status) = result.value.first() {
            if let Some(err) = &status.err {
                return BundleStatus::Failed {
                    error: Some(err.to_string()),
                };
            }

            if let Some(confirmation_status) = &status.confirmation_status
                && (confirmation_status == "confirmed" || confirmation_status == "finalized")
            {
                return BundleStatus::Landed { slot: status.slot };
            }

            return BundleStatus::Pending;
        }
        BundleStatus::Unknown
    }

    /// Polls signature statuses until all bundle transactions land or fail.
    pub async fn wait_for_landing_on_chain(
        &self,
        signatures: &[String],
    ) -> Result<BundleStatus, JitoError> {
        if signatures.is_empty() {
            return Err(JitoError::InvalidSignature {
                reason: "no signatures provided for confirmation".to_string(),
            });
        }

        let parsed_signatures: Vec<Signature> = signatures
            .iter()
            .map(|s| Signature::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JitoError::InvalidSignature {
                reason: e.to_string(),
            })?;
        tokio::time::sleep(Duration::from_secs(DEFAULT_INITIAL_CONFIRM_DELAY_SECS)).await;
        let max_attempts = self.config.confirm_policy.max_attempts;
        let interval_ms = self.config.confirm_policy.interval_ms;
        let mut had_successful_poll = false;
        let mut last_rpc_error: Option<String> = None;
        for _attempt in 0..max_attempts {
            match self
                .rpc_client
                .get_signature_statuses(&parsed_signatures)
                .await
            {
                Ok(statuses) => {
                    had_successful_poll = true;
                    for (j, status) in statuses.value.iter().enumerate() {
                        if let Some(s) = status
                            && let Some(err) = &s.err
                        {
                            return Ok(BundleStatus::Failed {
                                error: Some(format!("transaction {j} failed: {err:?}")),
                            });
                        }
                    }
                    let all_confirmed = statuses.value.iter().all(|status| {
                        status.as_ref().is_some_and(|s| {
                            s.confirmation_status.as_ref().is_some_and(|cs| {
                                cs == &TransactionConfirmationStatus::Confirmed
                                    || cs == &TransactionConfirmationStatus::Finalized
                            })
                        })
                    });
                    if all_confirmed {
                        let slot = statuses
                            .value
                            .first()
                            .and_then(|s| s.as_ref().map(|s| s.slot));
                        return Ok(BundleStatus::Landed { slot });
                    }
                }
                Err(e) => {
                    tracing::warn!("get_signature_statuses poll failed: {e}");
                    last_rpc_error = Some(e.to_string());
                }
            }
            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }
        Err(Self::polling_timeout_error(
            max_attempts,
            had_successful_poll,
            last_rpc_error.as_deref(),
        ))
    }

    /// Polls Jito bundle status endpoint until final status or timeout.
    pub async fn wait_for_landing_via_jito(
        &self,
        bundle_id: &str,
    ) -> Result<BundleStatus, JitoError> {
        tokio::time::sleep(Duration::from_secs(DEFAULT_INITIAL_CONFIRM_DELAY_SECS)).await;
        let max_attempts = self.config.confirm_policy.max_attempts;
        let interval_ms = self.config.confirm_policy.interval_ms;
        for _attempt in 0..max_attempts {
            let status = self.get_bundle_status(bundle_id).await;
            match &status {
                BundleStatus::Landed { .. } | BundleStatus::Failed { .. } => {
                    return Ok(status);
                }
                _ => {}
            }
            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }
        Err(JitoError::ConfirmationTimeout {
            attempts: max_attempts,
        })
    }

    /// Converts polling terminal state into either timeout or actionable RPC error.
    fn polling_timeout_error(
        max_attempts: u32,
        had_successful_poll: bool,
        last_rpc_error: Option<&str>,
    ) -> JitoError {
        if had_successful_poll {
            return JitoError::ConfirmationTimeout {
                attempts: max_attempts,
            };
        }

        let reason = match last_rpc_error {
            Some(err) => {
                format!("get_signature_statuses failed for all {max_attempts} attempts: {err}")
            }
            None => format!("get_signature_statuses failed for all {max_attempts} attempts"),
        };
        JitoError::Network { reason }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polling_timeout_error_returns_timeout_after_successful_polls() {
        let err = JitoBundler::polling_timeout_error(10, true, Some("ignored"));
        assert!(
            matches!(err, JitoError::ConfirmationTimeout { attempts: 10 }),
            "expected ConfirmationTimeout, got {err:?}"
        );
    }

    #[test]
    fn polling_timeout_error_returns_network_when_all_polls_fail() {
        let err = JitoBundler::polling_timeout_error(5, false, Some("rpc unavailable"));
        assert!(
            matches!(err, JitoError::Network { .. }),
            "expected Network error, got {err:?}"
        );
        let err_text = err.to_string();
        assert!(
            err_text.contains("rpc unavailable"),
            "expected error details to include RPC failure, got {err_text}"
        );
    }
}
````

## File: src/client/types.rs
````rust
use serde::Deserialize;

/// One bundle status entry returned by `getBundleStatuses`.
#[derive(Debug, Deserialize)]
pub struct BundleStatusValue {
    /// Confirmation status string from block engine.
    pub confirmation_status: Option<String>,
    /// Slot where bundle landed, when available.
    pub slot: Option<u64>,
    /// Raw error object when status is failed.
    pub err: Option<serde_json::Value>,
}
/// Parsed payload wrapper for `getBundleStatuses`.
#[derive(Debug, Deserialize)]
pub struct StatusResult {
    /// Returned status entries.
    pub value: Vec<BundleStatusValue>,
}
````

## File: src/config/confirm_policy.rs
````rust
use crate::constants::{DEFAULT_CONFIRM_INTERVAL_MS, DEFAULT_MAX_CONFIRM_ATTEMPTS};

/// Polling policy used when waiting for landing confirmation.
#[derive(Debug, Clone)]
pub struct ConfirmPolicy {
    /// Maximum number of polling attempts.
    pub max_attempts: u32,
    /// Delay between polling attempts in milliseconds.
    pub interval_ms: u64,
}

impl Default for ConfirmPolicy {
    /// Returns default confirmation polling settings.
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_CONFIRM_ATTEMPTS,
            interval_ms: DEFAULT_CONFIRM_INTERVAL_MS,
        }
    }
}
````

## File: src/config/jito.rs
````rust
use crate::config::confirm_policy::ConfirmPolicy;
use crate::config::network::Network;
use crate::config::tip_strategy::TipStrategy;
use crate::constants::DEFAULT_COMPUTE_UNIT_LIMIT;
use solana_pubkey::Pubkey;

/// User-facing runtime configuration for `JitoBundler`.
#[derive(Debug, Clone)]
pub struct JitoConfig {
    /// Target network and block-engine endpoints.
    pub network: Network,
    /// Solana RPC URL used for chain reads.
    pub rpc_url: String,
    /// Optional Helius RPC URL for atomic simulation.
    pub helius_rpc_url: Option<String>,
    /// Optional Jito UUID for authenticated requests.
    pub uuid: Option<String>,
    /// Strategy used to resolve bundle tip amount.
    pub tip_strategy: TipStrategy,
    /// Confirmation polling behavior after send.
    pub confirm_policy: ConfirmPolicy,
    /// Optional `jitodontfront` pubkey to inject.
    pub jitodontfront_pubkey: Option<Pubkey>,
    /// Compute unit limit prepended to each transaction.
    pub compute_unit_limit: u32,
}

impl JitoConfig {
    // --- Construction ---
    /// Creates a config with sensible defaults for mainnet usage.
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            network: Network::Mainnet,
            rpc_url: rpc_url.into(),
            helius_rpc_url: None,
            uuid: None,
            tip_strategy: TipStrategy::default(),
            confirm_policy: ConfirmPolicy::default(),
            jitodontfront_pubkey: None,
            compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
        }
    }

    // --- Builder Setters ---
    /// Sets target Jito network configuration.
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Sets optional Helius RPC URL for atomic simulation.
    pub fn with_helius_rpc_url(mut self, url: impl Into<String>) -> Self {
        self.helius_rpc_url = Some(url.into());
        self
    }

    /// Sets optional Jito UUID for authenticated requests.
    pub fn with_uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Sets tip strategy used during bundle build.
    pub fn with_tip_strategy(mut self, strategy: TipStrategy) -> Self {
        self.tip_strategy = strategy;
        self
    }

    /// Sets confirmation polling behavior.
    pub fn with_confirm_policy(mut self, policy: ConfirmPolicy) -> Self {
        self.confirm_policy = policy;
        self
    }

    /// Enables `jitodontfront` account injection.
    pub fn with_jitodontfront(mut self, pubkey: Pubkey) -> Self {
        self.jitodontfront_pubkey = Some(pubkey);
        self
    }

    /// Sets compute unit limit prepended to each transaction.
    pub fn with_compute_unit_limit(mut self, limit: u32) -> Self {
        self.compute_unit_limit = limit;
        self
    }
}
````

## File: src/config/mod.rs
````rust
pub mod confirm_policy;
pub mod jito;
pub mod network;
pub mod tip_strategy;
````

## File: src/config/network.rs
````rust
use crate::constants::{JITO_BLOCK_ENGINE_URL, JITO_MAINNET_ENDPOINTS, JITO_TIPS_FLOOR_URL};

/// Supported block-engine network configurations.
#[derive(Debug, Clone)]
pub enum Network {
    /// Use default Jito mainnet endpoints.
    Mainnet,
    /// Use custom block-engine and tip-floor URLs.
    Custom {
        /// Custom block-engine base URL.
        block_engine_url: String,
        /// Custom tip-floor API URL.
        tip_floor_url: String,
    },
}

impl Network {
    // --- Endpoint Accessors ---
    /// Returns the block-engine URL for status/send JSON-RPC calls.
    pub fn block_engine_url(&self) -> &str {
        match self {
            Network::Mainnet => JITO_BLOCK_ENGINE_URL,
            Network::Custom {
                block_engine_url, ..
            } => block_engine_url,
        }
    }

    /// Returns the tip-floor API URL for tip resolution.
    pub fn tip_floor_url(&self) -> &str {
        match self {
            Network::Mainnet => JITO_TIPS_FLOOR_URL,
            Network::Custom { tip_floor_url, .. } => tip_floor_url,
        }
    }

    /// Returns true when using custom network endpoints.
    pub fn is_custom(&self) -> bool {
        matches!(self, Network::Custom { .. })
    }

    /// Returns ordered send endpoints used for retry strategy.
    pub fn send_endpoints(&self) -> Vec<String> {
        match self {
            Network::Mainnet => JITO_MAINNET_ENDPOINTS
                .iter()
                .map(|ep| format!("{JITO_BLOCK_ENGINE_URL}?endpoint={ep}"))
                .collect(),
            Network::Custom {
                block_engine_url, ..
            } => vec![block_engine_url.clone()],
        }
    }
}
````

## File: src/config/tip_strategy.rs
````rust
use crate::constants::{DEFAULT_TIP_LAMPORTS, MAX_TIP_LAMPORTS};

/// Strategy for selecting bundle tip lamports.
#[derive(Debug, Clone)]
pub enum TipStrategy {
    /// Uses a fixed tip amount.
    Fixed(u64),
    /// Uses raw fetched tip floor value.
    FetchFloor,
    /// Uses fetched floor clamped to configured min/max.
    FetchFloorWithCap {
        /// Minimum tip in lamports.
        min: u64,
        /// Maximum tip in lamports.
        max: u64,
    },
}

impl Default for TipStrategy {
    /// Returns default capped floor strategy.
    fn default() -> Self {
        TipStrategy::FetchFloorWithCap {
            min: DEFAULT_TIP_LAMPORTS,
            max: MAX_TIP_LAMPORTS,
        }
    }
}
````

## File: src/analysis.rs
````rust
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
````

## File: src/constants.rs
````rust
use solana_pubkey::{Pubkey, pubkey};

pub const JITO_BLOCK_ENGINE_URL: &str = "https://mainnet.block-engine.jito.wtf/api/v1/bundles";

pub const JITO_MAINNET_ENDPOINTS: [&str; 5] = ["mainnet", "amsterdam", "frankfurt", "ny", "tokyo"];

pub const JITO_TIPS_FLOOR_URL: &str = "https://bundles.jito.wtf/api/v1/bundles/tip_floor";

pub const JITO_EXPLORER_URL: &str = "https://explorer.jito.wtf/events";

pub const SOLANA_EXPLORER_URL: &str = "https://explorer.solana.com/tx";

pub const JITO_TIP_ACCOUNTS: [Pubkey; 8] = [
    pubkey!("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"),
    pubkey!("HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe"),
    pubkey!("Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY"),
    pubkey!("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49"),
    pubkey!("DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh"),
    pubkey!("ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt"),
    pubkey!("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL"),
    pubkey!("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"),
];

pub const DEFAULT_TIP_LAMPORTS: u64 = 100_000;
pub const MAX_TIP_LAMPORTS: u64 = 10_000_000;

/// Hard Jito protocol limit: bundles may contain at most 5 transactions.
pub const MAX_BUNDLE_TRANSACTIONS: usize = 5;

pub const SOLANA_MAX_TX_SIZE: usize = 1232;

pub const DEFAULT_COMPUTE_UNIT_LIMIT: u32 = 3_000_000;

pub const DEFAULT_MAX_CONFIRM_ATTEMPTS: u32 = 30;
pub const DEFAULT_CONFIRM_INTERVAL_MS: u64 = 2_000;
pub const DEFAULT_INITIAL_CONFIRM_DELAY_SECS: u64 = 5;

pub const SYSTEM_PROGRAM_ID: Pubkey = pubkey!("11111111111111111111111111111111");
````

## File: src/error.rs
````rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JitoError {
    #[error("bundle must contain at least 1 transaction, got {count}")]
    InvalidBundleSize { count: usize },
    #[error(
        "tip account {tip_account} found in address lookup table — this will cause a runtime failure"
    )]
    TipAccountInLut { tip_account: String },
    #[error("transaction {index} exceeds max size: {size}/{max} bytes")]
    TransactionOversized {
        index: usize,
        size: usize,
        max: usize,
    },
    #[error("failed to compile v0 message for transaction {index}: {reason}")]
    MessageCompileFailed { index: usize, reason: String },
    #[error("failed to create versioned transaction {index}: {reason}")]
    TransactionCreationFailed { index: usize, reason: String },
    #[error("simulation failed: {details}")]
    SimulationFailed { details: String },
    #[error("bundle rejected by Jito: {reason}")]
    BundleRejected { reason: String },
    #[error("all {count} endpoints failed, last error: {last_error}")]
    AllEndpointsFailed { count: usize, last_error: String },
    #[error("bundle confirmation timed out after {attempts} attempts")]
    ConfirmationTimeout { attempts: u32 },
    #[error("bundle failed on-chain: {reason}")]
    OnChainFailure { reason: String },
    #[error("failed to fetch tip floor: {reason}")]
    TipFloorFetchFailed { reason: String },
    #[error("serialization error: {reason}")]
    Serialization { reason: String },
    #[error("JSON-RPC error ({code}): {message}")]
    JsonRpc { code: i64, message: String },
    #[error("network error: {reason}")]
    Network { reason: String },
    #[error("invalid signature format: {reason}")]
    InvalidSignature { reason: String },
    #[error("get_latest_blockhash error: {reason}")]
    GetLatestBlockhash { reason: String },
}
````

## File: src/lib.rs
````rust
#![doc = include_str!("../README.md")]

pub mod analysis;
pub mod bundler;
pub mod client;
pub mod config;
pub mod constants;
pub mod error;
pub mod tip;
pub mod types;

pub use error::JitoError;
````

## File: src/tip.rs
````rust
use crate::config::tip_strategy::TipStrategy;
use crate::constants::{JITO_TIP_ACCOUNTS, SYSTEM_PROGRAM_ID};
use crate::error::JitoError;
use crate::types::JitoTipFloorResponse;
use rand::Rng;
use reqwest::Client;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

pub struct TipHelper;

impl TipHelper {
    const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

    // --- Tip Account / Instruction ---
    /// Picks a random Jito tip account from known constants.
    pub fn get_random_tip_account() -> Pubkey {
        let index = rand::rng().random_range(0..JITO_TIP_ACCOUNTS.len());
        JITO_TIP_ACCOUNTS[index]
    }

    /// Creates a system transfer instruction for the tip payment.
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

    // --- Tip Resolution ---
    /// Fetches current tip floor data and returns computed lamports.
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

        let tip_in_lamports = Self::compute_tip_floor_lamports(tip_data)?;

        Ok((tip_in_lamports, tip_data.clone()))
    }

    /// Resolves effective tip amount for the provided strategy.
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
                    &TipStrategy::FetchFloorWithCap {
                        min: *min,
                        max: *max,
                    },
                ))
            }
        }
    }

    /// Converts EMA 50th percentile SOL tip value into lamports.
    fn compute_tip_floor_lamports(tip_data: &JitoTipFloorResponse) -> Result<u64, JitoError> {
        let ema_50th = tip_data.ema_landed_tips_50th_percentile;
        if !ema_50th.is_finite() {
            return Err(JitoError::TipFloorFetchFailed {
                reason: format!("invalid tip floor value (non-finite): {ema_50th}"),
            });
        }
        if ema_50th < 0.0 {
            return Err(JitoError::TipFloorFetchFailed {
                reason: format!("invalid tip floor value (negative): {ema_50th}"),
            });
        }

        let tip_float = (ema_50th * Self::LAMPORTS_PER_SOL).ceil();
        if !tip_float.is_finite() || tip_float > u64::MAX as f64 {
            return Err(JitoError::TipFloorFetchFailed {
                reason: format!("tip floor value is out of range: {ema_50th} SOL"),
            });
        }

        Ok(tip_float as u64)
    }

    /// Applies strategy transforms such as min/max clamping.
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

    /// Builds a synthetic tip-floor payload for strategy tests.
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

    /// Verifies random tip account selection always returns known accounts.
    #[test]
    fn random_tip_account_is_valid() {
        for _ in 0..100 {
            let account = TipHelper::get_random_tip_account();
            assert!(JITO_TIP_ACCOUNTS.contains(&account));
        }
    }

    /// Verifies raw floor strategy does not apply min/max clamping.
    #[test]
    fn fetch_floor_does_not_clamp_by_default() {
        let tip_data = make_tip_floor(20.0);
        let tip = TipHelper::compute_tip_floor_lamports(&tip_data);
        assert!(
            tip.is_ok(),
            "expected valid tip floor conversion, got {tip:?}"
        );
        assert_eq!(tip.unwrap_or_default(), 20_000_000_000);
    }

    /// Verifies invalid floor values return typed errors.
    #[test]
    fn fetch_floor_negative_or_nan_return_error() {
        let negative = make_tip_floor(-0.1);
        assert!(
            TipHelper::compute_tip_floor_lamports(&negative).is_err(),
            "expected negative tip floor to be rejected"
        );

        let nan = make_tip_floor(f64::NAN);
        assert!(
            TipHelper::compute_tip_floor_lamports(&nan).is_err(),
            "expected NaN tip floor to be rejected"
        );
    }

    /// Verifies infinite tip floors are rejected.
    #[test]
    fn fetch_floor_infinite_returns_error() {
        let infinite = make_tip_floor(f64::INFINITY);
        assert!(
            TipHelper::compute_tip_floor_lamports(&infinite).is_err(),
            "expected infinite tip floor to be rejected"
        );
    }

    /// Verifies capped floor strategy applies both min and max bounds.
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
````

## File: src/types.rs
````rust
use serde::{Deserialize, Serialize};

/// Generic JSON-RPC request payload wrapper.
#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<T: Serialize> {
    /// JSON-RPC protocol version.
    pub jsonrpc: &'static str,
    /// Request id used for matching responses.
    pub id: u64,
    /// RPC method name.
    pub method: &'static str,
    /// Method-specific parameters.
    pub params: T,
}

/// Generic JSON-RPC response payload wrapper.
#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    /// JSON-RPC protocol version.
    pub jsonrpc: String,
    /// Response id corresponding to request id.
    pub id: u64,
    /// Success result payload when present.
    pub result: Option<T>,
    /// Error payload when present.
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error object.
#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    /// JSON-RPC error code.
    pub code: i64,
    /// Human-readable error message.
    pub message: String,
}

/// Jito tip-floor API response entry.
#[derive(Debug, Deserialize, Clone)]
pub struct JitoTipFloorResponse {
    /// Response timestamp.
    pub time: String,
    /// 25th percentile landed tip (SOL units).
    pub landed_tips_25th_percentile: f64,
    /// 50th percentile landed tip (SOL units).
    pub landed_tips_50th_percentile: f64,
    /// 75th percentile landed tip (SOL units).
    pub landed_tips_75th_percentile: f64,
    /// 95th percentile landed tip (SOL units).
    pub landed_tips_95th_percentile: f64,
    /// 99th percentile landed tip (SOL units).
    pub landed_tips_99th_percentile: f64,
    /// EMA 50th percentile landed tip (SOL units).
    pub ema_landed_tips_50th_percentile: f64,
}

/// High-level bundle landing status.
#[derive(Clone)]
pub enum BundleStatus {
    /// Bundle is still pending.
    Pending,
    /// Bundle landed with optional slot.
    Landed { slot: Option<u64> },
    /// Bundle failed with optional error details.
    Failed { error: Option<String> },
    /// Status could not be determined.
    Unknown,
}

impl std::fmt::Debug for BundleStatus {
    /// Formats bundle status in a compact human-readable form.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleStatus::Pending => write!(f, "Pending"),
            BundleStatus::Landed { slot } => write!(f, "Landed(slot: {slot:?})"),
            BundleStatus::Failed { error } => write!(f, "Failed(error: {error:?})"),
            BundleStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Result returned after bundle submission.
#[derive(Debug, Clone)]
pub struct BundleResult {
    /// Returned bundle id.
    pub bundle_id: String,
    /// Transaction signatures from submitted bundle.
    pub signatures: Vec<String>,
    /// Explorer URL for bundle status.
    pub explorer_url: String,
}

/// Parameters for Helius `simulateBundle` call.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleParams {
    /// Base64-encoded transactions in bundle order.
    pub encoded_transactions: Vec<String>,
}

/// Top-level Helius `simulateBundle` result.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleApiResult {
    /// Simulation context metadata.
    pub context: SimulateBundleContext,
    /// Simulation output payload.
    pub value: SimulateBundleValue,
}

/// Simulation context metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleContext {
    /// API version that handled simulation.
    pub api_version: String,
    /// Slot used for simulation context.
    pub slot: u64,
}

/// Full simulation output.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleValue {
    /// Overall simulation summary.
    pub summary: SimulateBundleSummary,
    /// Per-transaction simulation results.
    pub transaction_results: Vec<TransactionSimulationResult>,
}

/// Overall simulation summary state.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimulateBundleSummary {
    /// All transactions simulated successfully.
    Succeeded,
    /// Simulation failed with details.
    Failed(SimulateBundleFailure),
}

/// Failure payload returned by Helius simulation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBundleFailure {
    /// Raw simulation error object.
    pub error: serde_json::Value,
    /// Optional failing transaction signature.
    pub tx_signature: Option<String>,
}

impl SimulateBundleFailure {
    /// Extracts a readable error message from Helius failure payload.
    pub fn error_message(&self) -> String {
        if let Some(tx_failure) = self.error.get("TransactionFailure")
            && let Some(arr) = tx_failure.as_array()
            && let Some(msg) = arr.get(1).and_then(|v| v.as_str())
        {
            return msg.to_string();
        }
        self.error.to_string()
    }
}

/// Per-transaction simulation result entry.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSimulationResult {
    /// Transaction-level error when present.
    pub err: Option<serde_json::Value>,
    /// Program logs emitted during execution.
    pub logs: Option<Vec<String>>,
    /// Compute units consumed by transaction.
    pub units_consumed: Option<u64>,
    /// Optional program return data.
    pub return_data: Option<ReturnData>,
    /// Optional pre-execution account states.
    pub pre_execution_accounts: Option<Vec<AccountState>>,
    /// Optional post-execution account states.
    pub post_execution_accounts: Option<Vec<AccountState>>,
}

/// Account snapshot in simulation response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountState {
    /// Base58-encoded account pubkey.
    pub pubkey: String,
    /// Account lamports balance.
    pub lamports: u64,
    /// Account data payload.
    pub data: Vec<String>,
    /// Base58-encoded owner program id.
    pub owner: String,
    /// Whether account is executable.
    pub executable: bool,
    /// Account rent epoch.
    pub rent_epoch: u64,
}

/// Program return data from simulation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnData {
    /// Program id that returned the data.
    pub program_id: String,
    /// Encoded return data payload.
    pub data: Vec<String>,
}
````

## File: tests/build/memo_bundle.rs
````rust
use crate::common;
use jito_bundle::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use jito_bundle::constants::{DEFAULT_COMPUTE_UNIT_LIMIT, SOLANA_MAX_TX_SIZE};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL"]
async fn build_memo_bundle_succeeds() {
    let env =
        common::load_test_env().expect("missing required .env values: KEYPAIR_PATH and RPC_URL");

    let rpc = RpcClient::new(env.rpc_url);
    let blockhash = rpc
        .get_latest_blockhash()
        .await
        .expect("failed to fetch latest blockhash");

    let payer_pubkey = env.keypair.pubkey();
    let slots = common::build_memo_slots(
        &payer_pubkey,
        &["jito-bundle test memo 1", "jito-bundle test memo 2"],
    );

    let inputs = BundleBuilderInputs {
        payer: &env.keypair,
        transactions_instructions: slots,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    };
    let bundle = BundleBuilder::build(inputs).expect("bundle build failed");
    common::print_bundle_info("build_memo_bundle", &bundle);
    assert_eq!(bundle.transactions.len(), 3);
    for (i, tx) in bundle.transactions.iter().enumerate() {
        let serialized = bincode::serialize(tx).unwrap_or_default();
        assert!(
            serialized.len() <= SOLANA_MAX_TX_SIZE,
            "transaction {i} is {size} bytes, exceeds {SOLANA_MAX_TX_SIZE}",
            size = serialized.len()
        );
    }
}
````

## File: tests/build/mod.rs
````rust
mod memo_bundle;
````

## File: tests/common/mod.rs
````rust
use jito_bundle::bundler::bundle::types::BuiltBundle;
use jito_bundle::bundler::types::{BundleInstructionSlots, BundleSlotView, TipMode};
use jito_bundle::config::jito::JitoConfig;
use jito_bundle::config::network::Network;
use jito_bundle::constants::DEFAULT_TIP_LAMPORTS;
use jito_bundle::types::BundleResult;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::{Pubkey, pubkey};
use solana_sdk::signature::Keypair;
use std::fs;
use std::str::FromStr;

const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
const BUNDLES_PATH: &str = "/api/v1/bundles";

pub struct TestEnv {
    pub keypair: Keypair,
    pub rpc_url: String,
    pub helius_url: Option<String>,
    pub jito_block_engine_url: Option<String>,
    pub jito_uuid: Option<String>,
    pub jitodontfront_pubkey: Option<Pubkey>,
    pub tip_lamports: u64,
}

pub fn load_test_env() -> Option<TestEnv> {
    dotenvy::dotenv().ok();

    let keypair_path = std::env::var("KEYPAIR_PATH").ok()?;
    let rpc_url = std::env::var("RPC_URL").ok()?;
    let helius_url = std::env::var("HELIUS_RPC_URL").ok();
    let jito_block_engine_url = std::env::var("JITO_BLOCK_ENGINE_URL").ok();
    let jito_uuid = std::env::var("JITO_UUID").ok();
    let jitodontfront_pubkey = std::env::var("JITODONTFRONT_PUBKEY")
        .ok()
        .and_then(|s| Pubkey::from_str(&s).ok());
    let tip_lamports = std::env::var("TIP_LAMPORTS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TIP_LAMPORTS);

    let file_contents = fs::read_to_string(&keypair_path).ok()?;
    let bytes: Vec<u8> = serde_json::from_str(&file_contents).ok()?;
    let keypair = Keypair::try_from(bytes.as_slice()).ok()?;

    Some(TestEnv {
        keypair,
        rpc_url,
        helius_url,
        jito_block_engine_url,
        jito_uuid,
        jitodontfront_pubkey,
        tip_lamports,
    })
}

fn build_custom_urls(base_url: &str) -> (String, String) {
    let trimmed = base_url.trim_end_matches('/');
    let block_engine_url = if trimmed.ends_with(BUNDLES_PATH) {
        trimmed.to_string()
    } else {
        format!("{trimmed}{BUNDLES_PATH}")
    };
    let tip_floor_url = format!("{trimmed}/tip_floor");
    (block_engine_url, tip_floor_url)
}

pub fn build_jito_config(env: &TestEnv) -> JitoConfig {
    let network = match &env.jito_block_engine_url {
        Some(base_url) => {
            let (block_engine_url, tip_floor_url) = build_custom_urls(base_url);
            println!("  block_engine_url: {block_engine_url}");
            println!("  tip_floor_url:    {tip_floor_url}");
            Network::Custom {
                block_engine_url,
                tip_floor_url,
            }
        }
        None => Network::Mainnet,
    };

    let mut config = JitoConfig::new(&env.rpc_url).with_network(network);

    if let Some(uuid) = &env.jito_uuid {
        config = config.with_uuid(uuid);
    }
    if let Some(helius_url) = &env.helius_url {
        config = config.with_helius_rpc_url(helius_url);
    }
    if let Some(jdf) = env.jitodontfront_pubkey {
        config = config.with_jitodontfront(jdf);
    }

    config
}

pub fn print_bundle_result(test_name: &str, result: &BundleResult) {
    let bar = "━".repeat(56);
    println!("\n{bar}");
    println!("  {test_name} — result");
    println!("{bar}");

    println!("  bundle_id: {}", result.bundle_id);
    println!("  explorer:  {}", result.explorer_url);

    let sigs = result.signatures.join(" | ");
    println!("  signatures: {sigs}");
    println!("{bar}\n");
}

pub fn create_memo_instruction(payer: &Pubkey, message: &str) -> Instruction {
    Instruction {
        program_id: MEMO_PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(*payer, true)],
        data: message.as_bytes().to_vec(),
    }
}

pub fn build_memo_slots(payer: &Pubkey, messages: &[&str]) -> BundleInstructionSlots {
    let mut slots: BundleInstructionSlots = [None, None, None, None, None];
    for (i, msg) in messages.iter().enumerate().take(5) {
        slots[i] = Some(vec![create_memo_instruction(payer, msg)]);
    }
    slots
}

fn short_pubkey(pk: &Pubkey) -> String {
    let s = pk.to_string();
    if s.len() > 8 {
        format!("{}..{}", &s[..4], &s[s.len() - 4..])
    } else {
        s
    }
}

pub fn print_bundle_info(test_name: &str, bundle: &BuiltBundle) {
    let bar = "━".repeat(56);
    println!("\n{bar}");
    println!("  {test_name}");
    println!("{bar}");

    let tx_count = bundle.transactions.len();
    let populated = bundle.populated_count();

    let bundle_id = bundle.transactions.first().map_or_else(
        || "n/a".to_string(),
        |tx| bs58::encode(&tx.signatures[0]).into_string(),
    );

    println!("  bundle_id: {bundle_id}");
    println!("  transactions: {tx_count} versioned · {populated} instruction slots");
    println!("  tip_account: {}", short_pubkey(&bundle.tip_account));
    println!(
        "  tip_mode: {}",
        if matches!(bundle.tip_mode, TipMode::SeparateTx) {
            "separate"
        } else {
            "inline"
        }
    );

    for (i, tx) in bundle.transactions.iter().enumerate() {
        let size = bincode::serialize(tx).map_or(0, |s| s.len());
        let sig = bs58::encode(&tx.signatures[0]).into_string();
        println!("  tx[{i}]: {sig} ({size} bytes)");
    }

    println!("  tip_lamports: {}", bundle.tip_lamports);
    println!("{bar}");
    println!("  {test_name}: BUILT (not yet sent)\n");
}
````

## File: tests/offline/mod.rs
````rust
use crate::common;
use jito_bundle::config::network::Network;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

#[test]
fn build_memo_slots_caps_at_five() {
    let payer = Keypair::new().pubkey();
    let messages = ["m1", "m2", "m3", "m4", "m5", "m6"];
    let slots = common::build_memo_slots(&payer, &messages);

    let populated_count = slots.iter().filter(|slot| slot.is_some()).count();
    assert_eq!(populated_count, 5, "expected at most 5 populated slots");
}

#[test]
fn build_jito_config_derives_custom_urls() {
    let env = common::TestEnv {
        keypair: Keypair::new(),
        rpc_url: "https://rpc.example.com".to_string(),
        helius_url: None,
        jito_block_engine_url: Some("https://proxy.example.com".to_string()),
        jito_uuid: None,
        jitodontfront_pubkey: None,
        tip_lamports: 100_000,
    };

    let config = common::build_jito_config(&env);
    match config.network {
        Network::Custom {
            block_engine_url,
            tip_floor_url,
        } => {
            assert_eq!(block_engine_url, "https://proxy.example.com/api/v1/bundles");
            assert_eq!(tip_floor_url, "https://proxy.example.com/tip_floor");
        }
        Network::Mainnet => panic!("expected custom network"),
    }
}
````

## File: tests/send/memo_send.rs
````rust
use crate::common;
use jito_bundle::client::jito_bundler::{BuildBundleOptions, JitoBundler};
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL; sends real bundle to mainnet"]
async fn send_memo_bundle_succeeds() {
    let env =
        common::load_test_env().expect("missing required .env values: KEYPAIR_PATH and RPC_URL");
    let config = common::build_jito_config(&env);
    let bundler = JitoBundler::new(config).expect("failed to create JitoBundler");
    let payer_pubkey = env.keypair.pubkey();
    let slots = common::build_memo_slots(
        &payer_pubkey,
        &["jito-bundle send test 1", "jito-bundle send test 2"],
    );
    let bundle = match bundler
        .build_bundle(BuildBundleOptions {
            payer: &env.keypair,
            transactions_instructions: slots,
            lookup_tables: &[],
        })
        .await
    {
        Ok(b) => b,
        Err(e) => {
            println!("bundle build failed: {e}");
            panic!("bundle build failed: {e}");
        }
    };

    common::print_bundle_info("send_memo_bundle", &bundle);

    let result = match bundler.send_and_confirm(&bundle).await {
        Ok(r) => r,
        Err(e) => {
            println!("send_and_confirm failed: {e}");
            panic!("send_and_confirm failed: {e}");
        }
    };

    common::print_bundle_result("send_memo_bundle", &result);
    assert!(
        !result.bundle_id.is_empty(),
        "bundle_id should not be empty on success"
    );
}
````

## File: tests/send/mod.rs
````rust
mod memo_send;
````

## File: tests/simulate/helius_simulation.rs
````rust
use crate::common;
use jito_bundle::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use jito_bundle::client::jito_bundler::JitoBundler;
use jito_bundle::config::jito::JitoConfig;
use jito_bundle::constants::DEFAULT_COMPUTE_UNIT_LIMIT;
use solana_sdk::signature::Signer;

#[tokio::test]
#[ignore = "requires .env with KEYPAIR_PATH and RPC_URL"]
async fn simulate_memo_bundle_on_helius() {
    let env =
        common::load_test_env().expect("missing required .env values: KEYPAIR_PATH and RPC_URL");
    let helius_url = env
        .helius_url
        .as_ref()
        .expect("missing HELIUS_RPC_URL for simulation test");
    let config = JitoConfig::new(&env.rpc_url).with_helius_rpc_url(helius_url);
    let bundler = JitoBundler::new(config).expect("failed to create JitoBundler");
    let blockhash = bundler
        .rpc_client
        .get_latest_blockhash()
        .await
        .expect("failed to fetch latest blockhash");
    let payer_pubkey = env.keypair.pubkey();
    let slots =
        common::build_memo_slots(&payer_pubkey, &["helius sim test 1", "helius sim test 2"]);
    let inputs = BundleBuilderInputs {
        payer: &env.keypair,
        transactions_instructions: slots,
        lookup_tables: &[],
        recent_blockhash: blockhash,
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    };
    let bundle = BundleBuilder::build(inputs).expect("bundle build failed");
    common::print_bundle_info("simulate_memo_bundle_on_helius", &bundle);
    let sim_result = bundler.simulate_helius(&bundle).await;
    assert!(
        sim_result.is_ok(),
        "helius simulation failed: {sim_result:?}"
    );
}
````

## File: tests/simulate/mod.rs
````rust
mod helius_simulation;
````

## File: tests/main.rs
````rust
#![allow(clippy::unwrap_used, reason = "test code")]
#![allow(clippy::expect_used, reason = "test code")]
#![allow(clippy::panic, reason = "test code")]
#![allow(clippy::print_stdout, reason = "tests print tx signatures to stdout")]

#[cfg(feature = "live-tests")]
mod build;
pub mod common;
mod offline;
#[cfg(feature = "live-tests")]
mod send;
#[cfg(feature = "live-tests")]
mod simulate;
````

## File: .gitignore
````
/target
.idea
/jito-rust-rpc
.env
````

## File: AGENTS.md
````markdown
# Repository Guidelines

## Project Structure & Module Organization
`jito-bundle` is a Rust library crate (`edition = 2024`, `rust-version = 1.85`).

- `src/`: library code.
- `src/bundler/`: bundle construction logic (`builder/`, `bundle/`, shared `types.rs`, unit tests).
- `src/client/`: network-facing flows (`jito_bundler.rs`, `send.rs`, `simulate.rs`, `status.rs`, shared `rpc.rs`).
- `src/config/`: runtime config types (`JitoConfig`, `Network`, `TipStrategy`, `ConfirmPolicy`).
- `tests/`: integration tests split by concern (`offline/`, `build/`, `send/`, `simulate/`), wired by `tests/main.rs`.
- `.github/workflows/ci.yml`: CI checks (`fmt`, `clippy`, `test`).

## Build, Test, and Development Commands
- `cargo build`: compile the crate.
- `cargo test`: run unit tests + default integration tests (offline only).
- `cargo test --features live-tests -- --ignored`: run opt-in live network tests.
- `cargo clippy --all-targets`: lint all targets.
- `cargo clippy --all-targets --all-features`: lint including feature-gated paths.
- `cargo fmt --all`: format source.
- `scripts/tests.sh`: run ignored integration tests with `nextest` and `live-tests`.

## Coding Style & Naming Conventions
Use standard Rust formatting (`rustfmt`, 4-space indentation). Prefer clear, typed APIs and avoid stringly-typed error handling. Public behavior-oriented names are snake_case (e.g., `wait_for_landing_on_chain`). Keep module responsibilities narrow and follow existing split-`impl` organization for `JitoBundler`.

Clippy is strict: avoid `unwrap`/`expect`/`panic` in production code; use typed errors (`JitoError`) and `Result`.

## Testing Guidelines
Write fast deterministic tests first (unit tests in `src/*`, offline integration tests in `tests/offline/`). Reserve real RPC/Jito/Helius tests for `live-tests` feature and `#[ignore]`.

Name tests by behavior, e.g., `tip_account_in_lut_rejected`.

## Commit & Pull Request Guidelines
Current history uses short one-line subjects (e.g., `refactor`, `prep refactor`). Keep commits small and focused, but prefer more descriptive imperative subjects like:

- `client: harden signature extraction`
- `tests: gate live flows behind feature`

PRs should include: summary, risk/impact, test commands run, and any env/config changes.

## Security & Configuration Tips
Never commit secrets or key material. Keep `.env` local. Common live-test vars: `KEYPAIR_PATH`, `RPC_URL`, optional `HELIUS_RPC_URL`, `JITO_UUID`, `JITODONTFRONT_PUBKEY`.
````

## File: Cargo.toml
````toml
[package]
name = "jito-bundle"
version = "0.1.5"
edition = "2024"
rust-version = "1.85"
license = "MIT"
authors = ["ohaddahan@gmail.com"]
description = "Rust client for Jito bundle"
repository = "https://github.com/ohaddahan/jito-bundle.git"
documentation = "https://docs.rs/jito-bundle"
exclude = [
  ".claude/*",
  ".github/*",
  "scripts/*",
  ".env*",
]

[features]
default = []
live-tests = []

[dependencies]
base64 = "0.22"
bincode = "1.3"
bs58 = "0.5"
rand = "0.9"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
solana-client = "2.3.3"
solana-compute-budget-interface = "2.2.2"
solana-instruction = "2.3.0"
solana-pubkey = "2.4.0"
solana-sdk = "2.3.1"
solana-transaction-status-client-types = "2.3.13"
thiserror = "2"
tokio = { version = "1", features = ["time"] }
tracing = "0.1"

[dev-dependencies]
dotenvy = "0.15"
tokio = { version = "1", features = ["full"] }

[lints.clippy]
pedantic = { level = "warn", priority = -1 }

# Pedantic overrides - too noisy or not applicable
missing_errors_doc = "allow"
missing_panics_doc = "allow"
must_use_candidate = "allow"
doc_markdown = "allow"
similar_names = "allow"
too_many_lines = "allow"
module_name_repetitions = "allow"
struct_excessive_bools = "allow"
fn_params_excessive_bools = "allow"
items_after_statements = "allow"
uninlined_format_args = "allow"
return_self_not_must_use = "allow"
cast_possible_truncation = "allow"
cast_sign_loss = "allow"
cast_precision_loss = "allow"
cast_possible_wrap = "allow"
default_trait_access = "allow"
match_same_arms = "allow"
unused_async = "allow"
manual_let_else = "allow"
used_underscore_binding = "allow"
ref_option_ref = "allow"
ref_option = "allow"
format_push_string = "allow"
case_sensitive_file_extension_comparisons = "allow"
unnecessary_wraps = "allow"
needless_pass_by_value = "allow"
trivially_copy_pass_by_ref = "allow"
no_effect_underscore_binding = "allow"
redundant_closure_for_method_calls = "allow"
wildcard_imports = "allow"

# Panic safety - deny operations that could panic
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"

# Prohibit allow attributes - no exceptions
allow_attributes = "deny"

# Allow large error types - error types need rich context for good UX
result_large_err = "allow"

# No debug macros in production code
dbg_macro = "deny"
# No incomplete code markers
todo = "deny"
unimplemented = "deny"
# Use tracing instead of println/eprintln in library code
print_stdout = "deny"
print_stderr = "deny"
````

## File: CLAUDE.md
````markdown
# jito-bundle

Standalone Rust library for submitting Jito Bundles on Solana.

## Crate Structure

8 public modules under `src/`, organized into 3 subdirectories + 5 root files (26 .rs files total). CI runs fmt + clippy + unit tests on every push (`.github/workflows/ci.yml`).

```
src/
├── lib.rs                       Re-exports + pub use JitoError
├── error.rs                     JitoError enum (15 typed variants via thiserror)
├── constants.rs                 Tip accounts, endpoint URLs, limits, SYSTEM_PROGRAM_ID
├── types.rs                     JSON-RPC request/response types, BundleStatus, simulation types
├── tip.rs                       TipHelper (stateless: random tip account, create tip ix, fetch floor)
├── analysis.rs                  TransactionAnalysis (stateless: size checks, LUT coverage diagnostics)
├── config/
│   ├── mod.rs                   Re-exports 4 sub-modules
│   ├── jito.rs                  JitoConfig with builder pattern (with_network, with_helius_rpc_url, etc.)
│   ├── network.rs               Network enum (Mainnet / Custom URLs)
│   ├── confirm_policy.rs        ConfirmPolicy (max_attempts + interval_ms)
│   └── tip_strategy.rs          TipStrategy (Fixed / FetchFloor / FetchFloorWithCap)
├── bundler/
│   ├── mod.rs                   Re-exports builder, bundle, tests, types
│   ├── types.rs                 BundleInstructionSlots type alias, TipMode enum, BundleSlotView trait
│   ├── tests.rs                 16 unit tests for bundle building
│   ├── builder/
│   │   ├── mod.rs               Re-exports types, utils
│   │   ├── types.rs             BundleBuilderInputs (input struct), BundleBuilder (mutable build state)
│   │   └── utils.rs             BundleBuilder impl — build pipeline, compile, sign, validate
│   └── bundle/
│       ├── mod.rs               Re-exports types
│       └── types.rs             BuiltBundle (immutable output artifact), BundleSlotView impl
└── client/
    ├── mod.rs                   Re-exports 6 sub-modules (types is private)
    ├── types.rs                 BundleStatusValue, StatusResult (private, used by status.rs)
    ├── jito_bundler.rs          JitoBundler facade + BuildBundleOptions input struct
    ├── rpc.rs                   impl JitoBundler — shared JSON-RPC utilities (send_json_rpc_request, parse_json_rpc_result, encode_transactions_base64, first_signature_base58)
    ├── send.rs                  impl JitoBundler — encode txs, retry across 5 geographic endpoints
    ├── simulate.rs              impl JitoBundler — per-tx RPC + Helius atomic simulation
    └── status.rs                impl JitoBundler — poll bundle status + on-chain confirmation
```

## Critical Bundle Rules

1. Max 5 transactions per bundle (enforced at compile time via `BundleInstructionSlots = [Option<Vec<Instruction>>; 5]`)
2. Instruction slots are compacted before build (gaps removed, order preserved)
3. jitodontfront goes into first tx's remaining_accounts (non-signer, non-writable)
4. Tip instruction is always last ix of last tx
5. If bundle < 5 txs: tip is a SEPARATE transaction compiled WITHOUT LUT (`TipMode::SeparateTx`)
6. If bundle == 5 txs: tip ix appended inline to last tx (with LUT) (`TipMode::InlineLastTx`)
7. MUST validate no LUT contains the tip account when tip is inline
8. Every tx gets a compute budget ix prepended

## Key Structs

- **`JitoBundler`** (`client/jito_bundler.rs`) — Facade owning `JitoConfig`, `reqwest::Client`, `RpcClient`. Methods: `new()`, `jito_post()`, `fetch_tip()`, `build_bundle()`, `simulate_helius()`, `send_and_confirm()`
- **`BuildBundleOptions<'a>`** (`client/jito_bundler.rs`) — Input struct for `build_bundle()` (payer, transactions_instructions, lookup_tables)
- **`BundleBuilderInputs<'a>`** (`bundler/builder/types.rs`) — Full input struct for `BundleBuilder::build()` (7 fields: payer, transactions_instructions, lookup_tables, recent_blockhash, tip_lamports, jitodontfront_pubkey, compute_unit_limit)
- **`BundleBuilder<'a>`** (`bundler/builder/types.rs`) — Mutable build state during bundle construction; has same fields as inputs plus tip_account and tip_mode
- **`BuiltBundle`** (`bundler/bundle/types.rs`) — Immutable output: transactions, tip_account, tip_lamports, tip_mode, instruction_slots
- **`BundleSlotView`** (`bundler/types.rs`) — Trait with `instruction_slots()`, `populated_count()`, `last_populated_index()` — implemented by both `BundleBuilder` and `BuiltBundle`
- **`TipMode`** (`bundler/types.rs`) — Enum: `SeparateTx` | `InlineLastTx`
- **`BundleInstructionSlots`** (`bundler/types.rs`) — Type alias for `[Option<Vec<Instruction>>; 5]`
- **`BundleResult`** (`types.rs`) — Submission result: `bundle_id: String`, `signatures: Vec<String>`, `explorer_url: String`. Derives `Clone`.
- **`TipHelper`** (`tip.rs`) — Stateless utility struct with static methods: `get_random_tip_account()`, `create_tip_instruction_to()`, `fetch_tip_floor()`, `resolve_tip()`
- **`TransactionAnalysis`** (`analysis.rs`) — Stateless utility struct with static methods for size checks and LUT diagnostics

## Gotchas / Lessons

- `thiserror` auto-interprets fields named `source` as `#[source]`, but `String` doesn't impl `std::error::Error`. Use `reason` instead.
- `solana_sdk::system_program` is deprecated — define `SYSTEM_PROGRAM_ID` via `pubkey!("111...")` macro
- `clippy::allow_attributes = "deny"` (stricter than `allow_attributes_without_reason`) means every `#[allow(...)]` needs a `reason = "..."`. In test code: `#[allow(clippy::unwrap_used, reason = "test code")]`.
- Edition 2024 enables let-chains (`if let Some(x) = foo && condition { ... }`)
- Solana crate versions are fragmented: `solana-sdk 2.3.1`, `solana-pubkey 2.4.0`, `solana-compute-budget-interface 2.2.2` — pin exact versions
- `unwrap_or_default()` is fine under `unwrap_used = "deny"` — it's a different method
- `client/send.rs`, `client/simulate.rs`, `client/status.rs`, `client/rpc.rs` all impl methods on `JitoBundler` (split impl blocks across files). Methods that use owned resources (`http_client`, `rpc_client`, `config`) must be `&self` instance methods, not static methods taking those resources as parameters.
- The bundle input field is named `transactions_instructions` — each slot is `Option<Vec<Instruction>>` via the `BundleInstructionSlots` type alias. Use `.iter().flatten()` to iterate only populated slots; use `.rposition()` to find the last populated slot.
- When iterating `Option` arrays in Rust, clippy prefers `.iter().flatten()` over `for slot { if let Some = slot }` (manual_flatten lint). Similarly, use `.enumerate()` instead of manual counter variables (explicit_counter_loop lint).
- `JitoBundler::build_bundle()` takes `BuildBundleOptions` as input struct (per >3 args rule). It fully destructures the struct before forwarding to `BundleBuilder::build()`.
- `jito_post()` appends UUID as both query param and `x-jito-auth` header, but skips the query param for custom networks (`is_custom()` check).
- `BundleBuilder` is mutable build state; `BuiltBundle` is the immutable output artifact. The conversion happens inside `BundleBuilder::build()` via `BuiltBundle::new()`.
- `client/rpc.rs` provides shared utilities: `send_json_rpc_request()` (POST + body extraction), `parse_json_rpc_result()` (JSON-RPC result/error extraction), `encode_transactions_base64()` (serialize + base64), `first_signature_base58()` (safe signature extraction). All four are reused by send, simulate, and status.
- `client/types.rs` is private (`mod types` not `pub mod types`) — contains `BundleStatusValue` and `StatusResult` used only by `status.rs`.
- `simulate_per_transaction()` and `wait_for_landing_via_jito()` are public API methods not used by the facade's `send_and_confirm()` flow. They exist for callers who want individual tx simulation or Jito-based (vs on-chain) confirmation polling.
- `first_signature_base58()` replaces direct `tx.signatures[0]` indexing throughout the codebase. Returns `Result<String, JitoError>` with `InvalidSignature` on missing signatures — avoids panics.
- `compute_tip_floor_lamports()` returns `Result<u64, JitoError>` with typed errors for non-finite, negative, or overflow values — previously coerced invalid values to 0 silently.
- `wait_for_landing_on_chain()` validates non-empty signatures upfront, tracks `had_successful_poll` and `last_rpc_error` for better timeout diagnostics. `polling_timeout_error()` returns `JitoError::Network` when all polls fail vs `ConfirmationTimeout` when polls succeeded but didn't confirm.
- `extract_signatures()` returns `Result<Vec<String>, JitoError>` (was `Vec<String>`) — uses `first_signature_base58` for safe extraction.
- `BundleResult` is a simple non-optional struct: `bundle_id: String`, `signatures: Vec<String>`, `explorer_url: String`. No `success`/`error` fields — errors are propagated via `Result<BundleResult, JitoError>`.
- `BundleStatus` derives `Clone` for downstream convenience.

## Dependencies

Solana: `solana-sdk 2.3.1`, `solana-client 2.3.3`, `solana-pubkey 2.4.0`, `solana-instruction 2.3.0`, `solana-compute-budget-interface 2.2.2`, `solana-transaction-status-client-types 2.3.13`
HTTP: `reqwest 0.12` (json), Encoding: `base64 0.22`, `bincode 1.3`, `bs58 0.5`
Error: `thiserror 2`, Async: `tokio 1` (time), Logging: `tracing 0.1`, Random: `rand 0.9`

## Testing

- Unit tests: 27 in lib — `bundler/tests.rs` (16), `analysis.rs` (1), `tip.rs` (5), `rpc.rs` (2), `status.rs` (2). Plus 3 README doc-tests (compile-check).
- Offline integration tests (always run, no network needed):
  - `tests/offline/mod.rs` — `build_memo_slots_caps_at_five`, `build_jito_config_derives_custom_urls`
- Live integration tests (gated behind `cfg(feature = "live-tests")`, all `#[ignore]`):
  - `tests/main.rs` — clippy allows (with reasons) + mod declarations. `build`, `send`, `simulate` only compiled with `live-tests` feature.
  - `tests/common/mod.rs` — `TestEnv`, `load_test_env()`, `build_jito_config()`, `create_memo_instruction()`, `build_memo_slots()`, `print_bundle_info()`, `print_bundle_result()`
  - `tests/build/memo_bundle.rs` — `build_memo_bundle_succeeds`
  - `tests/send/memo_send.rs` — `send_memo_bundle_succeeds`
  - `tests/simulate/helius_simulation.rs` — `simulate_memo_bundle_on_helius`
- Integration tests use `.expect()` to fail loudly when env is missing (not silent early returns)
- Required `.env` vars: `KEYPAIR_PATH`, `RPC_URL`
- Optional `.env` vars: `HELIUS_RPC_URL`, `JITO_BLOCK_ENGINE_URL`, `JITO_UUID`, `JITODONTFRONT_PUBKEY`, `TIP_LAMPORTS`
- Custom `JITO_BLOCK_ENGINE_URL` → `build_custom_urls()` derives both block engine + tip floor URLs from base
- Run unit tests: `cargo test`
- Run integration tests: `cargo test --features live-tests -- --ignored`
- CI: `.github/workflows/ci.yml` runs `cargo fmt --check`, `cargo clippy --all-targets`, `cargo test` on every push/PR

## See Also

- `PLAN.md` — full architecture, state machine, design decisions
- `FOR_USER.md` — plain-language project explanation
````

## File: FOR_USER.md
````markdown
# jito-bundle — A Developer's Guide

## What Is This?

This is a standalone Rust library for submitting **Jito Bundles** on Solana. Think of it as a well-packaged toolkit: you hand it a set of Solana instructions, and it handles everything needed to get them executed atomically as a Jito bundle — tipping the validator, simulating the transactions, sending them across multiple endpoints, and confirming they landed on-chain.

Previously, this logic lived inside the `worker-service` monolith. We extracted it into its own crate so any Solana project can use Jito bundles without pulling in an entire web service.

## What Are Jito Bundles?

Solana transactions normally go through the standard leader pipeline. Jito Bundles let you submit up to **5 transactions** that are guaranteed to execute **atomically and in order** — either all of them land in the same slot, or none of them do. This is critical for DeFi operations where transaction ordering matters (think: arbitrage, liquidations, or multi-step swaps).

The catch: you have to **tip** the Jito validator for this privilege. The tip is a simple SOL transfer to one of 8 pre-defined Jito tip accounts.

## How the Architecture Works

```
You (caller)
    │
    ▼
┌────────────────┐
│  JitoBundler   │  ← The facade — the only struct you interact with
│                │
│  1. fetch_tip  │  → Gets current tip floor from Jito API
│  2. build      │  → Constructs signed transactions
│  3. simulate   │  → Tests them against Helius/RPC
│  4. send       │  → Submits to Jito endpoints
│  5. confirm    │  → Polls until landed on-chain
└────────────────┘
```

Under the hood, `JitoBundler` is the single struct that owns all resources (HTTP client, RPC client, config). Its implementation is **split across multiple files** — each file adds a group of related methods via separate `impl JitoBundler` blocks.

### The Three Layers

**Layer 1: Configuration** (`config/`)
- `JitoConfig` — builder pattern struct with `with_network()`, `with_uuid()`, etc.
- `Network` — `Mainnet` (5 geographic endpoints) or `Custom` (your own URLs)
- `TipStrategy` — `Fixed(lamports)`, `FetchFloor`, or `FetchFloorWithCap { min, max }`
- `ConfirmPolicy` — how many times to poll and how long to wait between polls

**Layer 2: Bundle Building** (`bundler/`)
- `BundleBuilderInputs` — the 7 parameters needed to build a bundle
- `BundleBuilder` — mutable state during construction (compaction, tip placement, LUT validation, compile + sign)
- `BuiltBundle` — the immutable output artifact (signed transactions + metadata)
- `BundleSlotView` — shared trait for querying slot occupancy (implemented by both builder and output)
- `TipMode` — enum tracking whether tip was placed as a separate tx or inline

**Layer 3: Client** (`client/`)
- `JitoBundler` — facade that orchestrates the full lifecycle
- `rpc.rs` — shared JSON-RPC utilities (send, parse, encode)
- `send.rs` — endpoint retry across 5 geographic Jito block engines
- `simulate.rs` — per-tx RPC simulation + atomic Helius `simulateBundle`
- `status.rs` — on-chain signature polling + Jito API status polling

**Shared RPC utilities** (`client/rpc.rs`) provide four reusable building blocks:
- `send_json_rpc_request()` — POST + body extraction
- `parse_json_rpc_result()` — JSON-RPC result/error extraction
- `encode_transactions_base64()` — serialize + base64
- `first_signature_base58()` — safe signature extraction (returns `Result` instead of panicking on missing signatures)

Two standalone helpers also exist:

- **TipHelper** (`tip.rs`) — Picks a random tip account from the 8 Jito accounts, creates the SOL transfer instruction, and fetches the current tip floor from Jito's API. `TipStrategy` controls whether the floor is used raw or capped.

- **TransactionAnalysis** (`analysis.rs`) — Diagnostic utility for post-mortem debugging: which accounts aren't in your LUTs, which transactions are oversized. Used automatically on compile failures.

## Data Flow: Build Pipeline

```
BundleBuilderInputs (7 fields)
    │
    ▼
BundleBuilder::build()
    │
    ├─ 1. compact_transactions()     Remove gaps, preserve order
    ├─ 2. validate count > 0         Reject empty bundles
    ├─ 3. apply_jitodont_front()     Frontrun protection account
    ├─ 4. Choose tip placement:
    │     count < 5 → append_tip_transaction()   (TipMode::SeparateTx)
    │     count = 5 → append_tip_instruction()   (TipMode::InlineLastTx)
    ├─ 5. validate_tip_not_in_luts() Only for inline mode
    └─ 6. build_versioned_transaction() per slot
          ├─ Prepend compute budget ix
          ├─ Compile v0 message (with or without LUTs)
          ├─ Sign with payer
          └─ Size check (≤ 1232 bytes)
    │
    ▼
BuiltBundle (immutable output)
    ├── transactions: Vec<VersionedTransaction>
    ├── tip_account: Pubkey
    ├── tip_lamports: u64
    ├── tip_mode: TipMode
    └── instruction_slots: BundleInstructionSlots
```

## The Subtle Rules That Took Time to Get Right

### The Separate Tip Transaction Rule

Jito allows max 5 transactions per bundle. If your bundle has fewer than 5 instructions, the tip goes into its own **separate transaction** compiled **without address lookup tables**. This avoids a subtle issue where the tip account might conflict with LUT entries.

If your bundle already has exactly 5 instructions (the max), there's no room for a separate tip tx, so the tip instruction gets appended to the last transaction inline.

The `TipMode` enum makes this explicit — you can inspect the built bundle to see which path was taken.

### Instruction Slot Compaction

`transactions_instructions` is a fixed 5-slot array, but callers may leave gaps. Before building, the library compacts the slots (removing gaps while preserving order) so the tip is always placed at the end of the bundle as intended.

### Tip Strategy Semantics

`FetchFloor` returns the raw floor (no clamping). If you want bounds, use `FetchFloorWithCap { min, max }`, which clamps the raw floor into your chosen range.

### The LUT Validation

Here's a non-obvious gotcha: if any of your address lookup tables contain a Jito tip account, the transaction will **fail at runtime**. Why? Because LUT lookups produce references with different writability semantics than what the tip transfer needs. The library validates this upfront and gives you a clear error instead of letting you waste SOL on a doomed bundle.

### Safe Signature Extraction

Early versions accessed `tx.signatures[0]` directly — a panic if the vec is empty. The library now uses `first_signature_base58()` everywhere, which returns `Result<String, JitoError::InvalidSignature>`. This rippled through `extract_signatures()` (now returns `Result`) and `simulate_per_transaction()`.

### Typed Tip Floor Errors

`compute_tip_floor_lamports()` used to silently coerce invalid values (NaN, negative, overflow) to 0 lamports. Now it returns `Result<u64, JitoError::TipFloorFetchFailed>` with specific messages for non-finite, negative, and out-of-range values. You find out about bad data from the Jito API immediately instead of tipping 0 and wondering why your bundle got rejected.

### The jitodontfront Mechanism

"jitodontfront" (Jito-don't-front) is frontrun protection. When enabled, the library adds a special pubkey to the first transaction's account list as a non-signer, non-writable account. This signals to Jito validators: "don't let anyone frontrun this bundle."

## Technology Choices

| Choice | Why |
|---|---|
| `thiserror` over `anyhow` | Typed errors. Callers can match on `JitoError::TooManyTransactions` vs `JitoError::SimulationFailed` instead of parsing strings |
| Own `SYSTEM_PROGRAM_ID` constant | `solana_sdk::system_program` is deprecated. Rather than adding `solana-system-interface`, we define the well-known pubkey directly |
| No `jito-rust-rpc` dependency | That crate uses `anyhow`, `serde_json::Value`, old deps. The ~100 LOC of useful RPC logic was cheaper to rewrite with proper types |
| Edition 2024 | Enables let-chains (`if let Some(x) = foo && condition {}`), which makes the bundle building code much cleaner |
| `BundleSlotView` trait | Eliminates method duplication between builder and output — both need `populated_count()` and `last_populated_index()` |
| Shared `rpc.rs` utilities | `send_json_rpc_request()`, `parse_json_rpc_result()`, `encode_transactions_base64()` are reused across send, simulate, and status |

## Code Structure

```
src/
├── lib.rs                       Re-exports 8 public modules + pub use JitoError
├── error.rs                     15 typed error variants
├── constants.rs                 Tip accounts, URLs, limits, SYSTEM_PROGRAM_ID
├── types.rs                     JSON-RPC types, BundleStatus, simulation types
├── tip.rs                       TipHelper (stateless utility with static methods)
├── analysis.rs                  TransactionAnalysis (stateless utility — size + LUT diagnostics)
├── config/
│   ├── jito.rs                  JitoConfig (builder pattern: with_network, with_helius_rpc_url, etc.)
│   ├── network.rs               Network enum (Mainnet / Custom URLs)
│   ├── confirm_policy.rs        ConfirmPolicy (max_attempts + interval_ms)
│   └── tip_strategy.rs          TipStrategy (Fixed / FetchFloor / FetchFloorWithCap)
├── bundler/
│   ├── types.rs                 BundleInstructionSlots, TipMode, BundleSlotView trait
│   ├── builder/
│   │   ├── types.rs             BundleBuilderInputs + BundleBuilder (mutable build state)
│   │   └── utils.rs             Build pipeline implementation
│   ├── bundle/
│   │   └── types.rs             BuiltBundle (immutable output artifact)
│   └── tests.rs                 16 unit tests for bundle building
└── client/
    ├── types.rs                 Private status response types
    ├── jito_bundler.rs          JitoBundler facade + BuildBundleOptions input struct
    ├── rpc.rs                   impl JitoBundler — shared JSON-RPC + signature utilities
    ├── send.rs                  impl JitoBundler — bundle submission with endpoint retry
    ├── simulate.rs              impl JitoBundler — RPC + Helius simulation
    └── status.rs                impl JitoBundler — landing confirmation polling
```

Notice how `send.rs`, `simulate.rs`, `status.rs`, and `rpc.rs` all add methods to the same `JitoBundler` struct via split `impl` blocks. This keeps files small and focused while keeping all resource access through `&self`.

The facade exposes a clean flow: `fetch_tip()` -> `build_bundle(BuildBundleOptions)` -> `send_and_confirm()`. The `BuildBundleOptions` input struct follows the Rust convention of using a struct when there are more than three parameters.

## Bugs We Ran Into

1. **`thiserror` source field magic**: Fields named `source` in `thiserror` enums get auto-treated as `#[source]`, but `String` doesn't implement `std::error::Error`. Renamed to `reason` everywhere.

2. **Solana crate version fragmentation**: You can't just write `solana-sdk = "2.3"` and expect everything to resolve. Different Solana crates have different latest 2.x versions: `solana-pubkey` is at 2.4.0, `solana-compute-budget-interface` is at 2.2.2. Pin exact versions.

3. **`allow_attributes = "deny"` strictness**: Our lint config denies ALL `#[allow(...)]` attributes without a `reason = "..."`. In test code, we use `#[allow(clippy::unwrap_used, reason = "test code")]`. In production code, we avoid `#[allow]` entirely and handle casts with explicit conditional logic.

4. **Deprecated `system_program` module**: Caught by a deprecation warning. The fix was defining our own `SYSTEM_PROGRAM_ID` constant via the `pubkey!` macro rather than pulling in yet another crate.

5. **Duplicate structs during refactoring**: The original code had `Bundle` and `BundleBuilder` with identical fields. We split them into `BundleBuilder` (mutable build state) and `BuiltBundle` (immutable output) with distinct field sets. The `BundleSlotView` trait provides shared slot-querying methods.

6. **Silent coercion vs typed errors**: `compute_tip_floor_lamports()` originally returned 0 for NaN/negative values. A 0 tip means your bundle gets deprioritized with no obvious error. Switching to `Result` with typed `TipFloorFetchFailed` errors catches bad API data immediately.

7. **Direct signature indexing**: `tx.signatures[0]` panics if there are no signatures. Extracting this into `first_signature_base58()` with a `Result` return made the entire send/simulate/status pipeline panic-free.

8. **`BundleResult` over-engineering**: The original had `success: bool`, `bundle_id: Option<String>`, `error: Option<String>`, `explorer_url: Option<String>`. Since errors already propagate via `Result<BundleResult, JitoError>`, the `BundleResult` itself only needs the success fields: `bundle_id: String`, `signatures`, `explorer_url: String`. Simpler struct, no impossible states.

9. **Polling timeout diagnostics**: `wait_for_landing_on_chain()` originally returned `ConfirmationTimeout` even when every single RPC poll failed (network issue, not timeout). Now it tracks `had_successful_poll` and returns `JitoError::Network` with the last RPC error when all polls fail — actionable error message instead of misleading "timeout".

## How Good Engineers Think About This

- **Facade pattern**: `JitoBundler` presents a simple API while hiding the complexity of sending, simulating, and confirming bundles. Users don't need to know about endpoint rotation or simulation strategies.

- **Typed errors over string errors**: Every failure mode has its own variant. This means you can write `match err { JitoError::ConfirmationTimeout { .. } => retry(), _ => fail() }` instead of `if err.to_string().contains("timeout")`.

- **Fail-fast validation**: Bundle size, LUT safety, and transaction size are all checked before any network calls. You find out about structural problems immediately, not after waiting for a failed simulation.

- **Defense in depth**: Even though we validate LUTs upfront, we still log comprehensive diagnostics when anything fails downstream. The `TransactionAnalysis` module exists purely for debugging — when a compilation or simulation fails, it tells you exactly which accounts weren't in your LUTs and which transactions were oversized.

- **Split impl blocks**: Rust lets you write `impl MyStruct` in multiple files. We use this to keep `JitoBundler`'s implementation organized by concern — sending, simulating, and status polling each live in their own file, but all access the struct's owned resources through `&self`. This avoids the anti-pattern of passing raw clients and config through static method parameters.

- **Input structs for clarity**: `BuildBundleOptions` and `BundleBuilderInputs` follow the convention of using a struct when a method has more than three parameters. The struct is fully destructured at the call site, ensuring every field is explicitly used — this makes it impossible to silently ignore a new field when the struct grows.

- **Flat flow methods**: The top-level `send_and_confirm` reads like a recipe: simulate -> send -> log -> wait -> interpret. Each step is a single named call. Interpretation logic (the match on landing status) is extracted into a private `interpret_landing_status` helper so the orchestrating method stays scannable without scrolling.

- **Builder vs Output separation**: `BundleBuilder` accumulates mutable state during construction; `BuiltBundle` is the sealed result. This prevents accidental mutation after build and makes the API's intent clear — once you have a `BuiltBundle`, it's ready for simulation and submission.

- **Shared RPC utilities**: `client/rpc.rs` extracts common JSON-RPC plumbing (send, parse, encode, signature extraction) so that `send.rs`, `simulate.rs`, and `status.rs` stay focused on their domain logic rather than duplicating HTTP/JSON handling.

- **Make impossible states unrepresentable**: `BundleResult` was redesigned from `{success: bool, bundle_id: Option, error: Option, explorer_url: Option}` to just `{bundle_id: String, signatures: Vec, explorer_url: String}`. Since the struct only exists inside `Ok(...)`, all fields are guaranteed present. No more checking `result.success` when you already handled the error.

- **Distinguish error causes**: `wait_for_landing_on_chain()` tracks whether any RPC poll succeeded. If all polls fail → `JitoError::Network` (check your RPC). If polls succeeded but none confirmed → `ConfirmationTimeout` (bundle might still land). Same timeout, different root cause, different action.

- **Offline tests**: The test suite now has `tests/offline/` with fast unit-level tests that don't need `.env` or network. CI runs these on every push. Live tests are gated behind `cfg(feature = "live-tests")` and only compile when you explicitly opt in.
````

## File: README.md
````markdown
<!-- repo-token-analysis-badges -->
![Claude](./repo-token-analysis/badges/claude.svg)
![GPT-4](./repo-token-analysis/badges/gpt4.svg)
![Gemini](./repo-token-analysis/badges/gemini.svg)
<!-- repo-token-analysis-badges -->

# jito-bundle

Standalone Rust library for submitting [Jito Bundles](https://docs.jito.wtf/) on Solana.

Submit up to 5 transactions that execute atomically and in order. The library handles tip calculation, bundle construction, simulation, endpoint retry, and on-chain confirmation.

Jito Bundle Explorer: [https://explorer.jito.wtf/](https://explorer.jito.wtf/)

## Features

- Atomic bundle construction from fixed-size `BundleInstructionSlots` input.
- Automatic tipping via `TipStrategy`.
- Endpoint retry across Jito mainnet regions.
- Optional Helius `simulateBundle` before sending.
- Optional `jitodontfront` account injection.
- Typed error handling via `JitoError`.

## Requirements

- Rust 1.85+ (edition 2024)
- Solana keypair with SOL for tips

## Usage

### Via `JitoBundler` facade (recommended)

```rust,no_run
use jito_bundle::bundler::types::BundleInstructionSlots;
use jito_bundle::client::jito_bundler::{BuildBundleOptions, JitoBundler};
use jito_bundle::config::jito::JitoConfig;
use solana_sdk::signature::Keypair;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = JitoConfig::new("https://api.mainnet-beta.solana.com")
        .with_helius_rpc_url("https://mainnet.helius-rpc.com/?api-key=...")
        .with_uuid("your-jito-uuid");
    let bundler = JitoBundler::new(config)?;

    let payer = Keypair::new();
    let slots: BundleInstructionSlots = [None, None, None, None, None];

    let bundle = bundler
        .build_bundle(BuildBundleOptions {
            payer: &payer,
            transactions_instructions: slots,
            lookup_tables: &[],
        })
        .await?;

    let result = bundler.send_and_confirm(&bundle).await?;
    println!("bundle landed: {}", result.bundle_id);
    Ok(())
}
```

### Direct construction via `BundleBuilder`

```rust,no_run
use jito_bundle::bundler::builder::types::{BundleBuilder, BundleBuilderInputs};
use jito_bundle::bundler::types::BundleInstructionSlots;
use jito_bundle::constants::DEFAULT_COMPUTE_UNIT_LIMIT;
use solana_sdk::hash::Hash;
use solana_sdk::signature::Keypair;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let payer = Keypair::new();
    let slots: BundleInstructionSlots = [None, None, None, None, None];

    let bundle = BundleBuilder::build(BundleBuilderInputs {
        payer: &payer,
        transactions_instructions: slots,
        lookup_tables: &[],
        recent_blockhash: Hash::default(),
        tip_lamports: 100_000,
        jitodontfront_pubkey: None,
        compute_unit_limit: DEFAULT_COMPUTE_UNIT_LIMIT,
    })?;

    println!("built {} transactions", bundle.transactions.len());
    Ok(())
}
```

## Public Types

- `BundleInstructionSlots`: `type BundleInstructionSlots = [Option<Vec<Instruction>>; 5]`
- `BuiltBundle`:
  - `transactions: Vec<VersionedTransaction>`
  - `tip_account: Pubkey`
  - `tip_lamports: u64`
  - `tip_mode: TipMode`

## Configuration

`JitoConfig` uses a builder pattern:

| Method | Description |
|---|---|
| `new(rpc_url)` | Required Solana RPC URL |
| `.with_network(Network)` | `Mainnet` (default) or `Custom { block_engine_url, tip_floor_url }` |
| `.with_helius_rpc_url(url)` | Enable Helius simulation before send |
| `.with_uuid(uuid)` | Jito authentication UUID |
| `.with_tip_strategy(TipStrategy)` | `Fixed`, `FetchFloor`, or `FetchFloorWithCap` |
| `.with_confirm_policy(ConfirmPolicy)` | Confirmation polling config |
| `.with_jitodontfront(pubkey)` | Optional frontrun protection account |
| `.with_compute_unit_limit(u32)` | Per-transaction compute budget |

## Corner Cases and Rules We Enforce

1. Bundle size is hard-capped at 5 transactions (Jito protocol limit).
2. Empty bundles are rejected (`InvalidBundleSize`).
3. Sparse input slots are compacted before build (gaps removed, order preserved).
4. Tip is always inserted.
5. If bundle has fewer than 5 transactions, tip is added as a separate transaction.
6. If bundle already has 5 transactions, tip is appended inline to the last transaction.
7. Tip instruction is always the last instruction of the final transaction.
8. If the chosen tip account appears in a lookup table for inline mode, build rejects it (prevents Jito runtime failure).
9. `jitodontfront` is enforced only in the first transaction; existing `jitodontfront*` accounts are removed by prefix match (suffix may vary).
10. `jitodontfront` account duplication is prevented.
11. Compute budget instruction is prepended to every transaction.
12. Every compiled transaction is size-checked against Solana max transaction size.

## Error Handling

All operations return `Result<T, JitoError>`.

```rust
use jito_bundle::JitoError;

fn handle(result: Result<(), JitoError>) {
    match result {
        Err(JitoError::ConfirmationTimeout { attempts }) => {
            println!("timed out after {attempts}");
        }
        Err(JitoError::SimulationFailed { details }) => {
            println!("simulation failed: {details}");
        }
        Err(e) => {
            println!("other error: {e}");
        }
        Ok(()) => {}
    }
}
```

## Testing

```bash
# Unit tests
cargo test

# Integration tests (requires .env with live credentials)
cargo test --features live-tests -- --ignored
```

## License

MIT
````
