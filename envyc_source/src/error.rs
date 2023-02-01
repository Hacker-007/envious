use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};

use crate::{source::SourceId, span::Span};

pub type EnvyErrorCode = u16;

pub enum EnvyErrorLevel {
    Info,
    Warning,
    Error,
}

impl From<EnvyErrorLevel> for Severity {
    fn from(level: EnvyErrorLevel) -> Self {
        match level {
            EnvyErrorLevel::Info => Severity::Note,
            EnvyErrorLevel::Warning => Severity::Warning,
            EnvyErrorLevel::Error => Severity::Error,
        }
    }
}

pub struct EnvyErrorAnnotation {
    pub span: Span,
    pub message: Option<String>,
}

impl EnvyErrorAnnotation {
    pub(self) fn to_codespan_label(self, label_style: LabelStyle) -> Label<SourceId> {
        let label = Label::new(label_style, self.span.source_id, self.span);
        match self.message {
            Some(message) => label.with_message(message),
            None => label,
        }
    }
}

pub struct EnvyError {
    pub level: EnvyErrorLevel,
    pub code: EnvyErrorCode,
    pub title: String,
    pub annotations: Vec<EnvyErrorAnnotation>,
    pub footer_notes: Vec<String>,
}

impl From<EnvyError> for Diagnostic<SourceId> {
    fn from(error: EnvyError) -> Self {
        Diagnostic::new(error.level.into())
            .with_code(format!("E{:04}", error.code))
            .with_message(error.title)
            .with_labels(
                error
                    .annotations
                    .into_iter()
                    .enumerate()
                    .map(|(idx, annotation)| {
                        let label_style = if idx == 0 {
                            LabelStyle::Primary
                        } else {
                            LabelStyle::Secondary
                        };

                        annotation.to_codespan_label(label_style)
                    })
                    .collect(),
            )
            .with_notes(error.footer_notes)
    }
}
