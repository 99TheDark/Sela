use std::hint;

use crate::parser::numbers::{ErrorSet, IsEmpty, NumberParsingError};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    UnderConsecutive,
    UnderEnd,
    LeadingZeros,
    NonNumeric,
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

    if src.len() > 1 && src[0] == b'0' {
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

                let new_result = result
                    .checked_mul(10)
                    .and_then(|result| result.checked_add(digit as u64));

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
    // let mut prev_byte = 0u8;

    if errors.is_empty() { Ok(result) } else { Err(errors) }
}
