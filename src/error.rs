use crate::token::span::Span;

pub const CONSOLE_WIDTH: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostics<'a> {
    file_name: String,
    src: &'a str,
    with_color: bool,
    errors: Vec<Error>,
}

trait NumDigits {
    fn num_digits(self) -> u32;
}

impl NumDigits for usize {
    fn num_digits(self) -> u32 {
        self.checked_ilog10().unwrap_or(0) + 1
    }
}

impl<'a> Diagnostics<'a> {
    pub fn new(file_name: String, src: &'a str) -> Self {
        Self {
            file_name,
            src,
            with_color: true,
            errors: Vec::new(),
        }
    }

    pub fn emit(&mut self, message: String, span: Span) {
        self.errors.push(Error { message, span });
    }

    fn above(&self, row: usize) -> u32 {
        const LEADING_SIZE: usize = 4;
        let top_row = row.saturating_sub(LEADING_SIZE);
        let left_pad = (row + 1).num_digits();

        // TODO: Very inefficient, compute this ahead of time
        let lines = self.src.split('\n').collect::<Vec<&str>>();
        for i in top_row..=row {
            let n = i + 1;
            let pad = left_pad - n.num_digits();
            println!("{}{} | {}", n, " ".repeat(pad as usize), lines[i]);
        }

        left_pad
    }

    fn yellow(&self, s: String) -> String {
        if self.with_color {
            format!("\x1b[33m{}\x1b[0m", s)
        } else {
            s
        }
    }

    fn red(&self, s: String) -> String {
        if self.with_color {
            format!("\x1b[31m{}\x1b[0m", s)
        } else {
            s
        }
    }

    pub fn print(self) {
        for error in &self.errors {
            println!(
                "{}: {}:{}:{}",
                self.red("Error".to_string()),
                self.file_name,
                error.span.start.row,
                error.span.start.col
            );

            let left_pad = self.above(error.span.start.row);

            // TODO: Currently assumes the error occurs on only one line, the span is not malformed, and the error message is short enough.
            println!(
                "{} | {}{} {}\n",
                " ".repeat(left_pad as usize),
                " ".repeat(error.span.start.col),
                self.red("^".repeat(error.span.end.col - error.span.start.col)),
                self.red(error.message.clone())
            );
        }
    }
}
