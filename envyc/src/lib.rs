mod assert;
mod compiler;
mod error;
mod lexical_analysis;
mod location;

#[cfg(test)]
mod tests {
    use crate::assert::lexer_asserter::LexerAsserter;
    use crate::compiler::compile_unit::CompileUnit;
    use crate::error::CompilerErrorKind;
    use crate::lexical_analysis::token::TokenKind;

    #[test]
    fn lex_whitespace_successful() {
        let compile_unit = CompileUnit::new("Test", b"  \t \n  \r");
        let mut asserter = LexerAsserter::new(&compile_unit, false);
        asserter.assert_token(TokenKind::Whitespace(' '));
        asserter.assert_token(TokenKind::Whitespace(' '));
        asserter.assert_token(TokenKind::Whitespace('\t'));
        asserter.assert_token(TokenKind::Whitespace(' '));
        asserter.assert_token(TokenKind::Whitespace('\n'));
        asserter.assert_token(TokenKind::Whitespace(' '));
        asserter.assert_token(TokenKind::Whitespace(' '));
        asserter.assert_token(TokenKind::Whitespace('\r'));
    }

    #[test]
    fn lex_unknown_char_fails() {
        let compile_unit = CompileUnit::new("Test", b"^");
        let mut asserter = LexerAsserter::new(&compile_unit, true);
        asserter.assert_error(CompilerErrorKind::UnrecognizedCharacter);
    }

    #[test]
    fn lex_simple_number_expression() {
        let compile_unit = CompileUnit::new("Test", b"1 + 3 * 4");
        let mut asserter = LexerAsserter::new(&compile_unit, true);
        asserter.assert_token(TokenKind::Integer(1));
        asserter.assert_token(TokenKind::Plus);
        asserter.assert_token(TokenKind::Integer(3));
        asserter.assert_token(TokenKind::Star);
        asserter.assert_token(TokenKind::Integer(4));
    }

    #[test]
    fn compile_compile_unit_successfully() {
        let compile_unit = CompileUnit::new("Test", b" ");
        assert!(compile_unit.compile().is_ok());
    }
}
