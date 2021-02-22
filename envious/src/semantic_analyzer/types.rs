use std::fmt::Display;

/// Enum that represents the different types of the
/// expressions. There is no current support for
/// generics. However, there is a plan to implement
/// this in the future.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Int,
    Float,
    Boolean,
    String,
    Function {
        arg_types: Vec<Type>,
        return_type: Box<Type>,
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "Void"),
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Boolean => write!(f, "Boolean"),
            Type::String => write!(f, "String"),
            Type::Function { .. } => write!(f, "Function"),
        }
    }
}