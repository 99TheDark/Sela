use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum KwBinOpKind {
    And,
    Or,
}

impl Symbol for KwBinOpKind {
    fn name(&self) -> &str {
        use KwBinOpKind::*;
        match self {
            And => "And",
            Or => "Or",
        }
    }

    fn as_str(&self) -> &str {
        use KwBinOpKind::*;
        match self {
            And => "and",
            Or => "or",
        }
    }
}
