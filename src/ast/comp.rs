use crate::ast::symbol::{Symbol, Symbolic};

#[derive(Debug, Copy, Clone)]
pub enum CompKind {
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl CompKind {
    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        Symbol::Comp(self)
    }
}

impl Symbolic for CompKind {
    #[inline(always)]
    fn name(&self) -> &str {
        use CompKind::*;
        match self {
            EqEq => "Equal",
            NotEq => "Not Equal",
            Lt => "Less Than",
            Gt => "Greater Than",
            LtEq => "Less Than or Equal",
            GtEq => "Greater Than or Equal",
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        use CompKind::*;
        match self {
            EqEq => "==",
            NotEq => "!=",
            Lt => "<",
            Gt => ">",
            LtEq => "<=",
            GtEq => ">=",
        }
    }
}
