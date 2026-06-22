use crate::{ast::symbol::Symbol, token::Token};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AssignKind {
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    ShrEq,
    ShlEq,
    XorEq,
    AndEq,
    OrEq,
}

impl AssignKind {
    pub fn from_token(tok: Token) -> Option<Self> {
        use crate::TokenKind::*;
        match tok.kind {
            Eq => Some(Self::Eq),
            PlusEq => Some(Self::AddEq),
            DashEq => Some(Self::SubEq),
            StarEq => Some(Self::MulEq),
            SlashEq => Some(Self::DivEq),
            PctEq => Some(Self::ModEq),
            GtGtEq => Some(Self::ShrEq),
            LtLtEq => Some(Self::ShlEq),
            CaretEq => Some(Self::XorEq),
            AndEq => Some(Self::AndEq),
            BarEq => Some(Self::OrEq),
            _ => None,
        }
    }
}

impl Symbol for AssignKind {
    fn name(&self) -> &str {
        use AssignKind::*;
        match self {
            Eq => "Assignment",
            AddEq => "Addition Assignment",
            SubEq => "Subtraction Assignment",
            MulEq => "Multiplication Assignment",
            DivEq => "Division Assignment",
            ModEq => "Modulus Assignment",
            ShrEq => "Right Shift Assignment",
            ShlEq => "Left Shift Assignment",
            XorEq => "Exclusive Or Assignment",
            AndEq => "And Assignment",
            OrEq => "Or Assignment",
        }
    }

    fn as_str(&self) -> &str {
        use AssignKind::*;
        match self {
            Eq => "=",
            AddEq => "+=",
            SubEq => "-=",
            MulEq => "*=",
            DivEq => "/=",
            ModEq => "%=",
            ShrEq => ">>=",
            ShlEq => "<<=",
            XorEq => "^=",
            AndEq => "&=",
            OrEq => "|=",
        }
    }
}
