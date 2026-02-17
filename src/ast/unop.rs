use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum UnOpKind {
    Neg,
    Not,
    Ref,
    Deref,
}

impl Symbol for UnOpKind {
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
