use crate::location::Location;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) location: Location,
}

impl Token {
    pub fn new(kind: TokenKind, location: Location) -> Self {
        Self { kind, location }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum TokenKind {
    Whitespace(char),
    Integer(i64),
    Plus,
    Minus,
    Star,
    ForwardSlash,
    EndOfFile,
}
