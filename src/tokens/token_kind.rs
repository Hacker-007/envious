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
    Plus,
    Minus,
    Star,
    Slash,
    EqualSign,
    Colon,
    
    Let,

    BuiltInFunction(String),
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
            TokenKind::Plus => "Plus Operator",
            TokenKind::Minus => "Minus Operator",
            TokenKind::Star => "Star Operator",
            TokenKind::Slash => "Slash Operator",
            TokenKind::EqualSign => "Equal Sign",
            TokenKind::Colon => "Colon",

            TokenKind::Let => "Let Keyword",

            TokenKind::BuiltInFunction(function_name) => return format!("<builtin function '{}'>", function_name)
        }.to_owned()
    }

    /// Checks if the name is a builtin function.
    ///
    /// # Arguments
    /// `name` - The name of the token to check.
    pub fn is_builtin(name: &str) -> Option<TokenKind> {
        match name.to_ascii_lowercase().as_str() {
            "print" => Some(TokenKind::BuiltInFunction("print".to_owned())),
            _ => None,
        }
    }
}
