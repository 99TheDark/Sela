use core::fmt;
use std::{convert, hint, ops};

use crate::{
    ast::{
        assign::AssignKind, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind,
        range::RangeKind, unop::UnOpKind, vis::VisKind,
    },
    core::span::Span,
};

pub mod assign;
pub mod binary;
pub mod binop;
pub mod comp;
pub mod kwbinop;
pub mod range;
pub mod string;
pub mod symbol;
pub mod unop;
pub mod vis;

#[derive(Copy, Clone)]
pub struct Node<'a> {
    pub kind: NodeKind<'a>,
    pub span: Span,
}

impl<'a> Node<'a> {
    pub const EMPTY: Self = Self { kind: NodeKind::Error, span: Span::ZERO };

    pub fn failed(span: Span) -> Self {
        Self { kind: NodeKind::Error, span }
    }

    pub fn new(kind: NodeKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node ({:?}, {:?})", self.kind, self.span)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NodeKind<'a> {
    Ident(&'a str),
    BinOp { lhs: NodeRef<'a>, op: BinOpKind, rhs: NodeRef<'a> },
    KwBinOp { lhs: NodeRef<'a>, op: KwBinOpKind, rhs: NodeRef<'a> },
    Comp { lhs: NodeRef<'a>, comp: CompKind, rhs: NodeRef<'a> },
    Range { from: Option<NodeRef<'a>>, range: RangeKind, to: Option<NodeRef<'a>> },
    UnOp { op: UnOpKind, rhs: NodeRef<'a> },
    Access { parent: NodeRef<'a>, child: NodeRef<'a> },
    Invoc { callee: NodeRef<'a>, args: NodeRef<'a> },
    Int(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(&'a [string::Fragment<'a>]),
    Decl { pat: NodeRef<'a>, val: NodeRef<'a> },
    Assign { pat: NodeRef<'a>, assign: AssignKind, val: NodeRef<'a> },
    Mut(NodeRef<'a>),
    Type { new: NodeRef<'a>, base: NodeRef<'a> },
    Alias { alt: NodeRef<'a>, src: NodeRef<'a> },
    If { cond: NodeRef<'a>, body: NodeRef<'a>, fallback: Option<NodeRef<'a>> },
    Loop { body: NodeRef<'a> },
    While { cond: NodeRef<'a>, body: NodeRef<'a> },
    For { clause: NodeRef<'a>, body: NodeRef<'a> },
    In { vari: NodeRef<'a>, iter: NodeRef<'a> },
    FuncSig { params: NodeRef<'a>, ret: Option<NodeRef<'a>> },
    Func { name: Option<NodeRef<'a>>, sig: NodeRef<'a>, body: Option<NodeRef<'a>> },
    Use { path: NodeRef<'a> },
    Vis { modif: VisKind, child: NodeRef<'a> },
    Charm,
    Parens(&'a [NodeRef<'a>]),
    Block(&'a [NodeRef<'a>]),
    Pair { lhs: NodeRef<'a>, rhs: NodeRef<'a> },

    Error,

    Unknown,
    UnknownInt,
    UnknownFloat,
    UnknownChar,
    UnknownString,
    UnknownRange { from: Option<NodeRef<'a>>, range: RangeKind, to: Option<NodeRef<'a>> },
}

pub type NodeRef<'a> = &'a Node<'a>;

impl<'a> ops::Try for NodeRef<'a> {
    type Output = Self;
    type Residual = Self;

    fn from_output(output: Self::Output) -> Self {
        output
    }

    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        match self.kind {
            NodeKind::Error => {
                hint::cold_path();
                ops::ControlFlow::Break(self)
            }
            _ => ops::ControlFlow::Continue(self),
        }
    }
}

impl<'a> ops::FromResidual<NodeRef<'a>> for NodeRef<'a> {
    fn from_residual(residual: NodeRef<'a>) -> Self {
        residual
    }
}

impl<'a> ops::FromResidual<Result<convert::Infallible, NodeRef<'a>>> for NodeRef<'a> {
    fn from_residual(residual: Result<convert::Infallible, NodeRef<'a>>) -> Self {
        match residual {
            Err(err) => err,
        }
    }
}

impl<'a> ops::Residual<NodeRef<'a>> for NodeRef<'a> {
    type TryType = Self;
}

impl<'a> From<NodeRef<'a>> for Result<NodeRef<'a>, NodeRef<'a>> {
    fn from(value: NodeRef<'a>) -> Self {
        match value.kind {
            NodeKind::Error => Err(value),
            _ => Ok(value),
        }
    }
}

impl<'a> Node<'a> {
    pub fn is_error(self) -> bool {
        matches!(self.kind, NodeKind::Error)
    }
}
