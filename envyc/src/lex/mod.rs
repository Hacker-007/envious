use std::str::Chars;

use smallvec::SmallVec;

use crate::{
    diagnostics::{Anchor, Diagnostic, DiagnosticBag, DiagnosticKind, Severity},
    lex::token::{
        buffer::{TokenIndex, TokenizedBuffer},
        Token, TokenKind, Trivia, TriviaKind,
    },
    source::{Source, Span},
};

pub mod token;

#[derive(Debug, Clone, Copy)]
enum LexMode {
    /// We are not within any special context and
    /// tokenize the source buffer as normal.
    Standard,
    /// We are inside a string fragment, starting
    /// and ending with a double quote (i.e. `"`).
    InString,
    /// We are inside an interpolated expression
    /// within a string, starting with `${` and
    /// ending with `}`.
    InStringInterpolation,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    /// The byte offset of `current` within the source.
    idx: usize,
    /// The character at `idx`, or `'\0'` once exhausted.
    current: char,
    /// The character immediately following `current`, or
    /// `'\0'` once exhausted.
    next: char,
    /// The remaining characters after `next`.
    chars: Chars<'a>,
    source: &'a Source,
    buffer: TokenizedBuffer,
    current_mode: LexMode,
    /// A stack of all previous modes the
    /// lexer was in to support nested lexing.
    prior_modes: Vec<LexMode>,
    /// A vector of all trivia we have encountered
    /// but not yet assigned to a token.
    unassigned_trivia: SmallVec<[Trivia; 4]>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        let mut chars = source.chars();
        let current = chars.next().unwrap_or('\0');
        let next = chars.next().unwrap_or('\0');
        Self {
            idx: 0,
            current,
            next,
            chars,
            source,
            buffer: TokenizedBuffer::default(),
            current_mode: LexMode::Standard,
            prior_modes: vec![],
            unassigned_trivia: SmallVec::default(),
        }
    }

    /// TODO: write doc comment
    pub fn lex(mut self, diagnostics: &mut DiagnosticBag) -> TokenizedBuffer {
        while !self.at_end() {
            match self.current_mode {
                LexMode::Standard => self.lex_standard(diagnostics),
                LexMode::InString => todo!(),
                LexMode::InStringInterpolation => todo!(),
            }
        }

        let eof = self.span_since(self.idx);
        self.mint_at(TokenKind::EndOfFile, eof);
        self.buffer
    }

    /// TODO: write doc comment
    fn lex_standard(&mut self, diagnostics: &mut DiagnosticBag) {
        if let Some(trivia) = self.scan_trivia() {
            self.unassigned_trivia.push(trivia);
            return;
        }

        match (self.current(), self.peek()) {
            ('0'..='9', _) => {
                let start = self.idx;
                self.skip_while(|ch| ch.is_ascii_digit());
                self.mint_at(TokenKind::IntLiteral, self.span_since(start));
            }
            _ => {
                let idx = self.mint(TokenKind::Error);
                diagnostics.add(Diagnostic {
                    kind: DiagnosticKind::UnknownCharacter,
                    primary: Anchor(self.source.id(), idx),
                    secondary: SmallVec::default(),
                    severity: Severity::Error,
                });
            }
        }
    }

    /// Mints a single-character token starting at the
    /// current position, consuming that character first
    /// so that trailing-trivia lookahead sees the character
    /// that follows it, not the token's own text.
    #[inline]
    fn mint(&mut self, kind: TokenKind) -> TokenIndex {
        let span = Span::new(self.idx, self.idx + self.current.len_utf8());
        self.advance();
        self.mint_at(kind, span)
    }

    fn mint_at(&mut self, kind: TokenKind, span: Span) -> TokenIndex {
        let (leading, trailing) = self.sweep_trivia();
        self.buffer.push(Token {
            kind,
            span,
            leading,
            trailing,
        })
    }

    /// Looks ahead from the current position for trivia that should
    /// retroactively become trailing trivia of the token just minted:
    /// a run of trivia up to and including the first newline, kept
    /// only if it contains a comment (the association rule is "trivia
    /// defaults to leading; a same-line trailing comment + its newline
    /// overrides that to trailing of the previous token").
    fn sweep_trivia(&mut self) -> (SmallVec<[Trivia; 1]>, SmallVec<[Trivia; 1]>) {
        let leading = self
            .unassigned_trivia
            .drain(..)
            .collect::<SmallVec<[Trivia; 1]>>();

        let mut seen = SmallVec::new();
        let mut saw_comment = false;
        while let Some(trivia) = self.scan_trivia() {
            saw_comment |= trivia.kind == TriviaKind::Comment;
            seen.push(trivia);
            if trivia.kind == TriviaKind::Newline {
                break;
            }
        }

        if saw_comment {
            (leading, seen)
        } else {
            self.unassigned_trivia.extend(seen);
            (leading, SmallVec::default())
        }
    }

    /// Consumes a single piece of trivia (a horizontal whitespace character,
    /// a newline character, or a whole `//` comment run) at the current position,
    /// or does nothing and returns `None` if the current character doesn't start
    /// trivia.
    fn scan_trivia(&mut self) -> Option<Trivia> {
        let start = self.idx;
        let kind = match (self.current(), self.peek()) {
            ('\t' | '\x0C' | '\r' | ' ', _) => {
                self.skip_while(|ch| matches!(ch, '\t' | '\x0C' | '\r' | ' '));
                TriviaKind::Whitespace
            }
            ('\n', _) => {
                self.advance();
                TriviaKind::Newline
            }
            ('/', '/') => {
                self.skip_while(|ch| ch != '\n');
                TriviaKind::Comment
            }
            _ => return None,
        };

        Some(Trivia {
            kind,
            span: self.span_since(start),
        })
    }

    /// Builds a [`Span`] from `start` to the current position.
    #[inline]
    fn span_since(&self, start: usize) -> Span {
        Span::new(start, self.idx)
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.idx >= self.source.len()
    }

    #[inline]
    fn current(&self) -> char {
        self.current
    }

    #[inline]
    fn peek(&self) -> char {
        self.next
    }

    #[inline]
    fn skip_while(&mut self, predicate: impl Fn(char) -> bool) {
        while !self.at_end() && predicate(self.current()) {
            self.advance();
        }
    }

    #[inline]
    fn advance(&mut self) {
        debug_assert!(!self.at_end());
        self.idx += self.current.len_utf8();
        self.current = self.next;
        self.next = self.chars.next().unwrap_or('\0');
    }
}
