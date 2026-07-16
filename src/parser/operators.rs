use crate::{
    ast::{self, binary::BinaryKind, unop::UnOpKind},
    parser::Parser,
    token::Token,
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_binary(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
        binary: BinaryKind,
    ) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.led_prec());
        binary.make_node(lhs, rhs, self.arena)
    }

    pub(super) fn parse_unop(&mut self, tok: Token, op: UnOpKind) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.nud_prec());
        let kind = ast::NodeKind::UnOp { op, rhs };
        self.alloc(ast::Node::new(kind, tok.span.to(rhs.span)))
        // let node = self.reserve::<ast::Node>();
        // let rhs = self.parse_expr(tok.nud_prec());
        // let kind = ast::NodeKind::UnOp { op, rhs };
        // self.fill(node, ast::Node::new(kind, tok.span.to(rhs.span)))
    }
}
