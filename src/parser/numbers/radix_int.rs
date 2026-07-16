use std::{convert, fmt, hint, ops};

use crate::parser::numbers::{ErrorSet, IsEmpty, NumberParsingError, float};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    UnderConsecutive,
    UnderEnd,
    UnderPostRadix,
    IncompleteRadix,
    UnsupportedRadix,
    LeadingZeros,
    InvalidDigit,
    TooLarge,
}

#[derive(Debug, Copy, Clone)]
pub struct ParsingError(pub(super) NumberParsingError);

impl ParsingError {
    pub fn new() -> Self {
        Self(NumberParsingError::new())
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Deref for ParsingError {
    type Target = NumberParsingError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ParsingError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
            LeadingZeros => self.0.leading_zeros(),
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
            LeadingZeros => self.0.set_leading_zeros(true),
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

pub(super) fn parse_bytes(mut src: &[u8]) -> Result<u64, ParsingError> {
    let mut errors = ParsingError::new();
    let mut result = 0u64;

    let radix: u64 = match src[1] {
        // b'z' | b'Z' => 36,
        b'x' | b'X' => 16,
        b'o' | b'O' => 8,
        // b'q' | b'Q' => 4,
        b'b' | b'B' => 2,
        _ => {
            errors.raise(ErrorKind::UnsupportedRadix);
            36
        }
    };

    match src.len() {
        0..=2 => errors.raise(ErrorKind::IncompleteRadix),
        _ if src[2] == b'_' => errors.raise(ErrorKind::UnderPostRadix),
        _ if src[2] == b'0' => {
            errors.raise(ErrorKind::LeadingZeros);
            let non_zero_pos = src.iter().position(|&b| b != b'0').map_or(0, |p| p + 1);
            src = &src[non_zero_pos..];
        }
        _ => {}
    }

    while let [byte, rest @ ..] = &src[2..] {
        'byte_match: {
            match (byte, rest.get(0)) {
                (b'_', Some(b'_')) => errors.raise(ErrorKind::UnderConsecutive),
                (b'_', None) => errors.raise(ErrorKind::UnderEnd),
                (b'_', _) => {}
                _ => {
                    if errors.has(ErrorKind::TooLarge) {
                        hint::cold_path();
                        break 'byte_match;
                    }

                    let digit = match byte {
                        b'0'..=b'9' => byte - b'0',
                        b'a'..=b'z' => byte - b'a' + 10,
                        b'A'..=b'Z' => byte - b'A' + 10,
                        _ => u8::MAX,
                    } as u64;

                    if digit >= radix {
                        hint::cold_path();
                        errors.raise(ErrorKind::InvalidDigit);
                        break 'byte_match;
                    }

                    let new_result =
                        result.checked_mul(radix).and_then(|result| result.checked_add(digit));

                    if let Some(new_result) = new_result {
                        result = new_result;
                    } else {
                        errors.raise(ErrorKind::TooLarge);
                    }
                }
            }
        }
        src = &src[1..];
    }

    if errors.is_empty() { Ok(result) } else { Err(errors) }
}
