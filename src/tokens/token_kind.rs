//! The TokenKind enum maintains all of the different Tokens that could occur within the program.
//! Using an enum allows for easy extensibility.

#[derive(Debug)]
pub enum TokenKind {
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
    EqualSign,
    ColonEqualSign,
    Comma,
    Colon,
    ColonColon,

    Let,
    If,
    Else,
}

impl TokenKind {
    /// Gets the name of the token based on the kind.
    pub fn get_name(&self) -> String {
        match self {
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
            TokenKind::EqualSign => "Equal Sign",
            TokenKind::ColonEqualSign => "Colon Equal Sign",
            TokenKind::Comma => "Comma",
            TokenKind::Colon => "Colon",
            TokenKind::ColonColon => "Colon Colon",

            TokenKind::Let => "Let Keyword",
            TokenKind::If => "If Keyword",
            TokenKind::Else => "Else Keyword",
        }
        .to_owned()
    }
}
