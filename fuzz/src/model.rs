//! Independent flat Unicode-scalar buffer. The widget uses UTF-8 strings per line.
#[derive(Clone)]
pub struct State {
    pub text: Vec<char>,
    pub cursor: usize,
}

impl State {
    pub fn position(&self) -> (usize, usize) {
        self.text[..self.cursor]
            .iter()
            .fold((0, 0), |(row, col), ch| {
                if *ch == '\n' {
                    (row + 1, 0)
                } else {
                    (row, col + 1)
                }
            })
    }

    pub fn jump(&mut self, row: usize, col: usize) {
        let lines: Vec<_> = self.text.split(|ch| *ch == '\n').collect();
        let row = row.min(lines.len() - 1);
        self.cursor = lines[..row]
            .iter()
            .map(|line| line.len() + 1)
            .sum::<usize>()
            + col.min(lines[row].len());
    }

    pub fn insert(&mut self, text: impl IntoIterator<Item = char>) {
        let chars: Vec<_> = text.into_iter().collect();
        self.text
            .splice(self.cursor..self.cursor, chars.iter().copied());
        self.cursor += chars.len();
    }
}

pub struct Model {
    pub state: State,
    undo: Vec<(State, State)>,
    redo: Vec<(State, State)>,
}

impl Model {
    pub fn new(text: &str) -> Self {
        Self {
            state: State {
                text: text.chars().collect(),
                cursor: 0,
            },
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    pub fn commit(&mut self, before: State) {
        if before.text != self.state.text {
            self.undo.push((before, self.state.clone()));
            self.redo.clear();
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some((before, after)) = self.undo.pop() {
            self.state = before.clone();
            self.redo.push((before, after));
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some((before, after)) = self.redo.pop() {
            self.state = after.clone();
            self.undo.push((before, after));
            true
        } else {
            false
        }
    }
}
