use crate::{
    ast::{self, assign::AssignKind},
    parser::Parser,
    token::kind::TokenKind,
};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_decl(&mut self) -> &'ast ast::Node<'ast> {
        let start = self.next();
        let pat = self.parse_expr(); // TODO: Parse pattern
        self.expect(TokenKind::Eq);
        let val = self.parse_expr();
        self.alloc(ast::Node::new(ast::NodeKind::Decl { pat, val }, start.span))
    }

    pub fn parse_assign(&mut self) -> &'ast ast::Node<'ast> {
        println!(
            "&& {:?} |> '{}', {:?} |> '{}'",
            self.current(),
            self.current().src(self.src),
            self.peek(),
            self.peek().src(self.src)
        );
        let vari = self.parse_access();

        println!("|| {:?} |> '{}'", self.peek(), self.peek().src(self.src));

        let Some(assign) = AssignKind::from_token(self.peek()) else { return vari };
        self.next();

        println!(":: => {:?}", assign);

        let val = self.parse_access();
        self.alloc(ast::Node::new(
            ast::NodeKind::Assign { vari, assign, val },
            vari.span.to(val.span),
        ))
    }
}
