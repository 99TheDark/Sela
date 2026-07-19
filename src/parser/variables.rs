use crate::{
    ast,
    parser::Parser,
    token::{Token, kind::TokenKind, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_decl(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let pat = self.parse_expr(Precedence::Assign)?;
        self.expect(TokenKind::Eq, tok.span.to(pat.span))?;
        let val = self.parse_expr(Precedence::Assign)?;
        self.alloc_node(ast::NodeKind::Decl { pat, val }, tok.span.to(val.span))
    }

    pub(super) fn parse_use(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let path = self.parse_expr(Precedence::None)?;
        self.alloc_node(ast::NodeKind::Use { path }, tok.span.to(path.span))
    }

    pub(super) fn parse_pair(&mut self, lhs: ast::NodeRef<'ast>, tok: Token) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.led_prec())?;
        self.alloc_node(ast::NodeKind::Pair { lhs, rhs }, lhs.span.to(rhs.span))
    }
}
