// Console module, see editor_directives.rs 
// for more info.

use macroquad::prelude::*;

pub struct EditorConsole {
    pub mode: bool,
    pub directive: Option<String>,
    pub directive_param: Option<String>
}

const CONSOLE_WIDTH: f32 = 255.0;

impl EditorConsole {
    /// Console constructor
    pub fn new() -> EditorConsole {
        EditorConsole { mode: false, directive: None, directive_param: None }
    }

    /// Console will be drawn to the right of the screen
    pub fn draw(&self) {
        // Console background
        draw_rectangle(screen_width() - CONSOLE_WIDTH,
            0.0,
            CONSOLE_WIDTH,
            screen_height(),
        WHITE
        );

        // Console foreground
        draw_rectangle(screen_width() - CONSOLE_WIDTH + 1.0,
            0.0,
            CONSOLE_WIDTH,
            screen_height(),
            WHITE
        );
    }
}
