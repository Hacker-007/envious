use std::sync::Mutex;

use codespan_reporting::term::{emit, termcolor::WriteColor, Config};

use crate::{
    error::{DiagnosticType, EnviousDiagnostic},
    source::{Source, SourceId, SourceMap},
};

pub struct CompilationContext<'text> {
    source_map: SourceMap<'text>,
    diagnostic_handler: Mutex<EnviousDiagnosticHandler>,
}

impl<'text> CompilationContext<'text> {
    pub fn new(diagnostic_stream_writer: Box<dyn WriteColor>) -> Self {
        Self {
            source_map: SourceMap::default(),
            diagnostic_handler: Mutex::new(EnviousDiagnosticHandler::new(diagnostic_stream_writer)),
        }
    }

    pub fn add_source(&mut self, name: &str, text: &'text str) -> SourceId {
        self.source_map.insert(name, text)
    }

    pub fn get_source(&self, source_id: SourceId) -> Option<&Source> {
        self.source_map.get(source_id)
    }

    pub fn emit_diagnostic(&self, diagnostic: EnviousDiagnostic) {
        let mut lock = self.diagnostic_handler.lock().unwrap();
        lock.handle_diagnostic(&self.source_map, &diagnostic);
    }

    pub fn get_diagnostic_stats(&self) -> DiagnosticStats {
        self.diagnostic_handler
            .lock()
            .unwrap()
            .get_stats()
    }
}

pub struct EnviousDiagnosticHandler {
    warning_count: usize,
    error_count: usize,
    diagnostic_stream_writer: Box<dyn WriteColor>,
}

impl EnviousDiagnosticHandler {
    pub fn new(diagnostic_stream_writer: Box<dyn WriteColor>) -> Self {
        Self {
            warning_count: 0,
            error_count: 0,
            diagnostic_stream_writer,
        }
    }

    pub fn handle_diagnostic(&mut self, source_map: &SourceMap, diagnostic: &EnviousDiagnostic) {
        emit(
            &mut self.diagnostic_stream_writer,
            &Config::default(),
            source_map,
            &diagnostic.into(),
        )
        .unwrap();

        match diagnostic.get_diagnostic_kind() {
            DiagnosticType::Warning => {
                self.warning_count += 1;
            }
            DiagnosticType::Error => {
                self.error_count += 1;
            }
        }
    }

    pub fn get_stats(&self) -> DiagnosticStats {
        DiagnosticStats {
            error_count: self.error_count,
            warning_count: self.warning_count,
        }
    }
}

pub struct DiagnosticStats {
    pub error_count: usize,
    pub warning_count: usize,
}

