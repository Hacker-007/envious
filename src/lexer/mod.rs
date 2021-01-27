pub mod token;

use crate::{error::Error, span::Span};

use self::token::{Token, TokenKind};

type LexResult = Result<Token, Error>;

pub struct Lexer<'a> {
    file_name: String,
    bytes: &'a [u8],
    index: usize,
    current_line: usize,
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

    pub fn get_tokens(&mut self) -> (Vec<Token>, Vec<Error>) {
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
                    match self.form_string(string_start) {
                        Ok(token) => tokens.push(token),
                        Err(error) => errors.push(error),
                    }
                }
                letter if letter.is_ascii_alphabetic() || letter == b'_' => {
                    match self.form_word(letter as char) {
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

    fn form_string(&mut self, string_start: u8) -> LexResult {
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
            // Ok((span, TokenKind::StringLiteral(string)))
            Ok((span, TokenKind::StringLiteral(1)))
        }
    }

    fn form_word(&mut self, letter: char) -> LexResult {
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
            // _ => Ok((self.make_span(start_column), TokenKind::Identifier(word))),
            _ => Ok((self.make_span(start_column), TokenKind::Identifier(1))),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    fn next(&mut self) -> Option<u8> {
        self.index += 1;
        self.current_column += 1;
        self.bytes.get(self.index - 1).copied()
    }

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
