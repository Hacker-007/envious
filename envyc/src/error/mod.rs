use crate::{lexer::token::TokenKind, semantic_analyzer::types::Type};

/// Enum used by compiler to construct the various errors.
/// Every error needs to keep a track of the span of the error
/// to provide a better representation when reported to the user,
/// unless the error stems from the LLVM compiler, which is not
/// derived from the user's code.
#[derive(Debug)]
pub enum Error<'a> {
    // Occurs when an integer that exceeeds the maximum possible value of an integer.
    IntegerOverflow(Span<'a>),
    // Occurs when a float that exceeeds the maximum possible value of a float.
    FloatOverflow(Span<'a>),
    // Occurs when a string that has been started but not closed.
    UnterminatedString(Span<'a>),
    // Occurs when a character that is not recognized by the `Lexer`.
    UnrecognizedCharacter(Span<'a>),

    // Occurs when an expression was expected by the `Parser` but
    // there were no more tokens to inspect.
    UnexpectedEndOfInput(Span<'a>),
    // Occurs when a token does not have a corresponding expression.
    ExpectedPrefixExpression {
        span: Span<'a>,
        found_kind: TokenKind,
    },
    // Occurs when a certain token was expected by an expression but
    // a different token was found.
    ExpectedKind {
        span: Span<'a>,
        expected_kinds: Vec<TokenKind>,
        actual_kind: TokenKind,
    },

    // Occurs when the specified operation could not be applied to operands.
    UnsupportedOperation {
        operation_span: Span<'a>,
        operands: Vec<(Span<'a>, Type)>,
    },
    // // Occurs when the type of an expression does not match the expected type.
    TypeMismatch {
        span: Span<'a>,
        expected_type: Type,
        actual_type: Type,
    },
    // Occurs when the type of two branches do not match. For example, if the type
    // of the then branch and the type of the else branch do not match, this error
    // is returned.
    ConflictingType {
        first_span: Span<'a>,
        first_type: Type,
        second_span: Span<'a>,
        second_type: Type,
    },
    // Occurs when a type was found that could not be used.
    IllegalType(Span<'a>),
    UndefinedVariable(Span<'a>),

    UnknownFunction(Span<'a>),
    /// Occurs when a function was expected during the LLVM compilation.
    ExpectedFunction,
    LLVMFunctionFailure,
}

pub mod reporter;
pub mod span;
pub use span::Span;
