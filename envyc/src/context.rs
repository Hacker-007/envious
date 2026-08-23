use crate::{diagnostics::DiagnosticBag, source::SourceMap};

#[derive(Debug, Default)]
pub struct CompilationContext {
    pub(crate) sources: SourceMap,
    pub(crate) diagnostics: DiagnosticBag,
}
