Output assets/editor.gif
Require tui-textarea-editor
Set Height 480
Source recordings/common.tape
Env TUI_TEXTAREA_RECORDING_SIZE "120x24"

Hide
Type "tui-textarea-editor /opt/tui-textarea/editor.txt"
Enter
Sleep 750ms
Show

Down 2
Ctrl+E
Enter
Type "The automated recording is reproducible."
Sleep 750ms
Ctrl+G
Type "recording"
Sleep 750ms
Enter
Sleep 1s

Hide
Ctrl+Q
Sleep 250ms
