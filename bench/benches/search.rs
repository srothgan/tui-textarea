use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;
use tui_textarea::{CursorMove, TextArea, WrapMode};
use tui_textarea_bench::{document, Frame};

fn scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");
    for (name, rows, unicode) in [
        ("ascii_256", 256, false),
        ("unicode_256", 256, true),
        ("document_2048", 2048, false),
    ] {
        let lines = document(rows, unicode);
        for (case, pattern, backward, expected_row) in [
            ("forward_near_match", r"target_\d+", false, Some(1)),
            ("backward_near_match", r"target_\d+", true, Some(rows - 2)),
            ("forward_far_match", "end_marker", false, Some(rows - 2)),
            ("backward_far_match", "begin_marker", true, Some(0)),
            ("forward_miss", "NEVER_PRESENT_IN_DOCUMENT", false, None),
            ("backward_miss", "NEVER_PRESENT_IN_DOCUMENT", true, None),
        ] {
            let mut initial = TextArea::new(lines.clone());
            initial.set_search_pattern(pattern).unwrap();
            if backward {
                initial.move_cursor(CursorMove::Jump(u16::MAX, u16::MAX));
            }
            let operation = |textarea: &mut TextArea<'_>| {
                if backward {
                    textarea.search_back(false)
                } else {
                    textarea.search_forward(false)
                }
            };
            let mut probe = initial.clone();
            assert_eq!(operation(&mut probe), expected_row.is_some());
            if let Some(row) = expected_row {
                let byte = probe
                    .search_pattern()
                    .unwrap()
                    .find(&lines[row])
                    .unwrap()
                    .start();
                assert_eq!(probe.cursor(), (row, lines[row][..byte].chars().count()));
            } else {
                assert_eq!(probe.cursor(), initial.cursor());
            }
            group.bench_function(BenchmarkId::new(case, name), |b| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        black_box(operation(textarea));
                        black_box(textarea.cursor());
                    },
                    BatchSize::LargeInput,
                );
            });
        }
    }
    group.finish();
}

fn compilation_and_frame(c: &mut Criterion) {
    let initial = TextArea::default();
    let pattern = r"(?u)\b[\p{L}_][\p{L}\p{N}_]*\b";
    let mut probe = initial.clone();
    probe.set_search_pattern(pattern).unwrap();
    assert!(probe.search_pattern().unwrap().is_match("東京"));
    c.bench_function("search_compile/unicode_identifier_regex", |b| {
        b.iter_batched_ref(
            || initial.clone(),
            |textarea| {
                textarea.set_search_pattern(black_box(pattern)).unwrap();
                black_box(textarea.search_pattern());
            },
            BatchSize::LargeInput,
        );
    });

    for (name, unicode) in [("ascii", false), ("unicode", true)] {
        let mut initial = TextArea::new(document(256, unicode));
        initial.set_search_pattern("end_marker").unwrap();
        initial.set_wrap_mode(WrapMode::Word);
        Frame::new(80, 24).draw(&initial);
        let mut probe = initial.clone();
        assert!(probe.search_forward(false));
        Frame::new(80, 24).draw(&probe);
        assert_eq!(probe.cursor().0, 254);
        assert!(probe.rendered_cursor_position().is_some());
        c.bench_function(
            &format!("search_scenario/find_and_frame_80x24/{name}"),
            |b| {
                b.iter_batched_ref(
                    || (initial.clone(), Frame::new(80, 24)),
                    |(textarea, frame)| {
                        black_box(textarea.search_forward(false));
                        frame.draw(textarea);
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
}

criterion_group!(benches, scan, compilation_and_frame);
criterion_main!(benches);
