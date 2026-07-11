use crate::ast::symbol::{Symbol, Symbolic};

#[derive(Debug, Copy, Clone)]
pub enum UnOpKind {
    Pos,
    Neg,
    Not,
    Ref,
    Deref,
}

impl UnOpKind {
    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        Symbol::UnOp(self)
    }
}

impl Symbolic for UnOpKind {
    #[inline(always)]
    fn name(&self) -> &str {
        use UnOpKind::*;
        match self {
            Pos => "Posit",
            Neg => "Negate",
            Not => "Not",
            Ref => "Reference",
            Deref => "Dereference",
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        use UnOpKind::*;
        match self {
            Pos => "+",
            Neg => "-",
            Not => "!",
            Ref => "&",
            Deref => "*",
        }
    }
}
