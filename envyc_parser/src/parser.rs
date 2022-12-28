use envyc_context::context::CompilationContext;
use envyc_error::error_handler::ErrorHandler;
use envyc_lexer::{lexer::LexicalAnalyzer, token::Token};

#[derive(Debug)]
pub struct Parser<'ctx, 'shared, E: ErrorHandler> {
    compilation_ctx: &'ctx CompilationContext<'shared, E>,
    next_position: usize,
    tokens: Vec<Token>,
}

impl<'ctx, 'shared, E: ErrorHandler> Parser<'ctx, 'shared, E> {
    pub fn new(
        compilation_ctx: &'ctx CompilationContext<'shared, E>,
        lexer: LexicalAnalyzer<'ctx, 'shared, '_, E>,
    ) -> Self {
        // TODO: Compare the performance benefits of eager vs lazy evaluation of tokens.
        Self {
            compilation_ctx,
            next_position: 0,
            tokens: lexer.collect(),
        }
    }
}
