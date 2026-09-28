# Contributing

## Workflow
- Work from short-lived branches (`feature/*`, `fix/*`, `chore/*`).
- Open a pull request into `main`.
- Keep each PR focused and reviewable.

## Quality Bar
- `cargo fmt --check` must pass.
- `cargo clippy --all-targets -- -D warnings` must pass.
- `cargo check` and `cargo test` must pass.
- Do not use `--all-features` for this crate because backend feature sets are mutually exclusive.

## Toolchains
CI runs two Rust toolchains because they catch different problems:
- **1.88.0, the MSRV declared in `Cargo.toml`** -- these are the required checks. They prove the `rust-version` promise still holds for downstream users. Install it with `rustup toolchain install 1.88.0` and reproduce locally with `cargo +1.88.0 test --locked`.
- **Current stable** -- the `cargo check (stable)`, `cargo test (stable)`, and `cargo clippy (stable)` checks. They surface new lints and upstream breakage early and are deliberately *not* required, so a fresh Rust release cannot block unrelated pull requests. Treat a stable-only failure as follow-up work rather than a merge blocker.

## Reviews
- At least one approval is required before merge.
- Keep PR descriptions explicit about behavior changes and compatibility impact.

## Dependency Changes
- Prefer patch/minor updates in batches.
- Handle major updates one crate at a time with tests after each update.
- `ratatui`, `ratatui-core`, and `ratatui-widgets` are split halves of one release and must be bumped together. Dependabot groups them for exactly this reason.
- A dependency bump must not raise the MSRV silently. If a new version needs more than Rust 1.88.0, that is a deliberate decision: update `rust-version`, the CI matrices, and the README in the same pull request.
- Pin GitHub Actions to full commit SHAs with the version in a trailing comment. Floating tags such as `@v7` silently absorb patch and minor releases, hiding them from both review and Dependabot. `dtolnay/rust-toolchain` is pinned the same way; the Rust version it installs lives in the `toolchain:` input, never in the ref.

## Commit Messages
- Use clear, imperative commit messages.
- Mention external references where relevant (for example `upstream PR #118`).
