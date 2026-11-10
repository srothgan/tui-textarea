## Problem

Describe the user-visible problem this PR solves.

## Change Summary

List the key code and behavior changes.

## Test Evidence

CI runs the required checks on Rust 1.88.0. See [the contributor guide](https://github.com/srothgan/tui-textarea/blob/main/CONTRIBUTING.md#quality-bar) for commands and backend configurations.

- [ ] Workspace and fuzz formatting checks
- [ ] `cargo check --locked --all-targets`
- [ ] Clippy for the default, optional-feature, and alternative-backend configurations
- [ ] Unit and integration tests for each backend, plus crossterm 0.29 doctests with and without optional features

Include the commands you ran and their results, and identify any configurations you could not check locally.

CI additionally runs the same check, test, and Clippy commands on current stable, and default and optional-feature tests on Windows. The stable checks are informational and are not required by branch protection.

## Breaking Changes

Describe any API or behavior changes that may break existing users.
