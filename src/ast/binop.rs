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
    fn name(&self) -> &str {
        use BinOpKind::*;
        match self {
            Add => "Add",
            Sub => "Subtract",
            Mul => "Multiply",
            Div => "Divide",
            Mod => "Modulo",
            And => "And",
            Or => "Or",
            Xor => "Exclusive Or",
            Shr => "Right Shift",
            Shl => "Left Shift",
        }
    }

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
