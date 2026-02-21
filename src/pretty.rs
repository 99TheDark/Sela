use std::{
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::pretty::theme::{Coloring, Spacing, Theme};

pub mod ast;
pub mod theme;

pub trait Pretty {
    fn title(&self) -> String;
    fn color(&self) -> Option<AnsiColor>;
    fn children(&self) -> Vec<&dyn Pretty>;
}

pub struct Formatter<W: io::Write> {
    buffer: W,
    stack: Vec<bool>,
    theme: Theme,
    colored: bool,
}

pub enum AnsiColor {
    // TODO: Maybe change to `= 30`, ...?
    Black,
    Gray,
    Red,
    Scarlet,
    Green,
    Lime,
    Yellow,
    Mustard,
    Blue,
    Steel,
    Purple,
    Magenta,
    Cyan,
    Teal,
    White,
    TrueWhite,
}

impl AnsiColor {
    pub fn ansi_code(&self) -> u8 {
        use AnsiColor::*;
        match self {
            Black => 30,
            Red => 31,
            Green => 32,
            Yellow => 33,
            Blue => 34,
            Purple => 35,
            Cyan => 36,
            White => 37,
            Gray => 90,
            Scarlet => 91,
            Lime => 92,
            Mustard => 93,
            Steel => 94,
            Magenta => 95,
            Teal => 96,
            TrueWhite => 97,
        }
    }

    pub fn color<S: Display>(&self, s: S) -> String {
        format!("\x1b[{}m{}\x1b[0m", self.ansi_code(), s)
    }
}

impl<W: io::Write> Formatter<W> {
    pub const fn new(buffer: W, theme: Theme, colored: bool) -> Self {
        Self {
            buffer,
            stack: Vec::new(),
            theme,
            colored,
        }
    }

    pub fn format(&mut self, node: &dyn Pretty) -> io::Result<()> {
        self.format_node(node, false)
    }

    fn format_node(&mut self, node: &dyn Pretty, is_last: bool) -> io::Result<()> {
        self.start_line(is_last)?;

        let title = if self.colored
            && let Some(color) = node.color()
        {
            color.color(node.title())
        } else {
            node.title()
        };
        writeln!(self.buffer, "{}", title)?;

        let children = node.children();
        let len = children.len();

        self.stack.push(is_last);
        for (i, child) in children.iter().enumerate() {
            self.format_node(*child, i == len - 1)?;
        }
        self.stack.pop();

        Ok(())
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> io::Result<()> {
        <Self as io::Write>::write_fmt(self, args)
    }

    fn start_line(&mut self, is_last: bool) -> std::io::Result<()> {
        if self.stack.is_empty() {
            return Ok(());
        }

        for &parent_last in &self.stack[1..] {
            if parent_last {
                self.buffer.write_all(self.empty())?;
            } else {
                self.buffer.write_all(self.down().as_bytes())?;
            }
        }

        self.buffer.write_all(self.connector(is_last).as_bytes())
    }

    fn empty(&self) -> &'static [u8] {
        match self.theme.spacing {
            Spacing::Full => b"    ",
            Spacing::Compact => b"  ",
        }
    }

    fn down(&self) -> String {
        match (self.theme.coloring, self.theme.spacing) {
            (Coloring::Colorful, Spacing::Full) => {
                format!("{}   ", AnsiColor::Gray.color(self.theme.v_bar))
            }
            (Coloring::Colorful, Spacing::Compact) => {
                format!("{} ", AnsiColor::Gray.color(self.theme.v_bar))
            }
            (Coloring::None, Spacing::Full) => format!("{}   ", self.theme.v_bar),
            (Coloring::None, Spacing::Compact) => format!("{} ", self.theme.v_bar),
        }
    }

    fn connector(&self, is_last: bool) -> String {
        let join = if is_last {
            self.theme.dl_bend
        } else {
            self.theme.l_conn
        };

        match (self.theme.coloring, self.theme.spacing) {
            (Coloring::Colorful, Spacing::Full) => {
                format!(
                    "{}{} ",
                    AnsiColor::Gray.color(join),
                    AnsiColor::Gray.color(self.theme.h_bar)
                )
            }
            (Coloring::Colorful, Spacing::Compact) => {
                format!("{} ", AnsiColor::Gray.color(join))
            }
            (Coloring::None, Spacing::Full) => format!("{}{} ", join, self.theme.h_bar),
            (Coloring::None, Spacing::Compact) => format!("{} ", join),
        }
    }

    pub fn push_level(&mut self, is_last: bool) {
        self.stack.push(is_last);
    }

    pub fn pop_level(&mut self) {
        self.stack.pop();
    }
}

impl<W: io::Write> io::Write for Formatter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

pub fn write_file(file_name: String, src: &dyn Pretty) -> io::Result<()> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);

    let mut f = Formatter::new(
        &mut writer,
        Theme::sharp(Coloring::None, Spacing::Full),
        false,
    );
    f.format_node(src, false)?;

    writer.flush()
}

pub fn print(src: &dyn Pretty) -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    let mut f = Formatter::new(
        &mut handle,
        Theme::round(Coloring::Colorful, Spacing::Full),
        true,
    );
    f.format(src)?;

    handle.flush()
}
