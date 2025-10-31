// Cursor navigation module

use macroquad::prelude::*;

use crate::editor_audio::EditorAudio;

#[allow(dead_code)]
pub struct EditorCursor {
    pub xy: (usize, usize)
}

impl EditorCursor {
    #[allow(dead_code)]
    pub fn new() -> EditorCursor {
        EditorCursor { xy: (0, 0) }
    }
}

pub static CURSOR_LINE_TO_WIDTH: bool = true;

/// Standard cursor navigation
#[allow(dead_code)] // Compiler won't shut the fuck up
pub fn file_text_navigation(cursor: &mut (usize, usize), text: &mut Vec<String>, audio: &EditorAudio) {
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
            cursor.0 = text[cursor.1].len();
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

/// Calculate the distance from the left or right of a whitespace if the cursor is inside text
/// or a character if the cursor is inside whitespace
#[allow(dead_code)]
fn calibrate_distance_to_whitespace_or_character(leftorright: bool, cursor_idx: usize, line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    if len == 0 {
        return 0;
    }

    let mut cursor = cursor_idx.min(len);
    let mut steps = 0;

    if leftorright {
        if cursor >= len {
            return 0;
        }

        let is_space = chars[cursor] == ' ';
        for i in cursor..len {
            if chars[i] == ' ' && !is_space {
                break;
            }
            if chars[i] != ' ' && is_space {
                break;
            }
            steps += 1;
        }

        return steps;
    } else {
        if cursor == 0 {
            return 0;
        }

        cursor -= 1;
        let is_space = chars[cursor] == ' ';

        while cursor > 0 {
            if chars[cursor - 1] == ' ' && !is_space {
                break;
            }
            if chars[cursor - 1] != ' ' && is_space {
                break;
            }
            cursor -= 1;
            steps += 1;
        }

        steps + 1
    }
}

/// Faster cursor navigation inside the file
/// only usable when the LCTRL key is down
#[allow(dead_code)]
pub fn file_text_special_navigation(cursor: &mut (usize, usize), text: &mut Vec<String>, audio: &EditorAudio) {
    let line = &text[cursor.1];
    let left_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(false, cursor.0, line);
    let right_steps_to_whitespace = calibrate_distance_to_whitespace_or_character(true, cursor.0, line);

    // Unsure what to do with this
    // if is_key_pressed(KeyCode::Up) {
    //     if cursor.1 > 0 {
    //         audio.play_nav();
    //         cursor.1 -= 1;
    //         cursor.0 = text[cursor.1].len();
    //     }
    // }

    // if is_key_pressed(KeyCode::Down) {
    //     if text.len() > cursor.1 + 1 {
    //         audio.play_nav();
    //         cursor.1 += 1;
    //         cursor.0 = text[cursor.1].len();
    //     }
    // }

    if is_key_pressed(KeyCode::Left) {
        if cursor.0 > 0 {
            audio.play_nav();
            cursor.0 = cursor.0.saturating_sub(left_steps_to_whitespace);
        } else if cursor.1 > 0 {
            audio.play_nav();
            cursor.1 -= 1;
            cursor.0 = text[cursor.1].len().saturating_sub(1);
        }
    }
    
    if is_key_pressed(KeyCode::Right) {
        if cursor.0 < text[cursor.1].len() {
            audio.play_nav();
            cursor.0 += right_steps_to_whitespace.min(text[cursor.1].len() - cursor.0);
        } else if cursor.1 + 1 < text.len() {
            audio.play_nav();
            cursor.1 += 1;
            cursor.0 = 0;
        }
    }
}
