use crate::token::span::Span;

pub const CONSOLE_WIDTH: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostics {
    errors: Vec<Error>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn emit(&mut self, message: String, span: Span) {
        self.errors.push(Error { message, span });
    }

    pub fn print(&self) {
        todo!()
    }
}
