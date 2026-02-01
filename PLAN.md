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
JitoBundler (facade — client/jito_bundler.rs)
  ├── TipHelper              — stateless: random tip account, tip ix, fetch floor  (tip.rs)
  ├── Bundle                 — core build logic, [Option<Vec<Instruction>>; 5]     (bundler/bundle.rs)
  ├── impl JitoBundler send  — encode txs, retry 5 geographic endpoints            (client/send.rs)
  ├── impl JitoBundler sim   — per-tx RPC + Helius atomic simulateBundle           (client/simulate.rs)
  ├── impl JitoBundler poll  — poll Jito API + Solana RPC for confirmation         (client/status.rs)
  └── TransactionAnalysis    — stateless: size checks, LUT coverage diagnostics    (analysis.rs)
```

Note: send/simulate/status are NOT separate structs — they are `impl JitoBundler` blocks split across files.

## Module Map
```
src/
  lib.rs                    — re-exports 8 modules + pub use JitoError
  error.rs                  — JitoError (thiserror, 15 variants)
  constants.rs              — tip accounts, endpoints, URLs, limits, SYSTEM_PROGRAM_ID
  types.rs                  — BundleStatus, BundleResult, JSON-RPC types, simulation types
  tip.rs                    — TipHelper (stateless utility)
  analysis.rs               — TransactionAnalysis (stateless utility)
  config/
    jito.rs                 — JitoConfig (builder pattern)
    network.rs              — Network enum (Mainnet / Custom)
    confirm_policy.rs       — ConfirmPolicy
    tip_strategy.rs         — TipStrategy
  bundler/
    bundle.rs               — Bundle struct + BundleBuilderInputs + build()
  client/
    jito_bundler.rs         — JitoBundler facade + BuildBundleOptions
    send.rs                 — impl JitoBundler (send_bundle, try_endpoint, etc.)
    simulate.rs             — impl JitoBundler (simulate_bundle_helius, etc.)
    status.rs               — impl JitoBundler (wait_for_landing_on_chain, etc.)
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
│BUILD     │  Bundle::build()
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
