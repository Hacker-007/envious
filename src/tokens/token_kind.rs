//! The TokenKind enum maintains all of the different Tokens that could occur within the program.
//! Using an enum allows for easy extensibility.

use super::classification::Classification;

#[derive(Debug)]
pub enum TokenKind {
    Whitespace(String),

    Void,
    Any,
    Int,
    Float,
    Boolean,
    String,
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Identifier(String),

    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    Plus,
    Minus,
    Star,
    Slash,
    PercentSign,
    EqualSign,
    ColonEqualSign,
    ExclamationEqualSign,
    Comma,
    Colon,
    ColonColon,
    
    Not,
    Or,
    And,
    Let,
    If,
    Else,
    Define,
}

impl TokenKind {
    /// Gets the name of the token based on the kind.
    pub fn get_name(&self) -> String {
        match self {
            TokenKind::Whitespace(_) => "",
            TokenKind::Void => "Void",
            TokenKind::Any => "Any",
            TokenKind::Int => "Int",
            TokenKind::Float => "Float",
            TokenKind::Boolean => "Boolean",
            TokenKind::String => "String",
            TokenKind::IntegerLiteral(_) => "Int Literal",
            TokenKind::FloatLiteral(_) => "Float Literal",
            TokenKind::BooleanLiteral(_) => "Boolean Literal",
            TokenKind::StringLiteral(_) => "String Literal",
            TokenKind::Identifier(_) => "Identifier",

            TokenKind::LeftParenthesis => "Left Parenthesis",
            TokenKind::RightParenthesis => "Right Parenthesis",
            TokenKind::LeftCurlyBrace => "Left Curly Brace",
            TokenKind::RightCurlyBrace => "Right Curly Brace",
            TokenKind::Plus => "Plus Operator",
            TokenKind::Minus => "Minus Operator",
            TokenKind::Star => "Star Operator",
            TokenKind::Slash => "Slash Operator",
            TokenKind::PercentSign => "Percent Sign",
            TokenKind::EqualSign => "Equal Sign",
            TokenKind::ColonEqualSign => "Colon Equal Sign",
            TokenKind::ExclamationEqualSign => "Exclamation Equal Sign",
            TokenKind::Comma => "Comma",
            TokenKind::Colon => "Colon",
            TokenKind::ColonColon => "Colon Colon",
            
            TokenKind::Not => "Not",
            TokenKind::Or => "Or",
            TokenKind::And => "And",
            TokenKind::Let => "Let Keyword",
            TokenKind::If => "If Keyword",
            TokenKind::Else => "Else Keyword",
            TokenKind::Define => "Define Keyword",
        }
        .to_owned()
    }

    /// Gets the length of the token.
    pub fn get_length(&self) -> usize {
        match self {
            TokenKind::Whitespace(whitespace) => whitespace.len(),
            TokenKind::Void => 4,
            TokenKind::Any => 3,
            TokenKind::Int => 3,
            TokenKind::Float => 4,
            TokenKind::Boolean => 7,
            TokenKind::String => 6,
            TokenKind::IntegerLiteral(val) => val.to_string().len(),
            TokenKind::FloatLiteral(val) => val.to_string().len(),
            TokenKind::BooleanLiteral(val) => val.to_string().len(),
            TokenKind::StringLiteral(val) => val.len(),
            TokenKind::Identifier(name) => name.len(),
            
            TokenKind::LeftParenthesis => 1,
            TokenKind::RightParenthesis => 1,
            TokenKind::LeftCurlyBrace => 1,
            TokenKind::RightCurlyBrace => 1,
            TokenKind::Plus => 1,
            TokenKind::Minus => 1,
            TokenKind::Star => 1,
            TokenKind::Slash => 1,
            TokenKind::PercentSign => 1,
            TokenKind::EqualSign => 1,
            TokenKind::ColonEqualSign => 2,
            TokenKind::ExclamationEqualSign => 2,
            TokenKind::Comma => 1,
            TokenKind::Colon => 1,
            TokenKind::ColonColon => 2,

            TokenKind::Not => 3,
            TokenKind::Or => 2,
            TokenKind::And => 3,
            TokenKind::Let => 3,
            TokenKind::If => 2,
            TokenKind::Else => 4,
            TokenKind::Define => 6,
        }
    }
    
    /// Gets the classification of the token based on the kind.
    pub fn get_classification(&self) -> Classification {
        match self {
            TokenKind::Whitespace(whitespace) => Classification::Whitespace(whitespace.to_owned()),
            TokenKind::Void => Classification::Type("Void".to_owned()),
            TokenKind::Any => Classification::Type("Any".to_owned()),
            TokenKind::Int => Classification::Type("Int".to_owned()),
            TokenKind::Float => Classification::Type("Float".to_owned()),
            TokenKind::Boolean => Classification::Type("Boolean".to_owned()),
            TokenKind::String => Classification::Type("String".to_owned()),
            TokenKind::IntegerLiteral(val) => Classification::Values(val.to_string()),
            TokenKind::FloatLiteral(val) => Classification::Values(val.to_string()),
            TokenKind::BooleanLiteral(val) => Classification::Values(val.to_string()),
            TokenKind::StringLiteral(val) => Classification::Values(format!("\"{}\"", val)),
            TokenKind::Identifier(name) => Classification::Identifier(name.to_owned()),

            TokenKind::LeftParenthesis => Classification::Punctuation("(".to_owned()),
            TokenKind::RightParenthesis => Classification::Punctuation(")".to_owned()),
            TokenKind::LeftCurlyBrace => Classification::Punctuation("{".to_owned()),
            TokenKind::RightCurlyBrace => Classification::Punctuation("}".to_owned()),
            TokenKind::Plus => Classification::Punctuation("+".to_owned()),
            TokenKind::Minus => Classification::Punctuation("-".to_owned()),
            TokenKind::Star => Classification::Punctuation("*".to_owned()),
            TokenKind::Slash => Classification::Punctuation("/".to_owned()),
            TokenKind::PercentSign => Classification::Punctuation("%".to_owned()),
            TokenKind::EqualSign => Classification::Punctuation("=".to_owned()),
            TokenKind::ColonEqualSign => Classification::Punctuation(":=".to_owned()),
            TokenKind::ExclamationEqualSign => Classification::Punctuation("!=".to_owned()),
            TokenKind::Comma => Classification::Punctuation(",".to_owned()),
            TokenKind::Colon => Classification::Punctuation(":".to_owned()),
            TokenKind::ColonColon => Classification::Punctuation("::".to_owned()),
            
            TokenKind::Not => Classification::Keyword("not".to_owned()),
            TokenKind::Or => Classification::Keyword("or".to_owned()),
            TokenKind::And => Classification::Keyword("and".to_owned()),
            TokenKind::Let => Classification::Keyword("let".to_owned()),
            TokenKind::If => Classification::Keyword("if".to_owned()),
            TokenKind::Else => Classification::Keyword("else".to_owned()),
            TokenKind::Define => Classification::Keyword("define".to_owned()),
        }
    }
}