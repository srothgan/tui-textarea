use crossterm::terminal::{SetSize, size};
use std::env;
use std::io::{self, Write};

const DEFAULT_RECORDING_WIDTH: u16 = 120;
const DEFAULT_RECORDING_HEIGHT: u16 = 30;
const RECORDING_FLAG: &str = "TUI_TEXTAREA_RECORDING";
const RECORDING_SIZE_VAR: &str = "TUI_TEXTAREA_RECORDING_SIZE";

#[derive(Default)]
pub struct RecordingSizeGuard {
    original_size: Option<(u16, u16)>,
}

impl RecordingSizeGuard {
    pub fn restore<W: Write>(mut self, writer: &mut W) -> io::Result<()> {
        if let Some((cols, rows)) = self.original_size.take() {
            crossterm::execute!(writer, SetSize(cols, rows))?;
        }
        Ok(())
    }
}

pub fn maybe_force_recording_size<W: Write>(writer: &mut W) -> io::Result<RecordingSizeGuard> {
    let Some((cols, rows)) = configured_recording_size()? else {
        return Ok(RecordingSizeGuard::default());
    };

    let original_size = size().ok();
    crossterm::execute!(writer, SetSize(cols, rows))?;

    Ok(RecordingSizeGuard { original_size })
}

fn configured_recording_size() -> io::Result<Option<(u16, u16)>> {
    if let Some(raw) = env::var_os(RECORDING_SIZE_VAR) {
        let raw = raw.into_string().map_err(|_| invalid_size())?;
        return parse_size(&raw).map(Some);
    }

    if env::var_os(RECORDING_FLAG).is_some() {
        return Ok(Some((DEFAULT_RECORDING_WIDTH, DEFAULT_RECORDING_HEIGHT)));
    }

    Ok(None)
}

fn parse_size(raw: &str) -> io::Result<(u16, u16)> {
    let (width, height) = raw.split_once('x').ok_or_else(invalid_size)?;
    let width = width.trim().parse().map_err(|_| invalid_size())?;
    let height = height.trim().parse().map_err(|_| invalid_size())?;

    if width == 0 || height == 0 {
        return Err(invalid_size());
    }

    Ok((width, height))
}

fn invalid_size() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("{RECORDING_SIZE_VAR} must be set as <cols>x<rows>, for example 120x30"),
    )
}
