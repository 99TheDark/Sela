use std::fmt;

use crate::{
    core::span::Span,
    token::{kind::TokenKind, precedence::Precedence},
};

pub mod kind;
pub mod precedence;

#[derive(Copy, Clone)]
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

    #[inline(always)]
    pub fn nud_prec(&self) -> Precedence {
        self.kind.nud_prec()
    }

    #[inline(always)]
    pub fn led_prec(&self) -> Precedence {
        self.kind.led_prec()
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token ({:?} {:?})", self.kind, self.span)
    }
}
