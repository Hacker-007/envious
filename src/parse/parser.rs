use std::{iter::Peekable, mem};

use crate::{
    context::CompilationContext,
    error::{EnviousDiagnostic, ParserDiagnosticKind},
    source::{Source, Span, Spanned, WithSpan},
};

use super::{
    syntax::{
        ast::{Function, Parameter, Program},
        expression::{Identifier, Type},
    },
    token_kind::TokenKind,
    token_stream::TokenStream,
};

#[derive(Debug)]
pub struct Parser<'ctx, 'source, 'text> {
    compilation_ctx: &'ctx CompilationContext<'text>,
    token_stream: Peekable<TokenStream<'ctx, 'source, 'text>>,
    source: &'source Source<'text>,
    previous_token: Spanned<TokenKind>,
    current_token: Spanned<TokenKind>,
}

impl<'ctx, 'source, 'text> Parser<'ctx, 'source, 'text> {
    pub fn new(
        compilation_ctx: &'ctx CompilationContext<'text>,
        source: &'source Source<'text>,
    ) -> Self {
        Self {
            compilation_ctx,
            token_stream: TokenStream::new(compilation_ctx, source).peekable(),
            source,
            previous_token: TokenKind::Dummy.with_span(Span::new(source.id(), 0, 0)),
            current_token: TokenKind::Dummy.with_span(Span::new(source.id(), 0, 0)),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut functions = vec![];
        loop {
            let token = self.next();
            match *token.item() {
                TokenKind::EndOfFile => break,
                TokenKind::Define => {
                    if let Some(function) = self.parse_function() {
                        functions.push(function);
                    }
                }
                found_kind => {
                    self.compilation_ctx
                        .emit_diagnostic(EnviousDiagnostic::ParserDiagnostic(
                            ParserDiagnosticKind::ExpectedKind {
                                span: token.span(),
                                expected_kinds: vec![TokenKind::Define],
                                found_kind,
                            },
                        ));
                }
            }
        }

        Program { functions }
    }

    fn parse_function(&mut self) -> Option<Function> {
        let define_keyword = self.current_token.span();
        let name = self.parse_identifier()?;
        let left_parenthesis = self.expect(vec![TokenKind::LeftParenthesis])?.span();
        let parameters = self.parse_parameters()?;
        let right_parenthesis = self.expect(vec![TokenKind::RightParenthesis])?.span();
        let equal_sign = self.expect(vec![TokenKind::Equal])?.span();
        Some(Function {
            define_keyword,
            name,
            left_parenthesis,
            parameters,
            right_parenthesis,
            equal_sign,
        })
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        let identifier_span = self.expect(vec![TokenKind::Identifer])?.span();
        let identifier = &self.source[identifier_span];
        let symbol_id = self.compilation_ctx.get_symbol(identifier);
        Some(symbol_id.with_span(identifier_span))
    }

    fn parse_parameters(&mut self) -> Option<Vec<Parameter>> {
        let mut parameters = vec![];
        let mut is_first_parameter = true;
        loop {
            let token = self.peek();
            let leading_comma = match *token.item() {
                TokenKind::RightParenthesis => break,
                TokenKind::Comma if !is_first_parameter => Some(self.next().span()),
                TokenKind::Identifer if is_first_parameter => {
                    is_first_parameter = false;
                    None
                }
                found_kind => {
                    let expected_kinds = if is_first_parameter {
                        vec![TokenKind::Identifer, TokenKind::RightParenthesis]
                    } else {
                        vec![TokenKind::Comma, TokenKind::RightParenthesis]
                    };

                    self.compilation_ctx
                        .emit_diagnostic(EnviousDiagnostic::ParserDiagnostic(
                            ParserDiagnosticKind::ExpectedKind {
                                span: token.span(),
                                expected_kinds,
                                found_kind,
                            },
                        ));

                    break;
                }
            };

            let name = self.parse_identifier()?;
            let colon = self.expect(vec![TokenKind::Colon])?.span();
            let ty = self.parse_type()?;
            parameters.push(Parameter {
                leading_comma,
                name,
                colon,
                ty,
            });
        }

        Some(parameters)
    }

    fn parse_type(&mut self) -> Option<Spanned<Type>> {
        let token = self.expect(vec![TokenKind::IntType, TokenKind::BooleanType])?;
        let span = token.span();
        let ty = match *token.item() {
            TokenKind::IntType => Type::Int.with_span(span),
            TokenKind::BooleanType => Type::Boolean.with_span(span),
            _ => unreachable!("tried to parse a type that was not expected!"),
        };

        Some(ty)
    }

    fn peek(&mut self) -> Spanned<TokenKind> {
        self.token_stream
            .peek()
            .copied()
            .unwrap_or(TokenKind::EndOfFile.with_span(Span::new(
                self.source.id(),
                self.source.len(),
                self.source.len(),
            )))
    }

    fn next(&mut self) -> Spanned<TokenKind> {
        let next_token = match self.token_stream.next() {
            Some(token) => token,
            None => {
                self.compilation_ctx
                    .emit_diagnostic(EnviousDiagnostic::ParserDiagnostic(
                        ParserDiagnosticKind::ExpectedKind {
                            span: self.current_token.span(),
                            expected_kinds: vec![],
                            found_kind: TokenKind::EndOfFile,
                        },
                    ));

                TokenKind::EndOfFile.with_span(Span::new(
                    self.source.id(),
                    self.source.len(),
                    self.source.len(),
                ))
            }
        };

        self.previous_token = mem::replace(&mut self.current_token, next_token);
        self.current_token
    }

    fn expect(&mut self, expected_kinds: Vec<TokenKind>) -> Option<Spanned<TokenKind>> {
        let token = self.next();
        let token_kind = *token.item();
        if expected_kinds.contains(&token_kind) {
            Some(token)
        } else {
            self.compilation_ctx
                .emit_diagnostic(EnviousDiagnostic::ParserDiagnostic(
                    ParserDiagnosticKind::ExpectedKind {
                        span: token.span(),
                        expected_kinds,
                        found_kind: token_kind,
                    },
                ));

            None
        }
    }
}
