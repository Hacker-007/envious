/// Enum that details the precedence for
/// different expressions. This is used in the
/// Pratt parsing algorithm utilized by the 
/// `Parser`.
pub enum Precedence {
    Constant,
    Addition,
    Multiplication,
    Unary,
    If,
}

impl From<Precedence> for usize {
    fn from(precedence: Precedence) -> Self {
        match precedence {
            Precedence::Constant => 1,
            Precedence::Addition => 2,
            Precedence::Multiplication => 3,
            Precedence::Unary => 4,
            Precedence::If => 5,
        }
    }
}
