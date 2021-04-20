use std::fmt::Display;

/// Enum that represents the different types of the
/// expressions. There is no current support for
/// generics. However, there is a plan to implement
/// this in the future.
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Void,
    Int,
    Float,
    Boolean,
    Char,
    Never,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Void, Type::Void)
            | (Type::Never, Type::Void)
            | (Type::Void, Type::Never)
            | (Type::Never, Type::Never)
            | (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::Boolean, Type::Boolean)
            | (Type::Char, Type::Char) => true,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "Void"),
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Char => write!(f, "Char"),
            Type::Never => write!(f, "Never"),
        }
    }
}
