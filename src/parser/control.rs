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
    #[inline]
    pub(super) fn parse_loop(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let lbrace = self.expect(TokenKind::LBrace);
        let body = self.parse_block(lbrace);
        self.alloc(ast::Node::new(ast::NodeKind::Loop { body }, tok.span.to(body.span)))
    }

    #[inline]
    pub(super) fn parse_while(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let cond = self.parse_expr(Precedence::None);
        let lbrace = self.expect(TokenKind::LBrace);
        let body = self.parse_block(lbrace);
        self.alloc(ast::Node::new(ast::NodeKind::While { cond, body }, tok.span.to(body.span)))
    }

    #[inline]
    pub(super) fn parse_for(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let vari = self.parse_expr(Precedence::None);
        self.expect(TokenKind::In);
        let iter = self.parse_expr(Precedence::None);
        let lbrace = self.expect(TokenKind::LBrace);
        let body = self.parse_block(lbrace);
        self.alloc(ast::Node::new(ast::NodeKind::For { vari, iter, body }, tok.span.to(body.span)))
    }

    #[inline]
    pub(super) fn parse_if(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let cond = self.parse_expr(Precedence::None);
        let lbrace = self.expect(TokenKind::LBrace);
        let body = self.parse_block(lbrace);
        let fallback = if self.peek().is(TokenKind::Else) {
            self.next();
            let else_lbrace = self.expect(TokenKind::LBrace);
            Some(self.parse_block(else_lbrace))
        } else {
            None
        };
        self.alloc(ast::Node::new(
            ast::NodeKind::If { cond, body, fallback },
            tok.span.to(body.span),
        ))
    }
}
