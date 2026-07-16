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
    pub(super) fn parse_invocation(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
    ) -> ast::NodeRef<'ast> {
        let rhs =
            self.parse_delimited(tok, TokenKind::Comma, TokenKind::RParen, ast::NodeKind::Parens);
        self.alloc(ast::Node::new(
            ast::NodeKind::Invoc { callee: lhs, args: rhs },
            lhs.span.to(rhs.span),
        ))
    }
}
