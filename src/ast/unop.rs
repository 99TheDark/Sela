use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum UnOpKind {
    Neg,
    Not,
    Ref,
    Deref,
}

impl Symbol for UnOpKind {
    fn name(&self) -> &str {
        use UnOpKind::*;
        match self {
            Neg => "Negate",
            Not => "Not",
            Ref => "Reference",
            Deref => "Dereference",
        }
    }

    fn as_str(&self) -> &str {
        use UnOpKind::*;
        match self {
            Neg => "-",
            Not => "!",
            Ref => "&",
            Deref => "*",
        }
    }
}
