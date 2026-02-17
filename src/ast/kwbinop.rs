use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum KwBinOpKind {
    And,
    Or,
}

impl Symbol for KwBinOpKind {
    fn as_str(&self) -> &str {
        use KwBinOpKind::*;
        match self {
            And => "and",
            Or => "or",
        }
    }
}
