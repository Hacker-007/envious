use crate::lex::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Power(u8);

impl Power {
    pub const MIN: Self = Self(0);
}

#[derive(Debug, Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct BindingPower {
    pub left: Power,
    pub right: Power,
}

impl BindingPower {
    pub fn new(precedence: u8, associativity: Associativity) -> Self {
        let base = precedence * 2;
        match associativity {
            Associativity::Left => Self {
                left: Power(base),
                right: Power(base + 1),
            },
            Associativity::Right => Self {
                left: Power(base + 1),
                right: Power(base),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfixOperator {
    Plus,
}

impl InfixOperator {
    pub fn from_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Plus => Some(Self::Plus),
            _ => None,
        }
    }

    pub fn bp(&self) -> BindingPower {
        match self {
            Self::Plus => BindingPower::new(1, Associativity::Left),
        }
    }
}
