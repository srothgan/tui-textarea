#![no_main]

use arbitrary::{Arbitrary, Result, Unstructured};
use libfuzzer_sys::fuzz_target;
use tui_textarea::{CursorMove, TextArea};
use tui_textarea_bench::{dummy_terminal, TerminalExt};

#[derive(Arbitrary)]
enum Op {
    InsertStr(String),
    DeleteStr(usize),
    DeleteChar,
    Move(CursorMove),
    StartSelection,
    CancelSelection,
    SelectAll,
    Copy,
    Cut,
    Paste,
    SetYankText(String),
    Undo,
    Redo,
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
    if let Some(((sr, sc), (er, ec))) = textarea.selection_range() {
        assert!(sr < lines.len(), "selection start row {sr} out of bounds");
        assert!(
            sc <= lines[sr].chars().count(),
            "selection start col {sc} out of bounds (line {sr} chars: {})",
            lines[sr].chars().count(),
        );
        assert!(er < lines.len(), "selection end row {er} out of bounds");
        assert!(
            ec <= lines[er].chars().count(),
            "selection end col {ec} out of bounds (line {er} chars: {})",
            lines[er].chars().count(),
        );
        assert!(
            (sr, sc) <= (er, ec),
            "selection start ({sr},{sc}) > end ({er},{ec})",
        );
    }
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
            Op::DeleteStr(n) => {
                textarea.delete_str(n);
            }
            Op::DeleteChar => {
                textarea.delete_char();
            }
            Op::Move(m) => textarea.move_cursor(m),
            Op::StartSelection => textarea.start_selection(),
            Op::CancelSelection => textarea.cancel_selection(),
            Op::SelectAll => textarea.select_all(),
            Op::Copy => textarea.copy(),
            Op::Cut => {
                textarea.cut();
            }
            Op::Paste => {
                textarea.paste();
            }
            Op::SetYankText(s) => textarea.set_yank_text(s),
            Op::Undo => {
                textarea.undo();
            }
            Op::Redo => {
                textarea.redo();
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
