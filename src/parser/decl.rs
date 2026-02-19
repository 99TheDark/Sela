use crate::{
    ast,
    parser::Parser,
    token::{Token, keyword::Keyword, kind::TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_decl(&mut self, init: Token) -> ast::Node {
        self.expect_keyword(Keyword::Mut);
        let vari = self.parse_expr();
        self.expect(TokenKind::Eq, true);
        let val = self.parse_expr();
        ast::Node::new(ast::NodeKind::Let(Box::new(vari), Box::new(val)), init.span)
    }
}
