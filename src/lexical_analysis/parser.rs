use std::iter::Peekable;

use crate::{
    context::CompilationContext,
    error::{EnviousDiagnostic, ParserDiagnosticKind},
    source::Source,
};

use super::{
    syntax::ast::{Function, Program},
    token_kind::TokenKind,
    token_stream::TokenStream,
};

#[derive(Debug)]
pub struct Parser<'ctx, 'source, 'text> {
    compilation_ctx: &'ctx CompilationContext<'text>,
    token_stream: Peekable<TokenStream<'ctx, 'source, 'text>>,
}

impl<'ctx, 'source, 'text> Parser<'ctx, 'source, 'text> {
    pub fn new(
        compilation_ctx: &'ctx CompilationContext<'text>,
        source: &'source Source<'text>,
    ) -> Self {
        Self {
            compilation_ctx,
            token_stream: TokenStream::new(compilation_ctx, source).peekable(),
        }
    }

    pub fn from_stream(token_stream: TokenStream<'ctx, 'source, 'text>) -> Self {
        Self {
            compilation_ctx: token_stream.get_compilation_ctx(),
            token_stream: token_stream.peekable(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut functions = vec![];
        while let Some(token) = self.token_stream.peek() {
            match token.item() {
                TokenKind::Define => {
                    functions.push(self.parse_function());
                }
                found_kind => {
                    self.compilation_ctx
                        .emit_diagnostic(EnviousDiagnostic::ParserDiagnostic(
                            ParserDiagnosticKind::ExpectedKind {
                                span: token.span(),
                                expected_kinds: vec![TokenKind::Define],
                                found_kind: *found_kind,
                            },
                        ));

                    self.token_stream.next();
                }
            }
        }

        Program { functions }
    }

    fn parse_function(&mut self) -> Function {
        todo!()
    }
}
