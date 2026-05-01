use std::io::{self, Write};

use crate::{
    diagnostics::format::DiagnosticFormatter,
    lex::buffer::TokenIndex,
    source::{SourceId, SourceMap},
};

mod format;

/// A combination of a source ID and token index
/// that can be used to resolve locations when
/// reporting diagnostics.
#[derive(Debug, Clone, Copy)]
pub struct Anchor(SourceId, TokenIndex);

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Hint,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedEndOfFile,
}

#[derive(Debug, Clone, Copy)]
pub struct Diagnostic {
    kind: DiagnosticKind,
    /// The primary anchor location of this
    /// diagnostic.
    primary: Anchor,
    severity: Severity,
}

#[derive(Debug, Default)]
pub struct DiagnosticBag(Vec<Diagnostic>);

impl DiagnosticBag {
    /// Stores the diagnostic until the next [`DiagnosticBag::flush`] call.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.0.push(diagnostic);
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
}
