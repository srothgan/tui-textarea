use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use tui_textarea::{CursorMove, Input, Key, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt, LOREM, SEED};

#[inline]
fn random_lorem(repeat: usize) -> usize {
    let mut rng = SmallRng::from_seed(SEED);
    let mut textarea = TextArea::default();
    let mut term = dummy_terminal();

    for _ in 0..repeat {
        for line in LOREM {
            let row = rng.gen_range(0..textarea.lines().len() as u16);
            textarea.move_cursor(CursorMove::Jump(row, 0));
            textarea.move_cursor(CursorMove::End);

            textarea.input(Input {
                key: Key::Enter,
                ctrl: false,
                alt: false,
                shift: false,
            });
            term.draw_textarea(&textarea);

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
    let mut group = c.benchmark_group("insert::random");

    // ~23ms/iter — fits 100 samples in ~5s, keep defaults
    group.bench_function("1_lorem", |b| {
        b.iter(|| std::hint::black_box(random_lorem(1)))
    });

    // ~419ms/iter — 20 samples × 419ms ≈ 8s
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(20);
    group.bench_function("10_lorem", |b| {
        b.iter(|| std::hint::black_box(random_lorem(10)))
    });

    // ~2.1s/iter — 10 samples × 2.1s ≈ 21s
    group.sample_size(10);
    group.bench_function("50_lorem", |b| {
        b.iter(|| std::hint::black_box(random_lorem(50)))
    });

    group.finish();
}

criterion_group!(insert_random, bench);
criterion_main!(insert_random);
