# Contributing

## Workflow

- Work from short-lived branches (`feature/*`, `fix/*`, `chore/*`).
- Open a pull request into `main`.
- Keep each PR focused and reviewable.

## Quality Bar

Run these commands from the repository root:

```sh
cargo fmt --all -- --check
cargo fmt --manifest-path fuzz/Cargo.toml --all -- --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

The formatting commands cover the main workspace and the separate fuzz workspace. Clippy checks the library, tests, and enabled examples. `cargo test` runs unit tests, integration tests, and doctests, and compiles enabled examples.

### Backend and Optional Feature Checks

CI tests and lints each configuration below on both Rust 1.88.0 and current stable. Windows additionally runs the default and optional-feature test commands; termion is tested on Linux only.

| Configuration | Cargo feature arguments |
| --- | --- |
| Default crossterm 0.29 | No additional arguments |
| Crossterm 0.29 with optional features | `--features search,serde,arbitrary` |
| Crossterm 0.28 | `--no-default-features --features crossterm_0_28,search,serde,arbitrary` |
| Custom backend | `--no-default-features --features no-backend,search,serde,arbitrary` |
| Termwiz | `--no-default-features --features termwiz,search,serde,arbitrary` |
| Termion (Linux) | `--no-default-features --features termion,search,serde,arbitrary` |

For crossterm 0.29, run `cargo test --locked` with the feature arguments from the table. This includes doctests with and without optional features. The documentation examples use crossterm 0.29; test alternative backends with `--lib --tests`, and compile their enabled examples through Clippy. For example:

```sh
cargo test --locked --features search,serde,arbitrary
cargo test --locked --no-default-features --features no-backend,search,serde,arbitrary --lib --tests
cargo clippy --locked --all-targets --no-default-features --features no-backend,search,serde,arbitrary -- -D warnings
```

Use the same Clippy command with each row's feature arguments. Test backends separately instead of using `--all-features`: that enables both crossterm versions and Unix-only termion, and does not verify that an individual backend works on its own. Coverage collection includes the library's unit and integration tests with `search`, `serde`, and `arbitrary` enabled.

### Watch Commands

Install [cargo-watch](https://github.com/watchexec/cargo-watch) with `cargo install cargo-watch --locked`, then run `cargo watch-check` or `cargo watch-test`. These aliases run the default configuration and all portable backend configurations from the table. The test watcher includes doctests for crossterm 0.29. Both watchers use cargo-watch's default file discovery, so changes to the manifest, README doctests, examples, and source files trigger another run.

On Linux, also run `cargo watch-check-termion` or `cargo watch-test-termion` when changing termion support. The portable aliases omit termion so they work on Windows.

## Toolchains

CI runs two Rust toolchains because they catch different problems:

- **1.88.0, the MSRV declared in `Cargo.toml`** -- the required `cargo check`, `cargo test`, and `cargo clippy --all-targets -- -D warnings` checks use this toolchain. The test and Clippy checks cover every configuration in the table. Install it with `rustup toolchain install 1.88.0` and reproduce any command above by inserting `+1.88.0` after `cargo`, for example `cargo +1.88.0 test --locked --features search,serde,arbitrary`.
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
