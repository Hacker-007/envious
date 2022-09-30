use crate::error::CompilerError;
use crate::lexical_analysis::token::Token;
use crate::lexical_analysis::Lexer;

#[derive(Debug)]
pub(crate) struct CompileUnit<'file> {
    pub(crate) file_name: String,
    pub(crate) file_contents: &'file [u8],
}

impl<'file> CompileUnit<'file> {
    pub fn new(file_name: &str, file_contents: &'file [u8]) -> Self {
        Self {
            file_name: file_name.to_string(),
            file_contents,
        }
    }

    pub fn compile(&self) -> Result<Vec<Token>, CompilerError> {
        let tokens = self.perform_lexical_analysis()?;
        Ok(tokens)
    }

    fn perform_lexical_analysis(&self) -> Result<Vec<Token>, CompilerError> {
        let lexer = Lexer::new(self);
        lexer.into_iter().collect::<Result<Vec<_>, _>>()
    }
}
