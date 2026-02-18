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

## Reviews
- At least one approval is required before merge.
- Keep PR descriptions explicit about behavior changes and compatibility impact.

## Dependency Changes
- Prefer patch/minor updates in batches.
- Handle major updates one crate at a time with tests after each update.

## Commit Messages
- Use clear, imperative commit messages.
- Mention external references where relevant (for example `upstream PR #118`).
