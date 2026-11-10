use crate::{
    decode::{lines, Decoder},
    support, MAX_STEPS,
};
use tui_textarea::{CursorMove, TextArea};

fn navigate(textarea: &mut TextArea<'_>, backward: bool, include: bool, simple: bool) {
    let before = textarea.cursor();
    let regex = textarea.search_pattern().cloned();
    let expected = if simple {
        let positions: Vec<_> = regex
            .as_ref()
            .into_iter()
            .flat_map(|regex| {
                textarea
                    .lines()
                    .iter()
                    .enumerate()
                    .flat_map(move |(row, line)| {
                        regex
                            .find_iter(line)
                            .map(move |found| (row, line[..found.start()].chars().count()))
                    })
            })
            .collect();
        if backward {
            positions
                .iter()
                .rev()
                .copied()
                .find(|position| {
                    if include {
                        *position <= before
                    } else {
                        *position < before
                    }
                })
                .or_else(|| positions.last().copied())
        } else {
            positions
                .iter()
                .copied()
                .find(|position| {
                    if include {
                        *position >= before
                    } else {
                        *position > before
                    }
                })
                .or_else(|| positions.first().copied())
        }
    } else {
        None
    };
    let found = if backward {
        textarea.search_back(include)
    } else {
        textarea.search_forward(include)
    };
    if simple {
        assert_eq!(
            found.then(|| textarea.cursor()),
            expected,
            "single-character search order"
        );
    }
    if found {
        let regex = regex.as_ref().expect("search succeeded without a pattern");
        let (row, col) = textarea.cursor();
        assert!(row < textarea.lines().len());
        let line = &textarea.lines()[row];
        assert!(col <= line.chars().count());
        let byte = line
            .char_indices()
            .nth(col)
            .map_or(line.len(), |(byte, _)| byte);
        assert_eq!(
            regex.find_at(line, byte).map(|matched| matched.start()),
            Some(byte),
            "search cursor must start a real regex match"
        );
    } else {
        assert_eq!(textarea.cursor(), before);
        if let Some(regex) = regex {
            assert!(
                !textarea.lines().iter().any(|line| regex
                    .find_iter(line)
                    .any(|found| found.start() != found.end())),
                "search missed every nonempty match"
            );
        }
    }
}

pub fn run(bytes: &[u8]) {
    let mut data = Decoder::new(bytes);
    let header = data.byte();
    let mut textarea = TextArea::new(data.initial(header));
    textarea.set_max_histories(32);
    let mut simple = false;
    let mut view = support::View::default();
    for _ in 0..MAX_STEPS {
        let Some(tag) = data.next() else { break };
        match tag % 14 {
            0 => {
                let index = data.byte() % 17;
                let pattern = if index == 15 {
                    regex::escape(&data.text())
                } else {
                    match index {
                        0 => "a",
                        1 => "界",
                        2 => "🦀",
                        3 => r"\d",
                        4 => r"\p{L}",
                        5 => r"[ \t]",
                        6 => r"\w+",
                        7 => "a+",
                        8 => "^",
                        9 => "$",
                        10 => "^$",
                        11 => r"\b",
                        12 => "a*",
                        13 => "(",
                        14 => "",
                        _ => ".",
                    }
                    .to_owned()
                };
                let old = textarea
                    .search_pattern()
                    .map(|pattern| pattern.as_str().to_owned());
                match textarea.set_search_pattern(&pattern) {
                    Ok(()) => {
                        simple = index <= 5 || index == 16;
                    }
                    Err(_) => assert_eq!(
                        textarea
                            .search_pattern()
                            .map(|pattern| pattern.as_str().to_owned()),
                        old
                    ),
                }
            }
            1 | 2 => navigate(&mut textarea, tag % 14 == 2, data.byte() & 1 != 0, simple),
            3 => {
                textarea.insert_str(data.text());
            }
            4 => {
                textarea.delete_str(data.count());
            }
            5 => textarea.move_cursor(CursorMove::Jump(data.word(), data.word())),
            6 => textarea.move_cursor(data.movement()),
            7 => textarea.set_lines(
                lines(&data.text()),
                (data.word() as usize, data.word() as usize),
            ),
            8 => {
                textarea.undo();
            }
            9 => {
                textarea.redo();
            }
            10 => view.resize(&mut data),
            11 => textarea.set_wrap_mode(data.wrap()),
            12 => textarea.set_max_histories(usize::from(data.byte() % 65)),
            _ => textarea.set_undo_coalescing(data.byte() & 1 != 0),
        }
        support::bounds(&textarea);
        if !support::budget(&textarea) {
            break;
        }
        view.draw(&textarea);
    }
    support::history_round_trip(&textarea);
}
