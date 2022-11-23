use envyc_context::context::CompilationContext;
use envyc_source::source::Source;

use crate::token::Token;

#[derive(Debug)]
pub struct LexicalAnalyzer<'ctx, 'source> {
    compilation_ctx: &'ctx CompilationContext,
    source: &'source Source,
    next_source_idx: usize,
}

impl<'ctx, 'source> LexicalAnalyzer<'ctx, 'source> {
    pub fn new(compilation_ctx: &'ctx CompilationContext, source: &'source Source) -> Self {
        Self {
            compilation_ctx,
            source,
            next_source_idx: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        todo!()
    }

    pub fn next(&mut self) -> Option<char> {
        self.next_source_idx += 1;
        self.source.get_char(self.next_source_idx - 1)
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get_char(self.next_source_idx)
    }
}

impl<'ctx, 'source> Iterator for LexicalAnalyzer<'ctx, 'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}