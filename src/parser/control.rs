use crate::{
    ast,
    parser::Parser,
    token::{Token, kind::TokenKind},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    #[inline]
    pub(super) fn parse_loop(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let lbrace = self.expect(TokenKind::LBrace, || tok.span)?;
        let body = self.parse_block(lbrace);
        self.alloc_node(ast::NodeKind::Loop { body }, tok.span.to(body.span))
    }

    #[inline]
    pub(super) fn parse_while(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let cond = self.parse_pre_body()?;
        let lbrace = self.expect(TokenKind::LBrace, || tok.span.to(cond.span))?;
        let body = self.parse_block(lbrace);
        self.alloc_node(ast::NodeKind::While { cond, body }, tok.span.to(body.span))
    }

    #[inline]
    pub(super) fn parse_for(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let clause = self.parse_pre_body()?;
        let lbrace = self.expect(TokenKind::LBrace, || tok.span.to(clause.span))?;
        let body = self.parse_block(lbrace);
        self.alloc_node(ast::NodeKind::For { clause, body }, tok.span.to(body.span))
    }

    #[inline]
    pub(super) fn parse_if(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let cond = self.parse_pre_body()?;
        let lbrace = self.expect(TokenKind::LBrace, || tok.span.to(cond.span))?;
        let body = self.parse_block(lbrace);
        let fallback = if self.peek().is(TokenKind::Else) {
            let else_kw = self.next();
            let else_lbrace = self.expect(TokenKind::LBrace, || tok.span.to(else_kw.span))?;
            Some(self.parse_block(else_lbrace))
        } else {
            None
        };
        self.alloc_node(ast::NodeKind::If { cond, body, fallback }, tok.span.to(body.span))
    }
}
