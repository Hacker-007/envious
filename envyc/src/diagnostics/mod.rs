use std::io::{self, Write};

use smallvec::SmallVec;

use crate::{
    diagnostics::format::DiagnosticFormatter,
    lex::token::{buffer::TokenIndex, TokenKind},
    source::{SourceId, SourceMap},
};

mod format;

/// A combination of a source ID and token index
/// that can be used to resolve locations when
/// reporting diagnostics.
#[derive(Debug, Clone, Copy)]
pub struct Anchor(pub(crate) SourceId, pub(crate) TokenIndex);

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Hint,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    UnknownCharacter,
    ExpectedToken(TokenKind),
    ExpectedExpression,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub(crate) kind: DiagnosticKind,
    /// The primary anchor location of this
    /// diagnostic.
    pub(crate) primary: Anchor,
    pub(crate) secondary: SmallVec<[Anchor; 1]>,
    pub(crate) severity: Severity,
}

impl Diagnostic {
    pub(crate) fn unknown_character(source: SourceId, at: TokenIndex) -> Self {
        Self {
            kind: DiagnosticKind::UnknownCharacter,
            primary: Anchor(source, at),
            secondary: SmallVec::default(),
            severity: Severity::Error,
        }
    }

    pub(crate) fn expected_token(source: SourceId, at: TokenIndex, expected: TokenKind) -> Self {
        Self {
            kind: DiagnosticKind::ExpectedToken(expected),
            primary: Anchor(source, at),
            secondary: SmallVec::new(),
            severity: Severity::Error,
        }
    }

    pub(crate) fn expected_expression(source: SourceId, at: TokenIndex) -> Self {
        Self {
            kind: DiagnosticKind::ExpectedExpression,
            primary: Anchor(source, at),
            secondary: SmallVec::new(),
            severity: Severity::Error,
        }
    }
}

#[derive(Debug, Default)]
pub struct DiagnosticBag(Vec<Diagnostic>);

impl DiagnosticBag {
    /// Stores the diagnostic until the next [`DiagnosticBag::flush`] call.
    #[inline]
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.0.push(diagnostic);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Formats all of the diagnostic in the buffer and outputs
    /// them to `sink`.
    pub fn flush(
        &mut self,
        sources: &SourceMap,
        formatter: &mut impl DiagnosticFormatter,
        sink: &mut impl Write,
    ) -> io::Result<()> {
        for diagnostic in self.0.drain(..) {
            formatter.format(sources, diagnostic, sink)?;
        }

        Ok(())
    }

    pub(crate) fn unknown_character(&mut self, source: SourceId, at: TokenIndex) {
        self.add(Diagnostic {
            kind: DiagnosticKind::UnknownCharacter,
            primary: Anchor(source, at),
            secondary: SmallVec::default(),
            severity: Severity::Error,
        })
    }

    pub(crate) fn expected_token(&mut self, source: SourceId, at: TokenIndex, expected: TokenKind) {
        self.add(Diagnostic {
            kind: DiagnosticKind::ExpectedToken(expected),
            primary: Anchor(source, at),
            secondary: SmallVec::new(),
            severity: Severity::Error,
        })
    }

    pub(crate) fn expected_expression(&mut self, source: SourceId, at: TokenIndex) {
        self.add(Diagnostic {
            kind: DiagnosticKind::ExpectedExpression,
            primary: Anchor(source, at),
            secondary: SmallVec::new(),
            severity: Severity::Error,
        })
    }
}
