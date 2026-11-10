use crate::{
    decode::{lines, Decoder},
    model::Model,
    support, MAX_STEPS,
};
use tui_textarea::{CursorMove, TextArea};

pub fn run(bytes: &[u8]) {
    let mut data = Decoder::new(bytes);
    let header = data.byte();
    let initial = data.initial(header);
    let mut textarea = TextArea::new(initial.clone());
    let mut view = support::View::default();
    view.draw(&textarea);
    if header & 8 == 0 {
        textarea.set_max_histories(MAX_STEPS * 2);
        let mut model = Model::new(&initial.join("\n"));
        for step in 0..MAX_STEPS {
            let Some(tag) = data.next() else { break };
            let op = tag % 11;
            let before = model.state.clone();
            match op {
                0 => {
                    let text = data.text();
                    // Match the string API's line-ending normalization, not its storage algorithm.
                    let normalized = text.replace("\r\n", "\n");
                    let normalized = normalized.strip_suffix('\r').unwrap_or(&normalized);
                    textarea.insert_str(&text);
                    model.state.insert(normalized.chars());
                }
                1 => {
                    let ch = data.character();
                    textarea.insert_char(ch);
                    model.state.insert([if ch == '\r' { '\n' } else { ch }]);
                }
                2 => {
                    textarea.delete_char();
                    if model.state.cursor > 0 {
                        model.state.cursor -= 1;
                        model.state.text.remove(model.state.cursor);
                    }
                }
                3 => {
                    textarea.delete_next_char();
                    if model.state.cursor < model.state.text.len() {
                        model.state.text.remove(model.state.cursor);
                    }
                }
                4 => {
                    let count = data.count();
                    // EOF deletions can record empty history entries; the API lane covers those.
                    if model.state.cursor < model.state.text.len() {
                        textarea.delete_str(count);
                        let end = model
                            .state
                            .cursor
                            .saturating_add(count)
                            .min(model.state.text.len());
                        model.state.text.drain(model.state.cursor..end);
                    }
                }
                5 => {
                    let row = data.word();
                    let col = data.word();
                    textarea.move_cursor(CursorMove::Jump(row, col));
                    model.state.jump(row as usize, col as usize);
                }
                6 => {
                    textarea.insert_newline();
                    model.state.insert(['\n']);
                }
                7 => {
                    textarea.clear();
                    model.state.text.clear();
                    model.state.cursor = 0;
                }
                8 => {
                    let text = data.text();
                    let row = data.word();
                    let col = data.word();
                    textarea.set_lines(lines(&text), (row as usize, col as usize));
                    model = Model::new(&text);
                    model.state.jump(row as usize, col as usize);
                    assert!(!textarea.undo());
                    assert!(!textarea.redo());
                }
                9 | 10 => {
                    let expected = if op == 9 { model.undo() } else { model.redo() };
                    let actual = if op == 9 {
                        textarea.undo()
                    } else {
                        textarea.redo()
                    };
                    assert_eq!(actual, expected, "history availability at step {step}");
                    support::bounds(&textarea);
                    // Undo cursor placement differs by edit kind. Model text exactly and
                    // use the valid public cursor as the origin of subsequent operations.
                    let (row, col) = textarea.cursor();
                    model.state.jump(row, col);
                }
                _ => unreachable!(),
            }
            if !matches!(op, 5 | 8 | 9 | 10) {
                model.commit(before);
            }
            assert_eq!(
                support::flat(&textarea),
                model.state.text,
                "text at step {step}, op {op}"
            );
            assert_eq!(
                textarea.cursor(),
                model.state.position(),
                "cursor at step {step}, op {op}"
            );
            support::bounds(&textarea);
            if !support::budget(&textarea) {
                break;
            }
            view.draw(&textarea);
        }
    } else {
        for _ in 0..MAX_STEPS {
            let Some(tag) = data.next() else { break };
            match tag % 25 {
                0 => {
                    textarea.insert_str(data.text());
                }
                1 => textarea.insert_char(data.character()),
                2 => {
                    textarea.delete_char();
                }
                3 => {
                    textarea.delete_next_char();
                }
                4 => {
                    textarea.delete_str(data.count());
                }
                5 => textarea.move_cursor(CursorMove::Jump(data.word(), data.word())),
                6 => textarea.insert_newline(),
                7 => {
                    textarea.clear();
                }
                8 => {
                    let replacement = lines(&data.text());
                    textarea.set_lines(replacement, (data.word() as usize, data.word() as usize));
                    assert!(!textarea.undo());
                    assert!(!textarea.redo());
                }
                9 => {
                    textarea.undo();
                }
                10 => {
                    textarea.redo();
                }
                11 => textarea.move_cursor(data.movement()),
                12 => {
                    textarea.input(data.input());
                }
                13 => {
                    textarea.input_without_shortcuts(data.input());
                }
                14 => {
                    textarea.delete_word();
                }
                15 => {
                    textarea.delete_next_word();
                }
                16 => {
                    textarea.delete_line_by_head();
                }
                17 => {
                    textarea.delete_line_by_end();
                }
                18 => {
                    textarea.delete_newline();
                }
                19 => {
                    textarea.insert_tab();
                }
                20 => textarea.set_max_histories(usize::from(data.byte() % 65)),
                21 => textarea.set_undo_coalescing(data.byte() & 1 != 0),
                22 => support::isolated_edit_round_trip(&textarea),
                23 => textarea.set_yank_text(data.text()),
                _ => support::history_round_trip(&textarea),
            }
            support::bounds(&textarea);
            if !support::budget(&textarea) {
                break;
            }
            view.draw(&textarea);
        }
    }
    support::history_round_trip(&textarea);
}
