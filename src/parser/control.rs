use crate::{ast, parser::Parser};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_if_else(&mut self) -> &'ast ast::Node<'ast> {
        todo!()
    }

    pub fn parse_for_loop(&mut self) -> &'ast ast::Node<'ast> {
        todo!()
    }

    pub fn parse_while_loop(&mut self) -> &'ast ast::Node<'ast> {
        todo!()
    }

    pub fn parse_loop(&mut self) -> &'ast ast::Node<'ast> {
        todo!()
    }
}
