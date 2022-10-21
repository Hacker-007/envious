pub(crate) mod token;

use crate::compiler::compile_session::CompileSession;
use crate::compiler::compile_unit::CompileUnit;
use crate::error::{CompilerError, CompilerErrorKind};
use crate::lexical_analysis::token::{Token, TokenKind};
use crate::location::Location;

pub(crate) struct Lexer<'session, 'file, 'unit> {
    compile_session: &'session mut CompileSession,
    compile_unit: &'unit CompileUnit<'file>,
    index: usize,
    ignore_whitespace: bool,
}

fn form_token(kind: TokenKind, start: usize, length: usize) -> Result<Token, CompilerError> {
    Ok(Token::new(kind, Location::new(start, start + length)))
}

impl<'session, 'file, 'unit> Lexer<'session, 'file, 'unit> {
    pub fn new(
        compile_session: &'session mut CompileSession,
        compile_unit: &'unit CompileUnit<'file>,
        ignore_whitespace: bool,
    ) -> Self {
        Self {
            compile_session,
            compile_unit,
            index: 0,
            ignore_whitespace,
        }
    }

    fn next_token(&mut self) -> Result<Token, CompilerError> {
        if self.ignore_whitespace {
            while let Some(byte) = self.peek() {
                if byte.is_ascii_whitespace() {
                    self.next();
                } else {
                    break;
                }
            }
        }

        if let Some((start_index, byte)) = self.next() {
            match byte {
                whitespace if whitespace.is_ascii_whitespace() => {
                    form_token(TokenKind::Whitespace(whitespace as char), start_index, 1)
                }
                digit if digit.is_ascii_digit() => {
                    self.tokenize_number((digit - b'0').into(), start_index)
                }
                b'+' => form_token(TokenKind::Plus, start_index, 1),
                b'-' => form_token(TokenKind::Minus, start_index, 1),
                b'*' => form_token(TokenKind::Star, start_index, 1),
                b'/' => form_token(TokenKind::ForwardSlash, start_index, 1),
                _ => Err(CompilerError::new(
                    CompilerErrorKind::UnrecognizedCharacter,
                    Location::new(start_index, start_index + 1),
                )),
            }
        } else {
            form_token(TokenKind::EndOfFile, self.index - 1, 1)
        }
    }

    fn tokenize_number(
        &mut self,
        first_digit: i64,
        start_index: usize,
    ) -> Result<Token, CompilerError> {
        let mut number = first_digit;
        while let Some(next_digit) = self.peek() {
            if next_digit.is_ascii_digit() {
                self.next();
                match number.checked_mul(10) {
                    Some(expanded_num) => {
                        let digit_value: i64 = (next_digit - b'0').into();
                        number = expanded_num + digit_value;
                    }
                    None => {
                        return Err(CompilerError::new(
                            CompilerErrorKind::IntegerOverflow,
                            Location::new(start_index, self.index),
                        ))
                    }
                }
            } else {
                break;
            }
        }

        form_token(
            TokenKind::Integer(number),
            start_index,
            self.index - start_index,
        )
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

impl<'session, 'file, 'unit> Iterator for Lexer<'session, 'file, 'unit> {
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
