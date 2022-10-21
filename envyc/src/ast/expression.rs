pub(crate) struct Expression {
    kind: ExpressionKind,
}

pub(crate) enum ExpressionKind {
    Integer(i64),
}