use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use tui_textarea::{Input, Key, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt, LOREM};

#[inline]
fn append_lorem(repeat: usize) -> usize {
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
        textarea.input(Input {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        });
        term.draw_textarea(&textarea);
    }
    textarea.lines().len()
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert::append");

    // ~16ms/iter — fits 100 samples in ~6s, keep defaults
    group.bench_function("1_lorem", |b| {
        b.iter(|| std::hint::black_box(append_lorem(1)))
    });

    // ~490ms/iter — 20 samples × 490ms ≈ 10s
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(20);
    group.bench_function("10_lorem", |b| {
        b.iter(|| std::hint::black_box(append_lorem(10)))
    });

    // ~5.9s/iter — 10 samples × 5.9s ≈ 59s (minimum viable)
    group.sample_size(10);
    group.bench_function("50_lorem", |b| {
        b.iter(|| std::hint::black_box(append_lorem(50)))
    });

    group.finish();
}

criterion_group!(insert_append, bench);
criterion_main!(insert_append);
