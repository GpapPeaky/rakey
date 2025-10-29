use macroquad::prelude::*;

#[path = "editor_cursor.rs"]
mod editor_cursor;

const ALPHA_VALUE: u8 = 255;
const FILE_TEXT_X_MARGIN: f32 = 10.0;
const FILE_TEXT_Y_MARGIN: f32 = 30.0;
const TAB_SIZE: usize = 5;
const TAB_PATTERN: &str = "     ";

const C_CONTROL_FLOW_STATEMENTS: [&str ; 12] = [
    "if",
    "else",
    "switch",
    "case",
    "default",
    "for",
    "while",
    "do",
    "break",
    "continue",
    "goto",
    "return"
];

const C_STORAGE_CLASS_SPECIFIERS: [&str ; 5] = [
    "auto",
    "static",
    "extern",
    "register",
    "typedef"
];

const C_TYPE_QUALIFIERS: [&str ; 1] = [
    "const"
];

const C_COMPOSITE_TYPES: [&str ; 3] = [
    "struct",
    "union",
    "enum"
];

const C_MISC: [&str ; 2] = [
    "sizeof",
    "inline"
];

const C_DATA_TYPES: [&str ; 9] = [
    "int",
    "float",
    "double",
    "char",
    "void",
    "short",
    "long",
    "signed",
    "unsigned"
];

fn char_to_byte(line: &str, char_idx: usize) -> usize {
    // We use UTF-8 so we need to count bytes NOT characters like C.
    line.char_indices().nth(char_idx).map(|(b, _)| b).unwrap_or(line.len())
}

pub fn calibrate_string_color(string: &str) -> Color {
    if C_CONTROL_FLOW_STATEMENTS.contains(&string) {
        return Color::from_rgba(150, 150, 200, ALPHA_VALUE);
    } else if C_TYPE_QUALIFIERS.contains(&string) {
        return Color::from_rgba(255, 255, 100, ALPHA_VALUE);
    } else if C_COMPOSITE_TYPES.contains(&string) {
        return Color::from_rgba(200, 0, 50, ALPHA_VALUE);
    } else if C_STORAGE_CLASS_SPECIFIERS.contains(&string) {
        return Color::from_rgba(0, 100, 250, ALPHA_VALUE);
    } else if C_MISC.contains(&string) {
        return Color::from_rgba(100, 255, 25, ALPHA_VALUE);
    } else if C_DATA_TYPES.contains(&string) {
        return Color::from_rgba(100, 100, 255, ALPHA_VALUE);
    } else {
        return Color::from_rgba(255, 255, 255, ALPHA_VALUE);
    }
}

pub fn record_special_keys(cursor_x: &mut usize, cursor_y: &mut usize, text: &mut Vec<String>) -> bool {
    if is_key_pressed(KeyCode::Backspace) {
        if *cursor_x > 0 {
            let line = &mut text[*cursor_y];
            
            if *cursor_x == 0 || line.is_empty() {
                return true;
            }

            // Check if we're just after a tab pattern
            if *cursor_x >= TAB_SIZE {
                let end = *cursor_x;
                let start = end - TAB_SIZE;

                let end_byte = char_to_byte(line, end);
                let start_byte = char_to_byte(line, start);
                if &line[start_byte..end_byte] == TAB_PATTERN {
                    line.replace_range(start_byte..end_byte, "");
                    *cursor_x -= TAB_SIZE;
                    return true;
                }
            }

            // Normal removal
            line.remove(*cursor_x - 1);
            *cursor_x -= 1;
        } else if *cursor_y > 0 {
            // Merge with previous line
            let current_line = text.remove(*cursor_y);
            *cursor_y -= 1;
            *cursor_x = text[*cursor_y].len();
            text[*cursor_y].push_str(&current_line);
        }

        // We will check if there was a special key pressed, and we will return from the 
        // normal keyboard recording function
        return true;
    }

    if is_key_pressed(KeyCode::Tab) {
        let line = &mut text[*cursor_y];
        let byte_idx = char_to_byte(line, *cursor_x);
        line.insert_str(byte_idx, TAB_PATTERN);

        *cursor_x += TAB_SIZE;

        return true;
    }

    if is_key_pressed(KeyCode::Enter) {
        let rest = text[*cursor_y].split_off(*cursor_x);
        *cursor_y += 1;
        *cursor_x = 0;
        text.insert(*cursor_y, rest);

        return true;
    }

    false
}

pub fn record_keyboard_to_file_text(cursor_x: &mut usize, cursor_y: &mut usize, text: &mut Vec<String>) {
    // let c = get_char_pressed().unwrap(); // Unwrap removes the Result/Option wrapper.

    if text.is_empty() { // Allocate memory for a new string
        text.push(String::new());
    }

    if record_special_keys(cursor_x, cursor_y, text) {
        return; // Handle the special key and terminate the call, as to 
        // not record any special escape character
    }

    if let Some(c) = get_char_pressed() {
        // We will also handle smart/smarter identation here.
        while *cursor_y >= text.len() {
            text.push(String::new());
        }

        if c.is_control() {
            return; // Control characters are handled elsewhere
        }

        let line = &mut text[*cursor_y];

        match c {
            '\u{8}' | '\r' | '\n' | '\t' => {
                // We also have to pre-terminate with these special characters,
                // since input is passed in a queue
                return; // Special characters will be handled elsewhere
            }

            _ => {
                let byte_idx = char_to_byte(line, *cursor_x);

                line.insert(byte_idx, c); // Normal insertion.
                *cursor_x += 1;
            }
        }
 
    }
}

pub fn draw(text: &Vec<String>, cursor_x: usize, cursor_y: usize) {
    let start_x = FILE_TEXT_X_MARGIN;
    let start_y = FILE_TEXT_Y_MARGIN;
    let font_size = 25.0;
    let line_spacing = font_size;

    let mut x;
    let mut y;

    for (line_index, line) in text.iter().enumerate() {
        x = start_x;
        y = start_y + line_index as f32 * line_spacing;

        // Split line into words and spaces
        for word in line.split_inclusive(|c: char| c.is_whitespace()) {
            let color = calibrate_string_color(word.trim());

            for ch in word.chars() {
                draw_text(&ch.to_string(), x, y, font_size, color);

                let char_width = measure_text(&ch.to_string(), None, font_size as u16, 1.0).width;
                x += char_width;
            }
        }

    }

    if cursor_y < text.len() {
        let line = &text[cursor_y];
        let cursor_text = &line[..cursor_x.min(line.len())];
        let text_before_cursor = measure_text(cursor_text, None, font_size as u16, 1.0);
        let cursor_x_pos = start_x + text_before_cursor.width;
        let cursor_y_pos = start_y + cursor_y as f32 * line_spacing;

        draw_line(
            cursor_x_pos,
            cursor_y_pos - font_size * 0.8,
            cursor_x_pos,
            cursor_y_pos + font_size * 0.2,
            1.5,
            WHITE,
        );
    }
}
