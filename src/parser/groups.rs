use crate::{
    ast,
    diagnostics::ErrorKind,
    parser::Parser,
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
    ) -> ast::NodeRef<'ast> {
        let mut elems = Vec::with_capacity(1); // Could make the capacity a parameter
        self.eat_nls();
        while self.peek().eof_not_is(end) {
            let elem = self.parse_expr(Precedence::None);
            self.recover_if_error(elem, |t| t == delim || t == end);
            elems.push(elem);

            self.eat_nls();
            match self.peek() {
                t if t.kind == end => break,
                t if t.kind == delim => {
                    self.true_next();
                }
                t => {
                    self.diag.emit(
                        ErrorKind::Syntax,
                        format!(
                            "Expected {:?} or {:?} token, found {:?} token instead",
                            delim, end, t.kind
                        ),
                        t.span,
                    );
                    self.recover(|t| t == delim || t == end);
                }
            }
        }
        let end = self.next();

        let elems = self.alloc(elems);
        let kind = constructor(&elems);
        self.alloc_node(kind, tok.span.to(end.span))
    }

    pub(super) fn parse_block(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let elems = self.parse_stmts(Some(TokenKind::RBrace));
        let elems = self.alloc(elems);
        let end = elems.last().map_or(tok.span, |last| last.span);
        self.alloc_node(ast::NodeKind::Block(elems), tok.span.to(end))
    }
}
