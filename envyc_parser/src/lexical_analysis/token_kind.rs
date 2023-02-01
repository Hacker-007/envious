use std::fmt::Display;

use envyc_source::span::Spanned;

pub type Token = Spanned<TokenKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,

    Int,
    Boolean,
    Identifer,

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

    Error,
    EndOfFile,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_representation = match self {
            TokenKind::Whitespace => "whitespace",
            TokenKind::Int => "int",
            TokenKind::Boolean => "boolean",
            TokenKind::Identifer => "identifier",
            TokenKind::LeftParenthesis => "left parenthesis",
            TokenKind::RightParenthesis => "right parenthesis",
            TokenKind::LeftCurlyBrace => "left curly bracket",
            TokenKind::RightCurlyBrace => "right curly bracket",
            TokenKind::LeftAngleBracket => "left angle bracket",
            TokenKind::RightAngleBracket => "right curly bracket",
            TokenKind::Plus => "plus sign",
            TokenKind::Minus => "minus sign",
            TokenKind::Star => "star sign",
            TokenKind::Slash => "slash sign",
            TokenKind::Percent => "percentage sign",
            TokenKind::Equal => "equal sign",
            TokenKind::ColonEqual => "colon equal sign",
            TokenKind::ExclamationEqual => "exclamation equal sign",
            TokenKind::LessThanEqual => "less than equal sign",
            TokenKind::GreaterThanEqual => "greater than equal sign",
            TokenKind::Comma => "comma",
            TokenKind::Colon => "colon",
            TokenKind::SemiColon => "semi-colon",
            TokenKind::ColonColon => "double colon sign",
            TokenKind::Not => "not keyword",
            TokenKind::Or => "or keyword",
            TokenKind::And => "and keyword",
            TokenKind::Let => "let keyword",
            TokenKind::If => "if keyword",
            TokenKind::Then => "then keyword",
            TokenKind::Else => "else keyword",
            TokenKind::While => "while keyword",
            TokenKind::Define => "define keyword",
            TokenKind::Return => "return keyword",
            TokenKind::Error => "error",
            TokenKind::EndOfFile => "end of file",
        };

        f.write_str(token_representation)
    }
}
