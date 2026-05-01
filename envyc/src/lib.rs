use crate::{
    context::CompilationContext,
    lex::{buffer::TokenizedBuffer, Lexer},
    source::SourceId,
};

pub mod context;
pub(crate) mod dense;
pub mod diagnostics;
pub mod lex;
pub mod source;

pub fn lex<'src>(ctx: &mut CompilationContext<'src>, bytes: &'src [u8]) -> SourceId {
    // Register the source with a placeholder buffer that will be replaced after lexing.
    let id = ctx.sources.register(bytes, TokenizedBuffer::default());
    let source = ctx.sources.get_mut(id);
    let buffer = Lexer::new(source).lex();
    let _ = std::mem::replace(source.buffer_mut(), buffer);

    id
}
