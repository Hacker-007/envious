pub mod token;

use crate::{error::Error, error::Span, interner::Interner};

use self::token::{Token, TokenKind};

/// Represents an internal type to simplify the code.
type LexResult<'a> = Result<Token<'a>, Error<'a>>;

/// Struct that transforms the input into a vector of tokens.
/// The `Lexer` operates on the slice of bytes to
/// diversify the possible sources of input to the program.
///
/// The `Lexer` follows a simple procedure: first, it gets
/// and decodes the next byte; then, consecutive bytes of
/// the same token type are grouped together; finally, the
/// `Token` is constructed.
pub struct Lexer<'a> {
    // The name of the file currently being analyzed.
    file_name: &'a str,
    // The bytes of the file being analyzed.
    bytes: &'a [u8],
    // The current index in the bytes slice.
    index: usize,
    // The current line in the input.
    // This, along with the current_column, is used to construct
    // `Span` information.
    current_line: usize,
    // The current column in the input.
    current_column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file_name: &'a str, bytes: &'a [u8]) -> Self {
        Self {
            file_name,
            bytes,
            index: 0,
            current_line: 1,
            current_column: 0,
        }
    }

    /// Performs the lexical analysis process and constructs a vector of tokens or
    /// a vector of errors. The `Interner` is used to cache different `String` literals
    /// and minimizes the memory used.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store different `String` literals.
    pub fn get_tokens(
        &mut self,
        interner: &mut Interner<String>,
    ) -> Result<Vec<Token<'a>>, Vec<Error<'a>>> {
        let mut tokens = vec![];
        let mut errors = vec![];
        while let Some(byte) = self.next() {
            match byte {
                whitespace if whitespace.is_ascii_whitespace() => {
                    tokens.push((
                        self.make_span(self.current_column),
                        TokenKind::Whitespace(whitespace as char),
                    ));
                    if whitespace == b'\n' {
                        self.current_line += 1;
                        self.current_column = 0;
                    }
                }
                b'-' if self.peek().map_or(false, |digit| digit.is_ascii_digit()) => {
                    let start_column = self.current_column;
                    let digit: i64 = (self.next().unwrap() - b'0').into();
                    match self.form_number(-digit, start_column) {
                        Ok(token) => tokens.push(token),
                        Err(error) => errors.push(error),
                    }
                }
                digit if digit.is_ascii_digit() => {
                    match self.form_number((digit - b'0').into(), self.current_column) {
                        Ok(token) => tokens.push(token),
                        Err(error) => errors.push(error),
                    }
                }
                b'\'' => {
                    match self.form_char() {
                        Ok(token) => tokens.push(token),
                        Err(error) => errors.push(error),
                    }
                }
                letter if letter.is_ascii_alphabetic() || letter == b'_' => {
                    match self.form_word(letter as char, interner) {
                        Ok(token) => tokens.push(token),
                        Err(error) => errors.push(error),
                    }
                }
                b'+' => tokens.push((self.make_span(self.current_column), TokenKind::Plus)),
                b'-' => tokens.push((self.make_span(self.current_column), TokenKind::Minus)),
                b'*' => tokens.push((self.make_span(self.current_column), TokenKind::Star)),
                b'/' => tokens.push((self.make_span(self.current_column), TokenKind::Slash)),
                b'%' => tokens.push((self.make_span(self.current_column), TokenKind::PercentSign)),
                b'!' if self.peek() == Some(b'=') => {
                    let start_column = self.current_column;
                    self.next();
                    tokens.push((
                        self.make_span(start_column),
                        TokenKind::ExclamationEqualSign,
                    ))
                }
                b'=' => tokens.push((self.make_span(self.current_column), TokenKind::EqualSign)),
                b'(' => tokens.push((
                    self.make_span(self.current_column),
                    TokenKind::LeftParenthesis,
                )),
                b')' => tokens.push((
                    self.make_span(self.current_column),
                    TokenKind::RightParenthesis,
                )),
                b'{' => tokens.push((
                    self.make_span(self.current_column),
                    TokenKind::LeftCurlyBrace,
                )),
                b'}' => tokens.push((
                    self.make_span(self.current_column),
                    TokenKind::RightCurlyBrace,
                )),
                b'<' if self.peek() == Some(b'=') => {
                    let start_column = self.current_column;
                    self.next();
                    tokens.push((self.make_span(start_column), TokenKind::LessThanEqualSign))
                }
                b'<' => tokens.push((
                    self.make_span(self.current_column),
                    TokenKind::LeftAngleBracket,
                )),
                b'>' if self.peek() == Some(b'=') => {
                    let start_column = self.current_column;
                    self.next();
                    tokens.push((
                        self.make_span(start_column),
                        TokenKind::GreaterThanEqualSign,
                    ))
                }
                b'>' => tokens.push((
                    self.make_span(self.current_column),
                    TokenKind::RightAngleBracket,
                )),
                b',' => tokens.push((self.make_span(self.current_column), TokenKind::Comma)),
                b':' if self.peek() == Some(b'=') => {
                    let start_column = self.current_column;
                    self.next();
                    tokens.push((self.make_span(start_column), TokenKind::ColonEqualSign))
                }
                b':' if self.peek() == Some(b':') => {
                    let start_column = self.current_column;
                    self.next();
                    tokens.push((self.make_span(start_column), TokenKind::ColonColon))
                }
                b':' => tokens.push((self.make_span(self.current_column), TokenKind::Colon)),
                b'\0' => break,
                _ => errors.push(Error::UnrecognizedCharacter(
                    self.make_span(self.current_column),
                )),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(tokens)
        }
    }

    /// Greedily walks through consecutive bytes and forms the largest possible number,
    /// either an int or a float. There are two possible errors that can occur: both involve
    /// forming a number that is greater than the highest possible value for the given type.
    ///
    /// # Arguments
    /// * `digit` - The first digit of the number.
    /// * `start_column` - The starting column of the number. This changes when dealing with negative numbers.
    fn form_number(&mut self, digit: i64, start_column: usize) -> LexResult<'a> {
        let mut number = digit.to_string();
        let mut seen_decimal_point = false;
        while let Some(next) = self.peek() {
            match next {
                digit if digit.is_ascii_digit() => {
                    number.push(digit.into());
                }
                b'.' if !seen_decimal_point => {
                    number.push('.');
                    seen_decimal_point = true;
                }
                _ => break,
            }

            self.next();
        }

        let span = self.make_span(start_column);
        if seen_decimal_point {
            match number.parse::<f64>() {
                Ok(float) => Ok((span, TokenKind::FloatLiteral(float))),
                Err(_) => Err(Error::FloatOverflow(span)),
            }
        } else {
            match number.parse::<i64>() {
                Ok(int) => Ok((span, TokenKind::IntegerLiteral(int))),
                Err(_) => Err(Error::IntegerOverflow(span)),
            }
        }
    }

    /// Walks through the character and ensures that exactly one character is represented.
    fn form_char(&mut self) -> LexResult<'a> {
        let (start_line, start_column) = (self.current_line, self.current_column);
        let ch = if let Some(ch) = self.next() {
            ch as char
        } else {
            return Err(Error::UnexpectedEndOfInput(Span::new(self.file_name, start_line, start_column, self.current_line, self.current_column)))
        };

        if let Some(b'\'') = self.next() {
            let span = Span::new(
            self.file_name,
            start_line,
            start_column,
            self.current_line,
            self.current_column,
        );
            Ok((span, TokenKind::CharLiteral(ch)))
        } else {
            return Err(Error::UnterminatedChar(Span::new(self.file_name, start_line, start_column, self.current_line, self.current_column)))
        }
    }

    /// Greedily walks through consecutive bytes and forms the largest possible word.
    /// This word may represent a type, a literal, or an identifier.
    ///
    /// # Arguments
    /// * `leter` - The character with which the word started with.
    /// * `interner` - The `Interner` which stores the different string literals.
    fn form_word(&mut self, letter: char, interner: &mut Interner<String>) -> LexResult<'a> {
        let start_column = self.current_column;
        let mut word = letter.to_string();
        while let Some(next) = self.peek() {
            if next.is_ascii_whitespace() {
                break;
            } else if !next.is_ascii_punctuation() || next == b'_' {
                word.push(self.next().unwrap() as char);
            } else {
                break;
            }
        }

        match word.as_str() {
            "Void" => Ok((self.make_span(start_column), TokenKind::Void)),
            "Int" => Ok((self.make_span(start_column), TokenKind::Int)),
            "Float" => Ok((self.make_span(start_column), TokenKind::Float)),
            "Boolean" => Ok((self.make_span(start_column), TokenKind::Boolean)),
            "Char" => Ok((self.make_span(start_column), TokenKind::Char)),
            "true" => Ok((
                self.make_span(start_column),
                TokenKind::BooleanLiteral(true),
            )),
            "false" => Ok((
                self.make_span(start_column),
                TokenKind::BooleanLiteral(false),
            )),
            "not" => Ok((self.make_span(start_column), TokenKind::Not)),
            "or" => Ok((self.make_span(start_column), TokenKind::Or)),
            "and" => Ok((self.make_span(start_column), TokenKind::And)),
            "let" => Ok((self.make_span(start_column), TokenKind::Let)),
            "if" => Ok((self.make_span(start_column), TokenKind::If)),
            "then" => Ok((self.make_span(start_column), TokenKind::Then)),
            "else" => Ok((self.make_span(start_column), TokenKind::Else)),
            "while" => Ok((self.make_span(start_column), TokenKind::While)),
            "define" => Ok((self.make_span(start_column), TokenKind::Define)),
            _ => {
                let id = interner.insert(word);
                Ok((self.make_span(start_column), TokenKind::Identifier(id)))
            }
        }
    }

    /// Peeks at the next byte without consuming it.
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    /// Consumes the next byte and increment both the index and
    /// the current column.
    fn next(&mut self) -> Option<u8> {
        self.index += 1;
        self.current_column += 1;
        self.bytes.get(self.index - 1).copied()
    }

    /// Helper method that creates a `Span` based on
    /// the start_column.
    ///
    /// # Arguments
    /// `start_column` - The starting column of the `Token`.
    fn make_span(&self, start_column: usize) -> Span<'a> {
        Span::new(
            self.file_name,
            self.current_line,
            start_column,
            self.current_line,
            self.current_column,
        )
    }
}
