Output assets/minimal.gif
Require tui-textarea-minimal
Set Height 360
Source recordings/common.tape
Env TUI_TEXTAREA_RECORDING_SIZE "120x18"

Hide
Type "tui-textarea-minimal"
Enter
Sleep 750ms
Show

Type "Hello from tui-textarea"
Enter
Type "Second editable line"
Sleep 750ms
Up
Sleep 500ms
Down
Sleep 1s

Hide
Escape
Sleep 250ms
