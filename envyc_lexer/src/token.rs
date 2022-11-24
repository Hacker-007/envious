use envyc_source::snippet::Snippet;

#[derive(Debug)]
pub struct Token {
    snippet: Snippet,
    kind: TokenKind,
}

impl Token {
    pub fn new(snippet: Snippet, kind: TokenKind) -> Self {
        Self {
            snippet,
            kind,
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Int,
    Boolean(bool),
    Identifer,
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