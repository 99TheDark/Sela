use crate::{
    ast::symbol::{Symbol, Symbolic},
    token::Token,
};

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

impl BinOpKind {
    pub fn from_token(tok: Token) -> Option<Self> {
        use crate::TokenKind::*;
        match tok.kind {
            Plus => Some(Self::Add),
            Dash => Some(Self::Sub),
            Star => Some(Self::Mul),
            Slash => Some(Self::Div),
            Pct => Some(Self::Mod),
            Amp => Some(Self::And),
            Bar => Some(Self::Or),
            Caret => Some(Self::Xor),
            GtGt => Some(Self::Shr),
            LtLt => Some(Self::Shl),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn to_sym(self) -> Symbol {
        Symbol::BinOp(self)
    }
}

impl Symbolic for BinOpKind {
    #[inline(always)]
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

    #[inline(always)]
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
