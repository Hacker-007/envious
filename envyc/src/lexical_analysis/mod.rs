pub(crate) mod token;

use crate::compiler::compile_unit::CompileUnit;
use crate::error::CompilerError;
use crate::lexical_analysis::token::{Token, TokenKind};
use crate::location::Location;

pub(crate) struct Lexer<'file, 'unit> {
    compile_unit: &'unit CompileUnit<'file>,
    index: usize,
}

fn create_token(kind: TokenKind, start: usize, length: usize) -> Result<Token, CompilerError> {
    Ok(Token::new(kind, Location::new(start, start + length)))
}

impl<'file, 'unit> Lexer<'file, 'unit> {
    pub fn new(compile_unit: &'unit CompileUnit<'file>) -> Self {
        Self {
            compile_unit,
            index: 0,
        }
    }

    fn next_token(&mut self) -> Result<Token, CompilerError> {
        if let Some((start_index, byte)) = self.next() {
            match byte {
                whitespace if whitespace.is_ascii_whitespace() => {
                    create_token(TokenKind::Whitespace(whitespace as char), start_index, 1)
                }
                b'+' => create_token(TokenKind::Plus, start_index, 1),
				b'-' => create_token(TokenKind::Minus, start_index, 1),
				b'*' => create_token(TokenKind::Star, start_index, 1),
				b'/' => create_token(TokenKind::ForwardSlash, start_index, 1),
                _ => Err(CompilerError::UnrecognizedCharacter(Location::new(
                    start_index,
                    start_index + 1,
                ))),
            }
        } else {
            create_token(TokenKind::EndOfFile, self.index - 1, 1)
        }
    }

    fn peek(&self) -> Option<u8> {
        self.compile_unit.file_contents.get(self.index).copied()
    }

    fn next(&mut self) -> Option<(usize, u8)> {
        self.index += 1;
        self.compile_unit
            .file_contents
            .get(self.index - 1)
            .copied()
            .map(|byte| (self.index - 1, byte))
    }
}

impl<'file, 'unit> Iterator for Lexer<'file, 'unit> {
    type Item = Result<Token, CompilerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token {
                kind: TokenKind::EndOfFile,
                ..
            }) => None,
            token => Some(token),
        }
    }
}
