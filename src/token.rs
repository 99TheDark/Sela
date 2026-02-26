use crate::{span::Span, token::kind::TokenKind};

pub mod keyword;
pub mod kind;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn src<'a>(&self, src: &'a str) -> &'a str {
        self.span.src(src)
    }

    pub fn debug_src(&self, src: &str) -> String {
        self.span.debug_src(src)
    }

    pub fn is_eof(self) -> bool {
        self.kind == TokenKind::EOF
    }

    pub fn is_nl(self) -> bool {
        self.kind == TokenKind::NewLine
    }

    pub fn is_ident(self) -> bool {
        self.kind == TokenKind::Ident
    }
}
