mod compiler;
mod lexical_analysis;
mod error;
mod location;

#[cfg(test)]
mod tests {
    use crate::compiler::compile_unit::CompileUnit;
	use crate::lexical_analysis::{Lexer, token::TokenKind};
	use crate::error::CompilerError;
	use crate::location::Location;
	use std::mem::discriminant;

	#[test]
	fn lex_whitespace_successful() {
		let compile_unit = CompileUnit::new("Test", b"   \t \n    \r    \n");
		let lexer = Lexer::new(&compile_unit);
        let result = lexer.into_iter().collect::<Result<Vec<_>, _>>();

		assert!(result.is_ok());
		let tokens = result.unwrap();
		for token in &tokens {
			assert_eq!(discriminant(&token.kind), discriminant(&TokenKind::Whitespace(' ')));
		}
	}

	#[test]
	fn lex_unknown_char_fails() {
		let compile_unit = CompileUnit::new("Test", b"^");
		let lexer = Lexer::new(&compile_unit);
        let result = lexer.into_iter().collect::<Result<Vec<_>, _>>();

		assert!(result.is_err());
		let error = result.unwrap_err();
		assert_eq!(discriminant(&error), discriminant(&CompilerError::UnrecognizedCharacter(Location::new(0, 1))))
	}
	
    #[test]
    fn compile_compile_unit_successfully() {
        let compile_unit = CompileUnit::new("Test", b" ");
        assert!(compile_unit.compile().is_ok());
    }
}
