use std::{io::Write, ops::Deref};

use crate::{
    ast::Ast,
    compiler::Compiler,
    diagnostics::{Anchor, Diagnostic, Severity},
    lex::token::buffer::TokenizedBuffer,
    source::{SourceId, SourceMap},
};
use codespan_reporting::{
    diagnostic::{
        Diagnostic as CodespanDiagnostic, Label, LabelStyle, Severity as CodespanSeverity,
    },
    files::{Error as CodespanError, Files},
    term::{
        self,
        termcolor::{Ansi, BufferedStandardStream, ColorChoice, NoColor, WriteColor},
        Config, WriteStyle,
    },
};

/// Formats a diagnostic and writes it in "human-readable" text to the
/// sink [`std::io::Write`].
pub trait DiagnosticFormatter {
    fn emit<W: Write>(
        &mut self,
        compiler: &Compiler,
        diagnostic: Diagnostic,
        sink: &mut W,
    ) -> std::io::Result<()>;
}

/// A diagnostic formatter that outputs a JSON payload for a message.
#[derive(Debug, Clone, Copy)]
pub struct JsonFormatter;

impl DiagnosticFormatter for JsonFormatter {
    fn emit<W: Write>(
        &mut self,
        _compiler: &Compiler,
        _diagnostic: Diagnostic,
        _sink: &mut W,
    ) -> std::io::Result<()> {
        todo!()
    }
}

/// A diagnostic formatter that outputs a pretty rendered message, similar
/// to the `rustc` compiler, with ANSI escape sequences.
#[derive(Debug, Clone, Copy)]
pub struct PrettyFormatter {
    pub color: bool,
}

impl PrettyFormatter {
    fn format(&self, compiler: &Compiler, diagnostic: Diagnostic) -> CodespanDiagnostic<SourceId> {
        let builder: CodespanDiagnostic<SourceId> = match diagnostic.severity {
            Severity::Hint => CodespanDiagnostic::help(),
            Severity::Warning => CodespanDiagnostic::warning(),
            Severity::Error => CodespanDiagnostic::error(),
        };

        let labels = diagnostic
            .secondary
            .into_iter()
            .map(|diagnostic| (diagnostic, LabelStyle::Secondary))
            .chain([(diagnostic.primary, LabelStyle::Primary)])
            .map(|(anchor, style)| {
                let tokens = compiler.tokens(anchor.0);
                let span = tokens
                    .span_at(anchor.1)
                    .expect("anchor spans are always valid");

                Label::new(style, anchor.0, span)
            });

        builder
            .with_code(diagnostic.kind.code())
            .with_message(&diagnostic.kind)
            .with_labels_iter(labels)
    }
}

impl DiagnosticFormatter for PrettyFormatter {
    fn emit<W: Write>(
        &mut self,
        compiler: &Compiler,
        diagnostic: Diagnostic,
        sink: &mut W,
    ) -> std::io::Result<()> {
        let diagnostic = self.format(compiler, diagnostic);
        let mut writer: &mut dyn WriteColor = if self.color {
            &mut Ansi::new(sink)
        } else {
            &mut NoColor::new(sink)
        };

        match term::emit_to_write_style(
            &mut writer,
            &Config::default(),
            &compiler.ctx.sources,
            &diagnostic,
        ) {
            Ok(()) => {}
            Err(CodespanError::Io(error)) => return Err(error),
            Err(error) => unreachable!("formatting should always succeed {:#?}", error),
        }

        Ok(())
    }
}

impl<'a> Files<'a> for SourceMap {
    type FileId = SourceId;
    type Name = &'a str;
    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, CodespanError> {
        Ok(self.get(id).name())
    }

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source, CodespanError> {
        Ok(self.get(id).text())
    }

    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> Result<usize, CodespanError> {
        self.get(id)
            .lines()
            .binary_search(&byte_index)
            .or_else(|next_line| Ok(next_line - 1))
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, CodespanError> {
        let source = self.get(id);
        let line_start = source
            .line(line_index)
            .ok_or_else(|| CodespanError::LineTooLarge {
                given: line_index,
                max: source.lines().len() - 1,
            })?;

        let line_end = source
            .line(line_index + 1)
            .ok_or_else(|| CodespanError::LineTooLarge {
                given: line_index,
                max: source.lines().len() - 1,
            })?;

        Ok(line_start..line_end)
    }
}
