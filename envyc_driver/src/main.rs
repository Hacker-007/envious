use envyc_context::{context::CompilationContext, diagnostic_handler::DiagnosticHandler};
use envyc_error::handler::StdoutHandler;
use envyc_lexer::lexer::LexicalAnalyzer;

pub fn parse<'ctx, D: DiagnosticHandler>(compilation_ctx: &'ctx CompilationContext<D>) {
    for source in compilation_ctx.get_sources() {
        // Generate a lexical analyzer which uses an iterator
        // over the source text to lazily generate tokens as needed
        // by the parser, as opposed to eagerly generating all of the
        // token at once.
        let lazy_lexical_analyzer = LexicalAnalyzer::new(compilation_ctx, source);
        // lazy_lexical_analyzer.for_each(|token| println!("{:#?}", token))
        lazy_lexical_analyzer.for_each(|_| {})
    }
}

fn main() {
    let mut compilation_ctx = CompilationContext::new(StdoutHandler);
    compilation_ctx.add_source(
        "test script".to_string(),
        r#"123
123
123
123
123
123
testing 1233 not 
 {true + false} @  < > 1312321321321321321
 
 hello testing"#.to_string(),
    );

    parse(&compilation_ctx);
}
