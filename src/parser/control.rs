use crate::{ast, parser::Parser, token::keyword::Keyword};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_if_else(&mut self) -> &'ast ast::Node<'ast> {
        let start = self.next();
        let cond = self.parse_expr();
        let body = self.parse_block();

        let (fallback, span) = if self.at_keyword(Keyword::Else) {
            self.advance();
            let block = self.parse_block();
            (Some(block), start.span.to(block.span))
        } else {
            (None, start.span.to(body.span))
        };

        self.alloc(ast::Node::new(
            ast::NodeKind::If {
                cond,
                body,
                fallback,
            },
            span,
        ))
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
