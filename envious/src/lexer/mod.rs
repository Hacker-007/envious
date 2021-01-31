pub mod token;

use crate::{error::Error, interner::Interner, error::Span};

use self::token::{Token, TokenKind};

/// Represents an internal type to simplify the code.
type LexResult = Result<Token, Error>;

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
    file_name: String,
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
    pub fn new(file_name: String, bytes: &'a [u8]) -> Self {
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
    pub fn get_tokens(&mut self, interner: &mut Interner<String>) -> (Vec<Token>, Vec<Error>) {
        let mut tokens = vec![];
        let mut errors = vec![];
        while let Some(byte) = self.next() {
            match byte {
                whitespace if whitespace.is_ascii_whitespace() => {
                    tokens.push((self.make_span(self.current_column), TokenKind::Whitespace));
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
                string_start @ b'\'' | string_start @ b'"' => {
                    match self.form_string(string_start, interner) {
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

        (tokens, errors)
    }

    /// Greedily walks through consecutive bytes and forms the largest possible number,
    /// either an int or a float. There are two possible errors that can occur: both involve
    /// forming a number that is greater than the highest possible value for the given type.
    ///
    /// # Arguments
    /// * `digit` - The first digit of the number.
    /// * `start_column` - The starting column of the number. This changes when dealing with negative numbers.
    fn form_number(&mut self, digit: i64, start_column: usize) -> LexResult {
        let mut number = digit;
        let mut floating_point: Option<i64> = None;
        while let Some(next) = self.peek() {
            match next {
                digit if digit.is_ascii_digit() => {
                    if let Some(decimals) = floating_point {
                        floating_point = Some(
                            decimals
                                .checked_mul(10)
                                .ok_or_else(|| Error::FloatOverflow(self.make_span(start_column)))?
                                .checked_add((digit - b'0').into())
                                .ok_or_else(|| {
                                    Error::FloatOverflow(self.make_span(start_column))
                                })?,
                        );
                    } else {
                        number = number
                            .checked_mul(10)
                            .ok_or_else(|| Error::IntegerOverflow(self.make_span(start_column)))?
                            .checked_add((digit - b'0').into())
                            .ok_or_else(|| Error::IntegerOverflow(self.make_span(start_column)))?;
                    }
                }
                b'.' if floating_point.is_none() => {
                    floating_point = Some(0);
                }
                _ => break,
            }

            self.next();
        }

        let span = self.make_span(start_column);
        if let Some(decimals) = floating_point {
            let float = format!("{}.{}", number, decimals).parse();
            match float {
                Ok(float) => Ok((span, TokenKind::FloatLiteral(float))),
                Err(_) => Err(Error::FloatOverflow(span)),
            }
        } else {
            Ok((span, TokenKind::IntegerLiteral(number)))
        }
    }

    /// Greedily walks through consecutive bytes and forms the largest possible string.
    /// A string can start with either a ' or a ". However, the string must end with the same
    /// character that it started with.
    ///
    /// # Arguments
    /// * `string_start` - The character with which the string started with.
    /// * `interner` - The `Interner` which stores the different string literals.
    fn form_string(&mut self, string_start: u8, interner: &mut Interner<String>) -> LexResult {
        let (start_line, start_column) = (self.current_line, self.current_column);
        let mut string = String::new();
        let mut is_terminated = false;
        while let Some(next) = self.peek() {
            if next == string_start {
                self.next();
                is_terminated = true;
                break;
            } else {
                string.push(next as char);
                self.next();
            }
        }

        let span = Span::new(
            self.file_name.clone(),
            start_line,
            start_column,
            self.current_line,
            self.current_column,
        );
        if !is_terminated {
            Err(Error::UnterminatedString(span))
        } else {
            let id = interner.insert(string);
            Ok((span, TokenKind::StringLiteral(id)))
        }
    }

    /// Greedily walks through consecutive bytes and forms the largest possible word.
    /// This word may represent a type, a literal, or an identifier.
    ///
    /// # Arguments
    /// * `leter` - The character with which the word started with.
    /// * `interner` - The `Interner` which stores the different string literals.
    fn form_word(&mut self, letter: char, interner: &mut Interner<String>) -> LexResult {
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
            "Any" => Ok((self.make_span(start_column), TokenKind::Any)),
            "Int" => Ok((self.make_span(start_column), TokenKind::Int)),
            "Float" => Ok((self.make_span(start_column), TokenKind::Float)),
            "Boolean" => Ok((self.make_span(start_column), TokenKind::Boolean)),
            "String" => Ok((self.make_span(start_column), TokenKind::String)),
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
    fn make_span(&self, start_column: usize) -> Span {
        Span::new(
            self.file_name.clone(),
            self.current_line,
            start_column,
            self.current_line,
            self.current_column,
        )
    }
}
