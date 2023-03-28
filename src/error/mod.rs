use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::source::{SourceId, Span};

#[derive(Debug)]
pub enum EnviousDiagnostic {
    LexerDiagnostic(LexerDiagnosticKind),
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
        }
    }
}

impl From<&EnviousDiagnostic> for Diagnostic<SourceId> {
    fn from(diagnostic: &EnviousDiagnostic) -> Self {
        match diagnostic {
            EnviousDiagnostic::LexerDiagnostic(kind) => kind.into(),
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
