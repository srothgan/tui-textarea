use criterion::{criterion_group, criterion_main, Criterion};
use tui_textarea::{CursorMove, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt, LOREM};

#[inline]
fn run(textarea: &TextArea<'_>) {
    let mut term = dummy_terminal();
    let mut t = textarea.clone();
    t.move_cursor(CursorMove::Jump(u16::MAX, u16::MAX));
    for _ in 0..100 {
        if !t.delete_word() {
            t = textarea.clone();
            t.move_cursor(CursorMove::Jump(u16::MAX, u16::MAX));
        }
        term.draw_textarea(&t);
    }
}

fn bench(c: &mut Criterion) {
    let mut lines = vec![];
    for _ in 0..10 {
        lines.extend(LOREM.iter().map(|s| s.to_string()));
    }
    let textarea = TextArea::new(lines);
    c.bench_function("delete::word", |b| b.iter(|| run(&textarea)));
}

criterion_group!(delete_word, bench);
criterion_main!(delete_word);
