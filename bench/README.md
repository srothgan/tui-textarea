Benchmarks for tui-textarea using [Criterion.rs][criterion].

## Running all benchmarks (Rust runner — recommended)

The Rust runner executes every benchmark target sequentially and prints
per-target start time, exit status, and wall duration.

```sh
cargo run -p tui-textarea-bench --bin runner
```

Example output:

```
[1/12] START insert_append at 2026-02-18T12:00:00Z
[1/12] DONE  insert_append status=ok duration=42.31s
[2/12] START insert_random at 2026-02-18T12:00:42Z
...
All 14 benchmark targets completed successfully.
```

## Running a single benchmark target

```sh
cargo bench -p tui-textarea-bench --bench insert_append
```

Available targets:

| Target             | What it measures                          |
|--------------------|-------------------------------------------|
| `insert_append`    | Appending lorem lines sequentially        |
| `insert_random`    | Inserting lorem lines at random positions |
| `insert_long`      | Typing a long line without newlines       |
| `search_forward`   | Forward regex search                      |
| `search_backward`  | Backward regex search                     |
| `cursor_char`      | Character-level cursor movement           |
| `cursor_word`      | Word-level cursor movement                |
| `cursor_paragraph` | Paragraph-level cursor movement           |
| `cursor_edge`      | Head/end and top/bottom jumps             |
| `delete_char`      | Deleting one character at a time          |
| `delete_word`      | Deleting one word at a time               |
| `delete_line`      | Deleting to line head                     |
| `wrap_render`      | Rendering with Word/Glyph/WordOrGlyph wrap modes |
| `undo_redo`        | Undo and redo traversal at varying history depths |

## Filtering within a target

Pass a filter string after `--` to run only matching cases:

```sh
cargo bench -p tui-textarea-bench --bench insert_append -- append::10_lorem
```

## Comparing branches with [critcmp][]

```sh
git checkout main
cargo bench -p tui-textarea-bench -- --save-baseline base

git checkout your-feature
cargo bench -p tui-textarea-bench -- --save-baseline change

critcmp base change
```

## Note on `cargo test -p tui-textarea-bench`

Running `cargo test` on this package is a smoke test only — it verifies
that the benchmark code compiles and runs without panicking, but produces
no timing data. Use `cargo bench` or the Rust runner for real measurements.

[criterion]: https://github.com/bheisler/criterion.rs
[critcmp]: https://github.com/BurntSushi/critcmp
