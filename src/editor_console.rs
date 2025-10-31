// Console module, see editor_directives.rs 
// for more info.

use macroquad::prelude::*;

pub struct EditorConsole {
    pub is_open: bool,
    pub directive: &str,
    pub directive_param: Option<&str>
}

impl EditorConsole {
    
}
