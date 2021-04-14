/// Enum that details the precedence for
/// different expressions. This is used in the
/// Pratt parsing algorithm utilized by the
/// `Parser`.
pub enum Precedence {
    Constant,
    Comparison,
    Addition,
    Multiplication,
    Unary,
    Application,
    If,
}

impl From<Precedence> for usize {
    fn from(precedence: Precedence) -> Self {
        match precedence {
            Precedence::Constant => 1,
            Precedence::Comparison => 2,
            Precedence::Addition => 3,
            Precedence::Multiplication => 4,
            Precedence::Unary => 5,
            Precedence::Application => 6,
            Precedence::If => 7,
        }
    }
}
