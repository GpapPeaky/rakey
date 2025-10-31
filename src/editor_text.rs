// Editor text manipulation and rendering 
// using a regex pattern to match highlight 
// colouring.
//
// The editor's identation can be switched on/off
// through the console.

use macroquad::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::editor_audio::EditorAudio;

#[path = "editor_cursor.rs"]
mod editor_cursor;

pub struct EditorGeneralTextStylizer {
    pub font: Font,
    pub font_size: u16,
    pub color: Color
}

impl EditorGeneralTextStylizer {
    pub async fn new() -> EditorGeneralTextStylizer {
        EditorGeneralTextStylizer {
            font: load_ttf_font("assets/font/scp_reg.ttf").await.unwrap(),
            font_size: 25,
            color: WHITE
        }
    }

    fn draw(&self, text: &str, x: f32, y: f32){
        draw_text_ex(text, x, y,
            TextParams { font: Some(&self.font), font_size: self.font_size, color: self.color, ..Default::default() });
    }
}

// Regex pattern order matters: comments, strings, numbers, words, punctuation, whitespace
static TOKEN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|<[^>\n]+>|\b\d+(\.\d+)?([fF]\b)?\b|#[\w_]+|[\w\*]+|[^\w\s]+|\s+"#
    ).unwrap()
});

const FILE_TEXT_X_MARGIN: f32 = 50.0;
const FILE_TEXT_Y_MARGIN: f32 = 60.0;
const TAB_SIZE: usize = 6;
const TAB_PATTERN: &str = "      ";

pub const BACKGROUND_COLOR: Color     = Color::from_rgba(8, 0, 15, 255);        // Theater dark — emotional void
const IDENTIFIER_COLOR: Color         = Color::from_rgba(190, 140, 230, 255);   // Pale violet — fateful names
const PUNCTUATION_COLOR: Color        = Color::from_rgba(255, 255, 255, 255);   // White — clarity in despair
const CONTROL_FLOW_COLOR: Color       = Color::from_rgba(130, 100, 255, 255);   // Tragic blue — falling motion
const STORAGE_CLASS_COLOR: Color      = Color::from_rgba(255, 70, 110, 255);    // Crimson sorrow — bleeding intent
const TYPE_QUALIFIER_COLOR: Color     = Color::from_rgba(255, 210, 90, 255);    // Pale gold — faded grandeur
const COMPOSITE_TYPE_COLOR: Color     = Color::from_rgba(140, 0, 180, 255);     // Dark purple — structure of fate
const MISC_COLOR: Color               = Color::from_rgba(100, 130, 200, 255);   // Twilight blue — haunting echo
const DATA_TYPE_COLOR: Color          = Color::from_rgba(60, 190, 150, 255);    // Teal — fragile balance
const NUMBER_LITERAL_COLOR: Color     = Color::from_rgba(255, 235, 150, 255);   // Candle gold — memory counts
const STRING_LITERAL_COLOR: Color     = Color::from_rgba(255, 120, 170, 255);   // Mourning rose — spoken sorrow
const COMMENT_COLOR: Color            = Color::from_rgba(100, 90, 110, 255);    // Smoke gray — whispered lament
const CURSOR_COLOR: Color             = Color::from_rgba(255, 0, 130, 255);     // Magenta glow — pulse of pain
const MACRO_COLOR: Color              = Color::from_rgba(255, 110, 0, 255);     // Ember orange — spark of catharsis
const MAIN_COLOR: Color               = Color::from_rgba(180, 60, 255, 255);    // Regal violet — tragic beauty

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

pub fn record_special_keys(cursor_x: &mut usize, cursor_y: &mut usize, text: &mut Vec<String>, audio: &EditorAudio) -> bool {
    if is_key_pressed(KeyCode::Backspace) {
        audio.play_delete();

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
        audio.play_space();

        let line = &mut text[*cursor_y];
        let byte_idx = char_to_byte(line, *cursor_x);
        line.insert_str(byte_idx, TAB_PATTERN);
        *cursor_x += TAB_SIZE;
        return true;
    }

    if is_key_pressed(KeyCode::Enter) {
        audio.play_return();

        let line = &mut text[*cursor_y];
        let rest = line.split_off(char_to_byte(line, *cursor_x));
        *cursor_y += 1;

        // TODO: Smarter identation here

        *cursor_x = 0;
        
        text.insert(*cursor_y, rest);
        return true;
    }

    false
}

pub fn record_keyboard_to_file_text(cursor_x: &mut usize, cursor_y: &mut usize, text: &mut Vec<String>, audio: &EditorAudio) {
    // let c = get_char_pressed().unwrap(); // Unwrap removes the Result/Option wrapper.

    if text.is_empty() { // Allocate memory for a new string
        text.push(String::new());
    }

    if record_special_keys(cursor_x, cursor_y, text, audio) {
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

            '<' => {
                audio.play_insert();

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c);
                
                *cursor_x += 1;
                
                let next_byte_idx = char_to_byte(line, *cursor_x);

                line.insert(next_byte_idx, '>');
            }

            '(' => {
                audio.play_insert();

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c);
                
                *cursor_x += 1;
                
                let next_byte_idx = char_to_byte(line, *cursor_x);

                line.insert(next_byte_idx, ')');
            }

            '{' => {
                audio.play_insert();

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c);
                
                *cursor_x += 1;
                
                let next_byte_idx = char_to_byte(line, *cursor_x);

                line.insert(next_byte_idx, '}');
            }

            '\'' => {
                audio.play_insert();

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c);
                
                *cursor_x += 1;
                
                let next_byte_idx = char_to_byte(line, *cursor_x);

                line.insert(next_byte_idx, '\'');
            }

            '"' => {
                audio.play_insert();

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c);
                
                *cursor_x += 1;
                
                let next_byte_idx = char_to_byte(line, *cursor_x);

                line.insert(next_byte_idx, '"');
            }

            '[' => {
                audio.play_insert();

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c);
                
                *cursor_x += 1;
                
                let next_byte_idx = char_to_byte(line, *cursor_x);

                line.insert(next_byte_idx, ']');
            }

            _ => {
                if c != ' ' { 
                    audio.play_insert();
                } else {
                    audio.play_space();
                }

                let line = &mut text[*cursor_y];

                let byte_idx = char_to_byte(line, *cursor_x);
                
                line.insert(byte_idx, c); // Normal insertion.
                *cursor_x += 1;
            }
        }
 
    }
}

pub fn draw(text: &Vec<String>, cursor_x: usize, cursor_y: usize, gts: &mut EditorGeneralTextStylizer) {
    let start_x = FILE_TEXT_X_MARGIN;
    let start_y = FILE_TEXT_Y_MARGIN;
    let line_spacing = gts.font_size as f32;
    
    // Draw cursor
    if cursor_y < text.len() {
        let line = &text[cursor_y];
        let cursor_text = &line[..cursor_x.min(line.len())];
        let text_before_cursor = measure_text(cursor_text, Some(&gts.font), gts.font_size, 1.0);
        let cursor_x_pos = start_x + text_before_cursor.width;
        let cursor_y_pos = start_y + cursor_y as f32 * line_spacing;
        let cursor_width = 1.2;

        draw_line(
            cursor_x_pos,
            cursor_y_pos - gts.font_size as f32 * 0.8,
            cursor_x_pos,
            cursor_y_pos + gts.font_size as f32  * 0.2,
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
            } else if token.trim_start().starts_with("#") {
                MACRO_COLOR
            } else if (token.starts_with('"') && token.ends_with('"')) || (token.starts_with('<') && token.ends_with('>')) {
                STRING_LITERAL_COLOR
            } else if token.chars().all(|c| c.is_whitespace()) {
                IDENTIFIER_COLOR
            } else if token.chars().all(|c| !c.is_alphanumeric() && !c.is_whitespace() && c != '_') {
                PUNCTUATION_COLOR
            } else if TOKEN_PATTERN.is_match(token) && token.chars().any(|c| c.is_ascii_digit()) {
                NUMBER_LITERAL_COLOR
            } else if token == "main" {
                MAIN_COLOR
            } else {
                // Normal identifiers like variable names and functions
                let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                calibrate_string_color(clean)
            };

            // FIXME Negative number colouring with a '-' is colored as a punctuation
            // FIXME Strings inside (str) are not coloured properly.
            // FIXME Strings broken by newlines are not colored properly.
            // FIXME Macros when brocken by white space, not colored properly.
            // FIXME Numbers inside identifiers, get coloured as numbers

            // Draw token at once using the general text stylizer
            gts.color = color;
            gts.draw(token, x, y);
            // draw_text(token, x, y, gts.font_size, color);

            // More effective cursor movement
            // Avoid cursor x/y calibration per character
            let token_width = measure_text(token, Some(&gts.font), gts.font_size, 1.0).width;
            x += token_width;
        }
    }
}
