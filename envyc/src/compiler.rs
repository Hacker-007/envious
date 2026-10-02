use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{
    ast::Ast,
    context::CompilationContext,
    diagnostics::{format::DiagnosticFormatter, DiagnosticBag},
    lex::{token::buffer::TokenizedBuffer, Lexer},
    parser::Parser,
    source::{Source, SourceId},
};

#[derive(Debug, Default)]
pub struct Compiler {
    pub(crate) ctx: CompilationContext,
    buffers: HashMap<SourceId, TokenizedBuffer>,
    asts: HashMap<SourceId, Ast>,
}

impl Compiler {
    pub fn lex(&mut self, name: impl ToString, source: impl Into<Box<str>>) -> SourceId {
        let id = self.ctx.sources.register(name, source);
        let source = self.ctx.sources.get(id);
        let buffer = Lexer::new(source, &mut self.ctx.diagnostics).lex();
        self.buffers.insert(id, buffer);
        id
    }

    pub fn update(&mut self, _id: SourceId, _source: impl Into<Box<str>>) {
        // TODO:
        // We need to update the text for `self.ctx.sources[id]`, re-lex
        // the new text and invalidate all dependents of that source.
    }

    pub fn parse(&mut self, id: SourceId) {
        let buffer = &self.buffers[&id];
        let diagnostics = &mut self.ctx.diagnostics;
        self.asts
            .entry(id)
            .or_insert_with(|| Parser::new(id, buffer, diagnostics).parse());
    }

    pub fn tokens(&self, id: SourceId) -> &TokenizedBuffer {
        &self.buffers[&id]
    }

    pub fn source(&self, id: SourceId) -> &Source {
        self.ctx.sources.get(id)
    }

    pub fn ast(&self, id: SourceId) -> &Ast {
        &self.asts[&id]
    }

    #[inline]
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.ctx.diagnostics
    }

    /// Formats all of the diagnostics and outputs them using the formatter to the
    /// sink.
    pub fn emit(
        &mut self,
        formatter: &mut impl DiagnosticFormatter,
        sink: &mut impl Write,
    ) -> io::Result<()> {
        for diagnostic in self.ctx.diagnostics.drain() {
            formatter.emit(self, diagnostic, sink)?;
        }

        Ok(())
    }
}
