use crate::{core::span::Span, token::kind::TokenKind};

pub mod kind;

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    #[inline(always)]
    pub fn src<'a>(&self, src: &'a str) -> &'a str {
        self.span.src(src)
    }

    #[inline(always)]
    pub fn debug_src(&self, src: &str) -> String {
        self.span.debug_src(src)
    }

    #[inline(always)]
    pub fn is_eof(self) -> bool {
        self.kind == TokenKind::EOF
    }

    #[inline(always)]
    pub fn is_nl(self) -> bool {
        self.kind == TokenKind::NewLine
    }

    pub fn is_ident(self) -> bool {
        self.kind == TokenKind::Ident
    }

    #[inline(always)]
    pub fn is(self, kind: TokenKind) -> bool {
        self.kind == kind && self.kind != TokenKind::EOF
    }

    #[inline(always)]
    pub fn eof_is(self, kind: TokenKind) -> bool {
        self.kind == kind && self.kind != TokenKind::EOF
    }

    #[inline(always)]
    pub fn eof_not_is(self, kind: TokenKind) -> bool {
        self.kind != kind && self.kind != TokenKind::EOF
    }

    pub fn nud_prec(&self) -> u8 {
        use TokenKind::*;
        match self.kind {
            DotDotLt | DotDotEq => 3,
            Dash | Star | Amp | Not => 15,
            _ => 0,
        }
    }

    pub fn led_prec(&self) -> u8 {
        use TokenKind::*;
        match self.kind {
            Comma => 1,

            // Illegal
            Eq | PlusEq | DashEq | StarEq | SlashEq | PctEq | GtGtEq | LtLtEq
            | CaretEq | AmpEq | BarEq => 2,

            DotDotLt | DotDotEq => 3,
            Or => 4,
            And => 5,
            EqEq | NotEq => 6,
            Gt | Lt | GtEq | LtEq => 7,
            Bar => 8,
            Caret => 9,
            Amp => 10,
            GtGt | LtLt => 11,
            Plus | Dash => 12,
            Star | Slash | Pct => 13,
            As => 14,
            At => 16,
            Colon => 17,
            Dot | LParen | LBrack => 18,

            _ => 0,
        }
    }
}
