use std::fmt;

use crate::{
    ast::{
        self, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind, range::RangeKind,
    },
    token::Token,
};

pub trait Symbol: fmt::Debug + Copy + Clone {
    fn as_str(&self) -> &str;
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryKind {
    BinOp(BinOpKind),
    KwBinOp(KwBinOpKind),
    Comp(CompKind),
    Range(RangeKind),
}

impl Symbol for BinaryKind {
    fn as_str(&self) -> &str {
        use BinaryKind::*;
        match self {
            BinOp(kind) => kind.as_str(),
            KwBinOp(kind) => kind.as_str(),
            Comp(kind) => kind.as_str(),
            Range(kind) => kind.as_str(),
        }
    }
}

impl BinaryKind {
    pub fn from_token(token: Token, src: &str) -> Option<Self> {
        use crate::TokenKind::*;

        let kind = match token.kind {
            Ident => match token.src(src) {
                "and" => Self::KwBinOp(KwBinOpKind::And),
                "or" => Self::KwBinOp(KwBinOpKind::Or),
                _ => return None,
            },
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
            And => Self::BinOp(BinOpKind::And),
            Bar => Self::BinOp(BinOpKind::Or),
            DotDot => Self::Range(RangeKind::Full),
            DotDotLt => Self::Range(RangeKind::Excl),
            DotDotEq => Self::Range(RangeKind::Incl),
            _ => return None,
        };

        Some(kind)
    }

    pub fn precedence(&self) -> u8 {
        use BinaryKind::*;
        match self {
            Range(RangeKind::Full | RangeKind::Excl | RangeKind::Incl) => 0,
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

    pub fn make_node(&self, left: ast::Node, right: ast::Node) -> ast::Node {
        let span = left.span.to(right.span);

        use BinaryKind::*;
        let kind = match self {
            BinOp(op) => ast::NodeKind::BinOp(Box::new(left), *op, Box::new(right)),
            KwBinOp(op) => ast::NodeKind::KwBinOp(Box::new(left), *op, Box::new(right)),
            Comp(cmp) => ast::NodeKind::Comp(Box::new(left), *cmp, Box::new(right)),
            Range(mode) => ast::NodeKind::Range(Box::new(left), *mode, Box::new(right)),
        };

        ast::Node::new(kind, span)
    }
}
