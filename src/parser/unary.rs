use crate::{
    ast::{self, unop::UnOpKind},
    parser::Parser,
    token::{Token, kind::TokenKind},
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_unop(&mut self) -> ast::Node {
        let Some(token) = self.tokens.peek() else {
            return ast::Node::EMPTY;
        };

        let span = token.span;

        use TokenKind::*;
        let sym = match token.kind {
            Dash => UnOpKind::Neg,
            Not => UnOpKind::Not,
            And => UnOpKind::Ref,
            Star => UnOpKind::Deref,
            _ => return self.parse_primary(),
        };

        self.tokens.next();
        let operand = Box::new(self.parse_unop());
        return ast::Node::new(ast::NodeKind::UnOp(sym, operand), span);
    }
}
