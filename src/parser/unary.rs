use crate::{
    ast::{self, unop::UnOpKind},
    parser::Parser,
    token::kind::TokenKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_unop(&mut self) -> ast::Node {
        let token = self.current();
        let span = token.span;

        use TokenKind::*;
        let sym = match token.kind {
            Dash => UnOpKind::Neg,
            Not => UnOpKind::Not,
            And => UnOpKind::Ref,
            Star => UnOpKind::Deref,
            _ => return self.parse_primary(),
        };

        self.advance();
        // self.eat_nls();

        let operand = Box::new(self.parse_unop());
        return ast::Node::new(ast::NodeKind::UnOp(sym, operand), span);
    }
}
