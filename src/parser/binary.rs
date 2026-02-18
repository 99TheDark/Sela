use crate::{
    ast::{self, symbol::BinaryKind},
    parser::Parser,
    token::Token,
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_binop(&mut self) -> ast::Node {
        self.parse_binop_pratt(0)
    }

    pub fn parse_binop_pratt(&mut self, min_prec: u8) -> ast::Node {
        let mut left = self.parse_unop();
        while let Some(token) = self.tokens.peek() {
            let Some(op) = BinaryKind::from_token(*token, self.src) else {
                break;
            };

            let right_prec = op.precedence();
            if right_prec < min_prec {
                break;
            }
            self.tokens.next();

            let right = self.parse_binop_pratt(right_prec);
            left = op.make_node(left, right);
        }
        left
    }
}
