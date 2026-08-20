Output assets/split.gif
Require tui-textarea-split
Set Height 280
Source recordings/common.tape
Env TUI_TEXTAREA_RECORDING_SIZE "120x14"

Hide
Type "tui-textarea-split"
Enter
Sleep 750ms
Show

Type "Editing the left textarea"
Sleep 500ms
Ctrl+X
Type "Now editing the right textarea"
Sleep 1s

Hide
Escape
Sleep 250ms
