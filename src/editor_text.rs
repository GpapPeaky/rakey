use macroquad::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

#[path = "editor_cursor.rs"]
mod editor_cursor;

// Regex pattern order matters: comments, strings, numbers, words, punctuation, whitespace
static TOKEN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|\b\d+(?:\.\d+)?\b|[\w\*]+|[^\w\s]+|\s+"#)
        .unwrap()
});

const FILE_TEXT_X_MARGIN: f32 = 50.0;
const FILE_TEXT_Y_MARGIN: f32 = 60.0;
const TAB_SIZE: usize = 5;
const TAB_PATTERN: &str = "     ";

pub const BACKGROUND_COLOR: Color     = Color::from_rgba(27, 36, 33, 255);     // Deep olive green — shade of olive leaves in shadow
const IDENTIFIER_COLOR: Color         = Color::from_rgba(226, 186, 120, 255);  // Wheat gold — sunlit stone & dry grass
const PUNCTUATION_COLOR: Color        = Color::from_rgba(140, 130, 115, 255);  // Weathered limestone gray
const CONTROL_FLOW_COLOR: Color       = Color::from_rgba(61, 130, 191, 255);   // Aegean blue — sea near Chania
const STORAGE_CLASS_COLOR: Color      = Color::from_rgba(108, 174, 186, 255);  // Turquoise — shallow coastal waters
const TYPE_QUALIFIER_COLOR: Color     = Color::from_rgba(197, 165, 103, 255);  // Olive-gold — ripe olive tone
const COMPOSITE_TYPE_COLOR: Color     = Color::from_rgba(177, 87, 52, 255);    // Terracotta red — Cretan pottery & soil
const MISC_COLOR: Color               = Color::from_rgba(205, 142, 79, 255);   // Honey amber — warm neutral accent
const DATA_TYPE_COLOR: Color          = Color::from_rgba(93, 166, 131, 255);   // Sage green — mountain herbs
const NUMBER_LITERAL_COLOR: Color     = Color::from_rgba(147, 200, 120, 255);  // Leaf green — olive groves
const STRING_LITERAL_COLOR: Color     = Color::from_rgba(185, 228, 255, 255);  // Pale sky blue — summer light
const COMMENT_COLOR: Color            = Color::from_rgba(0, 255, 40, 255);     // Green
const CURSOR_COLOR: Color             = Color::from_rgba(255, 243, 204, 255);  // Sunlight beige — stands out gently on olive background

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

fn calibrate_string_color(string: &str) -> Color {
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
    } else if string.chars().all(|c| c.is_ascii_digit()) {
        return NUMBER_LITERAL_COLOR;
    } else {
        return IDENTIFIER_COLOR;
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
    
    // Draw cursor
    if cursor_y < text.len() {
        let line = &text[cursor_y];
        let cursor_text = &line[..cursor_x.min(line.len())];
        let text_before_cursor = measure_text(cursor_text, None, font_size as u16, 1.0);
        let cursor_x_pos = start_x + text_before_cursor.width;
        let cursor_y_pos = start_y + cursor_y as f32 * line_spacing;
        let cursor_width = 4.0;

        draw_line(
            cursor_x_pos + cursor_width / 5.0,
            cursor_y_pos - font_size * 0.8,
            cursor_x_pos + cursor_width / 5.0,
            cursor_y_pos + font_size * 0.2,
            cursor_width,
            CURSOR_COLOR,
        );
    }

    let mut x;
    let mut y;    

    for (line_index, line) in text.iter().enumerate() {
        x = start_x;
        y = start_y + line_index as f32 * line_spacing;

        for cap in TOKEN_PATTERN.find_iter(line) {
            let token = cap.as_str();

            let color = if token.starts_with("//") || token.starts_with("/*") {
                COMMENT_COLOR
            } else if token.starts_with('"') && token.ends_with('"') {
                STRING_LITERAL_COLOR
            } else if token.chars().all(|c| c.is_whitespace()) {
                IDENTIFIER_COLOR
            } else if token.chars().all(|c| !c.is_alphanumeric() && !c.is_whitespace() && c != '_') {
                PUNCTUATION_COLOR
            } else if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
                NUMBER_LITERAL_COLOR
            } else {
                // Normal identifiers/keywords
                let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                calibrate_string_color(clean)
            };

            // Draw token at once
            draw_text(token, x, y, font_size, color);

            // More effective cursor movement
            // Avoid cursor x/y calibration per character
            let token_width = measure_text(token, None, font_size as u16, 1.0).width;
            x += token_width;
        }
    }
}
