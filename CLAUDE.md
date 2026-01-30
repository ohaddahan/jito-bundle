# jito-bundle

Standalone Rust library for submitting Jito Bundles on Solana.

## Crate Structure

11 modules under `src/`:
- `config.rs` — `JitoConfig` with builder pattern, `Network` enum, `TipStrategy`, `ConfirmPolicy`
- `error.rs` — `JitoError` enum with 14 typed variants via `thiserror`
- `constants.rs` — tip accounts, endpoint URLs, limits, `SYSTEM_PROGRAM_ID`
- `types.rs` — JSON-RPC request/response types, `BundleStatus`, simulation types
- `tip.rs` — `TipHelper` (random tip account, create tip ix, compile tip tx, fetch floor)
- `bundle.rs` — `BundleBuilder::build()` (the core bundle construction logic)
- `send.rs` — `JitoBundler` instance methods (encode txs, retry across 5 geographic endpoints)
- `simulate.rs` — `JitoBundler` instance methods (per-tx RPC + Helius atomic simulation)
- `status.rs` — `JitoBundler` instance methods (poll bundle status + on-chain confirmation)
- `analysis.rs` — `TransactionAnalysis` (size checks, LUT coverage diagnostics)
- `bundler.rs` — `JitoBundler` (high-level facade owning HTTP + RPC clients)

## Critical Bundle Rules

1. Max 5 transactions per bundle
2. jitodontfront goes into first tx's remaining_accounts (non-signer, non-writable)
3. Tip instruction is always last ix of last tx
4. If bundle < 5 txs: tip is a SEPARATE transaction compiled WITHOUT LUT
5. If bundle == 5 txs: tip ix appended inline to last tx (with LUT)
6. MUST validate no LUT contains the tip account before compilation
7. Every tx gets a compute budget ix prepended

## Gotchas / Lessons

- `thiserror` auto-interprets fields named `source` as `#[source]`, but `String` doesn't impl `std::error::Error`. Use `reason` instead.
- `solana_sdk::system_program` is deprecated — define `SYSTEM_PROGRAM_ID` via `pubkey!("111...")` macro
- `clippy::allow_attributes = "deny"` means NO `#[allow(...)]` anywhere. Handle casts with explicit conditionals instead.
- Edition 2024 enables let-chains (`if let Some(x) = foo && condition { ... }`)
- Solana crate versions are fragmented: `solana-sdk 2.3.1`, `solana-pubkey 2.4.0`, `solana-compute-budget-interface 2.2.2` — pin exact versions
- `unwrap_or_default()` is fine under `unwrap_used = "deny"` — it's a different method
- `status.rs`, `send.rs`, `simulate.rs` all impl methods on `JitoBundler` (split impl blocks across files). Methods that use owned resources (`http_client`, `rpc_client`, `config`) must be `&self` instance methods, not static methods taking those resources as parameters.

## Dependencies

Solana: `solana-sdk 2.3.1`, `solana-client 2.3.3`, `solana-pubkey 2.4.0`, `solana-instruction 2.3.0`, `solana-compute-budget-interface 2.2.2`, `solana-transaction-status-client-types 2.3.13`
HTTP: `reqwest 0.12` (json), Encoding: `base64 0.22`, `bincode 1.3`, `bs58 0.5`
Error: `thiserror 2`, Async: `tokio 1` (time), Logging: `tracing 0.1`, Random: `rand 0.9`

## Testing

- Unit tests in `tip.rs` (1 test) and `bundle.rs` (15 tests covering jitodontfront, tx count/coverage, bundle size validation, transaction size, and tip scenarios)
- Integration tests in `tests/integration.rs` (2 tests: `build_memo_bundle_succeeds`, `simulate_memo_bundle_on_helius`) — require `.env` with `KEYPAIR_PATH`, `RPC_URL`, and optionally `HELIUS_RPC_URL`
- Run unit tests: `cargo test`
- Run integration tests: `cargo test -- --ignored`

## See Also

- `PLAN.md` — full architecture, state machine, design decisions
- `FOR_USER.md` — plain-language project explanation
