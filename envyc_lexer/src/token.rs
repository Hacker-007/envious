use envyc_source::snippet::Snippet;

#[derive(Debug)]
pub struct Token {
    snippet: Snippet,
    kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
}