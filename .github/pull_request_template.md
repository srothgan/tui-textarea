## Problem
Describe the user-visible problem this PR solves.

## Change Summary
List the key code and behavior changes.

## Test Evidence
Required checks. CI runs these on the 1.88.0 MSRV toolchain:
- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo check`
- [ ] `cargo test`

CI additionally runs `cargo check`, `cargo test`, and `cargo clippy` on current stable. Those are informational and do not block the merge.

## Breaking Changes
Describe any API or behavior changes that may break existing users.
