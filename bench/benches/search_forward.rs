use criterion::{criterion_group, criterion_main, Criterion};
use tui_textarea::TextArea;
use tui_textarea_bench::{dummy_terminal, TerminalExt, LOREM};

#[inline]
fn run(pat: &str, mut textarea: TextArea<'_>) {
    let mut term = dummy_terminal();
    textarea.set_search_pattern(pat).unwrap();
    term.draw_textarea(&textarea);
    for _ in 0..100 {
        textarea.search_forward(false);
        term.draw_textarea(&textarea);
    }
    textarea.set_search_pattern(r"").unwrap();
    term.draw_textarea(&textarea);
}

fn short(c: &mut Criterion) {
    let textarea = TextArea::from(LOREM.iter().map(|s| s.to_string()));
    c.bench_function("search::forward_short", |b| {
        b.iter(|| run(r"\w*i\w*", textarea.clone()))
    });
}

fn long(c: &mut Criterion) {
    let mut lines = vec![];
    for _ in 0..10 {
        lines.extend(LOREM.iter().map(|s| s.to_string()));
    }
    let textarea = TextArea::new(lines);
    c.bench_function("search::forward_long", |b| {
        b.iter(|| run(r"[A-Z]\w*", textarea.clone()))
    });
}

criterion_group!(search_forward, short, long);
criterion_main!(search_forward);
