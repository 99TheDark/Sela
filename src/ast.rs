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

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

impl Node {
    pub const EMPTY: Self = Self {
        kind: NodeKind::Unknown,
        span: Span::ZERO,
    };

    pub fn new(kind: NodeKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum NodeKind {
    BinOp(Box<Node>, BinOpKind, Box<Node>),
    KwBinOp(Box<Node>, KwBinOpKind, Box<Node>),
    Comp(Box<Node>, CompKind, Box<Node>),
    Range(Box<Node>, RangeKind, Box<Node>),
    UnOp(UnOpKind, Box<Node>),
    Int(i64),
    Let(Box<Node>),
    Unknown,
}
