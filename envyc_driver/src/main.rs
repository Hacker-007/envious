use envyc_context::context::CompilationContext;
use envyc_lexer::lexer::LexicalAnalyzer;

pub fn parse<'ctx>(compilation_ctx: &'ctx CompilationContext) {
    for source in compilation_ctx.get_sources() {
        // Generate a lexical analyzer which uses an iterator
        // over the source text to lazily generate tokens as needed
        // by the parser, as opposed to eagerly generating all of the
        // token at once.
        let lazy_lexical_analyzer = LexicalAnalyzer::new(compilation_ctx, source);
        lazy_lexical_analyzer.for_each(|token| println!("{:#?}", token))
    }
}

fn main() {
    let mut compilation_ctx = CompilationContext::new();
    compilation_ctx.add_source("define get_one() = 1".to_string());
    parse(&compilation_ctx);
}
