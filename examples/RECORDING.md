# Recording Examples

The crossterm examples share an opt-in recording mode so every capture uses the same terminal dimensions. The recommended Windows workflow uses ScreenToGif for capture and the repository script for repeatable example startup; VHS is not part of this workflow.

## Capture Preset

Create a reusable ScreenToGif recorder preset with these settings:

- Capture the terminal client area at `120x30` cells.
- Use Cascadia Mono at the same font size, terminal theme, and window position for the complete set.
- Record at 15 frames per second.
- Hide the mouse pointer unless the example demonstrates mouse interaction.
- Trim startup, shutdown, and long idle frames before exporting.
- Export an optimized looping GIF under `.github/assets/examples/<example-name>.gif`.

Use ScreenToGif's window selection or snap-to-window support after the script resizes the terminal. Keep the recorder region unchanged between examples so the resulting GIFs align in the README.

## Run an Example

From the repository root, start a recordable example with:

```powershell
.\scripts\run-example-recording.ps1 minimal
```

The script sets the recording environment, requests the canonical `120x30` terminal size, runs Cargo with the correct arguments, and restores the caller's environment variables afterward. It also creates a disposable editor fixture under `target/recording` when recording the `editor` example.

Pass a different terminal size only when intentionally producing a separate set:

```powershell
.\scripts\run-example-recording.ps1 split -Size 140x35
```

The underlying environment variables can still be used directly:

```powershell
$env:TUI_TEXTAREA_RECORDING = "1"
$env:TUI_TEXTAREA_RECORDING_SIZE = "120x30"
cargo run --locked --example minimal
```

## Recording Sequence

Use the following short interaction as the canonical take for each example:

| Example | Recording sequence |
|---------|--------------------|
| `minimal` | Type two short lines, move the cursor, edit one word, then exit. |
| `editor` | Edit the fixture, open search with Ctrl+F, find `recording`, close search, then exit. |
| `single_line` | Enter invalid input, correct it to `1.56`, then submit. |
| `split` | Type in the first textarea, switch focus, type in the second, then exit. |
| `variable` | Enter lines until the textarea reaches its maximum height, then undo twice and exit. |
| `vim` | Enter insert mode, type a short sentence, return to normal mode, navigate, then exit. |
| `popup_placeholder` | Hold on the styled placeholder, type a short message to replace it, clear the message to reveal it again, then exit. |
| `password` | Type a short password, delete two characters, replace them, then submit. |
| `wrap` | Switch through keys 1 to 4, leave `WordOrGlyph` active, navigate through visual rows with Up and Down, then exit. |
| `undo_coalescing` | Type `hello`, pause for at least one second, type `world`, press Ctrl+Z twice with a short pause between presses, then exit. |

The `termion` and `termwiz` examples are excluded because this recording workflow targets the crossterm backend on Windows.

## Review Checklist

Before replacing a README recording, verify that:

- The GIF begins on a stable rendered frame and ends without showing terminal teardown.
- The capture has the same pixel dimensions, font, theme, and frame rate as the rest of the set.
- Keystrokes are deliberate and the demonstrated behavior is understandable without narration.
- Text remains legible after GitHub renders the image at the README width.
- The optimized file is small enough to load quickly and contains no unrelated desktop content.
- The filename matches the Rust example name with underscores changed to hyphens only when the README already uses that convention.

Commit the Rust example and its recording separately so code review remains clear and a GIF can be regenerated without obscuring source changes.
