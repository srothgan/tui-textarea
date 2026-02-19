#![no_main]

use arbitrary::{Arbitrary, Result, Unstructured};
use libfuzzer_sys::fuzz_target;
use tui_textarea::{CursorMove, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt};

#[derive(Arbitrary)]
enum Op {
    InsertStr(String),
    InsertChar(char),
    InsertNewline,
    DeleteStr(usize),
    DeleteChar,
    DeleteNextChar,
    Clear,
    Move(CursorMove),
    Undo,
    Redo,
    SetMaxHistories(u8),
}

fn assert_invariants(textarea: &TextArea<'_>) {
    let lines = textarea.lines();
    assert!(!lines.is_empty(), "lines must never be empty");
    let (row, col) = textarea.cursor();
    assert!(
        row < lines.len(),
        "cursor row {row} out of bounds (lines: {})",
        lines.len()
    );
    assert!(
        col <= lines[row].chars().count(),
        "cursor col {col} out of bounds (line {row} chars: {})",
        lines[row].chars().count(),
    );
}

fn fuzz(data: &[u8]) -> Result<()> {
    let mut term = dummy_terminal();
    let mut data = Unstructured::new(data);
    let text = <&str>::arbitrary(&mut data)?;
    let mut textarea = TextArea::from(text.lines());
    for _ in 0..100 {
        match Op::arbitrary(&mut data)? {
            Op::InsertStr(s) => {
                textarea.insert_str(s);
            }
            Op::InsertChar(c) => textarea.insert_char(c),
            Op::InsertNewline => textarea.insert_newline(),
            Op::DeleteStr(n) => {
                textarea.delete_str(n);
            }
            Op::DeleteChar => {
                textarea.delete_char();
            }
            Op::DeleteNextChar => {
                textarea.delete_next_char();
            }
            Op::Clear => {
                textarea.clear();
            }
            Op::Move(m) => textarea.move_cursor(m),
            Op::Undo => {
                textarea.undo();
            }
            Op::Redo => {
                textarea.redo();
            }
            Op::SetMaxHistories(n) => textarea.set_max_histories(n as usize),
        }
        term.draw_textarea(&textarea);
        assert_invariants(&textarea);
    }
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let _ = fuzz(data);
});
