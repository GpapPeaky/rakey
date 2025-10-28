use macroquad::prelude::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

// TODO: Add backspace, enter handling, add '*' highlighting break exclusion

#[macroquad::main("rakey")]
async fn main() {
    let mut file_text: Vec<String> = vec![];
    let mut file_cursor: EditorCursor = EditorCursor::new();

    loop {
        clear_background(BLACK);

        record_keyboard_to_file_text(&mut file_cursor.x, &mut file_cursor.y, &mut file_text);

        draw(&mut file_text, file_cursor.x, file_cursor.y);

        next_frame().await;
    }
}
