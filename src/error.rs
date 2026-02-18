use crate::token::span::Span;

pub struct Error {
    pub message: String,
    pub span: Span,
}

pub struct Diagnostics {
    pub errors: Vec<Error>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn emit(&mut self) {
        todo!()
    }
}
