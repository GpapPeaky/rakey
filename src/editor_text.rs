use macroquad::prelude::*;

#[path = "editor_cursor.rs"]
mod editor_cursor;

const ALPHA_VALUE: u8 = 255;
const FILE_TEXT_X_MARGIN: f32 = 10.0;
const FILE_TEXT_Y_MARGIN: f32 = 30.0;

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

pub fn record_keyboard_to_file_text(cursor_x: &mut usize, cursor_y: &mut usize, text: &mut Vec<String>) {
    // let c = get_char_pressed().unwrap(); // Unwrap removes the Result/Option wrapper.

    if let Some(c) = get_char_pressed() {
        // We will also handle smart/smarter identation here.
        while *cursor_y >= text.len() {
            text.push(String::new());
        }

        let line = &mut text[*cursor_y];

        match c {
            _ => {
                line.insert(*cursor_x, c); // Normal insertion.
                *cursor_x += 1;
            }
        }
 
    }
}

pub fn draw(text: &Vec<String>, cursor_x: usize, cursor_y: usize) {
    let start_x = FILE_TEXT_X_MARGIN;
    let start_y = FILE_TEXT_Y_MARGIN;
    let font_size = 25.0;
    let line_spacing = font_size * 1.2;

    for (line_index, line) in text.iter().enumerate() {
        let mut x = start_x;
        let y = start_y + line_index as f32 * line_spacing;

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
}
