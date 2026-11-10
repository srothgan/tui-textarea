# Fuzzing

These targets exercise public editor behavior using bounded operation traces, the backend-free widget, and in-memory Ratatui buffers. They have no benchmark package dependency.

## Targets and assertions

| Target | Operations and checks |
| --- | --- |
| `editing_history` | Basic Unicode edits, jumps, resets, and undo/redo against an independent flat-character model; a second lane exercises keyboard shortcuts, the wider editing API, history limits, and coalescing. Both check bounds and history text round trips. |
| `selection_atomic` | Selection, copy/cut/paste, valid and rejected atomic ranges, deletion expansion, and resets. Checks selected/yanked text, independently calculated cut/paste/deletion contents, range ordering, atomic boundaries, and history round trips. |
| `layout` | Resizing, scrolling, wrapping, tabs, line numbers, blocks, alignment, masking, placeholders, and row constraints. Checks cached versus fresh measurement, ASCII glyph row counts, repeated rendering, word-wrap aliases, hit-test bounds, and unambiguous ASCII cursor mapping. |
| `search` | Unicode, consuming and zero-width patterns, invalid/empty patterns, and escaped arbitrary literals. Checks exact navigation order for single-character matches, valid match positions, failed-search cursor stability, edits, resets, and history. |

The model uses Unicode scalar positions, matching the public editing API. Undo cursor placement varies by edit kind: history text and availability are checked independently, while the valid returned cursor becomes the origin of subsequent edits. Replacement paste is not assumed to be one undo entry. Empty selections preserve the yank buffer. Overlapping and zero-width patterns receive match-validity checks without imposing an undocumented traversal policy. Mapping equality excludes tab, grapheme, wrapping, and end-of-line ambiguity.

Time-dependent coalescing boundaries remain in deterministic tests. Fuzz targets never sleep or assume a particular wall-clock interval.

## Replay on stable Rust

Run from the repository root with Rust 1.88.0 or later, including on Windows. Disabling default features avoids compiling the native libFuzzer runtime.

```sh
cargo run --manifest-path fuzz/Cargo.toml --locked --no-default-features --example seed_corpus -- --check
cargo run --manifest-path fuzz/Cargo.toml --locked --no-default-features --example replay -- --seeds
cargo clippy --manifest-path fuzz/Cargo.toml --locked --no-default-features --lib --examples -- -D warnings
cargo fmt --manifest-path fuzz/Cargo.toml --all -- --check
```

Replay one crash using the same assertions without a fuzzer or sanitizer:

```sh
cargo run --manifest-path fuzz/Cargo.toml --locked --no-default-features --example replay -- layout fuzz/artifacts/layout/crash-INPUT_HASH
```

## Mutation campaigns on Linux

Install nightly and cargo-fuzz. Native sanitizer campaigns are validated on Linux; stable replay remains available on Windows.

```sh
rustup toolchain install nightly --profile minimal
cargo install cargo-fuzz --locked --version 0.13.2
cargo +nightly fuzz list
cargo +nightly fuzz build
mkdir -p fuzz/corpus/editing_history fuzz/corpus/selection_atomic fuzz/corpus/layout fuzz/corpus/search
for target in editing_history selection_atomic layout search; do
    cargo +nightly fuzz run "$target" "fuzz/corpus/$target" "fuzz/seeds/$target" -- \
        -runs=10000 -max_total_time=120 -max_len=4096 -timeout=10 -rss_limit_mb=2048
done
```

The first corpus directory receives generated inputs; the second supplies checked-in seeds. Keep that order so discoveries do not overwrite deliberate seeds. Iteration and elapsed-time budgets both apply; reaching either ends the campaign. A crash stops that target early and must be investigated before treating the run as passed. Increase both budgets for longer local campaigns.

cargo-fuzz enables AddressSanitizer, debug assertions, and overflow checks by default. Record `rustc +nightly -Vv`, `cargo fuzz --version`, the source revision, and exact command when sharing a finding. Reuse the same toolchain and `fuzz/Cargo.lock` to reproduce it.

## CI campaigns and artifacts

The [Fuzz workflow](../.github/workflows/fuzz.yml) verifies deliberate seeds, replays all seeds, and lints the harness on Linux with Rust 1.88.0 and current stable. The [Windows workflow](../.github/workflows/cross-platform.yml) also verifies and replays seeds on both toolchains. These deterministic checks do not need nightly Rust or a sanitizer.

Native sanitizer campaigns use nightly Rust and cargo-fuzz 0.13.2 on Linux. Every target gets an independent job; one failure does not cancel the others. Pull requests and pushes to `main` run a smoke campaign with `-runs=10000 -max_total_time=60`. Weekly campaigns are scheduled for Monday at 03:23 UTC. To start the same longer campaign manually, open **Actions → Fuzz → Run workflow** and select a branch. Both longer modes use `-runs=1000000 -max_total_time=1800` per target. GitHub must have the workflow on the default branch before the schedule and manual trigger become available.

All campaigns use `-max_len=4096 -timeout=10 -rss_limit_mb=2048`. The iteration or elapsed-time limit ends a campaign, whichever is reached first; the per-input timeout and memory limit turn hangs or excessive memory use into failures. The campaign step has a 32-minute timeout, and each job has a 45-minute limit including installation and compilation. "Nightly" names the Rust toolchain, not the run schedule.

Smoke jobs start from the checked-in seeds. Long jobs restore the latest available generated corpus for each target, then pass the writable corpus directory first and the deliberate seed directory second. Only longer runs on the default branch save new shared corpus snapshots, including discoveries made before a crash. Cache reuse is best effort: an evicted or missing cache starts from seeds. The `fuzz-corpus-v1` cache prefix must change if the trace encoding changes incompatibly. Generated inputs are never committed automatically.

cargo-fuzz has no `--locked` flag. CI fetches dependencies with `cargo fetch --locked`, builds and runs offline, and checks that `fuzz/Cargo.lock` stayed unchanged. The installed nightly compiler can change between runs, so each job records its exact compiler version and source commit.

Each job attempts to upload a `fuzz-<target>-<run-id>-<attempt>` artifact even after failure, retained for 14 days. It contains `target/fuzz-ci/<target>/` with environment details, the fuzz lockfile, build/campaign logs, and the exact campaign command, plus `fuzz/artifacts/<target>/` if a failure produced inputs. A failure before recording starts, or a forcibly terminated runner, may leave no artifact. Download the artifact from the workflow run, inspect the logs, and use the recorded source revision, nightly compiler, and lockfile to reproduce a crash. The commands below also support stable replay and minimization. A confirmed library bug needs a focused regression test, a fix, and a preserved minimized seed; a harness failure needs its assertions corrected before the campaign counts as passing.

## Reproduce and minimize

cargo-fuzz prints the crash path and reproduction command. Substitute the actual artifact path:

```sh
cargo +nightly fuzz run layout fuzz/artifacts/layout/crash-INPUT_HASH
cargo +nightly fuzz tmin layout fuzz/artifacts/layout/crash-INPUT_HASH -- -max_total_time=60 -timeout=10
```

Minimization preserves the failure, which may be a library bug or an incorrect harness assumption. Inspect the trace, add a focused unit/integration regression for a confirmed library bug, and preserve the minimized input under a descriptive name in `fuzz/seeds/<target>/`. Replay all seeds after the fix.

## Seed format and bounds

`src/seeds.rs` defines deliberate traces for empty content, Unicode and joined emoji, malformed UTF-8, CRLF normalization, oversized deletion, history branching, atomic ranges, tiny viewports, extreme signed scrolling, placeholders, cached measurement, and search edge cases. Regression seeds cover zero-width-character deletion, empty selections preserving the yank buffer, atomic deletion clearing stale selections, backward search from end-of-line, and wrapped search-highlight clipping. Regenerate the binary files with:

```sh
cargo run --manifest-path fuzz/Cargo.toml --locked --no-default-features --example seed_corpus
```

The generator rewrites only its named fixtures; additional minimized regression seeds are preserved. `--check` verifies generated fixtures, and `replay --seeds` also runs additional seed files.

The first byte selects one of eight initial documents. In `editing_history`, bit 3 selects the wider API lane. Remaining bytes are operation tags reduced modulo the target's operation count, followed by compact arguments. Counts include zero and `usize::MAX`; words are little-endian; strings use a one-byte length reduced modulo 65 and decode malformed UTF-8 lossily. Missing arguments become zero, so truncated inputs still exercise operations. Layout consumes two hit-test coordinate bytes after every operation. See `src/decode.rs` and each target module for the exact encoding; the seed builder makes intentional traces readable.

Only the first 4,096 bytes and at most 128 operations are consumed. Execution stops after an operation exceeds 128 lines or 2,048 Unicode scalars of content; one operation may cross that threshold. Frames are at most 64 × 12 cells. History storage and history-probe iterations are bounded. libFuzzer's timeout and RSS settings provide campaign-level bounds.

Checked-in seeds live in `seeds/`. Git preserves their exact bytes without line-ending conversion. Generated `corpus/`, crash `artifacts/`, and build `target/` directories are ignored. Old targets used a different encoding: translate useful old regressions into deliberate traces or ordinary tests before reusing them.
