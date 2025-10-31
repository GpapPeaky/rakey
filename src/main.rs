use macroquad::prelude::*;

#[path = "editor_audio.rs"]
mod editor_audio;
use editor_audio::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

#[macroquad::main("Calliope")]
async fn main() {
    set_fullscreen(true);
    
    // Editor audio
    let audio = EditorAudio::new().await;
    // General text stylizer
    let mut gts = EditorGeneralTextStylizer::new().await;

    let mut file_text: Vec<String> = vec![];
    let mut file_cursor = EditorCursor::new(); // Cursor's x and y
    
    loop {
        clear_background(BACKGROUND_COLOR);

        file_text_navigation(&mut file_cursor.xy, &mut file_text, &audio).await;

        record_keyboard_to_file_text(&mut file_cursor.xy.0, &mut file_cursor.xy.1, &mut file_text, &audio);

        draw(&mut file_text, file_cursor.xy.0, file_cursor.xy.1, &mut gts);

        draw_fps();

        next_frame().await;
    }
}
