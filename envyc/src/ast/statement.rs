

use crate::location::Location;

use super::expression::Expression;

pub(crate) struct Statement {
    location: Location,
    kind: StatementKind,
}

pub(crate) enum StatementKind {
    Semi(Expression)
}

