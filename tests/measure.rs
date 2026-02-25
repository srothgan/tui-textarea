use ratatui::style::Style;
use ratatui::widgets::{Block, Borders};
use tui_textarea::{TextArea, TextAreaMeasure, WrapMode};

#[test]
fn measure_without_wrap_uses_logical_line_count() {
    let mut textarea = TextArea::from(["hello", "world"]);
    let have = textarea.measure(20);
    let want = TextAreaMeasure {
        content_rows: 2,
        preferred_rows: 2,
        min_rows: 1,
        max_rows: u16::MAX,
    };
    assert_eq!(have, want);
}

#[test]
fn measure_with_wrap_counts_visual_rows() {
    let mut textarea = TextArea::from(["abcdef"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);

    let have = textarea.measure(5);
    assert_eq!(have.content_rows, 2);
    assert_eq!(have.preferred_rows, 2);
    assert_eq!(have.min_rows, 1);
}

#[test]
fn measure_accounts_for_line_number_width_when_wrapping() {
    let mut textarea = TextArea::from(["abcdefg"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);
    assert_eq!(textarea.measure(8).content_rows, 1);

    textarea.set_line_number_style(Style::default());
    assert_eq!(textarea.measure(8).content_rows, 2);
}

#[test]
fn measure_adds_block_chrome_rows_to_preferred_and_min() {
    let mut textarea = TextArea::from(["line"]);
    textarea.set_block(Block::default().borders(Borders::ALL));

    let have = textarea.measure(12);
    assert_eq!(have.content_rows, 1);
    assert_eq!(have.preferred_rows, 3);
    assert_eq!(have.min_rows, 3);
    assert_eq!(have.max_rows, u16::MAX);
}

#[test]
fn measure_wraps_even_at_zero_width() {
    let mut textarea = TextArea::from(["ab"]);
    textarea.set_wrap_mode(WrapMode::WordOrGlyph);

    let have = textarea.measure(0);
    assert_eq!(have.content_rows, 2);
    assert_eq!(have.preferred_rows, 2);
}

#[test]
fn measure_respects_configured_min_rows() {
    let mut textarea = TextArea::from(["line"]);
    textarea.set_min_rows(4);

    let have = textarea.measure(12);
    assert_eq!(have.content_rows, 1);
    assert_eq!(have.preferred_rows, 4);
    assert_eq!(have.min_rows, 4);
    assert_eq!(have.max_rows, u16::MAX);
}

#[test]
fn measure_respects_configured_max_rows() {
    let mut textarea: TextArea<'_> = (0..10).map(|n| n.to_string()).collect();
    textarea.set_max_rows(3);

    let have = textarea.measure(12);
    assert_eq!(have.content_rows, 10);
    assert_eq!(have.preferred_rows, 3);
    assert_eq!(have.min_rows, 1);
    assert_eq!(have.max_rows, 3);
}

#[test]
fn min_max_rows_setters_normalize_and_keep_order() {
    let mut textarea = TextArea::default();
    textarea.set_max_rows(0);
    assert_eq!(textarea.max_rows(), 1);

    textarea.set_min_rows(5);
    assert_eq!(textarea.min_rows(), 5);
    assert_eq!(textarea.max_rows(), 5);
}

#[test]
fn block_intrinsic_min_rows_overrides_configured_bounds() {
    let mut textarea = TextArea::from(["line"]);
    textarea.set_block(Block::default().borders(Borders::ALL));
    textarea.set_min_rows(1);
    textarea.set_max_rows(2);

    let have = textarea.measure(12);
    assert_eq!(have.preferred_rows, 3);
    assert_eq!(have.min_rows, 3);
    assert_eq!(have.max_rows, 3);
}
