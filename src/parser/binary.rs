use crate::{
    ast::{self, symbol::BinaryKind},
    parser::Parser,
    token::kind::TokenKind,
};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_access(&mut self) -> &'ast ast::Node<'ast> {
        let left = self.parse_binop();
        if !self.at_and_eat(TokenKind::Dot) {
            return left;
        }

        let right = self.parse_binop();
        self.alloc(ast::Node::new(
            ast::NodeKind::Access { parent: left, child: right },
            left.span.to(right.span),
        ))
    }

    pub fn parse_binop(&mut self) -> &'ast ast::Node<'ast> {
        self.parse_binop_pratt(0)
    }

    pub fn parse_binop_pratt(&mut self, min_prec: u8) -> &'ast ast::Node<'ast> {
        let mut left = self.parse_unop();

        loop {
            let tok = self.current();
            if tok.is_eof() {
                break;
            }

            let Some(op) = BinaryKind::from_token(tok, self.src) else {
                break;
            };

            let prec = op.precedence();
            if prec < min_prec {
                break;
            }

            self.advance();
            //self.eat_nls();

            let right = self.parse_binop_pratt(prec + 1);
            left = op.make_node(&left, right, &self.arena);
        }
        left
    }
}
