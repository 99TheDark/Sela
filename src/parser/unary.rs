use crate::{
    ast::{self, unop::UnOpKind},
    parser::RDParser,
    token::kind::TokenKind,
};

impl<'ast, 'diag, 'src> RDParser<'ast, 'diag, 'src> {
    pub fn parse_unop(&mut self) -> &'ast ast::Node<'ast> {
        let token = self.current();
        let span = token.span;

        use TokenKind::*;
        let sym = match token.kind {
            Dash => UnOpKind::Neg,
            Not => UnOpKind::Not,
            Amp => UnOpKind::Ref,
            Star => UnOpKind::Deref,
            _ => return self.parse_primary(),
        };

        self.advance();

        let operand = Box::new(self.parse_unop());
        self.alloc(ast::Node::new(ast::NodeKind::UnOp { op: sym, rhs: &operand }, span))
    }
}
