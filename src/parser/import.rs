use crate::{ast, parser::Parser};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_use(&mut self) -> &'ast ast::Node<'ast> {
        let start = self.next();
        let path = self.parse_access();
        self.alloc(ast::Node::new(ast::NodeKind::Use { path }, start.span.to(path.span)))
    }
}
