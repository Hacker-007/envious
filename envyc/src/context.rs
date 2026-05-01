use crate::{diagnostics::DiagnosticBag, source::SourceMap};

#[derive(Debug)]
pub struct CompilationContext<'src> {
    pub(crate) sources: SourceMap<'src>,
    pub(crate) diagnostics: DiagnosticBag,
}
