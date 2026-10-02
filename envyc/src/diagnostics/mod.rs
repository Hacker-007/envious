use std::fmt::Display;

use smallvec::SmallVec;

use crate::{
    lex::token::{buffer::TokenIndex, TokenKind},
    source::SourceId,
};

pub mod format;

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
    ExpectedToken {
        expected: TokenKind,
        actual: TokenKind,
    },
    ExpectedExpression,
}

impl DiagnosticKind {
    pub fn code(&self) -> &'static str {
        match self {
            DiagnosticKind::UnknownCharacter => "E001",
            DiagnosticKind::ExpectedToken { .. } => "E002",
            DiagnosticKind::ExpectedExpression => "E003",
        }
    }
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticKind::UnknownCharacter => write!(f, "found an unknown character"),
            DiagnosticKind::ExpectedToken { expected, actual } => {
                write!(f, "expected to find `{expected}` but got `{actual}`")
            }
            DiagnosticKind::ExpectedExpression => write!(f, "expected the start of an expression"),
        }
    }
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

    pub(crate) fn unknown_character(&mut self, source: SourceId, at: TokenIndex) {
        self.add(Diagnostic {
            kind: DiagnosticKind::UnknownCharacter,
            primary: Anchor(source, at),
            secondary: SmallVec::default(),
            severity: Severity::Error,
        })
    }

    pub(crate) fn expected_token(
        &mut self,
        source: SourceId,
        at: TokenIndex,
        expected: TokenKind,
        actual: TokenKind,
    ) {
        self.add(Diagnostic {
            kind: DiagnosticKind::ExpectedToken { expected, actual },
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

    pub(crate) fn drain(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.0)
    }
}
