mod common;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders};
use std::io;
use tui_textarea::{Input, Key, TextArea};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let recording = common::maybe_force_recording_size(&mut stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightBlue))
            .title("Styled Placeholder"),
    );

    textarea.set_style(Style::default().fg(Color::Yellow));
    textarea.set_styled_placeholder(
        Text::from_iter([
            Line::from(vec![
                Span::styled(
                    "Required: ",
                    Style::default()
                        .fg(Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("enter your message", Style::default().fg(Color::DarkGray)),
            ]),
            Line::styled(
                "The placeholder can contain multiple styled lines.",
                Style::default().fg(Color::LightBlue),
            ),
        ])
        .style(Style::default().fg(Color::DarkGray)),
    );
    loop {
        term.draw(|f| {
            let width = 58.min(f.area().width);
            let height = 5.min(f.area().height);
            let area = Rect {
                width,
                height,
                x: f.area().x + f.area().width.saturating_sub(width) / 2,
                y: f.area().y + f.area().height.saturating_sub(height) / 2,
            };
            f.render_widget(&textarea, area);
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
