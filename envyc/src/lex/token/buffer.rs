use std::ops::{Deref, DerefMut};

use crate::{
    dense::{DenseIndex, DenseRange, DenseVec},
    lex::token::{Token, TokenKind, Trivia},
    source::Span,
};

/// The index into the [`TokenizedBuffer`] for a
/// given token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenIndex(DenseIndex);

impl TokenIndex {
    pub const ZERO: Self = Self(DenseIndex::ZERO);

    pub fn advance(self, delta: u32) -> Self {
        Self(self.0.saturating_add(delta))
    }
}

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

    pub fn kind_at(&self, idx: TokenIndex) -> Option<TokenKind> {
        self.kinds.get(idx.0).copied()
    }

    pub fn span_at(&self, idx: TokenIndex) -> Option<Span> {
        self.spans.get(idx.0).copied()
    }
}
