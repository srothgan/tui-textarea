mod common;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io;
use tui_textarea::{Input, Key, TextArea};

fn centered_area(area: Rect) -> Rect {
    let width = 72.min(area.width);
    let height = 9.min(area.height);
    Rect {
        width,
        height,
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
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

    let mut textarea = TextArea::default();
    textarea.set_undo_coalescing(true);
    textarea.set_placeholder_text("Try: type \"hello \", pause, then type \"world\"");
    textarea.set_placeholder_style(Style::default().fg(Color::DarkGray));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Timed Undo Coalescing"),
    );

    loop {
        term.draw(|f| {
            let area = centered_area(f.area());
            let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(3)]).split(area);
            f.render_widget(&textarea, chunks[0]);
            f.render_widget(
                Paragraph::new(vec![
                    Line::styled(
                        "Characters typed continuously are grouped into one undo step.",
                        Style::default().fg(Color::LightCyan),
                    ),
                    Line::styled(
                        "Pause for at least 500 ms to start a new group, then press Ctrl+U once per group.",
                        Style::default().fg(Color::DarkGray),
                    ),
                    Line::styled("Esc exits.", Style::default().fg(Color::DarkGray)),
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

    Ok(())
}
