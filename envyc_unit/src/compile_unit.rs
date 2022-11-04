use crate::source::Source;

// The current stage of a compilation unit, which is used to drive
// the progress of the unit in different situations. These stages
// do not contain any information regarding the results of these stages,
// but simply represent the current stage of the unit to inform the
// dependecy graph about which dependencies can be run and which
// ones must wait.
pub enum CompilationStage {
    YetToStart,
    Working,
    LexicallyAnalyzed,
}

// The fundamental unit of compilation. All operations related to
// the compilation process must run through this unit. The idea behind
// this is that a dependency graph of these units can be constructed to
// easily run operations in parallel and to easily allow incremental
// compilation, as compilation units can be swapped out, re-evaluated,
// and dependent units can be re-evaluated, leading to faster compiles.
pub struct CompileUnit {
    id: usize,
    source: Source,
    compilation_stage: CompilationStage,
    token_stream: Option<Vec<usize>>,
}

impl CompileUnit {
    pub fn new(id: usize, source: Source) -> Self {
        Self {
            id: 0,
            source,
            compilation_stage: CompilationStage::YetToStart,
            token_stream: None,
        }
    }

    pub fn lexically_analyze(&mut self) {
        self.compilation_stage = CompilationStage::Working;
        // Tokenize the source file into a token stream that can
        // be used for the next stage.
        self.compilation_stage = CompilationStage::LexicallyAnalyzed;
    }

    pub fn get_current_stage(&self) -> &CompilationStage {
        &self.compilation_stage
    }
}
