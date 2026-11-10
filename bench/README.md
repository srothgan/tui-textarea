# Benchmarks

This suite measures the in-process cost of tui-textarea as an embedded Ratatui widget: editing a buffer, navigating it, traversing history, computing layout, painting cells, and searching text. The five targets use Criterion on stable Rust. Run commands from the repository root.

## Run and validate

Compile and run every workload once, including its preflight assertions, without collecting timing samples:

```sh
cargo bench --locked -p tui-textarea-bench --benches -- --test
```

Compile without execution, or lint the benchmark code on the minimum supported Rust version:

```sh
cargo bench --locked -p tui-textarea-bench --benches --no-run
cargo +1.88.0 clippy --locked -p tui-textarea-bench --all-targets -- -D warnings
```

Measure all workloads, one target, or a named workload:

```sh
cargo bench --locked -p tui-textarea-bench --benches
cargo bench --locked -p tui-textarea-bench --bench editing
cargo bench --locked -p tui-textarea-bench --bench layout -- layout_measure
```

There is no custom runner. Cargo selects the targets and Criterion handles filtering, warm-up, sampling, reports, and saved baselines. The library harness is excluded from benchmarking, so Criterion flags reach only Criterion executables.

## Feature coverage

| Target | Measured workloads | Relevant widget behavior |
| --- | --- | --- |
| [editing](benches/editing.rs) | First edits at a known middle position; continuous insert/backspace pairs; character-by-character typing and multiline append at EOF; bulk multiline insertion; dispersed edits; input plus painting; selection copy/cut/replacement; whole-buffer replacement; atomic token deletion and range registration | Editing latency, allocation on the first edit versus retained capacity, bounded-history eviction, clipboard/yank handling, atomic mentions/tokens, and buffer reset |
| [cursor](benches/cursor.rs) | Individual forward/back, up/down, word, and paragraph moves; reusable navigation pairs; head/end and top/bottom pairs; atomic-skip pairs | Direction-specific logical and visual navigation, long-line positions, ASCII and Unicode paths, tabs, word boundaries, and atomic range lookup |
| [layout](benches/layout.rs) | Warm and first-frame painting; width changes; cached and invalidated measurement; hit testing; viewport jumps; line numbers and borders; selection and custom/search highlights; masking; styled placeholders; native cursor mode | Rendering and layout costs in editors, chat composers, password fields, and dynamically sized inputs |
| [history](benches/history.rs) | Exact undo/redo traversals at 50, 200, and 500 edits; continuous typing and backspacing with coalescing enabled and disabled | History capacity and traversal, redo preparation, and edit grouping overhead |
| [search](benches/search.rs) | Forward/backward nearby and distant matches and full-buffer misses; explicit Unicode regex compilation; search followed by painting | Search scaling, compiled-pattern reuse, and the visible update after finding a match |

Fixtures in [workloads.rs](src/workloads.rs) represent an eight-line input, 256 Unicode lines, a 2,048-line document, and a single unbroken 8,192-character token. Unicode text includes wide characters, a combining character, and tabs. Blank lines create paragraphs; unique markers provide exact expected search positions. The unbroken token exercises oversized-word splitting rather than a long line made entirely of short words. Cursor word movement additionally uses a long line containing word separators, starting at column 4,095 so navigation includes position-dependent indexing work.

Word and glyph wrapping are measured separately from unwrapped rendering. `WordOrGlyph` is a compatibility name for `Word`, so it does not need a duplicate performance series. Backend event decoding and terminal I/O are outside this suite: these targets use the shared backend-independent widget implementation. Individual keybinding variants and inexpensive configuration accessors remain covered by correctness tests.

## What the timers include

Mutating one-shot workloads use [Criterion's batched reference timing loop](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html). Buffer construction, cloning, cursor placement, selection, regex preparation, and framebuffer allocation happen in setup; dropping the setup value happens after timing. Library work performed by the measured method, including allocation, history updates, cache invalidation, and destruction of replaced content, stays in the measurement.

First-edit cases start from a newly loaded, rendered, unwrapped buffer with empty history. Cloning can remove spare string capacity, so these cases deliberately measure first-edit allocation. The separate `editing_steady` cases reuse a wrapped buffer, retain capacity, and fill its bounded history before measurement. They return to the original text and cursor after every insert/backspace pair.

Cursor pairs reuse their input and return to their origin after every iteration. This avoids both cloning overhead and the old failure mode of repeatedly attempting movement at a boundary. Single-direction cases use prepared snapshots so forward and backward costs can also be inspected independently. Search cases start from the same position on every iteration. Undo and redo use separate prepared states; preparing redo by undoing the buffer is not timed. Each history fixture verifies its exact usable depth, so a label of 500 means 500 successful operations, rather than a traversal truncated by the default capacity of 50.

A frame means clearing a reusable Ratatui buffer and calling the widget's render implementation. Warm frames reuse layout at the same dimensions. First-frame cases include the first layout at the viewport width. Resize cases shrink from 120 to 40 columns and grow back, including buffer resizing and both paints. Terminal diffing, terminal system calls, and application event-loop scheduling are not included.

These are per-workload wall-clock timings, not CPU-time counters, end-to-end event-to-screen latency, or application latency percentiles. Scheduling and machine load can affect elapsed samples. Allocation costs contribute to the elapsed time, but allocation counts and peak memory are not measured. Prepared snapshots also touch their data before timing; first-frame measurements describe uncached widget layout, not cold hardware caches or file-loading latency.

Each named iteration has an explicit unit:

- An ordinary edit, search, measurement, or frame case performs one operation.
- Cursor and atomic-skip pairs perform two moves.
- Steady-edit pairs perform two edits; their frame variants also paint after each edit.
- Resize and viewport-jump cases perform a round trip with two paints.
- A history traversal performs the number of successful undo/redo operations in its ID.
- Typing/backspace cases process one fixed phrase; throughput counts characters.
- EOF typing cases process 64 characters or four short lines through normalized input events; throughput counts keypresses. Bulk append inserts 32 lines in one call.
- Dispersed-edit scenarios perform 32 cursor jumps and 32 insertions; throughput counts edits.

Preflight assertions run outside the timers and also run in smoke mode. They verify changed text and undo restoration, cursor endpoints, real deletions, exact history depths, clipboard results, search positions, and visible render state. Outputs and selected runtime inputs use [`std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) to discourage optimization from eliminating the intended work. It is a best-effort optimization barrier, not a correctness oracle.

Coalescing benchmarks simulate continuous input without deliberate pauses. The 500 ms grouping timeout uses real time; timeout boundary semantics belong in the existing deterministic history tests, not in throughput measurements.

## Establish and compare baselines

Use one machine, the same Rust toolchain, the same lockfile and feature configuration, and the same benchmark parameters for both runs. Use an idle machine with a consistent power mode. Do not run compilation, fuzzing, or other benchmarks concurrently. These are optimized bench-profile builds, including the workspace's thin LTO setting; debug builds are not performance references.

Criterion's standard defaults are retained: 100 samples, three seconds of warm-up, five seconds of measurement, and 100,000 bootstrap resamples. For a shorter local reference run, specify its budget explicitly:

```sh
cargo +1.99.1 bench --locked -p tui-textarea-bench --benches -- --warm-up-time 0.5 --measurement-time 2 --sample-size 50 --save-baseline widget-v1
```

After changing the implementation, repeat the same command and parameters with `--baseline widget-v1` instead of `--save-baseline widget-v1`. Criterion compares each matching workload against its saved reference. To keep a second independently named snapshot, use `--save-baseline candidate`; tools such as [critcmp](https://github.com/BurntSushi/critcmp) can compare saved snapshots.

Raw samples, estimates, saved baselines, and HTML reports live under `target/criterion/` and are local build artifacts. Record the commit, working-tree changes, Rust version, CPU/OS, selected workloads, feature configuration, profile, and command alongside any results you share. A short reference run supports investigation; repeat suspicious results with the standard or a longer measurement budget before making optimization decisions. Statistical significance alone does not establish a useful latency improvement.

## Migration from the inherited targets

The files were consolidated, while useful workload distinctions were retained:

| Previous targets | Replacement |
| --- | --- |
| `insert_append`, `insert_long`, `insert_random` | `editing_typing` preserves growing-text and append traces; `editing_scenario/32_scattered_edits` preserves dispersed insertion with fixed positions |
| `delete_char`, `delete_word`, `delete_line` | `editing_first_edit` performs a checked, successful deletion from a known position; `editing_steady` adds sustained editing |
| `cursor_char`, `cursor_word`, `cursor_paragraph`, `cursor_edge` | `cursor_step` isolates each direction; `cursor` and `cursor_atomic` measure repeatable pairs without boundary no-ops |
| `undo_redo` | `history` verifies the requested history depth and prepares redo outside the timer |
| `search_forward`, `search_backward` | `search` separates nearby matches, distant matches, and misses; compilation and search-plus-frame have their own cases |
| `wrap_render` | `layout_frame` separates warm painting, first layout, and resizing; measurement and rendering features have additional groups |

The target names and timing boundaries have changed. Establish a fresh reference with this suite; do not compare its numbers directly with the inherited benchmarks. Fuzzing uses its own backend-free harness, documented in [the fuzzing guide](../fuzz/README.md).
