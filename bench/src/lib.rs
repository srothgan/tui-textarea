// Compatibility helpers for the existing fuzz targets. Benchmarks paint into Frame directly.

use ratatui::backend::{Backend, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};
use ratatui::prelude::backend::ClearType;
use ratatui::Terminal;
use std::io;
use tui_textarea::TextArea;

mod workloads;
pub use workloads::{document, fixtures, Fixture, Frame};

pub struct DummyBackend {
    width: u16,
    height: u16,
    cursor: (u16, u16),
}

impl Default for DummyBackend {
    #[inline]
    fn default() -> Self {
        Self {
            width: 40,
            height: 12,
            cursor: (0, 0),
        }
    }
}

impl Backend for DummyBackend {
    type Error = io::Error;

    #[inline]
    fn draw<'a, I>(&mut self, _content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        Ok(())
    }

    #[inline]
    fn hide_cursor(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn show_cursor(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        Ok(self.cursor)
    }

    #[inline]
    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.cursor = (x, y);
        Ok(())
    }

    #[inline]
    fn clear(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn clear_region(&mut self, _clear_type: ClearType) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn size(&self) -> io::Result<Size> {
        Ok(Size {
            width: self.width,
            height: self.height,
        })
    }

    #[inline]
    fn window_size(&mut self) -> io::Result<WindowSize> {
        Ok(WindowSize {
            columns_rows: Size {
                width: self.width,
                height: self.height,
            },
            pixels: Size {
                width: self.width * 6,
                height: self.height * 12,
            },
        })
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn get_cursor_position(&mut self) -> io::Result<Position> {
        let (x, y) = self.cursor;
        Ok(Position { x, y })
    }

    #[inline]
    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        let Position { x, y } = position.into();
        self.cursor = (x, y);
        Ok(())
    }
}

impl DummyBackend {
    #[inline]
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}

#[inline]
pub fn dummy_terminal() -> Terminal<DummyBackend> {
    Terminal::new(DummyBackend::default()).unwrap()
}

pub trait TerminalExt {
    fn draw_textarea(&mut self, textarea: &TextArea<'_>);
}

impl TerminalExt for Terminal<DummyBackend> {
    #[inline]
    fn draw_textarea(&mut self, textarea: &TextArea<'_>) {
        self.draw(|f| f.render_widget(textarea, f.area())).unwrap();
    }
}
