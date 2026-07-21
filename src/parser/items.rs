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
        let decls = if self.peek_next().is(TokenKind::LBrack) {
            let lbrack = self.true_next();
            Some(self.parse_bracks(lbrack))
        } else {
            None
        };

        let target = {
            let (idea, typ) = {
                let first = self.parse_expr(Precedence::None);
                if self.peek().is(TokenKind::For) {
                    self.advance();
                    let second = self.parse_expr(Precedence::None);
                    (Some(first), second)
                } else {
                    (None, first)
                }
            };

            self.alloc_node(
                ast::NodeKind::ImplTarget { idea, typ },
                idea.map_or(typ.span, |n| n.span.to(typ.span)),
            )
        };

        let lbrace = self.expect(TokenKind::LBrace, tok.span.to(target.span))?;
        let body = self.parse_block(lbrace);

        self.alloc_node(ast::NodeKind::Impl { decls, target, body }, tok.span.to(body.span))
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
