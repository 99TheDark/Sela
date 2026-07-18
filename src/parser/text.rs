use crate::{
    ast,
    parser::{PResult, Parser},
    token::{Token, kind::TokenKind, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_char(&mut self, _tok: Token) -> PResult<'ast> {
        todo!()
    }

    pub(super) fn parse_string(&mut self, tok: Token) -> PResult<'ast> {
        // TODO: Unescape
        let mut frags = vec![ast::string::Fragment::String(tok.src(self.src).to_string())];
        let mut end = tok.span;
        while self.peek().eof_is(TokenKind::Dollar) {
            self.next();
            self.expect(TokenKind::LParen);
            let interp = self.parse_expr(Precedence::None);
            let rparen = self.expect(TokenKind::RParen);

            frags.push(ast::string::Fragment::Expr(interp));

            if self.peek().is(TokenKind::String) {
                // TODO: Same deal here
                let str_tok = self.next();
                frags.push(ast::string::Fragment::String(str_tok.src(self.src).to_string()));

                end = str_tok.span;
            } else {
                end = rparen.span;
            }
        }

        let frags = self.alloc(frags);
        self.alloc(ast::Node::new(ast::NodeKind::String(&frags), tok.span.to(end)))
    }
}
