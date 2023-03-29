use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lexical_analysis::token_kind::TokenKind,
    source::{SourceId, Span},
};

#[derive(Debug)]
pub enum EnviousDiagnostic {
    LexerDiagnostic(LexerDiagnosticKind),
    ParserDiagnostic(ParserDiagnosticKind),
}

#[derive(Debug)]
pub enum DiagnosticType {
    Error,
    Warning,
}

impl EnviousDiagnostic {
    pub fn get_diagnostic_kind(&self) -> DiagnosticType {
        match self {
            Self::LexerDiagnostic(kind) => kind.get_diagnostic_kind(),
            Self::ParserDiagnostic(kind) => kind.get_diagnostic_kind(),
        }
    }
}

impl From<&EnviousDiagnostic> for Diagnostic<SourceId> {
    fn from(diagnostic: &EnviousDiagnostic) -> Self {
        match diagnostic {
            EnviousDiagnostic::LexerDiagnostic(kind) => kind.into(),
            EnviousDiagnostic::ParserDiagnostic(kind) => kind.into(),
        }
    }
}

#[derive(Debug)]
pub enum LexerDiagnosticKind {
    UnknownCharacter(Span),
}

impl LexerDiagnosticKind {
    pub fn get_diagnostic_kind(&self) -> DiagnosticType {
        match self {
            Self::UnknownCharacter(_) => DiagnosticType::Error,
        }
    }
}

impl From<&LexerDiagnosticKind> for Diagnostic<SourceId> {
    fn from(diagnostic: &LexerDiagnosticKind) -> Self {
        match diagnostic {
            LexerDiagnosticKind::UnknownCharacter(span) => Diagnostic::error()
                .with_code("E0001")
                .with_message("unrecognized character")
                .with_labels(vec![Label::primary(
                    span.source_id(),
                    span.start()..span.end(),
                )]),
        }
    }
}

#[derive(Debug)]
pub enum ParserDiagnosticKind {
    ExpectedKind {
        span: Span,
        expected_kinds: Vec<TokenKind>,
        found_kind: TokenKind,
    },
}

impl ParserDiagnosticKind {
    pub fn get_diagnostic_kind(&self) -> DiagnosticType {
        match self {
            Self::ExpectedKind { .. } => DiagnosticType::Error,
        }
    }
}

impl From<&ParserDiagnosticKind> for Diagnostic<SourceId> {
    fn from(diagnostic: &ParserDiagnosticKind) -> Self {
        match diagnostic {
            ParserDiagnosticKind::ExpectedKind {
                span,
                expected_kinds,
                found_kind,
            } => {
                let expected_kinds_message = match &expected_kinds[..] {
                    [] => unreachable!("got no expected kinds for error message!"),
                    [kind] => format!("{}", kind),
                    [kind_a, kind_b] => format!("{} or {}", kind_a, kind_b),
                    [first_kind, rest @ .., last] => format!(
                        "{}, {}or {}",
                        first_kind,
                        rest.iter()
                            .map(|kind| format!("{}", kind))
                            .collect::<Vec<_>>()
                            .join(", "),
                        last
                    ),
                };

                Diagnostic::error()
                    .with_code("E0002")
                    .with_message("unexpected item")
                    .with_labels(vec![Label::primary(
                        span.source_id(),
                        span.start()..span.end(),
                    )])
                    .with_notes(vec![
                        format!("expected {}", expected_kinds_message),
                        format!("got {}", found_kind),
                    ])
            }
        }
    }
}
