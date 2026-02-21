use crate::{
    ast,
    parser::Parser,
    token::{Token, keyword::Keyword, kind::TokenKind},
};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_decl(&mut self, init: Token) -> &'ast ast::Node<'ast> {
        self.expect_keyword(Keyword::Mut);
        let vari = self.parse_expr();
        self.expect(TokenKind::Eq);
        let val = self.parse_expr();
        self.alloc(ast::Node::new(ast::NodeKind::Let(&vari, &val), init.span))
    }
}
