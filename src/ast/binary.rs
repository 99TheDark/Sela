use bumpalo::Bump;

use crate::{
    ast::{
        self, assign::AssignKind, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind,
        symbol::Symbol,
    },
    token::Token,
};

#[derive(Debug, Copy, Clone)]
pub enum BinaryKind {
    Assign(AssignKind),
    BinOp(BinOpKind),
    KwBinOp(KwBinOpKind),
    Comp(CompKind),
}

impl BinaryKind {
    pub fn from_token(token: Token) -> Option<Self> {
        use crate::TokenKind::*;

        let kind = match token.kind {
            Plus => Self::BinOp(BinOpKind::Add),
            PlusEq => Self::Assign(AssignKind::AddEq),
            Dash => Self::BinOp(BinOpKind::Sub),
            DashEq => Self::Assign(AssignKind::SubEq),
            Star => Self::BinOp(BinOpKind::Mul),
            StarEq => Self::Assign(AssignKind::MulEq),
            Slash => Self::BinOp(BinOpKind::Div),
            SlashEq => Self::Assign(AssignKind::DivEq),
            Pct => Self::BinOp(BinOpKind::Mod),
            PctEq => Self::Assign(AssignKind::ModEq),
            Gt => Self::Comp(CompKind::Gt),
            Lt => Self::Comp(CompKind::Lt),
            EqEq => Self::Comp(CompKind::EqEq),
            NotEq => Self::Comp(CompKind::NotEq),
            GtEq => Self::Comp(CompKind::GtEq),
            LtEq => Self::Comp(CompKind::LtEq),
            GtGt => Self::BinOp(BinOpKind::Shr),
            GtGtEq => Self::Assign(AssignKind::ShrEq),
            LtLt => Self::BinOp(BinOpKind::Shl),
            LtLtEq => Self::Assign(AssignKind::ShlEq),
            Caret => Self::BinOp(BinOpKind::Xor),
            CaretEq => Self::Assign(AssignKind::XorEq),
            Amp => Self::BinOp(BinOpKind::And),
            AmpEq => Self::Assign(AssignKind::AndEq),
            Bar => Self::BinOp(BinOpKind::Or),
            BarEq => Self::Assign(AssignKind::OrEq),
            Eq => Self::Assign(AssignKind::Eq),
            And => Self::KwBinOp(KwBinOpKind::And),
            Or => Self::KwBinOp(KwBinOpKind::Or),
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
            Assign(asgn) => ast::NodeKind::Assign { pat: lhs, assign: *asgn, val: rhs },
            BinOp(op) => ast::NodeKind::BinOp { lhs, op: *op, rhs },
            KwBinOp(op) => ast::NodeKind::KwBinOp { lhs, op: *op, rhs },
            Comp(comp) => ast::NodeKind::Comp { lhs, comp: *comp, rhs },
        };

        alloc.alloc(ast::Node::new(kind, span))
    }

    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        use BinaryKind::*;
        match self {
            Assign(kind) => kind.to_sym(),
            BinOp(kind) => kind.to_sym(),
            KwBinOp(kind) => kind.to_sym(),
            Comp(kind) => kind.to_sym(),
        }
    }
}
