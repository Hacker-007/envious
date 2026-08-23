use crate::{
    ast::{
        binding::{Associativity, BindingPower, InfixOperator, Power},
        Ast, AstBuilder, Expression, ExpressionIndex, Literal,
    },
    diagnostics::{Diagnostic, DiagnosticBag},
    lex::token::{
        buffer::{TokenIndex, TokenizedBuffer},
        TokenKind,
    },
    source::SourceId,
};

#[derive(Debug)]
pub struct Parser<'a> {
    idx: TokenIndex,
    source: SourceId,
    buffer: &'a TokenizedBuffer,
    ast: AstBuilder,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> Parser<'a> {
    pub fn new(
        source: SourceId,
        buffer: &'a TokenizedBuffer,
        diagnostics: &'a mut DiagnosticBag,
    ) -> Self {
        Self {
            idx: TokenIndex::ZERO,
            source,
            buffer,
            ast: AstBuilder::default(),
            diagnostics,
        }
    }

    pub fn parse(mut self) -> Ast {
        let root = self.parse_expression(Power::MIN);
        if self.expect(TokenKind::EndOfFile).is_none() {
            debug_assert!(false, "parser advanced beyond end-of-file token");
        }

        self.ast.finish(root)
    }

    fn parse_expression(&mut self, minimum: Power) -> ExpressionIndex {
        // TODO: implement Pratt parsing for expressions.
        let mut lhs = self.parse_primary();
        while let Some(operator) = InfixOperator::from_kind(self.current()) {
            let bp = operator.bp();
            if bp.left < minimum {
                break;
            }

            let token = self.bump();
            let rhs = self.parse_expression(bp.right);
            lhs = self.ast.allocate_binary((operator, token), lhs, rhs);
        }

        lhs
    }

    fn parse_primary(&mut self) -> ExpressionIndex {
        if let Some(token) = self.eat(TokenKind::IntLiteral) {
            return self.ast.allocate_literal(Literal::Integer(token));
        }

        self.bump();
        self.diagnostics.expected_expression(self.source, self.idx);
        self.ast.allocate_error()
    }

    fn is_at(&self, kind: TokenKind) -> bool {
        self.current() == kind
    }

    #[inline]
    fn current(&self) -> TokenKind {
        self.nth(0)
    }

    fn nth(&self, distance: u32) -> TokenKind {
        self.buffer
            .kind_at(self.idx.advance(distance))
            .unwrap_or(TokenKind::EndOfFile)
    }

    fn bump(&mut self) -> TokenIndex {
        let current = self.idx;
        if !self.is_at(TokenKind::EndOfFile) {
            self.idx = self.idx.advance(1);
        }

        current
    }

    fn eat(&mut self, kind: TokenKind) -> Option<TokenIndex> {
        self.is_at(kind).then(|| self.bump())
    }

    fn expect(&mut self, kind: TokenKind) -> Option<TokenIndex> {
        if let Some(idx) = self.eat(kind) {
            return Some(idx);
        }

        self.diagnostics.expected_token(self.source, self.idx, kind);
        None
    }
}
