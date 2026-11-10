use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use tui_textarea::TextArea;

fn history(depth: usize) -> TextArea<'static> {
    let mut textarea = TextArea::from(["seed"]);
    textarea.move_cursor(tui_textarea::CursorMove::End);
    textarea.set_max_histories(depth);
    assert_eq!(textarea.max_histories(), depth);
    for i in 0..depth {
        if i % 2 == 0 {
            assert!(textarea.insert_str("edit"));
        } else {
            assert!(textarea.delete_char());
        }
    }
    textarea
}

fn traversal(c: &mut Criterion) {
    let mut group = c.benchmark_group("history");
    for depth in [50, 200, 500] {
        let undo_ready = history(depth);
        let expected = undo_ready.lines().to_vec();
        let mut redo_ready = undo_ready.clone();
        for _ in 0..depth {
            assert!(redo_ready.undo());
        }
        assert!(!redo_ready.undo(), "history depth must be exactly {depth}");
        assert_eq!(redo_ready.lines(), ["seed"]);
        let mut probe = redo_ready.clone();
        for _ in 0..depth {
            assert!(probe.redo());
        }
        assert!(!probe.redo());
        assert_eq!(probe.lines(), expected);
        group.throughput(Throughput::Elements(depth as u64));
        for (name, initial, operation) in [
            (
                "undo",
                &undo_ready,
                TextArea::undo as fn(&mut TextArea<'static>) -> bool,
            ),
            (
                "redo",
                &redo_ready,
                TextArea::redo as fn(&mut TextArea<'static>) -> bool,
            ),
        ] {
            group.bench_with_input(BenchmarkId::new(name, depth), initial, |b, initial| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        for _ in 0..depth {
                            black_box(operation(textarea));
                        }
                        black_box(textarea.lines());
                        black_box(textarea.cursor());
                    },
                    BatchSize::LargeInput,
                );
            });
        }
    }
    group.finish();
}

fn coalescing(c: &mut Criterion) {
    const TEXT: &str = "alpha beta gamma delta ";
    let chars: Vec<_> = TEXT.chars().collect();
    let mut group = c.benchmark_group("history_typing");
    group.throughput(Throughput::Elements(chars.len() as u64));
    for enabled in [false, true] {
        let mut initial = TextArea::default();
        initial.set_max_histories(chars.len());
        initial.set_undo_coalescing(enabled);
        let mut probe = initial.clone();
        for &ch in &chars {
            probe.insert_char(ch);
        }
        assert_eq!(probe.lines(), [TEXT]);
        let mut steps = 0;
        while probe.undo() {
            steps += 1;
        }
        assert_eq!(probe.lines(), [""]);
        if enabled {
            assert_eq!(steps, 4);
        } else {
            assert_eq!(steps, chars.len());
        }
        group.bench_function(
            if enabled {
                "coalescing_on"
            } else {
                "coalescing_off"
            },
            |b| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        for &ch in &chars {
                            textarea.insert_char(black_box(ch));
                        }
                        black_box(textarea.lines());
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

fn backspace(c: &mut Criterion) {
    const TEXT: &str = "alpha beta gamma delta ";
    let count = TEXT.chars().count();
    let mut group = c.benchmark_group("history_backspace");
    group.throughput(Throughput::Elements(count as u64));
    for enabled in [false, true] {
        let mut initial = TextArea::from([TEXT]);
        initial.move_cursor(tui_textarea::CursorMove::End);
        initial.set_max_histories(count);
        initial.set_undo_coalescing(enabled);
        let mut probe = initial.clone();
        for _ in 0..count {
            assert!(probe.delete_char());
        }
        assert_eq!(probe.lines(), [""]);
        let mut steps = 0;
        while probe.undo() {
            steps += 1;
        }
        assert_eq!(probe.lines(), [TEXT]);
        assert_eq!(steps, if enabled { 5 } else { count });
        group.bench_function(
            if enabled {
                "coalescing_on"
            } else {
                "coalescing_off"
            },
            |b| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        for _ in 0..count {
                            black_box(textarea.delete_char());
                        }
                        black_box(textarea.lines());
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}
criterion_group!(benches, traversal, coalescing, backspace);
criterion_main!(benches);
