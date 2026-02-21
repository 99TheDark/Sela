use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum CompKind {
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl Symbol for CompKind {
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
