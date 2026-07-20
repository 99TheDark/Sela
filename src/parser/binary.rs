use crate::{
    ast::{self, binary::BinaryKind},
    parser::Parser,
    token::{Token, precedence::Precedence},
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
        let rhs = self.parse_expr(tok.led_prec())?;
        binary.make_node(lhs, rhs, self.arena) // Maybe move out of binary
    }

    pub(super) fn parse_in(&mut self, lhs: ast::NodeRef<'ast>) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(Precedence::Relat)?;
        self.alloc_node(ast::NodeKind::In { vari: lhs, iter: rhs }, lhs.span.to(rhs.span))
    }
}
