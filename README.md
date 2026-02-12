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
    println!("bundle landed: {:?}", result.bundle_id);
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
cargo test -- --ignored
```

## License

MIT
