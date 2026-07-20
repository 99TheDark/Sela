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
    pub(super) fn parse_func(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let name = if self.peek().is(TokenKind::Ident) {
            let ident_tok = self.next();
            Some(self.parse_ident(ident_tok))
        } else {
            None
        };

        let lparen =
            self.expect(TokenKind::LParen, name.map_or(tok.span, |n| tok.span.to(n.span)))?;
        let params = self.parse_parens(lparen);

        let ret = if self.peek().is(TokenKind::Arrow) {
            self.next();
            Some(self.parse_pre_body()?) // Might need to change?
        } else {
            None
        };

        let body = if self.peek().is(TokenKind::LBrace) {
            let lbrace_tok = self.next();
            Some(self.parse_block(lbrace_tok))
        } else {
            None
        };

        let sig = self.alloc_node(
            ast::NodeKind::FuncSig { params, ret },
            params.span.to(ret.map_or(params.span, |ret| ret.span)),
        );

        self.alloc_node(
            ast::NodeKind::Func { name, sig, body },
            tok.span.to(body.map_or(sig.span, |body| body.span)),
        )
    }
}
