use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use tui_textarea::{AtomicRange, CursorMove, TextArea, WrapMode};
use tui_textarea_bench::{document, Frame};

fn pair(c: &mut Criterion, name: &str, mut initial: TextArea<'static>, moves: [CursorMove; 2]) {
    let start = initial.cursor();
    let mut probe = initial.clone();
    probe.move_cursor(moves[0]);
    assert_ne!(probe.cursor(), start, "{name}: first move must do work");
    probe.move_cursor(moves[1]);
    assert_eq!(
        probe.cursor(),
        start,
        "{name}: pair must return to its origin"
    );
    c.bench_function(name, |b| {
        b.iter(|| {
            black_box(&mut initial).move_cursor(black_box(moves[0]));
            black_box(initial.cursor());
            black_box(&mut initial).move_cursor(black_box(moves[1]));
            black_box(initial.cursor());
        });
    });
}

fn movement(c: &mut Criterion) {
    for (name, lines) in [
        ("ascii_256", document(256, false)),
        ("unicode_256", document(256, true)),
        (
            "long_line_8192_at_col_4095",
            vec!["word ".repeat(1638) + "xx"],
        ),
    ] {
        let mut initial = TextArea::new(lines);
        if initial.lines().len() == 1 {
            initial.move_cursor(CursorMove::Jump(0, 4095));
        }
        pair(
            c,
            &format!("cursor/forward_back_pair/{name}"),
            initial.clone(),
            [CursorMove::Forward, CursorMove::Back],
        );
        pair(
            c,
            &format!("cursor/word_pair/{name}"),
            initial,
            [CursorMove::WordForward, CursorMove::WordBack],
        );
    }
    for (name, unicode) in [("ascii", false), ("unicode", true)] {
        for (mode_name, mode) in [
            ("none", WrapMode::None),
            ("word", WrapMode::Word),
            ("glyph", WrapMode::Glyph),
        ] {
            let mut initial = TextArea::new(document(256, unicode));
            initial.set_wrap_mode(mode);
            Frame::new(40, 24).draw(&initial);
            pair(
                c,
                &format!("cursor/down_up_pair/{name}_{mode_name}_40x24"),
                initial,
                [CursorMove::Down, CursorMove::Up],
            );
        }
    }
    for rows in [256, 2048] {
        pair(
            c,
            &format!("cursor/paragraph_pair/{rows}_lines"),
            TextArea::new(document(rows, false)),
            [CursorMove::ParagraphForward, CursorMove::ParagraphBack],
        );
    }
}

fn atoms(c: &mut Criterion) {
    let mut group = c.benchmark_group("cursor_atomic");
    group.throughput(Throughput::Elements(2));
    for count in [1, 256, 2048] {
        let mut initial = TextArea::from(vec!["left [[token]] right"; count]);
        initial.set_atomic_ranges((0..count).map(|row| AtomicRange {
            row,
            start_col: 5,
            end_col: 14,
        }));
        initial.move_cursor(CursorMove::Jump((count / 2) as u16, 5));
        let mut probe = initial.clone();
        probe.move_cursor(CursorMove::Forward);
        assert_eq!(probe.cursor(), (count / 2, 14));
        probe.move_cursor(CursorMove::Back);
        assert_eq!(probe.cursor(), (count / 2, 5));
        group.bench_with_input(
            BenchmarkId::new("skip_pair", count),
            &initial,
            |b, initial| {
                let mut textarea = initial.clone();
                b.iter(|| {
                    black_box(&mut textarea).move_cursor(CursorMove::Forward);
                    black_box(textarea.cursor());
                    black_box(&mut textarea).move_cursor(CursorMove::Back);
                    black_box(textarea.cursor());
                });
            },
        );
    }
    group.finish();
}

fn directional(c: &mut Criterion) {
    let mut group = c.benchmark_group("cursor_step");
    for (name, unicode) in [("ascii_256", false), ("unicode_256", true)] {
        let mut initial = TextArea::new(document(256, unicode));
        initial.set_wrap_mode(WrapMode::Word);
        initial.move_cursor(CursorMove::Jump(128, 12));
        Frame::new(40, 24).draw(&initial);
        for (direction, movement) in [
            ("forward", CursorMove::Forward),
            ("back", CursorMove::Back),
            ("down", CursorMove::Down),
            ("up", CursorMove::Up),
            ("word_forward", CursorMove::WordForward),
            ("word_back", CursorMove::WordBack),
            ("paragraph_forward", CursorMove::ParagraphForward),
            ("paragraph_back", CursorMove::ParagraphBack),
        ] {
            let mut probe = initial.clone();
            probe.move_cursor(movement);
            assert_ne!(probe.cursor(), initial.cursor(), "{direction}/{name}");
            assert!(probe.cursor().0 < probe.lines().len());
            assert!(probe.cursor().1 <= probe.lines()[probe.cursor().0].chars().count());
            group.bench_function(BenchmarkId::new(direction, name), |b| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        textarea.move_cursor(black_box(movement));
                        black_box(textarea.cursor());
                    },
                    criterion::BatchSize::LargeInput,
                );
            });
        }
    }
    group.finish();
    for (name, unicode) in [("ascii_256", false), ("unicode_256", true)] {
        pair(
            c,
            &format!("cursor/head_end_pair/{name}"),
            TextArea::new(document(256, unicode)),
            [CursorMove::End, CursorMove::Head],
        );
        pair(
            c,
            &format!("cursor/bottom_top_pair/{name}"),
            TextArea::new(document(256, unicode)),
            [CursorMove::Bottom, CursorMove::Top],
        );
    }
}
criterion_group!(benches, movement, atoms, directional);
criterion_main!(benches);
