use envyc_context::interner::InternId;
use envyc_source::snippet::Snippet;

#[derive(Debug)]
pub struct Token {
    snippet: Snippet,
    kind: TokenKind,
}

impl Token {
    pub fn new(snippet: Snippet, kind: TokenKind) -> Self {
        Self { snippet, kind }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Int(i64),
    Boolean(bool),
    Identifer(InternId),

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
    SemiColon,
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
    Return,
}
