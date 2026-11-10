//! Bounded operation traces and behavioral oracles shared by fuzzing and replay.
mod decode;
mod editing;
mod layout;
mod model;
mod search;
pub mod seeds;
mod selection;
mod support;

pub const MAX_INPUT: usize = 4096;
pub const MAX_STEPS: usize = 128;
pub const MAX_CHARS: usize = 2048;
pub const MAX_LINES: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    EditingHistory,
    SelectionAtomic,
    Layout,
    Search,
}

impl Target {
    pub const ALL: [Self; 4] = [
        Self::EditingHistory,
        Self::SelectionAtomic,
        Self::Layout,
        Self::Search,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::EditingHistory => "editing_history",
            Self::SelectionAtomic => "selection_atomic",
            Self::Layout => "layout",
            Self::Search => "search",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|target| target.name() == name)
    }
}

pub fn run(target: Target, data: &[u8]) {
    match target {
        Target::EditingHistory => editing::run(data),
        Target::SelectionAtomic => selection::run(data),
        Target::Layout => layout::run(data),
        Target::Search => search::run(data),
    }
}
