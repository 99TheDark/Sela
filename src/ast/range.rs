use crate::ast::symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub enum RangeKind {
    Full,
    Excl,
    Incl,
}

impl Symbol for RangeKind {
    fn name(&self) -> &str {
        use RangeKind::*;
        match self {
            Full => "Full Range",
            Excl => "Exclusive Range",
            Incl => "Inclusive Range",
        }
    }

    fn as_str(&self) -> &str {
        use RangeKind::*;
        match self {
            Full => "..",
            Excl => "..<",
            Incl => "..=",
        }
    }
}
