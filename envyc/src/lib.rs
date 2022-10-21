mod ast;
mod compiler;
mod error;
mod lexical_analysis;
mod location;

mod assert;

#[cfg(test)]
mod tests {
    use crate::assert::lexer_asserter::LexerAsserter;
    use crate::compiler::compile_session::CompileSession;
    use crate::compiler::compile_unit::CompileUnit;
    use crate::error::CompilerErrorKind;
    use crate::lexical_analysis::token::TokenKind;

    #[test]
    fn lex_whitespace_successful() {
        let mut compile_session = CompileSession::new();
        let compile_unit = CompileUnit::new("Test", b"  \t \n  \r");
        let mut asserter = LexerAsserter::new(&mut compile_session, &compile_unit, false);
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
        let mut compile_session = CompileSession::new();
        let compile_unit = CompileUnit::new("Test", b"^");
        let mut asserter = LexerAsserter::new(&mut compile_session, &compile_unit, true);
        asserter.assert_error(CompilerErrorKind::UnrecognizedCharacter);
    }

    #[test]
    fn lex_simple_number_expression() {
        let mut compile_session = CompileSession::new();
        let compile_unit = CompileUnit::new("Test", b"1 + 3 * 4");
        let mut asserter = LexerAsserter::new(&mut compile_session, &compile_unit, true);
        asserter.assert_token(TokenKind::Integer(1));
        asserter.assert_token(TokenKind::Plus);
        asserter.assert_token(TokenKind::Integer(3));
        asserter.assert_token(TokenKind::Star);
        asserter.assert_token(TokenKind::Integer(4));
    }
}
