use crate::{
    ast::{self, symbol::BinaryKind},
    parser::Parser,
};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
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
            self.eat_nls();

            let right = self.parse_binop_pratt(prec + 1);
            left = op.make_node(&left, right, &self.arena);
        }
        left
    }
}
