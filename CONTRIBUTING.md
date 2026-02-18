# Contributing

## Workflow
- Work from short-lived branches (`feature/*`, `fix/*`, `chore/*`).
- Open a pull request into `main`.
- Keep each PR focused and reviewable.

## Quality Bar
- `cargo fmt --check` must pass.
- `cargo clippy --all-targets --all-features -- -D warnings` must pass in Linux CI.
- `cargo check --all-features` and `cargo test --all-features` must pass in Linux CI.
- On Windows local development, use default-feature checks (`cargo check` and `cargo test`) because `termion` is Unix-only.

## Reviews
- At least one approval is required before merge.
- Keep PR descriptions explicit about behavior changes and compatibility impact.

## Dependency Changes
- Prefer patch/minor updates in batches.
- Handle major updates one crate at a time with tests after each update.

## Commit Messages
- Use clear, imperative commit messages.
- Mention external references where relevant (for example `upstream PR #118`).
