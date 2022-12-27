use envyc_context::context::CompilationContext;
use envyc_error::error_handler::ErrorHandler;
use envyc_lexer::lexer::LexicalAnalyzer;

pub fn parse<'ctx, E: ErrorHandler>(compilation_ctx: &'ctx CompilationContext<E>) {
    for (_, source) in compilation_ctx.get_sources() {
        // Generate a lexical analyzer which uses an iterator
        // over the source text to lazily generate tokens as needed
        // by the parser, as opposed to eagerly generating all of the
        // token at once.
        let lazy_lexical_analyzer = LexicalAnalyzer::new(compilation_ctx, source);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
