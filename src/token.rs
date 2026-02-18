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

    pub fn debug_src(&self, src: &str) -> String {
        self.span.debug_src(src)
    }

    pub fn src<'a>(&self, src: &'a str) -> &'a str {
        self.span.src(src)
    }
}
