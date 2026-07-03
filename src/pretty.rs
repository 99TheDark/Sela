use std::{
    fs::File,
    io::{self, BufWriter, StdoutLock, Write},
};

use crate::pretty::{
    self,
    color::AnsiColor,
    theme::{Coloring, Spacing, Theme},
};

pub mod ast;
pub mod builder;
pub mod color;
pub mod error;
pub mod formatter;
pub mod node;
pub mod theme;

pub use builder::*;
pub use error::*;
pub use formatter::*;
pub use node::*;

pub trait Pretty<'a, B: io::Write> {
    fn fmt_title<'w>(&self, f: &mut pretty::Formatter<'w, B>) -> pretty::Result;

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&'a self) -> ChildNodes<'a, B>;
}

pub fn write_file<'a>(
    file_name: String,
    src: &'a dyn Pretty<'a, BufWriter<File>>,
) -> pretty::Result {
    let file = File::create(file_name)?;
    let mut buffer = BufWriter::new(file);

    Formatter::new(&mut buffer, Theme::sharp(Coloring::None, Spacing::Full))
        .format(src)?;

    buffer.flush()?;
    Ok(())
}

pub fn print<'a>(src: &'a dyn Pretty<'a, StdoutLock<'_>>) -> pretty::Result {
    let stdout = std::io::stdout();
    let mut buffer = stdout.lock();

    Formatter::new(&mut buffer, Theme::round(Coloring::Colorful, Spacing::Full))
        .format(src)?;

    buffer.flush()?;
    Ok(())
}
