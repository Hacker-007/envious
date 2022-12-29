use envyc_context::{context::CompilationContext, symbol::Symbol};
use envyc_error::{
    error::{Diagnostic, Level},
    error_handler::ErrorHandler,
};

use envyc_lexer::{
    lexer::LexicalAnalyzer,
    token::{Token, TokenKind},
};

use crate::ast::{Function, Member, Parameter, Program, SnippetedSymbol};

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

    pub fn parse(&mut self) -> Program {
        let mut members = vec![];
        while self.current().kind != TokenKind::EndOfFile {
            let start = self.current();
            let member = self.parse_member();
            members.push(member);
            if self.current() == start {
                self.next();
            }
        }

        self.expect(TokenKind::EndOfFile);
        Program::new(members)
    }

    fn parse_member(&mut self) -> Member {
        if self.current().kind == TokenKind::Define {
            Member::Function(self.parse_function())
        } else {
            self.parse_global_statement()
        }
    }

    fn parse_function(&mut self) -> Function {
        let define_keyword = self.expect(TokenKind::Define);
        let identifier = self.parse_identifier();
        let left_parenthesis = self.expect(TokenKind::LeftParenthesis);
        let parameters = self.parse_parameter_list();
        let right_parenthesis = self.expect(TokenKind::RightParenthesis);
        let equal_sign = self.expect(TokenKind::Equal);
        Function::new(
            define_keyword.snippet,
            identifier,
            left_parenthesis.snippet,
            parameters,
            right_parenthesis.snippet,
            equal_sign.snippet,
        )
    }

    fn parse_global_statement(&mut self) -> Member {
        todo!()
    }

    fn parse_parameter_list(&mut self) -> Vec<Parameter> {
        let mut parameters = vec![];
        if self.current().kind != TokenKind::RightParenthesis {
            let parameter = self.parse_identifier();
            parameters.push(Parameter::new(parameter));
            while self.current().kind != TokenKind::RightParenthesis {
                self.expect(TokenKind::Comma);
                let parameter = self.parse_identifier();
                parameters.push(Parameter::new(parameter));
            }
        }

        parameters
    }

    fn parse_identifier(&mut self) -> SnippetedSymbol {
        let current = self.current();
        return match current.kind {
            TokenKind::Identifer(symbol) => {
                self.next();
                SnippetedSymbol::new(current.snippet, symbol)
            }
            _ => {
                self.compilation_ctx.emit_diagnostic(
                    Diagnostic::new(Level::Error, vec!["unexpected {} found"], current.snippet)
                        .add_footer(
                            Level::Warning,
                            format!(
                                "expected a(n) {} but found a(n) {} instead",
                                TokenKind::Identifer(Symbol(0)),
                                current.kind
                            ),
                        ),
                );

                SnippetedSymbol::new(current.snippet, Symbol(0))
            }
        };
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        let current = self.current();
        if current.kind == kind {
            self.next()
        } else {
            let mut diagnostic =
                Diagnostic::new(Level::Error, vec!["unexpected {} found"], current.snippet)
                    .add_footer(
                        Level::Warning,
                        format!(
                            "expected a(n) {} but found a(n) {} instead",
                            kind, current.kind
                        ),
                    );

            if kind == TokenKind::EndOfFile {
                diagnostic = diagnostic.add_footer(
                    Level::Hint,
                    "there might be an issue with the source encoding",
                );
            }

            self.compilation_ctx.emit_diagnostic(diagnostic);
            Token::new(current.snippet, kind)
        }
    }

    fn peek(&self, offset: usize) -> Token {
        let idx = self.next_position + offset;
        if idx >= self.tokens.len() {
            self.tokens[self.tokens.len() - 1]
        } else {
            self.tokens[idx]
        }
    }

    fn current(&self) -> Token {
        self.peek(0)
    }

    fn next(&mut self) -> Token {
        let token = self.current();
        self.next_position += 1;
        token
    }
}
