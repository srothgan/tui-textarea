// Inserting a long line is slower than multiple short lines into TextArea
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use tui_textarea::{Input, Key, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt, LOREM};

#[inline]
fn append_long_lorem(repeat: usize) -> usize {
    let mut textarea = TextArea::default();
    let mut term = dummy_terminal();

    for _ in 0..repeat {
        for line in LOREM {
            for c in line.chars() {
                textarea.input(Input {
                    key: Key::Char(c),
                    ctrl: false,
                    alt: false,
                    shift: false,
                });
                term.draw_textarea(&textarea);
            }
        }
    }

    textarea.lines().len()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert::long");

    // ~25ms/iter — fits 100 samples in ~6s, keep defaults
    group.bench_function("1_lorem", |b| {
        b.iter(|| std::hint::black_box(append_long_lorem(1)))
    });

    // ~324ms/iter — 20 samples × 324ms ≈ 6s
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(20);
    group.bench_function("5_lorem", |b| {
        b.iter(|| std::hint::black_box(append_long_lorem(5)))
    });

    // ~1.1s/iter — 10 samples × 1.1s ≈ 11s
    group.sample_size(10);
    group.bench_function("10_lorem", |b| {
        b.iter(|| std::hint::black_box(append_long_lorem(10)))
    });

    group.finish();
}

criterion_group!(insert_long, bench);
criterion_main!(insert_long);
