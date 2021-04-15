/// Enum that details the precedence for
/// different expressions. This is used in the
/// Pratt parsing algorithm utilized by the
/// `Parser`.
pub enum Precedence {
    Constant,
    Logic,
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
            Precedence::Logic => 2,
            Precedence::Comparison => 3,
            Precedence::Addition => 4,
            Precedence::Multiplication => 5,
            Precedence::Unary => 6,
            Precedence::Application => 7,
            Precedence::If => 8,
        }
    }
}
