use std::fmt::Display;

use envyc_context::symbol::Symbol;
use envyc_source::snippet::Snippet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub snippet: Snippet,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(snippet: Snippet, kind: TokenKind) -> Self {
        Self { snippet, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    EndOfFile,

    Int(i64),
    Boolean(bool),
    Identifer(Symbol),

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
    Percent,
    Equal,
    ColonEqual,
    ExclamationEqual,
    LessThanEqual,
    GreaterThanEqual,
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::EndOfFile => write!(f, "end of file"),
            TokenKind::Int(_) => write!(f, "int"),
            TokenKind::Boolean(_) => write!(f, "boolean"),
            TokenKind::Identifer(_) => write!(f, "identifier"),
            TokenKind::LeftParenthesis => write!(f, "left parenthesis"),
            TokenKind::RightParenthesis => write!(f, "right parenthesis"),
            TokenKind::LeftCurlyBrace => write!(f, "left curly bracket"),
            TokenKind::RightCurlyBrace => write!(f, "right curly bracket"),
            TokenKind::LeftAngleBracket => write!(f, "left angle bracket"),
            TokenKind::RightAngleBracket => write!(f, "right curly bracket"),
            TokenKind::Plus => write!(f, "plus sign"),
            TokenKind::Minus => write!(f, "minus sign"),
            TokenKind::Star => write!(f, "star sign"),
            TokenKind::Slash => write!(f, "slash sign"),
            TokenKind::Percent => write!(f, "percentage sign"),
            TokenKind::Equal => write!(f, "equal sign"),
            TokenKind::ColonEqual => write!(f, "colon equal sign"),
            TokenKind::ExclamationEqual => write!(f, "exclamation equal sign"),
            TokenKind::LessThanEqual => write!(f, "less than equal sign"),
            TokenKind::GreaterThanEqual => write!(f, "greater than equal sign"),
            TokenKind::Comma => write!(f, "comma"),
            TokenKind::Colon => write!(f, "colon"),
            TokenKind::SemiColon => write!(f, "semi-colon"),
            TokenKind::ColonColon => write!(f, "double colon sign"),
            TokenKind::Not => write!(f, "not keyword"),
            TokenKind::Or => write!(f, "or keyword"),
            TokenKind::And => write!(f, "and keyword"),
            TokenKind::Let => write!(f, "let keyword"),
            TokenKind::If => write!(f, "if keyword"),
            TokenKind::Then => write!(f, "then keyword"),
            TokenKind::Else => write!(f, "else keyword"),
            TokenKind::While => write!(f, "while keyword"),
            TokenKind::Define => write!(f, "define keyword"),
            TokenKind::Return => write!(f, "return keyword"),
        }
    }
}
