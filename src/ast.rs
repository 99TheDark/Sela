use core::fmt;

use crate::{
    ast::{
        binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind, range::RangeKind,
        unop::UnOpKind,
    },
    token::span::Span,
};

pub mod binop;
pub mod comp;
pub mod kwbinop;
pub mod range;
pub mod symbol;
pub mod unop;

pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

impl Node {
    pub const EMPTY: Self = Self {
        kind: NodeKind::Unknown,
        span: Span::ZERO,
    };

    pub fn failed(span: Span) -> Self {
        Self {
            kind: NodeKind::Unknown,
            span,
        }
    }

    pub fn new(kind: NodeKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node ({:?}): ", self.span)?;
        self.kind.fmt(f)
    }
}

#[derive(Debug)]
pub enum NodeKind {
    Ident(String),
    BinOp(Box<Node>, BinOpKind, Box<Node>),
    KwBinOp(Box<Node>, KwBinOpKind, Box<Node>),
    Comp(Box<Node>, CompKind, Box<Node>),
    Range(Box<Node>, RangeKind, Box<Node>),
    UnOp(UnOpKind, Box<Node>),
    Int(i64),
    Bool(bool),
    Let(Box<Node>, Box<Node>),
    Unknown,
}
