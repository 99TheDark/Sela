use std::fmt::Display;

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
