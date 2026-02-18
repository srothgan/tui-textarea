use criterion::{criterion_group, criterion_main, Criterion};
use tui_textarea::CursorMove;
use tui_textarea_bench::{prepare_cursor_textarea, run_cursor, Restore};

fn bench(c: &mut Criterion) {
    let textarea = prepare_cursor_textarea();
    c.bench_function("cursor::word::forward", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::WordForward],
                Restore::TopLeft,
                1000,
            ))
        })
    });
    c.bench_function("cursor::word::back", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::WordBack],
                Restore::BottomRight,
                1000,
            ))
        })
    });
}

criterion_group!(cursor_word, bench);
criterion_main!(cursor_word);
