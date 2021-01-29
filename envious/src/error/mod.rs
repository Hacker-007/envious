use crate::{lexer::token::TokenKind, semantic_analyzer::types::Type, span::Span};

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

    UnsupportedOperation {
        operation_span: Span,
        operands: Vec<(Span, Type)>,
    },
    TypeMismatch {
        span: Span,
        expected_type: Type,
        actual_type: Type,
    },
    ConflictingType {
        first_span: Span,
        first_type: Type,
        second_span: Span,
        second_type: Type,
    }
}

impl Error {
    pub fn report_error(&self) {
        // TODO: Use annotate-snippets crate to construct good looking error messages.
        println!("{:#?}", self);
    }
}
