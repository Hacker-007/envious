use crate::{dense::DenseVec, lex::token::TokenInfo};

/// The index into the [`TokenizedBuffer`] for a
/// given token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenIndex(u32);

/// A buffer of tokenized source code.
///
/// The buffer maintains individual value vectors for all lexed
/// entities and provides lightweight handles to each token.
#[derive(Debug, Clone, Default)]
pub struct TokenizedBuffer {
    pub(crate) tokens: Vec<TokenInfo>,
    pub(crate) start_positions: Vec<u32>,

    pub(crate) ints: DenseVec<i64>,
}
