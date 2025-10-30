// Cursor navigation module

use macroquad::prelude::*;

use crate::editor_audio::EditorAudio;

#[allow(dead_code)] // Compiler won't shut the fuck up
pub async fn file_text_navigation(cursor: &mut (usize, usize), text: &mut Vec<String>, audio: &EditorAudio) {
    // TODO make up an down keys move to the same column if possible, else the smaller one.

    if is_key_pressed(KeyCode::Up) {
        if cursor.1 > 0 {
            audio.play_nav();
            cursor.1 -= 1;
            cursor.0 = text[cursor.1].len();
        }
    }

    if is_key_pressed(KeyCode::Down) {
        if text.len() > cursor.1 + 1 {
            audio.play_nav();
            cursor.1 += 1;
            cursor.0 = text[cursor.1].len();
        }
    }

    if is_key_pressed(KeyCode::Left) {
        if cursor.0 > 0 {
            audio.play_nav();
            cursor.0 -= 1;
        } else if cursor.1 > 0 {
            audio.play_nav();
            // Move to end of previous line
            cursor.1 -= 1;
            cursor.0 = text[cursor.1].len() - 1;
        }
    }

    if is_key_pressed(KeyCode::Right) {
        if cursor.0 < text[cursor.1].len() {
            audio.play_nav();
            cursor.0 += 1;
        } else if cursor.1 + 1 < text.len() {
            audio.play_nav();
            // Move to start of next line
            cursor.1 += 1;
            cursor.0 = 0;
        }
    }
}
