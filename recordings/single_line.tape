Output assets/single_line.gif
Require tui-textarea-single-line
Set Height 200
Source recordings/common.tape
Env TUI_TEXTAREA_RECORDING_SIZE "120x10"

Hide
Type "tui-textarea-single-line"
Enter
Sleep 750ms
Show

Type "not a number"
Sleep 750ms
Ctrl+J
Type "1.56"
Sleep 1s

Hide
Enter
Sleep 250ms
