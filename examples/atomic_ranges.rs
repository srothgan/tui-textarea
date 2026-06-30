use ratatui::style::{Color, Style};
use tui_textarea::{AtomicDeleteDirection, AtomicRange, CursorMove, TextArea};

fn placeholder_ranges(row: usize, line: &str) -> Vec<AtomicRange> {
    let mut ranges = Vec::new();
    let mut offset = 0;

    while let Some(start_delta) = line[offset..].find("[[") {
        let start_byte = offset + start_delta;
        let Some(end_delta) = line[start_byte..].find("]]") else {
            break;
        };
        let end_byte = start_byte + end_delta + 2;
        ranges.push(AtomicRange {
            row,
            start_col: line[..start_byte].chars().count(),
            end_col: line[..end_byte].chars().count(),
        });
        offset = end_byte;
    }

    ranges
}

fn byte_offset(line: &str, col: usize) -> usize {
    line.char_indices()
        .nth(col)
        .map(|(offset, _)| offset)
        .unwrap_or(line.len())
}

fn refresh_atoms(textarea: &mut TextArea<'_>) {
    let ranges: Vec<_> = textarea
        .lines()
        .iter()
        .enumerate()
        .flat_map(|(row, line)| placeholder_ranges(row, line))
        .collect();
    let highlights: Vec<_> = ranges
        .iter()
        .map(|range| {
            let line = &textarea.lines()[range.row];
            (
                (
                    (range.row, byte_offset(line, range.start_col)),
                    (range.row, byte_offset(line, range.end_col)),
                ),
                Style::default().fg(Color::Yellow),
            )
        })
        .collect();

    textarea.set_atomic_ranges(ranges);
    textarea.clear_custom_highlight();
    for (range, style) in highlights {
        textarea.custom_highlight(range, style, 10);
    }
}

fn main() {
    let mut textarea = TextArea::from(["See [[image:cat.png]] before sending."]);
    refresh_atoms(&mut textarea);

    textarea.move_cursor(CursorMove::Jump(0, 4));
    textarea.delete_atomic_range_at_cursor(AtomicDeleteDirection::Forward);
    refresh_atoms(&mut textarea);
}
