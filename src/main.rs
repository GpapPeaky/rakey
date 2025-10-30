use macroquad::prelude::*;

#[path = "editor_audio.rs"]
mod editor_audio;
use editor_audio::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

#[macroquad::main("rakey")]
async fn main() {
    set_fullscreen(true);
    
    let audio = EditorAudio::load().await;
    let mut file_text: Vec<String> = vec![];
    let mut file_cursor = ( 0, 0 ); // Cursor's x and y
    
    loop {
        clear_background(BACKGROUND_COLOR);

        file_text_navigation(&mut file_cursor, &mut file_text).await;

        record_keyboard_to_file_text(&mut file_cursor.0, &mut file_cursor.1, &mut file_text, &audio);

        draw(&mut file_text, file_cursor.0, file_cursor.1);

        draw_fps();

        next_frame().await;
    }
}
