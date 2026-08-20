mod common;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io;
use tui_textarea::{Input, Key, TextArea};

const MIN_ROWS: u16 = 3;
const MAX_ROWS: u16 = 8;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let recording = common::maybe_force_recording_size(&mut stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_min_rows(MIN_ROWS);
    textarea.set_max_rows(MAX_ROWS);
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Auto-sized TextArea"),
    );

    loop {
        term.draw(|f| {
            let measure = textarea.measure(f.area().width);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(measure.preferred_rows),
                    Constraint::Length(2),
                    Constraint::Min(0),
                ])
                .split(f.area());
            f.render_widget(&textarea, chunks[0]);
            f.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled("preferred rows: ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            measure.preferred_rows.to_string(),
                            Style::default().fg(Color::LightCyan),
                        ),
                        Span::raw(format!("  (min: {}, max: {})", measure.min_rows, measure.max_rows)),
                    ]),
                    Line::styled(
                        "Type lines to grow the textarea; Backspace or Ctrl+Z to shrink it; Esc exits.",
                        Style::default().fg(Color::DarkGray),
                    ),
                ]),
                chunks[1],
            );
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    recording.restore(term.backend_mut())?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
