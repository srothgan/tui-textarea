use crate::{
    decode::{lines, Decoder},
    support, MAX_STEPS,
};
use tui_textarea::{AtomicDeleteDirection, AtomicRange, CursorMove, TextArea};

fn atoms(textarea: &TextArea<'_>) {
    let mut previous = None;
    for range in textarea.atomic_ranges() {
        assert!(range.row < textarea.lines().len());
        assert!(range.start_col < range.end_col);
        assert!(range.end_col <= textarea.lines()[range.row].chars().count());
        if let Some((row, end)) = previous {
            assert!(range.row > row || (range.row == row && range.start_col >= end));
        }
        previous = Some((range.row, range.end_col));
        let (row, col) = textarea.cursor();
        assert!(row != range.row || col <= range.start_col || col >= range.end_col);
        if let Some((start, end)) = textarea.selection_range() {
            for (row, col) in [start, end] {
                assert!(row != range.row || col <= range.start_col || col >= range.end_col);
            }
        }
    }
}

fn delete_count(textarea: &mut TextArea<'_>, count: usize) {
    textarea.cancel_selection();
    let mut expected = support::flat(textarea);
    let start = support::offset(textarea.lines(), textarea.cursor());
    let mut end = start.saturating_add(count).min(expected.len());
    if start != end {
        for range in textarea.atomic_ranges() {
            let atom_start = support::offset(textarea.lines(), (range.row, range.start_col));
            let atom_end = support::offset(textarea.lines(), (range.row, range.end_col));
            if start < atom_end && end > atom_start {
                end = end.max(atom_end);
            }
        }
    }
    expected.drain(start..end);
    textarea.delete_str(count);
    assert_eq!(
        support::flat(textarea),
        expected,
        "range deletion must expand across intersecting atoms"
    );
}

pub fn run(bytes: &[u8]) {
    let mut data = Decoder::new(bytes);
    let header = data.byte();
    let mut textarea = TextArea::new(data.initial(header));
    textarea.set_max_histories(64);
    let mut view = support::View::default();
    for _ in 0..MAX_STEPS {
        let Some(tag) = data.next() else { break };
        match tag % 20 {
            0 => textarea.move_cursor(CursorMove::Jump(data.word(), data.word())),
            1 => textarea.start_selection(),
            2 => textarea.cancel_selection(),
            3 => textarea.select_all(),
            4 => {
                let expected = support::selected(&textarea);
                let before = textarea.lines().to_vec();
                let yank = textarea.yank_text();
                textarea.copy();
                let expected = expected.filter(|text| !text.is_empty()).unwrap_or(yank);
                assert_eq!(textarea.yank_text(), expected);
                assert_eq!(textarea.lines(), before);
            }
            5 => {
                let expected = support::selected(&textarea);
                let yank = textarea.yank_text();
                let mut contents = support::flat(&textarea);
                if let Some((start, end)) = textarea.selection_range() {
                    contents.drain(
                        support::offset(textarea.lines(), start)
                            ..support::offset(textarea.lines(), end),
                    );
                }
                let changed = textarea.cut();
                assert_eq!(changed, expected.as_ref().is_some_and(|s| !s.is_empty()));
                let expected = expected.filter(|text| !text.is_empty()).unwrap_or(yank);
                assert_eq!(textarea.yank_text(), expected);
                assert_eq!(support::flat(&textarea), contents);
            }
            6 => {
                let mut expected = support::flat(&textarea);
                let (start, end) = textarea
                    .selection_range()
                    .unwrap_or((textarea.cursor(), textarea.cursor()));
                let start = support::offset(textarea.lines(), start);
                let end = support::offset(textarea.lines(), end);
                expected.splice(start..end, textarea.yank_text().chars());
                textarea.paste();
                assert_eq!(support::flat(&textarea), expected);
            }
            7 => textarea.set_yank_text(data.text()),
            8 => {
                textarea.insert_str(data.text());
            }
            9 => delete_count(&mut textarea, data.count()),
            10 => {
                let row = usize::from(data.byte()) % textarea.lines().len();
                let len = textarea.lines()[row].chars().count();
                let start = usize::from(data.byte()) % (len + 1);
                let end = start.saturating_add(usize::from(data.byte()) + 1).min(len);
                let ranges = if start < end {
                    vec![AtomicRange {
                        row,
                        start_col: start,
                        end_col: end,
                    }]
                } else {
                    Vec::new()
                };
                textarea.try_set_atomic_ranges(ranges.clone()).unwrap();
                assert_eq!(textarea.atomic_ranges(), ranges);
            }
            11 => {
                let count = usize::from(data.byte() % 9);
                let ranges: Vec<_> = textarea
                    .lines()
                    .iter()
                    .enumerate()
                    .filter_map(|(row, line)| {
                        (!line.is_empty()).then_some(AtomicRange {
                            row,
                            start_col: 0,
                            end_col: line.chars().count().min(3),
                        })
                    })
                    .take(count)
                    .collect();
                let mut reversed = ranges.clone();
                reversed.reverse();
                textarea.try_set_atomic_ranges(reversed).unwrap();
                assert_eq!(textarea.atomic_ranges(), ranges);
            }
            12 => {
                let previous = textarea.atomic_ranges().to_vec();
                let cursor = textarea.cursor();
                let selection = textarea.selection_range();
                let bad = if data.byte() & 1 == 0 {
                    AtomicRange {
                        row: textarea.lines().len(),
                        start_col: 0,
                        end_col: 1,
                    }
                } else {
                    AtomicRange {
                        row: 0,
                        start_col: 0,
                        end_col: 0,
                    }
                };
                let error = textarea.try_set_atomic_ranges([bad]).unwrap_err();
                assert!(!error.rejected.is_empty());
                assert_eq!(textarea.atomic_ranges(), previous);
                assert_eq!(textarea.cursor(), cursor);
                assert_eq!(textarea.selection_range(), selection);
            }
            13 => {
                let direction = if data.byte() & 1 == 0 {
                    AtomicDeleteDirection::Forward
                } else {
                    AtomicDeleteDirection::Backward
                };
                let expected_range = textarea.atomic_range_at_cursor(direction);
                let mut expected = support::flat(&textarea);
                if let Some(range) = expected_range {
                    let start = support::offset(textarea.lines(), (range.row, range.start_col));
                    let end = support::offset(textarea.lines(), (range.row, range.end_col));
                    expected.drain(start..end);
                }
                assert_eq!(
                    textarea.delete_atomic_range_at_cursor(direction),
                    expected_range
                );
                assert_eq!(support::flat(&textarea), expected);
                if expected_range.is_some() {
                    assert!(textarea.atomic_ranges().is_empty());
                }
            }
            14 => {
                textarea.undo();
            }
            15 => {
                textarea.redo();
            }
            16 => {
                textarea.set_lines(
                    lines(&data.text()),
                    (data.word() as usize, data.word() as usize),
                );
                assert!(textarea.atomic_ranges().is_empty());
                assert!(!textarea.is_selecting());
                assert!(!textarea.undo());
            }
            17 => textarea.move_cursor(data.movement()),
            18 => {
                textarea.input(data.input());
            }
            _ => support::history_round_trip(&textarea),
        }
        support::bounds(&textarea);
        atoms(&textarea);
        if !support::budget(&textarea) {
            break;
        }
        view.draw(&textarea);
    }
    support::history_round_trip(&textarea);
}
