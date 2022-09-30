use crate::compiler::compile_unit::CompileUnit;
use crate::error::CompilerError;
use crate::{
    error::CompilerErrorKind,
    lexical_analysis::{
        token::{Token, TokenKind},
        Lexer,
    },
};

pub(crate) struct LexerAsserter<'file, 'unit> {
    lexer: Lexer<'file, 'unit>,
}

impl<'file, 'unit> LexerAsserter<'file, 'unit> {
    pub fn new(compile_unit: &'unit CompileUnit<'file>) -> Self {
        Self {
            lexer: Lexer::new(compile_unit),
        }
    }

    pub fn assert_token(&mut self, expected_kind: TokenKind) {
        let token = self.lexer.next();
        match token {
            None => panic!(
                "Expected to find {:?}, but found a {:?} instead.",
                expected_kind,
                TokenKind::EndOfFile
            ),
            Some(Ok(Token {
                kind: found_kind, ..
            })) if found_kind != expected_kind => panic!(
                "Expected to find {:?}, but found a {:?} instead.",
                expected_kind, found_kind
            ),
            Some(Err(CompilerError {
                kind: error_kind, ..
            })) => panic!(
                "Expected to find {:?}, but got a {:?} instead.",
                expected_kind, error_kind
            ),
            _ => {}
        }
    }

    pub fn assert_error(&mut self, expected_error: CompilerErrorKind) {
        let token = self.lexer.next();
        match token {
            None => panic!(
                "Expected to find {:?}, but found a {:?} instead.",
                expected_error,
                TokenKind::EndOfFile
            ),
            Some(Ok(Token {
                kind: found_token_kind,
                ..
            })) => panic!(
                "Expected to find {:?}, but found a {:?} instead.",
                expected_error, found_token_kind
            ),
            Some(Err(CompilerError {
                kind: found_error_kind,
                ..
            })) if found_error_kind != expected_error => panic!(
                "Expected to find {:?}, but got a {:?} instead.",
                expected_error, found_error_kind
            ),
            _ => {}
        }
    }
}
