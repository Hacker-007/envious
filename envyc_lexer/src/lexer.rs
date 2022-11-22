use envyc_context::context::CompilationContext;
use envyc_source::source::Source;

#[derive(Debug)]
pub struct LexicalAnalyzer<'ctx, 'source> {
    compilation_ctx: &'ctx CompilationContext,
    source: &'source Source,
}

impl<'ctx, 'source> LexicalAnalyzer<'ctx, 'source> {
    pub fn new(compilation_ctx: &'ctx CompilationContext, source: &'source Source) -> Self {
        Self {
            compilation_ctx,
            source,
        }
    }

    pub fn next_token(&mut self) -> Option<()> {
        todo!()
    }
}

impl<'ctx, 'source> Iterator for LexicalAnalyzer<'ctx, 'source> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}