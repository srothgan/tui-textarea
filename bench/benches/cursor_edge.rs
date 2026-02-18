use criterion::{criterion_group, criterion_main, Criterion};
use tui_textarea::CursorMove;
use tui_textarea_bench::{prepare_cursor_textarea, run_cursor, Restore};

fn bench(c: &mut Criterion) {
    let textarea = prepare_cursor_textarea();
    c.bench_function("cursor::edge::head_end", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::End, CursorMove::Head],
                Restore::None,
                500,
            ))
        })
    });
    c.bench_function("cursor::edge::top_bottom", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::Bottom, CursorMove::Top],
                Restore::None,
                500,
            ))
        })
    });
}

criterion_group!(cursor_edge, bench);
criterion_main!(cursor_edge);
