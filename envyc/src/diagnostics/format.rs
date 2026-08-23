use crate::{diagnostics::Diagnostic, source::SourceMap};

/// Formats a diagnostic and writes it in "human-readable" text to
/// a sink [`std::io::Write`].
pub trait DiagnosticFormatter {
    fn format<W>(
        &mut self,
        sources: &SourceMap,
        diagnostic: Diagnostic,
        sink: &mut W,
    ) -> std::io::Result<()>;
}

/// A diagnostic formatter that outputs a JSON payload for a
/// message.
pub struct JsonFormatter;

impl DiagnosticFormatter for JsonFormatter {
    fn format<W>(
        &mut self,
        _sources: &SourceMap,
        _diagnostic: Diagnostic,
        _sink: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}

/// A diagnostic formatter that outputs a pretty rendered
/// message, similar to the `rustc` compiler.
pub struct PrettyFormatter;

impl DiagnosticFormatter for PrettyFormatter {
    fn format<W>(
        &mut self,
        _sources: &SourceMap,
        _diagnostic: Diagnostic,
        _sink: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}
