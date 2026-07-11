use std::convert;

use crate::parser::numbers::{ErrorSet, IsEmpty, NumberParsingError, float};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    UnderConsecutive,
    UnderEnd,
    UnderPostRadix,
    IncompleteRadix,
    UnsupportedRadix,
    InvalidDigit,
    TooLarge,
}

pub struct ParsingError(pub(super) NumberParsingError);

impl ParsingError {
    pub fn new() -> Self {
        Self(NumberParsingError::new())
    }
}

impl ErrorSet for ParsingError {
    type Kind = ErrorKind;

    fn has(&self, kind: Self::Kind) -> bool {
        use ErrorKind::*;
        match kind {
            UnderConsecutive => self.0.under_consecutive(),
            UnderEnd => self.0.under_end(),
            UnderPostRadix => self.0.under_post_radix(),
            IncompleteRadix => self.0.incomplete_radix(),
            UnsupportedRadix => self.0.unsupported_radix(),
            InvalidDigit => self.0.invalid_digit(),
            TooLarge => self.0.too_large(),
        }
    }

    fn raise(&mut self, kind: Self::Kind) {
        use ErrorKind::*;
        match kind {
            UnderConsecutive => self.0.set_under_consecutive(true),
            UnderEnd => self.0.set_under_end(true),
            UnderPostRadix => self.0.set_under_post_radix(true),
            IncompleteRadix => self.0.set_incomplete_radix(true),
            UnsupportedRadix => self.0.set_unsupported_radix(true),
            InvalidDigit => self.0.set_invalid_digit(true),
            TooLarge => self.0.set_too_large(true),
        }
    }
}

impl convert::From<float::ParsingError> for ParsingError {
    fn from(value: float::ParsingError) -> Self {
        Self(value.0)
    }
}

impl IsEmpty for ParsingError {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
