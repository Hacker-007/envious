use crate::span::Span;

pub type Token = (Span, TokenKind);

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,

    Void,
    Any,
    Int,
    Float,
    Boolean,
    String,
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(usize),
    Identifier(usize),

    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftAngleBracket,
    RightAngleBracket,
    Plus,
    Minus,
    Star,
    Slash,
    PercentSign,
    EqualSign,
    ColonEqualSign,
    ExclamationEqualSign,
    LessThanEqualSign,
    GreaterThanEqualSign,
    Comma,
    Colon,
    ColonColon,

    Not,
    Or,
    And,
    Let,
    If,
    Then,
    Else,
    While,
    Define,
}
