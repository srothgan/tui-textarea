# Example Recording

This directory supports an opt-in recording mode for the crossterm examples so GIF captures can use the same terminal size and aspect ratio.

## Default Size

Set `TUI_TEXTAREA_RECORDING=1` to force the default recording size:

- `120x30`
- aspect ratio: `4:1`

PowerShell:

```powershell
$env:TUI_TEXTAREA_RECORDING=1
cargo run --example minimal
```

## Custom Size

Set `TUI_TEXTAREA_RECORDING_SIZE=<cols>x<rows>` to override the default:

```powershell
$env:TUI_TEXTAREA_RECORDING_SIZE="140x35"
cargo run --example split
```

Valid format:

- `120x30`
- `140x35`
- `160x40`

## Covered Examples

The shared recording-size helper is enabled for these crossterm examples:

- `minimal`
- `editor`
- `single_line`
- `split`
- `variable`
- `vim`
- `popup_placeholder`
- `password`

`termion` and `termwiz` are not wired through this helper.

## Notes

- The helper resizes the terminal on startup and restores the previous size on exit.
- Use one fixed size for a whole GIF set if you want every capture to align cleanly in the README.
- If you only care about consistent aspect ratio, choose any size with the same width-to-height ratio.
