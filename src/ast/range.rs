use crate::{
    ast::symbol::{Symbol, Symbolic},
    token::Token,
};

#[derive(Debug, Copy, Clone)]
pub enum RangeKind {
    Full,
    Excl,
    Incl,
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

    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        Symbol::Range(self)
    }
}

impl Symbolic for RangeKind {
    #[inline(always)]
    fn name(&self) -> &str {
        use RangeKind::*;
        match self {
            Full => "Full Range",
            Excl => "Exclusive Range",
            Incl => "Inclusive Range",
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        use RangeKind::*;
        match self {
            Full => "..",
            Excl => "..<",
            Incl => "..=",
        }
    }
}
