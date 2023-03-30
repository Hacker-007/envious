use crate::{
    context::CompilationContext,
    error::{EnviousDiagnostic, LexerDiagnosticKind},
    source::{Source, SourceIter, SourcePos, Span, Spanned, WithSpan},
};

use super::token_kind::TokenKind;

#[derive(Debug)]
pub struct TokenStream<'ctx, 'source, 'text> {
    compilation_ctx: &'ctx CompilationContext<'text>,
    source_iter: SourceIter<'source, 'text>,
}

impl<'ctx, 'source, 'text> TokenStream<'ctx, 'source, 'text> {
    pub fn new(
        compilation_ctx: &'ctx CompilationContext<'text>,
        source: &'source Source<'text>,
    ) -> Self {
        Self {
            compilation_ctx,
            source_iter: SourceIter::new(source),
        }
    }

    pub fn get_compilation_ctx(&self) -> &'ctx CompilationContext<'text> {
        self.compilation_ctx
    }

    fn next_token(&mut self) -> Option<Spanned<TokenKind>> {
        loop {
            let (start, ch) = self.next()?;
            match ch {
                whitespace if whitespace.is_whitespace() => {
                    self.continue_while(start, |ch| ch.is_whitespace());
                }
                '/' if matches!(self.peek(), Some((_, '/'))) => {
                    self.continue_while(start, |ch| ch != '\n');
                    self.next();
                }
                digit if digit.is_ascii_digit() => return Some(self.tokenize_number(start)),
                letter if letter.is_alphabetic() => return Some(self.tokenize_word(start)),
                '(' => {
                    return Some(
                        TokenKind::LeftParenthesis.with_span(self.make_span(start, start + 1)),
                    )
                }
                ')' => {
                    return Some(
                        TokenKind::RightParenthesis.with_span(self.make_span(start, start + 1)),
                    )
                }
                '{' => {
                    return Some(
                        TokenKind::LeftCurlyBrace.with_span(self.make_span(start, start + 1)),
                    )
                }
                '}' => {
                    return Some(
                        TokenKind::RightCurlyBrace.with_span(self.make_span(start, start + 1)),
                    )
                }
                '<' => {
                    return Some(self.optionally_continue(
                        start,
                        |ch| ch == '=',
                        TokenKind::LessThanEqual,
                        TokenKind::LeftAngleBracket,
                    ))
                }
                '>' => {
                    return Some(self.optionally_continue(
                        start,
                        |ch| ch == '=',
                        TokenKind::GreaterThanEqual,
                        TokenKind::RightAngleBracket,
                    ))
                }
                '+' => return Some(TokenKind::Plus.with_span(self.make_span(start, start + 1))),
                '-' => return Some(TokenKind::Minus.with_span(self.make_span(start, start + 1))),
                '*' => return Some(TokenKind::Star.with_span(self.make_span(start, start + 1))),
                '/' => return Some(TokenKind::Slash.with_span(self.make_span(start, start + 1))),
                '%' => return Some(TokenKind::Percent.with_span(self.make_span(start, start + 1))),
                '=' => return Some(TokenKind::Equal.with_span(self.make_span(start, start + 1))),
                '.' => return Some(TokenKind::Period.with_span(self.make_span(start, start + 1))),
                ',' => return Some(TokenKind::Comma.with_span(self.make_span(start, start + 1))),
                ':' => {
                    return Some(self.optionally_continue(
                        start,
                        |ch| ch == ':',
                        TokenKind::ColonColon,
                        TokenKind::Colon,
                    ))
                }
                ';' => {
                    return Some(TokenKind::SemiColon.with_span(self.make_span(start, start + 1)))
                }
                '\0' => return Some(TokenKind::EndOfFile.with_span(self.make_span(start, start))),
                _ => {
                    let span = self.make_span(start, start + 1);
                    self.compilation_ctx
                        .emit_diagnostic(EnviousDiagnostic::LexerDiagnostic(
                            LexerDiagnosticKind::UnknownCharacter(span),
                        ))
                }
            }
        }
    }

    fn tokenize_number(&mut self, start: SourcePos) -> Spanned<TokenKind> {
        let digits_span = self.continue_while(start, |ch| ch.is_ascii_digit());
        TokenKind::Int.with_span(digits_span)
    }

    fn tokenize_word(&mut self, start: SourcePos) -> Spanned<TokenKind> {
        let word_span = self.continue_while(start, |ch| ch.is_alphanumeric() || ch == '_');
        let word = self.source_iter.get_text(word_span);
        match word {
            "Int" => TokenKind::IntType,
            "Bool" => TokenKind::BooleanType,
            "true" => TokenKind::Boolean,
            "false" => TokenKind::Boolean,
            "not" => TokenKind::Not,
            "or" => TokenKind::Or,
            "and" => TokenKind::And,
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "define" => TokenKind::Define,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifer,
        }
        .with_span(word_span)
    }

    fn continue_while<P: Fn(char) -> bool>(&mut self, start: SourcePos, predicate: P) -> Span {
        let mut end = start;
        loop {
            match self.peek() {
                Some((next_pos, ch)) if predicate(ch) => {
                    self.next();
                    end = next_pos;
                }
                _ => break,
            }
        }

        self.make_span(start, end + 1)
    }

    fn optionally_continue<P: Fn(char) -> bool>(
        &mut self,
        start: SourcePos,
        predicate: P,
        consumed_kind: TokenKind,
        default_kind: TokenKind,
    ) -> Spanned<TokenKind> {
        match self.peek() {
            Some((end, ch)) if predicate(ch) => {
                self.next();
                consumed_kind.with_span(self.make_span(start, end + 1))
            }
            _ => default_kind.with_span(self.make_span(start, start + 1)),
        }
    }

    fn make_span(&self, start: SourcePos, end: SourcePos) -> Span {
        self.source_iter.span(start, end)
    }

    fn peek(&mut self) -> Option<(SourcePos, char)> {
        self.source_iter.peek()
    }

    fn next(&mut self) -> Option<(SourcePos, char)> {
        self.source_iter.next()
    }
}

impl<'ctx, 'source, 'text> Iterator for TokenStream<'ctx, 'source, 'text> {
    type Item = Spanned<TokenKind>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
