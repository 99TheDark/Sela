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
    pub(super) fn parse_delimited<F: FnOnce(&'ast [ast::NodeRef<'ast>]) -> ast::NodeKind<'ast>>(
        &mut self,
        tok: Token,
        delim: TokenKind,
        end: TokenKind,
        constructor: F,
    ) -> PResult<'ast> {
        let mut elems = Vec::with_capacity(4);
        self.eat_nls();
        while self.peek().eof_not_is(end) {
            let expr = self.parse_expr(Precedence::None);
            elems.push(expr);
            self.eat_nls();

            if self.peek().kind == end {
                break;
            } else {
                self.expect(delim);
            }
        }
        let end = self.next();

        let elems = self.alloc(elems);
        let kind = constructor(&elems);
        self.alloc(ast::Node::new(kind, tok.span.to(end.span)))
    }

    pub(super) fn parse_block(&mut self, tok: Token) -> PResult<'ast> {
        let elems = self.parse_stmts(|tok| tok.is(TokenKind::RBrace));
        let elems = self.alloc(elems);
        let end = elems.last().map_or(tok.span, |last| last.span);
        self.alloc(ast::Node::new(ast::NodeKind::Block(elems), tok.span.to(end)))
    }
}
