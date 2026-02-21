use crate::{
    ast,
    parser::Parser,
    token::{Token, kind::TokenKind},
};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_decl(&mut self, init: Token) -> &'ast ast::Node<'ast> {
        let vari = self.parse_expr(); // TODO: Parse pattern
        self.expect(TokenKind::Eq);
        let val = self.parse_expr();
        self.alloc(ast::Node::new(ast::NodeKind::Decl(&vari, &val), init.span))
    }
}
