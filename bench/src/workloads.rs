use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use std::hint::black_box;
use tui_textarea::{CursorMove, TextArea};

pub struct Fixture {
    pub name: &'static str,
    pub lines: Vec<String>,
}

impl Fixture {
    pub fn textarea(&self) -> TextArea<'static> {
        TextArea::new(self.lines.clone())
    }

    pub fn at_middle(&self) -> TextArea<'static> {
        let mut textarea = self.textarea();
        let row = self.lines.len() / 2;
        let col = self.lines[row].chars().count() / 2;
        textarea.move_cursor(CursorMove::Jump(row as u16, col as u16));
        textarea
    }
}

/// Paragraphs, identifiers, and unique markers give navigation and search known targets.
pub fn document(rows: usize, unicode: bool) -> Vec<String> {
    assert!((8..=u16::MAX as usize).contains(&rows));
    (0..rows)
        .map(|row| {
            if row % 8 == 7 {
                return String::new();
            }
            let body = if unicode {
                "Rust 🦀 cafe\u{301} 東京\tword boundaries and terminal editing"
            } else {
                "let value = compute_result(); // edit this terminal document"
            };
            let marker = if row == 0 {
                " begin_marker"
            } else if row == rows - 2 {
                " end_marker"
            } else {
                ""
            };
            format!("target_{row:04} {body}{marker}")
        })
        .collect()
}

pub fn fixtures() -> Vec<Fixture> {
    vec![
        Fixture {
            name: "input_8_lines",
            lines: document(8, false),
        },
        Fixture {
            name: "unicode_256_lines",
            lines: document(256, true),
        },
        Fixture {
            name: "document_2048_lines",
            lines: document(2048, false),
        },
        Fixture {
            name: "long_token_8192_chars",
            lines: vec!["abcdefgh".repeat(1024)],
        },
    ]
}

/// A frame includes clearing and painting cells, without terminal diffing or terminal I/O.
pub struct Frame {
    pub area: Rect,
    pub buffer: Buffer,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        let area = Rect::new(0, 0, width, height);
        Self {
            area,
            buffer: Buffer::empty(area),
        }
    }

    pub fn draw(&mut self, textarea: &TextArea<'_>) {
        self.buffer.reset();
        black_box(textarea).render(self.area, &mut self.buffer);
        black_box(&self.buffer);
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.area = Rect::new(0, 0, width, height);
        self.buffer.resize(self.area);
    }
}
