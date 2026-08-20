Output assets/undo_coalescing.gif
Require tui-textarea-undo-coalescing
Set Height 240
Source recordings/common.tape
Env TUI_TEXTAREA_RECORDING_SIZE "120x12"

Hide
Type "tui-textarea-undo-coalescing"
Enter
Sleep 750ms
Show

Type "hello "
Sleep 1s
Type "world"
Sleep 750ms
Ctrl+U
Sleep 750ms
Ctrl+U
Sleep 1s

Hide
Escape
Sleep 250ms
