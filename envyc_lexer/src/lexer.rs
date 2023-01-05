use envyc_context::{context::CompilationContext, diagnostic_handler::DiagnosticHandler};
use envyc_error::diagnostic::{Diagnostic, Level, Message, SubDiagnostic};
use envyc_source::{
    snippet::{Snippet, SourcePos},
    source::{Source, SourceIter},
};

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct LexicalAnalyzer<'ctx, 'source, D: DiagnosticHandler> {
    compilation_ctx: &'ctx CompilationContext<D>,
    source: &'source Source,
    source_iter: SourceIter<'source>,
}

impl<'ctx, 'source, D: DiagnosticHandler> LexicalAnalyzer<'ctx, 'source, D> {
    pub fn new(compilation_ctx: &'ctx CompilationContext<D>, source: &'source Source) -> Self {
        Self {
            compilation_ctx,
            source,
            source_iter: SourceIter::new(source),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let (start, ch) = self.next()?;
        match ch {
            whitespace if whitespace.is_whitespace() => Some(self.tokenize_whitespace(start)),
            digit if digit.is_digit(10) => Some(self.tokenize_number(start)),
            letter if letter.is_alphabetic() => Some(self.tokenize_word(start)),
            '(' => Some(self.single_character_token(start, TokenKind::LeftParenthesis)),
            ')' => Some(self.single_character_token(start, TokenKind::RightParenthesis)),
            '{' => Some(self.single_character_token(start, TokenKind::LeftCurlyBrace)),
            '}' => Some(self.single_character_token(start, TokenKind::RightCurlyBrace)),
            '<' => Some(self.optionally_continue(
                start,
                |ch| ch == '=',
                TokenKind::LessThanEqual,
                TokenKind::LeftAngleBracket,
            )),
            '>' => Some(self.optionally_continue(
                start,
                |ch| ch == '=',
                TokenKind::GreaterThanEqual,
                TokenKind::RightAngleBracket,
            )),
            '+' => Some(self.single_character_token(start, TokenKind::Plus)),
            '-' => Some(self.single_character_token(start, TokenKind::Minus)),
            '*' => Some(self.single_character_token(start, TokenKind::Star)),
            '/' => Some(self.single_character_token(start, TokenKind::Slash)),
            '%' => Some(self.single_character_token(start, TokenKind::Percent)),
            '=' => Some(self.single_character_token(start, TokenKind::Equal)),
            ',' => Some(self.single_character_token(start, TokenKind::Comma)),
            ':' => Some(self.optionally_continue(
                start,
                |ch| ch == ':',
                TokenKind::ColonColon,
                TokenKind::Colon,
            )),
            ';' => Some(self.single_character_token(start, TokenKind::SemiColon)),
            '\0' => Some(self.single_character_token(start, TokenKind::EndOfFile)),
            _ => {
                let snippet = self.cook_snippet(start, start + 1);
                Diagnostic::new(Level::Error, "unrecognized character")
                    .add_child(SubDiagnostic::new(
                        vec![Message::new(Level::Error, "", Some(snippet))],
                        snippet,
                    ))
                    .emit(&self.compilation_ctx);

                Some(self.single_character_token(start, TokenKind::Error))
            }
        }
    }

    fn tokenize_whitespace(&mut self, start: SourcePos) -> Token {
        let whitespace_snippet = self.continue_while(start, |ch| ch.is_whitespace());
        Token::new(whitespace_snippet, TokenKind::Whitespace)
    }

    fn tokenize_number(&mut self, start: SourcePos) -> Token {
        let digits_snippet = self.continue_while(start, |ch| ch.is_digit(10));
        Token::new(digits_snippet, TokenKind::Int)
    }

    fn tokenize_word(&mut self, start: SourcePos) -> Token {
        let word_snippet = self.continue_while(start, |ch| ch.is_alphanumeric() || ch == '_');
        let word = self.source.get_text(word_snippet.start, word_snippet.end);
        let kind = match word {
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
        };

        Token::new(word_snippet, kind)
    }

    fn continue_while<P: Fn(char) -> bool>(&mut self, start: SourcePos, predicate: P) -> Snippet {
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

        self.cook_snippet(start, end + SourcePos(1))
    }

    fn optionally_continue<P: Fn(char) -> bool>(
        &mut self,
        start: SourcePos,
        predicate: P,
        consumed_kind: TokenKind,
        default_kind: TokenKind,
    ) -> Token {
        match self.peek() {
            Some((end, ch)) if predicate(ch) => {
                self.next();
                return Token::new(self.cook_snippet(start, end + 1), consumed_kind);
            }
            _ => Token::new(self.cook_snippet(start, start + 1), default_kind),
        }
    }

    fn single_character_token(&self, start: SourcePos, kind: TokenKind) -> Token {
        Token::new(self.cook_snippet(start, start + 1), kind)
    }

    fn cook_snippet(&self, start: SourcePos, end: SourcePos) -> Snippet {
        Snippet::new(self.source.id, start, end)
    }

    fn peek(&mut self) -> Option<(SourcePos, char)> {
        self.source_iter.peek()
    }

    fn next(&mut self) -> Option<(SourcePos, char)> {
        self.source_iter.next()
    }
}

impl<'ctx, 'source, D: DiagnosticHandler> Iterator for LexicalAnalyzer<'ctx, 'source, D> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
