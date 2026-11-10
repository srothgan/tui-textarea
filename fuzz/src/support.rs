use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget,
};
use tui_textarea::TextArea;

pub fn bounds(textarea: &TextArea<'_>) {
    let lines = textarea.lines();
    assert!(!lines.is_empty());
    assert!(lines.iter().all(|line| !line.contains('\n')));
    let (row, col) = textarea.cursor();
    assert!(
        row < lines.len(),
        "cursor row {row}, line count {}",
        lines.len()
    );
    assert!(col <= lines[row].chars().count(), "cursor column {col}");
    if let Some((start, end)) = textarea.selection_range() {
        for (row, col) in [start, end] {
            assert!(row < lines.len());
            assert!(col <= lines[row].chars().count());
        }
        assert!(start <= end);
    }
}

pub fn budget(textarea: &TextArea<'_>) -> bool {
    textarea.lines().len() <= crate::MAX_LINES
        && textarea
            .lines()
            .iter()
            .map(|line| line.chars().count())
            .sum::<usize>()
            <= crate::MAX_CHARS
}

pub fn flat(textarea: &TextArea<'_>) -> Vec<char> {
    textarea.lines().join("\n").chars().collect()
}

pub fn offset(lines: &[String], position: (usize, usize)) -> usize {
    lines[..position.0]
        .iter()
        .map(|line| line.chars().count() + 1)
        .sum::<usize>()
        + position.1
}

pub fn selected(textarea: &TextArea<'_>) -> Option<String> {
    let (start, end) = textarea.selection_range()?;
    let chars = flat(textarea);
    Some(
        chars[offset(textarea.lines(), start)..offset(textarea.lines(), end)]
            .iter()
            .collect(),
    )
}

pub fn history_round_trip(textarea: &TextArea<'_>) {
    let expected = textarea.lines().to_vec();
    let mut probe = textarea.clone();
    let mut steps = 0;
    while probe.undo() {
        steps += 1;
        assert!(steps <= crate::MAX_STEPS * 2);
        bounds(&probe);
    }
    for _ in 0..steps {
        assert!(probe.redo());
        bounds(&probe);
    }
    assert_eq!(
        probe.lines(),
        expected,
        "undo/redo must restore text at the original history position"
    );
}

pub fn isolated_edit_round_trip(textarea: &TextArea<'_>) {
    let mut probe = textarea.clone();
    probe.cancel_selection();
    probe.set_undo_coalescing(false);
    probe.set_max_histories(crate::MAX_STEPS * 2);
    let before = probe.lines().to_vec();
    probe.insert_char('x');
    let after = probe.lines().to_vec();
    assert!(probe.undo());
    assert_eq!(probe.lines(), before);
    assert!(probe.redo());
    assert_eq!(probe.lines(), after);
}

pub struct View {
    pub area: Rect,
    pub buffer: Buffer,
}

impl Default for View {
    fn default() -> Self {
        Self::new(Rect::new(0, 0, 40, 6))
    }
}

impl View {
    pub fn new(area: Rect) -> Self {
        Self {
            area,
            buffer: Buffer::empty(area),
        }
    }

    pub fn resize(&mut self, data: &mut crate::decode::Decoder<'_>) {
        let widths = [0, 1, 2, 4, 8, 16, 40, 64];
        let heights = [0, 1, 2, 4, 8, 12];
        self.area = Rect::new(
            u16::from(data.byte() % 4),
            u16::from(data.byte() % 4),
            widths[usize::from(data.byte()) % widths.len()],
            heights[usize::from(data.byte()) % heights.len()],
        );
        self.buffer.resize(self.area);
    }

    pub fn draw(&mut self, textarea: &TextArea<'_>) {
        self.buffer.reset();
        textarea.render(self.area, &mut self.buffer);
        bounds(textarea);
        if let Some(position) = textarea.rendered_cursor_position() {
            assert!(self.area.contains(position));
        }
    }

    pub fn hit(&self, textarea: &TextArea<'_>, position: Position) {
        let inner = textarea
            .block()
            .map_or(self.area, |block| block.inner(self.area));
        let result = textarea.cursor_at_position(position);
        assert_eq!(
            result.is_some(),
            inner.contains(position) && !inner.is_empty()
        );
        if let Some((row, col)) = result {
            assert!(row < textarea.lines().len());
            assert!(col <= textarea.lines()[row].chars().count());
        }
    }
}
