use bumpalo::Bump;

use crate::{
    ast::{self, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind, symbol::Symbol},
    token::{Token, keyword::Keyword},
};

#[derive(Debug, Copy, Clone)]
pub enum BinaryKind {
    BinOp(BinOpKind),
    KwBinOp(KwBinOpKind),
    Comp(CompKind),
}

impl Symbol for BinaryKind {
    fn name(&self) -> &str {
        use BinaryKind::*;
        match self {
            BinOp(kind) => kind.name(),
            KwBinOp(kind) => kind.name(),
            Comp(kind) => kind.name(),
        }
    }

    fn as_str(&self) -> &str {
        use BinaryKind::*;
        match self {
            BinOp(kind) => kind.as_str(),
            KwBinOp(kind) => kind.as_str(),
            Comp(kind) => kind.as_str(),
        }
    }
}

impl BinaryKind {
    pub fn from_token(token: Token, src: &str) -> Option<Self> {
        use crate::TokenKind::*;

        let kind = match token.kind {
            Ident => {
                use Keyword::*;
                match token.to_keyword(src) {
                    And => Self::KwBinOp(KwBinOpKind::And),
                    Or => Self::KwBinOp(KwBinOpKind::Or),
                    _ => return None,
                }
            }
            Plus => Self::BinOp(BinOpKind::Add),
            Dash => Self::BinOp(BinOpKind::Sub),
            Star => Self::BinOp(BinOpKind::Mul),
            Slash => Self::BinOp(BinOpKind::Div),
            Pct => Self::BinOp(BinOpKind::Mod),
            Gt => Self::Comp(CompKind::Gt),
            Lt => Self::Comp(CompKind::Lt),
            EqEq => Self::Comp(CompKind::EqEq),
            NotEq => Self::Comp(CompKind::NotEq),
            GtEq => Self::Comp(CompKind::GtEq),
            LtEq => Self::Comp(CompKind::LtEq),
            GtGt => Self::BinOp(BinOpKind::Shr),
            LtLt => Self::BinOp(BinOpKind::Shl),
            Caret => Self::BinOp(BinOpKind::Xor),
            Amp => Self::BinOp(BinOpKind::And),
            Bar => Self::BinOp(BinOpKind::Or),
            _ => return None,
        };

        Some(kind)
    }

    pub fn make_node<'ast>(
        &self,
        lhs: &'ast ast::Node<'ast>,
        rhs: &'ast ast::Node<'ast>,
        alloc: &'ast Bump,
    ) -> &'ast ast::Node<'ast> {
        let span = lhs.span.to(rhs.span);

        use BinaryKind::*;
        let kind = match self {
            BinOp(op) => ast::NodeKind::BinOp { lhs, op: *op, rhs },
            KwBinOp(op) => ast::NodeKind::KwBinOp { lhs, op: *op, rhs },
            Comp(comp) => ast::NodeKind::Comp { lhs, comp: *comp, rhs },
        };

        alloc.alloc(ast::Node::new(kind, span))
    }
}
