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
