use std::{fmt, hint, ops};

use crate::parser::numbers::{ErrorSet, IsEmpty, NumberParsingError};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    UnderConsecutive,
    UnderEnd,
    LeadingZeros,
    NonNumeric,
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
            LeadingZeros => self.0.leading_zeros(),
            NonNumeric => self.0.non_numeric(),
            TooLarge => self.0.too_large(),
        }
    }

    fn raise(&mut self, kind: Self::Kind) {
        use ErrorKind::*;
        match kind {
            UnderConsecutive => self.0.set_under_consecutive(true),
            UnderEnd => self.0.set_under_end(true),
            LeadingZeros => self.0.set_leading_zeros(true),
            NonNumeric => self.0.set_non_numeric(true),
            TooLarge => self.0.set_too_large(true),
        }
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

    if src.len() > 1 && src[0] == b'0' && src[1] != b'_' {
        errors.raise(ErrorKind::LeadingZeros);
        let non_zero_pos = src.iter().position(|&b| b != b'0').map_or(0, |p| p + 1);
        src = &src[non_zero_pos..];
    }

    while let [byte, rest @ ..] = src {
        src = &src[1..];
        match (byte, rest.get(0)) {
            (b'_', Some(b'_')) => errors.raise(ErrorKind::UnderConsecutive),
            (b'_', None) => errors.raise(ErrorKind::UnderEnd),
            (b'_', _) => {}
            (b'0'..=b'9', _) => {
                if errors.has(ErrorKind::TooLarge) {
                    hint::cold_path();
                    continue;
                }

                let digit = byte - b'0';

                let new_result =
                    result.checked_mul(10).and_then(|result| result.checked_add(digit as u64));

                if let Some(new_result) = new_result {
                    result = new_result;
                } else {
                    hint::cold_path();
                    errors.raise(ErrorKind::TooLarge);
                }
            }
            _ => errors.raise(ErrorKind::NonNumeric),
        }
    }

    if errors.is_empty() { Ok(result) } else { Err(errors) }
}

#[derive(Default)]
pub(super) struct FloatParsingOutput {
    pub part: u64,
    pub num_digits: usize,
    pub num_digits_capped: usize,
    pub len: usize,
}

pub(super) fn parse_bytes_for_float(
    mut src: &[u8],
    max_digits: usize,
    can_lead_zeros: bool,
    stop_at_exp: bool,
) -> Result<FloatParsingOutput, ParsingError> {
    let mut errors = ParsingError::new();
    let mut result = 0u64;
    let mut length = 0;
    let mut num_digits = 0;
    let mut frozen = false;
    let mut first_digit: Option<u8> = None;

    while let [byte, rest @ ..] = src {
        if *byte == b'.' {
            break;
        }
        if *byte == b'e' | b'E' && stop_at_exp {
            break;
        }

        src = &src[1..];
        match (byte, rest.get(0)) {
            (b'_', Some(b'_')) => errors.raise(ErrorKind::UnderConsecutive),
            (b'_', None) => errors.raise(ErrorKind::UnderEnd),
            (b'_', _) => {}
            (b'0'..=b'9', _) => {
                num_digits += 1;
                if num_digits > max_digits {
                    frozen = true;
                }

                if frozen {
                    hint::cold_path();
                    continue;
                }

                let digit = byte - b'0';
                let new_result =
                    result.checked_mul(10).and_then(|result| result.checked_add(digit as u64));

                match first_digit {
                    Some(first) if first == 0 && !can_lead_zeros => {
                        errors.raise(ErrorKind::LeadingZeros)
                    }
                    None => first_digit = Some(digit),
                    _ => {}
                }

                if let Some(new_result) = new_result {
                    result = new_result;
                } else {
                    hint::cold_path();
                    errors.raise(ErrorKind::TooLarge);
                    frozen = true;
                }
            }
            _ => errors.raise(ErrorKind::NonNumeric),
        }

        length += 1;
    }

    if errors.is_empty() {
        Ok(FloatParsingOutput {
            part: result,
            num_digits,
            num_digits_capped: num_digits.min(max_digits),
            len: length,
        })
    } else {
        Err(errors)
    }
}
