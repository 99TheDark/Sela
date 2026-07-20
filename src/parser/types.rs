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
    pub(super) fn parse_type(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let new = self.parse_expr(Precedence::Assign)?;
        self.expect(TokenKind::Eq, tok.span.to(new.span))?;
        let base = self.parse_expr(Precedence::Assign)?;
        self.alloc_node(ast::NodeKind::Type { new, base }, tok.span.to(base.span))
    }

    pub(super) fn parse_alias(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let alt = self.parse_expr(Precedence::Assign)?;
        self.expect(TokenKind::Eq, tok.span.to(alt.span))?;
        let src = self.parse_expr(Precedence::Assign)?;
        self.alloc_node(ast::NodeKind::Alias { alt, src }, tok.span.to(src.span))
    }
}
