use crate::span::Span;

pub type Token = (Span, TokenKind);

#[derive(Debug)]
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
    StringLiteral(String),
    Identifier(String),

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
