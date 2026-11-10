use tui_textarea::{CursorMove, Input, Key, WrapMode};

pub struct Decoder<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: &data[..data.len().min(crate::MAX_INPUT)],
            offset: 0,
        }
    }

    pub fn next(&mut self) -> Option<u8> {
        let value = self.data.get(self.offset).copied()?;
        self.offset += 1;
        Some(value)
    }

    pub fn byte(&mut self) -> u8 {
        self.next().unwrap_or(0)
    }

    pub fn word(&mut self) -> u16 {
        u16::from_le_bytes([self.byte(), self.byte()])
    }

    pub fn count(&mut self) -> usize {
        match self.byte() {
            0 => 0,
            1 => usize::MAX,
            n => usize::from(n - 1),
        }
    }

    pub fn text(&mut self) -> String {
        let len = usize::from(self.byte()) % 65;
        let end = (self.offset + len).min(self.data.len());
        let value = String::from_utf8_lossy(&self.data[self.offset..end]).into_owned();
        self.offset = end;
        value
    }

    pub fn initial(&mut self, header: u8) -> Vec<String> {
        let text = match header % 8 {
            0 => String::new(),
            1 => "alpha beta\nsecond line\n".into(),
            2 => "ab界cd\t🦀\n東京 abc\n".into(),
            3 => "cafe\u{301} 👩\u{200d}💻\nzero\u{200d}width".into(),
            4 => "x".repeat(1024),
            5 => "a  b\t c\n\nlast\n".into(),
            6 => "left [[token]] right\nanother [[atom]]\n".into(),
            _ => self.text(),
        };
        lines(&text)
    }

    pub fn character(&mut self) -> char {
        character(self.byte())
    }

    pub fn movement(&mut self) -> CursorMove {
        let tag = self.byte() % 15;
        let row = self.word();
        let col = self.word();
        match tag {
            0 => CursorMove::Forward,
            1 => CursorMove::Back,
            2 => CursorMove::Up,
            3 => CursorMove::Down,
            4 => CursorMove::Head,
            5 => CursorMove::End,
            6 => CursorMove::Top,
            7 => CursorMove::Bottom,
            8 => CursorMove::WordForward,
            9 => CursorMove::WordEnd,
            10 => CursorMove::WordBack,
            11 => CursorMove::ParagraphForward,
            12 => CursorMove::ParagraphBack,
            13 => CursorMove::Jump(row, col),
            _ => CursorMove::InViewport,
        }
    }

    pub fn input(&mut self) -> Input {
        let tag = self.byte() % 21;
        let value = self.byte();
        let flags = self.byte();
        let key = match tag {
            0 => Key::Char(character(value)),
            1 => Key::F(value % 12 + 1),
            2 => Key::Backspace,
            3 => Key::Enter,
            4 => Key::Left,
            5 => Key::Right,
            6 => Key::Up,
            7 => Key::Down,
            8 => Key::Tab,
            9 => Key::Delete,
            10 => Key::Home,
            11 => Key::End,
            12 => Key::PageUp,
            13 => Key::PageDown,
            14 => Key::Esc,
            15 => Key::Copy,
            16 => Key::Cut,
            17 => Key::Paste,
            18 => Key::MouseScrollDown,
            19 => Key::MouseScrollUp,
            _ => Key::Null,
        };
        Input {
            key,
            ctrl: flags & 1 != 0,
            alt: flags & 2 != 0,
            shift: flags & 4 != 0,
        }
    }

    pub fn wrap(&mut self) -> WrapMode {
        match self.byte() % 4 {
            0 => WrapMode::None,
            1 => WrapMode::Word,
            2 => WrapMode::Glyph,
            _ => WrapMode::WordOrGlyph,
        }
    }
}

pub fn character(value: u8) -> char {
    match value {
        0..=25 => char::from(b'a' + value),
        26..=51 => char::from(b'A' + value - 26),
        52..=61 => char::from(b'0' + value - 52),
        _ => [
            ' ', '_', '界', '🦀', '\u{301}', '\t', '\n', '\r', '\0', '\u{200d}', '👩', '💻',
            '\u{fe0f}', '.', '!', 'ß',
        ][usize::from(value - 62) % 16],
    }
}

pub fn lines(text: &str) -> Vec<String> {
    text.split('\n').map(str::to_owned).collect()
}
