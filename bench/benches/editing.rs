use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use tui_textarea::{
    AtomicDeleteDirection, AtomicRange, CursorMove, Input, Key, TextArea, WrapMode,
};
use tui_textarea_bench::{document, fixtures, Frame};

#[derive(Clone, Copy)]
enum Edit {
    Insert,
    Backspace,
    Word,
    Line,
    Paste,
}

impl Edit {
    fn name(self) -> &'static str {
        match self {
            Self::Insert => "insert_char_middle",
            Self::Backspace => "backspace_middle",
            Self::Word => "delete_word_middle",
            Self::Line => "delete_line_prefix",
            Self::Paste => "paste_multiline",
        }
    }

    fn apply(self, textarea: &mut TextArea<'_>, payload: &str) {
        match self {
            Self::Insert => textarea.insert_char('x'),
            Self::Backspace => {
                black_box(textarea.delete_char());
            }
            Self::Word => {
                black_box(textarea.delete_word());
            }
            Self::Line => {
                black_box(textarea.delete_line_by_head());
            }
            Self::Paste => {
                black_box(textarea.insert_str(payload));
            }
        }
    }
}

fn operations(c: &mut Criterion) {
    let payload = document(32, true).join("\n");
    let mut group = c.benchmark_group("editing_first_edit");
    for fixture in fixtures() {
        let initial = fixture.at_middle();
        Frame::new(80, 24).draw(&initial);
        let before_chars: usize = initial.lines().iter().map(|s| s.chars().count()).sum();
        for edit in [
            Edit::Insert,
            Edit::Backspace,
            Edit::Word,
            Edit::Line,
            Edit::Paste,
        ] {
            let mut probe = initial.clone();
            edit.apply(&mut probe, &payload);
            let after_chars: usize = probe.lines().iter().map(|s| s.chars().count()).sum();
            assert_ne!(
                probe.lines(),
                initial.lines(),
                "{}: {}",
                fixture.name,
                edit.name()
            );
            match edit {
                Edit::Insert => assert_eq!(after_chars, before_chars + 1),
                Edit::Backspace => assert_eq!(after_chars + 1, before_chars),
                Edit::Word | Edit::Line => assert!(after_chars < before_chars),
                Edit::Paste => assert_eq!(probe.lines().len(), initial.lines().len() + 31),
            }
            assert!(probe.undo());
            assert_eq!(probe.lines(), initial.lines());
            group.throughput(if matches!(edit, Edit::Paste) {
                Throughput::Bytes(payload.len() as u64)
            } else {
                Throughput::Elements(1)
            });
            group.bench_with_input(
                BenchmarkId::new(edit.name(), fixture.name),
                &initial,
                |b, initial| {
                    b.iter_batched_ref(
                        || initial.clone(),
                        |textarea| {
                            edit.apply(textarea, black_box(&payload));
                            black_box(textarea.cursor());
                            black_box(textarea.lines());
                        },
                        BatchSize::LargeInput,
                    );
                },
            );
        }
    }
    group.finish();
}

fn scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("editing_scenario");
    for (name, unicode) in [("ascii", false), ("unicode", true)] {
        let initial = TextArea::new(document(256, unicode));
        // Fixed, dispersed positions: no RNG generation is charged to editing.
        let rows: Vec<_> = (0..32).map(|i| ((i * 73 + 19) % 256) as u16).collect();
        let apply = |textarea: &mut TextArea<'_>| {
            for &row in &rows {
                textarea.move_cursor(CursorMove::Jump(row, 0));
                textarea.insert_char('x');
            }
        };
        let mut probe = initial.clone();
        apply(&mut probe);
        for &row in &rows {
            assert_eq!(
                probe.lines()[row as usize],
                format!("x{}", initial.lines()[row as usize])
            );
        }
        group.throughput(Throughput::Elements(32));
        group.bench_function(format!("32_scattered_edits/{name}"), |b| {
            b.iter_batched_ref(
                || initial.clone(),
                |textarea| {
                    apply(textarea);
                    black_box(textarea.lines());
                },
                BatchSize::LargeInput,
            );
        });

        let mut wrapped = initial;
        wrapped.set_wrap_mode(WrapMode::Word);
        let mut frame = Frame::new(80, 24);
        frame.draw(&wrapped);
        let input = Input {
            key: Key::Char('x'),
            ..Input::default()
        };
        let mut probe = wrapped.clone();
        assert!(probe.input(input.clone()));
        frame.draw(&probe);
        assert_eq!(probe.lines()[0], format!("x{}", wrapped.lines()[0]));
        assert!(probe.rendered_cursor_position().is_some());
        group.throughput(Throughput::Elements(1));
        group.bench_function(format!("first_input_and_frame_80x24/{name}"), |b| {
            b.iter_batched_ref(
                || (wrapped.clone(), Frame::new(80, 24)),
                |(textarea, frame)| {
                    black_box(textarea.input(input.clone()));
                    frame.draw(textarea);
                },
                BatchSize::LargeInput,
            );
        });
    }

    for count in [1, 256] {
        let mut initial = TextArea::from(vec!["left [[token]] right"; count]);
        initial.set_atomic_ranges((0..count).map(|row| AtomicRange {
            row,
            start_col: 5,
            end_col: 14,
        }));
        initial.move_cursor(CursorMove::Jump((count / 2) as u16, 5));
        let mut probe = initial.clone();
        assert!(probe
            .delete_atomic_range_at_cursor(AtomicDeleteDirection::Forward)
            .is_some());
        assert_eq!(probe.lines()[count / 2], "left  right");
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("delete_atomic_token", count),
            &initial,
            |b, initial| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        black_box(
                            textarea.delete_atomic_range_at_cursor(AtomicDeleteDirection::Forward),
                        );
                        black_box(textarea.lines());
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

fn sustained(c: &mut Criterion) {
    let mut group = c.benchmark_group("editing_steady");
    group.throughput(Throughput::Elements(2));
    for (name, unicode) in [("ascii", false), ("unicode", true)] {
        for paint in [false, true] {
            let mut textarea = TextArea::new(document(256, unicode));
            textarea.set_wrap_mode(WrapMode::Word);
            textarea.move_cursor(CursorMove::Jump(128, 12));
            let original = textarea.lines().to_vec();
            let cursor = textarea.cursor();
            let mut frame = Frame::new(80, 24);
            frame.draw(&textarea);
            // Fill the bounded history and retain string capacity before measuring steady editing.
            for _ in 0..50 {
                textarea.insert_char('x');
                assert!(textarea.delete_char());
            }
            assert_eq!(textarea.lines(), original);
            assert_eq!(textarea.cursor(), cursor);
            let case = if paint {
                "insert_backspace_frame_pair"
            } else {
                "insert_backspace_pair"
            };
            group.bench_function(format!("{case}/{name}"), |b| {
                b.iter(|| {
                    textarea.insert_char(black_box('x'));
                    if paint {
                        frame.draw(&textarea);
                    }
                    black_box(textarea.delete_char());
                    if paint {
                        frame.draw(&textarea);
                    }
                    black_box(textarea.lines());
                });
            });
            assert_eq!(textarea.lines(), original);
            assert_eq!(textarea.cursor(), cursor);
        }
    }
    group.finish();
}

fn selection_and_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("selection");
    for fixture in fixtures().into_iter().take(3) {
        let initial = fixture.textarea();
        Frame::new(80, 24).draw(&initial);
        let mut selected = initial.clone();
        selected.select_all();
        assert!(selected.is_selecting());
        let expected_yank = fixture.lines.join("\n");
        let mut probe = selected.clone();
        probe.copy();
        assert_eq!(probe.yank_text(), expected_yank);
        assert_eq!(probe.lines(), initial.lines());
        group.bench_function(BenchmarkId::new("copy_all", fixture.name), |b| {
            b.iter_batched_ref(
                || selected.clone(),
                |textarea| {
                    textarea.copy();
                    black_box(&*textarea);
                },
                BatchSize::LargeInput,
            );
        });

        let mut probe = selected.clone();
        assert!(probe.cut());
        assert_eq!(probe.lines(), [""]);
        assert_eq!(probe.yank_text(), expected_yank);
        assert!(probe.undo());
        assert_eq!(probe.lines(), initial.lines());
        group.bench_function(BenchmarkId::new("cut_all", fixture.name), |b| {
            b.iter_batched_ref(
                || selected.clone(),
                |textarea| {
                    black_box(textarea.cut());
                    black_box(&*textarea);
                },
                BatchSize::LargeInput,
            );
        });

        let replacement = ["replacement 🦀".to_owned(), "line two".to_owned()];
        let mut clipboard = selected.clone();
        clipboard.set_yank_text(replacement.join("\n"));
        let mut probe = clipboard.clone();
        assert!(probe.paste());
        assert_eq!(probe.lines(), replacement);
        assert!(probe.undo());
        // Selection replacement records deletion and insertion separately.
        assert_eq!(probe.lines(), [""]);
        assert!(probe.undo());
        assert_eq!(probe.lines(), initial.lines());
        assert!(!probe.undo());
        group.bench_function(
            BenchmarkId::new("paste_over_selection", fixture.name),
            |b| {
                b.iter_batched_ref(
                    || clipboard.clone(),
                    |textarea| {
                        black_box(textarea.paste());
                        black_box(&*textarea);
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        let mut probe = initial.clone();
        probe.set_lines(replacement.to_vec(), (1, 4));
        assert_eq!(probe.lines(), replacement);
        assert_eq!(probe.cursor(), (1, 4));
        assert!(!probe.undo());
        group.bench_function(BenchmarkId::new("replace_buffer", fixture.name), |b| {
            b.iter_batched_ref(
                || (initial.clone(), replacement.to_vec()),
                |(textarea, lines)| {
                    textarea.set_lines(std::mem::take(lines), black_box((1, 4)));
                    black_box(&*textarea);
                },
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

fn atomic_configuration(c: &mut Criterion) {
    let mut group = c.benchmark_group("editing_atomic");
    for count in [1, 256, 2048] {
        let initial = TextArea::from(vec!["left [[token]] right"; count]);
        let ranges: Vec<_> = (0..count)
            .map(|row| AtomicRange {
                row,
                start_col: 5,
                end_col: 14,
            })
            .collect();
        let mut probe = initial.clone();
        probe.set_atomic_ranges(ranges.clone());
        assert_eq!(probe.atomic_ranges(), ranges);
        group.bench_function(BenchmarkId::new("set_ranges", count), |b| {
            b.iter_batched_ref(
                || (initial.clone(), ranges.clone()),
                |(textarea, ranges)| {
                    textarea.set_atomic_ranges(std::mem::take(ranges));
                    black_box(textarea.atomic_ranges());
                },
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}
fn typing_and_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("editing_typing");
    let typed: String = "type some text ".repeat(5).chars().take(64).collect();
    for fixture in fixtures() {
        let mut initial = fixture.textarea();
        initial.move_cursor(CursorMove::Jump(u16::MAX, u16::MAX));
        Frame::new(80, 24).draw(&initial);
        for (name, payload) in [
            ("64_chars_at_eof", typed.clone()),
            ("4_lines_at_eof", "let x = 1;\ncall(x);\n".repeat(2)),
        ] {
            let inputs: Vec<_> = payload
                .chars()
                .map(|ch| Input {
                    key: if ch == '\n' {
                        Key::Enter
                    } else {
                        Key::Char(ch)
                    },
                    ..Input::default()
                })
                .collect();
            let mut probe = initial.clone();
            for input in &inputs {
                assert!(probe.input(input.clone()));
            }
            assert_eq!(
                probe.lines().join("\n"),
                format!("{}{}", initial.lines().join("\n"), payload)
            );
            group.throughput(Throughput::Elements(inputs.len() as u64));
            group.bench_function(BenchmarkId::new(name, fixture.name), |b| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        for input in &inputs {
                            black_box(textarea.input(black_box(input.clone())));
                        }
                        black_box(textarea.lines());
                    },
                    BatchSize::LargeInput,
                );
            });
        }
        let payload = document(32, true).join("\n");
        let mut probe = initial.clone();
        assert!(probe.insert_str(&payload));
        assert_eq!(
            probe.lines().join("\n"),
            format!("{}{}", initial.lines().join("\n"), payload)
        );
        group.throughput(Throughput::Bytes(payload.len() as u64));
        group.bench_function(
            BenchmarkId::new("bulk_append_32_lines", fixture.name),
            |b| {
                b.iter_batched_ref(
                    || initial.clone(),
                    |textarea| {
                        black_box(textarea.insert_str(black_box(&payload)));
                        black_box(textarea.lines());
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}
criterion_group!(
    benches,
    operations,
    scenarios,
    sustained,
    selection_and_buffer,
    atomic_configuration,
    typing_and_append
);
criterion_main!(benches);
