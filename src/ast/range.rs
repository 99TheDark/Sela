use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum RangeKind {
    Full,
    Excl,
    Incl,
}

impl Symbol for RangeKind {
    fn as_str(&self) -> &str {
        use RangeKind::*;
        match self {
            Full => "..",
            Excl => "..<",
            Incl => "..=",
        }
    }
}
