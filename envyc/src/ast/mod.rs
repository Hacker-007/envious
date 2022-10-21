use std::iter::Peekable;

use crate::{
    compiler::{compile_session::CompileSession, compile_unit::CompileUnit},
    error::{CompilerError, CompilerErrorKind},
    lexical_analysis::{
        token::{Token, TokenKind},
        Lexer,
    },
    location::Location,
};

pub(crate) mod expression;
pub(crate) mod p;
pub(crate) mod statement;

pub(crate) struct Parser<'session, 'file, 'unit> {
    compile_session: &'session mut CompileSession,
    compile_unit: &'unit CompileUnit<'file>,
    lexer: Peekable<Lexer<'session, 'file, 'unit>>,
    last_seen_location: Location,
}

impl<'session, 'file, 'unit> Parser<'session, 'file, 'unit> {
    pub fn new(
        compile_session: &'session mut CompileSession,
        compile_unit: &'unit CompileUnit<'file>,
    ) -> Self {
        Self {
            compile_session,
            compile_unit,
            lexer: Lexer::new(compile_session, compile_unit, true).peekable(),
            last_seen_location: Location::new(0, 0),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, CompilerError> {
        match self.next() {
            Some(token) => Ok(token),
            None => Err(CompilerError::new(
                CompilerErrorKind::UnrecognizedCharacter,
                self.last_seen_location,
            )),
        }
    }

    fn next(&mut self) -> Option<Token> {
        while let Some(token) = self.lexer.next() {
            match token {
                Ok(token) => return Some(token),
                Err(error) => {
                    self.compile_session.emit_error(error);
                }
            }
        }

        None
    }
}
