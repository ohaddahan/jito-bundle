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

Under the hood, `JitoBundler` is the single struct that owns all resources (HTTP client, RPC client, config). Its implementation is **split across multiple files** — each file adds a group of related methods via separate `impl JitoBundler` blocks. Two standalone helpers also exist:

- **TipHelper** (`tip.rs`) — Picks a random tip account from the 8 Jito accounts, creates the SOL transfer instruction, and fetches the current tip floor from Jito's API so you're not overpaying or underpaying.

- **Bundle** (`bundler/bundle.rs`) — The core logic. Takes your instructions (via `BundleBuilderInputs`) and applies all the Jito rules: prepends compute budget, handles jitodontfront (frontrun protection), places the tip correctly, validates LUT safety, and checks transaction sizes.

- **Send methods** (`client/send.rs`) — `impl JitoBundler` methods that encode transactions to base64 and send them via JSON-RPC to Jito's block engine. If one endpoint fails, it tries the next. Jito has 5 geographic endpoints (mainnet, Amsterdam, Frankfurt, New York, Tokyo).

- **Simulate methods** (`client/simulate.rs`) — `impl JitoBundler` methods that run your bundle through simulation before sending real SOL. Supports both per-transaction RPC simulation and Helius's atomic `simulateBundle` endpoint.

- **Status methods** (`client/status.rs`) — `impl JitoBundler` methods that poll both the Jito API and Solana RPC to confirm your bundle actually landed. Handles rate limiting (429s), timeouts, and on-chain failures.

This "split impl" pattern keeps each file focused on one concern while keeping all state access through `&self` — no passing around raw clients or config structs.

## The Subtle Rules That Took Time to Get Right

### The Separate Tip Transaction Rule

Jito allows max 5 transactions per bundle. If your bundle has fewer than 5 instructions, the tip goes into its own **separate transaction** compiled **without address lookup tables**. This avoids a subtle issue where the tip account might conflict with LUT entries.

If your bundle already has exactly 5 instructions (the max), there's no room for a separate tip tx, so the tip instruction gets appended to the last transaction inline.

### The LUT Validation

Here's a non-obvious gotcha: if any of your address lookup tables contain a Jito tip account, the transaction will **fail at runtime**. Why? Because LUT lookups produce references with different writability semantics than what the tip transfer needs. The library validates this upfront and gives you a clear error instead of letting you waste SOL on a doomed bundle.

### The jitodontfront Mechanism

"jitodontfront" (Jito-don't-front) is frontrun protection. When enabled, the library adds a special pubkey to the first transaction's account list as a non-signer, non-writable account. This signals to Jito validators: "don't let anyone frontrun this bundle."

## Technology Choices

| Choice | Why |
|---|---|
| `thiserror` over `anyhow` | Typed errors. Callers can match on `JitoError::TooManyTransactions` vs `JitoError::SimulationFailed` instead of parsing strings |
| Own `SYSTEM_PROGRAM_ID` constant | `solana_sdk::system_program` is deprecated. Rather than adding `solana-system-interface`, we define the well-known pubkey directly |
| No `jito-rust-rpc` dependency | That crate uses `anyhow`, `serde_json::Value`, old deps. The ~100 LOC of useful RPC logic was cheaper to rewrite with proper types |
| Edition 2024 | Enables let-chains (`if let Some(x) = foo && condition {}`), which makes the bundle building code much cleaner |

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
│   └── bundle.rs                Bundle struct + BundleBuilderInputs (core build logic)
└── client/
    ├── jito_bundler.rs          JitoBundler facade + BuildBundleOptions input struct
    ├── send.rs                  impl JitoBundler — bundle submission with endpoint retry
    ├── simulate.rs              impl JitoBundler — RPC + Helius simulation
    └── status.rs                impl JitoBundler — landing confirmation polling
```

Notice how `send.rs`, `simulate.rs`, and `status.rs` all add methods to the same `JitoBundler` struct via split `impl` blocks. This keeps files small and focused while keeping all resource access through `&self`.

The facade exposes a clean flow: `fetch_tip()` → `build_bundle(BuildBundleOptions)` → `send_and_confirm()`. The `BuildBundleOptions` input struct follows the Rust convention of using a struct when there are more than three parameters.

## Bugs We Ran Into

1. **`thiserror` source field magic**: Fields named `source` in `thiserror` enums get auto-treated as `#[source]`, but `String` doesn't implement `std::error::Error`. Renamed to `reason` everywhere.

2. **Solana crate version fragmentation**: You can't just write `solana-sdk = "2.3"` and expect everything to resolve. Different Solana crates have different latest 2.x versions: `solana-pubkey` is at 2.4.0, `solana-compute-budget-interface` is at 2.2.2. Pin exact versions.

3. **`allow_attributes_without_reason = "deny"` strictness**: Our lint config denies `#[allow(...)]` without a `reason = "..."`. In test code, we use `#[allow(clippy::unwrap_used, reason = "test code")]`. In production code, we avoid `#[allow]` entirely and handle casts with explicit conditional logic.

4. **Deprecated `system_program` module**: Caught by a deprecation warning. The fix was defining our own `SYSTEM_PROGRAM_ID` constant via the `pubkey!` macro rather than pulling in yet another crate.

## How Good Engineers Think About This

- **Facade pattern**: `JitoBundler` presents a simple API while hiding the complexity of sending, simulating, and confirming bundles. Users don't need to know about endpoint rotation or simulation strategies.

- **Typed errors over string errors**: Every failure mode has its own variant. This means you can write `match err { JitoError::ConfirmationTimeout { .. } => retry(), _ => fail() }` instead of `if err.to_string().contains("timeout")`.

- **Fail-fast validation**: Bundle size, LUT safety, and transaction size are all checked before any network calls. You find out about structural problems immediately, not after waiting for a failed simulation.

- **Defense in depth**: Even though we validate LUTs upfront, we still log comprehensive diagnostics when anything fails downstream. The `TransactionAnalysis` module exists purely for debugging — when a compilation or simulation fails, it tells you exactly which accounts weren't in your LUTs and which transactions were oversized.

- **Split impl blocks**: Rust lets you write `impl MyStruct` in multiple files. We use this to keep `JitoBundler`'s implementation organized by concern — sending, simulating, and status polling each live in their own file, but all access the struct's owned resources through `&self`. This avoids the anti-pattern of passing raw clients and config through static method parameters.

- **Input structs for clarity**: `BuildBundleOptions` and `BundleBuilderInputs` follow the convention of using a struct when a method has more than three parameters. The struct is fully destructured at the call site, ensuring every field is explicitly used — this makes it impossible to silently ignore a new field when the struct grows.

- **Flat flow methods**: The top-level `send_and_confirm` reads like a recipe: simulate → send → log → wait → interpret. Each step is a single named call. Interpretation logic (the match on landing status) is extracted into a private `interpret_landing_status` helper so the orchestrating method stays scannable without scrolling.
