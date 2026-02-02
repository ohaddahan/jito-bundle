# jito-bundle

Standalone Rust library for submitting [Jito Bundles](https://docs.jito.wtf/) on Solana.

Submit up to 5 transactions that execute **atomically and in order** — the library handles tip calculation, bundle construction, simulation, endpoint retry, and on-chain confirmation.

## Features

- **Atomic bundle construction** — fixed-size `[Option<Vec<Instruction>>; 5]` input, compacted before build to remove gaps while preserving order
- **Automatic tipping** — resolves tip via `TipStrategy`, picks a random tip account, places the tip instruction correctly
- **Endpoint retry** — tries all 5 Jito geographic endpoints (mainnet, Amsterdam, Frankfurt, New York, Tokyo)
- **Helius simulation** — optional atomic `simulateBundle` before sending real SOL
- **Frontrun protection** — optional jitodontfront pubkey support
- **Typed errors** — 15 `thiserror` variants, no string matching required
- **Strict lints** — clippy pedantic + deny on `unwrap`/`expect`/`panic` in library code

## Requirements

- Rust 1.85+ (edition 2024)
- Solana keypair with SOL for tips

## Usage

### Via `JitoBundler` facade (recommended)

```rust
use jito_bundle::client::jito_bundler::{BuildBundleOptions, JitoBundler};
use jito_bundle::config::jito::JitoConfig;

// 1. Configure
let config = JitoConfig::new("https://api.mainnet-beta.solana.com")
    .with_helius_rpc_url("https://mainnet.helius-rpc.com/?api-key=...")
    .with_uuid("your-jito-uuid");

let bundler = JitoBundler::new(config)?;

// 2. Fetch current tip floor
let tip_lamports = bundler.fetch_tip().await?;

// 3. Build bundle
let blockhash = bundler.rpc_client.get_latest_blockhash().await?;
let bundle = bundler.build_bundle(BuildBundleOptions {
    payer: &keypair,
    transactions_instructions: slots, // [Option<Vec<Instruction>>; 5]
    lookup_tables: &[],
    recent_blockhash: blockhash,
    tip_lamports,
})?;

// 4. Simulate + send + confirm
let result = bundler.send_and_confirm(&bundle).await?;
println!("bundle landed: {:?}", result.bundle_id);
```

### Direct `Bundle` construction

```rust
use jito_bundle::bundler::bundle::{Bundle, BundleBuilderInputs};

let bundle = Bundle::new(BundleBuilderInputs {
    payer: &keypair,
    transactions_instructions: slots,
    lookup_tables: &[],
    recent_blockhash: blockhash,
    tip_lamports: 100_000,
    jitodontfront_pubkey: None,
    compute_unit_limit: 3_000_000,
}).build()?;
// bundle.versioned_transaction is ready to encode and send
```

## Configuration

`JitoConfig` uses a builder pattern:

| Method | Description |
|---|---|
| `new(rpc_url)` | Required — Solana RPC URL |
| `.with_network(Network)` | `Mainnet` (default) or `Custom { block_engine_url, tip_floor_url }` |
| `.with_helius_rpc_url(url)` | Enable Helius atomic simulation before sending |
| `.with_uuid(uuid)` | Jito authentication UUID |
| `.with_tip_strategy(TipStrategy)` | `Fixed(u64)`, `FetchFloor` (raw floor), `FetchFloorWithCap { min, max }` (default: floor with cap 100k..10M lamports) |
| `.with_confirm_policy(ConfirmPolicy)` | Polling config (default: 30 attempts, 2s interval) |
| `.with_jitodontfront(pubkey)` | Enable frontrun protection |
| `.with_compute_unit_limit(u32)` | Per-tx compute budget (default: 3M) |

## Bundle Rules

1. Max 5 transactions per bundle
2. Instruction slots are compacted before build (gaps removed, order preserved)
3. jitodontfront goes into first tx's `remaining_accounts` (non-signer, non-writable)
4. Tip instruction is always last instruction of last transaction
5. If bundle < 5 txs: tip is a **separate** transaction compiled **without** LUTs
6. If bundle == 5 txs: tip appended inline to last transaction
7. LUT validation: rejects bundles where any LUT contains the tip account when tip is inline
8. Compute budget instruction prepended to every transaction

## Error Handling

All operations return `Result<T, JitoError>`. Match on specific variants:

```rust
use jito_bundle::JitoError;

match result {
    Err(JitoError::ConfirmationTimeout { attempts }) => { /* retry */ }
    Err(JitoError::SimulationFailed { details }) => { /* debug */ }
    Err(JitoError::AllEndpointsFailed { count, last_error }) => { /* network issue */ }
    Err(e) => { /* other */ }
    Ok(r) => { /* success */ }
}
```

## Testing

```bash
# Unit tests (21 tests, no network required)
cargo test

# Integration tests (requires .env with live credentials)
cargo test -- --ignored
```

Integration tests need a `.env` file:

```env
# Required
KEYPAIR_PATH=/path/to/keypair.json
RPC_URL=https://api.mainnet-beta.solana.com

# Optional
HELIUS_RPC_URL=https://mainnet.helius-rpc.com/?api-key=...
JITO_BLOCK_ENGINE_URL=https://mainnet.block-engine.jito.wtf
JITO_UUID=your-uuid
TIP_LAMPORTS=100000
```

## License

MIT
