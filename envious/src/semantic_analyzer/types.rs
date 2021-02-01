/// Enum that represents the different types of the
/// expressions. There is no current support for
/// generics. However, there is a plan to implement
/// this in the future.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Void,
    Int,
    Float,
    Boolean,
    String,
}
