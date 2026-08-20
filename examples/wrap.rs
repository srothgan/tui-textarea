mod common;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io;
use tui_textarea::{Input, Key, TextArea, WrapMode};

fn centered_area(area: Rect) -> Rect {
    let width = 72.min(area.width);
    let height = 17.min(area.height);
    Rect {
        width,
        height,
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
    }
}

fn mode_name(mode: WrapMode) -> &'static str {
    match mode {
        WrapMode::None => "None",
        WrapMode::Word => "Word",
        WrapMode::Glyph => "Glyph",
        WrapMode::WordOrGlyph => "WordOrGlyph",
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let recording = common::maybe_force_recording_size(&mut stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::from([
        "Soft wrapping keeps long logical lines readable without changing their contents. Use Up and Down to move through each visual row.",
        "Unicode and tabs: 日本語 🚀 café\tone\ttwo\tthree",
        "AReallyLongTokenWithoutAnySpacesThatShowsTheDifferenceBetweenWordAndWordOrGlyphWrappingModes",
    ]);
    let mut mode = WrapMode::WordOrGlyph;
    textarea.set_wrap_mode(mode);
    textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Soft Wrapping"),
    );

    loop {
        term.draw(|f| {
            let area = centered_area(f.area());
            let chunks = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).split(area);
            f.render_widget(&textarea, chunks[0]);
            f.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled("Mode: ", Style::default().fg(Color::DarkGray)),
                        Span::styled(mode_name(mode), Style::default().fg(Color::LightCyan)),
                    ]),
                    Line::styled(
                        "1 None  2 Word  3 Glyph  4 WordOrGlyph  |  Up/Down navigate visual rows  |  Esc exits",
                        Style::default().fg(Color::DarkGray),
                    ),
                ]),
                chunks[1],
            );
        })?;

        let input: Input = crossterm::event::read()?.into();
        mode = match input {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: Key::Char('1'),
                ctrl: false,
                alt: false,
                ..
            } => WrapMode::None,
            Input {
                key: Key::Char('2'),
                ctrl: false,
                alt: false,
                ..
            } => WrapMode::Word,
            Input {
                key: Key::Char('3'),
                ctrl: false,
                alt: false,
                ..
            } => WrapMode::Glyph,
            Input {
                key: Key::Char('4'),
                ctrl: false,
                alt: false,
                ..
            } => WrapMode::WordOrGlyph,
            input => {
                textarea.input(input);
                continue;
            }
        };
        textarea.set_wrap_mode(mode);
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    recording.restore(term.backend_mut())?;
    term.show_cursor()?;

    Ok(())
}
