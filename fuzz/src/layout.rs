use crate::{
    decode::{lines, Decoder},
    support::{self, View},
    MAX_STEPS,
};
use ratatui_core::{
    layout::{Alignment, Position, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
};
use ratatui_widgets::{
    block::{Block, Padding},
    borders::Borders,
};
use tui_textarea::{CursorRenderMode, TextArea, WrapMode};

fn measurement(textarea: &mut TextArea<'_>, width: u16) {
    let expected = textarea.measure(width);
    assert_eq!(textarea.measure(width), expected, "repeated measurement");
    assert!(expected.min_rows >= 1);
    assert!(expected.max_rows >= expected.min_rows);
    assert!((expected.min_rows..=expected.max_rows).contains(&expected.preferred_rows));
    assert!(expected.content_rows >= 1);
    let mut fresh = textarea.clone();
    fresh.set_lines(textarea.lines().to_vec(), textarea.cursor());
    assert_eq!(
        fresh.measure(width),
        expected,
        "cached and invalidated measurement differ"
    );

    if textarea.is_empty() && !textarea.placeholder().lines.is_empty() {
        return;
    }
    if textarea.wrap_mode() == WrapMode::None {
        assert_eq!(expected.content_rows as usize, textarea.lines().len());
    } else if textarea.wrap_mode() == WrapMode::Glyph
        && textarea
            .lines()
            .iter()
            .all(|line| line.chars().all(|ch| ch.is_ascii_graphic() || ch == ' '))
    {
        let area = Rect::new(0, 0, width, u16::MAX);
        let inner = textarea.block().map_or(area, |block| block.inner(area));
        let gutter = if textarea.line_number_style().is_some() {
            textarea.lines().len().to_string().len() + 2
        } else {
            0
        };
        let usable = usize::from(inner.width)
            .saturating_sub(gutter)
            .saturating_sub(1)
            .max(1);
        let rows: usize = textarea
            .lines()
            .iter()
            .map(|line| line.len().div_ceil(usable).max(1))
            .sum();
        assert_eq!(expected.content_rows as usize, rows, "ASCII glyph rows");
    }
}

fn word_alias(textarea: &TextArea<'_>, area: Rect) {
    let mut word = textarea.clone();
    let mut alias = textarea.clone();
    word.set_wrap_mode(WrapMode::Word);
    alias.set_wrap_mode(WrapMode::WordOrGlyph);
    assert_eq!(word.measure(area.width), alias.measure(area.width));
    let mut a = View::new(area);
    let mut b = View::new(area);
    a.draw(&word);
    b.draw(&alias);
    assert_eq!(
        a.buffer, b.buffer,
        "word wrap compatibility variant differs"
    );
    assert_eq!(
        word.rendered_cursor_position(),
        alias.rendered_cursor_position()
    );
}

pub fn run(bytes: &[u8]) {
    let mut data = Decoder::new(bytes);
    let header = data.byte();
    let mut textarea = TextArea::new(data.initial(header));
    textarea.set_max_histories(32);
    let mut view = View::default();
    view.draw(&textarea);
    for step in 0..MAX_STEPS {
        let Some(tag) = data.next() else { break };
        match tag % 18 {
            0 => {
                textarea.insert_str(data.text());
            }
            1 => {
                textarea.delete_str(data.count());
            }
            2 => view.resize(&mut data),
            3 => textarea.set_wrap_mode(data.wrap()),
            4 => textarea.set_tab_length(data.byte() % 17),
            5 => {
                if data.byte() & 1 == 0 {
                    textarea.remove_line_number();
                } else {
                    textarea.set_line_number_style(Style::default().fg(Color::Blue));
                }
            }
            6 => match data.byte() % 3 {
                0 => {
                    textarea.remove_block();
                }
                1 => textarea.set_block(Block::default().borders(Borders::ALL)),
                _ => textarea.set_block(Block::default().padding(Padding::uniform(1))),
            },
            7 => textarea.set_alignment(match data.byte() % 3 {
                0 => Alignment::Left,
                1 => Alignment::Center,
                _ => Alignment::Right,
            }),
            8 => textarea.set_placeholder_text(data.text()),
            9 => {
                let text = data.text();
                let placeholder = Text::from(
                    lines(&text)
                        .into_iter()
                        .map(|line| {
                            Line::from(vec![Span::styled(line, Style::default().fg(Color::Blue))])
                        })
                        .collect::<Vec<_>>(),
                );
                textarea.set_styled_placeholder(placeholder);
            }
            10 => match data.byte() % 3 {
                0 => textarea.clear_mask_char(),
                1 => textarea.set_mask_char('*'),
                _ => textarea.set_mask_char('•'),
            },
            11 => {
                textarea.set_min_rows(u16::from(data.byte() % 33));
                textarea.set_max_rows(u16::from(data.byte() % 33));
            }
            12 => textarea.move_cursor(data.movement()),
            13 => textarea.scroll((data.word() as i16, data.word() as i16)),
            14 => textarea.set_lines(
                lines(&data.text()),
                (data.word() as usize, data.word() as usize),
            ),
            15 => {
                // Hit queries are checked below, after rendering the current geometry.
            }
            16 => {
                textarea.input(data.input());
            }
            _ => textarea.set_cursor_render_mode(if data.byte() & 1 == 0 {
                CursorRenderMode::Cell
            } else {
                CursorRenderMode::Hidden
            }),
        }
        support::bounds(&textarea);
        if !support::budget(&textarea) {
            break;
        }
        measurement(&mut textarea, view.area.width);
        view.draw(&textarea);
        let first = view.buffer.clone();
        let cursor = textarea.rendered_cursor_position();
        view.draw(&textarea);
        assert_eq!(
            first, view.buffer,
            "repeated rendering changed cells at step {step}"
        );
        assert_eq!(cursor, textarea.rendered_cursor_position());
        view.hit(
            &textarea,
            Position::new(u16::from(data.byte()), u16::from(data.byte())),
        );
        if let Some(position) = cursor {
            view.hit(&textarea, position);
            let (row, col) = textarea.cursor();
            let line = &textarea.lines()[row];
            if textarea.wrap_mode() == WrapMode::None
                && col < line.len()
                && line.chars().all(|ch| ch.is_ascii_graphic() || ch == ' ')
            {
                assert_eq!(
                    textarea.cursor_at_position(position),
                    Some((row, col)),
                    "unambiguous ASCII cursor/hit-test round trip"
                );
            }
        }
        if step % 8 == 0 {
            word_alias(&textarea, view.area);
        }
    }
}
