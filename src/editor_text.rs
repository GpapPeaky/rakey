use macroquad::prelude::*;
use regex::Regex;

#[path = "editor_cursor.rs"]
mod editor_cursor;

const FILE_TEXT_X_MARGIN: f32 = 50.0;
const FILE_TEXT_Y_MARGIN: f32 = 60.0;
const TAB_SIZE: usize = 5;
const TAB_PATTERN: &str = "     ";

const DEFAULT_TEXT_COLOR: Color = WHITE;
const PUNCTUATION_COLOR: Color = YELLOW;

const CONTROL_FLOW_COLOR: Color = PINK;
const STORAGE_CLASS_COLOR: Color = BLUE;
const TYPE_QUALIFIER_COLOR: Color = MAGENTA;
const COMPOSITE_TYPE_COLOR: Color = RED;
const MISC_COLOR: Color = PURPLE;
const DATA_TYPE_COLOR: Color = ORANGE;

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
        return CONTROL_FLOW_COLOR;
    } else if C_TYPE_QUALIFIERS.contains(&string) {
        return TYPE_QUALIFIER_COLOR;
    } else if C_COMPOSITE_TYPES.contains(&string) {
        return COMPOSITE_TYPE_COLOR;
    } else if C_STORAGE_CLASS_SPECIFIERS.contains(&string) {
        return STORAGE_CLASS_COLOR;
    } else if C_MISC.contains(&string) {
        return MISC_COLOR;
    } else if C_DATA_TYPES.contains(&string) {
        return DATA_TYPE_COLOR;
    } else {
        return DEFAULT_TEXT_COLOR;
    }
}

pub fn record_special_keys(cursor_x: &mut usize, cursor_y: &mut usize, text: &mut Vec<String>) -> bool {
    if is_key_pressed(KeyCode::Backspace) {
        if text.is_empty() {
            return true;
        }
    
        // Clamp cursor_x to line length
        let line = &mut text[*cursor_y];
        let line_len = line.chars().count();
        *cursor_x = (*cursor_x).min(line_len);
    
        if *cursor_x == 0 {
            // Merge with previous line if possible
            if *cursor_y > 0 {
                let current_line = text.remove(*cursor_y);
                *cursor_y -= 1;
                *cursor_x = text[*cursor_y].chars().count();
                text[*cursor_y].push_str(&current_line);
            }
            return true;
        }
    
        let cursor_pos = *cursor_x;
    
        // Tab deletion
        if cursor_pos >= TAB_SIZE {
            let start_char = cursor_pos - TAB_SIZE;
            let end_char = cursor_pos;
            let start_byte = char_to_byte(line, start_char);
            let end_byte = char_to_byte(line, end_char);
    
            if &line[start_byte..end_byte] == TAB_PATTERN {
                line.replace_range(start_byte..end_byte, "");
                *cursor_x -= TAB_SIZE;
                return true;
            }
        }
    
        // Normal deletion
        let byte_idx = char_to_byte(line, cursor_pos - 1);
        if byte_idx < line.len() {
            line.remove(byte_idx);
            *cursor_x -= 1;
        }
    
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
        let line = &mut text[*cursor_y];
        let rest = line.split_off(char_to_byte(line, *cursor_x));
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
        match c {
            '\u{8}' | '\r' | '\n' | '\t' => {
                // We also have to pre-terminate with these special characters,
                // since input is passed in a queue
                return; // Special characters will be handled elsewhere
            }

            _ => {
                let line = &mut text[*cursor_y];

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

    let pattern = Regex::new(r"[\w\*]+|[^\w\s]+|\s+").unwrap(); 

    for (line_index, line) in text.iter().enumerate() {
        x = start_x;
        y = start_y + line_index as f32 * line_spacing;

        // Match to the pattern
        for word in pattern.find_iter(line) {
            let token = word.as_str();

            // Word cleanup and color calibration
            let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
            let color = calibrate_string_color(clean);

            for ch in token.chars() {
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

        let cursor_width = 7.0;

        draw_line(
            cursor_x_pos + cursor_width / 5.0,
            cursor_y_pos - font_size * 0.8,
            cursor_x_pos + cursor_width / 5.0,
            cursor_y_pos + font_size * 0.2,
            cursor_width,
            DEFAULT_TEXT_COLOR,
        );
    }
}
