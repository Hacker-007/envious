use crate::{diagnostics::Diagnostic, source::SourceMap};

/// Formats a diagnostic and writes it in "human-readable" text to
/// a sink [`std::io::Write`].
pub trait DiagnosticFormatter {
    fn format<'src, W>(
        &mut self,
        sources: &SourceMap<'src>,
        diagnostic: Diagnostic,
        sink: &mut W,
    ) -> std::io::Result<()>;
}

/// A diagnostic formatter that outputs a pretty rendered
/// message, similar to the `rustc` compiler.
pub struct PrettyFormatter;

impl DiagnosticFormatter for PrettyFormatter {
    fn format<'src, W>(
        &mut self,
        _sources: &SourceMap<'src>,
        _diagnostic: Diagnostic,
        _sink: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
