use crate::{
    lex::{buffer::TokenizedBuffer, token::writer::TokenWriter},
    source::Source,
};

pub mod buffer;
pub mod token;

#[derive(Debug, Clone, Copy)]
enum LexMode {
    /// We are not within any special context and
    /// tokenize the source buffer as normal.
    Standard,
    /// We are inside a string fragment, starting
    /// and ending with a double quote (i.e. `"`).
    InString,
    /// We are inside an interpolated expression
    /// within a string, starting with `${` and
    /// ending with `}`.
    InStringInterpolation,
}

#[derive(Debug)]
pub struct Lexer<'a, 'src> {
    idx: usize,
    source: &'a mut Source<'src>,
    current_mode: LexMode,
    /// A stack of all previous modes the
    /// lexer was in to support nested lexing.
    prior_modes: Vec<LexMode>,
    buffer: TokenizedBuffer,
}

impl<'a, 'src> Lexer<'a, 'src> {
    pub fn new(source: &'a mut Source<'src>) -> Self {
        Self {
            idx: 0,
            source,
            current_mode: LexMode::Standard,
            prior_modes: vec![],
            buffer: TokenizedBuffer::default(),
        }
    }

    pub fn lex(mut self) -> TokenizedBuffer {
        while !self.at_end() {
            match self.current_mode {
                LexMode::Standard => self.lex_standard(),
                LexMode::InString => todo!(),
                LexMode::InStringInterpolation => todo!(),
            }
        }

        self.buffer
    }

    fn lex_standard(&mut self) {
        let start = self.idx;
        let byte = self.current();
        self.advance();
        match byte {
            b' ' | b'\t' | b'\x0C' | b'\r' => self.writer_at(start).write_whitespace(),
            b'\n' => self.writer_at(start).write_newline(),
            _ => {}
        }
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.idx >= self.source.len()
    }

    #[inline]
    fn current(&self) -> u8 {
        self.source.get(self.idx).unwrap()
    }

    #[inline]
    fn advance(&mut self) {
        self.idx += 1;
    }

    #[inline]
    fn writer_at(&mut self, start_position: usize) -> TokenWriter<'_, 'src, '_> {
        TokenWriter::new(self.source, &mut self.buffer, start_position as u32)
    }
}
