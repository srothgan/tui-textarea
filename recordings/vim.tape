Output assets/vim.gif
Require tui-textarea-vim
Set Height 320
Source recordings/common.tape
Env TUI_TEXTAREA_RECORDING_SIZE "120x16"

Hide
Type "tui-textarea-vim"
Enter
Sleep 750ms
Show

Type "i"
Type "Vim-style editing with tui-textarea"
Sleep 500ms
Escape
Type "bbb"
Type "v"
Type "e"
Sleep 750ms
Type "y"
Sleep 1s

Hide
Type "q"
Sleep 250ms
