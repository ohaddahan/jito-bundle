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
