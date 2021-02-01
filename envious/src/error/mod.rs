use crate::{lexer::token::TokenKind, semantic_analyzer::types::Type};

/// Enum used by compiler to construct the various errors.
/// Every error needs to keep a track of the span of the error
/// to provide a better representation when reported to the user,
/// unless the error stems from the LLVM compiler, which is not
/// derived from the user's code.
#[derive(Debug)]
pub enum Error {
    // Occurs when an integer that exceeeds the maximum possible value of an integer.
    IntegerOverflow(Span),
    // Occurs when a float that exceeeds the maximum possible value of a float.
    FloatOverflow(Span),
    // Occurs when a string that has been started but not closed.
    UnterminatedString(Span),
    // Occurs when a character that is not recognized by the `Lexer`.
    UnrecognizedCharacter(Span),

    // Occurs when an expression was expected by the `Parser` but
    // there were no more tokens to inspect.
    UnexpectedEndOfInput,
    // Occurs when a token does not have a corresponding expression.
    ExpectedPrefixExpression {
        span: Span,
        found_kind: TokenKind,
    },
    // Occurs when a certain token was expected by an expression but
    // a different token was found.
    ExpectedKind {
        span: Span,
        expected_kind: TokenKind,
        actual_kind: TokenKind,
    },

    // Occurs when the specified operation could not be applied to operands.
    UnsupportedOperation {
        operation_span: Span,
        operands: Vec<(Span, Type)>,
    },
    // Occurs when the type of an expression does not match the expected type.
    TypeMismatch {
        span: Span,
        expected_type: Type,
        actual_type: Type,
    },
    // Occurs when the type of two branches do not match. For example, if the type
    // of the then branch and the type of the else branch do not match, this error
    // is returned.
    ConflictingType {
        first_span: Span,
        first_type: Type,
        second_span: Span,
        second_type: Type,
    },
    // Occurs when a cast could not be performed between the original type and the new
    // type.
    IllegalCast {
        span: Span,
        from_type: Type,
        to_type: Type,
    },

    /// Occurs when a function was expected during the LLVM compilation.
    ExpectedFunction,
}

impl Error {
    /// Reports the error to the user. Note that this method does not consume the error.
    /// This allows errors to be reported in many different places.
    pub fn report_error(&self) {
        // TODO: Use annotate-snippets crate to construct good looking error messages.
        println!("{:#?}", self);
    }
}

pub mod span;
pub use span::Span;
