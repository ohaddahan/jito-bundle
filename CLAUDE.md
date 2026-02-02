# jito-bundle

Standalone Rust library for submitting Jito Bundles on Solana.

## Crate Structure

8 public modules under `src/`, organized into 3 subdirectories + 5 root files (18 .rs files total):

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
│   ├── mod.rs                   Re-exports bundle
│   └── bundle.rs                Bundle struct + build() — core bundle construction, [Option<Vec<Instruction>>; 5]
└── client/
    ├── mod.rs                   Re-exports 4 sub-modules
    ├── jito_bundler.rs          JitoBundler facade + BuildBundleOptions input struct
    ├── send.rs                  impl JitoBundler — encode txs, retry across 5 geographic endpoints
    ├── simulate.rs              impl JitoBundler — per-tx RPC + Helius atomic simulation
    └── status.rs                impl JitoBundler — poll bundle status + on-chain confirmation
```

## Critical Bundle Rules

1. Max 5 transactions per bundle (enforced at compile time via `[Option<Vec<Instruction>>; 5]`)
2. Instruction slots are compacted before build (gaps removed, order preserved)
3. jitodontfront goes into first tx's remaining_accounts (non-signer, non-writable)
4. Tip instruction is always last ix of last tx
5. If bundle < 5 txs: tip is a SEPARATE transaction compiled WITHOUT LUT
6. If bundle == 5 txs: tip ix appended inline to last tx (with LUT)
7. MUST validate no LUT contains the tip account when tip is inline
8. Every tx gets a compute budget ix prepended

## Key Structs

- **`JitoBundler`** (`client/jito_bundler.rs`) — Facade owning `JitoConfig`, `reqwest::Client`, `RpcClient`. Methods: `new()`, `jito_post()`, `fetch_tip()`, `build_bundle()`, `simulate_helius()`, `send_and_confirm()`
- **`BuildBundleOptions<'a>`** (`client/jito_bundler.rs`) — Input struct for `build_bundle()` (payer, transactions_instructions, lookup_tables, recent_blockhash, tip_lamports)
- **`Bundle<'a>`** (`bundler/bundle.rs`) — Core bundle type. `BundleBuilderInputs` feeds `Bundle::new()`, then `build()` produces `Vec<VersionedTransaction>`
- **`TipHelper`** (`tip.rs`) — Stateless utility struct with static methods: `get_random_tip_account()`, `create_tip_instruction_to()`, `fetch_tip_floor()`, `resolve_tip()`
- **`TransactionAnalysis`** (`analysis.rs`) — Stateless utility struct with static methods for size checks and LUT diagnostics

## Gotchas / Lessons

- `thiserror` auto-interprets fields named `source` as `#[source]`, but `String` doesn't impl `std::error::Error`. Use `reason` instead.
- `solana_sdk::system_program` is deprecated — define `SYSTEM_PROGRAM_ID` via `pubkey!("111...")` macro
- `clippy::allow_attributes_without_reason = "deny"` means every `#[allow(...)]` needs a `reason = "..."`. Handle casts with explicit conditionals when possible.
- Edition 2024 enables let-chains (`if let Some(x) = foo && condition { ... }`)
- Solana crate versions are fragmented: `solana-sdk 2.3.1`, `solana-pubkey 2.4.0`, `solana-compute-budget-interface 2.2.2` — pin exact versions
- `unwrap_or_default()` is fine under `unwrap_used = "deny"` — it's a different method
- `client/send.rs`, `client/simulate.rs`, `client/status.rs` all impl methods on `JitoBundler` (split impl blocks across files). Methods that use owned resources (`http_client`, `rpc_client`, `config`) must be `&self` instance methods, not static methods taking those resources as parameters.
- The bundle field is named `transactions_instructions` (not `transactions`) — each slot is `Option<Vec<Instruction>>` in a fixed-size array of 5. Use `.iter().flatten()` to iterate only populated slots; use `.rposition()` to find the last populated slot.
- When iterating `Option` arrays in Rust, clippy prefers `.iter().flatten()` over `for slot { if let Some = slot }` (manual_flatten lint). Similarly, use `.enumerate()` instead of manual counter variables (explicit_counter_loop lint).
- `JitoBundler::build_bundle()` takes `BuildBundleOptions` as input struct (per >3 args rule). It fully destructures the struct before forwarding to `Bundle::new()`.
- `jito_post()` appends UUID as both query param and `x-jito-auth` header, but skips the query param for custom networks (`is_custom()` check).

## Dependencies

Solana: `solana-sdk 2.3.1`, `solana-client 2.3.3`, `solana-pubkey 2.4.0`, `solana-instruction 2.3.0`, `solana-compute-budget-interface 2.2.2`, `solana-transaction-status-client-types 2.3.13`
HTTP: `reqwest 0.12` (json), Encoding: `base64 0.22`, `bincode 1.3`, `bs58 0.5`
Error: `thiserror 2`, Async: `tokio 1` (time), Logging: `tracing 0.1`, Random: `rand 0.9`

## Testing

- Unit tests: 21 total — `bundler/bundle.rs` (16), `analysis.rs` (1), `tip.rs` (4)
- Integration tests in multi-module layout under `tests/`:
  - `tests/main.rs` — clippy allows (with reasons) + mod declarations for build, send, simulate
  - `tests/common/mod.rs` — `TestEnv`, `load_test_env()`, `build_jito_config()`, `create_memo_instruction()`, `build_memo_slots()`, `print_bundle_info()`, `print_bundle_result()`
  - `tests/build/memo_bundle.rs` — `build_memo_bundle_succeeds`
  - `tests/send/memo_send.rs` — `send_memo_bundle_succeeds`
  - `tests/simulate/helius_simulation.rs` — `simulate_memo_bundle_on_helius`
- All integration tests are `#[ignore]` (require live network)
- Required `.env` vars: `KEYPAIR_PATH`, `RPC_URL`
- Optional `.env` vars: `HELIUS_RPC_URL`, `JITO_BLOCK_ENGINE_URL`, `JITO_UUID`, `JITODONTFRONT_PUBKEY`, `TIP_LAMPORTS`
- Custom `JITO_BLOCK_ENGINE_URL` → `build_custom_urls()` derives both block engine + tip floor URLs from base
- Run unit tests: `cargo test`
- Run integration tests: `cargo test -- --ignored`

## See Also

- `PLAN.md` — full architecture, state machine, design decisions
- `FOR_USER.md` — plain-language project explanation
