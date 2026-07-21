use crate::{
    ast,
    parser::{Parser, groups::DelimEnclosement},
    token::{Token, kind::TokenKind, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_enum(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let relat = self.parse_relat();
        let lbrace = self.expect(TokenKind::LBrace, tok.span.to(relat.span))?;
        let body = self.parse_block(lbrace);

        let kind = ast::NodeKind::Enum { relat, body };
        self.alloc_node(kind, tok.span.to(body.span))
    }

    pub(super) fn parse_impl(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    pub(super) fn parse_idea(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    pub(super) fn parse_relat(&mut self) -> ast::NodeRef<'ast> {
        let child = self.parse_expr(Precedence::Pair); // ! Why does this work???
        if self.peek().is(TokenKind::Colon) {
            let colon = self.true_next();

            self.parse_delimited(
                colon,
                TokenKind::Comma,
                TokenKind::LBrace,
                DelimEnclosement::Unenclosed { start: colon.span },
                |elems| ast::NodeKind::Relat { child, parents: elems },
            )
        } else {
            child
        }
    }
}
