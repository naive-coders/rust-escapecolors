const ESCAPE_CHAR: u8 = 27;
const CSI_CHAR: u8 = 91;
const ZERO_CHAR: u8 = 48;
const NINE_CHAR: u8 = 57;
const CLOSING_CHAR: u8 = 109;
const PARAMETER_DELIMITER_CHAR: u8 = 59;

pub enum AnsiParserState {
    RawText(Vec<u8>),
    Fe(Vec<u8>, Vec<u8>),
    Csi(Vec<u8>, Vec<u8>),
    Color(Vec<u8>, Vec<u8>),
}

impl AnsiParserState {
    pub fn new() -> AnsiParserState {
        AnsiParserState::RawText(vec![])
    }

    pub fn raw_text(self) -> String {
        match self {
            Self::RawText(bytes) => String::from_utf8(bytes.to_vec()).unwrap(),
            Self::Fe(bytes, esc) | Self::Csi(bytes, esc) | Self::Color(bytes, esc) => {
                String::from_utf8(sum_buffers(bytes, esc)).unwrap()
            }
        }
    }

    pub fn push_byte(self, byte: u8) -> AnsiParserState {
        match self {
            Self::RawText(buf) => match byte {
                b if b == ESCAPE_CHAR => AnsiParserState::Fe(buf, vec![b]),
                _ => AnsiParserState::RawText(add_to_buf(buf, byte)),
            },
            Self::Fe(buf, esc) => match byte {
                b if b == CSI_CHAR => AnsiParserState::Csi(buf, add_to_buf(esc, b)),
                _ => reset_to_raw(buf, esc, byte),
            },
            Self::Csi(buf, esc) => match byte {
                b if (ZERO_CHAR..=NINE_CHAR).contains(&b) => {
                    AnsiParserState::Color(buf, add_to_buf(esc, b))
                }
                _ => reset_to_raw(buf, esc, byte),
            },
            Self::Color(buf, esc) => match byte {
                b if (ZERO_CHAR..NINE_CHAR).contains(&b) => {
                    AnsiParserState::Color(buf, add_to_buf(esc, b))
                }
                b if b == PARAMETER_DELIMITER_CHAR => {
                    AnsiParserState::Color(buf, add_to_buf(esc, b))
                }
                b if b == CLOSING_CHAR => AnsiParserState::RawText(buf),
                _ => reset_to_raw(buf, esc, byte),
            },
        }
    }
}

fn reset_to_raw(mut buf: Vec<u8>, mut current_esc: Vec<u8>, byte: u8) -> AnsiParserState {
    buf.append(&mut current_esc);
    buf.push(byte);
    AnsiParserState::RawText(buf)
}

fn add_to_buf(mut buf: Vec<u8>, byte: u8) -> Vec<u8> {
    buf.push(byte);
    buf
}

fn sum_buffers(buf: Vec<u8>, esc: Vec<u8>) -> Vec<u8> {
    let mut result = buf.to_vec();
    result.append(&mut esc.to_vec());
    result
}
