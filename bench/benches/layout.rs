use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;
use tui_textarea::{CursorMove, WrapMode};
use tui_textarea_bench::{fixtures, Frame};

fn render(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_frame");
    for fixture in fixtures() {
        for (mode_name, mode) in [
            ("none", WrapMode::None),
            ("word", WrapMode::Word),
            ("glyph", WrapMode::Glyph),
        ] {
            for width in [40, 120] {
                let mut textarea = fixture.textarea();
                textarea.set_wrap_mode(mode);
                let mut frame = Frame::new(width, 24);
                frame.draw(&textarea);
                assert!(frame.buffer.content.iter().any(|cell| cell.symbol() != " "));
                assert!(textarea.rendered_cursor_position().is_some());
                let id = format!("{}_{}_{}x24", fixture.name, mode_name, width);
                group.bench_function(BenchmarkId::new("warm", &id), |b| {
                    b.iter(|| frame.draw(black_box(&textarea)));
                });
            }
        }

        let mut initial = fixture.textarea();
        initial.set_wrap_mode(WrapMode::Word);
        assert!(initial.rendered_cursor_position().is_none());
        let mut probe_frame = Frame::new(80, 24);
        let probe = initial.clone();
        probe_frame.draw(&probe);
        assert!(probe.rendered_cursor_position().is_some());
        group.bench_function(
            BenchmarkId::new("first_word_frame_80x24", fixture.name),
            |b| {
                b.iter_batched_ref(
                    || (initial.clone(), Frame::new(80, 24)),
                    |(textarea, frame)| frame.draw(textarea),
                    BatchSize::LargeInput,
                );
            },
        );

        let mut frame = Frame::new(120, 24);
        frame.draw(&initial);
        frame.resize(40, 24);
        frame.draw(&initial);
        assert_eq!(frame.buffer.area.width, 40);
        assert!(initial.rendered_cursor_position().is_some());
        frame.resize(120, 24);
        frame.draw(&initial);
        group.bench_function(
            BenchmarkId::new("resize_word_120_40_120", fixture.name),
            |b| {
                b.iter(|| {
                    frame.resize(40, 24);
                    frame.draw(&initial);
                    frame.resize(120, 24);
                    frame.draw(&initial);
                });
            },
        );
    }
    group.finish();
}

fn measurement(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_measure");
    for fixture in fixtures() {
        let mut initial = fixture.textarea();
        initial.set_wrap_mode(WrapMode::Word);
        let expected = initial.measure(80);
        assert!(expected.content_rows >= initial.lines().len() as u16);
        assert_eq!(initial.measure(80), expected);
        let mut cached = initial.clone();
        group.bench_function(BenchmarkId::new("cached_80", fixture.name), |b| {
            b.iter(|| black_box(black_box(&mut cached).measure(black_box(80))));
        });

        let mut wide = initial.clone();
        wide.measure(120);
        let expected_narrow = wide.clone().measure(40);
        assert!(expected_narrow.content_rows >= expected.content_rows);
        group.bench_function(
            BenchmarkId::new("width_change_120_to_40", fixture.name),
            |b| {
                b.iter_batched_ref(
                    || wide.clone(),
                    |textarea| {
                        black_box(textarea.measure(black_box(40)));
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        let setup = || {
            let mut textarea = initial.clone();
            textarea.insert_char('x');
            textarea
        };
        let mut probe = setup();
        assert_eq!(probe.lines()[0], format!("x{}", initial.lines()[0]));
        assert!(probe.measure(80).content_rows >= expected.content_rows);
        group.bench_function(BenchmarkId::new("after_edit_80", fixture.name), |b| {
            b.iter_batched_ref(
                setup,
                |textarea| {
                    black_box(textarea.measure(black_box(80)));
                },
                BatchSize::LargeInput,
            );
        });

        let mut target = initial.clone();
        let row = if target.lines().len() == 1 { 0 } else { 4 };
        target.move_cursor(CursorMove::Jump(row, 40));
        Frame::new(80, 24).draw(&target);
        let position = target.rendered_cursor_position().unwrap();
        assert_eq!(target.cursor_at_position(position), Some(target.cursor()));
        group.bench_function(
            BenchmarkId::new("hit_test_rendered_cursor", fixture.name),
            |b| {
                b.iter(|| black_box(target.cursor_at_position(black_box(position))));
            },
        );
    }
    group.finish();
}

fn styled(c: &mut Criterion) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span, Text};
    use ratatui::widgets::{Block, Borders};
    use tui_textarea::{CursorRenderMode, TextArea};

    let mut group = c.benchmark_group("layout_features");
    let mut cases: Vec<(&str, TextArea<'static>)> = Vec::new();
    let base = TextArea::new(tui_textarea_bench::document(256, true));
    let mut chrome = base.clone();
    chrome.set_block(Block::default().borders(Borders::ALL).title("Editor"));
    chrome.set_line_number_style(Style::default().fg(Color::DarkGray));
    cases.push(("border_and_line_numbers", chrome));

    let mut selection = base.clone();
    selection.start_selection();
    selection.move_cursor(CursorMove::Jump(12, 20));
    assert!(selection.selection_range().is_some());
    cases.push(("selection", selection));

    for (name, count) in [
        ("custom_highlights_64", 64),
        ("custom_highlights_2048", 2048),
    ] {
        let mut textarea = base.clone();
        let rows: Vec<_> = textarea
            .lines()
            .iter()
            .enumerate()
            .filter_map(|(row, line)| (!line.is_empty()).then_some(row))
            .collect();
        for i in 0..count {
            let row = rows[(i / 8) % rows.len()];
            let start = i % 8;
            textarea.custom_highlight(
                ((row, start), (row, start + 3)),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
                (i % 3) as u8,
            );
        }
        cases.push((name, textarea));
    }

    let mut search = base.clone();
    search
        .set_search_pattern(r"target_\d+|boundaries|東京")
        .unwrap();
    cases.push(("search_highlighting", search));

    let mut masked = TextArea::from(["correct horse battery staple"]);
    masked.set_mask_char('•');
    cases.push(("password_mask", masked));

    let mut placeholder = TextArea::default();
    placeholder.set_styled_placeholder(Text::from(vec![
        Line::from(vec![
            Span::styled("Message", Style::default().fg(Color::Blue)),
            Span::raw(" goes here"),
        ]),
        Line::from("Shift+Enter adds a line"),
    ]));
    cases.push(("styled_placeholder", placeholder));

    let mut native = base;
    native.set_cursor_render_mode(CursorRenderMode::Hidden);
    cases.push(("native_cursor", native));

    for (name, mut textarea) in cases {
        textarea.set_wrap_mode(WrapMode::Word);
        let mut frame = Frame::new(80, 24);
        frame.draw(&textarea);
        assert!(frame.buffer.content.iter().any(|cell| cell.symbol() != " "));
        assert!(textarea.rendered_cursor_position().is_some());
        group.bench_function(name, |b| b.iter(|| frame.draw(black_box(&textarea))));
    }
    group.finish();
}

fn viewport(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_viewport");
    for (name, rows, unicode) in [("unicode_256", 256, true), ("document_2048", 2048, false)] {
        let mut textarea = tui_textarea::TextArea::new(tui_textarea_bench::document(rows, unicode));
        textarea.set_wrap_mode(WrapMode::Word);
        let mut frame = Frame::new(80, 24);
        frame.draw(&textarea);
        textarea.move_cursor(CursorMove::Bottom);
        frame.draw(&textarea);
        assert_eq!(textarea.cursor().0, rows - 1);
        assert!(textarea.rendered_cursor_position().is_some());
        textarea.move_cursor(CursorMove::Top);
        frame.draw(&textarea);
        assert_eq!(textarea.cursor(), (0, 0));
        group.bench_function(BenchmarkId::new("jump_bottom_top_frames", name), |b| {
            b.iter(|| {
                textarea.move_cursor(CursorMove::Bottom);
                frame.draw(&textarea);
                textarea.move_cursor(CursorMove::Top);
                frame.draw(&textarea);
            });
        });
        assert_eq!(textarea.cursor(), (0, 0));
    }
    group.finish();
}
criterion_group!(benches, render, measurement, styled, viewport);
criterion_main!(benches);
