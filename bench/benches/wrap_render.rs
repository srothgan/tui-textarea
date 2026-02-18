use criterion::{criterion_group, criterion_main, Criterion};
use tui_textarea::{TextArea, WrapMode};
use tui_textarea_bench::{dummy_terminal, TerminalExt, LOREM};

const MODES: &[WrapMode] = &[
    WrapMode::None,
    WrapMode::Word,
    WrapMode::Glyph,
    WrapMode::WordOrGlyph,
];

fn make_textarea(repeat: usize, mode: WrapMode) -> TextArea<'static> {
    let mut lines = Vec::with_capacity(LOREM.len() * repeat);
    for _ in 0..repeat {
        lines.extend(LOREM.iter().map(|s| s.to_string()));
    }
    let mut ta = TextArea::new(lines);
    ta.set_wrap_mode(mode);
    ta
}

// Short text: 7 lines, each ~63 chars. DummyBackend width=40 → all lines wrap.
fn short(c: &mut Criterion) {
    for mode in MODES {
        let ta = make_textarea(1, *mode);
        let name = format!("wrap_render::short::{:?}", mode);
        c.bench_function(&name, |b| {
            b.iter(|| {
                let mut term = dummy_terminal();
                for _ in 0..50 {
                    term.draw_textarea(&ta);
                }
                std::hint::black_box(())
            })
        });
    }
}

// Long text: 70 lines, same wrapping pressure.
fn long(c: &mut Criterion) {
    for mode in MODES {
        let ta = make_textarea(10, *mode);
        let name = format!("wrap_render::long::{:?}", mode);
        c.bench_function(&name, |b| {
            b.iter(|| {
                let mut term = dummy_terminal();
                for _ in 0..50 {
                    term.draw_textarea(&ta);
                }
                std::hint::black_box(())
            })
        });
    }
}

criterion_group!(wrap_render, short, long);
criterion_main!(wrap_render);
