use crate::{
    ast::{self, unop::UnOpKind, vis::VisKind},
    parser::Parser,
    token::{Token, kind::TokenKind, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_ref(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        if self.peek_next().is(TokenKind::Annot) {
            let src = self.true_next().span.shrink(1, 0).src(self.src);
            let rhs = self.parse_expr(Precedence::Unary);
            self.alloc_node(ast::NodeKind::Life(src, rhs), tok.span.to(rhs.span))
        } else {
            self.parse_unop(tok, UnOpKind::Ref)
        }
    }

    pub(super) fn parse_mut(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let inner = self.parse_expr(Precedence::Unary)?;
        let kind = ast::NodeKind::Mut(inner);
        self.alloc_node(kind, tok.span.to(inner.span))
    }

    pub(super) fn parse_vis(&mut self, tok: Token, modif: VisKind) -> ast::NodeRef<'ast> {
        let child = self.parse_expr(Precedence::Unary)?;
        let kind = ast::NodeKind::Vis { modif, child };
        self.alloc_node(kind, tok.span.to(child.span))
    }

    pub(super) fn parse_unop(&mut self, tok: Token, op: UnOpKind) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(Precedence::Unary)?;
        let kind = ast::NodeKind::UnOp { op, rhs };
        self.alloc_node(kind, tok.span.to(rhs.span))
    }
}
