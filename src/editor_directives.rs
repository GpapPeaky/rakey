// The console will be allowed to execute directives
// that enables the user to switch from folder to folder
// write files, remove files, visit a line in a file
// , change pallete and create files.
//
// The user can go from console, to insert mode and (vize versa)
// by pressing 'CTRL +`'.
//
// All directives have the ':' prefix, as the console will
// handle input as a switch-to-file directive.
//
// Directives include:
//      File specific:
//              :l <N>  : Go to line N inside the file, if possible, else throw an error
//              :w      : Write the current open file
//              :o      : Open a folder, create process -> native file explorer
//              :i      : Current file info display
//              :r <f>  : Remove a file with name 'f'
//      Folder specific:
//              :c <f>  : Create a new file with name 'f'
//              :f <f>  : Create a new folder with name 'f'
//              :rf <f> : Remove a folder with name 'f' with all its contents
//              :ls     : List all files in the folder
//
//      Options: <saved in options.rak file>
//              :p <p>  : Change to pallet of name 'p'
//              :audio  : Audio on/off switch
//              :e      : Exit, close editor
//              :smart  : Smart identation on/off switch
//              :v <N>  : Set editor sounds volume
//
// When the console is faced with a directive without a ':' prefix
// it will view it as a switch-to-file command and will try to switch 
// to a file with that name if found.
// The console, as long as you are typing, will display files with names close to it.
// Pressing TAB will select the first seen file closest to the name given and autocomplete it
// in the console.
//