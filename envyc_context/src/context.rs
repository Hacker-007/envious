use envyc_error::diagnostics::Diagnostic;
use envyc_source::source::Source;

#[derive(Debug)]
pub struct CompilationContext {
    sources: Vec<Source>,
}

impl CompilationContext {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    pub fn add_source(&mut self, text: String) {
        let source_id = self.sources.len();
        self.sources.push(Source::new(source_id, text));
    }

    pub fn get_sources(&self) -> std::slice::Iter<Source> {
        self.sources.iter()
    }

    pub fn emit_diagnostic(&self, diagnostic: Diagnostic) {
        println!("{:#?}", diagnostic);
        println!("TODO: emit diagnostics to dedicated error handler")
    }
}
