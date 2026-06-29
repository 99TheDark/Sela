use core::fmt;

use smallvec::SmallVec;

use crate::{
    ast::{
        assign::AssignKind, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind,
        range::RangeKind, unop::UnOpKind,
    },
    core::span::Span,
};

pub mod assign;
pub mod binary;
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

pub type NodeRef<'a> = &'a Node<'a>;

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

// TODO: Use bumpalo references instead; `BinOp(&'ast ast::BinOpData<'ast>),`
// Allows for fixed-size (~8 byte data + 8 byte tag) `NodeKind`s and thus 24-bye `Node`s
// Does add indirection, however
#[derive(Debug)]
pub enum NodeKind<'a> {
    Ident(&'a str),
    BinOp { lhs: NodeRef<'a>, op: BinOpKind, rhs: NodeRef<'a> },
    KwBinOp { lhs: NodeRef<'a>, op: KwBinOpKind, rhs: NodeRef<'a> },
    Comp { lhs: NodeRef<'a>, comp: CompKind, rhs: NodeRef<'a> },
    Range { from: Option<NodeRef<'a>>, range: RangeKind, to: Option<NodeRef<'a>> },
    UnOp { op: UnOpKind, rhs: NodeRef<'a> },
    Access { parent: NodeRef<'a>, child: NodeRef<'a> },
    Invoc { callee: NodeRef<'a>, args: NodeRef<'a> },
    Int(i64),
    Float(f64),
    Bool(bool),
    Decl { pat: NodeRef<'a>, val: NodeRef<'a> },
    Assign { vari: NodeRef<'a>, assign: AssignKind, val: NodeRef<'a> },
    If { cond: NodeRef<'a>, body: NodeRef<'a>, fallback: Option<NodeRef<'a>> },
    Loop { body: NodeRef<'a> },
    While { cond: NodeRef<'a>, body: NodeRef<'a> },
    For { vari: NodeRef<'a>, iter: NodeRef<'a>, body: NodeRef<'a> },
    Use { path: NodeRef<'a> },
    Parens(SmallVec<[NodeRef<'a>; 4]>),
    Block(Vec<NodeRef<'a>>),
    Pair { lhs: NodeRef<'a>, rhs: NodeRef<'a> },
    Unknown,
}
