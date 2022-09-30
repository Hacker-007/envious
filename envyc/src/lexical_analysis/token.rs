use crate::location::Location;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) location: Location,
}

impl Token {
    pub fn new(kind: TokenKind, location: Location) -> Self {
        Self { kind, location }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TokenKind {
    Whitespace(char),
	Plus,
	Minus,
	Star,
	ForwardSlash,
    EndOfFile,
}
