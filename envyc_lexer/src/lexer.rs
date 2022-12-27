use envyc_context::context::CompilationContext;
use envyc_error::{error::{Diagnostic, Level}, error_handler::ErrorHandler};
use envyc_source::{
    snippet::{Snippet, SourcePos},
    source::{Source, SourceIter},
};

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct LexicalAnalyzer<'ctx, 'shared, 'source, E: ErrorHandler> {
    compilation_ctx: &'ctx CompilationContext<'shared, E>,
    source: &'source Source,
    source_iter: SourceIter<'source>,
    pos: usize,
}

impl<'ctx, 'shared, 'source, E: ErrorHandler> LexicalAnalyzer<'ctx, 'shared, 'source, E> {
    pub fn new(compilation_ctx: &'ctx CompilationContext<'shared, E>, source: &'source Source) -> Self {
        Self {
            compilation_ctx,
            source,
            source_iter: SourceIter::new(source),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.continue_while(|ch| ch.is_whitespace());
        match self.next()? {
            digit if digit.is_digit(10) => Some(self.tokenize_number(self.pos - 1)),
            letter if letter.is_alphabetic() => Some(self.tokenize_word(self.pos - 1)),
            '(' => Some(self.single_character_token(TokenKind::LeftParenthesis)),
            ')' => Some(self.single_character_token(TokenKind::RightParenthesis)),
            '{' => Some(self.single_character_token(TokenKind::LeftCurlyBrace)),
            '}' => Some(self.single_character_token(TokenKind::RightCurlyBrace)),
            '<' => self.optionally_continue(
                |ch| ch == '=',
                TokenKind::LessThanEqualSign,
                TokenKind::LeftAngleBracket,
            ),
            '>' => self.optionally_continue(
                |ch| ch == '=',
                TokenKind::GreaterThanEqualSign,
                TokenKind::RightAngleBracket,
            ),
            '+' | '-' if matches!(self.peek(), Some(digit) if digit.is_digit(10)) => {
                Some(self.tokenize_number(self.pos - 1))
            }
            '+' => Some(self.single_character_token(TokenKind::Plus)),
            '-' => Some(self.single_character_token(TokenKind::Minus)),
            '*' => Some(self.single_character_token(TokenKind::Star)),
            '/' => Some(self.single_character_token(TokenKind::Slash)),
            '%' => Some(self.single_character_token(TokenKind::PercentSign)),
            '=' => Some(self.single_character_token(TokenKind::EqualSign)),
            ',' => Some(self.single_character_token(TokenKind::Comma)),
            ':' => {
                self.optionally_continue(|ch| ch == ':', TokenKind::ColonColon, TokenKind::Colon)
            }
            ';' => Some(self.single_character_token(TokenKind::SemiColon)),
            _ => {
                self.compilation_ctx.emit_diagnostic(Diagnostic::new(
                    Level::Error,
                    vec!["unrecognized character"],
                    self.generate_snippet(self.pos - 1),
                ));

                None
            }
        }
    }

    pub fn tokenize_number(&mut self, start: usize) -> Token {
        let digits_snippet = self.continue_while(|ch| ch.is_digit(10));
        let word = self
            .source
            .get_range(digits_snippet.low, digits_snippet.high);
        let value = match word.parse() {
            Ok(value) => value,
            Err(_) => 0,
        };

        Token::new(
            digits_snippet.with_low(SourcePos(start)),
            TokenKind::Int(value),
        )
    }

    pub fn tokenize_word(&mut self, start: usize) -> Token {
        let snippet = self
            .continue_while(|ch| ch.is_alphabetic() || ch == '_')
            .with_low(SourcePos(start));
        let word = self.source.get_range(snippet.low, snippet.high);
        let kind = match word {
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
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
            _ => {
                let symbol = self.compilation_ctx.intern_symbol(word);
                TokenKind::Identifer(symbol)
            }
        };

        Token::new(snippet, kind)
    }

    pub fn continue_while<P: Fn(char) -> bool>(&mut self, predicate: P) -> Snippet {
        let start = self.pos - 1;
        loop {
            match self.peek() {
                Some(ch) if predicate(ch) => {
                    self.next();
                }
                _ => break,
            }
        }

        self.generate_snippet(start)
    }

    pub fn optionally_continue<P: Fn(char) -> bool>(
        &mut self,
        predicate: P,
        consumed_kind: TokenKind,
        default_kind: TokenKind,
    ) -> Option<Token> {
        let start = self.pos - 1;
        match self.peek()? {
            ch if predicate(ch) => {
                self.next();
                return Some(Token::new(self.generate_snippet(start), consumed_kind));
            }
            _ => Some(Token::new(self.generate_snippet(start), default_kind)),
        }
    }

    pub fn single_character_token(&self, kind: TokenKind) -> Token {
        Token::new(self.generate_snippet(self.pos - 1), kind)
    }

    pub fn generate_snippet(&self, start: usize) -> Snippet {
        Snippet::new(self.source.id, SourcePos(start), SourcePos(self.pos - 1))
    }

    pub fn peek(&mut self) -> Option<char> {
        self.source_iter.peek()
    }

    pub fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.source_iter.next()
    }
}

impl<'ctx, 'shared, 'source, E: ErrorHandler> Iterator for LexicalAnalyzer<'ctx, 'shared, 'source, E> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
