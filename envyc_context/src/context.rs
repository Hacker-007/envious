use envyc_source::source::{Source, SourceId};

use crate::diagnostic_handler::DiagnosticHandler;

#[derive(Debug)]
pub struct CompilationContext<D: DiagnosticHandler> {
    sources: Vec<Source>,
    pub diagnostic_handler: D,
}

impl<D: DiagnosticHandler> CompilationContext<D> {
    pub fn new(diagnostic_handler: D) -> Self {
        Self {
            sources: vec![],
            diagnostic_handler,
        }
    }

    pub fn add_source(&mut self, name: String, text: String) {
        let source_id = self.sources.len();
        self.sources.push(Source::new(source_id, name, text));
    }

    pub fn get_source(&self, source_id: SourceId) -> &Source {
        &self.sources[source_id]
    }

    pub fn get_sources(&self) -> std::slice::Iter<Source> {
        self.sources.iter()
    }
}
