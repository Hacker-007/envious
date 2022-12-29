use envyc_context::context::CompilationContext;
use envyc_error::error_handler::ErrorHandler;
use envyc_lexer::lexer::LexicalAnalyzer;
use envyc_parser::parser::Parser;

pub fn parse<'ctx, E: ErrorHandler>(compilation_ctx: &'ctx CompilationContext<E>) {
    for (_, source) in compilation_ctx.get_sources() {
        // Generate a lexical analyzer which uses an iterator
        // over the source text to lazily generate tokens as needed
        // by the parser, as opposed to eagerly generating all of the
        // token at once.
        let lazy_lexical_analyzer = LexicalAnalyzer::new(compilation_ctx, source);
        let mut syntactic_analyzer = Parser::new(&compilation_ctx, lazy_lexical_analyzer);
        let program = syntactic_analyzer.parse();
        println!("{:#?}", program);
    }
}

fn main() {
    use std::sync::RwLock;

    use envyc_context::shared_resources::SharedResources;
    use envyc_error::error_handler::StdoutErrorHandler;
    use envyc_source::source::SourceMeta;

    let shared_resources = RwLock::new(SharedResources::new(StdoutErrorHandler::default()));
    let mut compilation_ctx = CompilationContext::new(&shared_resources);
    compilation_ctx.add_source(
        SourceMeta::String,
        "define test(x, wonderful) =".to_string()
    );

    parse(&compilation_ctx);
}
