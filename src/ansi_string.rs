use crate::ansi_parser::AnsiParserState;

pub struct AnsiString {
    pub original: String,
    pub without_colors: String,
}

impl AnsiString {
    pub fn new(original_string: String) -> AnsiString {
        AnsiString {
            original: original_string.to_string(),
            without_colors: original_string
                .bytes()
                .fold(AnsiParserState::new(), |acc, b| acc.push_byte(b))
                .raw_text(),
        }
    }
}
