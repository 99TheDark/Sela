use crate::ast::symbol::{Symbol, Symbolic};

#[derive(Debug, Copy, Clone)]
pub enum VisKind {
    Pub,
    Inn,
    Pri,
}

impl VisKind {
    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        Symbol::Vis(self)
    }
}

impl Symbolic for VisKind {
    #[inline(always)]
    fn name(&self) -> &str {
        use VisKind::*;
        match self {
            Pub => "Public",
            Inn => "Inner",
            Pri => "Private",
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        use VisKind::*;
        match self {
            Pub => "pub",
            Inn => "inn",
            Pri => "pri",
        }
    }
}
