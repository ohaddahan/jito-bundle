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
