use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shr,
    Shl,
}

impl Symbol for BinOpKind {
    fn as_str(&self) -> &str {
        use BinOpKind::*;
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            And => "&",
            Or => "|",
            Xor => "^",
            Shr => ">>",
            Shl => "<<",
        }
    }
}
