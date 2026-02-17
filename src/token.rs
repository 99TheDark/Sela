use crate::token::{kind::TokenKind, span::Span};

pub mod keyword;
pub mod kind;
pub mod span;

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn str_value(&self, src: &str) -> String {
        self.span.str_value(src)
    }
}
