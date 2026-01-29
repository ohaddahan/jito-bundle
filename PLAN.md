# Goal

Create a standalone lib for Jito Bundle submission on Solana.

# Design

## Config
- `JitoConfig` with `Network` enum (Mainnet / Custom URLs), RPC URL, optional Helius RPC, optional UUID
- `TipStrategy`: Fixed / FetchFloor / FetchFloorWithCap
- `ConfirmPolicy`: max_attempts + interval_ms
- Optional `jitodontfront_pubkey` for frontrun protection
- Configurable `compute_unit_limit` (default 3M)

## Bundle Rules (Implemented)
- Max 5 transactions per bundle (validated)
- jitodontfront: added as non-signer non-writable account to first tx remaining_accounts
- Tip placement: last instruction of last transaction
- If bundle < 5 txs: tip gets its own separate transaction compiled WITHOUT LUT (empty lookup tables)
- If bundle == 5 txs: tip instruction appended inline to last transaction
- LUT validation: before compilation, verify no LUT contains the randomly selected tip account
- Each transaction validated ≤ 1232 bytes after compilation
- Compute budget instruction prepended to every transaction

## Architecture
```
JitoBundler (facade)
  ├── TipHelper       — random tip account, tip instruction, compile tip tx, fetch floor
  ├── BundleBuilder   — construct Vec<VersionedTransaction> with all rules
  ├── SendHelper      — encode txs, retry across 5 geographic endpoints
  ├── SimulateHelper  — per-tx RPC sim + Helius atomic simulateBundle
  ├── StatusHelper    — poll Jito API + Solana RPC for confirmation
  └── TransactionAnalysis — size checks, LUT coverage diagnostics
```

## Module Map
```
src/
  lib.rs           — re-exports
  config.rs        — JitoConfig, Network, TipStrategy, ConfirmPolicy
  error.rs         — JitoError (thiserror, 14 variants)
  constants.rs     — tip accounts, endpoints, URLs, limits
  types.rs         — BundleStatus, BundleResult, JSON-RPC types, simulation types
  tip.rs           — TipHelper
  send.rs          — SendHelper
  simulate.rs      — SimulateHelper
  status.rs        — StatusHelper
  bundle.rs        — BundleBuilder
  bundler.rs       — JitoBundler (high-level facade)
  analysis.rs      — TransactionAnalysis
```

## Key Decisions
1. Dropped `jito-rust-rpc` as dependency — uses anyhow, untyped JSON, old crate versions
2. Used `thiserror` for typed errors instead of `anyhow`
3. `SYSTEM_PROGRAM_ID` constant avoids deprecated `solana_sdk::system_program`
4. Tip instruction manually encodes system transfer (opcode 2 + LE lamports)
5. Edition 2024 (requires Rust 1.85+)
6. All clippy pedantic lints + panic safety (deny unwrap/expect/panic)

# State Machine

```
[Config + Instructions]
    │
    ▼
┌──────────┐
│FETCH_TIP │ resolve via TipStrategy
└────┬─────┘
     ▼
┌──────────┐  InvalidBundleSize / TipAccountInLut / TransactionOversized
│BUILD     │  BundleBuilder::build()
└────┬─────┘
     ▼
┌──────────┐  SimulationFailed
│SIMULATE  │  Helius or per-tx RPC
└────┬─────┘
     ▼
┌──────────┐  AllEndpointsFailed
│SEND      │  try all 5 endpoints
└────┬─────┘
     ▼
┌──────────┐  ConfirmationTimeout
│CONFIRM   │  poll on-chain signatures
└────┬─────┘
   ┌─┴─┐
   ▼   ▼
LANDED FAILED
```

# References

- Ported from worker-service/src/jito_bundle/ (8 files, ~1200 LOC)
- Jito docs: https://docs.jito.wtf/
