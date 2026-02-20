use crate::{
    ast::{self, symbol::BinaryKind},
    parser::Parser,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_binop(&mut self) -> ast::Node {
        self.parse_binop_pratt(0)
    }

    pub fn parse_binop_pratt(&mut self, min_prec: u8) -> ast::Node {
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
            left = op.make_node(left, right);
        }
        left
    }
}
