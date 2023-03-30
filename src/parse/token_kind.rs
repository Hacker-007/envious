use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,

    Int,
    Boolean,
    Identifer,

    IntType,
    BooleanType,

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
    Period,
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

    EndOfFile,

    Dummy,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_representation = match self {
            TokenKind::Whitespace => "a whitespace",
            TokenKind::Int => "an integer",
            TokenKind::Boolean => "a boolean",
            TokenKind::Identifer => "an identifier",
            TokenKind::IntType => "the integer type",
            TokenKind::BooleanType => "the boolean type",
            TokenKind::LeftParenthesis => "a left parenthesis",
            TokenKind::RightParenthesis => "a right parenthesis",
            TokenKind::LeftCurlyBrace => "a left curly bracket",
            TokenKind::RightCurlyBrace => "a right curly bracket",
            TokenKind::LeftAngleBracket => "a left angle bracket",
            TokenKind::RightAngleBracket => "a right curly bracket",
            TokenKind::Plus => "a plus sign",
            TokenKind::Minus => "a minus sign",
            TokenKind::Star => "a star sign",
            TokenKind::Slash => "a slash sign",
            TokenKind::Percent => "a percentage sign",
            TokenKind::Equal => "an equal sign",
            TokenKind::ColonEqual => "a colon equal sign",
            TokenKind::ExclamationEqual => "an exclamation equal sign",
            TokenKind::LessThanEqual => "a less than equal sign",
            TokenKind::GreaterThanEqual => "a greater than equal sign",
            TokenKind::Period => "a period",
            TokenKind::Comma => "a comma",
            TokenKind::Colon => "a colon",
            TokenKind::SemiColon => "a semi-colon",
            TokenKind::ColonColon => "a double colon sign",
            TokenKind::Not => "the not keyword",
            TokenKind::Or => "the or keyword",
            TokenKind::And => "the and keyword",
            TokenKind::Let => "the let keyword",
            TokenKind::If => "the if keyword",
            TokenKind::Then => "the then keyword",
            TokenKind::Else => "the else keyword",
            TokenKind::While => "the while keyword",
            TokenKind::Define => "the define keyword",
            TokenKind::Return => "the return keyword",
            TokenKind::EndOfFile => "the end of the file",
            TokenKind::Dummy => unreachable!("tried to use dummy token!"),
        };

        f.write_str(token_representation)
    }
}
