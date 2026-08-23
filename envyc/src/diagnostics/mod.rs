use std::io::{self, Write};

use smallvec::SmallVec;

use crate::{
    diagnostics::format::DiagnosticFormatter,
    lex::token::buffer::TokenIndex,
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
    UnexpectedEndOfFile,
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
}
