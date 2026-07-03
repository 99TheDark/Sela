/*use crate::{
    ast::{
        self, binary::BinaryKind, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind,
    },
    parser::RDParser,
    token::kind::TokenKind,
};

fn precedence(b: BinaryKind) -> u8 {
    use BinaryKind::*;
    match b {
        KwBinOp(KwBinOpKind::Or) => 1,
        KwBinOp(KwBinOpKind::And) => 2,
        Comp(
            CompKind::EqEq
            | CompKind::NotEq
            | CompKind::Lt
            | CompKind::Gt
            | CompKind::LtEq
            | CompKind::GtEq,
        ) => 3,
        BinOp(BinOpKind::Or) => 4,
        BinOp(BinOpKind::Xor) => 5,
        BinOp(BinOpKind::And) => 6,
        BinOp(BinOpKind::Shl | BinOpKind::Shr) => 7,
        BinOp(BinOpKind::Add | BinOpKind::Sub) => 8,
        BinOp(BinOpKind::Mul | BinOpKind::Div | BinOpKind::Mod) => 9,
    }
}

impl<'ast, 'diag, 'src> RDParser<'ast, 'diag, 'src> {
    pub fn parse_access(&mut self) -> &'ast ast::Node<'ast> {
        // TODO: Wait this isn't right... that means a+b.c+d = (a+b).(c+d)
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

            let prec = precedence(op);
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
*/
