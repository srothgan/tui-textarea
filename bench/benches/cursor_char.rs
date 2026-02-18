use criterion::{criterion_group, criterion_main, Criterion};
use tui_textarea::CursorMove;
use tui_textarea_bench::{prepare_cursor_textarea, run_cursor, Restore};

fn bench(c: &mut Criterion) {
    let textarea = prepare_cursor_textarea();
    c.bench_function("cursor::char::forward", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::Forward],
                Restore::TopLeft,
                1000,
            ))
        })
    });
    c.bench_function("cursor::char::back", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::Back],
                Restore::BottomRight,
                1000,
            ))
        })
    });
    c.bench_function("cursor::char::down", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::Down],
                Restore::TopLeft,
                1000,
            ))
        })
    });
    c.bench_function("cursor::char::up", |b| {
        b.iter(|| {
            std::hint::black_box(run_cursor(
                textarea.clone(),
                &[CursorMove::Up],
                Restore::BottomLeft,
                1000,
            ))
        })
    });
}

criterion_group!(cursor_char, bench);
criterion_main!(cursor_char);
