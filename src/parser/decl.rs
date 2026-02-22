use crate::{ast, parser::Parser, token::kind::TokenKind};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_decl(&mut self) -> &'ast ast::Node<'ast> {
        let start = self.next();
        let pat = self.parse_expr(); // TODO: Parse pattern
        self.expect(TokenKind::Eq);
        let val = self.parse_expr();
        self.alloc(ast::Node::new(ast::NodeKind::Decl { pat, val }, start.span))
    }
}
