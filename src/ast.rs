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

pub struct Node<'a> {
    pub kind: NodeKind<'a>,
    pub span: Span,
}

impl<'a> Node<'a> {
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

    pub fn new(kind: NodeKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node ({:?}): ", self.span)?;
        self.kind.fmt(f)
    }
}

#[derive(Debug)]
pub enum NodeKind<'a> {
    Ident(String),
    BinOp(&'a Node<'a>, BinOpKind, &'a Node<'a>),
    KwBinOp(&'a Node<'a>, KwBinOpKind, &'a Node<'a>),
    Comp(&'a Node<'a>, CompKind, &'a Node<'a>),
    Range(&'a Node<'a>, RangeKind, &'a Node<'a>),
    UnOp(UnOpKind, &'a Node<'a>),
    Int(i64),
    Bool(bool),
    Let(&'a Node<'a>, &'a Node<'a>),
    Unknown,
}
