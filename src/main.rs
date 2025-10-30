use macroquad::prelude::*;

mod editor_cursor;
use editor_cursor::*;

mod editor_text;
use editor_text::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "rakey".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_fullscreen(true);
    
    let mut file_text: Vec<String> = vec![];
    let mut file_cursor = ( 0, 0 );
    
    loop {
        clear_background(BACKGROUND_COLOR);

        file_text_navigation(&mut file_cursor, &mut file_text);

        record_keyboard_to_file_text(&mut file_cursor.0, &mut file_cursor.1, &mut file_text);

        draw(&mut file_text, file_cursor.0, file_cursor.1);

        draw_fps();

        next_frame().await;
    }
}
