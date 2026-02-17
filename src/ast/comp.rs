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
