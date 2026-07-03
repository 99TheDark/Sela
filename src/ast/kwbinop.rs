use crate::ast::symbol::{Symbol, Symbolic};

#[derive(Debug, Copy, Clone)]
pub enum KwBinOpKind {
    And,
    Or,
}

impl KwBinOpKind {
    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        Symbol::KwBinOp(self)
    }
}

impl Symbolic for KwBinOpKind {
    #[inline(always)]
    fn name(&self) -> &str {
        use KwBinOpKind::*;
        match self {
            And => "And",
            Or => "Or",
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        use KwBinOpKind::*;
        match self {
            And => "and",
            Or => "or",
        }
    }
}
