use crate::{ast::symbol::Symbol, token::Token};

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

impl RangeKind {
    pub fn from_token(tok: Token) -> Option<Self> {
        use crate::TokenKind::*;
        let kind = match tok.kind {
            DotDot => RangeKind::Full,
            DotDotLt => RangeKind::Excl,
            DotDotEq => RangeKind::Incl,
            _ => return None,
        };
        Some(kind)
    }
}
