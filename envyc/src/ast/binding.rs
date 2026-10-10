use std::fmt::Display;

use crate::token::TokenKind;

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

    pub fn left(precedence: u8) -> Self {
        Self {
            left: Power(precedence),
            right: Power(0),
        }
    }

    pub fn right(precedence: u8) -> Self {
        Self {
            left: Power(0),
            right: Power(precedence),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOperator {
    Negate,
    BitwiseComplement,
}

impl PrefixOperator {
    pub fn from_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Minus => Some(Self::Negate),
            _ => None,
        }
    }

    pub fn bp(&self) -> BindingPower {
        match self {
            Self::Negate => BindingPower::right(7),
            Self::BitwiseComplement => BindingPower::right(7),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfixOperator {
    Multiply,
    Divide,
    Plus,
    Minus,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}

impl InfixOperator {
    pub fn from_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Asterisk => Some(Self::Multiply),
            TokenKind::ForwardSlash => Some(Self::Divide),
            TokenKind::Plus => Some(Self::Plus),
            TokenKind::Minus => Some(Self::Minus),
            TokenKind::LeftShift => Some(Self::LeftShift),
            TokenKind::RightShift => Some(Self::RightShift),
            TokenKind::Ampersand => Some(Self::BitwiseAnd),
            TokenKind::Caret => Some(Self::BitwiseXor),
            TokenKind::Bar => Some(Self::BitwiseOr),
            _ => None,
        }
    }

    pub fn bp(&self) -> BindingPower {
        match self {
            Self::LeftShift => BindingPower::new(6, Associativity::Left),
            Self::RightShift => BindingPower::new(6, Associativity::Left),
            Self::BitwiseAnd => BindingPower::new(5, Associativity::Left),
            Self::BitwiseXor => BindingPower::new(4, Associativity::Left),
            Self::BitwiseOr => BindingPower::new(3, Associativity::Left),
            Self::Multiply => BindingPower::new(2, Associativity::Left),
            Self::Divide => BindingPower::new(2, Associativity::Left),
            Self::Plus => BindingPower::new(1, Associativity::Left),
            Self::Minus => BindingPower::new(1, Associativity::Left),
        }
    }
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::LeftShift => write!(f, "<<"),
            Self::RightShift => write!(f, ">>"),
            Self::BitwiseAnd => write!(f, "&"),
            Self::BitwiseXor => write!(f, "^"),
            Self::BitwiseOr => write!(f, "|"),
        }
    }
}
