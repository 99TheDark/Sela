use crate::{
    ast::{self, assign::AssignKind},
    parser::RDParser,
    token::kind::TokenKind,
};

impl<'ast, 'diag, 'src> RDParser<'ast, 'diag, 'src> {
    pub fn parse_decl(&mut self) -> &'ast ast::Node<'ast> {
        let start = self.next();
        let pat = self.parse_expr(); // TODO: Parse pattern
        self.expect(TokenKind::Eq);
        let val = self.parse_expr();
        self.alloc(ast::Node::new(ast::NodeKind::Decl { pat, val }, start.span))
    }

    pub fn parse_assign(&mut self) -> &'ast ast::Node<'ast> {
        let vari = self.parse_access();

        let Some(assign) = AssignKind::from_token(self.peek()) else { return vari };
        self.next();

        let val = self.parse_access();
        self.alloc(ast::Node::new(
            ast::NodeKind::Assign { vari, assign, val },
            vari.span.to(val.span),
        ))
    }
}
