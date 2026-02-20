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
  badges/
    claude.svg
    gemini.svg
    gpt4.svg
  .tokenignore.default
  dependencies.md
  history.json
  report.json
  tree.md
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

## File: .tokenignore
`````
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
`````

## File: .config/nextest.toml
`````toml
[profile.default]
success-output = "immediate"
failure-output = "immediate"
status-level = "pass"
final-status-level = "all"
slow-timeout = { period = "60s", terminate-after = 3 }
`````

## File: .github/workflows/ci.yml
`````yaml
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
`````

## File: .github/workflows/token-analysis.yml
`````yaml
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
`````

## File: repo-token-analysis/badges/claude.svg
`````
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="144" height="20" role="img" aria-label="claude: 16.33% context"><title>claude: 16.33% context</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="r"><rect width="144" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="45" height="20" fill="#555"/><rect x="45" width="99" height="20" fill="#4c1"/><rect width="144" height="20" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="235" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="350">claude</text><text x="235" y="140" transform="scale(.1)" fill="#fff" textLength="350">claude</text><text aria-hidden="true" x="935" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="890">16.33% context</text><text x="935" y="140" transform="scale(.1)" fill="#fff" textLength="890">16.33% context</text></g></svg>
`````

## File: repo-token-analysis/badges/gemini.svg
`````
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="138" height="20" role="img" aria-label="gemini: 3.27% context"><title>gemini: 3.27% context</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="r"><rect width="138" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="47" height="20" fill="#555"/><rect x="47" width="91" height="20" fill="#4c1"/><rect width="138" height="20" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="245" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="370">gemini</text><text x="245" y="140" transform="scale(.1)" fill="#fff" textLength="370">gemini</text><text aria-hidden="true" x="915" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="810">3.27% context</text><text x="915" y="140" transform="scale(.1)" fill="#fff" textLength="810">3.27% context</text></g></svg>
`````

## File: repo-token-analysis/badges/gpt4.svg
`````
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="134" height="20" role="img" aria-label="gpt4: 25.52% context"><title>gpt4: 25.52% context</title><linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="r"><rect width="134" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#r)"><rect width="35" height="20" fill="#555"/><rect x="35" width="99" height="20" fill="#4c1"/><rect width="134" height="20" fill="url(#s)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="185" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="250">gpt4</text><text x="185" y="140" transform="scale(.1)" fill="#fff" textLength="250">gpt4</text><text aria-hidden="true" x="835" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="890">25.52% context</text><text x="835" y="140" transform="scale(.1)" fill="#fff" textLength="890">25.52% context</text></g></svg>
`````

## File: repo-token-analysis/.tokenignore.default
`````
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
`````

## File: repo-token-analysis/dependencies.md
`````markdown
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
`````

## File: repo-token-analysis/history.json
`````json
[
  {
    "timestamp": "2026-02-17T16:07:27.610Z",
    "tiktoken_total": 32663
  }
]
`````

## File: repo-token-analysis/report.json
`````json
{
  "timestamp": "2026-02-17T16:07:27.610Z",
  "tokenizers": {
    "code2prompt": 92223,
    "repomix": 34954,
    "tiktoken": 32663
  },
  "models": [
    {
      "name": "claude",
      "context_size": 200000,
      "tiktoken_tokens": 32663,
      "percent_used": 16.33,
      "status": "green"
    },
    {
      "name": "gpt4",
      "context_size": 128000,
      "tiktoken_tokens": 32663,
      "percent_used": 25.52,
      "status": "green"
    },
    {
      "name": "gemini",
      "context_size": 1000000,
      "tiktoken_tokens": 32663,
      "percent_used": 3.27,
      "status": "green"
    }
  ],
  "threshold_percent": 75,
  "diagnostics": {
    "top_offenders": [
      {
        "file": "src/bundler/tests.rs",
        "tokens": 4427,
        "percent_of_smallest_model": 3.46
      },
      {
        "file": "FOR_USER.md",
        "tokens": 3923,
        "percent_of_smallest_model": 3.06
      },
      {
        "file": "CLAUDE.md",
        "tokens": 2861,
        "percent_of_smallest_model": 2.24
      },
      {
        "file": "src/bundler/builder/utils.rs",
        "tokens": 1913,
        "percent_of_smallest_model": 1.49
      },
      {
        "file": "src/tip.rs",
        "tokens": 1857,
        "percent_of_smallest_model": 1.45
      },
      {
        "file": "src/client/status.rs",
        "tokens": 1545,
        "percent_of_smallest_model": 1.21
      },
      {
        "file": "tests/common/mod.rs",
        "tokens": 1513,
        "percent_of_smallest_model": 1.18
      },
      {
        "file": "src/client/jito_bundler.rs",
        "tokens": 1462,
        "percent_of_smallest_model": 1.14
      },
      {
        "file": "src/types.rs",
        "tokens": 1441,
        "percent_of_smallest_model": 1.13
      },
      {
        "file": "README.md",
        "tokens": 1341,
        "percent_of_smallest_model": 1.05
      }
    ],
    "suggestions": [],
    "trend": {
      "previous_total": null,
      "delta": null,
      "delta_percent": null,
      "fastest_growing": []
    }
  }
}
`````

## File: repo-token-analysis/tree.md
`````markdown
<directory>workspace</directory>

<source-tree>
  workspace
├── AGENTS.md
├── CLAUDE.md
├── Cargo.lock
├── Cargo.toml
├── FOR_USER.md
├── README.md
├── repo-token-analysis
├── scripts
│   └── tests.sh
├── src
│   ├── analysis.rs
│   ├── bundler
│   │   ├── builder
│   │   │   ├── mod.rs
│   │   │   ├── types.rs
│   │   │   └── utils.rs
│   │   ├── bundle
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   ├── mod.rs
│   │   ├── tests.rs
│   │   └── types.rs
│   ├── client
│   │   ├── jito_bundler.rs
│   │   ├── mod.rs
│   │   ├── rpc.rs
│   │   ├── send.rs
│   │   ├── simulate.rs
│   │   ├── status.rs
│   │   └── types.rs
│   ├── config
│   │   ├── confirm_policy.rs
│   │   ├── jito.rs
│   │   ├── mod.rs
│   │   ├── network.rs
│   │   └── tip_strategy.rs
│   ├── constants.rs
│   ├── error.rs
│   ├── lib.rs
│   ├── tip.rs
│   └── types.rs
└── tests
    ├── build
    │   ├── memo_bundle.rs
    │   └── mod.rs
    ├── common
    │   └── mod.rs
    ├── main.rs
    ├── offline
    │   └── mod.rs
    ├── send
    │   ├── memo_send.rs
    │   └── mod.rs
    └── simulate
        ├── helius_simulation.rs
        └── mod.rs

</source-tree>

<files>
      <file path="AGENTS.md">
        ```md
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

```
      </file>
      <file path="CLAUDE.md">
        ```md
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

```
      </file>
      <file path="Cargo.lock">
        ```lock
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "adler2"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "320119579fcad9c21884f5c4861d16174d0e06250625266f50fe6898340abefa"

[[package]]
name = "agave-feature-set"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d52a2c365c0245cbb8959de725fc2b44c754b673fdf34c9a7f9d4a25c35a7bf1"
dependencies = [
 "ahash",
 "solana-epoch-schedule",
 "solana-hash",
 "solana-pubkey",
 "solana-sha256-hasher",
 "solana-svm-feature-set",
]

[[package]]
name = "ahash"
version = "0.8.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5a15f179cd60c4584b8a8c596927aadc462e27f2ca70c04e0071964a73ba7a75"
dependencies = [
 "cfg-if",
 "getrandom 0.3.4",
 "once_cell",
 "version_check",
 "zerocopy",
]

[[package]]
name = "aho-corasick"
version = "1.1.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ddd31a130427c27518df266943a5308ed92d4b226cc639f5a8f1002816174301"
dependencies = [
 "memchr",
]

[[package]]
name = "alloc-no-stdlib"
version = "2.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cc7bb162ec39d46ab1ca8c77bf72e890535becd1751bb45f64c597edb4c8c6b3"

[[package]]
name = "alloc-stdlib"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "94fb8275041c72129eb51b7d0322c29b8387a0386127718b096429201a5d6ece"
dependencies = [
 "alloc-no-stdlib",
]

[[package]]
name = "anyhow"
version = "1.0.100"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a23eb6b1614318a8071c9b2521f36b424b2c83db5eb3a0fead4a6c0809af6e61"

[[package]]
name = "ark-bn254"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a22f4561524cd949590d78d7d4c5df8f592430d221f7f3c9497bbafd8972120f"
dependencies = [
 "ark-ec",
 "ark-ff",
 "ark-std",
]

[[package]]
name = "ark-ec"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "defd9a439d56ac24968cca0571f598a61bc8c55f71d50a89cda591cb750670ba"
dependencies = [
 "ark-ff",
 "ark-poly",
 "ark-serialize",
 "ark-std",
 "derivative",
 "hashbrown 0.13.2",
 "itertools 0.10.5",
 "num-traits",
 "zeroize",
]

[[package]]
name = "ark-ff"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ec847af850f44ad29048935519032c33da8aa03340876d351dfab5660d2966ba"
dependencies = [
 "ark-ff-asm",
 "ark-ff-macros",
 "ark-serialize",
 "ark-std",
 "derivative",
 "digest 0.10.7",
 "itertools 0.10.5",
 "num-bigint 0.4.6",
 "num-traits",
 "paste",
 "rustc_version",
 "zeroize",
]

[[package]]
name = "ark-ff-asm"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3ed4aa4fe255d0bc6d79373f7e31d2ea147bcf486cba1be5ba7ea85abdb92348"
dependencies = [
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "ark-ff-macros"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7abe79b0e4288889c4574159ab790824d0033b9fdcb2a112a3182fac2e514565"
dependencies = [
 "num-bigint 0.4.6",
 "num-traits",
 "proc-macro2",
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "ark-poly"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d320bfc44ee185d899ccbadfa8bc31aab923ce1558716e1997a1e74057fe86bf"
dependencies = [
 "ark-ff",
 "ark-serialize",
 "ark-std",
 "derivative",
 "hashbrown 0.13.2",
]

[[package]]
name = "ark-serialize"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "adb7b85a02b83d2f22f89bd5cac66c9c89474240cb6207cb1efc16d098e822a5"
dependencies = [
 "ark-serialize-derive",
 "ark-std",
 "digest 0.10.7",
 "num-bigint 0.4.6",
]

[[package]]
name = "ark-serialize-derive"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ae3281bc6d0fd7e549af32b52511e1302185bd688fd3359fa36423346ff682ea"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "ark-std"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "94893f1e0c6eeab764ade8dc4c0db24caf4fe7cbbaafc0eba0a9030f447b5185"
dependencies = [
 "num-traits",
 "rand 0.8.5",
]

[[package]]
name = "arrayref"
version = "0.3.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "76a2e8124351fda1ef8aaaa3bbd7ebbcb486bbcd4225aca0aa0d84bb2db8fecb"

[[package]]
name = "arrayvec"
version = "0.7.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7c02d123df017efcdfbd739ef81735b36c5ba83ec3c59c80a9d7ecc718f92e50"

[[package]]
name = "asn1-rs"
version = "0.5.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7f6fd5ddaf0351dff5b8da21b2fb4ff8e08ddd02857f0bf69c47639106c0fff0"
dependencies = [
 "asn1-rs-derive",
 "asn1-rs-impl",
 "displaydoc",
 "nom",
 "num-traits",
 "rusticata-macros",
 "thiserror 1.0.69",
 "time",
]

[[package]]
name = "asn1-rs-derive"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "726535892e8eae7e70657b4c8ea93d26b8553afb1ce617caee529ef96d7dee6c"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
 "synstructure 0.12.6",
]

[[package]]
name = "asn1-rs-impl"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2777730b2039ac0f95f093556e61b6d26cebed5393ca6f152717777cec3a42ed"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "async-channel"
version = "1.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "81953c529336010edd6d8e358f886d9581267795c61b19475b71314bffa46d35"
dependencies = [
 "concurrent-queue",
 "event-listener 2.5.3",
 "futures-core",
]

[[package]]
name = "async-compression"
version = "0.4.37"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d10e4f991a553474232bc0a31799f6d24b034a84c0971d80d2e2f78b2e576e40"
dependencies = [
 "compression-codecs",
 "compression-core",
 "pin-project-lite",
 "tokio",
]

[[package]]
name = "async-lock"
version = "3.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "290f7f2596bd5b78a9fec8088ccd89180d7f9f55b94b0576823bbbdc72ee8311"
dependencies = [
 "event-listener 5.4.1",
 "event-listener-strategy",
 "pin-project-lite",
]

[[package]]
name = "async-trait"
version = "0.1.89"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9035ad2d096bed7955a320ee7e2230574d28fd3c3a0f186cbea1ff3c7eed5dbb"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "atomic-waker"
version = "1.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1505bd5d3d116872e7271a6d4e16d81d0c8570876c8de68093a09ac269d8aac0"

[[package]]
name = "atty"
version = "0.2.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8"
dependencies = [
 "hermit-abi 0.1.19",
 "libc",
 "winapi",
]

[[package]]
name = "autocfg"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c08606f8c3cbf4ce6ec8e28fb0014a2c086708fe954eaa885384a6165172e7e8"

[[package]]
name = "base64"
version = "0.12.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3441f0f7b02788e948e47f457ca01f1d7e6d92c693bc132c22b087d3141c03ff"

[[package]]
name = "base64"
version = "0.13.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9e1b586273c5702936fe7b7d6896644d8be71e6314cfe09d3167c95f712589e8"

[[package]]
name = "base64"
version = "0.22.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "72b3254f16251a8381aa12e40e3c4d2f0199f8c6508fbecb9d91f575e0fbb8c6"

[[package]]
name = "bincode"
version = "1.3.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b1f45e9417d87227c7a56d22e471c6206462cba514c7590c09aff4cf6d1ddcad"
dependencies = [
 "serde",
]

[[package]]
name = "bitflags"
version = "2.10.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "812e12b5285cc515a9c72a5c1d3b6d46a19dac5acfef5265968c166106e31dd3"
dependencies = [
 "serde_core",
]

[[package]]
name = "blake3"
version = "1.8.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2468ef7d57b3fb7e16b576e8377cdbde2320c60e1491e961d11da40fc4f02a2d"
dependencies = [
 "arrayref",
 "arrayvec",
 "cc",
 "cfg-if",
 "constant_time_eq",
 "cpufeatures",
 "digest 0.10.7",
]

[[package]]
name = "block-buffer"
version = "0.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4152116fd6e9dadb291ae18fc1ec3575ed6d84c29642d97890f4b4a3417297e4"
dependencies = [
 "generic-array",
]

[[package]]
name = "block-buffer"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3078c7629b62d3f0439517fa394996acacc5cbc91c5a20d8c658e77abd503a71"
dependencies = [
 "generic-array",
]

[[package]]
name = "borsh"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "115e54d64eb62cdebad391c19efc9dce4981c690c85a33a12199d99bb9546fee"
dependencies = [
 "borsh-derive 0.10.4",
 "hashbrown 0.13.2",
]

[[package]]
name = "borsh"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d1da5ab77c1437701eeff7c88d968729e7766172279eab0676857b3d63af7a6f"
dependencies = [
 "borsh-derive 1.6.0",
 "cfg_aliases",
]

[[package]]
name = "borsh-derive"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "831213f80d9423998dd696e2c5345aba6be7a0bd8cd19e31c5243e13df1cef89"
dependencies = [
 "borsh-derive-internal",
 "borsh-schema-derive-internal",
 "proc-macro-crate 0.1.5",
 "proc-macro2",
 "syn 1.0.109",
]

[[package]]
name = "borsh-derive"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0686c856aa6aac0c4498f936d7d6a02df690f614c03e4d906d1018062b5c5e2c"
dependencies = [
 "once_cell",
 "proc-macro-crate 3.4.0",
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "borsh-derive-internal"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "65d6ba50644c98714aa2a70d13d7df3cd75cd2b523a2b452bf010443800976b3"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "borsh-schema-derive-internal"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "276691d96f063427be83e6692b86148e488ebba9f48f77788724ca027ba3b6d4"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "brotli"
version = "8.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4bd8b9603c7aa97359dbd97ecf258968c95f3adddd6db2f7e7a5bef101c84560"
dependencies = [
 "alloc-no-stdlib",
 "alloc-stdlib",
 "brotli-decompressor",
]

[[package]]
name = "brotli-decompressor"
version = "5.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "874bb8112abecc98cbd6d81ea4fa7e94fb9449648c93cc89aa40c81c24d7de03"
dependencies = [
 "alloc-no-stdlib",
 "alloc-stdlib",
]

[[package]]
name = "bs58"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bf88ba1141d185c399bee5288d850d63b8369520c1eafc32a0430b5b6c287bf4"
dependencies = [
 "tinyvec",
]

[[package]]
name = "bumpalo"
version = "3.19.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5dd9dc738b7a8311c7ade152424974d8115f2cdad61e8dab8dac9f2362298510"

[[package]]
name = "bv"
version = "0.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8834bb1d8ee5dc048ee3124f2c7c1afcc6bc9aed03f11e9dfd8c69470a5db340"
dependencies = [
 "feature-probe",
 "serde",
]

[[package]]
name = "bytemuck"
version = "1.24.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1fbdf580320f38b612e485521afda1ee26d10cc9884efaaa750d383e13e3c5f4"
dependencies = [
 "bytemuck_derive",
]

[[package]]
name = "bytemuck_derive"
version = "1.10.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f9abbd1bc6865053c427f7198e6af43bfdedc55ab791faed4fbd361d789575ff"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "byteorder"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1fd0f2584146f6f2ef48085050886acf353beff7305ebd1ae69500e27c67f64b"

[[package]]
name = "bytes"
version = "1.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e748733b7cbc798e1434b6ac524f0c1ff2ab456fe201501e6497c8417a4fc33"
dependencies = [
 "serde",
]

[[package]]
name = "caps"
version = "0.5.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fd1ddba47aba30b6a889298ad0109c3b8dcb0e8fc993b459daa7067d46f865e0"
dependencies = [
 "libc",
]

[[package]]
name = "cc"
version = "1.2.54"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6354c81bbfd62d9cfa9cb3c773c2b7b2a3a482d569de977fd0e961f6e7c00583"
dependencies = [
 "find-msvc-tools",
 "jobserver",
 "libc",
 "shlex",
]

[[package]]
name = "cesu8"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6d43a04d8753f35258c91f8ec639f792891f748a1edbd759cf1dcea3382ad83c"

[[package]]
name = "cfg-if"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801"

[[package]]
name = "cfg_aliases"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "613afe47fcd5fac7ccf1db93babcb082c5994d996f20b8b159f2ad1658eb5724"

[[package]]
name = "cfg_eval"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "45565fc9416b9896014f5732ac776f810ee53a66730c17e4020c3ec064a8f88f"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "chrono"
version = "0.4.43"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fac4744fb15ae8337dc853fee7fb3f4e48c0fbaa23d0afe49c447b4fab126118"
dependencies = [
 "num-traits",
]

[[package]]
name = "combine"
version = "4.6.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ba5a308b75df32fe02788e748662718f03fde005016435c444eea572398219fd"
dependencies = [
 "bytes",
 "memchr",
]

[[package]]
name = "compression-codecs"
version = "0.4.36"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "00828ba6fd27b45a448e57dbfe84f1029d4c9f26b368157e9a448a5f49a2ec2a"
dependencies = [
 "brotli",
 "compression-core",
 "flate2",
 "memchr",
]

[[package]]
name = "compression-core"
version = "0.4.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "75984efb6ed102a0d42db99afb6c1948f0380d1d91808d5529916e6c08b49d8d"

[[package]]
name = "concurrent-queue"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4ca0197aee26d1ae37445ee532fefce43251d24cc7c166799f4d46817f1d3973"
dependencies = [
 "crossbeam-utils",
]

[[package]]
name = "console"
version = "0.15.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "054ccb5b10f9f2cbf51eb355ca1d05c2d279ce1804688d0db74b4733a5aeafd8"
dependencies = [
 "encode_unicode",
 "libc",
 "once_cell",
 "unicode-width",
 "windows-sys 0.59.0",
]

[[package]]
name = "console_error_panic_hook"
version = "0.1.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a06aeb73f470f66dcdbf7223caeebb85984942f22f1adb2a088cf9668146bbbc"
dependencies = [
 "cfg-if",
 "wasm-bindgen",
]

[[package]]
name = "console_log"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e89f72f65e8501878b8a004d5a1afb780987e2ce2b4532c562e367a72c57499f"
dependencies = [
 "log",
 "web-sys",
]

[[package]]
name = "constant_time_eq"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3d52eff69cd5e647efe296129160853a42795992097e8af39800e1060caeea9b"

[[package]]
name = "core-foundation"
version = "0.9.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "91e195e091a93c46f7102ec7818a2aa394e1e1771c3ab4825963fa03e45afb8f"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "core-foundation"
version = "0.10.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b2a6cd9ae233e7f62ba4e9353e81a88df7fc8a5987b8d445b4d90c879bd156f6"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "core-foundation-sys"
version = "0.8.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "773648b94d0e5d620f64f280777445740e61fe701025087ec8b57f45c791888b"

[[package]]
name = "cpufeatures"
version = "0.2.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "59ed5838eebb26a2bb2e58f6d5b5316989ae9d08bab10e0e6d103e656d1b0280"
dependencies = [
 "libc",
]

[[package]]
name = "crc32fast"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9481c1c90cbf2ac953f07c8d4a58aa3945c425b7185c9154d67a65e4230da511"
dependencies = [
 "cfg-if",
]

[[package]]
name = "crossbeam-channel"
version = "0.5.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "82b8f8f868b36967f9606790d1903570de9ceaf870a7bf9fbbd3016d636a2cb2"
dependencies = [
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-deque"
version = "0.8.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9dd111b7b7f7d55b72c0a6ae361660ee5853c9af73f70c3c2ef6858b950e2e51"
dependencies = [
 "crossbeam-epoch",
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-epoch"
version = "0.9.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5b82ac4a3c2ca9c3460964f020e1402edd5753411d7737aa39c3714ad1b5420e"
dependencies = [
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-utils"
version = "0.8.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d0a5c400df2834b80a4c3327b3aad3a4c4cd4de0629063962b03235697506a28"

[[package]]
name = "crunchy"
version = "0.2.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "460fbee9c2c2f33933d720630a6a0bac33ba7053db5344fac858d4b8952d77d5"

[[package]]
name = "crypto-common"
version = "0.1.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "78c8292055d1c1df0cce5d180393dc8cce0abec0a7102adb6c7b1eef6016d60a"
dependencies = [
 "generic-array",
 "typenum",
]

[[package]]
name = "crypto-mac"
version = "0.8.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b584a330336237c1eecd3e94266efb216c56ed91225d634cb2991c5f3fd1aeab"
dependencies = [
 "generic-array",
 "subtle",
]

[[package]]
name = "curve25519-dalek"
version = "3.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0b9fdf9972b2bd6af2d913799d9ebc165ea4d2e65878e329d9c6b372c4491b61"
dependencies = [
 "byteorder",
 "digest 0.9.0",
 "rand_core 0.5.1",
 "subtle",
 "zeroize",
]

[[package]]
name = "curve25519-dalek"
version = "4.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "97fb8b7c4503de7d6ae7b42ab72a5a59857b4c937ec27a3d4539dba95b5ab2be"
dependencies = [
 "cfg-if",
 "cpufeatures",
 "curve25519-dalek-derive",
 "digest 0.10.7",
 "fiat-crypto",
 "rand_core 0.6.4",
 "rustc_version",
 "subtle",
 "zeroize",
]

[[package]]
name = "curve25519-dalek-derive"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f46882e17999c6cc590af592290432be3bce0428cb0d5f8b6715e4dc7b383eb3"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "darling"
version = "0.21.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9cdf337090841a411e2a7f3deb9187445851f91b309c0c0a29e05f74a00a48c0"
dependencies = [
 "darling_core",
 "darling_macro",
]

[[package]]
name = "darling_core"
version = "0.21.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1247195ecd7e3c85f83c8d2a366e4210d588e802133e1e355180a9870b517ea4"
dependencies = [
 "fnv",
 "ident_case",
 "proc-macro2",
 "quote",
 "strsim",
 "syn 2.0.114",
]

[[package]]
name = "darling_macro"
version = "0.21.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d38308df82d1080de0afee5d069fa14b0326a88c14f15c5ccda35b4a6c414c81"
dependencies = [
 "darling_core",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "dashmap"
version = "5.5.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "978747c1d849a7d2ee5e8adc0159961c48fb7e5db2f06af6723b80123bb53856"
dependencies = [
 "cfg-if",
 "hashbrown 0.14.5",
 "lock_api",
 "once_cell",
 "parking_lot_core",
]

[[package]]
name = "data-encoding"
version = "2.10.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d7a1e2f27636f116493b8b860f5546edb47c8d8f8ea73e1d2a20be88e28d1fea"

[[package]]
name = "der-parser"
version = "8.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dbd676fbbab537128ef0278adb5576cf363cff6aa22a7b24effe97347cfab61e"
dependencies = [
 "asn1-rs",
 "displaydoc",
 "nom",
 "num-bigint 0.4.6",
 "num-traits",
 "rusticata-macros",
]

[[package]]
name = "deranged"
version = "0.5.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ececcb659e7ba858fb4f10388c250a7252eb0a27373f1a72b8748afdd248e587"
dependencies = [
 "powerfmt",
]

[[package]]
name = "derivation-path"
version = "0.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6e5c37193a1db1d8ed868c03ec7b152175f26160a5b740e5e484143877e0adf0"

[[package]]
name = "derivative"
version = "2.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fcc3dd5e9e9c0b295d6e1e4d811fb6f157d5ffd784b8d202fc62eac8035a770b"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
]

[[package]]
name = "digest"
version = "0.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3dd60d1080a57a05ab032377049e0591415d2b31afd7028356dbf3cc6dcb066"
dependencies = [
 "generic-array",
]

[[package]]
name = "digest"
version = "0.10.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9ed9a281f7bc9b7576e61468ba615a66a5c8cfdff42420a70aa82701a3b1e292"
dependencies = [
 "block-buffer 0.10.4",
 "crypto-common",
 "subtle",
]

[[package]]
name = "displaydoc"
version = "0.2.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "97369cbbc041bc366949bc74d34658d6cda5621039731c6310521892a3a20ae0"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "dlopen2"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "09b4f5f101177ff01b8ec4ecc81eead416a8aa42819a2869311b3420fa114ffa"
dependencies = [
 "dlopen2_derive",
 "libc",
 "once_cell",
 "winapi",
]

[[package]]
name = "dlopen2_derive"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a6cbae11b3de8fce2a456e8ea3dada226b35fe791f0dc1d360c0941f0bb681f3"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "dotenvy"
version = "0.15.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1aaf95b3e5c8f23aa320147307562d361db0ae0d51242340f558153b4eb2439b"

[[package]]
name = "ed25519"
version = "1.5.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "91cff35c70bba8a626e3185d8cd48cc11b5437e1a5bcd15b9b5fa3c64b6dfee7"
dependencies = [
 "signature",
]

[[package]]
name = "ed25519-dalek"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c762bae6dcaf24c4c84667b8579785430908723d5c889f469d76a41d59cc7a9d"
dependencies = [
 "curve25519-dalek 3.2.0",
 "ed25519",
 "rand 0.7.3",
 "serde",
 "sha2 0.9.9",
 "zeroize",
]

[[package]]
name = "ed25519-dalek-bip32"
version = "0.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9d2be62a4061b872c8c0873ee4fc6f101ce7b889d039f019c5fa2af471a59908"
dependencies = [
 "derivation-path",
 "ed25519-dalek",
 "hmac 0.12.1",
 "sha2 0.10.9",
]

[[package]]
name = "either"
version = "1.15.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "48c757948c5ede0e46177b7add2e67155f70e33c07fea8284df6576da70b3719"

[[package]]
name = "encode_unicode"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34aa73646ffb006b8f5147f3dc182bd4bcb190227ce861fc4a4844bf8e3cb2c0"

[[package]]
name = "encoding_rs"
version = "0.8.35"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "75030f3c4f45dafd7586dd6780965a8c7e8e285a5ecb86713e63a79c5b2766f3"
dependencies = [
 "cfg-if",
]

[[package]]
name = "env_logger"
version = "0.9.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a12e6657c4c97ebab115a42dcee77225f7f482cdd841cf7088c657a42e9e00e7"
dependencies = [
 "atty",
 "humantime",
 "log",
 "regex",
 "termcolor",
]

[[package]]
name = "equivalent"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "877a4ace8713b0bcf2a4e7eec82529c029f1d0619886d18145fea96c3ffe5c0f"

[[package]]
name = "errno"
version = "0.3.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "39cab71617ae0d63f51a36d69f866391735b51691dbda63cf6f96d042b63efeb"
dependencies = [
 "libc",
 "windows-sys 0.61.2",
]

[[package]]
name = "event-listener"
version = "2.5.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0206175f82b8d6bf6652ff7d71a1e27fd2e4efde587fd368662814d6ec1d9ce0"

[[package]]
name = "event-listener"
version = "5.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e13b66accf52311f30a0db42147dadea9850cb48cd070028831ae5f5d4b856ab"
dependencies = [
 "concurrent-queue",
 "parking",
 "pin-project-lite",
]

[[package]]
name = "event-listener-strategy"
version = "0.5.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8be9f3dfaaffdae2972880079a491a1a8bb7cbed0b8dd7a347f668b4150a3b93"
dependencies = [
 "event-listener 5.4.1",
 "pin-project-lite",
]

[[package]]
name = "fastbloom"
version = "0.14.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4e7f34442dbe69c60fe8eaf58a8cafff81a1f278816d8ab4db255b3bef4ac3c4"
dependencies = [
 "getrandom 0.3.4",
 "libm",
 "rand 0.9.2",
 "siphasher 1.0.2",
]

[[package]]
name = "fastrand"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "37909eebbb50d72f9059c3b6d82c0463f2ff062c9e95845c43a6c9c0355411be"

[[package]]
name = "feature-probe"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "835a3dc7d1ec9e75e2b5fb4ba75396837112d2060b03f7d43bc1897c7f7211da"

[[package]]
name = "fiat-crypto"
version = "0.2.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "28dea519a9695b9977216879a3ebfddf92f1c08c05d984f8996aecd6ecdc811d"

[[package]]
name = "find-msvc-tools"
version = "0.1.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8591b0bcc8a98a64310a2fae1bb3e9b8564dd10e381e6e28010fde8e8e8568db"

[[package]]
name = "five8"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a75b8549488b4715defcb0d8a8a1c1c76a80661b5fa106b4ca0e7fce59d7d875"
dependencies = [
 "five8_core",
]

[[package]]
name = "five8_const"
version = "0.1.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "26dec3da8bc3ef08f2c04f61eab298c3ab334523e55f076354d6d6f613799a7b"
dependencies = [
 "five8_core",
]

[[package]]
name = "five8_core"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2551bf44bc5f776c15044b9b94153a00198be06743e262afaaa61f11ac7523a5"

[[package]]
name = "flate2"
version = "1.1.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b375d6465b98090a5f25b1c7703f3859783755aa9a80433b36e0379a3ec2f369"
dependencies = [
 "crc32fast",
 "miniz_oxide",
]

[[package]]
name = "fnv"
version = "1.0.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1"

[[package]]
name = "foreign-types"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1"
dependencies = [
 "foreign-types-shared",
]

[[package]]
name = "foreign-types-shared"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b"

[[package]]
name = "form_urlencoded"
version = "1.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cb4cb245038516f5f85277875cdaa4f7d2c9a0fa0468de06ed190163b1581fcf"
dependencies = [
 "percent-encoding",
]

[[package]]
name = "futures"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "65bc07b1a8bc7c85c5f2e110c476c7389b4554ba72af57d8445ea63a576b0876"
dependencies = [
 "futures-channel",
 "futures-core",
 "futures-executor",
 "futures-io",
 "futures-sink",
 "futures-task",
 "futures-util",
]

[[package]]
name = "futures-channel"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2dff15bf788c671c1934e366d07e30c1814a8ef514e1af724a602e8a2fbe1b10"
dependencies = [
 "futures-core",
 "futures-sink",
]

[[package]]
name = "futures-core"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "05f29059c0c2090612e8d742178b0580d2dc940c837851ad723096f87af6663e"

[[package]]
name = "futures-executor"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e28d1d997f585e54aebc3f97d39e72338912123a67330d723fdbb564d646c9f"
dependencies = [
 "futures-core",
 "futures-task",
 "futures-util",
]

[[package]]
name = "futures-io"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9e5c1b78ca4aae1ac06c48a526a655760685149f0d465d21f37abfe57ce075c6"

[[package]]
name = "futures-macro"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "162ee34ebcb7c64a8abebc059ce0fee27c2262618d7b60ed8faf72fef13c3650"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "futures-sink"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e575fab7d1e0dcb8d0c7bcf9a63ee213816ab51902e6d244a95819acacf1d4f7"

[[package]]
name = "futures-task"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f90f7dce0722e95104fcb095585910c0977252f286e354b5e3bd38902cd99988"

[[package]]
name = "futures-timer"
version = "3.0.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f288b0a4f20f9a56b5d1da57e2227c661b7b16168e2f72365f57b63326e29b24"

[[package]]
name = "futures-util"
version = "0.3.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9fa08315bb612088cc391249efdc3bc77536f16c91f6cf495e6fbe85b20a4a81"
dependencies = [
 "futures-channel",
 "futures-core",
 "futures-io",
 "futures-macro",
 "futures-sink",
 "futures-task",
 "memchr",
 "pin-project-lite",
 "pin-utils",
 "slab",
]

[[package]]
name = "generic-array"
version = "0.14.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85649ca51fd72272d7821adaf274ad91c288277713d9c18820d8499a7ff69e9a"
dependencies = [
 "typenum",
 "version_check",
]

[[package]]
name = "gethostname"
version = "0.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c1ebd34e35c46e00bb73e81363248d627782724609fe1b6396f553f68fe3862e"
dependencies = [
 "libc",
 "winapi",
]

[[package]]
name = "getrandom"
version = "0.1.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8fc3cb4d91f53b50155bdcfd23f6a4c39ae1969c2ae85982b135750cccaf5fce"
dependencies = [
 "cfg-if",
 "js-sys",
 "libc",
 "wasi 0.9.0+wasi-snapshot-preview1",
 "wasm-bindgen",
]

[[package]]
name = "getrandom"
version = "0.2.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff2abc00be7fca6ebc474524697ae276ad847ad0a6b3faa4bcb027e9a4614ad0"
dependencies = [
 "cfg-if",
 "js-sys",
 "libc",
 "wasi 0.11.1+wasi-snapshot-preview1",
 "wasm-bindgen",
]

[[package]]
name = "getrandom"
version = "0.3.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "899def5c37c4fd7b2664648c28120ecec138e4d395b459e5ca34f9cce2dd77fd"
dependencies = [
 "cfg-if",
 "js-sys",
 "libc",
 "r-efi",
 "wasip2",
 "wasm-bindgen",
]

[[package]]
name = "governor"
version = "0.6.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "68a7f542ee6b35af73b06abc0dad1c1bae89964e4e253bc4b587b91c9637867b"
dependencies = [
 "cfg-if",
 "dashmap",
 "futures",
 "futures-timer",
 "no-std-compat",
 "nonzero_ext",
 "parking_lot",
 "portable-atomic",
 "quanta",
 "rand 0.8.5",
 "smallvec",
 "spinning_top",
]

[[package]]
name = "h2"
version = "0.4.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2f44da3a8150a6703ed5d34e164b875fd14c2cdab9af1252a9a1020bde2bdc54"
dependencies = [
 "atomic-waker",
 "bytes",
 "fnv",
 "futures-core",
 "futures-sink",
 "http 1.4.0",
 "indexmap",
 "slab",
 "tokio",
 "tokio-util",
 "tracing",
]

[[package]]
name = "hashbrown"
version = "0.13.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "43a3c133739dddd0d2990f9a4bdf8eb4b21ef50e4851ca85ab661199821d510e"
dependencies = [
 "ahash",
]

[[package]]
name = "hashbrown"
version = "0.14.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e5274423e17b7c9fc20b6e7e208532f9b19825d82dfd615708b70edd83df41f1"

[[package]]
name = "hashbrown"
version = "0.16.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "841d1cc9bed7f9236f321df977030373f4a4163ae1a7dbfe1a51a2c1a51d9100"

[[package]]
name = "hermit-abi"
version = "0.1.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"
dependencies = [
 "libc",
]

[[package]]
name = "hermit-abi"
version = "0.5.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fc0fef456e4baa96da950455cd02c081ca953b141298e41db3fc7e36b1da849c"

[[package]]
name = "histogram"
version = "0.6.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "12cb882ccb290b8646e554b157ab0b71e64e8d5bef775cd66b6531e52d302669"

[[package]]
name = "hmac"
version = "0.8.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "126888268dcc288495a26bf004b38c5fdbb31682f992c84ceb046a1f0fe38840"
dependencies = [
 "crypto-mac",
 "digest 0.9.0",
]

[[package]]
name = "hmac"
version = "0.12.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6c49c37c09c17a53d937dfbb742eb3a961d65a994e6bcdcf37e7399d0cc8ab5e"
dependencies = [
 "digest 0.10.7",
]

[[package]]
name = "hmac-drbg"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "17ea0a1394df5b6574da6e0c1ade9e78868c9fb0a4e5ef4428e32da4676b85b1"
dependencies = [
 "digest 0.9.0",
 "generic-array",
 "hmac 0.8.1",
]

[[package]]
name = "http"
version = "0.2.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "601cbb57e577e2f5ef5be8e7b83f0f63994f25aa94d673e54a92d5c516d101f1"
dependencies = [
 "bytes",
 "fnv",
 "itoa",
]

[[package]]
name = "http"
version = "1.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3ba2a386d7f85a81f119ad7498ebe444d2e22c2af0b86b069416ace48b3311a"
dependencies = [
 "bytes",
 "itoa",
]

[[package]]
name = "http-body"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1efedce1fb8e6913f23e0c92de8e62cd5b772a67e7b3946df930a62566c93184"
dependencies = [
 "bytes",
 "http 1.4.0",
]

[[package]]
name = "http-body-util"
version = "0.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b021d93e26becf5dc7e1b75b1bed1fd93124b374ceb73f43d4d4eafec896a64a"
dependencies = [
 "bytes",
 "futures-core",
 "http 1.4.0",
 "http-body",
 "pin-project-lite",
]

[[package]]
name = "httparse"
version = "1.10.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6dbf3de79e51f3d586ab4cb9d5c3e2c14aa28ed23d180cf89b4df0454a69cc87"

[[package]]
name = "humantime"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "135b12329e5e3ce057a9f972339ea52bc954fe1e9358ef27f95e89716fbc5424"

[[package]]
name = "hyper"
version = "1.8.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2ab2d4f250c3d7b1c9fcdff1cece94ea4e2dfbec68614f7b87cb205f24ca9d11"
dependencies = [
 "atomic-waker",
 "bytes",
 "futures-channel",
 "futures-core",
 "h2",
 "http 1.4.0",
 "http-body",
 "httparse",
 "itoa",
 "pin-project-lite",
 "pin-utils",
 "smallvec",
 "tokio",
 "want",
]

[[package]]
name = "hyper-rustls"
version = "0.27.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3c93eb611681b207e1fe55d5a71ecf91572ec8a6705cdb6857f7d8d5242cf58"
dependencies = [
 "http 1.4.0",
 "hyper",
 "hyper-util",
 "rustls 0.23.36",
 "rustls-pki-types",
 "tokio",
 "tokio-rustls 0.26.4",
 "tower-service",
 "webpki-roots 1.0.5",
]

[[package]]
name = "hyper-tls"
version = "0.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "70206fc6890eaca9fde8a0bf71caa2ddfc9fe045ac9e5c70df101a7dbde866e0"
dependencies = [
 "bytes",
 "http-body-util",
 "hyper",
 "hyper-util",
 "native-tls",
 "tokio",
 "tokio-native-tls",
 "tower-service",
]

[[package]]
name = "hyper-util"
version = "0.1.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "727805d60e7938b76b826a6ef209eb70eaa1812794f9424d4a4e2d740662df5f"
dependencies = [
 "base64 0.22.1",
 "bytes",
 "futures-channel",
 "futures-core",
 "futures-util",
 "http 1.4.0",
 "http-body",
 "hyper",
 "ipnet",
 "libc",
 "percent-encoding",
 "pin-project-lite",
 "socket2 0.6.2",
 "system-configuration",
 "tokio",
 "tower-service",
 "tracing",
 "windows-registry",
]

[[package]]
name = "icu_collections"
version = "2.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4c6b649701667bbe825c3b7e6388cb521c23d88644678e83c0c4d0a621a34b43"
dependencies = [
 "displaydoc",
 "potential_utf",
 "yoke",
 "zerofrom",
 "zerovec",
]

[[package]]
name = "icu_locale_core"
version = "2.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "edba7861004dd3714265b4db54a3c390e880ab658fec5f7db895fae2046b5bb6"
dependencies = [
 "displaydoc",
 "litemap",
 "tinystr",
 "writeable",
 "zerovec",
]

[[package]]
name = "icu_normalizer"
version = "2.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5f6c8828b67bf8908d82127b2054ea1b4427ff0230ee9141c54251934ab1b599"
dependencies = [
 "icu_collections",
 "icu_normalizer_data",
 "icu_properties",
 "icu_provider",
 "smallvec",
 "zerovec",
]

[[package]]
name = "icu_normalizer_data"
version = "2.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7aedcccd01fc5fe81e6b489c15b247b8b0690feb23304303a9e560f37efc560a"

[[package]]
name = "icu_properties"
version = "2.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "020bfc02fe870ec3a66d93e677ccca0562506e5872c650f893269e08615d74ec"
dependencies = [
 "icu_collections",
 "icu_locale_core",
 "icu_properties_data",
 "icu_provider",
 "zerotrie",
 "zerovec",
]

[[package]]
name = "icu_properties_data"
version = "2.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "616c294cf8d725c6afcd8f55abc17c56464ef6211f9ed59cccffe534129c77af"

[[package]]
name = "icu_provider"
version = "2.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85962cf0ce02e1e0a629cc34e7ca3e373ce20dda4c4d7294bbd0bf1fdb59e614"
dependencies = [
 "displaydoc",
 "icu_locale_core",
 "writeable",
 "yoke",
 "zerofrom",
 "zerotrie",
 "zerovec",
]

[[package]]
name = "ident_case"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b9e0384b61958566e926dc50660321d12159025e767c18e043daf26b70104c39"

[[package]]
name = "idna"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3b0875f23caa03898994f6ddc501886a45c7d3d62d04d2d90788d47be1b1e4de"
dependencies = [
 "idna_adapter",
 "smallvec",
 "utf8_iter",
]

[[package]]
name = "idna_adapter"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3acae9609540aa318d1bc588455225fb2085b9ed0c4f6bd0d9d5bcd86f1a0344"
dependencies = [
 "icu_normalizer",
 "icu_properties",
]

[[package]]
name = "indexmap"
version = "2.13.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7714e70437a7dc3ac8eb7e6f8df75fd8eb422675fc7678aff7364301092b1017"
dependencies = [
 "equivalent",
 "hashbrown 0.16.1",
]

[[package]]
name = "indicatif"
version = "0.17.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "183b3088984b400f4cfac3620d5e076c84da5364016b4f49473de574b2586235"
dependencies = [
 "console",
 "number_prefix",
 "portable-atomic",
 "unicode-width",
 "web-time",
]

[[package]]
name = "ipnet"
version = "2.11.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "469fb0b9cefa57e3ef31275ee7cacb78f2fdca44e4765491884a2b119d4eb130"

[[package]]
name = "iri-string"
version = "0.7.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c91338f0783edbd6195decb37bae672fd3b165faffb89bf7b9e6942f8b1a731a"
dependencies = [
 "memchr",
 "serde",
]

[[package]]
name = "itertools"
version = "0.10.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b0fd2260e829bddf4cb6ea802289de2f86d6a7a690192fbe91b3f46e0f2c8473"
dependencies = [
 "either",
]

[[package]]
name = "itertools"
version = "0.12.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ba291022dbbd398a455acf126c1e341954079855bc60dfdda641363bd6922569"
dependencies = [
 "either",
]

[[package]]
name = "itoa"
version = "1.0.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "92ecc6618181def0457392ccd0ee51198e065e016d1d527a7ac1b6dc7c1f09d2"

[[package]]
name = "jito-bundle"
version = "0.1.5"
dependencies = [
 "base64 0.22.1",
 "bincode",
 "bs58",
 "dotenvy",
 "rand 0.9.2",
 "reqwest",
 "serde",
 "serde_json",
 "solana-client",
 "solana-compute-budget-interface",
 "solana-instruction",
 "solana-pubkey",
 "solana-sdk",
 "solana-transaction-status-client-types",
 "thiserror 2.0.18",
 "tokio",
 "tracing",
]

[[package]]
name = "jni"
version = "0.21.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1a87aa2bb7d2af34197c04845522473242e1aa17c12f4935d5856491a7fb8c97"
dependencies = [
 "cesu8",
 "cfg-if",
 "combine",
 "jni-sys",
 "log",
 "thiserror 1.0.69",
 "walkdir",
 "windows-sys 0.45.0",
]

[[package]]
name = "jni-sys"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8eaf4bc02d17cbdd7ff4c7438cafcdf7fb9a4613313ad11b4f8fefe7d3fa0130"

[[package]]
name = "jobserver"
version = "0.1.34"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9afb3de4395d6b3e67a780b6de64b51c978ecf11cb9a462c66be7d4ca9039d33"
dependencies = [
 "getrandom 0.3.4",
 "libc",
]

[[package]]
name = "js-sys"
version = "0.3.85"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8c942ebf8e95485ca0d52d97da7c5a2c387d0e7f0ba4c35e93bfcaee045955b3"
dependencies = [
 "once_cell",
 "wasm-bindgen",
]

[[package]]
name = "jsonrpc-core"
version = "18.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "14f7f76aef2d054868398427f6c54943cf3d1caa9a7ec7d0c38d69df97a965eb"
dependencies = [
 "futures",
 "futures-executor",
 "futures-util",
 "log",
 "serde",
 "serde_derive",
 "serde_json",
]

[[package]]
name = "keccak"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ecc2af9a1119c51f12a14607e783cb977bde58bc069ff0c3da1095e635d70654"
dependencies = [
 "cpufeatures",
]

[[package]]
name = "lazy_static"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bbd2bcb4c963f2ddae06a2efc7e9f3591312473c50c6685e1f298068316e66fe"

[[package]]
name = "libc"
version = "0.2.180"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bcc35a38544a891a5f7c865aca548a982ccb3b8650a5b06d0fd33a10283c56fc"

[[package]]
name = "libm"
version = "0.2.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6d2cec3eae94f9f509c767b45932f1ada8350c4bdb85af2fcab4a3c14807981"

[[package]]
name = "libsecp256k1"
version = "0.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c9d220bc1feda2ac231cb78c3d26f27676b8cf82c96971f7aeef3d0cf2797c73"
dependencies = [
 "arrayref",
 "base64 0.12.3",
 "digest 0.9.0",
 "hmac-drbg",
 "libsecp256k1-core",
 "libsecp256k1-gen-ecmult",
 "libsecp256k1-gen-genmult",
 "rand 0.7.3",
 "serde",
 "sha2 0.9.9",
 "typenum",
]

[[package]]
name = "libsecp256k1-core"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d0f6ab710cec28cef759c5f18671a27dae2a5f952cdaaee1d8e2908cb2478a80"
dependencies = [
 "crunchy",
 "digest 0.9.0",
 "subtle",
]

[[package]]
name = "libsecp256k1-gen-ecmult"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ccab96b584d38fac86a83f07e659f0deafd0253dc096dab5a36d53efe653c5c3"
dependencies = [
 "libsecp256k1-core",
]

[[package]]
name = "libsecp256k1-gen-genmult"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "67abfe149395e3aa1c48a2beb32b068e2334402df8181f818d3aee2b304c4f5d"
dependencies = [
 "libsecp256k1-core",
]

[[package]]
name = "linux-raw-sys"
version = "0.11.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "df1d3c3b53da64cf5760482273a98e575c651a67eec7f77df96b5b642de8f039"

[[package]]
name = "litemap"
version = "0.8.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6373607a59f0be73a39b6fe456b8192fcc3585f602af20751600e974dd455e77"

[[package]]
name = "lock_api"
version = "0.4.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "224399e74b87b5f3557511d98dff8b14089b3dadafcab6bb93eab67d3aace965"
dependencies = [
 "scopeguard",
]

[[package]]
name = "log"
version = "0.4.29"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5e5032e24019045c762d3c0f28f5b6b8bbf38563a65908389bf7978758920897"

[[package]]
name = "lru-slab"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "112b39cec0b298b6c1999fee3e31427f74f676e4cb9879ed1a121b43661a4154"

[[package]]
name = "memchr"
version = "2.7.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f52b00d39961fc5b2736ea853c9cc86238e165017a493d1d5c8eac6bdc4cc273"

[[package]]
name = "memmap2"
version = "0.5.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "83faa42c0a078c393f6b29d5db232d8be22776a891f8f56e5284faee4a20b327"
dependencies = [
 "libc",
]

[[package]]
name = "memoffset"
version = "0.9.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "488016bfae457b036d996092f6cb448677611ce4449e970ceaf42695203f218a"
dependencies = [
 "autocfg",
]

[[package]]
name = "mime"
version = "0.3.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6877bb514081ee2a7ff5ef9de3281f14a4dd4bceac4c09388074a6b5df8a139a"

[[package]]
name = "minimal-lexical"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "68354c5c6bd36d73ff3feceb05efa59b6acb7626617f4962be322a825e61f79a"

[[package]]
name = "miniz_oxide"
version = "0.8.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1fa76a2c86f704bdb222d66965fb3d63269ce38518b83cb0575fca855ebb6316"
dependencies = [
 "adler2",
 "simd-adler32",
]

[[package]]
name = "mio"
version = "1.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a69bcab0ad47271a0234d9422b131806bf3968021e5dc9328caf2d4cd58557fc"
dependencies = [
 "libc",
 "wasi 0.11.1+wasi-snapshot-preview1",
 "windows-sys 0.61.2",
]

[[package]]
name = "native-tls"
version = "0.2.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "87de3442987e9dbec73158d5c715e7ad9072fda936bb03d19d7fa10e00520f0e"
dependencies = [
 "libc",
 "log",
 "openssl",
 "openssl-probe 0.1.6",
 "openssl-sys",
 "schannel",
 "security-framework 2.11.1",
 "security-framework-sys",
 "tempfile",
]

[[package]]
name = "nix"
version = "0.30.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "74523f3a35e05aba87a1d978330aef40f67b0304ac79c1c00b294c9830543db6"
dependencies = [
 "bitflags",
 "cfg-if",
 "cfg_aliases",
 "libc",
 "memoffset",
]

[[package]]
name = "no-std-compat"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b93853da6d84c2e3c7d730d6473e8817692dd89be387eb01b94d7f108ecb5b8c"

[[package]]
name = "nom"
version = "7.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d273983c5a657a70a3e8f2a01329822f3b8c8172b73826411a55751e404a0a4a"
dependencies = [
 "memchr",
 "minimal-lexical",
]

[[package]]
name = "nonzero_ext"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "38bf9645c8b145698bb0b18a4637dcacbc421ea49bef2317e4fd8065a387cf21"

[[package]]
name = "num"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8536030f9fea7127f841b45bb6243b27255787fb4eb83958aa1ef9d2fdc0c36"
dependencies = [
 "num-bigint 0.2.6",
 "num-complex",
 "num-integer",
 "num-iter",
 "num-rational",
 "num-traits",
]

[[package]]
name = "num-bigint"
version = "0.2.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "090c7f9998ee0ff65aa5b723e4009f7b217707f1fb5ea551329cc4d6231fb304"
dependencies = [
 "autocfg",
 "num-integer",
 "num-traits",
]

[[package]]
name = "num-bigint"
version = "0.4.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a5e44f723f1133c9deac646763579fdb3ac745e418f2a7af9cd0c431da1f20b9"
dependencies = [
 "num-integer",
 "num-traits",
]

[[package]]
name = "num-complex"
version = "0.2.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6b19411a9719e753aff12e5187b74d60d3dc449ec3f4dc21e3989c3f554bc95"
dependencies = [
 "autocfg",
 "num-traits",
]

[[package]]
name = "num-conv"
version = "0.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cf97ec579c3c42f953ef76dbf8d55ac91fb219dde70e49aa4a6b7d74e9919050"

[[package]]
name = "num-derive"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ed3955f1a9c7c0c15e092f9c887db08b1fc683305fdf6eb6684f22555355e202"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "num-integer"
version = "0.1.46"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7969661fd2958a5cb096e56c8e1ad0444ac2bbcd0061bd28660485a44879858f"
dependencies = [
 "num-traits",
]

[[package]]
name = "num-iter"
version = "0.1.45"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1429034a0490724d0075ebb2bc9e875d6503c3cf69e235a8941aa757d83ef5bf"
dependencies = [
 "autocfg",
 "num-integer",
 "num-traits",
]

[[package]]
name = "num-rational"
version = "0.2.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c000134b5dbf44adc5cb772486d335293351644b801551abe8f75c84cfa4aef"
dependencies = [
 "autocfg",
 "num-bigint 0.2.6",
 "num-integer",
 "num-traits",
]

[[package]]
name = "num-traits"
version = "0.2.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "071dfc062690e90b734c0b2273ce72ad0ffa95f0c74596bc250dcfd960262841"
dependencies = [
 "autocfg",
]

[[package]]
name = "num_cpus"
version = "1.17.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "91df4bbde75afed763b708b7eee1e8e7651e02d97f6d5dd763e89367e957b23b"
dependencies = [
 "hermit-abi 0.5.2",
 "libc",
]

[[package]]
name = "num_enum"
version = "0.7.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b1207a7e20ad57b847bbddc6776b968420d38292bbfe2089accff5e19e82454c"
dependencies = [
 "num_enum_derive",
 "rustversion",
]

[[package]]
name = "num_enum_derive"
version = "0.7.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff32365de1b6743cb203b710788263c44a03de03802daf96092f2da4fe6ba4d7"
dependencies = [
 "proc-macro-crate 3.4.0",
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "number_prefix"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "830b246a0e5f20af87141b25c173cd1b609bd7779a4617d6ec582abaf90870f3"

[[package]]
name = "oid-registry"
version = "0.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9bedf36ffb6ba96c2eb7144ef6270557b52e54b20c0a8e1eb2ff99a6c6959bff"
dependencies = [
 "asn1-rs",
]

[[package]]
name = "once_cell"
version = "1.21.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "42f5e15c9953c5e4ccceeb2e7382a716482c34515315f7b03532b8b4e8393d2d"

[[package]]
name = "opaque-debug"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c08d65885ee38876c4f86fa503fb49d7b507c2b62552df7c70b2fce627e06381"

[[package]]
name = "openssl"
version = "0.10.75"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "08838db121398ad17ab8531ce9de97b244589089e290a384c900cb9ff7434328"
dependencies = [
 "bitflags",
 "cfg-if",
 "foreign-types",
 "libc",
 "once_cell",
 "openssl-macros",
 "openssl-sys",
]

[[package]]
name = "openssl-macros"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a948666b637a0f465e8564c73e89d4dde00d72d4d473cc972f390fc3dcee7d9c"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "openssl-probe"
version = "0.1.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d05e27ee213611ffe7d6348b942e8f942b37114c00cc03cec254295a4a17852e"

[[package]]
name = "openssl-probe"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7c87def4c32ab89d880effc9e097653c8da5d6ef28e6b539d313baaacfbafcbe"

[[package]]
name = "openssl-sys"
version = "0.9.111"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "82cab2d520aa75e3c58898289429321eb788c3106963d0dc886ec7a5f4adc321"
dependencies = [
 "cc",
 "libc",
 "pkg-config",
 "vcpkg",
]

[[package]]
name = "parking"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f38d5652c16fde515bb1ecef450ab0f6a219d619a7274976324d5e377f7dceba"

[[package]]
name = "parking_lot"
version = "0.12.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "93857453250e3077bd71ff98b6a65ea6621a19bb0f559a85248955ac12c45a1a"
dependencies = [
 "lock_api",
 "parking_lot_core",
]

[[package]]
name = "parking_lot_core"
version = "0.9.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2621685985a2ebf1c516881c026032ac7deafcda1a2c9b7850dc81e3dfcb64c1"
dependencies = [
 "cfg-if",
 "libc",
 "redox_syscall",
 "smallvec",
 "windows-link",
]

[[package]]
name = "paste"
version = "1.0.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "57c0d7b74b563b49d38dae00a0c37d4d6de9b432382b2892f0574ddcae73fd0a"

[[package]]
name = "pbkdf2"
version = "0.11.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "83a0692ec44e4cf1ef28ca317f14f8f07da2d95ec3fa01f86e4467b725e60917"
dependencies = [
 "digest 0.10.7",
]

[[package]]
name = "pem"
version = "1.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a8835c273a76a90455d7344889b0964598e3316e2a79ede8e36f16bdcf2228b8"
dependencies = [
 "base64 0.13.1",
]

[[package]]
name = "percent-encoding"
version = "2.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9b4f627cb1b25917193a259e49bdad08f671f8d9708acfd5fe0a8c1455d87220"

[[package]]
name = "percentage"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2fd23b938276f14057220b707937bcb42fa76dda7560e57a2da30cb52d557937"
dependencies = [
 "num",
]

[[package]]
name = "pin-project-lite"
version = "0.2.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3b3cff922bd51709b605d9ead9aa71031d81447142d828eb4a6eba76fe619f9b"

[[package]]
name = "pin-utils"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"

[[package]]
name = "pkg-config"
version = "0.3.32"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7edddbd0b52d732b21ad9a5fab5c704c14cd949e5e9a1ec5929a24fded1b904c"

[[package]]
name = "portable-atomic"
version = "1.13.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f89776e4d69bb58bc6993e99ffa1d11f228b839984854c7daeb5d37f87cbe950"

[[package]]
name = "potential_utf"
version = "0.1.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b73949432f5e2a09657003c25bca5e19a0e9c84f8058ca374f49e0ebe605af77"
dependencies = [
 "zerovec",
]

[[package]]
name = "powerfmt"
version = "0.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "439ee305def115ba05938db6eb1644ff94165c5ab5e9420d1c1bcedbba909391"

[[package]]
name = "ppv-lite86"
version = "0.2.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85eae3c4ed2f50dcfe72643da4befc30deadb458a9b590d720cde2f2b1e97da9"
dependencies = [
 "zerocopy",
]

[[package]]
name = "proc-macro-crate"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1d6ea3c4595b96363c13943497db34af4460fb474a95c43f4446ad341b8c9785"
dependencies = [
 "toml",
]

[[package]]
name = "proc-macro-crate"
version = "3.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "219cb19e96be00ab2e37d6e299658a0cfa83e52429179969b0f0121b4ac46983"
dependencies = [
 "toml_edit",
]

[[package]]
name = "proc-macro2"
version = "1.0.106"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8fd00f0bb2e90d81d1044c2b32617f68fcb9fa3bb7640c23e9c748e53fb30934"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "qstring"
version = "0.7.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d464fae65fff2680baf48019211ce37aaec0c78e9264c84a3e484717f965104e"
dependencies = [
 "percent-encoding",
]

[[package]]
name = "quanta"
version = "0.12.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f3ab5a9d756f0d97bdc89019bd2e4ea098cf9cde50ee7564dde6b81ccc8f06c7"
dependencies = [
 "crossbeam-utils",
 "libc",
 "once_cell",
 "raw-cpuid",
 "wasi 0.11.1+wasi-snapshot-preview1",
 "web-sys",
 "winapi",
]

[[package]]
name = "quinn"
version = "0.11.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b9e20a958963c291dc322d98411f541009df2ced7b5a4f2bd52337638cfccf20"
dependencies = [
 "bytes",
 "cfg_aliases",
 "pin-project-lite",
 "quinn-proto",
 "quinn-udp",
 "rustc-hash",
 "rustls 0.23.36",
 "socket2 0.6.2",
 "thiserror 2.0.18",
 "tokio",
 "tracing",
 "web-time",
]

[[package]]
name = "quinn-proto"
version = "0.11.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f1906b49b0c3bc04b5fe5d86a77925ae6524a19b816ae38ce1e426255f1d8a31"
dependencies = [
 "bytes",
 "fastbloom",
 "getrandom 0.3.4",
 "lru-slab",
 "rand 0.9.2",
 "ring",
 "rustc-hash",
 "rustls 0.23.36",
 "rustls-pki-types",
 "rustls-platform-verifier",
 "slab",
 "thiserror 2.0.18",
 "tinyvec",
 "tracing",
 "web-time",
]

[[package]]
name = "quinn-udp"
version = "0.5.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "addec6a0dcad8a8d96a771f815f0eaf55f9d1805756410b39f5fa81332574cbd"
dependencies = [
 "cfg_aliases",
 "libc",
 "once_cell",
 "socket2 0.6.2",
 "tracing",
 "windows-sys 0.60.2",
]

[[package]]
name = "quote"
version = "1.0.44"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "21b2ebcf727b7760c461f091f9f0f539b77b8e87f2fd88131e7f1b433b3cece4"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "r-efi"
version = "5.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "69cdb34c158ceb288df11e18b4bd39de994f6657d83847bdffdbd7f346754b0f"

[[package]]
name = "rand"
version = "0.7.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6a6b1679d49b24bbfe0c803429aa1874472f50d9b363131f0e89fc356b544d03"
dependencies = [
 "getrandom 0.1.16",
 "libc",
 "rand_chacha 0.2.2",
 "rand_core 0.5.1",
 "rand_hc",
]

[[package]]
name = "rand"
version = "0.8.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
dependencies = [
 "libc",
 "rand_chacha 0.3.1",
 "rand_core 0.6.4",
]

[[package]]
name = "rand"
version = "0.9.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6db2770f06117d490610c7488547d543617b21bfa07796d7a12f6f1bd53850d1"
dependencies = [
 "rand_chacha 0.9.0",
 "rand_core 0.9.5",
]

[[package]]
name = "rand_chacha"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f4c8ed856279c9737206bf725bf36935d8666ead7aa69b52be55af369d193402"
dependencies = [
 "ppv-lite86",
 "rand_core 0.5.1",
]

[[package]]
name = "rand_chacha"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e6c10a63a0fa32252be49d21e7709d4d4baf8d231c2dbce1eaa8141b9b127d88"
dependencies = [
 "ppv-lite86",
 "rand_core 0.6.4",
]

[[package]]
name = "rand_chacha"
version = "0.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3022b5f1df60f26e1ffddd6c66e8aa15de382ae63b3a0c1bfc0e4d3e3f325cb"
dependencies = [
 "ppv-lite86",
 "rand_core 0.9.5",
]

[[package]]
name = "rand_core"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "90bde5296fc891b0cef12a6d03ddccc162ce7b2aff54160af9338f8d40df6d19"
dependencies = [
 "getrandom 0.1.16",
]

[[package]]
name = "rand_core"
version = "0.6.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c"
dependencies = [
 "getrandom 0.2.17",
]

[[package]]
name = "rand_core"
version = "0.9.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "76afc826de14238e6e8c374ddcc1fa19e374fd8dd986b0d2af0d02377261d83c"
dependencies = [
 "getrandom 0.3.4",
]

[[package]]
name = "rand_hc"
version = "0.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ca3129af7b92a17112d59ad498c6f81eaf463253766b90396d39ea7a39d6613c"
dependencies = [
 "rand_core 0.5.1",
]

[[package]]
name = "raw-cpuid"
version = "11.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "498cd0dc59d73224351ee52a95fee0f1a617a2eae0e7d9d720cc622c73a54186"
dependencies = [
 "bitflags",
]

[[package]]
name = "rayon"
version = "1.11.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "368f01d005bf8fd9b1206fb6fa653e6c4a81ceb1466406b81792d87c5677a58f"
dependencies = [
 "either",
 "rayon-core",
]

[[package]]
name = "rayon-core"
version = "1.13.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "22e18b0f0062d30d4230b2e85ff77fdfe4326feb054b9783a3460d8435c8ab91"
dependencies = [
 "crossbeam-deque",
 "crossbeam-utils",
]

[[package]]
name = "redox_syscall"
version = "0.5.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ed2bf2547551a7053d6fdfafda3f938979645c44812fbfcda098faae3f1a362d"
dependencies = [
 "bitflags",
]

[[package]]
name = "regex"
version = "1.12.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "843bc0191f75f3e22651ae5f1e72939ab2f72a4bc30fa80a066bd66edefc24d4"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-automata",
 "regex-syntax",
]

[[package]]
name = "regex-automata"
version = "0.4.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5276caf25ac86c8d810222b3dbb938e512c55c6831a10f3e6ed1c93b84041f1c"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-syntax",
]

[[package]]
name = "regex-syntax"
version = "0.8.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7a2d987857b319362043e95f5353c0535c1f58eec5336fdfcf626430af7def58"

[[package]]
name = "reqwest"
version = "0.12.28"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eddd3ca559203180a307f12d114c268abf583f59b03cb906fd0b3ff8646c1147"
dependencies = [
 "base64 0.22.1",
 "bytes",
 "encoding_rs",
 "futures-channel",
 "futures-core",
 "futures-util",
 "h2",
 "http 1.4.0",
 "http-body",
 "http-body-util",
 "hyper",
 "hyper-rustls",
 "hyper-tls",
 "hyper-util",
 "js-sys",
 "log",
 "mime",
 "native-tls",
 "percent-encoding",
 "pin-project-lite",
 "quinn",
 "rustls 0.23.36",
 "rustls-pki-types",
 "serde",
 "serde_json",
 "serde_urlencoded",
 "sync_wrapper",
 "tokio",
 "tokio-native-tls",
 "tokio-rustls 0.26.4",
 "tower",
 "tower-http",
 "tower-service",
 "url",
 "wasm-bindgen",
 "wasm-bindgen-futures",
 "web-sys",
 "webpki-roots 1.0.5",
]

[[package]]
name = "reqwest-middleware"
version = "0.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "57f17d28a6e6acfe1733fe24bcd30774d13bffa4b8a22535b4c8c98423088d4e"
dependencies = [
 "anyhow",
 "async-trait",
 "http 1.4.0",
 "reqwest",
 "serde",
 "thiserror 1.0.69",
 "tower-service",
]

[[package]]
name = "ring"
version = "0.17.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a4689e6c2294d81e88dc6261c768b63bc4fcdb852be6d1352498b114f61383b7"
dependencies = [
 "cc",
 "cfg-if",
 "getrandom 0.2.17",
 "libc",
 "untrusted",
 "windows-sys 0.52.0",
]

[[package]]
name = "rustc-hash"
version = "2.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "357703d41365b4b27c590e3ed91eabb1b663f07c4c084095e60cbed4362dff0d"

[[package]]
name = "rustc_version"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cfcb3a22ef46e85b45de6ee7e79d063319ebb6594faafcf1c225ea92ab6e9b92"
dependencies = [
 "semver",
]

[[package]]
name = "rusticata-macros"
version = "4.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "faf0c4a6ece9950b9abdb62b1cfcf2a68b3b67a10ba445b3bb85be2a293d0632"
dependencies = [
 "nom",
]

[[package]]
name = "rustix"
version = "1.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "146c9e247ccc180c1f61615433868c99f3de3ae256a30a43b49f67c2d9171f34"
dependencies = [
 "bitflags",
 "errno",
 "libc",
 "linux-raw-sys",
 "windows-sys 0.61.2",
]

[[package]]
name = "rustls"
version = "0.21.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f56a14d1f48b391359b22f731fd4bd7e43c97f3c50eee276f3aa09c94784d3e"
dependencies = [
 "log",
 "ring",
 "rustls-webpki 0.101.7",
 "sct",
]

[[package]]
name = "rustls"
version = "0.23.36"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c665f33d38cea657d9614f766881e4d510e0eda4239891eea56b4cadcf01801b"
dependencies = [
 "once_cell",
 "ring",
 "rustls-pki-types",
 "rustls-webpki 0.103.9",
 "subtle",
 "zeroize",
]

[[package]]
name = "rustls-native-certs"
version = "0.8.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "612460d5f7bea540c490b2b6395d8e34a953e52b491accd6c86c8164c5932a63"
dependencies = [
 "openssl-probe 0.2.1",
 "rustls-pki-types",
 "schannel",
 "security-framework 3.5.1",
]

[[package]]
name = "rustls-pki-types"
version = "1.14.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "be040f8b0a225e40375822a563fa9524378b9d63112f53e19ffff34df5d33fdd"
dependencies = [
 "web-time",
 "zeroize",
]

[[package]]
name = "rustls-platform-verifier"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1d99feebc72bae7ab76ba994bb5e121b8d83d910ca40b36e0921f53becc41784"
dependencies = [
 "core-foundation 0.10.1",
 "core-foundation-sys",
 "jni",
 "log",
 "once_cell",
 "rustls 0.23.36",
 "rustls-native-certs",
 "rustls-platform-verifier-android",
 "rustls-webpki 0.103.9",
 "security-framework 3.5.1",
 "security-framework-sys",
 "webpki-root-certs",
 "windows-sys 0.61.2",
]

[[package]]
name = "rustls-platform-verifier-android"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f87165f0995f63a9fbeea62b64d10b4d9d8e78ec6d7d51fb2125fda7bb36788f"

[[package]]
name = "rustls-webpki"
version = "0.101.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8b6275d1ee7a1cd780b64aca7726599a1dbc893b1e64144529e55c3c2f745765"
dependencies = [
 "ring",
 "untrusted",
]

[[package]]
name = "rustls-webpki"
version = "0.103.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d7df23109aa6c1567d1c575b9952556388da57401e4ace1d15f79eedad0d8f53"
dependencies = [
 "ring",
 "rustls-pki-types",
 "untrusted",
]

[[package]]
name = "rustversion"
version = "1.0.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b39cdef0fa800fc44525c84ccb54a029961a8215f9619753635a9c0d2538d46d"

[[package]]
name = "ryu"
version = "1.0.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a50f4cf475b65d88e057964e0e9bb1f0aa9bbb2036dc65c64596b42932536984"

[[package]]
name = "same-file"
version = "1.0.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502"
dependencies = [
 "winapi-util",
]

[[package]]
name = "schannel"
version = "0.1.28"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "891d81b926048e76efe18581bf793546b4c0eaf8448d72be8de2bbee5fd166e1"
dependencies = [
 "windows-sys 0.61.2",
]

[[package]]
name = "scopeguard"
version = "1.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "94143f37725109f92c262ed2cf5e59bce7498c01bcc1502d7b9afe439a4e9f49"

[[package]]
name = "sct"
version = "0.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "da046153aa2352493d6cb7da4b6e5c0c057d8a1d0a9aa8560baffdd945acd414"
dependencies = [
 "ring",
 "untrusted",
]

[[package]]
name = "security-framework"
version = "2.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "897b2245f0b511c87893af39b033e5ca9cce68824c4d7e7630b5a1d339658d02"
dependencies = [
 "bitflags",
 "core-foundation 0.9.4",
 "core-foundation-sys",
 "libc",
 "security-framework-sys",
]

[[package]]
name = "security-framework"
version = "3.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b3297343eaf830f66ede390ea39da1d462b6b0c1b000f420d0a83f898bbbe6ef"
dependencies = [
 "bitflags",
 "core-foundation 0.10.1",
 "core-foundation-sys",
 "libc",
 "security-framework-sys",
]

[[package]]
name = "security-framework-sys"
version = "2.15.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cc1f0cbffaac4852523ce30d8bd3c5cdc873501d96ff467ca09b6767bb8cd5c0"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "semver"
version = "1.0.27"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d767eb0aabc880b29956c35734170f26ed551a859dbd361d140cdbeca61ab1e2"

[[package]]
name = "serde"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a8e94ea7f378bd32cbbd37198a4a91436180c5bb472411e48b5ec2e2124ae9e"
dependencies = [
 "serde_core",
 "serde_derive",
]

[[package]]
name = "serde-big-array"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "11fc7cc2c76d73e0f27ee52abbd64eec84d46f370c88371120433196934e4b7f"
dependencies = [
 "serde",
]

[[package]]
name = "serde_bytes"
version = "0.11.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a5d440709e79d88e51ac01c4b72fc6cb7314017bb7da9eeff678aa94c10e3ea8"
dependencies = [
 "serde",
 "serde_core",
]

[[package]]
name = "serde_core"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "41d385c7d4ca58e59fc732af25c3983b67ac852c1a25000afe1175de458b67ad"
dependencies = [
 "serde_derive",
]

[[package]]
name = "serde_derive"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d540f220d3187173da220f885ab66608367b6574e925011a9353e4badda91d79"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "serde_json"
version = "1.0.149"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "83fc039473c5595ace860d8c4fafa220ff474b3fc6bfdb4293327f1a37e94d86"
dependencies = [
 "itoa",
 "memchr",
 "serde",
 "serde_core",
 "zmij",
]

[[package]]
name = "serde_urlencoded"
version = "0.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3491c14715ca2294c4d6a88f15e84739788c1d030eed8c110436aafdaa2f3fd"
dependencies = [
 "form_urlencoded",
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "serde_with"
version = "3.16.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4fa237f2807440d238e0364a218270b98f767a00d3dada77b1c53ae88940e2e7"
dependencies = [
 "serde_core",
 "serde_with_macros",
]

[[package]]
name = "serde_with_macros"
version = "3.16.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "52a8e3ca0ca629121f70ab50f95249e5a6f925cc0f6ffe8256c45b728875706c"
dependencies = [
 "darling",
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "sha1"
version = "0.10.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3bf829a2d51ab4a5ddf1352d8470c140cadc8301b2ae1789db023f01cedd6ba"
dependencies = [
 "cfg-if",
 "cpufeatures",
 "digest 0.10.7",
]

[[package]]
name = "sha2"
version = "0.9.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4d58a1e1bf39749807d89cf2d98ac2dfa0ff1cb3faa38fbb64dd88ac8013d800"
dependencies = [
 "block-buffer 0.9.0",
 "cfg-if",
 "cpufeatures",
 "digest 0.9.0",
 "opaque-debug",
]

[[package]]
name = "sha2"
version = "0.10.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a7507d819769d01a365ab707794a4084392c824f54a7a6a7862f8c3d0892b283"
dependencies = [
 "cfg-if",
 "cpufeatures",
 "digest 0.10.7",
]

[[package]]
name = "sha3"
version = "0.10.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "75872d278a8f37ef87fa0ddbda7802605cb18344497949862c0d4dcb291eba60"
dependencies = [
 "digest 0.10.7",
 "keccak",
]

[[package]]
name = "shlex"
version = "1.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0fda2ff0d084019ba4d7c6f371c95d8fd75ce3524c3cb8fb653a3023f6323e64"

[[package]]
name = "signal-hook"
version = "0.3.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d881a16cf4426aa584979d30bd82cb33429027e42122b169753d6ef1085ed6e2"
dependencies = [
 "libc",
 "signal-hook-registry",
]

[[package]]
name = "signal-hook-registry"
version = "1.4.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c4db69cba1110affc0e9f7bcd48bbf87b3f4fc7c61fc9155afd4c469eb3d6c1b"
dependencies = [
 "errno",
 "libc",
]

[[package]]
name = "signature"
version = "1.6.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "74233d3b3b2f6d4b006dc19dee745e73e2a6bfb6f93607cd3b02bd5b00797d7c"

[[package]]
name = "simd-adler32"
version = "0.3.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e320a6c5ad31d271ad523dcf3ad13e2767ad8b1cb8f047f75a8aeaf8da139da2"

[[package]]
name = "siphasher"
version = "0.3.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "38b58827f4464d87d377d175e90bf58eb00fd8716ff0a62f80356b5e61555d0d"

[[package]]
name = "siphasher"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b2aa850e253778c88a04c3d7323b043aeda9d3e30d5971937c1855769763678e"

[[package]]
name = "slab"
version = "0.4.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7a2ae44ef20feb57a68b23d846850f861394c2e02dc425a50098ae8c90267589"

[[package]]
name = "smallvec"
version = "1.15.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "67b1b7a3b5fe4f1376887184045fcf45c69e92af734b7aaddc05fb777b6fbd03"

[[package]]
name = "socket2"
version = "0.5.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e22376abed350d73dd1cd119b57ffccad95b4e585a7cda43e286245ce23c0678"
dependencies = [
 "libc",
 "windows-sys 0.52.0",
]

[[package]]
name = "socket2"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "86f4aa3ad99f2088c990dfa82d367e19cb29268ed67c574d10d0a4bfe71f07e0"
dependencies = [
 "libc",
 "windows-sys 0.60.2",
]

[[package]]
name = "solana-account"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0f949fe4edaeaea78c844023bfc1c898e0b1f5a100f8a8d2d0f85d0a7b090258"
dependencies = [
 "bincode",
 "serde",
 "serde_bytes",
 "serde_derive",
 "solana-account-info",
 "solana-clock",
 "solana-instruction",
 "solana-pubkey",
 "solana-sdk-ids",
 "solana-sysvar",
]

[[package]]
name = "solana-account-decoder-client-types"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5519e8343325b707f17fbed54fcefb325131b692506d0af9e08a539d15e4f8cf"
dependencies = [
 "base64 0.22.1",
 "bs58",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-account",
 "solana-pubkey",
 "zstd",
]

[[package]]
name = "solana-account-info"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c8f5152a288ef1912300fc6efa6c2d1f9bb55d9398eb6c72326360b8063987da"
dependencies = [
 "bincode",
 "serde",
 "solana-program-error",
 "solana-program-memory",
 "solana-pubkey",
]

[[package]]
name = "solana-address-lookup-table-interface"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d1673f67efe870b64a65cb39e6194be5b26527691ce5922909939961a6e6b395"
dependencies = [
 "bincode",
 "bytemuck",
 "serde",
 "serde_derive",
 "solana-clock",
 "solana-instruction",
 "solana-pubkey",
 "solana-sdk-ids",
 "solana-slot-hashes",
]

[[package]]
name = "solana-atomic-u64"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d52e52720efe60465b052b9e7445a01c17550666beec855cce66f44766697bc2"
dependencies = [
 "parking_lot",
]

[[package]]
name = "solana-big-mod-exp"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "75db7f2bbac3e62cfd139065d15bcda9e2428883ba61fc8d27ccb251081e7567"
dependencies = [
 "num-bigint 0.4.6",
 "num-traits",
 "solana-define-syscall",
]

[[package]]
name = "solana-bincode"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "19a3787b8cf9c9fe3dd360800e8b70982b9e5a8af9e11c354b6665dd4a003adc"
dependencies = [
 "bincode",
 "serde",
 "solana-instruction",
]

[[package]]
name = "solana-blake3-hasher"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a1a0801e25a1b31a14494fc80882a036be0ffd290efc4c2d640bfcca120a4672"
dependencies = [
 "blake3",
 "solana-define-syscall",
 "solana-hash",
 "solana-sanitize",
]

[[package]]
name = "solana-bn254"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4420f125118732833f36facf96a27e7b78314b2d642ba07fa9ffdacd8d79e243"
dependencies = [
 "ark-bn254",
 "ark-ec",
 "ark-ff",
 "ark-serialize",
 "bytemuck",
 "solana-define-syscall",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-borsh"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "718333bcd0a1a7aed6655aa66bef8d7fb047944922b2d3a18f49cbc13e73d004"
dependencies = [
 "borsh 0.10.4",
 "borsh 1.6.0",
]

[[package]]
name = "solana-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cc55d1f263e0be4127daf33378d313ea0977f9ffd3fba50fa544ca26722fc695"
dependencies = [
 "async-trait",
 "bincode",
 "dashmap",
 "futures",
 "futures-util",
 "indexmap",
 "indicatif",
 "log",
 "quinn",
 "rayon",
 "solana-account",
 "solana-client-traits",
 "solana-commitment-config",
 "solana-connection-cache",
 "solana-epoch-info",
 "solana-hash",
 "solana-instruction",
 "solana-keypair",
 "solana-measure",
 "solana-message",
 "solana-pubkey",
 "solana-pubsub-client",
 "solana-quic-client",
 "solana-quic-definitions",
 "solana-rpc-client",
 "solana-rpc-client-api",
 "solana-rpc-client-nonce-utils",
 "solana-signature",
 "solana-signer",
 "solana-streamer",
 "solana-thin-client",
 "solana-time-utils",
 "solana-tpu-client",
 "solana-transaction",
 "solana-transaction-error",
 "solana-udp-client",
 "thiserror 2.0.18",
 "tokio",
]

[[package]]
name = "solana-client-traits"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "83f0071874e629f29e0eb3dab8a863e98502ac7aba55b7e0df1803fc5cac72a7"
dependencies = [
 "solana-account",
 "solana-commitment-config",
 "solana-epoch-info",
 "solana-hash",
 "solana-instruction",
 "solana-keypair",
 "solana-message",
 "solana-pubkey",
 "solana-signature",
 "solana-signer",
 "solana-system-interface",
 "solana-transaction",
 "solana-transaction-error",
]

[[package]]
name = "solana-clock"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1bb482ab70fced82ad3d7d3d87be33d466a3498eb8aa856434ff3c0dfc2e2e31"
dependencies = [
 "serde",
 "serde_derive",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-sysvar-id",
]

[[package]]
name = "solana-cluster-type"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7ace9fea2daa28354d107ea879cff107181d85cd4e0f78a2bedb10e1a428c97e"
dependencies = [
 "serde",
 "serde_derive",
 "solana-hash",
]

[[package]]
name = "solana-commitment-config"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ac49c4dde3edfa832de1697e9bcdb7c3b3f7cb7a1981b7c62526c8bb6700fb73"
dependencies = [
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-compute-budget-interface"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8432d2c4c22d0499aa06d62e4f7e333f81777b3d7c96050ae9e5cb71a8c3aee4"
dependencies = [
 "borsh 1.6.0",
 "serde",
 "serde_derive",
 "solana-instruction",
 "solana-sdk-ids",
]

[[package]]
name = "solana-connection-cache"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "45c1cff5ebb26aefff52f1a8e476de70ec1683f8cc6e4a8c86b615842d91f436"
dependencies = [
 "async-trait",
 "bincode",
 "crossbeam-channel",
 "futures-util",
 "indexmap",
 "log",
 "rand 0.8.5",
 "rayon",
 "solana-keypair",
 "solana-measure",
 "solana-metrics",
 "solana-time-utils",
 "solana-transaction-error",
 "thiserror 2.0.18",
 "tokio",
]

[[package]]
name = "solana-cpi"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8dc71126edddc2ba014622fc32d0f5e2e78ec6c5a1e0eb511b85618c09e9ea11"
dependencies = [
 "solana-account-info",
 "solana-define-syscall",
 "solana-instruction",
 "solana-program-error",
 "solana-pubkey",
 "solana-stable-layout",
]

[[package]]
name = "solana-decode-error"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8c781686a18db2f942e70913f7ca15dc120ec38dcab42ff7557db2c70c625a35"
dependencies = [
 "num-traits",
]

[[package]]
name = "solana-define-syscall"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2ae3e2abcf541c8122eafe9a625d4d194b4023c20adde1e251f94e056bb1aee2"

[[package]]
name = "solana-derivation-path"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "939756d798b25c5ec3cca10e06212bdca3b1443cb9bb740a38124f58b258737b"
dependencies = [
 "derivation-path",
 "qstring",
 "uriparse",
]

[[package]]
name = "solana-ed25519-program"
version = "2.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a1feafa1691ea3ae588f99056f4bdd1293212c7ece28243d7da257c443e84753"
dependencies = [
 "bytemuck",
 "bytemuck_derive",
 "ed25519-dalek",
 "solana-feature-set",
 "solana-instruction",
 "solana-precompile-error",
 "solana-sdk-ids",
]

[[package]]
name = "solana-epoch-info"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "90ef6f0b449290b0b9f32973eefd95af35b01c5c0c34c569f936c34c5b20d77b"
dependencies = [
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-epoch-rewards"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "86b575d3dd323b9ea10bb6fe89bf6bf93e249b215ba8ed7f68f1a3633f384db7"
dependencies = [
 "serde",
 "serde_derive",
 "solana-hash",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-sysvar-id",
]

[[package]]
name = "solana-epoch-rewards-hasher"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "96c5fd2662ae7574810904585fd443545ed2b568dbd304b25a31e79ccc76e81b"
dependencies = [
 "siphasher 0.3.11",
 "solana-hash",
 "solana-pubkey",
]

[[package]]
name = "solana-epoch-schedule"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3fce071fbddecc55d727b1d7ed16a629afe4f6e4c217bc8d00af3b785f6f67ed"
dependencies = [
 "serde",
 "serde_derive",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-sysvar-id",
]

[[package]]
name = "solana-example-mocks"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "84461d56cbb8bb8d539347151e0525b53910102e4bced875d49d5139708e39d3"
dependencies = [
 "serde",
 "serde_derive",
 "solana-address-lookup-table-interface",
 "solana-clock",
 "solana-hash",
 "solana-instruction",
 "solana-keccak-hasher",
 "solana-message",
 "solana-nonce",
 "solana-pubkey",
 "solana-sdk-ids",
 "solana-system-interface",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-feature-gate-interface"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "43f5c5382b449e8e4e3016fb05e418c53d57782d8b5c30aa372fc265654b956d"
dependencies = [
 "bincode",
 "serde",
 "serde_derive",
 "solana-account",
 "solana-account-info",
 "solana-instruction",
 "solana-program-error",
 "solana-pubkey",
 "solana-rent",
 "solana-sdk-ids",
 "solana-system-interface",
]

[[package]]
name = "solana-feature-set"
version = "2.2.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "93b93971e289d6425f88e6e3cb6668c4b05df78b3c518c249be55ced8efd6b6d"
dependencies = [
 "ahash",
 "lazy_static",
 "solana-epoch-schedule",
 "solana-hash",
 "solana-pubkey",
 "solana-sha256-hasher",
]

[[package]]
name = "solana-fee-calculator"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d89bc408da0fb3812bc3008189d148b4d3e08252c79ad810b245482a3f70cd8d"
dependencies = [
 "log",
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-fee-structure"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "33adf673581c38e810bf618f745bf31b683a0a4a4377682e6aaac5d9a058dd4e"
dependencies = [
 "serde",
 "serde_derive",
 "solana-message",
 "solana-native-token",
]

[[package]]
name = "solana-genesis-config"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b3725085d47b96d37fef07a29d78d2787fc89a0b9004c66eed7753d1e554989f"
dependencies = [
 "bincode",
 "chrono",
 "memmap2",
 "serde",
 "serde_derive",
 "solana-account",
 "solana-clock",
 "solana-cluster-type",
 "solana-epoch-schedule",
 "solana-fee-calculator",
 "solana-hash",
 "solana-inflation",
 "solana-keypair",
 "solana-logger",
 "solana-poh-config",
 "solana-pubkey",
 "solana-rent",
 "solana-sdk-ids",
 "solana-sha256-hasher",
 "solana-shred-version",
 "solana-signer",
 "solana-time-utils",
]

[[package]]
name = "solana-hard-forks"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6c28371f878e2ead55611d8ba1b5fb879847156d04edea13693700ad1a28baf"
dependencies = [
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-hash"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b5b96e9f0300fa287b545613f007dfe20043d7812bee255f418c1eb649c93b63"
dependencies = [
 "borsh 1.6.0",
 "bytemuck",
 "bytemuck_derive",
 "five8",
 "js-sys",
 "serde",
 "serde_derive",
 "solana-atomic-u64",
 "solana-sanitize",
 "wasm-bindgen",
]

[[package]]
name = "solana-inflation"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "23eef6a09eb8e568ce6839573e4966850e85e9ce71e6ae1a6c930c1c43947de3"
dependencies = [
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-instruction"
version = "2.3.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bab5682934bd1f65f8d2c16f21cb532526fcc1a09f796e2cacdb091eee5774ad"
dependencies = [
 "bincode",
 "borsh 1.6.0",
 "getrandom 0.2.17",
 "js-sys",
 "num-traits",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-define-syscall",
 "solana-pubkey",
 "wasm-bindgen",
]

[[package]]
name = "solana-instructions-sysvar"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e0e85a6fad5c2d0c4f5b91d34b8ca47118fc593af706e523cdbedf846a954f57"
dependencies = [
 "bitflags",
 "solana-account-info",
 "solana-instruction",
 "solana-program-error",
 "solana-pubkey",
 "solana-sanitize",
 "solana-sdk-ids",
 "solana-serialize-utils",
 "solana-sysvar-id",
]

[[package]]
name = "solana-keccak-hasher"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c7aeb957fbd42a451b99235df4942d96db7ef678e8d5061ef34c9b34cae12f79"
dependencies = [
 "sha3",
 "solana-define-syscall",
 "solana-hash",
 "solana-sanitize",
]

[[package]]
name = "solana-keypair"
version = "2.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bd3f04aa1a05c535e93e121a95f66e7dcccf57e007282e8255535d24bf1e98bb"
dependencies = [
 "ed25519-dalek",
 "ed25519-dalek-bip32",
 "five8",
 "rand 0.7.3",
 "solana-derivation-path",
 "solana-pubkey",
 "solana-seed-derivable",
 "solana-seed-phrase",
 "solana-signature",
 "solana-signer",
 "wasm-bindgen",
]

[[package]]
name = "solana-last-restart-slot"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4a6360ac2fdc72e7463565cd256eedcf10d7ef0c28a1249d261ec168c1b55cdd"
dependencies = [
 "serde",
 "serde_derive",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-sysvar-id",
]

[[package]]
name = "solana-loader-v2-interface"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d8ab08006dad78ae7cd30df8eea0539e207d08d91eaefb3e1d49a446e1c49654"
dependencies = [
 "serde",
 "serde_bytes",
 "serde_derive",
 "solana-instruction",
 "solana-pubkey",
 "solana-sdk-ids",
]

[[package]]
name = "solana-loader-v3-interface"
version = "5.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6f7162a05b8b0773156b443bccd674ea78bb9aa406325b467ea78c06c99a63a2"
dependencies = [
 "serde",
 "serde_bytes",
 "serde_derive",
 "solana-instruction",
 "solana-pubkey",
 "solana-sdk-ids",
 "solana-system-interface",
]

[[package]]
name = "solana-loader-v4-interface"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "706a777242f1f39a83e2a96a2a6cb034cb41169c6ecbee2cf09cb873d9659e7e"
dependencies = [
 "serde",
 "serde_bytes",
 "serde_derive",
 "solana-instruction",
 "solana-pubkey",
 "solana-sdk-ids",
 "solana-system-interface",
]

[[package]]
name = "solana-logger"
version = "2.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "db8e777ec1afd733939b532a42492d888ec7c88d8b4127a5d867eb45c6eb5cd5"
dependencies = [
 "env_logger",
 "lazy_static",
 "libc",
 "log",
 "signal-hook",
]

[[package]]
name = "solana-measure"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "11dcd67cd2ae6065e494b64e861e0498d046d95a61cbbf1ae3d58be1ea0f42ed"

[[package]]
name = "solana-message"
version = "2.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1796aabce376ff74bf89b78d268fa5e683d7d7a96a0a4e4813ec34de49d5314b"
dependencies = [
 "bincode",
 "blake3",
 "lazy_static",
 "serde",
 "serde_derive",
 "solana-bincode",
 "solana-hash",
 "solana-instruction",
 "solana-pubkey",
 "solana-sanitize",
 "solana-sdk-ids",
 "solana-short-vec",
 "solana-system-interface",
 "solana-transaction-error",
 "wasm-bindgen",
]

[[package]]
name = "solana-metrics"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0375159d8460f423d39e5103dcff6e07796a5ec1850ee1fcfacfd2482a8f34b5"
dependencies = [
 "crossbeam-channel",
 "gethostname",
 "log",
 "reqwest",
 "solana-cluster-type",
 "solana-sha256-hasher",
 "solana-time-utils",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-msg"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f36a1a14399afaabc2781a1db09cb14ee4cc4ee5c7a5a3cfcc601811379a8092"
dependencies = [
 "solana-define-syscall",
]

[[package]]
name = "solana-native-token"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "61515b880c36974053dd499c0510066783f0cc6ac17def0c7ef2a244874cf4a9"

[[package]]
name = "solana-net-utils"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d7a9e831d0f09bd92135d48c5bc79071bb59c0537b9459f1b4dec17ecc0558fa"
dependencies = [
 "anyhow",
 "bincode",
 "bytes",
 "itertools 0.12.1",
 "log",
 "nix",
 "rand 0.8.5",
 "serde",
 "serde_derive",
 "socket2 0.5.10",
 "solana-serde",
 "tokio",
 "url",
]

[[package]]
name = "solana-nonce"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "703e22eb185537e06204a5bd9d509b948f0066f2d1d814a6f475dafb3ddf1325"
dependencies = [
 "serde",
 "serde_derive",
 "solana-fee-calculator",
 "solana-hash",
 "solana-pubkey",
 "solana-sha256-hasher",
]

[[package]]
name = "solana-nonce-account"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cde971a20b8dbf60144d6a84439dda86b5466e00e2843091fe731083cda614da"
dependencies = [
 "solana-account",
 "solana-hash",
 "solana-nonce",
 "solana-sdk-ids",
]

[[package]]
name = "solana-offchain-message"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b526398ade5dea37f1f147ce55dae49aa017a5d7326606359b0445ca8d946581"
dependencies = [
 "num_enum",
 "solana-hash",
 "solana-packet",
 "solana-pubkey",
 "solana-sanitize",
 "solana-sha256-hasher",
 "solana-signature",
 "solana-signer",
]

[[package]]
name = "solana-packet"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "004f2d2daf407b3ec1a1ca5ec34b3ccdfd6866dd2d3c7d0715004a96e4b6d127"
dependencies = [
 "bincode",
 "bitflags",
 "cfg_eval",
 "serde",
 "serde_derive",
 "serde_with",
]

[[package]]
name = "solana-perf"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "37192c0be5c222ca49dbc5667288c5a8bb14837051dd98e541ee4dad160a5da9"
dependencies = [
 "ahash",
 "bincode",
 "bv",
 "bytes",
 "caps",
 "curve25519-dalek 4.1.3",
 "dlopen2",
 "fnv",
 "libc",
 "log",
 "nix",
 "rand 0.8.5",
 "rayon",
 "serde",
 "solana-hash",
 "solana-message",
 "solana-metrics",
 "solana-packet",
 "solana-pubkey",
 "solana-rayon-threadlimit",
 "solana-sdk-ids",
 "solana-short-vec",
 "solana-signature",
 "solana-time-utils",
]

[[package]]
name = "solana-poh-config"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d650c3b4b9060082ac6b0efbbb66865089c58405bfb45de449f3f2b91eccee75"
dependencies = [
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-precompile-error"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4d87b2c1f5de77dfe2b175ee8dd318d196aaca4d0f66f02842f80c852811f9f8"
dependencies = [
 "num-traits",
 "solana-decode-error",
]

[[package]]
name = "solana-precompiles"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "36e92768a57c652edb0f5d1b30a7d0bc64192139c517967c18600debe9ae3832"
dependencies = [
 "lazy_static",
 "solana-ed25519-program",
 "solana-feature-set",
 "solana-message",
 "solana-precompile-error",
 "solana-pubkey",
 "solana-sdk-ids",
 "solana-secp256k1-program",
 "solana-secp256r1-program",
]

[[package]]
name = "solana-presigner"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "81a57a24e6a4125fc69510b6774cd93402b943191b6cddad05de7281491c90fe"
dependencies = [
 "solana-pubkey",
 "solana-signature",
 "solana-signer",
]

[[package]]
name = "solana-program"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "98eca145bd3545e2fbb07166e895370576e47a00a7d824e325390d33bf467210"
dependencies = [
 "bincode",
 "blake3",
 "borsh 0.10.4",
 "borsh 1.6.0",
 "bs58",
 "bytemuck",
 "console_error_panic_hook",
 "console_log",
 "getrandom 0.2.17",
 "lazy_static",
 "log",
 "memoffset",
 "num-bigint 0.4.6",
 "num-derive",
 "num-traits",
 "rand 0.8.5",
 "serde",
 "serde_bytes",
 "serde_derive",
 "solana-account-info",
 "solana-address-lookup-table-interface",
 "solana-atomic-u64",
 "solana-big-mod-exp",
 "solana-bincode",
 "solana-blake3-hasher",
 "solana-borsh",
 "solana-clock",
 "solana-cpi",
 "solana-decode-error",
 "solana-define-syscall",
 "solana-epoch-rewards",
 "solana-epoch-schedule",
 "solana-example-mocks",
 "solana-feature-gate-interface",
 "solana-fee-calculator",
 "solana-hash",
 "solana-instruction",
 "solana-instructions-sysvar",
 "solana-keccak-hasher",
 "solana-last-restart-slot",
 "solana-loader-v2-interface",
 "solana-loader-v3-interface",
 "solana-loader-v4-interface",
 "solana-message",
 "solana-msg",
 "solana-native-token",
 "solana-nonce",
 "solana-program-entrypoint",
 "solana-program-error",
 "solana-program-memory",
 "solana-program-option",
 "solana-program-pack",
 "solana-pubkey",
 "solana-rent",
 "solana-sanitize",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-secp256k1-recover",
 "solana-serde-varint",
 "solana-serialize-utils",
 "solana-sha256-hasher",
 "solana-short-vec",
 "solana-slot-hashes",
 "solana-slot-history",
 "solana-stable-layout",
 "solana-stake-interface",
 "solana-system-interface",
 "solana-sysvar",
 "solana-sysvar-id",
 "solana-vote-interface",
 "thiserror 2.0.18",
 "wasm-bindgen",
]

[[package]]
name = "solana-program-entrypoint"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "32ce041b1a0ed275290a5008ee1a4a6c48f5054c8a3d78d313c08958a06aedbd"
dependencies = [
 "solana-account-info",
 "solana-msg",
 "solana-program-error",
 "solana-pubkey",
]

[[package]]
name = "solana-program-error"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9ee2e0217d642e2ea4bee237f37bd61bb02aec60da3647c48ff88f6556ade775"
dependencies = [
 "borsh 1.6.0",
 "num-traits",
 "serde",
 "serde_derive",
 "solana-decode-error",
 "solana-instruction",
 "solana-msg",
 "solana-pubkey",
]

[[package]]
name = "solana-program-memory"
version = "2.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3a5426090c6f3fd6cfdc10685322fede9ca8e5af43cd6a59e98bfe4e91671712"
dependencies = [
 "solana-define-syscall",
]

[[package]]
name = "solana-program-option"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dc677a2e9bc616eda6dbdab834d463372b92848b2bfe4a1ed4e4b4adba3397d0"

[[package]]
name = "solana-program-pack"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "319f0ef15e6e12dc37c597faccb7d62525a509fec5f6975ecb9419efddeb277b"
dependencies = [
 "solana-program-error",
]

[[package]]
name = "solana-pubkey"
version = "2.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9b62adb9c3261a052ca1f999398c388f1daf558a1b492f60a6d9e64857db4ff1"
dependencies = [
 "borsh 0.10.4",
 "borsh 1.6.0",
 "bytemuck",
 "bytemuck_derive",
 "curve25519-dalek 4.1.3",
 "five8",
 "five8_const",
 "getrandom 0.2.17",
 "js-sys",
 "num-traits",
 "rand 0.8.5",
 "serde",
 "serde_derive",
 "solana-atomic-u64",
 "solana-decode-error",
 "solana-define-syscall",
 "solana-sanitize",
 "solana-sha256-hasher",
 "wasm-bindgen",
]

[[package]]
name = "solana-pubsub-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d18a7476e1d2e8df5093816afd8fffee94fbb6e442d9be8e6bd3e85f88ce8d5c"
dependencies = [
 "crossbeam-channel",
 "futures-util",
 "http 0.2.12",
 "log",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-account-decoder-client-types",
 "solana-clock",
 "solana-pubkey",
 "solana-rpc-client-types",
 "solana-signature",
 "thiserror 2.0.18",
 "tokio",
 "tokio-stream",
 "tokio-tungstenite",
 "tungstenite",
 "url",
]

[[package]]
name = "solana-quic-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "44feb5f4a97494459c435aa56de810500cc24e22d0afc632990a8e54a07c05a4"
dependencies = [
 "async-lock",
 "async-trait",
 "futures",
 "itertools 0.12.1",
 "log",
 "quinn",
 "quinn-proto",
 "rustls 0.23.36",
 "solana-connection-cache",
 "solana-keypair",
 "solana-measure",
 "solana-metrics",
 "solana-net-utils",
 "solana-pubkey",
 "solana-quic-definitions",
 "solana-rpc-client-api",
 "solana-signer",
 "solana-streamer",
 "solana-tls-utils",
 "solana-transaction-error",
 "thiserror 2.0.18",
 "tokio",
]

[[package]]
name = "solana-quic-definitions"
version = "2.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fbf0d4d5b049eb1d0c35f7b18f305a27c8986fc5c0c9b383e97adaa35334379e"
dependencies = [
 "solana-keypair",
]

[[package]]
name = "solana-rayon-threadlimit"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "02cc2a4cae3ef7bb6346b35a60756d2622c297d5fa204f96731db9194c0dc75b"
dependencies = [
 "num_cpus",
]

[[package]]
name = "solana-rent"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d1aea8fdea9de98ca6e8c2da5827707fb3842833521b528a713810ca685d2480"
dependencies = [
 "serde",
 "serde_derive",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-sysvar-id",
]

[[package]]
name = "solana-rent-collector"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "127e6dfa51e8c8ae3aa646d8b2672bc4ac901972a338a9e1cd249e030564fb9d"
dependencies = [
 "serde",
 "serde_derive",
 "solana-account",
 "solana-clock",
 "solana-epoch-schedule",
 "solana-genesis-config",
 "solana-pubkey",
 "solana-rent",
 "solana-sdk-ids",
]

[[package]]
name = "solana-rent-debits"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4f6f9113c6003492e74438d1288e30cffa8ccfdc2ef7b49b9e816d8034da18cd"
dependencies = [
 "solana-pubkey",
 "solana-reward-info",
]

[[package]]
name = "solana-reserved-account-keys"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e4b22ea19ca2a3f28af7cd047c914abf833486bf7a7c4a10fc652fff09b385b1"
dependencies = [
 "lazy_static",
 "solana-feature-set",
 "solana-pubkey",
 "solana-sdk-ids",
]

[[package]]
name = "solana-reward-info"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "18205b69139b1ae0ab8f6e11cdcb627328c0814422ad2482000fa2ca54ae4a2f"
dependencies = [
 "serde",
 "serde_derive",
]

[[package]]
name = "solana-rpc-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8d3161ac0918178e674c1f7f1bfac40de3e7ed0383bd65747d63113c156eaeb"
dependencies = [
 "async-trait",
 "base64 0.22.1",
 "bincode",
 "bs58",
 "futures",
 "indicatif",
 "log",
 "reqwest",
 "reqwest-middleware",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-account",
 "solana-account-decoder-client-types",
 "solana-clock",
 "solana-commitment-config",
 "solana-epoch-info",
 "solana-epoch-schedule",
 "solana-feature-gate-interface",
 "solana-hash",
 "solana-instruction",
 "solana-message",
 "solana-pubkey",
 "solana-rpc-client-api",
 "solana-signature",
 "solana-transaction",
 "solana-transaction-error",
 "solana-transaction-status-client-types",
 "solana-version",
 "solana-vote-interface",
 "tokio",
]

[[package]]
name = "solana-rpc-client-api"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2dbc138685c79d88a766a8fd825057a74ea7a21e1dd7f8de275ada899540fff7"
dependencies = [
 "anyhow",
 "jsonrpc-core",
 "reqwest",
 "reqwest-middleware",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-account-decoder-client-types",
 "solana-clock",
 "solana-rpc-client-types",
 "solana-signer",
 "solana-transaction-error",
 "solana-transaction-status-client-types",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-rpc-client-nonce-utils"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "87f0ee41b9894ff36adebe546a110b899b0d0294b07845d8acdc73822e6af4b0"
dependencies = [
 "solana-account",
 "solana-commitment-config",
 "solana-hash",
 "solana-message",
 "solana-nonce",
 "solana-pubkey",
 "solana-rpc-client",
 "solana-sdk-ids",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-rpc-client-types"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8ea428a81729255d895ea47fba9b30fd4dacbfe571a080448121bd0592751676"
dependencies = [
 "base64 0.22.1",
 "bs58",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-account",
 "solana-account-decoder-client-types",
 "solana-clock",
 "solana-commitment-config",
 "solana-fee-calculator",
 "solana-inflation",
 "solana-pubkey",
 "solana-transaction-error",
 "solana-transaction-status-client-types",
 "solana-version",
 "spl-generic-token",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-sanitize"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "61f1bc1357b8188d9c4a3af3fc55276e56987265eb7ad073ae6f8180ee54cecf"

[[package]]
name = "solana-sdk"
version = "2.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8cc0e4a7635b902791c44b6581bfb82f3ada32c5bc0929a64f39fe4bb384c86a"
dependencies = [
 "bincode",
 "bs58",
 "getrandom 0.1.16",
 "js-sys",
 "serde",
 "serde_json",
 "solana-account",
 "solana-bn254",
 "solana-client-traits",
 "solana-cluster-type",
 "solana-commitment-config",
 "solana-compute-budget-interface",
 "solana-decode-error",
 "solana-derivation-path",
 "solana-ed25519-program",
 "solana-epoch-info",
 "solana-epoch-rewards-hasher",
 "solana-feature-set",
 "solana-fee-structure",
 "solana-genesis-config",
 "solana-hard-forks",
 "solana-inflation",
 "solana-instruction",
 "solana-keypair",
 "solana-message",
 "solana-native-token",
 "solana-nonce-account",
 "solana-offchain-message",
 "solana-packet",
 "solana-poh-config",
 "solana-precompile-error",
 "solana-precompiles",
 "solana-presigner",
 "solana-program",
 "solana-program-memory",
 "solana-pubkey",
 "solana-quic-definitions",
 "solana-rent-collector",
 "solana-rent-debits",
 "solana-reserved-account-keys",
 "solana-reward-info",
 "solana-sanitize",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-secp256k1-program",
 "solana-secp256k1-recover",
 "solana-secp256r1-program",
 "solana-seed-derivable",
 "solana-seed-phrase",
 "solana-serde",
 "solana-serde-varint",
 "solana-short-vec",
 "solana-shred-version",
 "solana-signature",
 "solana-signer",
 "solana-system-transaction",
 "solana-time-utils",
 "solana-transaction",
 "solana-transaction-context",
 "solana-transaction-error",
 "solana-validator-exit",
 "thiserror 2.0.18",
 "wasm-bindgen",
]

[[package]]
name = "solana-sdk-ids"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c5d8b9cc68d5c88b062a33e23a6466722467dde0035152d8fb1afbcdf350a5f"
dependencies = [
 "solana-pubkey",
]

[[package]]
name = "solana-sdk-macro"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "86280da8b99d03560f6ab5aca9de2e38805681df34e0bb8f238e69b29433b9df"
dependencies = [
 "bs58",
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "solana-secp256k1-program"
version = "2.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f19833e4bc21558fe9ec61f239553abe7d05224347b57d65c2218aeeb82d6149"
dependencies = [
 "bincode",
 "digest 0.10.7",
 "libsecp256k1",
 "serde",
 "serde_derive",
 "sha3",
 "solana-feature-set",
 "solana-instruction",
 "solana-precompile-error",
 "solana-sdk-ids",
 "solana-signature",
]

[[package]]
name = "solana-secp256k1-recover"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "baa3120b6cdaa270f39444f5093a90a7b03d296d362878f7a6991d6de3bbe496"
dependencies = [
 "borsh 1.6.0",
 "libsecp256k1",
 "solana-define-syscall",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-secp256r1-program"
version = "2.2.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ce0ae46da3071a900f02d367d99b2f3058fe2e90c5062ac50c4f20cfedad8f0f"
dependencies = [
 "bytemuck",
 "openssl",
 "solana-feature-set",
 "solana-instruction",
 "solana-precompile-error",
 "solana-sdk-ids",
]

[[package]]
name = "solana-seed-derivable"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3beb82b5adb266c6ea90e5cf3967235644848eac476c5a1f2f9283a143b7c97f"
dependencies = [
 "solana-derivation-path",
]

[[package]]
name = "solana-seed-phrase"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "36187af2324f079f65a675ec22b31c24919cb4ac22c79472e85d819db9bbbc15"
dependencies = [
 "hmac 0.12.1",
 "pbkdf2",
 "sha2 0.10.9",
]

[[package]]
name = "solana-serde"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1931484a408af466e14171556a47adaa215953c7f48b24e5f6b0282763818b04"
dependencies = [
 "serde",
]

[[package]]
name = "solana-serde-varint"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2a7e155eba458ecfb0107b98236088c3764a09ddf0201ec29e52a0be40857113"
dependencies = [
 "serde",
]

[[package]]
name = "solana-serialize-utils"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "817a284b63197d2b27afdba829c5ab34231da4a9b4e763466a003c40ca4f535e"
dependencies = [
 "solana-instruction",
 "solana-pubkey",
 "solana-sanitize",
]

[[package]]
name = "solana-sha256-hasher"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5aa3feb32c28765f6aa1ce8f3feac30936f16c5c3f7eb73d63a5b8f6f8ecdc44"
dependencies = [
 "sha2 0.10.9",
 "solana-define-syscall",
 "solana-hash",
]

[[package]]
name = "solana-short-vec"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c54c66f19b9766a56fa0057d060de8378676cb64987533fa088861858fc5a69"
dependencies = [
 "serde",
]

[[package]]
name = "solana-shred-version"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "afd3db0461089d1ad1a78d9ba3f15b563899ca2386351d38428faa5350c60a98"
dependencies = [
 "solana-hard-forks",
 "solana-hash",
 "solana-sha256-hasher",
]

[[package]]
name = "solana-signature"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "64c8ec8e657aecfc187522fc67495142c12f35e55ddeca8698edbb738b8dbd8c"
dependencies = [
 "ed25519-dalek",
 "five8",
 "rand 0.8.5",
 "serde",
 "serde-big-array",
 "serde_derive",
 "solana-sanitize",
]

[[package]]
name = "solana-signer"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7c41991508a4b02f021c1342ba00bcfa098630b213726ceadc7cb032e051975b"
dependencies = [
 "solana-pubkey",
 "solana-signature",
 "solana-transaction-error",
]

[[package]]
name = "solana-slot-hashes"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0c8691982114513763e88d04094c9caa0376b867a29577939011331134c301ce"
dependencies = [
 "serde",
 "serde_derive",
 "solana-hash",
 "solana-sdk-ids",
 "solana-sysvar-id",
]

[[package]]
name = "solana-slot-history"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "97ccc1b2067ca22754d5283afb2b0126d61eae734fc616d23871b0943b0d935e"
dependencies = [
 "bv",
 "serde",
 "serde_derive",
 "solana-sdk-ids",
 "solana-sysvar-id",
]

[[package]]
name = "solana-stable-layout"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9f14f7d02af8f2bc1b5efeeae71bc1c2b7f0f65cd75bcc7d8180f2c762a57f54"
dependencies = [
 "solana-instruction",
 "solana-pubkey",
]

[[package]]
name = "solana-stake-interface"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5269e89fde216b4d7e1d1739cf5303f8398a1ff372a81232abbee80e554a838c"
dependencies = [
 "borsh 0.10.4",
 "borsh 1.6.0",
 "num-traits",
 "serde",
 "serde_derive",
 "solana-clock",
 "solana-cpi",
 "solana-decode-error",
 "solana-instruction",
 "solana-program-error",
 "solana-pubkey",
 "solana-system-interface",
 "solana-sysvar-id",
]

[[package]]
name = "solana-streamer"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5643516e5206b89dd4bdf67c39815606d835a51a13260e43349abdb92d241b1d"
dependencies = [
 "async-channel",
 "bytes",
 "crossbeam-channel",
 "dashmap",
 "futures",
 "futures-util",
 "governor",
 "histogram",
 "indexmap",
 "itertools 0.12.1",
 "libc",
 "log",
 "nix",
 "pem",
 "percentage",
 "quinn",
 "quinn-proto",
 "rand 0.8.5",
 "rustls 0.23.36",
 "smallvec",
 "socket2 0.5.10",
 "solana-keypair",
 "solana-measure",
 "solana-metrics",
 "solana-net-utils",
 "solana-packet",
 "solana-perf",
 "solana-pubkey",
 "solana-quic-definitions",
 "solana-signature",
 "solana-signer",
 "solana-time-utils",
 "solana-tls-utils",
 "solana-transaction-error",
 "solana-transaction-metrics-tracker",
 "thiserror 2.0.18",
 "tokio",
 "tokio-util",
 "x509-parser",
]

[[package]]
name = "solana-svm-feature-set"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f24b836eb4d74ec255217bdbe0f24f64a07adeac31aca61f334f91cd4a3b1d5"

[[package]]
name = "solana-system-interface"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "94d7c18cb1a91c6be5f5a8ac9276a1d7c737e39a21beba9ea710ab4b9c63bc90"
dependencies = [
 "js-sys",
 "num-traits",
 "serde",
 "serde_derive",
 "solana-decode-error",
 "solana-instruction",
 "solana-pubkey",
 "wasm-bindgen",
]

[[package]]
name = "solana-system-transaction"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5bd98a25e5bcba8b6be8bcbb7b84b24c2a6a8178d7fb0e3077a916855ceba91a"
dependencies = [
 "solana-hash",
 "solana-keypair",
 "solana-message",
 "solana-pubkey",
 "solana-signer",
 "solana-system-interface",
 "solana-transaction",
]

[[package]]
name = "solana-sysvar"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8c3595f95069f3d90f275bb9bd235a1973c4d059028b0a7f81baca2703815db"
dependencies = [
 "base64 0.22.1",
 "bincode",
 "bytemuck",
 "bytemuck_derive",
 "lazy_static",
 "serde",
 "serde_derive",
 "solana-account-info",
 "solana-clock",
 "solana-define-syscall",
 "solana-epoch-rewards",
 "solana-epoch-schedule",
 "solana-fee-calculator",
 "solana-hash",
 "solana-instruction",
 "solana-instructions-sysvar",
 "solana-last-restart-slot",
 "solana-program-entrypoint",
 "solana-program-error",
 "solana-program-memory",
 "solana-pubkey",
 "solana-rent",
 "solana-sanitize",
 "solana-sdk-ids",
 "solana-sdk-macro",
 "solana-slot-hashes",
 "solana-slot-history",
 "solana-stake-interface",
 "solana-sysvar-id",
]

[[package]]
name = "solana-sysvar-id"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5762b273d3325b047cfda250787f8d796d781746860d5d0a746ee29f3e8812c1"
dependencies = [
 "solana-pubkey",
 "solana-sdk-ids",
]

[[package]]
name = "solana-thin-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6c1025715a113e0e2e379b30a6bfe4455770dc0759dabf93f7dbd16646d5acbe"
dependencies = [
 "bincode",
 "log",
 "rayon",
 "solana-account",
 "solana-client-traits",
 "solana-clock",
 "solana-commitment-config",
 "solana-connection-cache",
 "solana-epoch-info",
 "solana-hash",
 "solana-instruction",
 "solana-keypair",
 "solana-message",
 "solana-pubkey",
 "solana-rpc-client",
 "solana-rpc-client-api",
 "solana-signature",
 "solana-signer",
 "solana-system-interface",
 "solana-transaction",
 "solana-transaction-error",
]

[[package]]
name = "solana-time-utils"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6af261afb0e8c39252a04d026e3ea9c405342b08c871a2ad8aa5448e068c784c"

[[package]]
name = "solana-tls-utils"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "14494aa87a75a883d1abcfee00f1278a28ecc594a2f030084879eb40570728f6"
dependencies = [
 "rustls 0.23.36",
 "solana-keypair",
 "solana-pubkey",
 "solana-signer",
 "x509-parser",
]

[[package]]
name = "solana-tpu-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "17895ce70fd1dd93add3fbac87d599954ded93c63fa1c66f702d278d96a6da14"
dependencies = [
 "async-trait",
 "bincode",
 "futures-util",
 "indexmap",
 "indicatif",
 "log",
 "rayon",
 "solana-client-traits",
 "solana-clock",
 "solana-commitment-config",
 "solana-connection-cache",
 "solana-epoch-schedule",
 "solana-measure",
 "solana-message",
 "solana-net-utils",
 "solana-pubkey",
 "solana-pubsub-client",
 "solana-quic-definitions",
 "solana-rpc-client",
 "solana-rpc-client-api",
 "solana-signature",
 "solana-signer",
 "solana-transaction",
 "solana-transaction-error",
 "thiserror 2.0.18",
 "tokio",
]

[[package]]
name = "solana-transaction"
version = "2.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "80657d6088f721148f5d889c828ca60c7daeedac9a8679f9ec215e0c42bcbf41"
dependencies = [
 "bincode",
 "serde",
 "serde_derive",
 "solana-bincode",
 "solana-feature-set",
 "solana-hash",
 "solana-instruction",
 "solana-keypair",
 "solana-message",
 "solana-precompiles",
 "solana-pubkey",
 "solana-sanitize",
 "solana-sdk-ids",
 "solana-short-vec",
 "solana-signature",
 "solana-signer",
 "solana-system-interface",
 "solana-transaction-error",
 "wasm-bindgen",
]

[[package]]
name = "solana-transaction-context"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "54a312304361987a85b2ef2293920558e6612876a639dd1309daf6d0d59ef2fe"
dependencies = [
 "bincode",
 "serde",
 "serde_derive",
 "solana-account",
 "solana-instruction",
 "solana-instructions-sysvar",
 "solana-pubkey",
 "solana-rent",
 "solana-sdk-ids",
]

[[package]]
name = "solana-transaction-error"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "222a9dc8fdb61c6088baab34fc3a8b8473a03a7a5fd404ed8dd502fa79b67cb1"
dependencies = [
 "serde",
 "serde_derive",
 "solana-instruction",
 "solana-sanitize",
]

[[package]]
name = "solana-transaction-metrics-tracker"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "03fc4e1b6252dc724f5ee69db6229feb43070b7318651580d2174da8baefb993"
dependencies = [
 "base64 0.22.1",
 "bincode",
 "log",
 "rand 0.8.5",
 "solana-packet",
 "solana-perf",
 "solana-short-vec",
 "solana-signature",
]

[[package]]
name = "solana-transaction-status-client-types"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "51f1d7c2387c35850848212244d2b225847666cb52d3bd59a5c409d2c300303d"
dependencies = [
 "base64 0.22.1",
 "bincode",
 "bs58",
 "serde",
 "serde_derive",
 "serde_json",
 "solana-account-decoder-client-types",
 "solana-commitment-config",
 "solana-message",
 "solana-reward-info",
 "solana-signature",
 "solana-transaction",
 "solana-transaction-context",
 "solana-transaction-error",
 "thiserror 2.0.18",
]

[[package]]
name = "solana-udp-client"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2dd36227dd3035ac09a89d4239551d2e3d7d9b177b61ccc7c6d393c3974d0efa"
dependencies = [
 "async-trait",
 "solana-connection-cache",
 "solana-keypair",
 "solana-net-utils",
 "solana-streamer",
 "solana-transaction-error",
 "thiserror 2.0.18",
 "tokio",
]

[[package]]
name = "solana-validator-exit"
version = "2.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7bbf6d7a3c0b28dd5335c52c0e9eae49d0ae489a8f324917faf0ded65a812c1d"

[[package]]
name = "solana-version"
version = "2.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3324d46c7f7b7f5d34bf7dc71a2883bdc072c7b28ca81d0b2167ecec4cf8da9f"
dependencies = [
 "agave-feature-set",
 "rand 0.8.5",
 "semver",
 "serde",
 "serde_derive",
 "solana-sanitize",
 "solana-serde-varint",
]

[[package]]
name = "solana-vote-interface"
version = "2.2.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b80d57478d6599d30acc31cc5ae7f93ec2361a06aefe8ea79bc81739a08af4c3"
dependencies = [
 "bincode",
 "num-derive",
 "num-traits",
 "serde",
 "serde_derive",
 "solana-clock",
 "solana-decode-error",
 "solana-hash",
 "solana-instruction",
 "solana-pubkey",
 "solana-rent",
 "solana-sdk-ids",
 "solana-serde-varint",
 "solana-serialize-utils",
 "solana-short-vec",
 "solana-system-interface",
]

[[package]]
name = "spinning_top"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d96d2d1d716fb500937168cc09353ffdc7a012be8475ac7308e1bdf0e3923300"
dependencies = [
 "lock_api",
]

[[package]]
name = "spl-generic-token"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "741a62a566d97c58d33f9ed32337ceedd4e35109a686e31b1866c5dfa56abddc"
dependencies = [
 "bytemuck",
 "solana-pubkey",
]

[[package]]
name = "stable_deref_trait"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6ce2be8dc25455e1f91df71bfa12ad37d7af1092ae736f3a6cd0e37bc7810596"

[[package]]
name = "strsim"
version = "0.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7da8b5736845d9f2fcb837ea5d9e2628564b3b043a70948a3f0b778838c5fb4f"

[[package]]
name = "subtle"
version = "2.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "13c2bddecc57b384dee18652358fb23172facb8a2c51ccc10d74c157bdea3292"

[[package]]
name = "syn"
version = "1.0.109"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "72b64191b275b66ffe2469e8af2c1cfe3bafa67b529ead792a6d0160888b4237"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "syn"
version = "2.0.114"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d4d107df263a3013ef9b1879b0df87d706ff80f65a86ea879bd9c31f9b307c2a"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "sync_wrapper"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0bf256ce5efdfa370213c1dabab5935a12e49f2c58d15e9eac2870d3b4f27263"
dependencies = [
 "futures-core",
]

[[package]]
name = "synstructure"
version = "0.12.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f36bdaa60a83aca3921b5259d5400cbf5e90fc51931376a9bd4a0eb79aa7210f"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 1.0.109",
 "unicode-xid",
]

[[package]]
name = "synstructure"
version = "0.13.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "728a70f3dbaf5bab7f0c4b1ac8d7ae5ea60a4b5549c8a5914361c99147a709d2"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "system-configuration"
version = "0.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3c879d448e9d986b661742763247d3693ed13609438cf3d006f51f5368a5ba6b"
dependencies = [
 "bitflags",
 "core-foundation 0.9.4",
 "system-configuration-sys",
]

[[package]]
name = "system-configuration-sys"
version = "0.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8e1d1b10ced5ca923a1fcb8d03e96b8d3268065d724548c0211415ff6ac6bac4"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "tempfile"
version = "3.24.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "655da9c7eb6305c55742045d5a8d2037996d61d8de95806335c7c86ce0f82e9c"
dependencies = [
 "fastrand",
 "getrandom 0.3.4",
 "once_cell",
 "rustix",
 "windows-sys 0.61.2",
]

[[package]]
name = "termcolor"
version = "1.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "06794f8f6c5c898b3275aebefa6b8a1cb24cd2c6c79397ab15774837a0bc5755"
dependencies = [
 "winapi-util",
]

[[package]]
name = "thiserror"
version = "1.0.69"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6aaf5339b578ea85b50e080feb250a3e8ae8cfcdff9a461c9ec2904bc923f52"
dependencies = [
 "thiserror-impl 1.0.69",
]

[[package]]
name = "thiserror"
version = "2.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4288b5bcbc7920c07a1149a35cf9590a2aa808e0bc1eafaade0b80947865fbc4"
dependencies = [
 "thiserror-impl 2.0.18",
]

[[package]]
name = "thiserror-impl"
version = "1.0.69"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4fee6c4efc90059e10f81e6d42c60a18f76588c3d74cb83a0b242a2b6c7504c1"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "thiserror-impl"
version = "2.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ebc4ee7f67670e9b64d05fa4253e753e016c6c95ff35b89b7941d6b856dec1d5"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "time"
version = "0.3.47"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "743bd48c283afc0388f9b8827b976905fb217ad9e647fae3a379a9283c4def2c"
dependencies = [
 "deranged",
 "itoa",
 "num-conv",
 "powerfmt",
 "serde_core",
 "time-core",
 "time-macros",
]

[[package]]
name = "time-core"
version = "0.1.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7694e1cfe791f8d31026952abf09c69ca6f6fa4e1a1229e18988f06a04a12dca"

[[package]]
name = "time-macros"
version = "0.2.27"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2e70e4c5a0e0a8a4823ad65dfe1a6930e4f4d756dcd9dd7939022b5e8c501215"
dependencies = [
 "num-conv",
 "time-core",
]

[[package]]
name = "tinystr"
version = "0.8.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "42d3e9c45c09de15d06dd8acf5f4e0e399e85927b7f00711024eb7ae10fa4869"
dependencies = [
 "displaydoc",
 "zerovec",
]

[[package]]
name = "tinyvec"
version = "1.10.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bfa5fdc3bce6191a1dbc8c02d5c8bffcf557bafa17c124c5264a458f1b0613fa"
dependencies = [
 "tinyvec_macros",
]

[[package]]
name = "tinyvec_macros"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20"

[[package]]
name = "tokio"
version = "1.49.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "72a2903cd7736441aac9df9d7688bd0ce48edccaadf181c3b90be801e81d3d86"
dependencies = [
 "bytes",
 "libc",
 "mio",
 "parking_lot",
 "pin-project-lite",
 "signal-hook-registry",
 "socket2 0.6.2",
 "tokio-macros",
 "windows-sys 0.61.2",
]

[[package]]
name = "tokio-macros"
version = "2.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "af407857209536a95c8e56f8231ef2c2e2aff839b22e07a1ffcbc617e9db9fa5"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "tokio-native-tls"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bbae76ab933c85776efabc971569dd6119c580d8f5d448769dec1764bf796ef2"
dependencies = [
 "native-tls",
 "tokio",
]

[[package]]
name = "tokio-rustls"
version = "0.24.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c28327cf380ac148141087fbfb9de9d7bd4e84ab5d2c28fbc911d753de8a7081"
dependencies = [
 "rustls 0.21.12",
 "tokio",
]

[[package]]
name = "tokio-rustls"
version = "0.26.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1729aa945f29d91ba541258c8df89027d5792d85a8841fb65e8bf0f4ede4ef61"
dependencies = [
 "rustls 0.23.36",
 "tokio",
]

[[package]]
name = "tokio-stream"
version = "0.1.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "32da49809aab5c3bc678af03902d4ccddea2a87d028d86392a4b1560c6906c70"
dependencies = [
 "futures-core",
 "pin-project-lite",
 "tokio",
]

[[package]]
name = "tokio-tungstenite"
version = "0.20.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "212d5dcb2a1ce06d81107c3d0ffa3121fe974b73f068c8282cb1c32328113b6c"
dependencies = [
 "futures-util",
 "log",
 "rustls 0.21.12",
 "tokio",
 "tokio-rustls 0.24.1",
 "tungstenite",
 "webpki-roots 0.25.4",
]

[[package]]
name = "tokio-util"
version = "0.7.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9ae9cec805b01e8fc3fd2fe289f89149a9b66dd16786abd8b19cfa7b48cb0098"
dependencies = [
 "bytes",
 "futures-core",
 "futures-sink",
 "pin-project-lite",
 "tokio",
]

[[package]]
name = "toml"
version = "0.5.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f4f7f0dd8d50a853a531c426359045b1998f04219d88799810762cd4ad314234"
dependencies = [
 "serde",
]

[[package]]
name = "toml_datetime"
version = "0.7.5+spec-1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "92e1cfed4a3038bc5a127e35a2d360f145e1f4b971b551a2ba5fd7aedf7e1347"
dependencies = [
 "serde_core",
]

[[package]]
name = "toml_edit"
version = "0.23.10+spec-1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "84c8b9f757e028cee9fa244aea147aab2a9ec09d5325a9b01e0a49730c2b5269"
dependencies = [
 "indexmap",
 "toml_datetime",
 "toml_parser",
 "winnow",
]

[[package]]
name = "toml_parser"
version = "1.0.6+spec-1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a3198b4b0a8e11f09dd03e133c0280504d0801269e9afa46362ffde1cbeebf44"
dependencies = [
 "winnow",
]

[[package]]
name = "tower"
version = "0.5.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ebe5ef63511595f1344e2d5cfa636d973292adc0eec1f0ad45fae9f0851ab1d4"
dependencies = [
 "futures-core",
 "futures-util",
 "pin-project-lite",
 "sync_wrapper",
 "tokio",
 "tower-layer",
 "tower-service",
]

[[package]]
name = "tower-http"
version = "0.6.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d4e6559d53cc268e5031cd8429d05415bc4cb4aefc4aa5d6cc35fbf5b924a1f8"
dependencies = [
 "async-compression",
 "bitflags",
 "bytes",
 "futures-core",
 "futures-util",
 "http 1.4.0",
 "http-body",
 "http-body-util",
 "iri-string",
 "pin-project-lite",
 "tokio",
 "tokio-util",
 "tower",
 "tower-layer",
 "tower-service",
]

[[package]]
name = "tower-layer"
version = "0.3.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "121c2a6cda46980bb0fcd1647ffaf6cd3fc79a013de288782836f6df9c48780e"

[[package]]
name = "tower-service"
version = "0.3.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8df9b6e13f2d32c91b9bd719c00d1958837bc7dec474d94952798cc8e69eeec3"

[[package]]
name = "tracing"
version = "0.1.44"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "63e71662fa4b2a2c3a26f570f037eb95bb1f85397f3cd8076caed2f026a6d100"
dependencies = [
 "log",
 "pin-project-lite",
 "tracing-attributes",
 "tracing-core",
]

[[package]]
name = "tracing-attributes"
version = "0.1.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7490cfa5ec963746568740651ac6781f701c9c5ea257c58e057f3ba8cf69e8da"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "tracing-core"
version = "0.1.36"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "db97caf9d906fbde555dd62fa95ddba9eecfd14cb388e4f491a66d74cd5fb79a"
dependencies = [
 "once_cell",
]

[[package]]
name = "try-lock"
version = "0.2.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e421abadd41a4225275504ea4d6566923418b7f05506fbc9c0fe86ba7396114b"

[[package]]
name = "tungstenite"
version = "0.20.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9e3dac10fd62eaf6617d3a904ae222845979aec67c615d1c842b4002c7666fb9"
dependencies = [
 "byteorder",
 "bytes",
 "data-encoding",
 "http 0.2.12",
 "httparse",
 "log",
 "rand 0.8.5",
 "rustls 0.21.12",
 "sha1",
 "thiserror 1.0.69",
 "url",
 "utf-8",
 "webpki-roots 0.24.0",
]

[[package]]
name = "typenum"
version = "1.19.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "562d481066bde0658276a35467c4af00bdc6ee726305698a55b86e61d7ad82bb"

[[package]]
name = "unicode-ident"
version = "1.0.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9312f7c4f6ff9069b165498234ce8be658059c6728633667c526e27dc2cf1df5"

[[package]]
name = "unicode-width"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b4ac048d71ede7ee76d585517add45da530660ef4390e49b098733c6e897f254"

[[package]]
name = "unicode-xid"
version = "0.2.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ebc1c04c71510c7f702b52b7c350734c9ff1295c464a03335b00bb84fc54f853"

[[package]]
name = "untrusted"
version = "0.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8ecb6da28b8a351d773b68d5825ac39017e680750f980f3a1a85cd8dd28a47c1"

[[package]]
name = "uriparse"
version = "0.6.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0200d0fc04d809396c2ad43f3c95da3582a2556eba8d453c1087f4120ee352ff"
dependencies = [
 "fnv",
 "lazy_static",
]

[[package]]
name = "url"
version = "2.5.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff67a8a4397373c3ef660812acab3268222035010ab8680ec4215f38ba3d0eed"
dependencies = [
 "form_urlencoded",
 "idna",
 "percent-encoding",
 "serde",
]

[[package]]
name = "utf-8"
version = "0.7.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "09cc8ee72d2a9becf2f2febe0205bbed8fc6615b7cb429ad062dc7b7ddd036a9"

[[package]]
name = "utf8_iter"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6c140620e7ffbb22c2dee59cafe6084a59b5ffc27a8859a5f0d494b5d52b6be"

[[package]]
name = "vcpkg"
version = "0.2.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "accd4ea62f7bb7a82fe23066fb0957d48ef677f6eeb8215f372f52e48bb32426"

[[package]]
name = "version_check"
version = "0.9.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0b928f33d975fc6ad9f86c8f283853ad26bdd5b10b7f1542aa2fa15e2289105a"

[[package]]
name = "walkdir"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "29790946404f91d9c5d06f9874efddea1dc06c5efe94541a7d6863108e3a5e4b"
dependencies = [
 "same-file",
 "winapi-util",
]

[[package]]
name = "want"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bfa7760aed19e106de2c7c0b581b509f2f25d3dacaf737cb82ac61bc6d760b0e"
dependencies = [
 "try-lock",
]

[[package]]
name = "wasi"
version = "0.9.0+wasi-snapshot-preview1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cccddf32554fecc6acb585f82a32a72e28b48f8c4c1883ddfeeeaa96f7d8e519"

[[package]]
name = "wasi"
version = "0.11.1+wasi-snapshot-preview1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ccf3ec651a847eb01de73ccad15eb7d99f80485de043efb2f370cd654f4ea44b"

[[package]]
name = "wasip2"
version = "1.0.2+wasi-0.2.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9517f9239f02c069db75e65f174b3da828fe5f5b945c4dd26bd25d89c03ebcf5"
dependencies = [
 "wit-bindgen",
]

[[package]]
name = "wasm-bindgen"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "64024a30ec1e37399cf85a7ffefebdb72205ca1c972291c51512360d90bd8566"
dependencies = [
 "cfg-if",
 "once_cell",
 "rustversion",
 "wasm-bindgen-macro",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-futures"
version = "0.4.58"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "70a6e77fd0ae8029c9ea0063f87c46fde723e7d887703d74ad2616d792e51e6f"
dependencies = [
 "cfg-if",
 "futures-util",
 "js-sys",
 "once_cell",
 "wasm-bindgen",
 "web-sys",
]

[[package]]
name = "wasm-bindgen-macro"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "008b239d9c740232e71bd39e8ef6429d27097518b6b30bdf9086833bd5b6d608"
dependencies = [
 "quote",
 "wasm-bindgen-macro-support",
]

[[package]]
name = "wasm-bindgen-macro-support"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5256bae2d58f54820e6490f9839c49780dff84c65aeab9e772f15d5f0e913a55"
dependencies = [
 "bumpalo",
 "proc-macro2",
 "quote",
 "syn 2.0.114",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-shared"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1f01b580c9ac74c8d8f0c0e4afb04eeef2acf145458e52c03845ee9cd23e3d12"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "web-sys"
version = "0.3.85"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "312e32e551d92129218ea9a2452120f4aabc03529ef03e4d0d82fb2780608598"
dependencies = [
 "js-sys",
 "wasm-bindgen",
]

[[package]]
name = "web-time"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5a6580f308b1fad9207618087a65c04e7a10bc77e02c8e84e9b00dd4b12fa0bb"
dependencies = [
 "js-sys",
 "wasm-bindgen",
]

[[package]]
name = "webpki-root-certs"
version = "1.0.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "36a29fc0408b113f68cf32637857ab740edfafdf460c326cd2afaa2d84cc05dc"
dependencies = [
 "rustls-pki-types",
]

[[package]]
name = "webpki-roots"
version = "0.24.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b291546d5d9d1eab74f069c77749f2cb8504a12caa20f0f2de93ddbf6f411888"
dependencies = [
 "rustls-webpki 0.101.7",
]

[[package]]
name = "webpki-roots"
version = "0.25.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5f20c57d8d7db6d3b86154206ae5d8fba62dd39573114de97c2cb0578251f8e1"

[[package]]
name = "webpki-roots"
version = "1.0.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "12bed680863276c63889429bfd6cab3b99943659923822de1c8a39c49e4d722c"
dependencies = [
 "rustls-pki-types",
]

[[package]]
name = "winapi"
version = "0.3.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"
dependencies = [
 "winapi-i686-pc-windows-gnu",
 "winapi-x86_64-pc-windows-gnu",
]

[[package]]
name = "winapi-i686-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"

[[package]]
name = "winapi-util"
version = "0.1.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c2a7b1c03c876122aa43f3020e6c3c3ee5c05081c9a00739faf7503aeba10d22"
dependencies = [
 "windows-sys 0.61.2",
]

[[package]]
name = "winapi-x86_64-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"

[[package]]
name = "windows-link"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5"

[[package]]
name = "windows-registry"
version = "0.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "02752bf7fbdcce7f2a27a742f798510f3e5ad88dbe84871e5168e2120c3d5720"
dependencies = [
 "windows-link",
 "windows-result",
 "windows-strings",
]

[[package]]
name = "windows-result"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7781fa89eaf60850ac3d2da7af8e5242a5ea78d1a11c49bf2910bb5a73853eb5"
dependencies = [
 "windows-link",
]

[[package]]
name = "windows-strings"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7837d08f69c77cf6b07689544538e017c1bfcf57e34b4c0ff58e6c2cd3b37091"
dependencies = [
 "windows-link",
]

[[package]]
name = "windows-sys"
version = "0.45.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "75283be5efb2831d37ea142365f009c02ec203cd29a3ebecbc093d52315b66d0"
dependencies = [
 "windows-targets 0.42.2",
]

[[package]]
name = "windows-sys"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d"
dependencies = [
 "windows-targets 0.52.6",
]

[[package]]
name = "windows-sys"
version = "0.59.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e38bc4d79ed67fd075bcc251a1c39b32a1776bbe92e5bef1f0bf1f8c531853b"
dependencies = [
 "windows-targets 0.52.6",
]

[[package]]
name = "windows-sys"
version = "0.60.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f2f500e4d28234f72040990ec9d39e3a6b950f9f22d3dba18416c35882612bcb"
dependencies = [
 "windows-targets 0.53.5",
]

[[package]]
name = "windows-sys"
version = "0.61.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc"
dependencies = [
 "windows-link",
]

[[package]]
name = "windows-targets"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8e5180c00cd44c9b1c88adb3693291f1cd93605ded80c250a75d472756b4d071"
dependencies = [
 "windows_aarch64_gnullvm 0.42.2",
 "windows_aarch64_msvc 0.42.2",
 "windows_i686_gnu 0.42.2",
 "windows_i686_msvc 0.42.2",
 "windows_x86_64_gnu 0.42.2",
 "windows_x86_64_gnullvm 0.42.2",
 "windows_x86_64_msvc 0.42.2",
]

[[package]]
name = "windows-targets"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9b724f72796e036ab90c1021d4780d4d3d648aca59e491e6b98e725b84e99973"
dependencies = [
 "windows_aarch64_gnullvm 0.52.6",
 "windows_aarch64_msvc 0.52.6",
 "windows_i686_gnu 0.52.6",
 "windows_i686_gnullvm 0.52.6",
 "windows_i686_msvc 0.52.6",
 "windows_x86_64_gnu 0.52.6",
 "windows_x86_64_gnullvm 0.52.6",
 "windows_x86_64_msvc 0.52.6",
]

[[package]]
name = "windows-targets"
version = "0.53.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4945f9f551b88e0d65f3db0bc25c33b8acea4d9e41163edf90dcd0b19f9069f3"
dependencies = [
 "windows-link",
 "windows_aarch64_gnullvm 0.53.1",
 "windows_aarch64_msvc 0.53.1",
 "windows_i686_gnu 0.53.1",
 "windows_i686_gnullvm 0.53.1",
 "windows_i686_msvc 0.53.1",
 "windows_x86_64_gnu 0.53.1",
 "windows_x86_64_gnullvm 0.53.1",
 "windows_x86_64_msvc 0.53.1",
]

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "597a5118570b68bc08d8d59125332c54f1ba9d9adeedeef5b99b02ba2b0698f8"

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "32a4622180e7a0ec044bb555404c800bc9fd9ec262ec147edd5989ccd0c02cd3"

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a9d8416fa8b42f5c947f8482c43e7d89e73a173cead56d044f6a56104a6d1b53"

[[package]]
name = "windows_aarch64_msvc"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e08e8864a60f06ef0d0ff4ba04124db8b0fb3be5776a5cd47641e942e58c4d43"

[[package]]
name = "windows_aarch64_msvc"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "09ec2a7bb152e2252b53fa7803150007879548bc709c039df7627cabbd05d469"

[[package]]
name = "windows_aarch64_msvc"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b9d782e804c2f632e395708e99a94275910eb9100b2114651e04744e9b125006"

[[package]]
name = "windows_i686_gnu"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c61d927d8da41da96a81f029489353e68739737d3beca43145c8afec9a31a84f"

[[package]]
name = "windows_i686_gnu"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8e9b5ad5ab802e97eb8e295ac6720e509ee4c243f69d781394014ebfe8bbfa0b"

[[package]]
name = "windows_i686_gnu"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "960e6da069d81e09becb0ca57a65220ddff016ff2d6af6a223cf372a506593a3"

[[package]]
name = "windows_i686_gnullvm"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0eee52d38c090b3caa76c563b86c3a4bd71ef1a819287c19d586d7334ae8ed66"

[[package]]
name = "windows_i686_gnullvm"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fa7359d10048f68ab8b09fa71c3daccfb0e9b559aed648a8f95469c27057180c"

[[package]]
name = "windows_i686_msvc"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "44d840b6ec649f480a41c8d80f9c65108b92d89345dd94027bfe06ac444d1060"

[[package]]
name = "windows_i686_msvc"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "240948bc05c5e7c6dabba28bf89d89ffce3e303022809e73deaefe4f6ec56c66"

[[package]]
name = "windows_i686_msvc"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e7ac75179f18232fe9c285163565a57ef8d3c89254a30685b57d83a38d326c2"

[[package]]
name = "windows_x86_64_gnu"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8de912b8b8feb55c064867cf047dda097f92d51efad5b491dfb98f6bbb70cb36"

[[package]]
name = "windows_x86_64_gnu"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "147a5c80aabfbf0c7d901cb5895d1de30ef2907eb21fbbab29ca94c5b08b1a78"

[[package]]
name = "windows_x86_64_gnu"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c3842cdd74a865a8066ab39c8a7a473c0778a3f29370b5fd6b4b9aa7df4a499"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "26d41b46a36d453748aedef1486d5c7a85db22e56aff34643984ea85514e94a3"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "24d5b23dc417412679681396f2b49f3de8c1473deb516bd34410872eff51ed0d"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0ffa179e2d07eee8ad8f57493436566c7cc30ac536a3379fdf008f47f6bb7ae1"

[[package]]
name = "windows_x86_64_msvc"
version = "0.42.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9aec5da331524158c6d1a4ac0ab1541149c0b9505fde06423b02f5ef0106b9f0"

[[package]]
name = "windows_x86_64_msvc"
version = "0.52.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "589f6da84c646204747d1270a2a5661ea66ed1cced2631d546fdfb155959f9ec"

[[package]]
name = "windows_x86_64_msvc"
version = "0.53.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d6bbff5f0aada427a1e5a6da5f1f98158182f26556f345ac9e04d36d0ebed650"

[[package]]
name = "winnow"
version = "0.7.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5a5364e9d77fcdeeaa6062ced926ee3381faa2ee02d3eb83a5c27a8825540829"
dependencies = [
 "memchr",
]

[[package]]
name = "wit-bindgen"
version = "0.51.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d7249219f66ced02969388cf2bb044a09756a083d0fab1e566056b04d9fbcaa5"

[[package]]
name = "writeable"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9edde0db4769d2dc68579893f2306b26c6ecfbe0ef499b013d731b7b9247e0b9"

[[package]]
name = "x509-parser"
version = "0.14.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e0ecbeb7b67ce215e40e3cc7f2ff902f94a223acf44995934763467e7b1febc8"
dependencies = [
 "asn1-rs",
 "base64 0.13.1",
 "data-encoding",
 "der-parser",
 "lazy_static",
 "nom",
 "oid-registry",
 "rusticata-macros",
 "thiserror 1.0.69",
 "time",
]

[[package]]
name = "yoke"
version = "0.8.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "72d6e5c6afb84d73944e5cedb052c4680d5657337201555f9f2a16b7406d4954"
dependencies = [
 "stable_deref_trait",
 "yoke-derive",
 "zerofrom",
]

[[package]]
name = "yoke-derive"
version = "0.8.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b659052874eb698efe5b9e8cf382204678a0086ebf46982b79d6ca3182927e5d"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
 "synstructure 0.13.2",
]

[[package]]
name = "zerocopy"
version = "0.8.35"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fdea86ddd5568519879b8187e1cf04e24fce28f7fe046ceecbce472ff19a2572"
dependencies = [
 "zerocopy-derive",
]

[[package]]
name = "zerocopy-derive"
version = "0.8.35"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0c15e1b46eff7c6c91195752e0eeed8ef040e391cdece7c25376957d5f15df22"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "zerofrom"
version = "0.1.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "50cc42e0333e05660c3587f3bf9d0478688e15d870fab3346451ce7f8c9fbea5"
dependencies = [
 "zerofrom-derive",
]

[[package]]
name = "zerofrom-derive"
version = "0.1.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d71e5d6e06ab090c67b5e44993ec16b72dcbaabc526db883a360057678b48502"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
 "synstructure 0.13.2",
]

[[package]]
name = "zeroize"
version = "1.8.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b97154e67e32c85465826e8bcc1c59429aaaf107c1e4a9e53c8d8ccd5eff88d0"
dependencies = [
 "zeroize_derive",
]

[[package]]
name = "zeroize_derive"
version = "1.4.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85a5b4158499876c763cb03bc4e49185d3cccbabb15b33c627f7884f43db852e"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "zerotrie"
version = "0.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2a59c17a5562d507e4b54960e8569ebee33bee890c70aa3fe7b97e85a9fd7851"
dependencies = [
 "displaydoc",
 "yoke",
 "zerofrom",
]

[[package]]
name = "zerovec"
version = "0.11.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6c28719294829477f525be0186d13efa9a3c602f7ec202ca9e353d310fb9a002"
dependencies = [
 "yoke",
 "zerofrom",
 "zerovec-derive",
]

[[package]]
name = "zerovec-derive"
version = "0.11.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eadce39539ca5cb3985590102671f2567e659fca9666581ad3411d59207951f3"
dependencies = [
 "proc-macro2",
 "quote",
 "syn 2.0.114",
]

[[package]]
name = "zmij"
version = "1.0.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "02aae0f83f69aafc94776e879363e9771d7ecbffe2c7fbb6c14c5e00dfe88439"

[[package]]
name = "zstd"
version = "0.13.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e91ee311a569c327171651566e07972200e76fcfe2242a4fa446149a3881c08a"
dependencies = [
 "zstd-safe",
]

[[package]]
name = "zstd-safe"
version = "7.2.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f49c4d5f0abb602a93fb8736af2a4f4dd9512e36f7f570d66e65ff867ed3b9d"
dependencies = [
 "zstd-sys",
]

[[package]]
name = "zstd-sys"
version = "2.0.16+zstd.1.5.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "91e19ebc2adc8f83e43039e79776e3fda8ca919132d68a1fed6a5faca2683748"
dependencies = [
 "cc",
 "pkg-config",
]

```
      </file>
      <file path="Cargo.toml">
        ```toml
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

```
      </file>
      <file path="FOR_USER.md">
        ```md
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

```
      </file>
      <file path="README.md">
        ```md
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

```
      </file>
      <file path="scripts/tests.sh">
        ```sh
#!/bin/bash
cargo nextest run --features live-tests --test main --run-ignored all

```
      </file>
      <file path="src/analysis.rs">
        ```rs
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

```
      </file>
      <file path="src/bundler/builder/mod.rs">
        ```rs
pub mod types;
pub mod utils;

```
      </file>
      <file path="src/bundler/builder/types.rs">
        ```rs
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

```
      </file>
      <file path="src/bundler/builder/utils.rs">
        ```rs
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

```
      </file>
      <file path="src/bundler/bundle/mod.rs">
        ```rs
pub mod types;

```
      </file>
      <file path="src/bundler/bundle/types.rs">
        ```rs
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

```
      </file>
      <file path="src/bundler/mod.rs">
        ```rs
pub mod builder;
pub mod bundle;
pub mod tests;
pub mod types;

```
      </file>
      <file path="src/bundler/tests.rs">
        ```rs
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

```
      </file>
      <file path="src/bundler/types.rs">
        ```rs
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

```
      </file>
      <file path="src/client/jito_bundler.rs">
        ```rs
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

```
      </file>
      <file path="src/client/mod.rs">
        ```rs
pub mod jito_bundler;
pub mod rpc;
pub mod send;
pub mod simulate;
pub mod status;
mod types;

```
      </file>
      <file path="src/client/rpc.rs">
        ```rs
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

```
      </file>
      <file path="src/client/send.rs">
        ```rs
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

```
      </file>
      <file path="src/client/simulate.rs">
        ```rs
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

```
      </file>
      <file path="src/client/status.rs">
        ```rs
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

```
      </file>
      <file path="src/client/types.rs">
        ```rs
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

```
      </file>
      <file path="src/config/confirm_policy.rs">
        ```rs
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

```
      </file>
      <file path="src/config/jito.rs">
        ```rs
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

```
      </file>
      <file path="src/config/mod.rs">
        ```rs
pub mod confirm_policy;
pub mod jito;
pub mod network;
pub mod tip_strategy;

```
      </file>
      <file path="src/config/network.rs">
        ```rs
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

```
      </file>
      <file path="src/config/tip_strategy.rs">
        ```rs
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

```
      </file>
      <file path="src/constants.rs">
        ```rs
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

```
      </file>
      <file path="src/error.rs">
        ```rs
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

```
      </file>
      <file path="src/lib.rs">
        ```rs
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

```
      </file>
      <file path="src/tip.rs">
        ```rs
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

```
      </file>
      <file path="src/types.rs">
        ```rs
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

```
      </file>
      <file path="tests/build/memo_bundle.rs">
        ```rs
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

```
      </file>
      <file path="tests/build/mod.rs">
        ```rs
mod memo_bundle;

```
      </file>
      <file path="tests/common/mod.rs">
        ```rs
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

```
      </file>
      <file path="tests/main.rs">
        ```rs
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

```
      </file>
      <file path="tests/offline/mod.rs">
        ```rs
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

```
      </file>
      <file path="tests/send/memo_send.rs">
        ```rs
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

```
      </file>
      <file path="tests/send/mod.rs">
        ```rs
mod memo_send;

```
      </file>
      <file path="tests/simulate/helius_simulation.rs">
        ```rs
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

```
      </file>
      <file path="tests/simulate/mod.rs">
        ```rs
mod helius_simulation;

```
      </file>
</files>
`````

## File: scripts/tests.sh
`````bash
#!/bin/bash
cargo nextest run --features live-tests --test main --run-ignored all
`````

## File: src/bundler/builder/mod.rs
`````rust
pub mod types;
pub mod utils;
`````

## File: src/bundler/builder/types.rs
`````rust
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
`````

## File: src/bundler/builder/utils.rs
`````rust
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
`````

## File: src/bundler/bundle/mod.rs
`````rust
pub mod types;
`````

## File: src/bundler/bundle/types.rs
`````rust
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
`````

## File: src/bundler/mod.rs
`````rust
pub mod builder;
pub mod bundle;
pub mod tests;
pub mod types;
`````

## File: src/bundler/tests.rs
`````rust
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
`````

## File: src/bundler/types.rs
`````rust
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
`````

## File: src/client/jito_bundler.rs
`````rust
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
`````

## File: src/client/mod.rs
`````rust
pub mod jito_bundler;
pub mod rpc;
pub mod send;
pub mod simulate;
pub mod status;
mod types;
`````

## File: src/client/rpc.rs
`````rust
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
`````

## File: src/client/send.rs
`````rust
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
`````

## File: src/client/simulate.rs
`````rust
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
`````

## File: src/client/status.rs
`````rust
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
`````

## File: src/client/types.rs
`````rust
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
`````

## File: src/config/confirm_policy.rs
`````rust
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
`````

## File: src/config/jito.rs
`````rust
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
`````

## File: src/config/mod.rs
`````rust
pub mod confirm_policy;
pub mod jito;
pub mod network;
pub mod tip_strategy;
`````

## File: src/config/network.rs
`````rust
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
`````

## File: src/config/tip_strategy.rs
`````rust
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
`````

## File: src/analysis.rs
`````rust
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
`````

## File: src/constants.rs
`````rust
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
`````

## File: src/error.rs
`````rust
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
`````

## File: src/lib.rs
`````rust
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
`````

## File: src/tip.rs
`````rust
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
`````

## File: src/types.rs
`````rust
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
`````

## File: tests/build/memo_bundle.rs
`````rust
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
`````

## File: tests/build/mod.rs
`````rust
mod memo_bundle;
`````

## File: tests/common/mod.rs
`````rust
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
`````

## File: tests/offline/mod.rs
`````rust
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
`````

## File: tests/send/memo_send.rs
`````rust
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
`````

## File: tests/send/mod.rs
`````rust
mod memo_send;
`````

## File: tests/simulate/helius_simulation.rs
`````rust
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
`````

## File: tests/simulate/mod.rs
`````rust
mod helius_simulation;
`````

## File: tests/main.rs
`````rust
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
`````

## File: .gitignore
`````
/target
.idea
/jito-rust-rpc
.env
`````

## File: AGENTS.md
`````markdown
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
`````

## File: Cargo.toml
`````toml
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
`````

## File: CLAUDE.md
`````markdown
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
`````

## File: FOR_USER.md
`````markdown
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
`````

## File: README.md
`````markdown
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
`````
