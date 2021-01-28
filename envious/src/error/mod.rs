use crate::{lexer::token::TokenKind, span::Span};

#[derive(Debug)]
pub enum Error {
    IntegerOverflow(Span),
    FloatOverflow(Span),
    UnterminatedString(Span),
    UnrecognizedCharacter(Span),
    UnexpectedEndOfInput,

    ExpectedPrefixExpression {
        span: Span,
        found_kind: TokenKind,
    },
    ExpectedKind {
        span: Span,
        expected_kind: TokenKind,
        actual_kind: TokenKind,
    },
}

impl Error {
    pub fn report_error(&self) {
        // TODO: Use annotate-snippets crate to construct good looking error messages.
        println!("{:#?}", self);
    }
}
