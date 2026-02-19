#![no_main]

use arbitrary::{Arbitrary, Result, Unstructured};
use libfuzzer_sys::fuzz_target;
use tui_textarea::{CursorMove, TextArea, WrapMode};
use tui_textarea_bench::{dummy_terminal, TerminalExt};

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
    let wrap_mode = WrapMode::arbitrary(&mut data)?;
    let width: u16 = *data.choose(&[1, 4, 10, 40, 80, 200])?;

    term.backend_mut().resize(width, 12);

    let mut textarea = TextArea::from(text.lines());
    textarea.set_wrap_mode(wrap_mode);

    for _ in 0..100 {
        let m = CursorMove::arbitrary(&mut data)?;
        textarea.move_cursor(m);
        term.draw_textarea(&textarea);
        assert_invariants(&textarea);

        if bool::arbitrary(&mut data)? {
            let s = <&str>::arbitrary(&mut data)?;
            textarea.insert_str(s);
            term.draw_textarea(&textarea);
            assert_invariants(&textarea);
        }
    }
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let _ = fuzz(data);
});
