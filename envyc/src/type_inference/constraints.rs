use super::monotype::MonotypeRef;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Constraint {
    Equal(MonotypeRef, MonotypeRef),
}
