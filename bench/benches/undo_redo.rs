use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use tui_textarea::TextArea;
use tui_textarea_bench::{dummy_terminal, TerminalExt};

/// Build a textarea with `depth` undo-able edits in history using insert_str,
/// interspersed with delete_str to cover the cursor-clamping code path.
fn make_history(depth: usize) -> TextArea<'static> {
    let mut ta = TextArea::default();
    for i in 0..depth {
        if i % 4 == 3 {
            // Every 4th edit is a delete_str — the operation fixed in e6cc8ab.
            ta.delete_str(3);
        } else {
            ta.insert_str("hello ");
        }
    }
    ta
}

#[inline]
fn undo_all(textarea: &TextArea<'_>) -> usize {
    let mut t = textarea.clone();
    let mut term = dummy_terminal();
    let mut count = 0;
    while t.undo() {
        term.draw_textarea(&t);
        count += 1;
    }
    count
}

#[inline]
fn redo_all(textarea: &TextArea<'_>) -> usize {
    // Start from fully-undone state so redo has work to do.
    let mut t = textarea.clone();
    while t.undo() {}
    let mut term = dummy_terminal();
    let mut count = 0;
    while t.redo() {
        term.draw_textarea(&t);
        count += 1;
    }
    count
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("undo_redo");
    // undo::50 ran Linear with R²=0.19 (Criterion warned "enable flat sampling").
    // All depths benefit from Flat since history-clone cost varies with depth.
    group.sampling_mode(SamplingMode::Flat);

    for depth in [50usize, 200, 500] {
        let ta = make_history(depth);
        group.bench_function(format!("undo::{}", depth), |b| {
            b.iter(|| std::hint::black_box(undo_all(&ta)))
        });
        group.bench_function(format!("redo::{}", depth), |b| {
            b.iter(|| std::hint::black_box(redo_all(&ta)))
        });
    }

    group.finish();
}

criterion_group!(undo_redo, bench);
criterion_main!(undo_redo);
