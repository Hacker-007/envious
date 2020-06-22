//! The Lexer struct tokenizes the input and returns a VecDeque of Tokens
//! The lexer may prematurely return an error if it can not parse a specific character.
//!
//! The lexer must be the first thing that is invoked because it generates the tokens necessary for the compilation stage.
//!
//! # Example
//! ```
//! # fn run() -> Result<(), Error> {
//! let contents = "print(1)";
//! let tokens = Lexer::new().lex(contents)?;
//! # Ok(())
//! # }
//! ```

use crate::{
    errors::{error::Error, error_kind::ErrorKind},
    tokens::{token::Token, token_kind::TokenKind},
};
use std::{collections::VecDeque, iter::Peekable, str::Chars};

pub struct Lexer {
    current_position: usize,
}

impl Lexer {
    /// Constructs a new lexer with the current position set to 0 (the first token).
    pub fn new() -> Lexer {
        Lexer {
            current_position: 0,
        }
    }

    /// This function lexes the input and returns either a VecDeque of tokens or an error.
    /// The return value of this function may change to returning a vector of errors.
    ///
    /// # Arguments
    /// * `contents` - The contents to lex. This will usually come from a file or from the REPL.
    pub fn lex(&mut self, contents: &str) -> Result<VecDeque<Token>, Error> {
        let mut iter = contents.chars().peekable();
        let mut tokens = VecDeque::new();
        while let Some(ch) = iter.next() {
            self.current_position += 1;

            // If the current character is a whitespace or a comment, handle it, and continue lexing.
            if ch.is_ascii_whitespace() || self.handle_comments(ch, &mut iter) {
                continue;
            }

            // Identify what the character is and try to lex as much of it as possible.
            match ch {
                '-' if iter.peek().map_or(false, |ch| ch.is_ascii_digit()) => tokens.push_back(self.make_number(ch, &mut iter)?),
                '0'..='9' => tokens.push_back(self.make_number(ch, &mut iter)?),
                '\'' | '"' => tokens.push_back(self.make_string(ch, &mut iter)?),
                '+' => tokens.push_back(Token::new(TokenKind::Plus, self.current_position)),
                '-' => tokens.push_back(Token::new(TokenKind::Minus, self.current_position)),
                '*' => tokens.push_back(Token::new(TokenKind::Star, self.current_position)),
                '/' => tokens.push_back(Token::new(TokenKind::Slash, self.current_position)),
                '=' => tokens.push_back(Token::new(TokenKind::EqualSign, self.current_position)),
                '(' => tokens.push_back(Token::new(TokenKind::LeftParenthesis, self.current_position)),
                ')' => tokens.push_back(Token::new(TokenKind::RightParenthesis, self.current_position)),
                ':' => tokens.push_back(Token::new(TokenKind::Colon, self.current_position)),
                letter if ch.is_ascii_alphabetic() => {
                    tokens.push_back(self.make_word(letter, &mut iter))
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::UnknownCharacter,
                        self.current_position,
                    ))
                }
            }
        }

        Ok(tokens)
    }

    /// This function produces an int, a float, or an error.
    ///
    /// # Arguments
    /// * `digit` - The first character of the number. This may also be a negative sign.
    /// * `iter` - The iterator which contains all of the characters.
    fn make_number(&mut self, digit: char, iter: &mut Peekable<Chars>) -> Result<Token, Error> {
        let initial_point = self.current_position;
        let mut number = digit.to_string();
        let mut has_decimal_point = false;
        while let Some(ch) = iter.peek() {
            // After the value of the character has been identified, it is important to remember to advance the iterator.
            // Otherwise, an infinite loop will be generated.
            if ch.is_ascii_digit() {
                number.push(self.advance(iter));
            } else if ch == &'.' && !has_decimal_point {
                number.push(self.advance(iter));
                has_decimal_point = true;
            } else {
                break;
            }
        }

        // If it does not have a decimal point, it must be an integer.
        if !has_decimal_point {
            if let Ok(value) = number.parse() {
                Ok(Token::new(TokenKind::IntegerLiteral(value), initial_point))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidNumberFormat,
                    self.current_position,
                ))
            }
        } else {
            if let Ok(value) = number.parse() {
                Ok(Token::new(TokenKind::FloatLiteral(value), initial_point))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidNumberFormat,
                    self.current_position,
                ))
            }
        }
    }

    /// This function produces an instruction, identifier, a special value, or a boolean. This funtion always succeeds because a word is always an identifier.
    ///
    /// # Arguments
    /// * `letter` - The first letter of the word.
    /// * `iter` - The iterator which contains all of the characters.
    fn make_word(&mut self, letter: char, iter: &mut Peekable<Chars>) -> Token {
        let initial_point = self.current_position;
        let mut word = letter.to_string();
        while let Some(ch) = iter.peek() {
            if ch.is_ascii_whitespace() {
                self.advance(iter);
                break;
            } else if !ch.is_ascii_punctuation() {
                word.push(self.advance(iter));
            } else {
                break;
            }
        }

        match word.to_ascii_lowercase().as_str() {
            "void" => Token::new(TokenKind::Void, initial_point),
            "any" => Token::new(TokenKind::Any, initial_point),
            "int" => Token::new(TokenKind::Int, initial_point),
            "float" => Token::new(TokenKind::Float, initial_point),
            "boolean" => Token::new(TokenKind::Boolean, initial_point),
            "string" => Token::new(TokenKind::String, initial_point),
            "true" => Token::new(TokenKind::BooleanLiteral(true), initial_point),
            "false" => Token::new(TokenKind::BooleanLiteral(false), initial_point),
            "let" => Token::new(TokenKind::Let, initial_point),
            _ => {
                if let Some(builtin_function) = TokenKind::is_builtin(word.as_str()) {
                    Token::new(builtin_function, initial_point)
                } else {
                    Token::new(TokenKind::Identifier(word), initial_point)
                }
            }
        }
    }

    /// This function produces a string or an error.
    ///
    /// # Arguments
    /// * `beginning_of_string` - The first opening quote used to begin the string. This could be ' or ".
    /// * `iter` - The iterator which contains all of the characters.
    fn make_string(
        &mut self,
        beginning_of_string: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<Token, Error> {
        let initial_point = self.current_position;
        let mut string = String::new();
        let mut is_terminated = false;
        while let Some(ch) = iter.peek() {
            if ch == &beginning_of_string {
                self.advance(iter);
                is_terminated = true;
                break;
            } else {
                string.push(self.advance(iter));
            }
        }

        // If the string does not end with the same quote used to open it, the function returns an error.
        if !is_terminated {
            Err(Error::new(ErrorKind::UnterminatedString, initial_point))
        } else {
            Ok(Token::new(TokenKind::StringLiteral(string), initial_point))
        }
    }

    /// This function handles comments. This function returns whether or not it found a commment and handled it.
    ///
    /// # Arguments
    /// * `ch` - The current character the lexer is looking at.
    /// * `iter` - The iterator which contains all of the characters.
    fn handle_comments(&mut self, ch: char, iter: &mut Peekable<Chars>) -> bool {
        if ch == '-' {
            match iter.peek() {
                Some('-') => {
                    self.handle_single_line_comments(iter);
                    true
                }
                Some('!') => {
                    self.handle_multi_line_comments(iter);
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// This function handles single line comments.
    ///
    /// # Arguments
    /// * `iter` - The iterator which contains all of the characters.
    fn handle_single_line_comments(&mut self, iter: &mut Peekable<Chars>) {
        self.advance(iter);
        while let Some(c) = iter.next() {
            self.current_position += 1;
            if c == '\n' {
                break;
            }
        }
    }

    /// This function handles multiline comments.
    ///
    /// # Arguments
    /// * `iter` - The iterator which contains all of the characters.
    fn handle_multi_line_comments(&mut self, iter: &mut Peekable<Chars>) {
        self.advance(iter);
        while let Some(c) = iter.next() {
            self.current_position += 1;
            if c == '!' {
                if let Some('-') = iter.peek() {
                    self.advance(iter);
                    break;
                }
            }
        }
    }

    /// This function increments the current position and returns the next character.
    /// The bounds check was already performed by the loops, so there is no need to return an option.
    ///
    /// # Arguments
    /// * `iter` - The iterator which contains all of the characters.
    fn advance(&mut self, iter: &mut Peekable<Chars>) -> char {
        self.current_position += 1;
        iter.next().unwrap()
    }
}
