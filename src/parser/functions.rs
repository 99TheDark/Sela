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
    pub(super) fn parse_func(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let name = if self.peek().is(TokenKind::Ident) {
            let ident_tok = self.next();
            Some(self.parse_ident(ident_tok))
        } else {
            None
        };

        let lparen = self.expect(TokenKind::LParen);
        let params = self.parse_delimited(
            lparen,
            TokenKind::Comma,
            TokenKind::RParen,
            ast::NodeKind::Parens,
        );

        let ret = if self.peek().is(TokenKind::Arrow) {
            self.next();
            Some(self.parse_expr(Precedence::None))
        } else {
            None
        };

        let body = if self.peek().is(TokenKind::LBrace) {
            let lbrace_tok = self.next();
            Some(self.parse_block(lbrace_tok))
        } else {
            None
        };

        let sig = self.alloc(ast::Node::new(
            ast::NodeKind::FuncSig { params, ret },
            params.span.to(ret.map_or(params.span, |ret| ret.span)),
        ));

        self.alloc(ast::Node::new(
            ast::NodeKind::Func { name, sig, body },
            tok.span.to(body.map_or(sig.span, |body| body.span)),
        ))
    }
}
