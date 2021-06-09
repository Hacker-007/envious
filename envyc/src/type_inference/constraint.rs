use super::monotype::MonotypeRef;

#[derive(Debug)]
pub enum Constraint<'a> {
    None(MonotypeRef<'a>),
    Or(Vec<MonotypeRef<'a>>),
}