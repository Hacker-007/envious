use crate::{
    dense::{DenseIndex, DenseRange, DenseVec},
    lex::token::{Token, TokenKind, Trivia},
    source::Span,
};

/// The index into the [`TokenizedBuffer`] for a
/// given token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenIndex(DenseIndex);

/// A buffer of tokenized source code.
#[derive(Debug, Clone, Default)]
pub struct TokenizedBuffer {
    pub(crate) kinds: DenseVec<TokenKind>,
    leading: DenseVec<DenseRange>,
    trailing: DenseVec<DenseRange>,

    spans: DenseVec<Span>,
    trivia: DenseVec<Trivia>,
}

impl TokenizedBuffer {
    pub fn push(&mut self, token: Token) -> TokenIndex {
        let idx = self.kinds.push(token.kind);
        self.spans.push(token.span);

        let leading_range = self.trivia.push_all(token.leading);
        let trailing_range = self.trivia.push_all(token.trailing);
        self.leading.push(leading_range);
        self.trailing.push(trailing_range);
        TokenIndex(idx)
    }
}
