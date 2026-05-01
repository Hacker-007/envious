use crate::{lex::{
    buffer::TokenizedBuffer,
    token::{TokenInfo, TokenKind},
}, source::Source};

/// The payload used for tokens that need no additional
/// data, such as whitespace and end-of-file tokens.
const EMPTY_PAYLOAD: u32 = 0;

#[derive(Debug)]
pub struct TokenWriter<'a, 'src, 'buffer> {
    source: &'a mut Source<'src>,
    buffer: &'buffer mut TokenizedBuffer,
    start_position: u32,
}

impl<'a, 'src, 'buffer> TokenWriter<'a, 'src, 'buffer> {
    pub fn new(source: &'a mut Source<'src>, buffer: &'buffer mut TokenizedBuffer, start_position: u32) -> Self {
        Self {
            source,
            buffer,
            start_position,
        }
    }

    pub fn write_whitespace(self) {
        let info = TokenInfo::new(TokenKind::Whitespace, EMPTY_PAYLOAD);
        self.write(info);
    }

    pub fn write_newline(self) {
        let info = TokenInfo::new(TokenKind::Whitespace, EMPTY_PAYLOAD);
        self.source.record_newline(self.start_position);
        self.write(info);
    }

    pub fn write_int(self, value: i64) {
        let idx = self.buffer.ints.push(value);
        let info = TokenInfo::new(TokenKind::IntLiteral, idx);
        self.write(info);
    }

    pub fn write_eof(self) {
        let info = TokenInfo::new(TokenKind::EndOfFile, EMPTY_PAYLOAD);
        self.write(info);
    }

    fn write(self, info: TokenInfo) {
        self.buffer.tokens.push(info);
        self.buffer.start_positions.push(self.start_position);
    }
}
