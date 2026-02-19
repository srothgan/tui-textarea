#![no_main]

use arbitrary::{Arbitrary, Result, Unstructured};
use libfuzzer_sys::fuzz_target;
use tui_textarea::{CursorMove, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt};

#[derive(Arbitrary)]
enum Op {
    SetPattern(String),
    SearchForward(bool),
    SearchBack(bool),
    Move(CursorMove),
    InsertStr(String),
    DeleteStr(usize),
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
            Op::SetPattern(pat) => {
                let _ = textarea.set_search_pattern(&pat);
            }
            Op::SearchForward(match_cursor) => {
                textarea.search_forward(match_cursor);
            }
            Op::SearchBack(match_cursor) => {
                textarea.search_back(match_cursor);
            }
            Op::Move(m) => textarea.move_cursor(m),
            Op::InsertStr(s) => {
                textarea.insert_str(s);
            }
            Op::DeleteStr(n) => {
                textarea.delete_str(n);
            }
        }
        term.draw_textarea(&textarea);
        assert_invariants(&textarea);
    }
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let _ = fuzz(data);
});
