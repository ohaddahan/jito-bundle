# Fix Plan

## Goals
- Make tip placement deterministic and correct even when instruction slots contain gaps.
- Ensure LUT selection and tip/LUT validation align with the actual position of the tip.
- Replace brittle jitodontfront account stripping with explicit pubkey handling.
- Clarify tip-floor clamping semantics vs. strategy intent.
- Add regression tests for the above.

## Plan (Updated)
- [x] **Normalize input slots**
  - Compacted `transactions_instructions` before build to remove gaps while preserving order.

- [x] **Fix tip placement + LUT selection logic**
  - Tip now reliably lands at the end of the compacted bundle; LUT skipping aligns with the tip-only tx.

- [x] **Replace jitodontfront stripping**
  - Removed prefix-based stripping; exact pubkey match only.

- [x] **Clarify tip floor clamping**
  - `FetchFloor` returns raw floor; `FetchFloorWithCap` applies min/max bounds.

- [x] **Add tests**
  - Added regression tests for gap handling, jitodontfront duplication, and tip strategy bounds.

## Remaining / Optional
- Consider adding a public API note or docs snippet explaining slot compaction behavior for callers.
- If this change should be released, update version/changelog and publish.

## Suggested Tests
- `cargo test`
