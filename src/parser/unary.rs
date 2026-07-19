use crate::{
    ast::{self, unop::UnOpKind},
    parser::Parser,
    token::{Token, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_mut(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let inner = self.parse_expr(Precedence::Unary)?;
        let kind = ast::NodeKind::Mut(inner);
        self.alloc_node(kind, tok.span.to(inner.span))
    }

    pub(super) fn parse_unop(&mut self, tok: Token, op: UnOpKind) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(Precedence::Unary)?;
        let kind = ast::NodeKind::UnOp { op, rhs };
        self.alloc_node(kind, tok.span.to(rhs.span))
    }
}
