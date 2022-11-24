use envyc_context::context::CompilationContext;
use envyc_source::{source::{Source, SourceIter}, snippet::{Snippet, SourcePos}};

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct LexicalAnalyzer<'ctx, 'source> {
    compilation_ctx: &'ctx CompilationContext,
    source: &'source Source,
    source_iter: SourceIter<'source>,
	pos: usize,
}

impl<'ctx, 'source> LexicalAnalyzer<'ctx, 'source> {
    pub fn new(compilation_ctx: &'ctx CompilationContext, source: &'source Source) -> Self {
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
            '+' | '-' if matches!(self.peek(), Some(digit) if digit.is_digit(10)) => Some(self.tokenize_number(self.pos - 1)),
            letter if letter.is_alphabetic() => Some(self.tokenize_word(self.pos - 1)),
            _ => None,
		}
    }

    pub fn tokenize_number(&mut self, start: usize) -> Token {
        let digits_snippet = self.continue_while(|ch| ch.is_digit(10));
        Token::new(digits_snippet.with_low(SourcePos(start)), TokenKind::Int)
    }

    pub fn tokenize_word(&mut self, start: usize) -> Token {
        let snippet = self.continue_while(|ch| ch.is_alphabetic() || ch == '_').with_low(SourcePos(start));
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
                println!("Intern identifiers into a constant pool in the context!");
                TokenKind::Identifer
            },
        };

        Token::new(snippet, kind)
    }

    pub fn continue_while<P: Fn(char) -> bool>(&mut self, predicate: P) -> Snippet {
        let start = self.pos - 1;
        loop {
            match self.peek() {
                Some(ch) if predicate(ch) => {
                    self.next();
                },
                _ => break,
            }
        }

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

impl<'ctx, 'source> Iterator for LexicalAnalyzer<'ctx, 'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}