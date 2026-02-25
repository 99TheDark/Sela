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
    pub const EMPTY: Self = Self { kind: NodeKind::Unknown, span: Span::ZERO };

    pub fn failed(span: Span) -> Self {
        Self { kind: NodeKind::Unknown, span }
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
    BinOp { lhs: &'a Node<'a>, op: BinOpKind, rhs: &'a Node<'a> },
    KwBinOp { lhs: &'a Node<'a>, op: KwBinOpKind, rhs: &'a Node<'a> },
    Comp { lhs: &'a Node<'a>, comp: CompKind, rhs: &'a Node<'a> },
    Range { from: Option<&'a Node<'a>>, range: RangeKind, to: Option<&'a Node<'a>> },
    UnOp { op: UnOpKind, rhs: &'a Node<'a> },
    Access { parent: &'a Node<'a>, child: &'a Node<'a> },
    Int(i64),
    Bool(bool),
    Decl { pat: &'a Node<'a>, val: &'a Node<'a> },
    If { cond: &'a Node<'a>, body: &'a Node<'a>, fallback: Option<&'a Node<'a>> },
    Block(Vec<&'a Node<'a>>),
    Unknown,
}
