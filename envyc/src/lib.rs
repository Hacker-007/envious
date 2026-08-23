#![allow(unused)]

use crate::{context::CompilationContext, lex::{Lexer, token::buffer::TokenizedBuffer}, source::SourceId};

pub mod context;
pub(crate) mod dense;
pub mod diagnostics;
pub mod lex;
pub mod source;

pub fn lex(ctx: &mut CompilationContext, source: impl Into<Box<str>>) -> (SourceId, TokenizedBuffer) {
    let (id, source) = ctx.sources.register(source);
    let buffer = Lexer::new(source).lex(&mut ctx.diagnostics);
    (id, buffer)
}

#[cfg(test)]
mod test {
    use crate::{context::CompilationContext, lex};

    #[test]
   fn lex_ok() {
        let mut ctx = CompilationContext::default();
        let (_, buffer) = lex(&mut ctx, "123");
        assert!(ctx.diagnostics.is_empty());
        assert_eq!(buffer.kinds.len(), 2);
    }
}