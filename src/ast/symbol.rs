use std::fmt;

use crate::ast::{
    assign::AssignKind, binop::BinOpKind, comp::CompKind, kwbinop::KwBinOpKind,
    range::RangeKind, unop::UnOpKind,
};

pub trait Symbolic {
    fn name(&self) -> &str;
    fn as_str(&self) -> &str;
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    BinOp(BinOpKind),
    KwBinOp(KwBinOpKind),
    Comp(CompKind),
    Range(RangeKind),
    UnOp(UnOpKind),
    Assign(AssignKind),
    Dot,
}

impl Symbolic for Symbol {
    #[inline(always)]
    fn name(&self) -> &str {
        use Symbol::*;
        match self {
            BinOp(kind) => kind.name(),
            KwBinOp(kind) => kind.name(),
            Comp(kind) => kind.name(),
            Range(kind) => kind.name(),
            UnOp(kind) => kind.name(),
            Assign(kind) => kind.name(),
            Dot => "Member Access",
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        use Symbol::*;
        match self {
            BinOp(kind) => kind.as_str(),
            KwBinOp(kind) => kind.as_str(),
            Comp(kind) => kind.as_str(),
            Range(kind) => kind.as_str(),
            UnOp(kind) => kind.as_str(),
            Assign(kind) => kind.as_str(),
            Dot => ".",
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
